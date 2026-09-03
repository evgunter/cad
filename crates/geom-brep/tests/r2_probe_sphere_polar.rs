//! R2 adversarial probes for CERT-1 (issue 723 / issue 893).
//!
//! Not a proposed suite — a reviewer's instrument. Every row prints
//! what it saw so the report can quote execution rather than reading.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf;
use crate::shared::topo;
use geom::Surface;
use geom_brep::props::{LoopEdge, curved_face};
use geom_core::Band;
use geom_core::Tol;

const RS: f64 = 0.010;
const PI: f64 = core::f64::consts::PI;

fn sphere() -> Surface<f64> {
    surf::sphere(RS)
}
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::sphere_rim(RS, v, u0, u1, a, b)
}
fn great(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::sphere_great(RS, u, t0, t1, a, b)
}

/// Report, never assert: print ACCEPT/REFUSE and the relative error
/// against a closed form so the reviewer reads execution.
fn report(kind: &str, edges: &[LoopEdge<f64>], exact: f64) -> Option<f64> {
    let band = Band::linear(Tol::witness()).unwrap();
    match curved_face(&sphere(), edges, 1.0, band) {
        Ok(fc) => {
            let rel = (fc.area - exact) / exact;
            println!(
                "  {kind:<52} ACCEPT area={:.15e} exact={exact:.15e} rel={rel:+.4e}",
                fc.area
            );
            Some(rel)
        }
        Err(e) => {
            println!("  {kind:<52} REFUSE {e:?}");
            None
        }
    }
}

// --- Claim 1: attack the span parse -----------------------------------

#[test]
fn probe_parse_attacks() {
    println!("\n== claim 1: span parse attacks ==");
    let b: f64 = 0.5;
    let cap = RS * RS * PI * (1.0 - b.sin());

    // (a) split vertex sitting EXACTLY on the north pole: both arcs
    //     are anchored at a pole, so `f` is exactly 0 -> Zero -> push.
    report(
        "half-cap, split AT the pole (t=pi/2)",
        &[
            rim(b, 0.0, PI, 0, 1),
            great(0.0, PI - b, PI / 2.0, 1, 2),
            great(0.0, PI / 2.0, b, 2, 0),
        ],
        cap,
    );

    // (b) split a hair OFF the pole, both sides.
    for d in [1e-3, 1e-7, 1e-12, 1e-15] {
        report(
            &format!("half-cap, split at pi/2 - {d:e}"),
            &[
                rim(b, 0.0, PI, 0, 1),
                great(0.0, PI - b, PI / 2.0 - d, 1, 2),
                great(0.0, PI / 2.0 - d, b, 2, 0),
            ],
            cap,
        );
    }

    // (c) the no-split twin: ONE arc from b to pi-b (span pi-2b,
    //     pole strictly interior). Pre-fix this was lo == hi.
    report(
        "half-cap, NO split (one arc over the pole)",
        &[rim(b, 0.0, PI, 0, 1), great(0.0, b, PI - b, 1, 0)],
        cap,
    );

    // (d) rimless hemisphere, one arc spanning BOTH poles (3pi/2) and
    //     one spanning neither (pi/2).
    let q = PI / 4.0;
    let hemi = 2.0 * PI * RS * RS;
    report(
        "rimless hemisphere, one arc spans BOTH poles",
        &[
            great(0.0, q, q + 3.0 * PI / 2.0, 0, 1),
            great(0.0, q + 3.0 * PI / 2.0, q + 2.0 * PI, 1, 0),
        ],
        hemi,
    );

    // (e) the same hemisphere with every parameter pushed past the
    //     chart seam by +2pi and by -2pi: no mod-2pi reduction may
    //     move the answer.
    for shift in [2.0 * PI, -2.0 * PI, 6.0 * PI] {
        report(
            &format!("rimless hemisphere at pi/4, params shifted {shift:+.3}"),
            &[
                great(0.0, shift + q, shift + 5.0 * q, 0, 1),
                great(0.0, shift + 5.0 * q, shift + 9.0 * q, 1, 0),
            ],
            hemi,
        );
    }

    // (f) rimless FULL-circle meridian as a single closed edge.
    report(
        "rimless: one full-circle meridian (dt = 2pi)",
        &[great(0.0, 0.0, 2.0 * PI, 0, 0)],
        hemi,
    );

    // (g) span of exactly pi anchored at a pole (claim 4's shape).
    report(
        "rimless hemisphere, arcs of exactly pi anchored at poles",
        &[
            great(0.0, -PI / 2.0, PI / 2.0, 0, 1),
            great(0.0, PI / 2.0, 3.0 * PI / 2.0, 1, 0),
        ],
        hemi,
    );

    // (h) a MULTI-WRAP span: dt = 3pi. Covers the whole circle, so
    //     both poles are attained; does the dot test still say so?
    report(
        "rimless: arcs of 3pi + pi (dt > 2pi on one arc)",
        &[
            great(0.0, 0.0, 3.0 * PI, 0, 1),
            great(0.0, 3.0 * PI, 4.0 * PI, 1, 0),
        ],
        hemi,
    );
}

