//! Where the ring-composite arm's crossing parameter stands along the
//! edge: a crossing is a proposal, read ON the surface within the band's
//! zero, so another admissible candidate may lie bands along the edge
//! from it (`work/scalar/a-crossing-parameters-check-bounds-across-the-face-and-its-readers-decide-along-the-edge.md`).

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::{NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Interval, Point3, Vec3};

use crate::curve_face::{OnSpan, cross};
use crate::curve_pair::Span;

const ZERO: f64 = 1e-9;

fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

fn unit(v: Vec3<f64>) -> Vec3<f64> {
    v * (1.0 / v.norm())
}

/// A Bézier carrier of `degree` that is the straight segment from `a`
/// to `b`, its parameter `t ∈ [0, 1]` linear along it.
fn segment(degree: usize, a: Point3<f64>, b: Point3<f64>) -> NurbsCurve3<f64> {
    let mut k = vec![0.0; degree + 1];
    k.extend(vec![1.0; degree + 1]);
    let kv = KnotVector::clamped(k, degree).unwrap();
    #[allow(clippy::cast_precision_loss)]
    let control = (0..=degree)
        .map(|i| a + (b - a) * (i as f64 / degree as f64))
        .collect();
    NurbsCurve3::new(kv, control, vec![1.0; degree + 1]).unwrap()
}

/// The `f64` residual's own sign change on `[0, 1]`, bisected to
/// adjacent scalars.
fn residual_root(net: &NurbsCurve3<f64>, surface: &Surface<f64>) -> f64 {
    let r = |t: f64| crate::implicit_residual(surface, net.eval(t));
    let (mut lo, mut hi) = (0.0, 1.0);
    let left = r(lo) > 0.0;
    for _ in 0..80 {
        let c = 0.5 * (lo + hi);
        if !(lo < c && c < hi) {
            break;
        }
        if (r(c) > 0.0) == left {
            lo = c;
        } else {
            hi = c;
        }
    }
    0.5 * (lo + hi)
}

/// The one crossing strictly inside the span.
fn crossing<T: geom_core::Decide>(net: &NurbsCurve3<T>, surface: &Surface<T>) -> T {
    let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(net.clone()));
    let span = Span::new(&carrier, (T::zero(), T::one()));
    let found = cross(&span, surface, band()).unwrap();
    match found.meetings[..] {
        [m] => match m.on_span {
            OnSpan::Inside { t } => t,
            OnSpan::End(end) => panic!("the crossing reads at {end:?}"),
        },
        ref other => panic!("one crossing: {other:?}"),
    }
}

/// **At `Interval` the stored crossing sits bands along the edge from its
/// witness's.** The cubic unit segment crosses the plane at a sine of
/// `10⁻³`, at `t = 0.3`. Its witness (the `f64` net) halves to the
/// crossing. The leaf is the same net with every control height widened
/// by `±0.4` of the zero band: its form proves no sign within
/// `0.4·zero / sin` of the crossing, where its halving reads ON at every
/// member, and the storing door admits that bracket's middle. The two
/// stored values both read ON and lie more than a hundred zero bands
/// apart along the edge, so a reader that places the stored parameter
/// along the edge (an end, a trim edge, the crossing before it) reads
/// the leaf and its witness at two places.
#[test]
fn a_leafs_stored_crossing_sits_bands_along_the_edge_from_its_witnesss() {
    let (sin, t_star, h) = (1e-3, 0.3, 0.4 * ZERO);
    let d = Vec3::new((1.0f64 - sin * sin).sqrt(), 0.0, sin);
    let net = segment(
        3,
        Point3::origin() - d * t_star,
        Point3::origin() + d * (1.0 - t_star),
    );
    let ground = Surface::Plane {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
    };
    let witness = crossing(&net, &ground);
    let point = |x: f64| Interval::from_bounds(x, x);
    let fat = net
        .map_scalar(point)
        .map_points(|q| Point3::new(q.x, q.y, q.z + Interval::from_bounds(-h, h)));
    let ground = ground.map_scalar(point);
    let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(fat.clone()));
    let leaf = crate::NurbsLane::<Interval>::certified()
        .admit_crossing(&carrier, crossing(&fat, &ground), &ground, band())
        .unwrap();
    assert!(
        (witness - t_star).abs() < 1e-12,
        "the witness halves to the crossing: {witness}"
    );
    assert!(
        leaf.lo().to_bits() == leaf.hi().to_bits(),
        "the stored crossing is one number: {leaf:?}"
    );
    // The span is a unit long, so a parameter difference is metres along it.
    let along = (leaf.lo() - witness).abs();
    assert!(
        along * sin + h <= ZERO,
        "the leaf's crossing reads ON at every member: {along} along"
    );
    assert!(
        along > 100.0 * ZERO,
        "the crossings lie {} zero bands apart along the edge",
        along / ZERO
    );
}

/// **On a thin torus the crossing sits bands along the edge from the
/// residual's own root.** The torus has a 100 m major radius and a 0.1 m
/// tube; a cubic segment leaves its outer equator at a sine of `10⁻³`.
/// The canonical quartic composed along the carrier carries terms of the
/// order of `R⁴`, so at `f64` its sign near the crossing is its rounding's,
/// and the halving stops on the first point that reads ON. The metred
/// residual changes sign at one parameter, found here to adjacent
/// scalars; the crossing lies more than a zero band along the edge from
/// it, and reads ON.
///
/// Measured beside this row (zero bands along the edge between the
/// arm's crossing and the residual's root, cubic and degree-7 carriers,
/// at sines `10⁻²` and `10⁻³`): torus 10 m by 1 m, below 0.01 at both;
/// 10 m by 1 cm, 0.01 at `10⁻²`, and in the band's gap at `10⁻³`
/// (refused: the form's lower bound on the distance reads within the
/// band's escalation); 100 m by 0.1 m, 0.3 and 4 on a cubic, 0.2 and 6
/// at degree 7; 100 m by 1 cm, 0.2 and 7 at `10⁻²`, in the gap at
/// `10⁻³`; a cylinder or a sphere of radius 1 km, 0.02 and 0.2; a plane
/// whose frame origin lies 10 km along it, 0.02 and below 0.01.
#[test]
fn on_a_thin_torus_the_crossing_sits_bands_along_the_edge_from_the_residuals_root() {
    let (major, minor, sin) = (100.0, 0.1, 1e-3);
    let n = unit(Vec3::new(0.3, 0.5, 0.81));
    let e1 = unit(n.cross(Vec3::unit_z()));
    let e2 = n.cross(e1);
    let q = Point3::new(0.1, 0.2, 0.3);
    // `q` on the outer equator, the carrier along the equator's tangent
    // there, tilted outward.
    let ring = Surface::Torus {
        frame: geom::Frame::new(q - e1 * (major + minor), n, e1),
        major_radius: major,
        minor_radius: minor,
    };
    let d = unit(e2 * (1.0f64 - sin * sin).sqrt() + e1 * sin);
    let len = 0.5 * (major + minor) * sin;
    let net = segment(3, q - d * (0.37 * len), q + d * (0.63 * len));
    let (found, root) = (crossing(&net, &ring), residual_root(&net, &ring));
    let off = |t: f64| crate::implicit_residual(&ring, net.eval(t)).abs();
    assert!(
        off(root) < 1e-3 * ZERO,
        "the residual's root: {}",
        off(root)
    );
    assert!(off(found) <= ZERO, "the crossing reads ON: {}", off(found));
    let along = (found - root).abs() * len;
    assert!(
        along > ZERO,
        "the crossing lies {} zero bands along the edge from the residual's root",
        along / ZERO
    );
}
