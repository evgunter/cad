//! **The mint's `General` image is the carrier's one image on its wall**
//! (C2, [`crate::carrier_image`]): the wall's foot path interpolated on
//! the carrier's own knots, refined by knot insertion only where its
//! measured miss asks. A refusal of the image the mint produced is the
//! lane's gap ([`super::minted_refusal`]), never a tolerance offer.
//!
//! - A curved spline carrier, the rational one-span arc of a ring's
//!   circle on that ring, and a line on a flat wall whose `v` is not
//!   affine in height (control rows at `z = 0, 0.1, 1`) bend on their
//!   walls and take knots past their chains: each certifies at `f64` and
//!   at `Interval`.
//! - A line and a degree-1 spline on a flat bilinear wall follow their
//!   chains' images and keep them. Nor does a line a few ε off that wall
//!   take a knot: its feet's miss is its standoff, which no finer image
//!   lowers.
//! - The threshold: a line on the uneven wall whose chain misses by 0.6
//!   of the image's share of the band keeps its chain, one at 1.6 of it
//!   takes a knot.
//! - A miss beside a standoff across the wall takes a knot: the standoff
//!   is subtracted as a vector, not a norm.
//! - A spline carrier's image holds the carrier's knots, each at the
//!   multiplicity that keeps its continuity.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Decide, Interval, Point3, Vec3};

use super::spline_door_rows::band;
use super::{LaneGap, Pcurve, PcurveCache, PcurveCertifyError, minted_refusal};
use crate::FittedLane;
use crate::carrier_image::{CLOSED_FORM_SPANS, IMAGE_DEGREE};
use crate::recourse::Reading;

/// The unit cylinder over `z ∈ [−½, ½]` through `1.2` rad from `x̂`, in
/// two rational quadratic sub-arcs: its `u = ½` knot line is the joint.
fn ring_wall() -> NurbsSurface<f64> {
    let h = 0.6_f64;
    let mut ku = vec![0.0; 3];
    ku.extend([0.5; 2]);
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=4 {
        let theta = f64::from(j) * h / 2.0;
        let (reach, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        for z in [-0.5, 0.5] {
            control.push(Point3::new(reach * theta.cos(), reach * theta.sin(), z));
            weights.push(w);
        }
    }
    NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap()
}

/// The ring's circle at `z = 0` through its whole `1.2` rad, as the
/// rational one-span quadratic arc: a spline carrier that bends.
fn ring_arc() -> Curve3<f64> {
    let h = 0.6_f64;
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, h.tan(), 0.0),
                Point3::new((2.0 * h).cos(), (2.0 * h).sin(), 0.0),
            ],
            vec![1.0, h.cos(), 1.0],
        )
        .unwrap(),
    ))
}

/// The plane `y = 0` over `x ∈ [0, 1]`, its `v` quadratic in height:
/// control rows at `z = 0, 0.1, 1`, so `z(v) = 0.2v − 0.1v² + … ` is
/// not affine and a straight line's feet bend in the chart.
fn uneven_wall() -> NurbsSurface<f64> {
    let mut control = Vec::new();
    for x in [0.0, 1.0] {
        for z in [0.0, 0.1, 1.0] {
            control.push(Point3::new(x, 0.0, z));
        }
    }
    NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        control,
        vec![1.0; 6],
    )
    .unwrap()
}

/// A diagonal line across [`uneven_wall`], over `t ∈ [0, 1.2]`.
fn uneven_line() -> Curve3<f64> {
    let r = core::f64::consts::FRAC_1_SQRT_2;
    Curve3::line(
        Point3::new(0.05, 0.0, 0.05),
        geom::stated_direction(Vec3::new(r, 0.0, r)),
    )
}

/// The plane `y = 0` over `[0, 1]²` as a bilinear wall: its chart is
/// affine, so every line's feet are straight in it.
fn flat_wall() -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(
        k(),
        k(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
        ],
        vec![1.0; 4],
    )
    .unwrap()
}

