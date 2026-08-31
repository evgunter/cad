//! **The fillet naming emitter** (M6-5 PR-1) — the composition
//! surgery's output, named from BIRTH data.
//!
//! The kernel hands over [`FilletNaming`]: rows written by the
//! surgery as it minted each entity, each naming the SOURCE entity the
//! mint was made for. This pass is a mechanical translation of those
//! rows into [`RoleSeg`]s — no geometry is read, nothing is matched.
//! That is the whole point: `Node::Fillet` was every-edge precisely
//! because a fillet had no birth channel, and matching after the fact
//! is what N4 forbids.
//!
//! # Covariance
//!
//! Every fillet segment carries the source entity's OWN name from the
//! target's table, so a fillet name is a function of the target's
//! names — the `FromA`/`Seam` shape. When an upstream bump moves the
//! target's names, these move with them; when the bump changes
//! nothing upstream, these are bit-identical. This emitter contributes
//! no independent judgment that could disagree.
//!
//! # The provenance channels, and the totality that closes them
//!
//! An output entity is either a recorded mint or a survivor keeping
//! its source arena key ([`FilletNaming`]'s module docs). Survivors
//! take [`RoleSeg::FromTarget`] of their upstream name; mints take
//! their role. Anything that is neither — a key minted without a
//! record — has no upstream name and surfaces as
//! [`NamingError::MissingUpstream`], loudly, rather than being guessed
//! around. The final [`check_total`] closes the other direction.
//!
//! One guard sits between those two cases, because "keeps its source
//! arena key" is a claim about NUMBERING and nothing here re-checks
//! it. A would-be survivor whose key the records list as RETIRED
//! refuses [`NamingError::Emission`].
//!
//! **That guard is unreachable BY CONSTRUCTION, and it is worth
//! saying which construction.** The surgery mutates a clone of the
//! target's own body, and `topo::Body`'s arenas are `slotmap::SlotMap`
//! over `new_key_type!` keys, which bump a slot's VERSION on removal:
//! a retired key is never reissued, so a retired key cannot reappear
//! in the output arena at all. There is no input this code can be
//! handed that reaches the refusal.
//!
//! It is kept because the property it rests on lives in another
//! crate's choice of container. If a future body ever numbered its
//! entities itself, or reused slots, an unrecorded mint would be named
//! `FromTarget` of an unrelated entity — and whether that misnaming
//! got caught would depend on whether the real owner of the name
//! happened to collide at insertion. That is luck, not a guarantee.
//! Same posture as `wire_fillet`'s refusal of `naming: None`.
//!
//! # An upstream tie PROPAGATES (B1)
//!
//! Every upstream name this emitter reads comes through
//! [`super::defer::upstream_name`], which reports whether the
//! operand's entry is an [`Entry::Tied`](super::table::Entry::Tied)
//! row, and a name built from any tied upstream is deferred into
//! [`super::defer::TieRows`] rather than inserted one member at a
//! time. That is what a tie needs: its members all carry the SAME
//! name, so the flush hands the whole candidate list to `insert_tied`
//! at once, and `Duplicate` keeps meaning what it says — the
//! no-silent-aliasing bug, never a legitimate N2 tie.
//!
//! A role that wraps SEVERAL upstream names is tie-descended when ANY
//! of them is. That is the conservative reading and the correct one:
//! the name it composes is not distinguishable across the tied
//! candidates, so the row belongs in the lane that merges.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use sweep::fillet::naming::{FilletNaming, RimSide};
use topo::{Body, EdgeKey, FaceKey, VertexKey};

use super::defer::{TieRows, put as put_row, upstream_name};
use super::emit::{NamingError, check_total, ent, name1};
use super::role::{EntityKind, RimSupport, RoleSeg};
use super::table::{EntityKey, NameTable};
use crate::node::RecipeNodeId;

