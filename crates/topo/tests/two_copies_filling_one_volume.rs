//! **Two solids whose material coincides over an area interfere**
//! (`census::planar_face_pairs`, arm 2 of the cross-solid backstop): a
//! face of each on one plane, facing the same way, sharing interior, is
//! [`ValidationError::SameSideInterference`] however every other cell of
//! the two boundaries coincides, and a pair that only abuts keeps its
//! rest and flush rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use std::collections::BTreeMap;
use topo::{Body, ContactRecords, ValidationError};

/// A prism over `base` from `z0` to `z1`, and one over `top`, grafted as
/// two solids of one body.
fn two(
    base: &[(f64, f64)],
    (z0, z1): (f64, f64),
    top: &[(f64, f64)],
    (w0, w1): (f64, f64),
) -> Body<f64> {
    let mut body = common::prism_z::<f64>(base, z0, z1, Tol::witness()).body;
    let t = common::prism_z::<f64>(top, w0, w1, Tol::witness()).body;
    topo::graft_disjoint_all_keyed(&mut body, &t).unwrap();
    body
}

/// What the census records over `body` at rest, each solid its own
/// group, and the flush pairs between them.
fn at_rest(body: &Body<f64>) -> (topo::RecordedAtRest, topo::FlushAtRest) {
    let groups: BTreeMap<_, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
    let recorded = topo::record_at_rest(
        body,
        &ContactRecords::default(),
        |records| {
            topo::validate_pseudomanifold(body, records, Tol::witness())
                .err()
                .unwrap_or_default()
        },
        &|s| groups.get(&s).copied(),
        0,
    );
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let flush =
        <f64 as topo::AtRestPolicy>::flush_at_rest(body, &|s| groups.get(&s).copied(), band);
    (recorded, flush)
}

const SQ: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

/// Coincident cubes, a half-height cube flush in the base, and a
/// quarter block in the base's corner (three faces coincident): no
/// vertex of either lies strictly inside the other and nothing pierces,
/// so each overlap shows only on its coincident faces. Each pair is one
/// `SameSideInterference`, decided and never undecidable, and the
/// at-rest door partitions it as interference. No row is recorded or
/// flush. Red if the face-pair arm stops reading aligned pairs, or arm 2
/// reports the pair undecidable beside its decided overlap.
#[test]
fn copies_whose_material_coincides_are_one_interference() {
    let half = [(0.0, 0.0), (0.5, 0.0), (0.5, 1.0), (0.0, 1.0)];
    for (label, body) in [
        ("coincident", two(&SQ, (0.0, 1.0), &SQ, (0.0, 1.0))),
        ("flush half", two(&SQ, (0.0, 1.0), &SQ, (0.0, 0.5))),
        ("corner quarter", two(&SQ, (0.0, 1.0), &half, (0.0, 0.5))),
    ] {
        let solids: Vec<_> = body.solids().map(|(s, _)| s).collect();
        let (recorded, flush) = at_rest(&body);
        let interference: Vec<_> = recorded
            .findings
            .iter()
            .filter_map(|f| topo::census_interference_solids(&body, f).map(|p| (f, p)))
            .collect();
        assert!(
            matches!(
                interference[..],
                [(ValidationError::SameSideInterference { .. }, (x, y))]
                    if [x, y] == solids[..] || [y, x] == solids[..]
            ),
            "{label}: one same-side interference between the two solids: {:?}",
            recorded.findings
        );
        assert!(
            !recorded
                .findings
                .iter()
                .any(|f| matches!(f, ValidationError::CensusUndecidable { .. })),
            "{label}: a decided overlap is not undecidable: {:?}",
            recorded.findings
        );
        assert!(
            recorded.rows.is_empty() && flush.rows.is_empty() && flush.findings.is_empty(),
            "{label}: an overlapping pair records nothing: {:?} {flush:?}",
            recorded.rows
        );
    }
}

/// A cube stacked on the base: the caps rest, opposed, and the four walls
/// lie flush, facing the same way with only their edges meeting. Nothing
/// interferes. Red if the same-side reading takes a flush pair's shared
/// boundary for shared interior.
#[test]
fn a_stacked_copy_rests_and_lies_flush() {
    let body = two(&SQ, (0.0, 1.0), &SQ, (1.0, 2.0));
    let (recorded, flush) = at_rest(&body);
    assert!(
        recorded.findings.is_empty() && flush.findings.is_empty(),
        "nothing refuses: {:?} {:?}",
        recorded.findings,
        flush.findings
    );
    let relations: Vec<_> = recorded.rows.iter().map(|r| r.relation).collect();
    assert_eq!(relations, [topo::Relation::SameOpposite], "the caps rest");
    assert_eq!(flush.rows.len(), 4, "the walls lie flush: {:?}", flush.rows);
}

/// **An overlap within the band is no interference.** A cube stacked on
/// the base and sunk into it by `d`, and one beside it pushed in by
/// `d`: the coplanar walls (and caps) share a strip of width `d`. Within
/// the band the census cannot say the strip exists, so no
/// `SameSideInterference` is decided; past it, the strip is one. Red if
/// the face-pair arm decides an overlap off a reading in the band.
#[test]
fn an_overlap_within_the_band_is_no_interference() {
    let (eps, k) = (Tol::witness().eps(), Tol::witness().k());
    let beside: Vec<(f64, f64)> = SQ.iter().map(|&(x, y)| (x + 1.0, y)).collect();
    for (label, d, decided) in [
        ("exact", 0.0, false),
        ("half eps", 0.5 * eps, false),
        ("gap half eps", -0.5 * eps, false),
        ("mid band", 0.5 * (1.0 + k) * eps, false),
        ("past band", 4.0 * k * eps, true),
    ] {
        let pushed: Vec<(f64, f64)> = beside.iter().map(|&(x, y)| (x - d, y)).collect();
        for (shape, body) in [
            ("stacked", two(&SQ, (0.0, 1.0), &SQ, (1.0 - d, 2.0 - d))),
            ("beside", two(&SQ, (0.0, 1.0), &pushed, (0.0, 1.0))),
        ] {
            let (recorded, _) = at_rest(&body);
            assert_eq!(
                recorded
                    .findings
                    .iter()
                    .any(|f| matches!(f, ValidationError::SameSideInterference { .. })),
                decided,
                "{shape}, {label}: {:?}",
                recorded.findings
            );
        }
    }
}
