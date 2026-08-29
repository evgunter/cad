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
                    RimSide::Plane => RimSupport::Plane,
                    RimSide::Sphere => RimSupport::Curved,
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
