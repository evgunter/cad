//! **The shell naming emitter** — the hollowing verb's output, named
//! from BIRTH data, under the shelling node's id.
//!
//! The kernel hands over [`ShellNaming`]: rows written by the doors
//! themselves as they act, each naming the SOURCE entity the mint was
//! made for. This pass is a mechanical translation of those rows into
//! [`RoleSeg`]s — no geometry is read, nothing is matched. The record's
//! own reading order is the translation:
//!
//! | record row | result entity | role |
//! |---|---|---|
//! | `outer` (survivors keep operand keys) | outer wall face | [`RoleSeg::FromTarget`] |
//! | `inner`, `inner_edges`, `inner_vertices` | cavity twin | [`RoleSeg::Inner`] |
//! | `rims[i].rim` | the chart's annular rim face | [`RoleSeg::Rim`] of `sources[0]`'s name |
//! | `rims[i].ring_edges` / `ring_vertices` | the rim's ring | rows of `inner_edges` / `inner_vertices` verbatim, so `Inner` of the boundary edge — no second role |
//! | `rims[i].holes[j].face` | a promoted hole annulus | [`RoleSeg::HoleRim`], `j` in pairing order |
//! | `dead` | nothing | nothing — a designated face's own name VANISHES |
//!
//! Every surviving edge and vertex is a source entity carried through
//! ([`RoleSeg::FromTarget`]): the rim face keeps its designated face's
//! outer loop, so those edges are the operand's, and every other
//! surviving edge or vertex is the outer wall's.
//!
//! # Covariance
//!
//! Every shell segment carries the source entity's OWN name from the
//! target's table, so a shell name is a function of the target's names.
//! When an upstream bump moves the target's names, these move with
//! them; when the bump changes nothing upstream, these are
//! bit-identical. The emitter contributes no independent judgment.
//!
//! # The provenance channels, and the totality that closes them
//!
//! An output entity is either a recorded mint or a survivor keeping its
//! source arena key. A would-be survivor whose key the record lists as
//! RETIRED refuses [`NamingError::Emission`] — unreachable by
//! construction (the arenas are slotmaps whose keys carry a version, so
//! a retired key is never reissued), and kept for the reason
//! `emit_blend` keeps its guard: the property rests on another crate's
//! container choice. A survivor face must additionally be an `outer`
//! row: the record lists every non-designated source face there, so a
//! face that is neither minted nor listed has no provenance and refuses
//! the same way. Anything that is neither a survivor nor a recorded
//! mint is [`NamingError::MissingUpstream`], loudly; the final
//! [`check_total`] closes the other direction.
//!
//! # An upstream tie PROPAGATES (B1)
//!
//! Every upstream name comes through [`super::defer::upstream_name`],
//! and a name built from a tied upstream is deferred into
//! [`super::defer::TieRows`] rather than inserted one member at a time
//! — the blend emitter's rule, for the blend emitter's reason: tied
//! members carry the SAME name, so the flush hands the whole candidate
//! list to `insert_tied` at once and `Duplicate` keeps meaning what it
//! says.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use topo::{Body, EdgeKey, FaceKey, ShellNaming, VertexKey};

use super::defer::{TieRows, put as put_row, upstream_name};
use super::emit::{NamingError, check_total, ent, name1};
use super::role::{EntityKind, RoleSeg};
use super::table::{EntityKey, NameTable};
use crate::node::RecipeNodeId;

