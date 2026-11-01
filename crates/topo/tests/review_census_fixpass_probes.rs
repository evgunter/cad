//! Review probes (PR 5605, fix pass): a flush row is written only where
//! two coplanar faces facing one way certifiably do not share interior
//! ([`flush_at_rest`]'s `planar_interiors_meet` behind Door 2's
//! refusal). Each fixture's footprints share interior (so every flush
//! row would be false) or certifiably do not (so the row count is
//! known).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use std::collections::BTreeMap;
use topo::{Body, SolidKey};

fn prism(profile: &[(f64, f64)], z0: f64, z1: f64) -> Body<f64> {
    common::prism_z::<f64>(profile, z0, z1, Tol::witness()).body
}

fn graft(mut a: Body<f64>, b: &Body<f64>) -> Body<f64> {
    topo::graft_disjoint_all_keyed(&mut a, b).unwrap();
    a
}

fn flush(body: &Body<f64>) -> (usize, usize) {
    let groups: BTreeMap<SolidKey, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let f = <f64 as topo::AtRestPolicy>::flush_at_rest(body, &|s| groups.get(&s).copied(), band);
    (f.rows.len(), f.findings.len())
}

/// A ring: [0, 3]² less the hole [1, 2]², z in [0, 1].
fn ring() -> Body<f64> {
    let tol = Tol::witness();
    let plate = common::finished(
        "plate",
        prism(&[(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)], 0.0, 1.0),
        tol,
    );
    let hole = common::finished(
        "hole",
        prism(&[(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)], -1.0, 2.0),
        tol,
    );
    let topo::BooleanResult::Body(out) = topo::subtract(&plate, &hole, tol).unwrap() else {
        panic!("a body");
    };
    out.body.into_body()
}

const SQ2: [(f64, f64); 4] = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
/// A U: notch x in [1, 2], y in [1, 2] of [0, 3] x [0, 2].
const U: [(f64, f64); 8] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 2.0),
    (2.0, 2.0),
    (2.0, 1.0),
    (1.0, 1.0),
    (1.0, 2.0),
    (0.0, 2.0),
];

/// **Footprints sharing interior: no flush row, any face.** Both
/// solids span z in [0, 1], so tops, bottoms and the walls on a shared
/// line all overlap in area. Red if any pair is written flush.
#[test]
fn fixpass_overlapping_footprints_write_no_flush_row() {
    let cases: Vec<(&str, Body<f64>)> = vec![
        (
            "identical squares",
            graft(prism(&SQ2, 0.0, 1.0), &prism(&SQ2, 0.0, 1.0)),
        ),
        (
            "half inside",
            graft(
                prism(&SQ2, 0.0, 1.0),
                &prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)], 0.0, 1.0),
            ),
        ),
        (
            "triangle on the square's corners",
            graft(
                prism(&SQ2, 0.0, 1.0),
                &prism(&[(0.0, 0.0), (2.0, 0.0), (0.0, 2.0)], 0.0, 1.0),
            ),
        ),
        (
            "identical U",
            graft(prism(&U, 0.0, 1.0), &prism(&U, 0.0, 1.0)),
        ),
        (
            "U and the bar under its notch",
            graft(
                prism(&U, 0.0, 1.0),
                &prism(&[(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)], 0.0, 1.0),
            ),
        ),
        (
            "strip across the ring's hole",
            graft(
                ring(),
                &prism(&[(1.0, 0.0), (2.0, 0.0), (2.0, 3.0), (1.0, 3.0)], 0.0, 1.0),
            ),
        ),
        (
            "ring and its own outline",
            graft(
                ring(),
                &prism(&[(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)], 0.0, 1.0),
            ),
        ),
        (
            "the hole's plug overhanging one side",
            graft(
                ring(),
                &prism(&[(1.0, 1.0), (2.0, 1.0), (2.0, 2.5), (1.0, 2.5)], 0.0, 1.0),
            ),
        ),
        (
            "a chevron whose corners all touch the square's boundary",
            graft(
                prism(&SQ2, 0.0, 1.0),
                &prism(
                    &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (1.0, 0.5), (0.0, 2.0)],
                    0.0,
                    1.0,
                ),
            ),
        ),
        (
            "a sliver 1e-3 wide",
            graft(
                prism(&SQ2, 0.0, 1.0),
                &prism(
                    &[(1.999, 0.0), (4.0, 0.0), (4.0, 2.0), (1.999, 2.0)],
                    0.0,
                    1.0,
                ),
            ),
        ),
    ];
    let mut bad = Vec::new();
    for (name, body) in &cases {
        let (rows, _) = flush(body);
        if rows != 0 {
            bad.push((*name, rows));
        }
    }
    assert!(
        bad.is_empty(),
        "flush rows on footprints sharing interior: {bad:?}"
    );
}

/// **Footprints that certifiably do not share interior.** The U's
/// notch filled exactly: tops and bottoms are flush, and so are the
/// y = 2 walls, which meet the U's along the notch's posts (4 rows). The
/// plug that fills the ring's hole exactly: tops and bottoms are flush,
/// its walls face the ring's; each pair is a row or a finding, never
/// silent (today the region walk escalates on the boolean-made ring and
/// both stand as `CensusUndecidable`). Red if a row is lost to a false
/// "meet".
#[test]
fn fixpass_exact_fills_are_flush() {
    let plug = prism(&[(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)], 0.0, 1.0);
    let (rows, findings) = flush(&graft(ring(), &plug));
    assert_eq!(
        rows + findings,
        2,
        "the ring's plug: {rows} rows, {findings} findings"
    );
    let notch = flush(&graft(prism(&U, 0.0, 1.0), &plug));
    assert_eq!(notch, (4, 0), "the U's notch");
}

/// **A plug that reaches over the U's right prong by `w`.** The plug
/// fills the notch with its top-right corner at (2 + w, 2) on the
/// prong's top edge: its top shares a sliver of width up to `w` with
/// the U's, so past the band the tops and bottoms are no flush pair
/// (nor the y = 2 walls on the right, which overlap by `w`); the left
/// y = 2 walls meet along the notch's post, one flush row.
#[test]
fn fixpass_a_plug_reaching_over_a_prong_by_w() {
    let eps = Tol::witness().get().eps;
    for w in [1e-2, 1e-3, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-12, 0.0] {
        let plug = prism(
            &[(1.0, 1.0), (2.0, 1.0), (2.0 + w, 2.0), (1.0, 2.0)],
            0.0,
            1.0,
        );
        let got = flush(&graft(prism(&U, 0.0, 1.0), &plug));
        if w > 1e3 * eps {
            assert_eq!(got, (1, 0), "w {w:e}");
        }
    }
}
