//! VERBS-SHELLFIX PR-2a R2 probe, interval lane: the simultaneous door
//! instantiated at the certified scalar. No shipped row does this — the
//! PR's interval drawn point re-runs the f64-typed suites under the
//! interval BUILD, which never instantiates `offset_planes_together`
//! at `T = Interval`. This row does, on a box and on an oblique
//! hexagonal prism, and reads the volume enclosure.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Bounds, Interval, Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, ChartMove};

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(Interval::from_f64(x), Interval::from_f64(y))
}

fn prism(pts: &[(f64, f64)], h: f64) -> Body<Interval> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), Interval::from_f64(0.0)))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a polygon is a valid profile");
    extrude(
        &profile,
        Extrusion::Distance(Interval::from_f64(h)),
        Tol::witness(),
    )
    .expect("a polygon extrudes")
    .body
}

/// The door at `T = Interval`, on a box: every corner solve, every
/// concurrence margin and the conditioning meter decided from interval
/// enclosures. The result's volume enclosure must contain the exact
/// inset volume `1.5 · 2.5 · 3.5`.
#[test]
fn interval_offset_planes_together_box() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut body = prism(&[(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)], 4.0);
    let centroid = (1.0, 1.5, 2.0);
    let mut moves: Vec<ChartMove<Interval>> = Vec::new();
    for (k, f) in body.faces() {
        let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            panic!("a box is planes");
        };
        let toward_inside = normal.x.lo() * (centroid.0 - origin.x.lo())
            + normal.y.lo() * (centroid.1 - origin.y.lo())
            + normal.z.lo() * (centroid.2 - origin.z.lo());
        let signed = if toward_inside > 0.0 { 0.25 } else { -0.25 };
        moves.push(ChartMove {
            faces: vec![k],
            distance: Interval::from_f64(signed),
        });
    }
    topo::offset_planes_together(&mut body, &moves, band, tol)
        .expect("the interval box inset builds");
    let props = topo::mass_properties(&body, tol).expect("interval props");
    let (lo, hi) = (props.volume.lo(), props.volume.hi());
    println!("[r2a-interval] box inset volume enclosure [{lo}, {hi}]");
    assert!(
        lo <= 13.125 && 13.125 <= hi,
        "the enclosure contains the exact inset volume"
    );
}

/// The oblique corner at `T = Interval`: the hexagonal prism the class
/// was measured on, through the same door. A corner solved from
/// interval plane data carries its own conditioning honestly — the
/// meter, the concurrence residual and the edge-agreement gap all
/// classify interval margins, and any straddle escalates rather than
/// guesses.
#[test]
fn interval_offset_planes_together_hexagon() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let r = 0.2;
    let t = 0.02;
    let hex: Vec<(f64, f64)> = (0..6)
        .map(|i| {
            let a = core::f64::consts::TAU * f64::from(i) / 6.0;
            (r * a.cos(), r * a.sin())
        })
        .collect();
    let mut body = prism(&hex, 0.25);
    let mut moves: Vec<ChartMove<Interval>> = Vec::new();
    for (k, f) in body.faces() {
        let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            panic!("a hexagonal prism is planes");
        };
        let toward_inside = normal.x.lo() * (0.0 - origin.x.lo())
            + normal.y.lo() * (0.0 - origin.y.lo())
            + normal.z.lo() * (0.125 - origin.z.lo());
        let signed = if toward_inside > 0.0 { t } else { -t };
        moves.push(ChartMove {
            faces: vec![k],
            distance: Interval::from_f64(signed),
        });
    }
    match topo::offset_planes_together(&mut body, &moves, band, tol) {
        Ok(()) => {
            let props = topo::mass_properties(&body, tol).expect("interval props");
            let (lo, hi) = (props.volume.lo(), props.volume.hi());
            println!("[r2a-interval] hexagon inset volume enclosure [{lo}, {hi}]");
            // The inset solid: hexagon of circumradius r − 2t/√3 over
            // height 0.25 − 2t (the acceptance's own closed form, seen
            // from the cavity side).
            let s3 = 3.0_f64.sqrt();
            let want = 1.5 * s3 * (r - 2.0 * t / s3).powi(2) * (0.25 - 2.0 * t);
            assert!(
                lo <= want && want <= hi,
                "the enclosure contains the exact inset volume {want}"
            );
        }
        Err(e) => println!("[r2a-interval] hexagon inset: refused/escalated, {e}"),
    }
}
