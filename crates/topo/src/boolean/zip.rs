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

use std::collections::{BTreeMap, BTreeSet};

use geom_core::Decide;

use super::BooleanError;
use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, VertexKey};
use crate::euler::{FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
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
        {
            let mut dead_so_far = BTreeSet::new();
            merges.iter().all(|&(dead, kept)| {
                !dead_so_far.contains(&kept) && dead_so_far.insert(dead) && dead != kept
            })
        },
        "a fusion row names a key an earlier row killed: {merges:?}"
    );
    merges
        .iter()
        .fold(v, |at, &(dead, kept)| if at == dead { kept } else { at })
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
}

/// **Fuses two coincident vertices**: a zero-length edge between them at
/// `p` (the canonical self-loop carrier, whose certification requires
/// the pair bitwise coincident), collapsed by a `kev` that keeps the
/// merged fan's carriers, each re-certified at the kept vertex under the
/// run's band. Returns the fusion `(dead, kept)` and, for a chord, the
/// face it divided off; `desync` names a joint that no longer resolves.
pub(super) fn fuse_by_joint<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    joint: Joint,
    p: geom_core::Point3<T>,
    desync: fn(&'static str) -> BooleanError,
    tol: Tol,
) -> Result<((VertexKey, VertexKey), Option<FaceKey>), BooleanError> {
    let carrier = EdgeCurveSpec::self_loop_circle_at(p);
    let (he, made) = match joint {
        Joint::Loops { target, ring } => (
            body.mekr(MekrSite::Cycles { target, ring }, carrier, tol)?
                .he_plus,
            None,
        ),
        Joint::Chord { he1, he2 } => {
            let made = body.mef(
                MefSite::Chords { he1, he2 },
                carrier,
                FaceSurface::Inherit,
                tol,
            )?;
            (made.he_plus, Some(made.face))
        }
    };
    let kept = body
        .get_half_edge(he)
        .ok_or_else(|| desync("a joint half-edge no longer resolves"))?
        .start;
    let dead = body
        .half_edge_end(he)
        .ok_or_else(|| desync("a joint half-edge has no end"))?;
    body.kev_describing(he, &[], tol)?;
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

    let cycle_of = |body: &Body<T>, l| -> Result<Vec<HalfEdgeKey>, BooleanError> {
        let LoopBoundary::Cycle { first } = body
            .get_loop(l)
            .ok_or_else(|| corr("section loop no longer resolves"))?
            .boundary
        else {
            return Err(corr("section loop is empty"));
        };
        body.loop_cycle(first)
            .ok_or_else(|| corr("section loop not walkable"))
    };
    let ob = cycle_of(body, outer)?;
    let ring_cycle = cycle_of(body, ring)?;
    let n = ob.len();
    if ring_cycle.len() != n {
        return Err(corr("seam cycles differ in length"));
    }

    // ---- Record-keyed alignment, antiparallel: ob[j] runs a_j →
    // a_{j+1}, so rs[j] runs from a correspondent of a_j to one of
    // a_{j−1}. ----
    let start_of = |body: &Body<T>, he| -> Result<VertexKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or_else(|| corr("seam half-edge no longer resolves"))?
            .start)
    };
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
        for &rhe in &ring_cycle {
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
    let p0 = point_of(body, ob[0])?;
    let joint = Joint::Loops {
        target: ob[0],
        ring: rs[0],
    };
    let (merge, _) = fuse_by_joint(body, joint, p0, corr, tol)?;
    report.vertex_merges.push(merge);
    for j in (1..n).rev() {
        let pj = point_of(body, ob[j])?;
        let joint = Joint::Chord {
            he1: ob[j],
            he2: rs[j],
        };
        let (merge, _) = fuse_by_joint(body, joint, pj, corr, tol)?;
        report.vertex_merges.push(merge);
        body.kef_minting(rs[(j + 1) % n], tol)?;
    }
    body.kef_minting(rs[1 % n], tol)?;
    for &he in &ob {
        let edge = body
            .get_half_edge(he)
            .ok_or_else(|| corr("surviving seam half-edge no longer resolves"))?
            .edge;
        report.seam_edges.push(edge);
    }
    Ok(report)
}