/// What the mint does with `carrier` on `wall` over `span` at `T`: the
/// image `general_image_lane` derives, and its certificate's refusal as
/// the mint ends it ([`minted_refusal`]).
struct Minted<T: geom_core::Real> {
    image: NurbsCurve2<T>,
    certified: Result<(), PcurveCertifyError>,
}

fn mint<T: geom_core::Certifies>(
    lane: FittedLane<T>,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    wall: &NurbsSurface<f64>,
) -> Minted<T> {
    let lift = |x: f64| T::from_f64(x);
    let carrier = carrier.map_scalar(lift);
    let wall = wall.map_scalar(lift);
    let (t0, t1) = (lift(span.0), lift(span.1));
    let image = lane
        .general_image(&carrier, (t0, t1), &wall, band())
        .unwrap_or_else(|e| panic!("the producer images the carrier at {}: {e:?}", T::NAME));
    let surface = Surface::Nurbs(Arc::new(wall));
    let certified = PcurveCache::certify_general(
        Arc::new(image.clone()),
        t0,
        t1,
        &carrier,
        &surface,
        band(),
        Some(lane),
    )
    .map(|_| ())
    .map_err(|e| {
        minted_refusal(
            e,
            &Pcurve::General(Arc::new(image.clone())),
            &carrier,
            &surface,
        )
    });
    Minted { image, certified }
}

/// The control points of a carrier's chain at the image's degree: a
/// closed form's uniform spans.
const CLOSED_FORM_CHAIN: usize = CLOSED_FORM_SPANS + IMAGE_DEGREE;

/// A carrier whose feet bend on its wall takes knots past its chain of
/// `chain` control points. Where `certifies`, the image certifies;
/// anywhere else its refusal ends the lane's gap, "not yet"
/// ([`LaneGap::Image`], or the same check escalated in band), never a
/// tolerance offer.
fn bends_and_refines<T: Decide>(name: &str, minted: &Minted<T>, chain: usize, certifies: bool) {
    if let Err(got) = &minted.certified {
        let ending = got.ending(Reading::Build);
        assert!(
            matches!(
                got,
                PcurveCertifyError::NoGeneralLane {
                    gap: LaneGap::Image,
                    ..
                } | PcurveCertifyError::Escalated {
                    check: super::PcurveCheck::GeneralLane,
                    ..
                }
            ) && ending.contains(geom_core::NOT_YET_ENDING)
                && !ending.contains("loosen"),
            "{name} at {}: a refused image is the lane's gap: {got:?} :: {ending}",
            T::NAME
        );
    }
    assert!(
        !certifies || minted.certified.is_ok(),
        "{name} at {}: the image certifies: {:?}",
        T::NAME,
        minted.certified
    );
    assert!(
        minted.image.degree() == IMAGE_DEGREE && minted.image.control().len() > chain,
        "{name} at {}: the feet bend, so the image takes knots past its chain of {chain} \
         (degree {}, {} control points)",
        T::NAME,
        minted.image.degree(),
        minted.image.control().len()
    );
}

fn the_bending_carriers_refine<T: geom_core::Certifies>(lane: FittedLane<T>) {
    // A one-span quadratic's chain at the image's degree: one cubic span.
    bends_and_refines(
        "the ring's spline arc",
        &mint::<T>(lane, &ring_arc(), (0.0, 1.0), &ring_wall()),
        IMAGE_DEGREE + 1,
        true,
    );
    // Below `ε = 1e-10` the hull over-reads the image's miss on this
    // stretched chart
    // (`a-climbed-general-image-on-a-stretched-chart-reads-its-hull-wide-at-fine-eps`),
    // which ends the lane's gap.
    bends_and_refines(
        "a line on the uneven wall",
        &mint::<T>(lane, &uneven_line(), (0.0, 1.2), &uneven_wall()),
        CLOSED_FORM_CHAIN,
        band().zero() >= 1e-10,
    );
}

#[test]
fn the_bending_carriers_refine_at_f64() {
    the_bending_carriers_refine(FittedLane::<f64>::certified());
}

#[test]
fn the_bending_carriers_refine_at_interval() {
    the_bending_carriers_refine(FittedLane::<Interval>::certified());
}

