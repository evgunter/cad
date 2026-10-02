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
//! the outer half-edge `a → a'` matches the ring half-edge running
//! from a correspondent of `a'` to one of `a`, so a vertex the seam
//! meets twice (a welded pinch, with one correspondent per meeting)
//! is told apart by the run's other end. A ring that runs the same
//! sense is refused before any surgery
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

/// The vertex `v` survives as through the fusions `(dead, kept)`,
/// in the order they were made.
pub(super) fn survivor(merges: &[(VertexKey, VertexKey)], v: VertexKey) -> VertexKey {
    merges
        .iter()
        .fold(v, |at, &(dead, kept)| if at == dead { kept } else { at })
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
pub(super) fn zip_seam<T: Decide>(
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
    let record_kev = |body: &mut Body<T>,
                      he: crate::entity::HalfEdgeKey,
                      report: &mut ZipReport|
     -> Result<(), BooleanError> {
        let kept = body
            .get_half_edge(he)
            .ok_or_else(|| corr("kev half-edge no longer resolves"))?
            .start;
        let dead = body
            .half_edge_end(he)
            .ok_or_else(|| corr("kev half-edge has no end"))?;
        // A merge of two vertices the section put a band apart (they
        // can differ by ulps): the merged fan keeps its carriers, each
        // re-certified at the kept vertex under the run's band.
        body.kev_describing(he, &[], tol)?;
        report.vertex_merges.push((dead, kept));
        Ok(())
    };
    let p0 = point_of(body, ob[0])?;
    let n0 = body.mekr(
        MekrSite::Cycles {
            target: ob[0],
            ring: rs[0],
        },
        EdgeCurveSpec::self_loop_circle_at(p0),
        tol,
    )?;
    record_kev(body, n0.he_plus, &mut report)?;
    for j in (1..n).rev() {
        let pj = point_of(body, ob[j])?;
        let nj = body.mef(
            MefSite::Chords {
                he1: ob[j],
                he2: rs[j],
            },
            EdgeCurveSpec::self_loop_circle_at(pj),
            FaceSurface::Inherit,
            tol,
        )?;
        record_kev(body, nj.he_plus, &mut report)?;
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
