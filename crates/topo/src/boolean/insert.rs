//! Paired null-edge insertion (Programs 15.11/15.12 motion, F9/F12).
//!
//! Surviving records — each a section-polygon edge germ with one In and
//! one Out code per side — are paired **consecutively in A-major
//! order** (the book's consumption), and each pair mints one null edge
//! in each solid spanning the orbit run between the two germs, with:
//!
//! - **F9 attributes as data**: the run's side (the shared code between
//!   the paired records) decides the minted copy's side —
//!   `NewVertexSide::Below` for an In-run (below ≙ IN), `Above` for an
//!   Out-run — derived from the F3 chain, never a he1/he2 slot.
//! - **Explicit cross-body correspondence keys**
//!   ([`super::NullEdgePairRecord`]): the A-edge and B-edge of a pair
//!   are tied by key, never by array position (`ssortnulledges` is
//!   engineered out).
//! - **The 15.11 consecutive-pairing invariant, GUARDED (F12)**: the
//!   book consumes surviving records two at a time and never argues
//!   that A-consecutive pairs are also B-consecutive for > 2 crossings.
//!   We check it: each pair must be cyclically adjacent among survivors
//!   in B-order too, and the run-side codes must agree at both ends
//!   (`r.own_start_code == r'.own_end_code` per solid). Violation ⇒
//!   typed [`super::BooleanError::PairingMismatch`] — fail-loud, never
//!   a mis-joined seam. The 4-crossing stress fixtures pin the passing
//!   cases.
//!
//! Run extraction: a germ in sector `k` transitions that sector's codes
//! `end → start` walking the array forward (array order follows the
//! orbit; the forward-crossed bound is the sector's START). The run
//! from germ r to germ r′ therefore spans sectors `r.own+1 ..= r′.own`,
//! whose orbit half-edges (deduplicated across subdivision twins) form
//! the `mev_null` fan; an empty span (both germs in one sector) is the
//! strut/dangling case (`Fan { he, he }` at the next sector's edge).

use geom_core::Decide;

use super::sectors::{BoolSector, PairRecord};
use super::{
    BoolNullEdgeRecord, BooleanError, NullEdgePairRecord, Operand, PairSite, SideCode, VvContact,
};
use crate::body::Body;
use crate::entity::{HalfEdgeKey, VertexKey};
use crate::euler::MevSite;
use crate::null::{NewVertexSide, NullEdge};

/// Output of one vertex-pair insertion.
#[derive(Debug)]
pub(super) struct InsertOut {
    /// Minted edges, both operands.
    pub edges: Vec<BoolNullEdgeRecord>,
    /// The correspondence pairs.
    pub pairs: Vec<NullEdgePairRecord>,
}

/// Validates codes, pairs survivors, mints the null edges in both
/// solids (module docs).
#[allow(clippy::too_many_arguments)]
pub(super) fn insert_null_pairs<T: Decide>(
    a_body: &mut Body<T>,
    b_body: &mut Body<T>,
    contact: VvContact,
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    records: &[PairRecord],
) -> Result<InsertOut, BooleanError> {
    let survivors: Vec<&PairRecord> = records.iter().filter(|r| r.intersect).collect();
    let mut out = InsertOut {
        edges: Vec::new(),
        pairs: Vec::new(),
    };
    if survivors.is_empty() {
        return Ok(out); // touching without crossing: 3′ contact only
    }
    if survivors.len() % 2 != 0 {
        return Err(BooleanError::ClassificationInvariant {
            what: "odd number of surviving crossing records at a vertex pair",
        });
    }
    for r in &survivors {
        let clean = |c: (SideCode, SideCode)| {
            (c.0 == SideCode::In && c.1 == SideCode::Out)
                || (c.0 == SideCode::Out && c.1 == SideCode::In)
        };
        if !clean(r.sa) || !clean(r.sb) {
            return Err(BooleanError::ClassificationInvariant {
                what: "surviving record without one In and one Out code per side",
            });
        }
    }
    // Survivors arrive in creation order = A-major; pair consecutively
    // (cyclically, starting at the first survivor).
    let mismatch = || BooleanError::PairingMismatch {
        a_vertex: contact.a,
        b_vertex: contact.b,
    };
    // B-order of survivors for the adjacency guard.
    let mut b_order: Vec<usize> = (0..survivors.len()).collect();
    b_order.sort_by_key(|&i| (survivors[i].b, survivors[i].a));
    let b_pos = |i: usize| b_order.iter().position(|&j| j == i).unwrap_or(usize::MAX);

    for pair_idx in 0..survivors.len() / 2 {
        let (i0, i1) = (2 * pair_idx, 2 * pair_idx + 1);
        let (r0, r1) = (survivors[i0], survivors[i1]);
        // F12 guard 1: B-cyclic adjacency of the pair among survivors.
        let (p0, p1) = (b_pos(i0), b_pos(i1));
        let n = survivors.len();
        let adjacent = (p0 + 1) % n == p1 || (p1 + 1) % n == p0;
        if !adjacent {
            return Err(mismatch());
        }
        // F12 guard 2 + run sides: the forward run r0 → r1 in A has
        // side r0.sa.0 (the code after crossing r0's germ) and must be
        // approached by r1 as its end code.
        let a_side_run = r0.sa.0;
        if r1.sa.1 != a_side_run {
            return Err(mismatch());
        }
        // In B the pair is adjacent; the forward run goes from the
        // B-earlier record to the B-later one.
        let (br0, br1) = if p1 == (p0 + 1) % n { (r0, r1) } else { (r1, r0) };
        let b_side_run = br0.sb.0;
        if br1.sb.1 != b_side_run {
            return Err(mismatch());
        }

        let a_rec = mint_run(
            a_body,
            Operand::A,
            contact.a,
            a_sectors,
            r0.a,
            r1.a,
            a_side_run,
        )?;
        let b_rec = mint_run(
            b_body,
            Operand::B,
            contact.b,
            b_sectors,
            br0.b,
            br1.b,
            b_side_run,
        )?;
        out.pairs.push(NullEdgePairRecord {
            a_edge: a_rec.edge,
            b_edge: b_rec.edge,
            site: PairSite::VertexVertex(contact),
        });
        out.edges.push(a_rec);
        out.edges.push(b_rec);
    }
    Ok(out)
}

