//! Reviewer probes for PR #5745 (ATREST, two copies filling one volume):
//! where the same-side face-pair reading must and must not fire, across
//! the band, at each ε row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use std::collections::BTreeMap;
use topo::{Body, ContactRecords};

fn shifted(p: &[(f64, f64)], dx: f64, dy: f64) -> Vec<(f64, f64)> {
    p.iter().map(|&(x, y)| (x + dx, y + dy)).collect()
}

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

/// The at-rest findings and flush reading, rendered: names of findings,
/// whether any is a same-side interference, and how many interference
/// pairs the gather localizes.
fn read(body: &Body<f64>) -> (Vec<String>, bool, usize, usize, usize) {
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
    let names: Vec<String> = recorded
        .findings
        .iter()
        .chain(flush.findings.iter())
        .map(|f| {
            let d = format!("{f:?}");
            d.split([' ', '{', '(']).next().unwrap().to_string()
        })
        .collect();
    let ssi = names.iter().any(|n| n == "SameSideInterference");
    let interf = recorded
        .findings
        .iter()
        .filter(|f| topo::census_interference_solids(body, f).is_some())
        .count();
    (names, ssi, interf, recorded.rows.len(), flush.rows.len())
}

const SQ: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

/// The offsets across the band: exact, inside ε, mid-band, past the band.
fn offsets() -> [(&'static str, f64, Option<bool>); 5] {
    let eps = Tol::witness().eps();
    let k = Tol::witness().k();
    [
        ("exact", 0.0, Some(false)),
        ("half eps", 0.5 * eps, Some(false)),
        ("gap half eps", -0.5 * eps, Some(false)),
        ("mid band", 0.5 * (1.0 + k) * eps, Some(false)),
        ("past band", 4.0 * k * eps, Some(true)),
    ]
}

fn check(label: &str, body: &Body<f64>, want: Option<bool>) {
    let (names, ssi, interf, rows, flush) = read(body);
    eprintln!(
        "PROBE eps={:e} {label}: ssi={ssi} interference={interf} rows={rows} flush={flush} findings={names:?}",
        Tol::witness().eps()
    );
    if std::env::var("PROBE_DUMP").is_ok() {
        dump(body);
    }
    if let Some(w) = want {
        assert_eq!(ssi, w, "{label}: same-side interference {names:?}");
    }
}

/// A cube stacked on the base, sunk by the offset: the walls are exactly
/// coplanar and overlap by a strip of height `d`. Only a strip past the
/// band is an overlap the census may decide.
#[test]
fn review_stacked_sunk_across_the_band() {
    for (label, d, want) in offsets() {
        let body = two(&SQ, (0.0, 1.0), &SQ, (1.0 - d, 2.0 - d));
        check(&format!("stacked {label}"), &body, want);
    }
}

/// A cube beside the base, pushed in by the offset: the caps and two
/// walls are exactly coplanar and overlap by a strip of width `d`.
#[test]
fn review_side_by_side_across_the_band() {
    for (label, d, want) in offsets() {
        let body = two(&SQ, (0.0, 1.0), &shifted(&SQ, 1.0 - d, 0.0), (0.0, 1.0));
        check(&format!("beside {label}"), &body, want);
    }
}

/// Diagonal neighbours: the caps are coplanar and facing the same way,
/// touching at one point. No overlap.
#[test]
fn review_corner_touch_is_no_interference() {
    let body = two(&SQ, (0.0, 1.0), &shifted(&SQ, 1.0, 1.0), (0.0, 1.0));
    check("corner touch", &body, Some(false));
}

/// A square tucked in an L's notch: the caps' boxes overlap over an area,
/// their interiors share only two edges. No overlap.
#[test]
fn review_l_notch_is_no_interference() {
    let l = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let body = two(&l, (0.0, 1.0), &shifted(&SQ, 1.0, 1.0), (0.0, 1.0));
    check("l notch", &body, Some(false));
}

/// A narrower copy inside the base, one wall in band of the base's: an
/// overlap (the caps share interior) beside a genuinely in-band touch.
/// The overlap is decided; what else is reported is printed.
#[test]
fn review_overlap_beside_an_in_band_wall() {
    let eps = Tol::witness().eps();
    let k = Tol::witness().k();
    let w = 1.0 - 0.5 * (1.0 + k) * eps;
    let narrow = [(0.0, 0.0), (w, 0.0), (w, 1.0), (0.0, 1.0)];
    let body = two(&SQ, (0.0, 1.0), &narrow, (0.0, 1.0));
    check("in-band wall", &body, Some(true));
}

/// Each pair-naming or touch finding at rest, printed (`PROBE_DUMP=1`).
fn dump(body: &Body<f64>) {
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
    for f in &recorded.findings {
        if matches!(
            f,
            topo::ValidationError::CensusUndecidable { .. }
                | topo::ValidationError::UnrecordedContact { .. }
        ) {
            eprintln!("  DUMP {f:?}");
        }
    }
}

/// A hollow cube (void `[1, 2]³` in `[0, 3]³`) beside a core: the core
/// filling the void exactly, and a half core flush against five of the
/// void's walls, meet the void only through opposed pairs (no overlap);
/// a slab in the wall behind the void's `x = 1` face shares that face's
/// plane and sense, and overlaps.
#[test]
fn review_cores_in_a_void() {
    use topo::{SolidContainment, VoidContainment, VoidEvidence, insert_void};
    let hollow = || {
        let mut outer = common::brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), Tol::witness());
        let hole = common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), Tol::witness());
        let (solid, _) = outer.solids().next().unwrap();
        let evidence = VoidEvidence {
            shells: hole
                .shells()
                .map(|(s, _)| (s, VoidContainment::Probed(SolidContainment::In)))
                .collect(),
        };
        insert_void(&mut outer, solid, hole, &evidence).unwrap();
        outer
    };
    for (label, x, want) in [
        ("core fills the void", (1.0, 2.0), Some(false)),
        ("half core on the void's walls", (1.0, 1.5), Some(false)),
        ("slab behind the void wall", (0.5, 1.0), None),
    ] {
        let mut body = hollow();
        let core = common::brick::<f64>(x, (1.0, 2.0), (1.0, 2.0), Tol::witness());
        topo::graft_disjoint_all_keyed(&mut body, &core).unwrap();
        check(label, &body, want);
    }
}