/// A carrier whose chain's image follows it keeps that chain, and
/// certifies.
fn straight_feet_keep_their_chain<T: geom_core::Certifies>(lane: FittedLane<T>) {
    let line = Curve3::line(
        Point3::new(0.1, 0.0, 0.2),
        geom::stated_direction(Vec3::new(0.6, 0.0, 0.8)),
    );
    let spline = Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point3::new(0.1, 0.0, 0.2), Point3::new(0.7, 0.0, 1.0)],
            vec![1.0; 2],
        )
        .unwrap(),
    ));
    for (name, carrier, chain) in [
        ("the line", line, CLOSED_FORM_CHAIN),
        ("the degree-1 spline", spline, IMAGE_DEGREE + 1),
    ] {
        let minted = mint::<T>(lane, &carrier, (0.0, 1.0), &flat_wall());
        assert!(
            minted.image.control().len() == chain && minted.certified.is_ok(),
            "{name} at {}: its chain's image, certified (degree {}, {} control points): {:?}",
            T::NAME,
            minted.image.degree(),
            minted.image.control().len(),
            minted.certified
        );
    }
    // In band of the wall: its certificate is the incidence read's, but
    // its image is its chain's.
    let standoff = 3.0 * band().zero();
    let off = Curve3::line(
        Point3::new(0.1, standoff, 0.2),
        geom::stated_direction(Vec3::new(0.6, 0.0, 0.8)),
    );
    let image = lane
        .general_image(
            &off.map_scalar(|x: f64| T::from_f64(x)),
            (T::from_f64(0.0), T::from_f64(1.0)),
            &flat_wall().map_scalar(|x: f64| T::from_f64(x)),
            band(),
        )
        .unwrap();
    assert!(
        image.control().len() == CLOSED_FORM_CHAIN,
        "a line {standoff:e} m off the wall at {}: its standoff is no bend (degree {}, {} \
         control points)",
        T::NAME,
        image.degree(),
        image.control().len()
    );
}

#[test]
fn straight_feet_keep_their_chain_at_f64() {
    straight_feet_keep_their_chain(FittedLane::<f64>::certified());
}

#[test]
fn straight_feet_keep_their_chain_at_interval() {
    straight_feet_keep_their_chain(FittedLane::<Interval>::certified());
}

/// The largest miss of the chain's image of `carrier` on `wall` over
/// `(0, l)`, midway between its feet and net of the standoff its feet
/// carry, as the producer reads it, at `f64`: the image taken under a
/// band too wide for any miss to ask a knot.
fn chain_miss(carrier: &Curve3<f64>, l: f64, wall: &NurbsSurface<f64>) -> f64 {
    let wide = geom_core::Band::new(1e3, 2e3).unwrap();
    let image = FittedLane::<f64>::certified()
        .general_image(carrier, (0.0, l), wall, wide)
        .unwrap();
    assert_eq!(
        image.control().len(),
        CLOSED_FORM_CHAIN,
        "a band of a kilometre keeps the chain"
    );
    let gap = |t: f64| {
        let p = image.eval(t);
        wall.eval(p.x, p.y) - carrier.eval(t)
    };
    // The chain's feet: the Greville abscissae of its uniform cubic knots.
    let n = CLOSED_FORM_SPANS;
    let knot = |i: isize| (i.clamp(0, n as isize) as f64) / n as f64;
    let nodes: Vec<f64> = (0..CLOSED_FORM_CHAIN as isize)
        .map(|j| l * (knot(j - 2) + knot(j - 1) + knot(j)) / 3.0)
        .collect();
    nodes
        .windows(2)
        .map(|w| (gap(0.5 * (w[0] + w[1])) - (gap(w[0]) + gap(w[1])) * 0.5).norm())
        .fold(0.0, f64::max)
}

/// The length of [`uneven_line`] whose chain's miss is `share` of the
/// band: the miss grows as the fourth power of the length, read again
/// until it lands within 2% of the share.
fn uneven_line_at(share: f64) -> f64 {
    let target = share * band().zero();
    let mut l = 0.3_f64;
    for _ in 0..12 {
        let miss = chain_miss(&uneven_line(), l, &uneven_wall());
        if (miss / target - 1.0).abs() <= 0.02 {
            return l;
        }
        l *= (target / miss).powf(0.25);
    }
    panic!("no length of the uneven line reads {share} of the band");
}

