//! Null-edge insertion: an explicit **transition-pair run scan in
//! worklist form** — the structural sidestep of the Program 14.7
//! head-rebinding erratum (F12): runs are enumerated up front from the
//! frozen entry array, then executed one `mev_null` each; nothing is
//! rebound implicitly, and ≥2 disjoint ABOVE-runs are just more
//! worklist items.
//!
//! Per run, the fan site is resolved **at execution time**: `he1` = the
//! run's first real half-edge (its own move — still at the base
//! vertex), `he2` = the orbit successor of the run's last half-edge
//! `next(mate(last))` *in the current body* — robust against earlier
//! runs' splices (a previously moved run or minted null-edge half may
//! be the successor; the exclusive bound is whatever the orbit holds
//! now, which is exactly "everything in this run and nothing else").
//! A run with no real half-edge (a lone wide-sector bisector duplicate
//! — both bounding edges below, interior crossing above) becomes the
//! **dangling** null edge: the strut site `Fan { he, he }` at the
//! sector's CW-next half-edge, splicing the strut inside that sector's
//! corner.
//!
//! Orientation is **data** (F9): the minted copy takes the ABOVE run,
//! so `mev_null(..., NewVertexSide::Above)` records
//! `NullEdge { below_end: old vertex, above_end: copy }` — no he1/he2
//! slot convention anywhere. The copy's side verdict is cached ON
//! (bitwise-coincident point: structural coincidence).

use slotmap::SecondaryMap;

use super::{NullEdgeRecord, PlaneSide, SectorEntry, SectorEntryKind, SplitReduceError};
use crate::body::Body;
use crate::entity::VertexKey;
use crate::euler::MevSite;
use crate::null::{NewVertexSide, NullEdge};

/// A maximal cyclic run of ABOVE entries, by entry index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct AboveRun {
    /// Index of the run's first entry.
    pub start: usize,
    /// Number of entries in the run (≥ 1).
    pub len: usize,
}

/// Enumerates the maximal cyclic ABOVE runs of a fully reclassified
/// entry array, in deterministic order (first run after the anchor
/// BELOW entry with the lowest index). A one-sided neighborhood (no
/// ABOVE, or no BELOW — a vertex the plane merely touches) has no runs:
/// nothing to separate, no null edge (ch. 14 §14.6).
pub(super) fn above_runs(entries: &[SectorEntry]) -> Vec<AboveRun> {
    let n = entries.len();
    let Some(anchor) = entries.iter().position(|e| e.class == PlaneSide::Below) else {
        return Vec::new(); // no BELOW anywhere: one-sided
    };
    let mut runs = Vec::new();
    let mut k = 0;
    while k < n {
        let idx = (anchor + 1 + k) % n;
        if entries[idx].class == PlaneSide::Above {
            let start = idx;
            let mut len = 0;
            while k < n && entries[(anchor + 1 + k) % n].class == PlaneSide::Above {
                len += 1;
                k += 1;
            }
            runs.push(AboveRun { start, len });
        } else {
            k += 1;
        }
    }
    runs
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::entity::HalfEdgeKey;

    fn entries(classes: &[PlaneSide]) -> Vec<SectorEntry> {
        classes
            .iter()
            .map(|&class| SectorEntry {
                he: HalfEdgeKey::default(),
                kind: SectorEntryKind::Edge,
                class,
            })
            .collect()
    }

    use PlaneSide::{Above as A, Below as B};

    /// One-sided neighborhoods produce no runs (tangent vertex — no
    /// surgery); mixed neighborhoods produce one run per maximal ABOVE
    /// stretch, including the wrap-around and ≥2 disjoint runs
    /// (Fig. 14.9 / the Program 14.7 erratum case).
    #[test]
    fn run_enumeration() {
        assert_eq!(above_runs(&entries(&[A, A, A])), vec![]);
        assert_eq!(above_runs(&entries(&[B, B])), vec![]);
        // Single run, wrapping the seam: entries 3,0 are one run.
        assert_eq!(
            above_runs(&entries(&[A, B, B, A])),
            vec![AboveRun { start: 3, len: 2 }]
        );
        // Two disjoint runs around one vertex.
        assert_eq!(
            above_runs(&entries(&[A, B, A, A, B, B])),
            vec![AboveRun { start: 2, len: 2 }, AboveRun { start: 0, len: 1 },]
        );
        // Three runs, each a single entry.
        assert_eq!(above_runs(&entries(&[A, B, A, B, A, B])).len(), 3);
    }
}

/// Executes the insertion worklist (module docs): one null edge per
/// run, F9 attributes recorded, the copy cached ON.
pub(super) fn insert_null_edges<T: geom_core::Decide>(
    body: &mut Body<T>,
    vertex: VertexKey,
    entries: &[SectorEntry],
    runs: &[AboveRun],
    sides: &mut SecondaryMap<VertexKey, PlaneSide>,
    records: &mut Vec<NullEdgeRecord>,
) -> Result<(), SplitReduceError> {
    let n = entries.len();
    for run in runs {
        let members = (0..run.len).map(|j| &entries[(run.start + j) % n]);
        let mut real = members.filter(|e| e.kind == SectorEntryKind::Edge);
        let first = real.next();
        let last = real.next_back().or(first);
        let (site, dangling) = match (first, last) {
            (Some(first), Some(last)) => {
                // he2 at execution time: the current orbit successor of
                // the run's last half-edge (module docs).
                let corrupt = SplitReduceError::CorruptOperand { vertex };
                let mate = body.mate(last.he).ok_or(corrupt)?;
                let he2 = body
                    .get_half_edge(mate)
                    .ok_or(SplitReduceError::CorruptOperand { vertex })?
                    .next;
                (MevSite::Fan { he1: first.he, he2 }, false)
            }
            // Dup-only run: the dangling strut inside the wide sector.
            // The entry after a bisector duplicate is always the next
            // real orbit edge; the strut splices immediately before it,
            // i.e. inside the sector's corner.
            _ => {
                let after = &entries[(run.start + run.len) % n];
                (
                    MevSite::Fan {
                        he1: after.he,
                        he2: after.he,
                    },
                    true,
                )
            }
        };
        let created = body.mev_null(site, NewVertexSide::Above)?;
        sides.insert(created.vertex, PlaneSide::On);
        records.push(NullEdgeRecord {
            at_vertex: vertex,
            edge: created.edge,
            attr: NullEdge {
                below_end: vertex,
                above_end: created.vertex,
            },
            dangling,
        });
    }
    Ok(())
}
