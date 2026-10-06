//! The seam zip (ch. 15 §15.8 step 5, per polygon pair):
//! `kfmrh(a_face, b_face)` — the cross-shell fusion (or same-shell
//! genus form once a previous polygon already fused the shells) that
//! ch. 12's `loopglue` was promised as a second consumer for — then
//! the loopglue zip itself: per coincident vertex pair one scaffolding
//! `mekr`/`mef` + `kev`, per doubled seam edge a `kef`; the section
//! faces die and the seam becomes ordinary edges of the result.
//!
//! **Correspondence is record data** (F9): the ring half-edge matched
//! to each outer half-edge comes from the null-pair vertex map built
//! by `setopfinish` — never from geometric point matching. The two
//! cycles must be **antiparallel** (A's kept loop and B's kept loop
//! run in opposite senses — the book's crossover carried through):
//! the outer half-edge leaving `a` (for `a → a⁺`, after `a⁻ → a`) is
//! paired with the ring half-edge leaving a correspondent of `a`, which
//! runs to a correspondent of `a⁻`; the zip joins the two at `a`. So a
//! vertex the seam meets twice (a welded pinch, with one correspondent
//! per meeting) is told apart by the ring run's other end. A ring that
//! leaves the right vertex but runs the same sense is refused before
//! any surgery
//! ([`BooleanError::SeamOrientation`]) rather than zipped.
//!
//! Scaffolding carriers use the canonical full-period self-loop spec
//! ([`EdgeCurveSpec::self_loop_circle_at`]), whose endpoint-pin
//! certification *requires* the zipped vertex pairs to be bitwise
//! coincident — the pipeline's coincidences are (crossing points are
//! computed once and inserted into both bodies; ring vertices copy the
//! pierce point bitwise); anything less refuses loudly at
//! certification, never zips approximately.
//!
//! **Pinches are crossed before any zip** ([`cross_pinches`]). Where
//! the zips' fusions would join a vertex to itself (a pinch both
//! operands keep as one vertex), the op stage first splits that vertex
//! across two corners of kept faces: `mev`, then `kemr` (two corners of
//! one ring) or `kef` (two faces' corners on one surface and sense, one
//! face ringless). That rewrites kept faces' topology, not only the
//! section faces', and a `kef` absorption is reported to the op stage
//! for its `Descendants` and naming rows. The pre-pass and the zip read
//! one alignment ([`align`]) and one fusion order ([`fusion_order`]).

use std::collections::{BTreeMap, BTreeSet};

use geom_core::Decide;

