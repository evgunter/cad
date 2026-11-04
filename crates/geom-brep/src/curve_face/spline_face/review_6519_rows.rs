//! Reviewer rows for `Pair::past_an_end`: the decision against the band
//! of the end's point, and which end it reads, are each load-bearing.
//! The arc and patch of `past_end_rows`.

#![allow(clippy::unwrap_used)]

use core::f64::consts::FRAC_PI_2;

use geom::{Curve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use super::{Cell, NurbsBoxes, Pair, Path, SpanCarrier};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const R: f64 = 0.25;

fn flat() -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .map(|(x, y)| Point3::new(x, y, 0.0))
        .to_vec();
    NurbsSurface::new(k(), k(), control, vec![1.0; 4]).unwrap()
}

fn arc() -> Curve3<f64> {
    Curve3::circle(
        Point3::new(1.0, 0.5, 0.0),
        Vec3::unit_x(),
        R,
        -Vec3::unit_z(),
    )
}

/// A box wholly past an end whose carrier comes within the band of the
/// end's point does not settle; one past the band of it does; and each
/// end is read against its own point, not the other's.
#[test]
fn review_6519_a_box_past_an_end_settles_only_past_the_band_of_that_ends_point() {
    let (curve, chart) = (arc(), flat());
    let carrier = SpanCarrier::of(&curve);
    let pair = Pair {
        path: Path::of(&carrier).unwrap(),
        boxes: NurbsBoxes::new(&chart),
        speeds: [R, 1.0, 1.0],
    };
    let span = (FRAC_PI_2 - 0.5, FRAC_PI_2 - 0.25);
    let (a, b) = span;
    // In parameter: a quarter of the band's zero past the end, and a box
    // of the floor's order beyond it.
    let near = 0.25 * band().zero() / R;
    let far = 64.0 * band().escalate() / R;
    let side = 4.0 * band().escalate() / R;
    let uv = [(0.0, 1.0), (0.0, 1.0)];
    let cell = |t: (f64, f64)| Cell([t, uv[0], uv[1]]);
    let cases = [
        (
            "within the band past b",
            cell((b + near, b + near + side)),
            false,
        ),
        (
            "past the band past b",
            cell((b + far, b + far + side)),
            true,
        ),
        (
            "within the band before a",
            cell((a - near - side, a - near)),
            false,
        ),
        (
            "past the band before a",
            cell((a - far - side, a - far)),
            true,
        ),
        ("straddling b", cell((b - side, b + side)), false),
        ("inside the span", cell((a + far, a + far + side)), false),
    ];
    for (what, x, settles) in cases {
        assert_eq!(pair.past_an_end(x, span, band()), settles, "{what}: {x:?}");
    }
}
