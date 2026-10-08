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
//! | `rims[i].rim` | the chart's annular rim face | [`RoleSeg::Rim`] of `sources[0]`'s name — `RimNaming::sources` preserves designation order, which is what makes "the first designated face" a fact of the record |
//! | `rims[i].sources[1..]`, where live | a seamed band's other branch faces | [`RoleSeg::Rim`] of each one's own name |
//! | `rims[i].ring_edges` / `ring_vertices` | the rim's ring | rows of `inner_edges` / `inner_vertices` verbatim, so `Inner` of the boundary edge — no second role |
//! | `rims[i].holes[j].face` | a promoted hole annulus | [`RoleSeg::HoleRim`], `j` in pairing order |
//! | `dead` | nothing | nothing — a designated face's own name VANISHES |
//! | `edge_joins` | an edge the closing join made | its input edges' names: the one it covers, or a [`RoleSeg::Merged`] set of each covered edge's name by the rows above |
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
//! list to [`super::defer::narrow_into`] at once and `Duplicate` keeps
//! meaning what it says.

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
            RoleSeg::Rim(f.name.clone()),
            f.tied,
        )?;
        // A seamed band keeps every face of its chart: each designated
        // face that survives besides the first is a branch of the same
        // rim, named for its own source.
        for &branch in &rim.sources[1..] {
            if branch != rim.rim && body.get_face(branch).is_some() {
                let b = up_f(branch)?;
                put(EntityKey::Face(branch), RoleSeg::Rim(b.name), b.tied)?;
            }
        }
        for (j, hole) in rim.holes.iter().enumerate() {
            put(
                EntityKey::Face(hole.face),
                RoleSeg::HoleRim {
                    of: f.name.clone(),
                    hole: super::emit::to_u32(j, "a shell rim's hole index exceeds u32")?,
                },
                f.tied,
            )?;
        }
    }
    // The twins. A designated face's twin is listed here too and dies
    // in the rim surgery; it never appears in the body, so its row is
    // read and never consulted. On a void designation the rim IS a
    // twin, and its rim role, minted above, is the one it carries.
    let rim_faces: BTreeSet<FaceKey> = rec.rims.iter().map(|rim| rim.rim).collect();
    for (twin, src) in &rec.inner {
        if rim_faces.contains(twin) {
            continue;
        }
        let s = up_f(*src)?;
        put(EntityKey::Face(*twin), RoleSeg::Inner(s.name), s.tied)?;
    }
    for (twin, src) in &rec.inner_edges {
        let s = up_e(*src)?;
        put(EntityKey::Edge(*twin), RoleSeg::Inner(s.name), s.tied)?;
    }
    for (twin, src) in &rec.inner_vertices {
        let s = up_v(*src)?;
        put(EntityKey::Vertex(*twin), RoleSeg::Inner(s.name), s.tied)?;
    }

    // ---- The joins (`ShellNaming::edge_joins`). ----
    // The shell ends with the join, so an edge it made is named by the
    // input edges it covers, each read by the rows above: `Inner` of
    // its source for a cavity twin, the operand's own name for a
    // survivor. One covered name is the edge's own; several are a
    // `Merged` set of them. The killed vertex and the absorbed edge are
    // no longer in the body, so their rows are never consulted.
    let mut covers: BTreeMap<EdgeKey, Vec<EdgeKey>> = BTreeMap::new();
    for j in &rec.edge_joins {
        let gone = covers.remove(&j.gone).unwrap_or_else(|| vec![j.gone]);
        covers
            .entry(j.kept)
            .or_insert_with(|| vec![j.kept])
            .extend(gone);
    }
    let mut joined: BTreeMap<EntityKey, (RoleSeg, bool)> = BTreeMap::new();
    for (kept, members) in covers {
        if body.get_edge(kept).is_none() {
            return Err(NamingError::Emission {
                what: "the shell recorded a join whose kept edge is not in its body",
            });
        }
        let mut names = BTreeSet::new();
        let mut tied = false;
        for m in members {
            let (seg, t) = match minted.get(&EntityKey::Edge(m)) {
                Some((seg, t)) => (seg.clone(), *t),
                None => {
                    let u = up_e(m)?;
                    (RoleSeg::FromTarget(u.name), u.tied)
                }
            };
            tied |= t;
            names.insert(name1(EntityKind::Edge, node, seg));
        }
        let name = if names.len() == 1 {
            names.pop_first()
        } else {
            Some(super::merged::edge_set(node, names))
        };
        let Some([seg]) = name.as_ref().map(|n| n.path.as_slice()) else {
            return Err(NamingError::Emission {
                what: "a shell join's name is not one segment",
            });
        };
        joined.insert(EntityKey::Edge(kept), (seg.clone(), tied));
    }
    // The joined names replace whatever the rows gave the kept edge.
    minted.extend(joined);

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
                (RoleSeg::FromTarget(u.name), u.tied)
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
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::{
        MeridianEnd, ProfileEdgeRef, ProfileVertexRef, band, meridian_vertex,
    };
    use crate::node::StepId;
    use geom_core::Tol;
    use profile::PieceRole;

    const TARGET: RecipeNodeId = RecipeNodeId::new(0, 1);
    const NODE: RecipeNodeId = RecipeNodeId::new(0, 2);

    fn at(step: u64) -> ProfileVertexRef {
        ProfileVertexRef::Piece {
            step: StepId::new(0, step),
            role: PieceRole::RunOut,
        }
    }

    /// `body` named whole under [`TARGET`]: each entity by a distinct
    /// role.
    fn named(body: &Body<f64>) -> NameTable {
        let mut t = NameTable::new();
        t.insert(
            name1(EntityKind::Body, TARGET, RoleSeg::OutputBody),
            ent(0, EntityKey::Body),
        )
        .unwrap();
        for (i, (f, _)) in (0u64..).zip(body.faces()) {
            let piece = ProfileEdgeRef::Piece {
                step: StepId::new(0, i),
                role: PieceRole::RunOut,
            };
            t.insert(band(TARGET, piece), ent(0, EntityKey::Face(f)))
                .unwrap();
        }
        for (i, (e, _)) in (0u64..).zip(body.edges()) {
            t.insert(
                name1(EntityKind::Edge, TARGET, RoleSeg::BandRim(at(i))),
                ent(0, EntityKey::Edge(e)),
            )
            .unwrap();
        }
        for (i, (v, _)) in (0u64..).zip(body.vertices()) {
            t.insert(
                meridian_vertex(MeridianEnd::Start, TARGET, at(i)),
                ent(0, EntityKey::Vertex(v)),
            )
            .unwrap();
        }
        t
    }

    /// The D-section (the half disc of radius 0.5 on `x ≥ 0`, extruded
    /// 0.8 along `z`) with its half-cylinder cut in two along its middle
    /// ruling, each wall arc split at its `x = r` point: the body, the
    /// two wall faces, and each split arc's two halves.
    fn split_d_section() -> (Body<f64>, [FaceKey; 2], [[EdgeKey; 2]; 2]) {
        let tol = Tol::witness();
        let (r, h) = (0.5, 0.8);
        let half_disc = profile::test_support::bulge_loop(vec![
            (geom_core::Point2::new(0.0, -r), 0.0),
            (geom_core::Point2::new(0.0, r), 1.0),
        ]);
        let mut body = sweep::extrude(
            &profile::Profile::new(profile::SketchPlane::xy(), vec![half_disc])
                .validate(tol)
                .unwrap(),
            sweep::Extrusion::Distance {
                depth: h,
                side: sweep::ExtrudeSide::Along,
            },
            tol,
        )
        .unwrap()
        .body;
        let point =
            |b: &Body<f64>, v: VertexKey| b.vertex_points().find(|(k, _)| *k == v).unwrap().1;
        let arcs: Vec<EdgeKey> = body
            .edges()
            .filter(|(_, e)| {
                body.get_curve_geom(e.curve)
                    .and_then(topo::CurveGeom::certified)
                    .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }))
            })
            .map(|(k, _)| k)
            .collect();
        assert_eq!(arcs.len(), 2, "the wall's two arcs");
        let mut mids = Vec::new();
        let mut halves = Vec::new();
        for arc in arcs {
            let curve = body.get_edge(arc).unwrap().curve;
            let (t0, t1) = body
                .get_curve_geom(curve)
                .and_then(topo::CurveGeom::certified)
                .unwrap()
                .params();
            let made = body.split_edge(arc, 0.5 * (t0 + t1), tol).unwrap();
            mids.push(made.vertex);
            halves.push([arc, made.new_edge]);
        }
        let wall = body
            .faces()
            .find(|(_, f)| {
                body.get_surface(f.surface).map(geom::Surface::kind)
                    == Some(geom::SurfaceKind::Cylinder)
            })
            .unwrap()
            .0;
        let outer = body.get_face(wall).unwrap().outer;
        let leaving = |v: VertexKey| {
            body.half_edges()
                .find(|(_, h)| h.start == v && h.parent_loop == outer)
                .unwrap()
                .0
        };
        let (he1, he2) = (leaving(mids[0]), leaving(mids[1]));
        let surface = body.get_face(wall).unwrap().surface;
        let (p, q) = (point(&body, mids[0]), point(&body, mids[1]));
        let made = body
            .mef(
                topo::MefSite::Chords { he1, he2 },
                geom_brep::EdgeCurveSpec {
                    description: geom_brep::EdgeDescriptionSpec::chart(surface),
                    carrier: geom::Curve3::Line {
                        origin: p,
                        dir: (q - p) / (q - p).norm(),
                    },
                    param_start: 0.0,
                    param_end: (q - p).norm(),
                },
                topo::FaceSurface::Inherit,
                tol,
            )
            .unwrap();
        topo::mint_pcurves(&mut body, tol).unwrap();
        (body, [wall, made.face], [halves[0], halves[1]])
    }

    /// **An edge the shell's closing join made is named by the input
    /// edges it covers.** Opening both faces of the split wall merges
    /// them; each split arc's midpoint is then a vertex between two
    /// edges of one circle, on the ring and on its cavity twin, and the
    /// join takes all four away. Each joined edge is the `Merged` set of
    /// its halves' names: the operand's own on the ring, `Inner` of them
    /// on the cavity's.
    #[test]
    fn a_joined_ring_arc_is_named_as_the_set_of_its_halves() {
        let tol = Tol::witness();
        let (body, wall, halves) = split_d_section();
        let target = named(&body);
        let shelled = topo::shell_open(
            &topo::test_support::finished("the split D-section", body, tol),
            0.05,
            &wall,
            tol,
        )
        .expect("the split window opens");
        assert_eq!(
            shelled.naming.edge_joins.len(),
            4,
            "two on the ring, two on its twin"
        );
        let t = name_shell(NODE, TARGET, &target, &shelled.body, &shelled.naming)
            .expect("the shell is named");
        let names: BTreeSet<_> = shelled
            .body
            .edges()
            .map(|(e, _)| t.name_of(&ent(0, EntityKey::Edge(e))).unwrap().clone())
            .collect();
        let of = |e: EdgeKey| target.name_of(&ent(0, EntityKey::Edge(e))).unwrap().clone();
        for pair in halves {
            for role in [RoleSeg::FromTarget, RoleSeg::Inner] {
                let want = super::super::merged::edge_set(
                    NODE,
                    pair.map(|h| name1(EntityKind::Edge, NODE, role(of(h).into()))),
                );
                assert!(names.contains(&want), "the joined edge is {want:?}");
            }
        }
    }
}
