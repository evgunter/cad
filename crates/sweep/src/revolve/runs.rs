//! A revolve loop's runs (crate README, "Walls: one per run"): the
//! cosurface verdicts a revolve reads, and the loop with each run
//! collapsed to one segment ([`Collapsed`]), which both the full and
//! the partial builder take. The verb-neutral collapse is
//! `swept::collapse_runs`; this adds the revolve's axis classes.

use geom_core::{Band, Decide, Real};

use super::axis::LoopClasses;
use super::{RevolveError, SweptSeg, WALL_COSURFACE};
use crate::swept::Join;

/// A loop's cosurface verdicts for a revolve
/// (`swept::cosurface_pairs`): two consecutive on-axis segments are one
/// carrier, the axis itself; a pair with one on-axis side is
/// structurally false.
fn loop_pairs<T: Decide>(
    segs: &[SweptSeg<T>],
    cls: &LoopClasses<T>,
    loop_index: usize,
    band: Band,
) -> Result<Vec<bool>, RevolveError> {
    let walled = |j: usize| cls.walls[j].kind().is_some();
    let n = segs.len();
    let mut pair =
        crate::swept::cosurface_pairs(segs, walled, WALL_COSURFACE, band, |j, source| {
            RevolveError::CosurfaceEscalated {
                loop_index,
                vertex_index: segs[j].canonical_vertex,
                source,
            }
        })?;
    for (j, p) in pair.iter_mut().enumerate() {
        if !walled(j) && !walled((j + n - 1) % n) {
            *p = true;
        }
    }
    Ok(pair)
}

/// A revolve's loop with each run of segments on one carrier collapsed
/// to one (crate README, "Walls: one per run"; `swept::collapse_runs`):
/// a station inside a run has no entity in either revolve, so the
/// builders never see it. The run's segment is its first one carried
/// to the run's end (an arc's sweep summed over the run), classified as
/// the first one was — the run is one carrier by the cosurface verdict,
/// and a run of on-axis segments is the axis.
pub(super) struct Collapsed<T: Real> {
    /// The collapsed swept segments, in run order.
    pub(super) segs: Vec<SweptSeg<T>>,
    /// Their classes: each run's leading vertex and first wall.
    pub(super) cls: LoopClasses<T>,
    /// How each collapsed segment's wall meets the previous one's.
    pub(super) joins: Vec<Join>,
    /// Per collapsed segment, the canonical segments its run holds, in
    /// swept order.
    pub(super) members: Vec<Vec<usize>>,
    /// The canonical loop's segment count.
    pub(super) n_canon: usize,
}

/// Collapses one loop's runs ([`Collapsed`]).
///
/// # Errors
///
/// [`RevolveError::PinnedRunStation`] for a station on the axis inside
/// a run of walls: the wall would reach the axis and carry on past it,
/// which the half-plane checks refuse first; surfaced rather than
/// trusted.
pub(super) fn collapse_runs<T: Decide>(
    segs: &[SweptSeg<T>],
    cls: &LoopClasses<T>,
    loop_index: usize,
    band: Band,
) -> Result<Collapsed<T>, RevolveError> {
    let pair = loop_pairs(segs, cls, loop_index, band)?;
    let col = crate::swept::collapse_runs(segs, &crate::swept::joins(segs, &pair), |seg, next| {
        seg.continued(next)
    });
    for run in &col.members {
        let walled = cls.walls[run[0]].kind().is_some();
        if let Some(&s) = run[1..].iter().find(|&&s| walled && cls.verts[s].pinned) {
            return Err(RevolveError::PinnedRunStation {
                loop_index,
                vertex_index: segs[s].canonical_vertex,
            });
        }
    }
    Ok(Collapsed {
        cls: LoopClasses {
            verts: col.members.iter().map(|run| cls.verts[run[0]]).collect(),
            walls: col.members.iter().map(|run| cls.walls[run[0]]).collect(),
        },
        members: col.canonical_members(|s| segs[s].canonical_segment),
        joins: col.joins,
        segs: col.segs,
        n_canon: segs.len(),
    })
}