/// Names one shell result.
///
/// `target` is the shell's single operand's table (body index 0),
/// `body` the shell output, `rec` its birth record.
///
/// # Errors
///
/// [`NamingError::MissingUpstream`] when a record (or a survivor) names
/// a source entity the target's table does not carry — a wiring bug;
/// [`NamingError::Emission`] on a survivor the record retired or does
/// not list; [`NamingError::Duplicate`] on aliasing at insertion;
/// [`NamingError::Unnamed`] if the result is not covered.
pub(crate) fn name_shell<T: geom_core::Real>(
    node: RecipeNodeId,
    target_node: RecipeNodeId,
    target: &NameTable,
    body: &Body<T>,
    rec: &ShellNaming,
) -> Result<Arc<NameTable>, NamingError> {
    // Every upstream read goes through the deferral's own reader, so
    // the tie bit travels with the name it belongs to.
    let up = |key: EntityKey| upstream_name(target, target_node, ent(0, key));
    let up_f = |k: FaceKey| up(EntityKey::Face(k));
    let up_e = |k: EdgeKey| up(EntityKey::Edge(k));
    let up_v = |k: VertexKey| up(EntityKey::Vertex(k));
    let b = Box::new;

    // ---- The mints, by role. ----
    let mut minted: BTreeMap<EntityKey, (RoleSeg, bool)> = BTreeMap::new();
    let mut put = |key: EntityKey, seg: RoleSeg, tied: bool| -> Result<(), NamingError> {
        // Two records for one key is an emission bug, not a tie: the
        // doors mint each entity once.
        if minted.insert(key, (seg, tied)).is_some() {
            return Err(NamingError::Emission {
                what: "the shell recorded one entity twice",
            });
        }
        Ok(())
    };

    // The rims come first so a designated face's twin — listed in
    // `inner` and then killed by the rim surgery — cannot claim the
    // rim's key: the rim IS the designated face's own key, and the
    // twin's key is a different, retired one, so neither collides; the
    // order only makes the read the record's own.
    for rim in &rec.rims {
        let Some(first) = rim.sources.first() else {
            return Err(NamingError::Emission {
                what: "the shell recorded a rim with no designated face",
            });
        };
        let f = up_f(*first)?;
        put(
            EntityKey::Face(rim.rim),
            RoleSeg::Rim(b(f.name.clone())),
            f.tied,
        )?;
        for (j, hole) in rim.holes.iter().enumerate() {
            put(
                EntityKey::Face(hole.face),
                RoleSeg::HoleRim {
                    of: b(f.name.clone()),
                    hole: j as u32,
                },
                f.tied,
            )?;
        }
    }
    // The twins. A designated face's twin is listed here too and dies
    // in the rim surgery; it never appears in the body, so its row is
    // read and never consulted.
    for (twin, src) in &rec.inner {
        let s = up_f(*src)?;
        put(EntityKey::Face(*twin), RoleSeg::Inner(b(s.name)), s.tied)?;
    }
    for (twin, src) in &rec.inner_edges {
        let s = up_e(*src)?;
        put(EntityKey::Edge(*twin), RoleSeg::Inner(b(s.name)), s.tied)?;
    }
    for (twin, src) in &rec.inner_vertices {
        let s = up_v(*src)?;
        put(EntityKey::Vertex(*twin), RoleSeg::Inner(b(s.name)), s.tied)?;
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
    // The retired set, as a lookup: a key the construction RETIRED can
    // never be a survivor, whatever its arena says.
    let retired_f: BTreeSet<FaceKey> = rec.dead.faces.iter().copied().collect();
    let retired_e: BTreeSet<EdgeKey> = rec.dead.edges.iter().copied().collect();
    let retired_v: BTreeSet<VertexKey> = rec.dead.vertices.iter().copied().collect();
    // A surviving FACE has a row of its own to answer to: `outer`
    // lists every non-designated source face, result key first.
    let outer: BTreeSet<FaceKey> = rec.outer.iter().map(|&(result, _)| result).collect();
    for (kind, key) in rows {
        let (seg, from_tie) = match minted.get(&key) {
            Some((seg, tied)) => (seg.clone(), *tied),
            None => {
                let (dead, listed) = match key {
                    EntityKey::Face(k) => (retired_f.contains(&k), outer.contains(&k)),
                    EntityKey::Edge(k) => (retired_e.contains(&k), true),
                    EntityKey::Vertex(k) => (retired_v.contains(&k), true),
                    EntityKey::Body => (false, true),
                };
                if dead {
                    return Err(NamingError::Emission {
                        what: "an output entity is neither minted nor a survivor: its key was \
                               recorded as RETIRED, so the match is an arena coincidence",
                    });
                }
                if !listed {
                    return Err(NamingError::Emission {
                        what: "an output face is neither minted nor an `outer` row of the \
                               shell's record, so it has no provenance",
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
