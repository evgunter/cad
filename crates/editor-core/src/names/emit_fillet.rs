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
//! # Both doors, one vocabulary
//!
//! This pass serves the composition surgery AND (since M6-5 PR-2) the
//! whole-body rebuild. The two doors differ only in how a shrunk
//! SUPPORT face reaches it: the surgery leaves the source face in
//! place, so it arrives as a survivor (an output key that is also a
//! source key); the whole-body rebuild mints into a fresh arena, so it
//! arrives as a [`FilletNaming::supports`] row. Both land on
//! [`RoleSeg::FromTarget`] of the same upstream name — a name must not
//! depend on which door built the body, or the same edit would rename
//! things by changing which door the request falls through.
//!
//! # The provenance channels, and the totality that closes them
//!
//! An output entity is either a recorded mint or a survivor keeping
//! its source arena key ([`FilletNaming`]'s module docs). Survivors
//! take [`RoleSeg::FromTarget`] of their upstream name; mints take
//! their role. Anything that is neither — a key minted without a
//! record — has no upstream name and surfaces as
//! [`NamingError::MissingUpstream`], loudly, rather than being guessed
//! around. The final [`check_total`] closes the other direction, for
//! both doors alike.

use std::collections::BTreeMap;
use std::sync::Arc;

use sweep::fillet::naming::{FilletNaming, RimSide};
use topo::{Body, EdgeKey, FaceKey, VertexKey};

use super::emit::{NamingError, check_total, ent, name1};
use super::role::{EntityKind, RimSupport, RoleSeg, StableName};
use super::table::{EntityKey, NameTable};
use crate::node::RecipeNodeId;

/// Names one fillet result, from either assembly door.
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

    // A shrunk support: same role as a surgery survivor, different
    // channel (module docs).
    for (f, src) in &rec.supports {
        put(EntityKey::Face(*f), RoleSeg::FromTarget(b(up_f(*src)?)))?;
    }
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
    for (kind, key) in rows {
        let seg = match minted.get(&key) {
            Some(seg) => seg.clone(),
            // Not minted: a survivor, keeping its source arena key.
            None => RoleSeg::FromTarget(b(up(key)?)),
        };
        t.insert(name1(kind, node, seg), ent(0, key))?;
    }
    check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}