/// Names one fillet result.
///
/// `target` is the fillet's single operand's table (body index 0 —
/// `body_operand` admits only single-body values), `body` the fillet
/// output, `rec` its birth records.
///
/// # Errors
///
/// [`NamingError::MissingUpstream`] when a record (or a survivor) names
/// a source entity the target's table does not carry — a wiring bug;
/// [`NamingError::Duplicate`] on aliasing at insertion;
/// [`NamingError::Unnamed`] if the result is not covered.
pub(crate) fn name_fillet<T: geom_core::Real>(
    node: RecipeNodeId,
    target_node: RecipeNodeId,
    target: &NameTable,
    body: &Body<T>,
    rec: &FilletNaming,
) -> Result<Arc<NameTable>, NamingError> {
    name_blend(node, target_node, target, body, rec)
}

/// The birth-record translation both blend emitters run.
///
/// It is ONE function because the two have one input type and one role
/// vocabulary (RECIPE-DOORS D3), and the only thing that separates a
/// chamfer's names from a fillet's is `node` — the minting id every
/// segment is stamped with. Written twice, the pair would drift, and
/// the thing they would drift on is the tie deferral.
pub(super) fn name_blend<T: geom_core::Real>(
    node: RecipeNodeId,
    target_node: RecipeNodeId,
    target: &NameTable,
    body: &Body<T>,
    rec: &FilletNaming,
) -> Result<Arc<NameTable>, NamingError> {
    // Every upstream read goes through the deferral's own reader, so
    // the tie bit travels with the name it belongs to and no call site
    // can drop it.
    let up = |key: EntityKey| upstream_name(target, target_node, ent(0, key));
    let up_f = |k: FaceKey| up(EntityKey::Face(k));
    let up_e = |k: EdgeKey| up(EntityKey::Edge(k));
    let up_v = |k: VertexKey| up(EntityKey::Vertex(k));
    let b = Box::new;

    // ---- The mints, by role. ----
    //
    // Each row carries the tie bit of the upstream names it wraps: ANY
    // tied name makes the composed name tie-descended, because the
    // name it composes does not distinguish the tied candidates.
    let mut minted: BTreeMap<EntityKey, (RoleSeg, bool)> = BTreeMap::new();
    let mut put = |key: EntityKey, seg: RoleSeg, tied: bool| -> Result<(), NamingError> {
        // Two records for one key is an emission bug, not a tie: the
        // surgery mints each entity once.
        if minted.insert(key, (seg, tied)).is_some() {
            return Err(NamingError::Emission {
                what: "the surgery recorded one entity twice",
            });
        }
        Ok(())
    };

    for (f, e) in &rec.blends {
        let e = up_e(*e)?;
        put(EntityKey::Face(*f), RoleSeg::BlendFace(b(e.name)), e.tied)?;
    }
    for (f, v) in &rec.corners {
        let v = up_v(*v)?;
        put(EntityKey::Face(*f), RoleSeg::CornerFace(b(v.name)), v.tied)?;
    }
    for (t, e, f) in &rec.trims {
        let (e, f2) = (up_e(*e)?, up_f(*f)?);
        let tied = e.tied || f2.tied;
        put(
            EntityKey::Edge(*t),
            RoleSeg::TrimEdge {
                edge: b(e.name),
                support: b(f2.name),
            },
            tied,
        )?;
    }
    for (foot, v, f) in &rec.feet {
        let (v, f) = (up_v(*v)?, up_f(*f)?);
        let tied = v.tied || f.tied;
        put(
            EntityKey::Vertex(*foot),
            RoleSeg::FootVertex {
                vertex: b(v.name),
                support: b(f.name),
            },
            tied,
        )?;
    }
    for (a, v, e) in &rec.arcs {
        let (v, e) = (up_v(*v)?, up_e(*e)?);
        let tied = v.tied || e.tied;
        put(
            EntityKey::Edge(*a),
            RoleSeg::CornerArc {
                vertex: b(v.name),
                edge: b(e.name),
            },
            tied,
        )?;
    }
    for (f, edges) in &rec.bands {
        // Canonical order = NAME order (the N3 `Merged` convention): a
        // rim is a cycle with no first edge, so only the SET is
        // covariant.
        let mut names = Vec::with_capacity(edges.len());
        let mut tied = false;
        for e in edges {
            let e = up_e(*e)?;
            tied |= e.tied;
            names.push(e.name);
        }
        names.sort();
        names.dedup();
        put(EntityKey::Face(*f), RoleSeg::BandFace(names), tied)?;
    }
    for (t, e, side) in &rec.rim_trims {
        let e = up_e(*e)?;
        let tied = e.tied;
        put(
            EntityKey::Edge(*t),
            RoleSeg::BandTrim {
                edge: b(e.name),
                support: match side {
                    RimSide::Host => RimSupport::Host,
                    RimSide::Mate => RimSupport::Mate,
                },
            },
            tied,
        )?;
    }
    for (foot, v) in &rec.rim_feet {
        let v = up_v(*v)?;
        put(
            EntityKey::Vertex(*foot),
            RoleSeg::BandFoot(b(v.name)),
            v.tied,
        )?;
    }
    for (v, m) in &rec.meridian_splits {
        let m = up_e(*m)?;
        put(EntityKey::Vertex(*v), RoleSeg::BandCross(b(m.name)), m.tied)?;
    }
    for (e, m) in &rec.meridian_remnants {
        let m = up_e(*m)?;
        put(EntityKey::Edge(*e), RoleSeg::BandCut(b(m.name)), m.tied)?;
    }
    for (e, m) in &rec.slits {
        let m = up_e(*m)?;
        put(EntityKey::Edge(*e), RoleSeg::BandSlit(b(m.name)), m.tied)?;
    }

    // ---- The table: the body row, then every output entity. ----
    let mut t = NameTable::new();
    let mut tie = TieRows::default();
    t.insert(
        name1(EntityKind::Body, node, RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    let mut rows: Vec<(EntityKind, EntityKey)> = Vec::new();
    rows.extend(
        body.faces()
            .map(|(k, _)| (EntityKind::Face, EntityKey::Face(k))),
    );
    rows.extend(
        body.edges()
            .map(|(k, _)| (EntityKind::Edge, EntityKey::Edge(k))),
    );
    rows.extend(
        body.vertices()
            .map(|(k, _)| (EntityKind::Vertex, EntityKey::Vertex(k))),
    );
    // The retired set, as a lookup: a key the fillet RETIRED can never
    // be a survivor, whatever its arena says.
    let retired_e: BTreeSet<EdgeKey> = rec.dead.edges.iter().copied().collect();
    let retired_v: BTreeSet<VertexKey> = rec.dead.vertices.iter().copied().collect();
    for (kind, key) in rows {
        let (seg, from_tie) = match minted.get(&key) {
            Some((seg, tied)) => (seg.clone(), *tied),
            // Not minted, so it must be a survivor keeping its source
            // arena key — UNLESS the records say that key was retired,
            // in which case the match is not provenance.
            //
            // Unreachable by construction: the arenas are slotmaps
            // whose keys carry a slot version, so a retired key is
            // never reissued and cannot come back here. The guard
            // holds the invariant against that container choice
            // changing, not against a state reachable today (module
            // docs).
            None => {
                let dead = match key {
                    EntityKey::Edge(k) => retired_e.contains(&k),
                    EntityKey::Vertex(k) => retired_v.contains(&k),
                    // Faces are never retired — a support shrinks, it
                    // does not die — so a face key can only be a real
                    // survivor, and `Retired` carries no face channel
                    // to consult. Asserted in both directions by
                    // `sweep/tests/m6_5_fillet_naming.rs`.
                    EntityKey::Face(_) | EntityKey::Body => false,
                };
                if dead {
                    return Err(NamingError::Emission {
                        what: "an output entity is neither minted nor a survivor: its key was \
                               recorded as RETIRED, so the match is an arena coincidence",
                    });
                }
                let u = up(key)?;
                (RoleSeg::FromTarget(b(u.name)), u.tied)
            }
        };
        put_row(
            &mut t,
            &mut tie,
            from_tie,
            name1(kind, node, seg),
            ent(0, key),
        )?;
    }
    // ONE stage, so one flush — and it must precede the totality
    // check, which reads the table this drains into.
    tie.flush(&mut t)?;
    check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

#[cfg(test)]
mod tie_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::sync::Arc;

    use geom_core::{Point2, Tol, Vec3};
    use topo::Body;

    use super::*;
    use crate::names::emit_sweep::name_extrude;
    use crate::names::table::{EntityRef, Entry};
    use profile::RawLoop;

    /// A unit cube and the extrude table that names it.
    fn cube() -> (Body<f64>, Arc<NameTable>) {
        let plane = profile::SketchPlane::from_frame(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let square = profile::ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .map(|(x, y)| Point2::new(x, y)),
        );
        let prof = profile::Profile::new(plane, vec![square])
            .validate(Tol::witness())
            .unwrap();
        let built = sweep::extrude(&prof, sweep::Extrusion::Distance(1.0_f64), Tol::witness())
            .expect("a unit cube extrudes");
        let table = name_extrude(RecipeNodeId(1), &built).expect("the extrude names");
        (built.body, table)
    }

    /// Rebuilds `table` with `a` and `b` TIED under `a`'s name — the
    /// planted upstream tie. Everything else is copied across
    /// unchanged, so the only difference from the real table is the one
    /// row under test.
    fn with_tie(table: &NameTable, a: EntityRef, b: EntityRef) -> NameTable {
        let tied_name = table.name_of(&a).expect("a is named").clone();
        let b_name = table.name_of(&b).expect("b is named").clone();
        let mut out = NameTable::new();
        for (name, entry) in table.iter() {
            if *name == tied_name || *name == b_name {
                continue;
            }
            match entry {
                Entry::Unique(e) => out.insert(name.clone(), *e).expect("a fresh row"),
                Entry::Tied(es) => out
                    .insert_tied(name.clone(), es.clone())
                    .expect("a fresh row"),
            }
        }
        let mut ents = vec![a, b];
        ents.sort();
        out.insert_tied(tied_name, ents).expect("the planted tie");
        out
    }

    /// Every edge of the cube, in arena order — the whole-body request
    /// `die_fillet` authors, which the assembly's front door admits.
    fn all_edges(body: &Body<f64>) -> Vec<topo::EdgeKey> {
        body.edges().map(|(k, _)| k).collect()
    }

    /// **A planted upstream tie flows through the deferral** — the
    /// #708 row, executed rather than described.
    ///
    /// Both members of an `Entry::Tied` row are blended, so both mints
    /// compose the SAME upstream name. Before the deferral the second
    /// insertion refused `Duplicate`, reporting the no-silent-aliasing
    /// bug for a legitimate N2 tie. Now the rows are deferred and
    /// flushed together, and the result carries one TIED entry.
    ///
    /// This is a unit test of the deferral path itself: today's tree
    /// mints no first tie, so no document reaches this state, and that
    /// is exactly why the emitter's behaviour under one has to be
    /// planted rather than waited for.
    #[test]
    fn a_planted_upstream_tie_reaches_the_output_table_as_a_tie() {
        let (body, table) = cube();
        let edges = all_edges(&body);
        let (a, b) = (edges[0], edges[1]);
        let planted = with_tie(
            &table,
            ent(0, EntityKey::Edge(a)),
            ent(0, EntityKey::Edge(b)),
        );

        let blended = sweep::fillet::build::fillet_edges(
            &body,
            &edges,
            0.125_f64,
            geom_core::Band::linear(Tol::witness()).expect("a band"),
            Tol::witness(),
        )
        .expect("every edge of a cube blends");
        let rec = blended.naming.as_ref().expect("the surgery keeps records");

        let out = name_blend(
            RecipeNodeId(2),
            RecipeNodeId(1),
            &planted,
            &blended.body,
            rec,
        )
        .expect("a tie propagates rather than refusing Duplicate");

        let widths: Vec<usize> = out
            .iter()
            .filter_map(|(_, e)| match e {
                Entry::Tied(es) => Some(es.len()),
                Entry::Unique(_) => None,
            })
            .collect();
        assert!(
            !widths.is_empty(),
            "the tie-descended rows must land as TIED entries, not be lost"
        );
        assert!(
            widths.iter().all(|w| *w >= 2),
            "a tied entry with one member is a narrowing bug, not a tie: {widths:?}"
        );

        // The CONTROL, and what makes the row above mean anything: the
        // same body, the same request, the untouched table — no tie
        // upstream, no tie downstream, and every row went through the
        // strict `insert`.
        let clean = name_blend(RecipeNodeId(2), RecipeNodeId(1), &table, &blended.body, rec)
            .expect("the untied table names as it always did");
        assert!(
            clean.iter().all(|(_, e)| matches!(e, Entry::Unique(_))),
            "an untied operand must produce no tied rows"
        );

        // The chamfer emitter is the same translation under a
        // different minting id, so the deferral reaches it by
        // construction — asserted, not assumed.
        let chamfered = sweep::fillet::build::chamfer_edges(
            &body,
            &edges,
            0.125_f64,
            geom_core::Band::linear(Tol::witness()).expect("a band"),
            Tol::witness(),
        )
        .expect("every edge of a cube chamfers");
        let crec = chamfered
            .naming
            .as_ref()
            .expect("the surgery keeps records");
        let cout = crate::names::name_chamfer(
            RecipeNodeId(3),
            RecipeNodeId(1),
            &planted,
            &chamfered.body,
            crec,
        )
        .expect("a tie propagates through the chamfer emitter too");
        // Held to the SAME bar as the fillet arm, not a weaker one.
        // `name_chamfer` delegating to `name_blend` is what makes the
        // deferral shared, but it is an implementation fact this row
        // must not assume: assert the property, so that a future
        // `emit_chamfer` which stops delegating still has to pass.
        let cwidths: Vec<usize> = cout
            .iter()
            .filter_map(|(_, e)| match e {
                Entry::Tied(es) => Some(es.len()),
                Entry::Unique(_) => None,
            })
            .collect();
        assert!(
            !cwidths.is_empty(),
            "the chamfer emitter must defer tie-descended rows as well"
        );
        assert!(
            cwidths.iter().all(|w| *w >= 2),
            "a tied entry with one member is a narrowing bug: {cwidths:?}"
        );
        let cclean = crate::names::name_chamfer(
            RecipeNodeId(3),
            RecipeNodeId(1),
            &table,
            &chamfered.body,
            crec,
        )
        .expect("the untied table names as it always did");
        assert!(
            cclean.iter().all(|(_, e)| matches!(e, Entry::Unique(_))),
            "an untied operand must produce no tied rows at the chamfer either"
        );
        // The mint id is the discrimination (D3): the same records
        // under a different node must produce DIFFERENT names, or the
        // shared role vocabulary would be an aliasing bug rather than a
        // deliberate reuse. This is what delegation could hide, so it
        // is asserted here rather than assumed.
        let fillet_names: BTreeSet<_> = clean.iter().map(|(n, _)| n.clone()).collect();
        let chamfer_names: BTreeSet<_> = cclean.iter().map(|(n, _)| n.clone()).collect();
        assert!(
            fillet_names.is_disjoint(&chamfer_names),
            "two blends under different nodes must share no name"
        );
    }
}
