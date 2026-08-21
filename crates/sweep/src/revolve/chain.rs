//! The meridian chain a revolve lays down before it sweeps: one
//! `mev` per swept position followed by the closing `mef`, all edges
//! carrying the shared lowering's `PlacedSegment` spec.
//!
//! Both revolve cases build it and both build it identically — the
//! wedge's start lamina, each of that lamina's hole rings, and the
//! full revolve's seam lamina differ only in which loop they grow from
//! and what surface the closing face receives. Those two are
//! parameters here, so the operator sequence itself exists once.

use geom_core::{Decide, Point3};
use topo::{
    Body, EulerOpError, FaceKey, FaceSurface, HalfEdgeKey, LoopKey, MefSite, MevSite, VertexKey,
};

use super::SweptSeg;
use super::axis::AxisFrame;
use crate::swept::placed_segment_spec;
use geom_core::Tol;

/// What one meridian chain left behind.
pub(super) struct MeridianChain {
    /// Chain half-edges in swept order: `hes[j]` runs swept position
    /// `j` → `j + 1`, carrying segment `j`, the closing `mef`'s
    /// half-edge last.
    pub(super) hes: Vec<HalfEdgeKey>,
    /// The chain's vertex per swept position: the anchor, then each
    /// `mev`'s new vertex. An on-axis position is the wedge's pole —
    /// the rotation fixes it, so that ONE vertex serves both chains.
    pub(super) verts: Vec<VertexKey>,
    /// The face the closing `mef` minted.
    pub(super) face: FaceKey,
}

/// Grows `segs` as a closed chain in `r#loop`, anchored at `anchor`
/// (the loop's only vertex — an `mvfs` seed or a bridge tip), and
/// closes it with an `mef` carrying `cap`. Every meridian chain is
/// laid down at the sketch placement: the wedge's end chain is not
/// built here but swept from this one, and a full period is
/// definitionally the identity.
///
/// The only thing that can go wrong here is an operator fault, so that
/// is what it returns; each caller's `?` lifts it through its own
/// `From<EulerOpError>`.
#[allow(clippy::too_many_arguments)] // the 8th is the run-tolerance witness, not a duty of its own
pub(super) fn build_chain<T: Decide>(
    body: &mut Body<T>,
    frame: &AxisFrame<T>,
    r#loop: LoopKey,
    anchor: VertexKey,
    segs: &[SweptSeg<T>],
    qs: &[Point3<T>],
    cap: FaceSurface<T>,
    tol: Tol,
) -> Result<MeridianChain, EulerOpError> {
    let (place, normal) = (frame.place, frame.n3);
    let n = segs.len();
    let mut hes = Vec::with_capacity(n);
    let first = body.mev(
        MevSite::Lone { r#loop },
        qs[1 % n],
        placed_segment_spec(&segs[0], place, normal, qs[0], qs[1 % n]),
        tol,
    )?;
    hes.push(first.he_plus);
    let mut verts = vec![anchor, first.vertex];
    let mut prev = first;
    for j in 2..n {
        let m = body.mev(
            MevSite::Fan {
                he1: prev.he_minus,
                he2: prev.he_minus,
            },
            qs[j],
            placed_segment_spec(&segs[j - 1], place, normal, qs[j - 1], qs[j]),
            tol,
        )?;
        hes.push(m.he_plus);
        verts.push(m.vertex);
        prev = m;
    }
    let close = body.mef(
        MefSite::Chords {
            he1: prev.he_minus,
            he2: first.he_plus,
        },
        placed_segment_spec(&segs[n - 1], place, normal, qs[n - 1], qs[0]),
        cap,
        tol,
    )?;
    hes.push(close.he_plus);
    Ok(MeridianChain {
        hes,
        verts,
        face: close.face,
    })
}
