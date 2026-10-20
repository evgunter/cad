//! **A line×line fillet at a hairpin corner rounds the corner it was
//! asked to.** The fillet reads tan(φ/2) of the corner's turn φ off
//! the legs' split `(v₁ × v₂, v₁·v₂)`, and at a hairpin (φ = π − δ) the
//! quotient `across / (|v₁||v₂| + along)` cancels: its relative error
//! is about 2⁻⁵²/δ². The setback, the tangent points and the stored
//! sweep all read it, so the stored arc lands off its end vertex, off
//! tangency with its legs, and at a setback that is not the authored
//! radius's. The probe is the incoming ray east from the origin, the
//! arrival carrier back west at a turn δ short of a half turn, and the
//! fillet's radius `r`: the arrival anchor either near the origin
//! (the corner gate's lever `L·δ`) or a leg's length past it (lever
//! `L`, which lets δ go to `Kε/L`).
#![allow(clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/sugar.rs",
    "crates/profile/src/path.rs",
    "crates/profile/src/path/",
    "crates/profile/tests/common/",
];

use crate::common::{anchor_fit, tol, try_replay_at};
use geom_core::{Bounds, Interval, Point2, Real};
use profile::{ConstructedLoop, ReplayErrorKind, Segment, Step, Target};

const ULP: f64 = f64::EPSILON;

/// The probe: east from the origin, a fillet of radius `r`, and the
/// arrival carrier through `anchor` along `(dx, dy)`, then a metre
/// further and home.
fn hairpin(anchor: Point2<f64>, (dx, dy): (f64, f64), r: f64) -> Vec<Step<f64>> {
    vec![
        Step::At(Point2::new(0.0, 0.0)),
        Step::Toward { dx: 1.0, dy: 0.0 },
        Step::Fillet { radius: r },
        Step::At(anchor),
        Step::Toward { dx, dy },
        Step::Line(1.0),
        Step::LineTo(Target::Start),
    ]
}

/// The corner at `(l, 0)`, the arrival carrier turning back by `δ`
/// short of a half turn, its anchor `l` past the origin when `far`.
fn grid_case(l: f64, delta: f64, r: f64, far: bool) -> Vec<Step<f64>> {
    let (s, c) = delta.sin_cos();
    let anchor = if far {
        Point2::new(l - 2.0 * l * c, 2.0 * l * s)
    } else {
        Point2::new(0.0, l * delta.tan())
    };
    hairpin(anchor, (-c, s), r)
}

/// The fillet arc, its start and end vertices, and the vertices either
/// side of it.
fn fillet<T: Real>(lp: &ConstructedLoop<T>) -> (geom_core::Arc2<T>, [Point2<T>; 4]) {
    let segments = lp.segments();
    let i = segments
        .iter()
        .position(|s| matches!(s, Segment::Arc(_)))
        .unwrap_or_else(|| panic!("the loop stores no fillet arc: {segments:?}"));
    let Segment::Arc(arc) = segments[i] else {
        unreachable!("position found an arc")
    };
    let v = lp.vertices();
    let n = v.len();
    (
        arc,
        [v[(i + n - 1) % n], v[i], v[(i + 1) % n], v[(i + 2) % n]],
    )
}

/// How far the stored arc ends from its end vertex, and how far its
/// rim sits off each leg's line: three lengths that are zero for the
/// fillet the legs describe.
fn misses(lp: &ConstructedLoop<f64>) -> [f64; 3] {
    let (arc, [p, a, b, q]) = fillet(lp);
    let off_line = |x: Point2<f64>, y: Point2<f64>| {
        let d = y - x;
        ((arc.centre - x).perp_dot(d).abs() / d.norm() - arc.radius).abs()
    };
    [(arc.landing(a) - b).norm(), off_line(p, a), off_line(b, q)]
}

/// **A counterexample search over hairpins**: legs of 100 m and 1e4 m,
/// turns from 0.3 rad down past the corner gate, radii from 1e-8 m to
/// 1e4 m, both anchors. Every fillet the replay builds lands on its end
/// vertex and sits on both legs' lines to within 64 ulps of the scene.
#[test]
fn a_hairpin_fillet_keeps_its_end_vertex_and_its_tangency() {
    let mut built = 0;
    for far in [false, true] {
        for l in [100.0, 1e4] {
            let scale = 64.0 * ULP * 2.0 * l;
            for de in 1..=26 {
                let delta = 10f64.powf(-0.5 * f64::from(de));
                for re in -8..=8 {
                    let r = 10f64.powf(-0.5 * f64::from(re) - 4.0);
                    let Ok(lp) = try_replay_at::<f64>(&grid_case(l, delta, r, far)) else {
                        continue;
                    };
                    built += 1;
                    let [land, inc, out] = misses(&lp);
                    assert!(
                        land <= scale && inc <= scale && out <= scale,
                        "far {far}, L {l:e}, δ {delta:e}, r {r:e}: the fillet misses its end \
                         vertex by {land:e} m and its legs' lines by {inc:e} and {out:e} m, \
                         past {scale:e}"
                    );
                }
            }
        }
    }
    // Each family builds a few dozen at every ε row.
    assert!(built >= 200, "only {built} hairpins built");
}

