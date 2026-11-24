//! Measurement rows for the work item
//! `work/scalar/a-crossing-parameters-check-bounds-across-the-face-and-its-readers-decide-along-the-edge.md`:
//! what [`super::settle`] answers on parameters the ON reading admits, and
//! how far [`super::crossings`]' proposal sits from the crossing where the
//! certified form's enclosure is wide there. Each row pins today's
//! behaviour.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::{NurbsCurve3, Surface};
use geom_core::interval::certification::Certification;
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Interval, Point3, Vec3};

use geom_core::k_stats::decide;
use geom_core::{Margin, Sign};

use super::{
    CompositeRefusal, Meeting, composite_rows, crossings, settle, share, strict_sign, value_at,
};

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

/// The sine of the hand-moved rows' crossing angle: about 0.57°.
const SIN: f64 = 1e-2;

/// The true crossing's parameter on [`rising`]: two escalation lengths
/// short of the span's end, the span a unit long.
const T_STAR: f64 = 1.0 - 20.0 * ZERO;

/// The plane `z = 0`.
fn ground() -> Surface<f64> {
    Surface::Plane {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
    }
}

/// A cubic unit segment rising through [`ground`] at [`SIN`], crossing
/// it at `t = T_STAR`.
fn rising() -> NurbsCurve3<f64> {
    let d = Vec3::new((1.0 - SIN * SIN).sqrt(), 0.0, SIN);
    segment(
        3,
        Point3::origin() - d * T_STAR,
        Point3::origin() + d * (1.0 - T_STAR),
    )
}

/// What `spline_composite_root_on` reads at `t`, as [`crossings`] reads it.
fn residual(t: f64) -> Result<Sign, CompositeRefusal> {
    decide(
        "spline_composite_root_on",
        Margin::of(crate::implicit_residual(&ground(), rising().eval_at(t))),
        band(),
    )
    .map_err(CompositeRefusal::Escalated)
}

/// **`settle` and the split's interiority reading answer alike, three
/// ways, on parameters the ON reading admits.** The carrier crosses the
/// plane at `SIN`, two escalation lengths short of its end, so every
/// parameter from a hundred zero bands before the true crossing to the
/// end reads ON. On each, `settle` and
/// [`crate::curve_pair::Span::place_param_read`] (what
/// `topo`'s `split_edge_minting` asks) agree: the true crossing and one
/// fifty zero bands before it are reported and inside; one half a zero
/// band short of the end is dropped and at the end
/// (`SplitParamNotInterior`); one five zero bands short escalates at
/// both. So a parameter `settle` reports is one the split reads inside,
/// and the three-way answer is the candidate's, not the crossing's.
///
/// [`crossings`] itself reports nothing on this carrier: the walk reads
/// the span's end within the band of the plane first, and that is the
/// end vertex's meeting.
#[test]
fn settle_and_the_splits_interiority_answer_alike_on_admitted_parameters() {
    use crate::curve_pair::{Place, Span, SpanEnd};
    let net = rising();
    let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(net.clone()));
    let span = Span::new(&carrier, (0.0, 1.0));
    let placed = |t: f64| {
        assert_eq!(residual(t).unwrap(), Sign::Zero, "t = {t} reads ON");
        let settled = settle(
            &net,
            (0.0, 1.0),
            band(),
            &[],
            vec![Meeting::Crossing(t)],
            &residual,
        );
        (
            settled,
            span.place_param_read(t, band()).map(|(place, _)| place),
        )
    };
    for t in [T_STAR, T_STAR - 50.0 * ZERO] {
        let (settled, place) = placed(t);
        assert!(
            matches!(&settled, Ok(ts) if ts.len() == 1 && ts[0].to_bits() == t.to_bits()),
            "reported at {t}: {settled:?}"
        );
        assert_eq!(place.unwrap(), Place::Inside, "t = {t}");
    }
    let (settled, place) = placed(1.0 - 0.5 * ZERO);
    assert!(
        matches!(&settled, Ok(ts) if ts.is_empty()),
        "dropped: {settled:?}"
    );
    assert_eq!(place.unwrap(), Place::End(SpanEnd::Stop));
    let (settled, place) = placed(1.0 - 5.0 * ZERO);
    assert!(
        matches!(settled, Err(CompositeRefusal::Escalated(_))),
        "in the gap of the end: {settled:?}"
    );
    assert!(place.is_err(), "in the gap of the end: {place:?}");
    let found = crossings(
        &net,
        (0.0, 1.0),
        &ground(),
        band(),
        Interval::from_certified,
    );
    assert!(
        matches!(&found, Ok(ts) if ts.is_empty()),
        "the walk reads the end within the band first: {found:?}"
    );
}

