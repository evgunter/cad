//! **A crossing's stored parameter decides nothing its bracket does
//! not** (the crate README of `geom-core`, DL5.2): what [`super::settle`]
//! and the split's interiority answer on the parameters one crossing
//! could be stored at, how far [`super::crossings`]' reach runs where the
//! certified form's enclosure is wide (a thin torus, a family of nets),
//! and that a leaf and its witness place alike or the leaf refuses.

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

/// The length of [`rising`]'s stretch that reads ON [`ground`], its
/// span a unit long: every parameter within `ZERO / SIN` of the crossing.
const STRETCH: f64 = 2.0 * ZERO / SIN;

fn settled(
    net: &NurbsCurve3<f64>,
    surface: &Surface<f64>,
    span: (f64, f64),
    meeting: Meeting,
) -> Result<Vec<f64>, CompositeRefusal> {
    settle(net, span, band(), &[], vec![meeting], &|p: Point3<f64>| {
        Margin::of(crate::implicit_residual(surface, p))
    })
    .map(|found| found.into_iter().map(|c| c.into_parts().0).collect())
}

/// **`settle` answers one way on the parameters one crossing's ON reading
/// admits, or refuses, and the split's interiority follows it.** The
/// carrier crosses the plane at `SIN`, so every parameter within
/// `ZERO / SIN` of the crossing reads ON; each is handed to `settle` with
/// that stretch as its reach.
///
/// - With the crossing two escalation lengths short of the span's end
///   the end lies inside the stretch, and all four parameters refuse at
///   the end's row. Placed on its own point alone each answered for
///   itself: the true crossing and one fifty zero bands before it
///   reported, one half a zero band short of the end dropped as the
///   end's, one five zero bands short escalated.
/// - With the span's end a thousand zero bands past the crossing, every
///   parameter of the stretch is reported, and
///   [`crate::curve_pair::Span::place_param_read`] (what `topo`'s
///   `split_edge_minting` asks) reads each inside: a parameter `settle`
///   reports is one the split takes, whichever was drawn.
///
/// [`crossings`] itself reports nothing on the first span: the walk reads
/// its end within the band of the plane first, and that is the end
/// vertex's meeting.
#[test]
fn settle_answers_one_way_or_refuses_on_admitted_parameters_and_the_split_follows() {
    use crate::curve_pair::{Place, Span};
    let net = rising();
    for t in [
        T_STAR,
        T_STAR - 50.0 * ZERO,
        1.0 - 0.5 * ZERO,
        1.0 - 5.0 * ZERO,
    ] {
        assert_eq!(residual(t).unwrap(), Sign::Zero, "t = {t} reads ON");
        let refused = settled(&net, &ground(), (0.0, 1.0), Meeting::Crossing(t, STRETCH));
        assert!(
            matches!(refused, Err(CompositeRefusal::Escalated(d)) if d.predicate == Some("spline_composite_root_at_end")),
            "the end lies within the reach of {t}: {refused:?}"
        );
    }
    // The same crossing on a carrier running on a thousand zero bands
    // past it.
    let d = Vec3::new((1.0 - SIN * SIN).sqrt(), 0.0, SIN);
    let long = segment(
        3,
        Point3::origin() - d * T_STAR,
        Point3::origin() + d * (1.0 - T_STAR + 1000.0 * ZERO),
    );
    let (_, end) = long.domain();
    // The carrier's parameter is its length over `scale`.
    let scale = 1.0 + 1000.0 * ZERO;
    let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(long.clone()));
    let span = Span::new(&carrier, (0.0, end));
    for along in [-90.0, -50.0, 0.0, 50.0, 90.0] {
        let t = (T_STAR + along * ZERO) / scale;
        let reported = settled(&long, &ground(), (0.0, end), Meeting::Crossing(t, STRETCH));
        assert!(
            matches!(reported.as_deref(), Ok([found]) if found.to_bits() == t.to_bits()),
            "no end within the reach of {t}: {reported:?}"
        );
        let place = span.place_param_read(t, band()).map(|(place, _)| place);
        assert_eq!(place.unwrap(), Place::Inside, "t = {t}");
    }
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