/// Mints one null edge spanning the run from the germ in sector `from`
/// (exclusive) through sector `to` (inclusive): fan half-edges =
/// deduplicated orbit edges of sectors `from+1 ..= to`; empty span ⇒
/// dangling strut inside sector `from`'s corner.
fn mint_run<T: Decide>(
    body: &mut Body<T>,
    operand: Operand,
    vertex: VertexKey,
    sectors: &[BoolSector<T>],
    from: usize,
    to: usize,
    run_side: SideCode,
) -> Result<BoolNullEdgeRecord, BooleanError> {
    let n = sectors.len();
    // Both germs inside ONE sector ⇒ the dangling strut (no orbit edge
    // moves; splice inside the corner, before the next sector's edge).
    let (site, dangling) = if from == to {
        let he = sectors[(from + 1) % n].he;
        (MevSite::Fan { he1: he, he2: he }, true)
    } else {
        let mut hes: Vec<HalfEdgeKey> = Vec::new();
        let mut k = (from + 1) % n;
        loop {
            let he = sectors[k].he;
            if hes.last() != Some(&he) {
                hes.push(he);
            }
            if k == to {
                break;
            }
            k = (k + 1) % n;
            if k == (from + 1) % n {
                return Err(BooleanError::ClassificationInvariant {
                    what: "run walk wrapped without reaching its closing germ",
                });
            }
        }
        // Wrap-around dedup (first == last can only happen via
        // subdivision twins at the seam).
        if hes.len() > 1 && hes.first() == hes.last() {
            hes.pop();
        }
        let first = *hes.first().ok_or(BooleanError::ClassificationInvariant {
            what: "empty non-strut run",
        })?;
        let last = *hes.last().unwrap_or(&first);
        // he2 at execution time: current orbit successor of the run's
        // last half-edge (PR 2's pattern — robust against prior
        // splices).
        let mate = body
            .mate(last)
            .ok_or(BooleanError::CorruptOperand { operand, vertex })?;
        let he2 = body
            .get_half_edge(mate)
            .ok_or(BooleanError::CorruptOperand { operand, vertex })?
            .next;
        (MevSite::Fan { he1: first, he2 }, false)
    };
    // The copy takes the run; its side is the run's side (F3-derived).
    let new_side = match run_side {
        SideCode::In => NewVertexSide::Below,
        SideCode::Out => NewVertexSide::Above,
        SideCode::On => {
            return Err(BooleanError::ClassificationInvariant {
                what: "run with On side reached insertion",
            });
        }
    };
    let created = body.mev_null(site, new_side)?;
    let attr = match new_side {
        NewVertexSide::Below => NullEdge {
            below_end: created.vertex,
            above_end: vertex,
        },
        NewVertexSide::Above => NullEdge {
            below_end: vertex,
            above_end: created.vertex,
        },
    };
    Ok(BoolNullEdgeRecord {
        operand,
        at_vertex: vertex,
        edge: created.edge,
        attr,
        dangling,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// The survivor-validation invariants: odd counts and dirty codes
    /// refuse loudly (unit-level; the geometric paths are pinned by the
    /// acceptance fixtures).
    #[test]
    fn survivor_validation() {
        let mk = |sa, sb| PairRecord {
            a: 0,
            b: 0,
            sa,
            sb,
            intersect: true,
        };
        let mut a = crate::fixtures::ops_cube().body;
        let mut b = crate::fixtures::ops_cube().body;
        let contact = VvContact {
            a: VertexKey::default(),
            b: VertexKey::default(),
        };
        use SideCode::{In, Out};
        let recs = vec![mk((In, Out), (In, Out))];
        let err =
            insert_null_pairs(&mut a, &mut b, contact, &[], &[], &recs).unwrap_err();
        assert!(matches!(err, BooleanError::ClassificationInvariant { .. }));
        let recs = vec![
            mk((In, In), (In, Out)),
            mk((Out, In), (Out, In)),
        ];
        let err =
            insert_null_pairs(&mut a, &mut b, contact, &[], &[], &recs).unwrap_err();
        assert!(matches!(err, BooleanError::ClassificationInvariant { .. }));
    }
}