/// **The threshold**: the image keeps its chain where the chain's miss
/// is within `IMAGE_MISS` of the band. A line on the uneven wall whose
/// miss is 0.6 of that share keeps its chain, and one at 1.6 of it takes
/// a knot: halving the share reds the first, doubling it the second.
fn the_threshold_is_the_images_share<T: geom_core::Certifies>(lane: FittedLane<T>) {
    // `IMAGE_MISS`, pinned here as a value: a row that read the constant
    // would move with it.
    let share = 1.0 / 16.0;
    for (r, refines) in [(0.6, false), (1.6, true)] {
        let l = uneven_line_at(r * share);
        let lift = |x: f64| T::from_f64(x);
        let image = lane
            .general_image(
                &uneven_line().map_scalar(lift),
                (lift(0.0), lift(l)),
                &uneven_wall().map_scalar(lift),
                band(),
            )
            .unwrap();
        assert!(
            (image.control().len() > CLOSED_FORM_CHAIN) == refines,
            "a line {l:e} m long whose chain's miss is {r} of the share at {}: takes a knot \
             {refines} (degree {}, {} control points)",
            T::NAME,
            image.degree(),
            image.control().len()
        );
    }
}

#[test]
fn the_threshold_is_the_images_share_at_f64() {
    the_threshold_is_the_images_share(FittedLane::<f64>::certified());
}

#[test]
fn the_threshold_is_the_images_share_at_interval() {
    the_threshold_is_the_images_share(FittedLane::<Interval>::certified());
}

/// **The standoff is read out as a vector.** A line 0.9 ε off the uneven
/// wall, across it, whose chain misses its path by 0.3 ε along it: read as
/// norms the excess is `√(0.9² + 0.3²) − 0.9 ≈ 0.05` ε, under the share,
/// and the image would keep its chain; read as vectors it is the chain's
/// 0.3 ε, and the image takes a knot.
fn a_miss_beside_a_standoff_takes_a_knot<T: geom_core::Certifies>(lane: FittedLane<T>) {
    let l = uneven_line_at(0.3);
    let r = core::f64::consts::FRAC_1_SQRT_2;
    let off = Curve3::line(
        Point3::new(0.05, 0.9 * band().zero(), 0.05),
        geom::stated_direction(Vec3::new(r, 0.0, r)),
    );
    let lift = |x: f64| T::from_f64(x);
    let image = lane
        .general_image(
            &off.map_scalar(lift),
            (lift(0.0), lift(l)),
            &uneven_wall().map_scalar(lift),
            band(),
        )
        .unwrap();
    assert!(
        image.control().len() > CLOSED_FORM_CHAIN,
        "a miss of 0.3 ε beside a standoff of 0.9 ε takes a knot at {} (degree {}, {} control \
         points)",
        T::NAME,
        image.degree(),
        image.control().len()
    );
}

#[test]
fn a_miss_beside_a_standoff_takes_a_knot_at_f64() {
    a_miss_beside_a_standoff_takes_a_knot(FittedLane::<f64>::certified());
}

#[test]
fn a_miss_beside_a_standoff_takes_a_knot_at_interval() {
    a_miss_beside_a_standoff_takes_a_knot(FittedLane::<Interval>::certified());
}