/// **A leaf and its witness place a crossing alike, or the leaf
/// refuses.** The cubic unit segment crosses the plane at a sine of
/// `10⁻³`, at `t = 0.3`. Its witness (the `f64` net) brackets the
/// crossing to the composite's rounding and reaches under a thousandth
/// of a zero band. The leaf is the same net with every control height
/// widened by `±0.4` of the zero band: its certified form reads no sign
/// within `0.4·zero / sin` of the crossing, so its bracket, which holds
/// every member's crossing, is hundreds of zero bands long, and its
/// reach is that length. Its proposal, where the middle of the residual's
/// enclosure changes sign, lies within that reach of its witness's.
///
/// Read against a span's end three hundred zero bands past the crossing,
/// which the leaf's reach holds and its witness's does not, the witness
/// reports the crossing and the leaf refuses at the end's row; against
/// one ten thousand zero bands past it, both report.
#[test]
fn a_leaf_and_its_witness_place_a_crossing_alike_or_the_leaf_refuses() {
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
    let plane = ground().map_scalar(Interval::point);
    let leaf = crossings(
        &fat,
        (Interval::point(0.0), Interval::point(1.0)),
        &plane,
        band(),
        |x| x,
    )
    .unwrap();
    assert!(
        witness.len() == 1 && leaf.len() == 1,
        "{witness:?} {leaf:?}"
    );
    let ((w, w_reach), (l, l_reach)) = (witness[0].into_parts(), leaf[0].into_parts());
    assert!(
        (w - t_star).abs() < 1e-12 && w_reach.along() < 1e-3 * ZERO,
        "the witness brackets the crossing to rounding: {w} reaching {}",
        w_reach.along()
    );
    assert!(
        l.lo().to_bits() == l.hi().to_bits(),
        "the leaf's proposal is one number: {l:?}"
    );
    // The span is a unit long, so a parameter difference is metres along it.
    let apart = (l.lo() - w).abs();
    assert!(
        apart <= l_reach.along() && l_reach.along() > 100.0 * ZERO,
        "the leaf reaches {} zero bands, its witness's proposal {} along",
        l_reach.along() / ZERO,
        apart / ZERO
    );
    let off = |p: Point3<Interval>| Margin::of(crate::implicit_residual(&plane, p));
    let leaf_settled = |end: f64| {
        settle(
            &fat,
            (Interval::point(0.0), Interval::point(end)),
            band(),
            &[],
            vec![Meeting::Crossing(l.lo(), l_reach.along())],
            &off,
        )
    };
    let witness_settled = |end: f64| {
        settled(
            &net,
            &ground(),
            (0.0, end),
            Meeting::Crossing(w, w_reach.along()),
        )
    };
    let near = t_star + 300.0 * ZERO;
    assert!(
        matches!(witness_settled(near).as_deref(), Ok([_])),
        "the witness's crossing is apart from the near end"
    );
    let refused = leaf_settled(near);
    assert!(
        matches!(refused, Err(CompositeRefusal::Escalated(d)) if d.predicate == Some("spline_composite_root_at_end")),
        "the near end lies within the leaf's reach: {refused:?}"
    );
    let far = t_star + 1e4 * ZERO;
    assert!(
        matches!(witness_settled(far).as_deref(), Ok([_]))
            && matches!(leaf_settled(far).as_deref(), Ok([_])),
        "both report against the far end"
    );
}

/// The thin torus of the row below, and the cubic segment leaving its
/// outer equator at `sin`: the segment and its length.
fn thin_torus(major: f64, minor: f64, sin: f64) -> (Surface<f64>, NurbsCurve3<f64>, f64) {
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
    (ring, net, len)
}