use super::BooleanError;
use crate::body::Body;
use crate::entity::{EntityId, Face, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::euler_ring::MekrSite;
use crate::live::{linked, proven};
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;

/// The seam vertex correspondence: each A-side vertex → its B-side
/// correspondents. One each, except a welded pinch, which a seam meets
/// once per pierce it fused.
pub(super) type SeamCorrespondence = BTreeMap<VertexKey, BTreeSet<VertexKey>>;

/// The vertex `v` survives as through the fusions `(dead, kept)`, in
/// the order they were made: the one reading of a fusion list. Every
/// writer appends a row as its kev runs, so a key is dead from its row
/// on and no later row names it.
pub(super) fn survivor(merges: &[(VertexKey, VertexKey)], v: VertexKey) -> VertexKey {
    debug_assert!(
        fusions_well_ordered(merges),
        "a fusion row names a key an earlier row killed: {merges:?}"
    );
    merges
        .iter()
        .fold(v, |at, &(dead, kept)| if at == dead { kept } else { at })
}

/// [`survivor`], refusing a corrupt fusion list in every build: a row
/// that keeps a key an earlier row killed, kills one twice, or fuses a
/// key into itself would fold `v` onto a dead key.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] on such a list.
pub(super) fn survivor_checked(
    merges: &[(VertexKey, VertexKey)],
    v: VertexKey,
) -> Result<VertexKey, BooleanError> {
    if !fusions_well_ordered(merges) {
        return Err(BooleanError::JoinDesync {
            what: "a fusion row names a key an earlier row killed",
        });
    }
    Ok(survivor(merges, v))
}

/// Whether every fusion row's keys are live when it is made: no row
/// keeps or kills a key an earlier row killed, and none fuses a key
/// into itself.
fn fusions_well_ordered(merges: &[(VertexKey, VertexKey)]) -> bool {
    let mut dead_so_far = BTreeSet::new();
    merges.iter().all(|&(dead, kept)| {
        !dead_so_far.contains(&kept) && dead_so_far.insert(dead) && dead != kept
    })
}

/// Where a zero-length joint runs between two vertices of one face.
pub(super) enum Joint {
    /// Across two of its loops (`mekr`, which joins them): `target`'s
    /// loop absorbs `ring`'s.
    Loops {
        target: HalfEdgeKey,
        ring: HalfEdgeKey,
    },
    /// Across one loop (`mef`, which divides the face).
    Chord { he1: HalfEdgeKey, he2: HalfEdgeKey },
    /// Across one ring of `face`: the `mef` divides the hole, and the
    /// face it divides off is a hole too, so `kfmrh` returns it to
    /// `face` as a ring: two holes meeting at the vertex.
    Hole {
        face: FaceKey,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
    },
}

/// **Fuses two coincident vertices**: a zero-length edge between them at
/// `p` (the canonical self-loop carrier, whose certification requires
/// the pair bitwise coincident), collapsed by a `kev` that keeps the
/// merged fan's carriers, each re-certified at the kept vertex under the
/// run's band. Returns the fusion `(dead, kept)` and, for a chord, the
/// face it divided off (a hole's goes back to its face as a ring);
/// `desync` names a joint that no longer resolves.
pub(super) fn fuse_by_joint<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    joint: Joint,
    p: geom_core::Point3<T>,
    desync: fn(&'static str) -> BooleanError,
    tol: Tol,
) -> Result<((VertexKey, VertexKey), Option<FaceKey>), BooleanError> {
    let carrier = EdgeCurveSpec::self_loop_circle_at(p);
    let hole_face = match joint {
        Joint::Hole { face, .. } => Some(face),
        _ => None,
    };
    let (he, made) = match joint {
        Joint::Loops { target, ring } => (
            body.mekr(MekrSite::Cycles { target, ring }, carrier, tol)?
                .he_plus,
            None,
        ),
        Joint::Chord { he1, he2 } | Joint::Hole { he1, he2, .. } => {
            let made = body.mef(
                MefSite::Chords { he1, he2 },
                carrier,
                FaceSurface::Inherit,
                tol,
            )?;
            (made.he_plus, Some(made.face))
        }
    };
    let hole = hole_face.zip(made);
    let kept = body
        .get_half_edge(he)
        .ok_or_else(|| desync("a joint half-edge no longer resolves"))?
        .start;
    let dead = body
        .half_edge_end(he)
        .ok_or_else(|| desync("a joint half-edge has no end"))?;
    body.kev_describing(he, &[], tol)?;
    if let Some((face, divided)) = hole {
        body.kfmrh_minting(face, divided, tol)?;
        return Ok(((dead, kept), None));
    }
    Ok(((dead, kept), made))
}

/// What one seam zip did to the arena — the F9-style record the op
/// stage consumes (M3 PR 6a): every vertex fusion (dead key → kept
/// key, the D5 descendant map's zip rows) and the surviving seam
/// edges (the D6 description pass's worklist — tracked lineage, never
/// a post-hoc scan).
#[derive(Debug, Default)]
pub(super) struct ZipReport {
    /// Vertex fusions in zip order: `(dead, kept)` per zipped pair.
    pub vertex_merges: Vec<(VertexKey, VertexKey)>,
    /// The seam edges surviving the zip (the outer cycle's edges), in
    /// cycle order.
    pub seam_edges: Vec<crate::entity::EdgeKey>,
    /// Seam edges KILLED by this zip as R-interior structure (the
    /// already-fused runs a slit zip consumes — e.g. the meridian
    /// seams of a closed cosurface band, which are segments AND
    /// interior to the contact region). Empty for a plain
    /// [`zip_seam`].
    pub interior_edges: Vec<crate::entity::EdgeKey>,
    /// Edge fusions, `(dead, kept)`: each ring edge the zip kills and
    /// the seam edge it lay on, which keeps its key.
    pub edge_merges: Vec<(crate::entity::EdgeKey, crate::entity::EdgeKey)>,
}

/// The order the loopglue zip fuses a seam's `n` vertex pairs in:
/// pair 0 first (its `mekr` joins the two loops), then `n − 1` down to
/// 1 (each a `mef` and the `kef` of the strip behind it).
pub(crate) fn fusion_order(n: usize) -> impl Iterator<Item = usize> {
    core::iter::once(0).chain((1..n).rev())
}

/// **A pinch the zips would fuse twice is crossed first.** The zips
/// fuse each seam's vertex pairs in their order (pair 0, then `n − 1`
/// down to 1), seam after seam. A pair whose two vertices are one by
/// then (a pinch both operands keep as one vertex, met by two seams or
/// twice by one) would join the vertex to itself. Its result is one
/// vertex shared by two cones of boundary, and that is legal only where
/// a face's boundary crosses from one cone to the other there, so that
/// the vertex's orbit is one cycle. So before any zip, the vertex is
/// split across a kept face's two corners ([`split_across`]): the
/// pair's earlier fusions move to a new vertex, which the pair's own
/// zip fuses back, so the zips see a tree of fusions and the face is
/// the one that crosses. `vmap` gains each new vertex's correspondents.
/// Returns the faces a crossing merged, `(absorbed, kept)`.
///
/// # Errors
///
/// [`BooleanError::PinchUncrossed`] where no kept face can cross.
pub(super) fn cross_pinches<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    seams: &[(FaceKey, FaceKey)],
    vmap: &mut SeamCorrespondence,
    tol: Tol,
) -> Result<Vec<(FaceKey, FaceKey)>, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let sections: BTreeSet<FaceKey> = seams.iter().flat_map(|&(a, b)| [a, b]).collect();
    let outer = |body: &Body<T>, f: FaceKey| -> Result<LoopKey, BooleanError> {
        Ok(body
            .get_face(f)
            .ok_or_else(|| corr("section face no longer resolves"))?
            .outer)
    };
    let pairs: usize = seams
        .iter()
        .map(|&(a, _)| Ok(section_cycle(body, outer(body, a)?)?.len()))
        .sum::<Result<usize, BooleanError>>()?;
    let mut absorbed = Vec::new();
    'pass: for _ in 0..=pairs {
        // Each vertex's root and, per vertex, its corners already fused.
        let mut root: BTreeMap<VertexKey, VertexKey> = BTreeMap::new();
        let mut fused: BTreeMap<VertexKey, Vec<HalfEdgeKey>> = BTreeMap::new();
        let find = |root: &BTreeMap<VertexKey, VertexKey>, mut v: VertexKey| {
            while let Some(&up) = root.get(&v) {
                v = up;
            }
            v
        };
        for &(a_face, b_face) in seams {
            let ob = section_cycle(body, outer(body, a_face)?)?;
            let rs = align(
                body,
                (a_face, b_face),
                &ob,
                &section_cycle(body, outer(body, b_face)?)?,
                vmap,
            )?;
            for j in fusion_order(ob.len()) {
                let (a, b) = (start_of(body, ob[j])?, start_of(body, rs[j])?);
                let (ra, rb) = (find(&root, a), find(&root, b));
                if ra != rb {
                    root.insert(rb, ra);
                    fused.entry(a).or_default().push(ob[j]);
                    fused.entry(b).or_default().push(rs[j]);
                    continue;
                }
                let empty = Vec::new();
                let mut fresh = None;
                for (v, keep) in [(a, ob[j]), (b, rs[j])] {
                    let moving = fused.get(&v).unwrap_or(&empty);
                    if let Some(split) = split_across(body, v, keep, moving, &sections, tol)? {
                        absorbed.extend(split.absorbed);
                        fresh = Some((v, split.vertex));
                        break;
                    }
                }
                let Some((v, new)) = fresh else {
                    return Err(BooleanError::PinchUncrossed { vertex: a });
                };
                match vmap.get(&v).cloned() {
                    Some(bs) => {
                        vmap.insert(new, bs);
                    }
                    None => {
                        for bs in vmap.values_mut() {
                            if bs.contains(&v) {
                                bs.insert(new);
                            }
                        }
                    }
                }
                continue 'pass;
            }
        }
        return Ok(absorbed);
    }
    // Unreachable while the A and B keys are disjoint: a split moves
    // every earlier corner of its vertex away, so no pair collides twice
    // and at most `pairs` passes split. Refused rather than asserted, as
    // a body this door did not build could break the premise.
    Err(corr("a pinch split did not separate its pair"))
}

