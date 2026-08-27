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
//! run in opposite senses — the book's crossover carried through);
//! that is *asserted structurally* before any surgery
//! ([`BooleanError::SeamOrientation`]) rather than assumed.
//!
//! Scaffolding carriers use the canonical full-period self-loop spec
//! ([`EdgeCurveSpec::self_loop_circle_at`]), whose endpoint-pin
//! certification *requires* the zipped vertex pairs to be bitwise
//! coincident — the pipeline's coincidences are (crossing points are
//! computed once and inserted into both bodies; ring vertices copy the
//! pierce point bitwise); anything less refuses loudly at
//! certification, never zips approximately.

use geom_core::Decide;
use slotmap::SecondaryMap;

use super::BooleanError;
use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, VertexKey};
use crate::euler::{FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;

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
    vmap: &SecondaryMap<VertexKey, VertexKey>,
    tol: Tol,
) -> Result<ZipReport, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let mut report = ZipReport::default();

    // ---- Fuse: B's section face becomes a ring of A's. ----
    let fused = body.kfmrh(a_face, b_face)?;
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

    // ---- Record-keyed alignment: rs[j] starts at vmap[start(ob[j])]. ----
    let start_of = |body: &Body<T>, he| -> Result<VertexKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or_else(|| corr("seam half-edge no longer resolves"))?
            .start)
    };
    let mut rs = Vec::with_capacity(n);
    for &b_he in &ob {
        let a_v = start_of(body, b_he)?;
        let b_v = *vmap
            .get(a_v)
            .ok_or_else(|| corr("outer seam vertex has no recorded B correspondent"))?;
        let matched = {
            let mut found = None;
            for &rhe in &ring_cycle {
                if start_of(body, rhe)? == b_v {
                    found = Some(rhe);
                    break;
                }
            }
            found.ok_or_else(|| corr("corresponding ring half-edge missing"))?
        };
        rs.push(matched);
    }

    // ---- Antiparallelism, asserted structurally: ring he at b_j must
    // run b_j → b_{j−1} (the outer runs a_j → a_{j+1}). ----
    for j in 0..n {
        let prev_a = start_of(body, ob[(j + n - 1) % n])?;
        let expect_end = *vmap
            .get(prev_a)
            .ok_or_else(|| corr("outer seam vertex has no recorded B correspondent"))?;
        let end = body
            .half_edge_end(rs[j])
            .ok_or_else(|| corr("ring half-edge has no end"))?;
        if end != expect_end {
            return Err(BooleanError::SeamOrientation { a_face, b_face });
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
        body.kev(he)?;
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
        body.kef(rs[(j + 1) % n])?;
    }
    body.kef(rs[1 % n])?;
    for &he in &ob {
        let edge = body
            .get_half_edge(he)
            .ok_or_else(|| corr("surviving seam half-edge no longer resolves"))?
            .edge;
        report.seam_edges.push(edge);
    }
    Ok(report)
}