/// **On a thin torus the crossing's reach is bands long, and its readers
/// carry it.** The torus has a 100 m major radius and a 0.1 m tube; a
/// cubic segment leaves its outer equator at a sine of `10⁻³`. The
/// canonical quartic composed along the carrier carries terms of the
/// order of `R⁴`, so its certified enclosure reads no sign over a
/// stretch about the crossing: the bracket is that stretch, and the
/// reach the carrier's length over it, more than a hundred zero bands.
/// The proposal is where the metred `f64` residual changes sign, which
/// reads ON, and the certified form reads no sign there.
///
/// A span's end a hundred zero bands past the proposal is inside the
/// reach, and the crossing refuses at the end's row; a reach of zero
/// would report it there.
///
/// Measured beside this row before the bracket was returned (cubic
/// carrier, zero bands along the edge between the first midpoint the
/// form read no sign at and the residual's root, at sines `10⁻²` and
/// `10⁻³`): torus 10 m by 1 m, 0.06 and 0.02; 10 m by 1 cm, 0.2 and 24;
/// 100 m by 0.1 m, 2.4 and 244; a cylinder of radius 1 km, 0.1 and 1; a
/// plane whose frame origin lies 10 km along it, 0.1 and 5.
#[test]
fn on_a_thin_torus_the_reach_is_bands_long_and_the_end_reading_carries_it() {
    let (ring, net, len) = thin_torus(100.0, 0.1, 1e-3);
    let found = crossings(&net, (0.0, 1.0), &ring, band(), Interval::from_certified).unwrap();
    assert_eq!(found.len(), 1, "{found:?}");
    let (proposal, reach) = found[0].into_parts();
    let root = residual_root(&net, &ring);
    let off = |t: f64| crate::implicit_residual(&ring, net.eval(t)).abs();
    assert!(
        off(proposal) < 1e-3 * ZERO && (proposal - root).abs() * len <= reach.along(),
        "the proposal is the residual's root, inside the reach: {} off, {} along",
        off(proposal),
        (proposal - root).abs() * len
    );
    let (pieces, _, _) =
        composite_rows(&net, &ring, (0.0, 1.0), &Interval::from_certified).unwrap();
    assert!(
        strict_sign(value_at(&pieces[0], share(&pieces[0], proposal))).is_none(),
        "the certified form reads no sign at the proposal"
    );
    assert!(
        reach.along() > 100.0 * ZERO && reach.along() < 1e5 * ZERO,
        "the reach is {} zero bands",
        reach.along() / ZERO
    );
    let near = proposal + 100.0 * ZERO / len;
    let refused = settled(
        &net,
        &ring,
        (0.0, near),
        Meeting::Crossing(proposal, reach.along()),
    );
    assert!(
        matches!(refused, Err(CompositeRefusal::Escalated(d)) if d.predicate == Some("spline_composite_root_at_end")),
        "an end a hundred zero bands on lies within the reach: {refused:?}"
    );
}

/// **A clean crossing of a thinner torus is found.** On a torus of 100 m
/// by 1 cm at a sine of `10⁻²` the form reads no sign over a stretch in
/// which a parameter reads in the band's gap. The proposal is not that
/// parameter but the residual's own sign change inside the bracket, which
/// reads ON; a search that stopped on the first parameter the form read
/// no sign at escalated here on its iterate.
#[test]
fn a_clean_crossing_of_a_thinner_torus_is_found_inside_its_bracket() {
    let (ring, net, len) = thin_torus(100.0, 0.01, 1e-2);
    let found = crossings(&net, (0.0, 1.0), &ring, band(), Interval::from_certified).unwrap();
    assert_eq!(found.len(), 1, "{found:?}");
    let (proposal, reach) = found[0].into_parts();
    let root = residual_root(&net, &ring);
    assert!(
        (proposal - root).abs() * len <= reach.along().min(1e-3 * ZERO),
        "the proposal is the residual's root: {} along, reaching {}",
        (proposal - root).abs() * len,
        reach.along()
    );
}