/// Splits `v` so that the corners `moving` leave it and `keep` stays,
/// across two corners of kept faces that part them in `v`'s orbit:
/// `mev` between the two moves `moving`'s side to a new vertex, and
/// killing the new edge crosses the two corners. Two corners of one
/// ring cross by `kemr`, which leaves two holes meeting at the point.
/// Two faces' corners on one surface, with one sense, cross by `kef`
/// where one of the faces has no ring: it dies into the other, whose
/// loop at its corner then meets itself there, the outer loop or a
/// ring (a face standing in the other's hole). A section face cannot
/// cross, as the zips kill it, and neither can an outer loop alone:
/// its halves would be a ring meeting the outer loop. `None` when no
/// corners qualify.
///
/// This is the inverse direction of `finish::pinch_site`, which joins
/// two vertices on one point across one face: across a ring (its
/// `Joint::Hole`) both leave two holes meeting at the point, one shape
/// at rest. Across an outer loop `pinch_site` divides the face, a step
/// this split does not take, so the two agree wherever both act.
///
/// `v` is a key the zip carries: its miss refuses
/// [`BooleanError::ZipCorrespondence`].
///
/// # Panics
///
/// Where a record past `v` does not resolve or its orbit does not close
/// (D2 row 4): the orbit, each member's loop and face, and `v`'s point.
/// The body is mid-zip, whose links hold by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
fn split_across<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    v: VertexKey,
    keep: HalfEdgeKey,
    moving: &[HalfEdgeKey],
    sections: &BTreeSet<FaceKey>,
    tol: Tol,
) -> Result<Option<Split>, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    if body.get_vertex(v).is_none() {
        return Err(corr("a pinch vertex no longer resolves"));
    }
    let orbit = body.vertex_orbit_linked(v);
    let at = |he: HalfEdgeKey| orbit.iter().position(|&h| h == he);
    let (Some(k), Some(ms)) = (
        at(keep),
        moving.iter().map(|&h| at(h)).collect::<Option<Vec<_>>>(),
    ) else {
        return Ok(None);
    };
    if ms.is_empty() {
        return Ok(None);
    }
    let n = orbit.len();
    // Whether position `x` lies in the run `[i, j)`, cyclically.
    let within = |x: usize, i: usize, j: usize| (x + n - i) % n < (j + n - i) % n;
    // Nothing writes the body until the search has chosen its site.
    let face_of = |he: HalfEdgeKey| -> (LoopKey, FaceKey, &Face) {
        let l = proven(&body.half_edges, he, EntityId::HalfEdge).parent_loop;
        let face = linked(
            &body.loops,
            l,
            EntityId::Loop,
            EntityId::HalfEdge(he),
            "parent_loop",
        )
        .face;
        let data = linked(&body.faces, face, EntityId::Face, EntityId::Loop(l), "face");
        (l, face, data)
    };
    let chart_of = |d: &Face| (d.surface, d.sense);
    let ringless = |d: &Face| d.rings.is_empty();
    let mut site = None;
    'search: for i in 0..n {
        let (l, face, fd) = face_of(orbit[i]);
        if sections.contains(&face) {
            continue;
        }
        for j in (0..n).filter(|&j| j != i) {
            if within(k, i, j) || !ms.iter().all(|&m| within(m, i, j)) {
                continue;
            }
            let (lj, fj, fjd) = face_of(orbit[j]);
            let crossing = if lj == l && fd.outer != l {
                Some(Crossing::OneLoop)
            } else if fj != face
                && !sections.contains(&fj)
                && (ringless(fd) || ringless(fjd))
                && chart_of(fjd) == chart_of(fd)
            {
                // `kef` kills only a ringless face, so the one that dies
                // is decided here: `face` where it is ringless, else `fj`,
                // which the test above then makes ringless. No pose
                // reached has two ringed faces of one chart at `v`; were
                // one to pass with the dying face holding a ring, `kef`
                // would refuse typed (`FaceHasRings`).
                Some(if ringless(fd) {
                    Crossing::TwoFaces {
                        dies: Half::Plus,
                        kept: fj,
                    }
                } else {
                    Crossing::TwoFaces {
                        dies: Half::Minus,
                        kept: face,
                    }
                })
            } else {
                None
            };
            if let Some(crossing) = crossing {
                site = Some((orbit[i], orbit[j], crossing));
                break 'search;
            }
        }
    }
    let Some((he1, he2, crossing)) = site else {
        return Ok(None);
    };
    // A ring through `v` three times or more is a pinch of three or
    // more holes, which only a pierce of three or more Out runs hangs;
    // its crossing is not one this split is measured to build.
    if let Crossing::OneLoop = crossing {
        let ring = face_of(he1).0;
        let holes = orbit.iter().filter(|&&h| face_of(h).0 == ring).count();
        if holes > 2 {
            return Err(BooleanError::PinchOfManyHolesInOneRing { vertex: v, holes });
        }
    }
    let p = body.resolve_vertex_point(v, crate::live::Proven);
    let made = body.mev(
        MevSite::Fan { he1, he2 },
        p,
        EdgeCurveSpec::self_loop_circle_at(p),
        tol,
    )?;
    let merged = match crossing {
        Crossing::OneLoop => {
            body.kemr(made.he_plus, made.he_minus)?;
            None
        }
        Crossing::TwoFaces { dies, kept } => {
            // `he_plus` lies in `he1`'s loop and `he_minus` in `he2`'s,
            // and `kef` kills the face of the half it is given.
            let he = match dies {
                Half::Plus => made.he_plus,
                Half::Minus => made.he_minus,
            };
            Some((body.kef_minting(he, tol)?.killed_face, kept))
        }
    };
    Ok(Some(Split {
        vertex: made.vertex,
        absorbed: merged,
    }))
}

