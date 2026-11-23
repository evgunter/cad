//! **A piece's crossing of a spline arc is levered at the piece's reach
//! from the meeting** (`spline_crossings`, `split_ring_path_spline_across`),
//! whatever the piece's kind: the sine is read where the two curves meet
//! and claims a definite crossing, so its arm is how far the piece runs
//! from there, from below. A piece met away from its far end reaches less
//! than its chord.
//!
//! Each row crosses a straight piece at the origin by a cubic spline
//! whose offset from the piece is `s·v + K·v³`: it meets the piece once,
//! at the sine `s`, and leaves the band within a few hundredths of the
//! meeting, so the table's reader finds one crossing and the lever alone
//! decides it.
//!
//! **No row holds the arc arm.** A half turn met at its middle by the
//! same cubic, bent to the circle (`(1 − v²/2 + s·v + K·v³, v, 0)`), at
//! `K` from 50 to 500 and sines `5·10⁻³` to `10⁻²` under this band, never
//! reaches the crossing reading: `meet` escalates on
//! `curve_pair_walk_pair_gap`, `curve_pair_meet_window` or
//! `curve_pair_meet_stretch`, or says nothing (`work/oneround/diameter-levers-of-a-datum-read-at-a-point.md`
//! holds that as open).

use geom::Curve3;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3};

use super::spline_wall::IsoPiece;
use super::{Piece, SplineArc, spline_crossings};

const ACROSS: &str = "split_ring_path_spline_across";

/// The steepness of the spline's cubic term: past the band within a few
/// hundredths of the meeting.
const K: f64 = 100.0;

/// The spline's half-span in `v`.
const HALF: f64 = 0.1;

/// The band every row reads at.
fn band() -> Band {
    Band::new(1e-3, 1e-2).unwrap()
}

/// The cubic Bézier spline over `u ∈ [0, 1]` whose point at
/// `v = HALF·(2u − 1)` is `at(v)`, each coordinate a cubic in `v` given
/// by its coefficients `[c0, c1, c2, c3]`.
fn cubic(coeffs: [[f64; 4]; 3]) -> SplineArc<f64> {
    // `v` in `u`: `v = −HALF + 2·HALF·u`; each coordinate's power basis
    // in `u`, then its Bézier ordinates.
    let (a, b) = (-HALF, 2.0 * HALF);
    let ordinates = |[c0, c1, c2, c3]: [f64; 4]| {
        // f(v) = c0 + c1 v + c2 v² + c3 v³ at v = a + b u.
        let p0 = c0 + c1 * a + c2 * a * a + c3 * a * a * a;
        let p1 = c1 * b + 2.0 * c2 * a * b + 3.0 * c3 * a * a * b;
        let p2 = c2 * b * b + 3.0 * c3 * a * b * b;
        let p3 = c3 * b * b * b;
        [
            p0,
            p0 + p1 / 3.0,
            p0 + 2.0 * p1 / 3.0 + p2 / 3.0,
            p0 + p1 + p2 + p3,
        ]
    };
    let [x, y, z] = coeffs.map(ordinates);
    let points = (0..4).map(|i| Point3::new(x[i], y[i], z[i])).collect();
    let net = geom::NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
        points,
        vec![1.0; 4],
    )
    .unwrap();
    SplineArc::on_by_construction(&Curve3::Nurbs(std::sync::Arc::new(net)), (0.0, 1.0)).unwrap()
}

/// The spline crossing the `x̂` axis at the origin at the sine `s`, in
/// the `xy` plane: `(v, s·v + K·v³, 0)`.
fn across_the_axis(s: f64) -> SplineArc<f64> {
    cubic([[0.0, 1.0, 0.0, 0.0], [0.0, s, 0.0, K], [0.0; 4]])
}

/// What `spline_crossings` reads of `piece` against `arc`.
fn crossings(piece: &Piece<f64>, arc: &SplineArc<f64>) -> String {
    match spline_crossings(piece, arc, false, 0.0, band()) {
        Ok(n) => format!("Ok({n:?})"),
        Err(d) => format!("Err({:?})", d.predicate),
    }
}

/// The straight line along `x̂` from `x = −1` to `x = 3`: met at the
/// origin it reaches 3, its chord 4.
fn ruling() -> Piece<f64> {
    Piece::Ruling {
        from: Point3::new(-1.0, 0.0, 0.0),
        to: Point3::new(3.0, 0.0, 0.0),
    }
}

/// The same line as an iso piece of a spline wall: a degree-one spline.
fn iso_line() -> Piece<f64> {
    let net = geom::NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![Point3::new(-1.0, 0.0, 0.0), Point3::new(3.0, 0.0, 0.0)],
        vec![1.0, 1.0],
    )
    .unwrap();
    Piece::Iso(IsoPiece {
        curve: std::sync::Arc::new(net),
        span: (0.0, 1.0),
        forward: true,
    })
}

/// `piece`, met at the origin by the spline at three sines, under the
/// band `(10⁻³, 10⁻²)`: the piece reaches 3 from the meeting, its chord
/// is 4.
///
/// - At `s = 3·10⁻³` the sine over the reach is `9·10⁻³`, in band, and
///   the crossing escalates. Over the chord it would read `1.2·10⁻²`,
///   definite, and count a crossing the reach does not prove.
/// - At `s = 5·10⁻⁴` it is `1.5·10⁻³`, in band again. Over half the reach
///   it would read Zero, and the path would say nothing.
/// - At `s = 6·10⁻³` it is `1.8·10⁻²`, definite: one crossing.
fn reads_its_reach_from_the_meeting(what: &str, piece: &Piece<f64>) {
    let escalated = format!("Err({:?})", Some(ACROSS));
    for (s, want) in [
        (3e-3, escalated.clone()),
        (5e-4, escalated),
        (6e-3, "Ok(Some(1))".to_owned()),
    ] {
        let got = crossings(piece, &across_the_axis(s));
        assert_eq!(got, want, "{what} at the sine {s}");
    }
}

/// **A ruling or segment piece is levered at its reach from the meeting,
/// not its chord.** Red where the arm is the chord (`ExtentBall` between
/// its ends, as it was) or half the reach.
#[test]
fn a_ruling_crossed_away_from_its_ends_is_levered_at_its_reach() {
    reads_its_reach_from_the_meeting("the ruling", &ruling());
}

/// **An iso piece of a spline wall is levered at its reach from the
/// meeting, not its chord** (`IsoPiece::travel`, as it was). Red where
/// the arm is the chord or half the reach.
#[test]
fn an_iso_piece_crossed_away_from_its_ends_is_levered_at_its_reach() {
    reads_its_reach_from_the_meeting("the iso piece", &iso_line());
}