/// One pinned hairpin: the authored data, and the fillet the reals give
/// it (mpmath at 50 digits from these f64 literals).
struct Pin {
    anchor: Point2<f64>,
    dir: (f64, f64),
    r: f64,
    t1: Point2<f64>,
    t2: Point2<f64>,
    centre: Point2<f64>,
}

/// Two hairpins the cancelling quotient got wrong past ε, and whose
/// loops validated: the far anchor's at δ = 1e-8, which it set back
/// 2.0e3 m where the reals give 6e3 m, and the near anchor's at
/// δ = 1e-4.
fn pins() -> [Pin; 2] {
    [
        Pin {
            anchor: Point2::new(-10000.0, 0.0002),
            dir: (-1.0, 1e-8),
            r: 3e-5,
            t1: Point2::new(4_000.000_000_000_000_5, 0.0),
            t2: Point2::new(4_000.000_000_000_000_5, 6e-5),
            centre: Point2::new(4_000.000_000_000_000_5, 3e-5),
        },
        Pin {
            anchor: Point2::new(0.0, 1.000_000_003_333_333_4),
            dir: (-0.999_999_995, 9.999_999_983_333_334e-5),
            r: 0.3,
            t1: Point2::new(4_000.000_005_000_000_3, 0.0),
            t2: Point2::new(4_000.000_035_000_000_5, 0.599_999_998_5),
            centre: Point2::new(4_000.000_005_000_000_3, 0.3),
        },
    ]
}

/// **The pinned hairpins against the reals.** At `f64` the tangent
/// points and the centre are the true fillet's to within 64 ulps of
/// the scene; at `Interval` the replay's enclosures hold them, where ε
/// is coarser than that resolution.
#[test]
fn the_pinned_hairpins_round_the_corner_the_reals_give() {
    let scale = 64.0 * ULP * 2e4;
    for (k, pin) in pins().into_iter().enumerate() {
        let steps = hairpin(pin.anchor, pin.dir, pin.r);
        let lp = try_replay_at::<f64>(&steps)
            .unwrap_or_else(|e| panic!("pin {k}: the f64 replay refused: {e}"));
        let (arc, [_, a, b, _]) = fillet(&lp);
        for (what, got, want) in [
            ("t1", a, pin.t1),
            ("t2", b, pin.t2),
            ("centre", arc.centre, pin.centre),
        ] {
            assert!(
                (got - want).norm() <= scale,
                "pin {k}: {what} {got:?} is {:e} m from the true {want:?}",
                (got - want).norm()
            );
        }
        match try_replay_at::<Interval>(&steps) {
            Ok(li) => {
                let (_, [_, a, b, _]) = fillet(&li);
                let holds = |e: Point2<Interval>, t: Point2<f64>| {
                    e.x.lo() <= t.x && t.x <= e.x.hi() && e.y.lo() <= t.y && t.y <= e.y.hi()
                };
                assert!(
                    holds(a, pin.t1) && holds(b, pin.t2),
                    "pin {k}: the Interval tangent points {a:?}, {b:?} miss the true ones"
                );
            }
            Err(e) => assert!(
                tol().get().eps < scale,
                "pin {k}: the Interval replay refused at ε {:e}, coarser than the scene's \
                 {scale:e}: {e}",
                tol().get().eps
            ),
        }
    }
}

/// **A radius that does not fit is refused, not rounded elsewhere.** At
/// the far anchor's δ = 1e-8 a 1e-4 m fillet sets back r·cot(δ/2) =
/// 2e4 m on a 1e4 m leg; the cancelling quotient read 6.7e3 m and
/// built an arc 1.3e-4 m off its end vertex, which validated.
#[test]
fn a_hairpin_radius_past_its_leg_is_refused() {
    let steps = hairpin(Point2::new(-10000.0, 0.0002), (-1.0, 1e-8), 1e-4);
    let err = match try_replay_at::<f64>(&steps) {
        Err(e) => e,
        Ok(lp) => panic!("the replay built {:?}", lp.vertices()),
    };
    let ReplayErrorKind::Path(path) = &err.kind else {
        panic!("not a path refusal: {err}")
    };
    let (_, _, setback, available) =
        anchor_fit(path).unwrap_or_else(|| panic!("not an anchor fit: {err}"));
    assert!(
        (setback - 2e4).abs() <= 1e-6 && available == 1e4,
        "the refusal reads setback {setback:e} of {available:e}: {err}"
    );
}