/// A pinch split: the new vertex, and the face a `kef` crossing
/// absorbed with the one it kept.
struct Split {
    vertex: VertexKey,
    absorbed: Option<(FaceKey, FaceKey)>,
}

/// How the edge a pinch split mints is killed, crossing two corners.
#[derive(Clone, Copy)]
enum Crossing {
    /// Both corners are one loop's: `kemr` splits the loop in two.
    OneLoop,
    /// The corners are two faces' of one surface and sense: `kef` kills
    /// the ringless one, the face of the minted edge's half `dies`, into
    /// `kept`.
    TwoFaces { dies: Half, kept: FaceKey },
}

/// A half of the edge a pinch split mints: `Plus` in the first
/// corner's loop, `Minus` in the second's.
#[derive(Clone, Copy)]
enum Half {
    Plus,
    Minus,
}

/// Zips one section-face pair (module docs).
pub(super) fn zip_seam<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    a_face: FaceKey,
    b_face: FaceKey,
    vmap: &SeamCorrespondence,
    tol: Tol,
) -> Result<ZipReport, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let mut report = ZipReport::default();

    // ---- Fuse: B's section face becomes a ring of A's. ----
    let fused = body.kfmrh_minting(a_face, b_face, tol)?;
    let ring = fused.ring;
    let outer = body
        .get_face(a_face)
        .ok_or_else(|| corr("A section face no longer resolves"))?
        .outer;

    let ob = section_cycle(body, outer)?;
    let rs = align(
        body,
        (a_face, b_face),
        &ob,
        &section_cycle(body, ring)?,
        vmap,
    )?;
    let n = ob.len();
    let ring_edges = rs
        .iter()
        .map(|&r| {
            body.get_half_edge(r)
                .map(|h| h.edge)
                .ok_or_else(|| corr("ring half-edge no longer resolves"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // ---- The loopglue zip (the reassembly-oracle sequence, driven by
    // records): pair 0 via mekr (kills the ring loop) + kev; pairs
    // n−1 … 1 via mef + kev + kef(rs[j+1 mod n]); final kef(rs[1]). ----
    let point_of =
        |body: &Body<T>, he: HalfEdgeKey| -> Result<geom_core::Point3<T>, BooleanError> {
            let v = start_of(body, he)?;
            body.get_vertex(v)
                .and_then(|vd| body.get_point(vd.point).copied())
                .ok_or_else(|| corr("seam vertex has no point"))
        };
    for j in fusion_order(n) {
        let pj = point_of(body, ob[j])?;
        let joint = if j == 0 {
            Joint::Loops {
                target: ob[0],
                ring: rs[0],
            }
        } else {
            Joint::Chord {
                he1: ob[j],
                he2: rs[j],
            }
        };
        let (merge, _) = fuse_by_joint(body, joint, pj, corr, tol)?;
        report.vertex_merges.push(merge);
        if j != 0 {
            body.kef_minting(rs[(j + 1) % n], tol)?;
        }
    }
    body.kef_minting(rs[1 % n], tol)?;
    for &he in &ob {
        let edge = body
            .get_half_edge(he)
            .ok_or_else(|| corr("surviving seam half-edge no longer resolves"))?
            .edge;
        report.seam_edges.push(edge);
    }
    // `rs[j]` runs between the correspondents of `ob[j]`'s start and
    // `ob[j - 1]`'s (`align`), so it lies on `ob[j - 1]`'s segment.
    report.edge_merges = ring_edges
        .into_iter()
        .enumerate()
        .map(|(j, dead)| (dead, report.seam_edges[(j + n - 1) % n]))
        .collect();
    Ok(report)
}

/// The half-edge cycle of a section face's loop `l`.
fn section_cycle<T: Decide>(body: &Body<T>, l: LoopKey) -> Result<Vec<HalfEdgeKey>, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let LoopBoundary::Cycle { first } = body
        .get_loop(l)
        .ok_or_else(|| corr("section loop no longer resolves"))?
        .boundary
    else {
        return Err(corr("section loop is empty"));
    };
    body.loop_cycle(first)
        .ok_or_else(|| corr("section loop not walkable"))
}

/// The record-keyed alignment, antiparallel: `ob[j]` runs `a_j →
/// a_{j+1}`, so the ring half-edge paired with it runs from a
/// correspondent of `a_j` to one of `a_{j−1}`.
fn align<T: Decide>(
    body: &Body<T>,
    (a_face, b_face): (FaceKey, FaceKey),
    ob: &[HalfEdgeKey],
    ring_cycle: &[HalfEdgeKey],
    vmap: &SeamCorrespondence,
) -> Result<Vec<HalfEdgeKey>, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let n = ob.len();
    if ring_cycle.len() != n {
        return Err(corr("seam cycles differ in length"));
    }
    let correspondents = |v: VertexKey| {
        vmap.get(&v)
            .ok_or_else(|| corr("outer seam vertex has no recorded B correspondent"))
    };
    let mut rs: Vec<HalfEdgeKey> = Vec::with_capacity(n);
    for j in 0..n {
        let from = correspondents(start_of(body, ob[j])?)?;
        let to = correspondents(start_of(body, ob[(j + n - 1) % n])?)?;
        let mut leaves = false;
        let mut matched = None;
        for &rhe in ring_cycle {
            if !from.contains(&start_of(body, rhe)?) {
                continue;
            }
            leaves = true;
            let end = body
                .half_edge_end(rhe)
                .ok_or_else(|| corr("ring half-edge has no end"))?;
            if to.contains(&end) && matched.replace(rhe).is_some() {
                return Err(corr("a seam half-edge has two ring matches"));
            }
        }
        match matched {
            Some(rhe) if rs.contains(&rhe) => {
                return Err(corr("a ring half-edge matches two seam half-edges"));
            }
            Some(rhe) => rs.push(rhe),
            None if leaves => return Err(BooleanError::SeamOrientation { a_face, b_face }),
            None => return Err(corr("corresponding ring half-edge missing")),
        }
    }
    Ok(rs)
}

/// The vertex a half-edge starts at.
fn start_of<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<VertexKey, BooleanError> {
    Ok(body
        .get_half_edge(he)
        .ok_or(BooleanError::ZipCorrespondence {
            what: "seam half-edge no longer resolves",
        })?
        .start)
}

/// **`split_across`: a stale pinch vertex refuses typed; a torn loop
/// past its orbit panics**, where it refused `ZipCorrespondence`.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use super::*;
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    #[test]
    fn a_stale_vertex_refuses_and_a_torn_loop_panics() {
        let tol = Tol::witness();
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(tol).body;
        let v = body.vertices().next().map(|(k, _)| k).unwrap();
        let orbit = body.vertex_orbit_linked(v);
        let (keep, moving) = (orbit[0], vec![orbit[1]]);
        let none = BTreeSet::new();
        assert!(
            split_across(&mut body.clone(), v, keep, &moving, &none, tol).is_ok(),
            "the sound corner answers"
        );
        let mut stale = body.clone();
        stale.vertices.remove(v);
        assert!(
            matches!(
                split_across(&mut stale, v, keep, &moving, &none, tol),
                Err(BooleanError::ZipCorrespondence { .. })
            ),
            "a pinch vertex that does not resolve refuses typed"
        );
        // A member's face, its loop kept: no step of the orbit reads it.
        let mut torn = body.clone();
        let l = torn.get_half_edge(keep).unwrap().parent_loop;
        let face = torn.get_loop(l).unwrap().face;
        torn.faces.remove(face);
        let named = format!(
            "{}'s face names {}",
            crate::entity::EntityId::Loop(l),
            crate::entity::EntityId::Face(face)
        );
        assert_torn_op_panics(
            "split_across (face)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| split_across(b, v, keep, &moving, &none, tol).map(|s| s.is_some()),
        );
        let lost = body.get_half_edge(keep).unwrap().parent_loop;
        body.loops.remove(lost);
        let named = format!(
            "'s parent_loop names {}",
            crate::entity::EntityId::Loop(lost)
        );
        assert_torn_op_panics(
            "split_across",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| split_across(b, v, keep, &moving, &none, tol).map(|s| s.is_some()),
        );
    }
}