// --- Claim 3 / structural: the lift's near-polar residual --------------

/// An ordinary spherical ZONE (rectangle) whose upper rim marches
/// toward the pole. Both rims sit at genuine extremes, so the margin
/// SHOULD be ~0 — but the lift recomputes the extreme's cosine from
/// its sine, and d(cos)/d(sin) = -tan v blows up at the pole.
#[test]
fn probe_near_polar_zone_at_its_own_extreme() {
    println!("\n== claim 3: zone whose rim marches to the pole ==");
    let v0: f64 = 0.2;
    for d in [1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10] {
        let v1 = PI / 2.0 - d;
        let exact = 2.0 * PI * RS * RS * (v1.sin() - v0.sin());
        report(
            &format!("zone [0,2pi]x[0.2, pi/2-{d:e}]"),
            &[rim(v0, 0.0, 2.0 * PI, 0, 0), rim(v1, 2.0 * PI, 0.0, 1, 1)],
            exact,
        );
    }
}

/// The same march, but on a face that also has meridians (a lune-cap):
/// rim at v0, rim at v1, two meridian arcs of span (v1 - v0).
#[test]
fn probe_near_polar_lune_at_its_own_extreme() {
    println!("\n== claim 3: half-zone with meridians, rim -> pole ==");
    let v0: f64 = 0.2;
    for d in [1e-2, 1e-4, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-12] {
        let v1 = PI / 2.0 - d;
        let exact = PI * RS * RS * (v1.sin() - v0.sin());
        report(
            &format!("half-zone [0,pi]x[0.2, pi/2-{d:e}]"),
            &[
                rim(v0, 0.0, PI, 0, 1),
                great(PI, v0, v1, 1, 2),
                rim(v1, PI, 0.0, 2, 3),
                great(0.0, v1, v0, 3, 0),
            ],
            exact,
        );
    }
}

/// dt -> 0: a very thin zone far from the pole. The pole must stay
/// decisively OUTSIDE and the face must certify.
#[test]
fn probe_thin_meridian_spans() {
    println!("\n== claim 4: dt near 0 ==");
    let v0: f64 = 0.2;
    for dv in [1e-2, 1e-5, 1e-8, 1e-11, 1e-14] {
        let v1: f64 = v0 + dv;
        let exact = PI * RS * RS * (v1.sin() - v0.sin());
        report(
            &format!("half-zone of height dv={dv:e}"),
            &[
                rim(v0, 0.0, PI, 0, 1),
                great(PI, v0, v1, 1, 2),
                rim(v1, PI, 0.0, 2, 3),
                great(0.0, v1, v0, 3, 0),
            ],
            exact,
        );
    }
}