/// **At `Interval` the leaf's proposal sits bands along the edge from
/// its witness's.** The cubic unit segment crosses the plane at a sine
/// of `10⁻³`, at `t = 0.3`. Its witness (the `f64` net) bisects to the
/// crossing. The leaf is the same net with every control height widened
/// by `±0.4` of the zero band: its certified form reads no sign within
/// `0.4·zero / sin` of the crossing, the first bisection midpoint inside
/// that stretch reads ON at every member, and it is returned. The two
/// proposals are both admitted and lie more than a hundred zero bands
/// apart along the edge, so a reader that places the stored parameter
/// along the edge (an end, a trim edge, the crossing before it) reads
/// the leaf and its witness at two places.
#[test]
fn a_leafs_proposal_sits_bands_along_the_edge_from_its_witnesss() {
    let (sin, t_star, h) = (1e-3, 0.3, 0.4 * ZERO);
    let d = Vec3::new((1.0f64 - sin * sin).sqrt(), 0.0, sin);
    let net = segment(
        3,
        Point3::origin() - d * t_star,
        Point3::origin() + d * (1.0 - t_star),
    );
    let witness = crossings(
        &net,
        (0.0, 1.0),
        &ground(),
        band(),
        Interval::from_certified,
    )
    .unwrap();
    let fat = net
        .map_scalar(Interval::point)
        .map_points(|q| Point3::new(q.x, q.y, q.z + Interval::from_bounds(-h, h)));
    let leaf = crossings(
        &fat,
        (Interval::point(0.0), Interval::point(1.0)),
        &ground().map_scalar(Interval::point),
        band(),
        |x| x,
    )
    .unwrap();
    assert!(
        witness.len() == 1 && leaf.len() == 1,
        "{witness:?} {leaf:?}"
    );
    assert!(
        (witness[0] - t_star).abs() < 1e-15,
        "the witness bisects to the crossing: {witness:?}"
    );
    assert!(
        leaf[0].lo().to_bits() == leaf[0].hi().to_bits(),
        "the leaf's proposal is one number: {leaf:?}"
    );
    // The span is a unit long, so a parameter difference is metres along it.
    let along = (leaf[0].lo() - witness[0]).abs();
    assert!(
        along * sin + h <= ZERO,
        "the leaf's proposal reads ON at every member: {along} along"
    );
    assert!(
        along > 100.0 * ZERO,
        "the proposals lie {} zero bands apart along the edge",
        along / ZERO
    );
}

/// **On a thin torus the `f64` proposal sits bands along the edge from
/// the residual's own root.** The torus has a 100 m major radius and a
/// 0.1 m tube; a cubic segment leaves its outer equator at a sine of
/// `10⁻³`. The canonical quartic composed along the carrier carries
/// terms of the order of `R⁴`, so its certified enclosure reads no sign
/// over a stretch about the crossing, and [`super::bisect_row`] returns
/// the first midpoint in that stretch that reads ON. The metred `f64`
/// residual changes sign at one parameter, found here to adjacent
/// scalars; the proposal lies more than a hundred zero bands along the
/// edge from it, reads ON, and the certified form reads no sign at
/// either. Nothing in the arm compares that stretch to the band.
///
/// Measured beside this row (cubic carrier, zero bands along the edge
/// between the proposal and the residual's root, at sines `10⁻²` and
/// `10⁻³`): torus 10 m by 1 m, 0.06 and 0.02; 10 m by 1 cm, 0.2 and 24;
/// 100 m by 0.1 m, 2.4 and 244; a cylinder of radius 1 km, 0.1 and 1; a
/// plane whose frame origin lies 10 km along it, 0.1 and 5. On a
/// degree-7 carrier the 10 m by 1 cm torus reads 5.7 at `10⁻²`. A torus
/// of 100 m by 1 cm refuses at `10⁻²`: a midpoint in the stretch reads
/// in the band's gap, and the bisection escalates on it.
#[test]
fn on_a_thin_torus_the_proposal_sits_bands_along_the_edge_from_the_residuals_root() {
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
    let found = crossings(&net, (0.0, 1.0), &ring, band(), Interval::from_certified).unwrap();
    assert_eq!(found.len(), 1, "{found:?}");
    let (proposal, root) = (found[0], residual_root(&net, &ring));
    let off = |t: f64| crate::implicit_residual(&ring, net.eval(t)).abs();
    assert!(
        off(root) < 1e-3 * ZERO,
        "the residual's root: {}",
        off(root)
    );
    assert!(
        off(proposal) <= ZERO,
        "the proposal reads ON: {}",
        off(proposal)
    );
    let (pieces, _) = composite_rows(&net, &ring, (0.0, 1.0), &Interval::from_certified).unwrap();
    for t in [proposal, root] {
        assert!(
            strict_sign(value_at(&pieces[0], share(&pieces[0], t))).is_none(),
            "the certified form reads no sign at {t}"
        );
    }
    let along = (proposal - root).abs() * len;
    assert!(
        along > 100.0 * ZERO,
        "the proposal lies {} zero bands along the edge from the residual's root",
        along / ZERO
    );
}