/// The whole unit cylinder over `z ∈ [0, 1]` in four rational quadratic
/// quarter arcs: closed along `u`, its seam the ruling at azimuth 0.
fn closed_wall() -> NurbsSurface<f64> {
    let w = core::f64::consts::FRAC_1_SQRT_2;
    let ring = [
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for (j, (x, y)) in ring.into_iter().enumerate() {
        for z in [0.0, 1.0] {
            control.push(Point3::new(x, y, z));
            weights.push(if j % 2 == 1 { w } else { 1.0 });
        }
    }
    let ku = vec![
        0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
    ];
    NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap()
}

/// The largest distance between `carrier` and `wall`'s point at `image`
/// over `span`, at 65 samples.
fn image_gap(
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    wall: &NurbsSurface<f64>,
    image: &NurbsCurve2<f64>,
) -> f64 {
    (0..=64)
        .map(|i| {
            let t = t0 + (t1 - t0) * f64::from(i) / 64.0;
            let p = image.eval(t);
            wall.eval(p.x, p.y).distance(carrier.eval(t))
        })
        .fold(0.0, f64::max)
}

/// **The foot path reads a closed wall's seam from the side it runs on**
/// (C2). The circle at `z = ½` of [`closed_wall`], whose seam ruling is
/// at azimuth 0:
///
/// - an arc from the seam toward positive azimuth starts at `u = 0`;
/// - the same arc run the other way round the axis leaves the seam
///   toward negative azimuth, and starts at `u = 1`, the same point of
///   the wall on the far side of its rectangle;
/// - an arc from azimuth −1 to the seam ends at `u = 1`.
///
/// Each image holds its carrier to the image's share of the band. A
/// path across the seam leaves the rectangle and is refused on its
/// miss.
///
/// Red against: the first foot taken as the nearest point alone (the
/// second arc starts at `u = 0`, stalls against the rectangle's side and
/// is refused), and each foot taken as the wall's nearest point with no
/// continuation (the third arc's last foot reads `u = 0`).
#[test]
fn the_foot_path_reads_a_closed_walls_seam_from_the_side_it_runs_on() {
    let wall = closed_wall();
    let circle = |axis: f64| {
        Curve3::circle(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::new(0.0, 0.0, axis),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        )
    };
    let lane = FittedLane::<f64>::certified();
    for (what, carrier, span, (u0, u1)) in [
        (
            "from the seam, up",
            circle(1.0),
            (0.0, 1.0),
            (Some(0.0), None),
        ),
        (
            "from the seam, down",
            circle(-1.0),
            (0.0, 1.0),
            (Some(1.0), None),
        ),
        ("to the seam", circle(1.0), (-1.0, 0.0), (None, Some(1.0))),
    ] {
        let image = lane
            .general_image(&carrier, span, &wall, band())
            .unwrap_or_else(|e| panic!("{what}: the arc is imaged: {e:?}"));
        let gap = image_gap(&carrier, span, &wall, &image);
        assert!(
            gap <= band().zero(),
            "{what}: the image holds the arc: {gap:e} m"
        );
        for (end, t, want) in [("start", span.0, u0), ("end", span.1, u1)] {
            let u = image.eval(t).x;
            assert!(
                want.is_none_or(|w: f64| (u - w).abs() <= 1e-9),
                "{what}: its {end} reads the seam at u = {want:?}: {u}"
            );
        }
    }
    let across = lane.general_image(&circle(1.0), (-0.5, 0.5), &wall, band());
    assert!(
        matches!(
            across,
            Err(PcurveCertifyError::FittedCertificate {
                limb: Some(crate::ssi::SsiLimb::ImageMiss),
                ..
            })
        ),
        "an arc across the seam is refused on its image's miss: {across:?}"
    );
}

/// A hairpin: the cubic profile from `(0, 0)` out along `x` and back to
/// `(0.3, gap)` above its start, extruded along `y`. Its two sheets stand
/// `gap` apart, far nearer than a step of the projection's seed grid,
/// and its return runs slower than its way out, so the grid's points on
/// the two sheets do not face each other.
fn folded_wall(gap: f64) -> NurbsSurface<f64> {
    let profile = [(0.0, 0.0), (3.0, 0.0), (1.5, gap), (0.3, gap)];
    let mut control = Vec::new();
    for (x, z) in profile {
        for y in [0.0, 1.0] {
            control.push(Point3::new(x, y, z));
        }
    }
    NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        vec![1.0; 8],
    )
    .unwrap()
}