/// A polar CAP: one rim, no meridian. Nothing pushes the pole level,
/// because the fold rides on the meridian arm only.
#[test]
fn probe_polar_cap_no_meridian() {
    println!("\n== structural: a cap with a rim and NO meridian ==");
    let v0: f64 = 0.5;
    let exact = 2.0 * PI * RS * RS * (1.0 - v0.sin());
    report(
        "full cap: one rim circle, no meridian",
        &[rim(v0, 0.0, 2.0 * PI, 0, 0)],
        exact,
    );
    // The same cap cut in half by a pole-crossing meridian arc — the
    // shape the unit DOES serve — for contrast.
    report(
        "half cap: rim + pole-crossing arc",
        &[rim(v0, 0.0, PI, 0, 1), great(0.0, v0, PI - v0, 1, 0)],
        exact / 2.0,
    );
}

/// The near-polar rim-separation band, swept: does the new lever
/// refuse everything near the pole, or only genuinely distinct pairs?
#[test]
fn probe_near_polar_separation_sweep() {
    println!("\n== claim 3: staircase separation sweep ==");
    let band = Band::linear(Tol::witness()).unwrap();
    println!(
        "  band.zero()={:e} band.escalate()={:e}",
        band.zero(),
        band.escalate()
    );
    let d0 = 0.002;
    let v2 = PI / 2.0 - d0;
    let v0: f64 = 0.2;
    for mult in [0.1, 0.5, 0.9, 1.0, 1.5, 3.0, 10.0, 100.0] {
        // separation as a multiple of `zero`, in POINT units at R
        let dv = mult * band.zero() / RS;
        let edges = vec![
            rim(v0, -1.0, 1.0, 0, 1),
            great(1.0, v0, v2, 1, 2),
            rim(v2, 1.0, 0.0, 2, 3),
            great(0.0, v2, v2 - dv, 3, 4),
            rim(v2 - dv, 0.0, -1.0, 4, 5),
            great(-1.0, v2 - dv, v0, 5, 0),
        ];
        match curved_face(&sphere(), &edges, 1.0, band) {
            Ok(fc) => println!("  sep = {mult:>6}*zero  ACCEPT area={:.9e}", fc.area),
            Err(e) => println!("  sep = {mult:>6}*zero  REFUSE {e:?}"),
        }
    }
    // The same sweep at mid-latitude, where the OLD axial lever was
    // already honest: the new lever must not have moved these.
    println!("  -- same sweep at v = 0.6 (mid-latitude control) --");
    let vm = 0.6;
    for mult in [0.1, 0.5, 0.9, 1.0, 1.5, 3.0, 10.0] {
        let dv = mult * band.zero() / RS;
        let edges = vec![
            rim(v0, -1.0, 1.0, 0, 1),
            great(1.0, v0, vm, 1, 2),
            rim(vm, 1.0, 0.0, 2, 3),
            great(0.0, vm, vm - dv, 3, 4),
            rim(vm - dv, 0.0, -1.0, 4, 5),
            great(-1.0, vm - dv, v0, 5, 0),
        ];
        match curved_face(&sphere(), &edges, 1.0, band) {
            Ok(fc) => println!("  sep = {mult:>6}*zero  ACCEPT area={:.9e}", fc.area),
            Err(e) => println!("  sep = {mult:>6}*zero  REFUSE {e:?}"),
        }
    }
}

// --- Claim 4: the shipped form AT INTERVAL ----------------------------
//
// The new suite `cert1_sphere_polar.rs` is f64-only: it has no
// Interval row at all, so `--features interval` re-runs it as f64.
// These rows drive the SAME geometry through the certified Interval
// decision scalar and compare outcomes.
#[cfg(feature = "interval")]
mod interval_lane {
    use crate::shared::surf;
    use crate::shared::topo;
    use geom::Surface;
    use geom_brep::props::{LoopEdge, curved_face};
    #[allow(unused_imports)]
    use geom_core::Decide as _;
    use geom_core::{Band, Interval, Real, Tol};

