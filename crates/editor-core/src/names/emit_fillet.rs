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
//! # KNOWN HAZARD — an upstream tie is not propagated (issue #708)
//!
//! `up()` reads the target's name through `NameTable::name_of` and
//! never inspects the [`Entry`](super::table::Entry) behind it. Every
//! member of an [`Entry::Tied`](super::table::Entry::Tied) row carries
//! the SAME name, so two tied sources both filleted would hand two
//! minted entities one upstream name and the second insertion would
//! refuse [`NamingError::Duplicate`] — an error whose contract is "the
//! no-silent-aliasing bug", reported for a legitimate N2 tie. The
//! generic emitters in [`super::emit`] carry the propagation arm this
//! one lacks (`Entry::Tied` → `insert_tied`).
//!
//! Unreachable today, and the reason is the SHAPE of the call sites
//! rather than how many there are: every production `insert_tied`
//! either matches on an upstream [`Entry::Tied`](super::table::Entry)
//! or descends from one ([`super::emit_topo`]'s tie lane is entered
//! only when `lookup` already returned `Tied`), so nothing mints a
//! FIRST tie. The propagation shape here is a naming-design question
//! (one `RoleSeg` slot holds one name; a tied source needs either N
//! segments or a tied mint), so it must be decided WITH the unit that
//! mints the first tie, not guessed against a hypothetical producer.
//! #708 carries the site-by-site table.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use sweep::fillet::naming::{FilletNaming, RimSide};
use topo::{Body, EdgeKey, FaceKey, VertexKey};

use super::emit::{NamingError, check_total, ent, name1};
use super::role::{EntityKind, RimSupport, RoleSeg, StableName};
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
    let up = |key: EntityKey| -> Result<StableName, NamingError> {
        target
            .name_of(&ent(0, key))
            .cloned()
            .ok_or(NamingError::MissingUpstream { node: target_node })
    };
    let up_f = |k: FaceKey| up(EntityKey::Face(k));
    let up_e = |k: EdgeKey| up(EntityKey::Edge(k));
    let up_v = |k: VertexKey| up(EntityKey::Vertex(k));
    let b = Box::new;

    // ---- The mints, by role. ----
    let mut minted: BTreeMap<EntityKey, RoleSeg> = BTreeMap::new();
    let mut put = |key: EntityKey, seg: RoleSeg| -> Result<(), NamingError> {
        // Two records for one key is an emission bug, not a tie: the
        // surgery mints each entity once.
        if minted.insert(key, seg).is_some() {
            return Err(NamingError::Emission {
                what: "the surgery recorded one entity twice",
            });
        }
        Ok(())
    };

    for (f, e) in &rec.blends {
        put(EntityKey::Face(*f), RoleSeg::BlendFace(b(up_e(*e)?)))?;
    }
    for (f, v) in &rec.corners {
        put(EntityKey::Face(*f), RoleSeg::CornerFace(b(up_v(*v)?)))?;
    }
    for (t, e, f) in &rec.trims {
        put(
            EntityKey::Edge(*t),
            RoleSeg::TrimEdge {
                edge: b(up_e(*e)?),
                support: b(up_f(*f)?),
            },
        )?;
    }
    for (foot, v, f) in &rec.feet {
        put(
            EntityKey::Vertex(*foot),
            RoleSeg::FootVertex {
                vertex: b(up_v(*v)?),
                support: b(up_f(*f)?),
            },
        )?;
    }
    for (a, v, e) in &rec.arcs {
        put(
            EntityKey::Edge(*a),
            RoleSeg::CornerArc {
                vertex: b(up_v(*v)?),
                edge: b(up_e(*e)?),
            },
        )?;
    }
    for (f, edges) in &rec.bands {
        // Canonical order = NAME order (the N3 `Merged` convention): a
        // rim is a cycle with no first edge, so only the SET is
        // covariant.
        let mut names = Vec::with_capacity(edges.len());
        for e in edges {
            names.push(up_e(*e)?);
        }
        names.sort();
        names.dedup();
        put(EntityKey::Face(*f), RoleSeg::BandFace(names))?;
    }
    for (t, e, side) in &rec.rim_trims {
        put(
            EntityKey::Edge(*t),
            RoleSeg::BandTrim {
                edge: b(up_e(*e)?),
                support: match side {
                    RimSide::Plane => RimSupport::Plane,
                    RimSide::Sphere => RimSupport::Curved,
                },
            },
        )?;
    }
    for (foot, v) in &rec.rim_feet {
        put(EntityKey::Vertex(*foot), RoleSeg::BandFoot(b(up_v(*v)?)))?;
    }
    for (v, m) in &rec.meridian_splits {
        put(EntityKey::Vertex(*v), RoleSeg::BandCross(b(up_e(*m)?)))?;
    }
    for (e, m) in &rec.meridian_remnants {
        put(EntityKey::Edge(*e), RoleSeg::BandCut(b(up_e(*m)?)))?;
    }
    for (e, m) in &rec.slits {
        put(EntityKey::Edge(*e), RoleSeg::BandSlit(b(up_e(*m)?)))?;
    }

    // ---- The table: the body row, then every output entity. ----
    let mut t = NameTable::new();
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
        let seg = match minted.get(&key) {
            Some(seg) => seg.clone(),
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
                RoleSeg::FromTarget(b(up(key)?))
            }
        };
        t.insert(name1(kind, node, seg), ent(0, key))?;
    }
    check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}