/// **The foot path stays on the sheet of a folded wall its carrier lies
/// on** (C2). A carrier across the upper sheet of [`folded_wall`], from
/// `u = 0.8` back to `u = 0.55`: the wall's nearest point by the
/// projection's one seed lands on the lower sheet at its start and at
/// others of its points (the row's own check), and the image follows the
/// upper one, holding the carrier to the band.
///
/// Red against: the first foot taken from the nearest seed alone, and
/// each foot taken as the projection's with no continuation (either
/// reads a foot `gap` below the carrier and the image misses it).
#[test]
fn the_foot_path_stays_on_the_sheet_its_carrier_lies_on() {
    let gap = 0.02;
    let wall = folded_wall(gap);
    let at = |t: f64| wall.eval(0.8 - 0.25 * t, 0.3 + 0.5 * t);
    let points: Vec<Point3<f64>> = (0..=16).map(|i| at(f64::from(i) / 16.0)).collect();
    let carrier = Curve3::Nurbs(Arc::new(NurbsCurve3::interpolate(&points, 3).unwrap()));
    let jumps = (0..=64)
        .filter(|i| {
            let p = carrier.eval(f64::from(*i) / 64.0);
            wall.project(p).is_ok_and(|foot| foot.distance > 0.5 * gap)
        })
        .count();
    let start_jumps = wall
        .project(carrier.eval(0.0))
        .is_ok_and(|foot| foot.distance > 0.5 * gap);
    assert!(
        jumps > 0 && start_jumps,
        "the fixture folds: the projection's own seed reads the lower sheet at the carrier's \
         start and at {jumps} of its points"
    );
    let image = FittedLane::<f64>::certified()
        .general_image(&carrier, (0.0, 1.0), &wall, band())
        .unwrap_or_else(|e| panic!("the carrier is imaged: {e:?}"));
    let miss = image_gap(&carrier, (0.0, 1.0), &wall, &image);
    let lowest = (0..=64)
        .map(|i| image.eval(f64::from(i) / 64.0).x)
        .fold(f64::INFINITY, f64::min);
    assert!(
        miss <= 1e-6 && lowest > 0.5,
        "the image stays on the upper sheet: {miss:e} m off the carrier, u down to {lowest}"
    );
}

/// [`ring_wall`]'s cylinder over `z ∈ [−½, 1]` with its sub-arcs' shared
/// knot at `u = 0.4`: the same wall, its chart's speed along `u` jumping
/// across that line.
fn split_ring_wall() -> NurbsSurface<f64> {
    let h = 0.6_f64;
    let mut ku = vec![0.0; 3];
    ku.extend([0.4; 2]);
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=4 {
        let theta = f64::from(j) * h / 2.0;
        let (reach, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        for z in [-0.5, 1.0] {
            control.push(Point3::new(reach * theta.cos(), reach * theta.sin(), z));
            weights.push(w);
        }
    }
    NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap()
}

/// **A foot where the path crosses a wall's knot line stands on the
/// line.** The circle at `z = 0` of [`split_ring_wall`] crosses the line
/// `u = 0.4` at `t = 0.6`, where the chart's speed jumps and the foot
/// path with it. The image breaks there, and its point at the break is
/// on the line to the bit, so the span on either side lies in one cell
/// of the wall, and the image certifies at every ε.
///
/// Red against: no break at a knot line (the image misses the path's
/// kink past its cap), and the break's foot left where Newton put it, an
/// ulp past the line, where the certificate's hull of the span before it
/// reads the next cell (refused at ε = 1e-12).
#[test]
fn a_foot_at_a_walls_knot_line_stands_on_the_line() {
    let wall = split_ring_wall();
    let carrier = Curve3::circle(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.0,
        Vec3::new(1.0, 0.0, 0.0),
    );
    let minted = mint::<f64>(FittedLane::<f64>::certified(), &carrier, (0.0, 1.2), &wall);
    let at_the_line = minted.image.eval(0.6).x;
    assert!(
        at_the_line.to_bits() == 0.4_f64.to_bits(),
        "the image's point at the crossing is on the line u = 0.4: {at_the_line:e}"
    );
    assert!(
        minted.certified.is_ok(),
        "the image certifies: {:?}",
        minted.certified
    );
}