    const RSF: f64 = 0.010;

    fn sphere<T: Real>() -> Surface<T> {
        surf::sphere(RSF)
    }
    fn rim<T: Real>(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<T> {
        topo::sphere_rim(RSF, v, u0, u1, a, b)
    }
    fn great<T: Real>(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
        topo::sphere_great(RSF, u, t0, t1, a, b)
    }

    const PI: f64 = core::f64::consts::PI;

    fn cases<T: geom_core::Decide>() -> Vec<(&'static str, Vec<LoopEdge<T>>)> {
        let b = 0.5;
        let q = PI / 4.0;
        vec![
            (
                "half-cap split at t=1.0 (the unit's row 1)",
                vec![
                    rim(b, 0.0, PI, 0, 1),
                    great(0.0, PI - b, 1.0, 1, 2),
                    great(0.0, 1.0, b, 2, 0),
                ],
            ),
            (
                "half-cap split EXACTLY at the pole",
                vec![
                    rim(b, 0.0, PI, 0, 1),
                    great(0.0, PI - b, PI / 2.0, 1, 2),
                    great(0.0, PI / 2.0, b, 2, 0),
                ],
            ),
            (
                "half-cap NO split",
                vec![rim(b, 0.0, PI, 0, 1), great(0.0, b, PI - b, 1, 0)],
            ),
            (
                "rimless hemisphere at +-pi/4 (the unit's row 2)",
                vec![
                    great(0.0, q, 5.0 * q, 0, 1),
                    great(0.0, 5.0 * q, 9.0 * q, 1, 0),
                ],
            ),
            (
                "rimless hemisphere, arcs of EXACTLY pi at the poles",
                vec![
                    great(0.0, -PI / 2.0, PI / 2.0, 0, 1),
                    great(0.0, PI / 2.0, 3.0 * PI / 2.0, 1, 0),
                ],
            ),
            (
                "full-circle meridian, dt = 2pi",
                vec![great(0.0, 0.0, 2.0 * PI, 0, 0)],
            ),
            (
                "thin zone, dt = 1e-5",
                vec![
                    rim(0.2, 0.0, PI, 0, 1),
                    great(PI, 0.2, 0.2 + 1e-5, 1, 2),
                    rim(0.2 + 1e-5, PI, 0.0, 2, 3),
                    great(0.0, 0.2 + 1e-5, 0.2, 3, 0),
                ],
            ),
        ]
    }

    #[test]
    fn probe_interval_outcomes_match_f64() {
        let band = Band::linear(Tol::witness()).unwrap();
        let mut mismatches = 0;
        for ((name, ef), (_, ei)) in cases::<f64>().into_iter().zip(cases::<Interval>()) {
            let a = curved_face(&sphere::<f64>(), &ef, 1.0, band);
            let bb = curved_face(&sphere::<Interval>(), &ei, Interval::from_f64(1.0), band);
            let tag = match (&a, &bb) {
                (Ok(_), Ok(_)) => "both ACCEPT",
                (Err(_), Err(_)) => "both REFUSE",
                _ => {
                    mismatches += 1;
                    "***OUTCOME MISMATCH***"
                }
            };
            println!("  {name:<48} {tag}");
            assert!(
                !tag.contains("MISMATCH"),
                "{name}: the certifying scalars disagreed — f64 {a:?} vs interval"
            );
            match (&a, &bb) {
                (Ok(x), Ok(_)) => println!("      f64 area={:.12e}   interval area encl", x.area),
                (Err(x), Err(y)) => {
                    println!("      f64: {x:?}");
                    println!("      itv: {y:?}");
                }
                (Ok(x), Err(y)) => println!("      f64 OK area={:.12e} / itv {y:?}", x.area),
                (Err(x), Ok(_)) => println!("      f64 {x:?} / itv OK"),
            }
        }
        assert_eq!(mismatches, 0, "outcome mismatches: {mismatches}");
    }
}
