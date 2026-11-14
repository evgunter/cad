//! **Each image form's speed bound against the sampled image**
//! ([`Pcurve::speed_bound`], `speed`'s module docs).
//!
//! A row samples the image on a fine schedule and reads its per-axis
//! speed as the largest difference quotient, which by the mean value
//! theorem never exceeds the true sup; the bound must dominate it on
//! every pose. Where the bound is attained the row pins it from below
//! too, so a padded rewrite reds; where it is not, the row states the
//! measured slack.

use core::f64::consts::{FRAC_PI_2, PI, TAU};
use std::sync::Arc;

use geom::Quarter;
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3, Vec2, Vec3};

use super::projected::{FramedCarrier, ProjectedChart, ProjectedImage};
use super::speed::{general_speeds, iso_arc_rate};
use super::*;
use crate::offset::Nappe;

/// The image's largest difference quotient per axis over `[t0, t1]` on
/// `n` steps, an angle channel's differences read in `(−π, π]` (a
/// projected image's azimuth is named on one branch per piece, so one
/// step can cross its cut while the angle moves a little).
fn sampled(image: &Pcurve<f64>, (t0, t1): (f64, f64), n: u32, angles: (bool, bool)) -> (f64, f64) {
    let wrap = |d: f64, angle: bool| {
        if angle {
            (d + PI).rem_euclid(TAU) - PI
        } else {
            d
        }
    };
    let h = (t1 - t0) / f64::from(n);
    let mut prev = image.eval(t0);
    let (mut su, mut sv) = (0.0f64, 0.0f64);
    for i in 1..=n {
        let p = image.eval(t0 + (t1 - t0) * f64::from(i) / f64::from(n));
        su = su.max(wrap(p.x - prev.x, angles.0).abs() / h);
        sv = sv.max(wrap(p.y - prev.y, angles.1).abs() / h);
        prev = p;
    }
    (su, sv)
}

/// `sampled ≤ bound` on both axes, at a relative rounding allowance far
/// below any chord count's `ceil`. Answers the two ratios
/// `sampled/bound`, `0` for a zero bound, so a collapsed bound never
/// reads as attained.
fn dominates(what: &str, sampled: (f64, f64), bound: (f64, f64)) -> (f64, f64) {
    for (axis, s, b) in [("u", sampled.0, bound.0), ("v", sampled.1, bound.1)] {
        assert!(
            s <= b * (1.0 + 1e-9) + 1e-12,
            "{what}: the image moves at {s:.17e} along {axis}, past its bound {b:.17e} — a \
             chord schedule sized from it would under-count"
        );
    }
    let ratio = |s: f64, b: f64| if b > 0.0 { s / b } else { 0.0 };
    (ratio(sampled.0, bound.0), ratio(sampled.1, bound.1))
}

/// **An arc rim's rate bound dominates its image's speed**, at one end
/// weight and at unequal ones, and is the sup the image reaches.
#[test]
fn an_arc_rims_rate_bounds_its_image_at_every_end_weight() {
    let angle = FRAC_PI_2;
    let breaks = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    for ends in [vec![], vec![1.0, 2.0], vec![2.0, 1.0], vec![1.0, 1e4]] {
        let image = Pcurve::IsoArc {
            p0: Point2::new(0.0, 0.0),
            pd: Vec2::new(1.0, 0.0),
            t0: 0.0,
            angle,
            breaks: breaks.clone(),
            ends: ends.clone().into(),
        };
        let bound = iso_arc_rate(angle, 1, &ends);
        assert_eq!(
            image.speed_bound(0.0, angle).unwrap(),
            (bound, 0.0),
            "ends {ends:?}: the image's bound is its displacement times the rate"
        );
        let (fastest, _) = sampled(&image, (0.0, angle), 20_000, (false, false));
        assert!(
            fastest <= bound * (1.0 + 1e-9),
            "ends {ends:?}: the image moves at {fastest}, past its rate bound {bound}"
        );
        assert!(
            fastest >= 0.99 * bound,
            "ends {ends:?}: the bound {bound} is the sup the image reaches, not {fastest}"
        );
    }
}

/// A degree-1 chart image: a polyline whose per-span speed IS a
/// difference coefficient, so the hull bound is the sup exactly.
fn polyline_image() -> geom::NurbsCurve2<f64> {
    let n = 9;
    let pts: Vec<Point2<f64>> = (0..n)
        .map(|i| {
            let t = f64::from(i) / f64::from(n - 1);
            Point2::new(1.0 + t + 0.15 * (6.0 * t).sin(), 0.4 * t * t - 0.2 * t)
        })
        .collect();
    let mut knots = vec![0.0];
    knots.extend((0..n).map(|i| f64::from(i) / f64::from(n - 1)));
    knots.push(1.0);
    let kv = KnotVector::clamped(knots, 1).unwrap();
    geom::NurbsCurve2::new(kv, pts, vec![1.0; n as usize]).unwrap()
}

/// A degree-3 image whose derivative-net maximum sits on an interior
/// coefficient (`Q_1^u = 3.0` against `1.2` elsewhere): a degree-2
/// derivative basis peaks at ½, so the hull is a genuine over-estimate
/// (sampled ≈ 2.4 against 3.0).
fn interior_max_cubic() -> geom::NurbsCurve2<f64> {
    let kv = interior_max_knots();
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(0.2, 0.0),
        Point2::new(1.2, 0.5),
        Point2::new(1.6, 1.0),
        Point2::new(1.8, 1.0),
    ];
    geom::NurbsCurve2::new(kv, pts, vec![1.0; 5]).unwrap()
}

fn interior_max_knots() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap()
}

/// A degree-2 image whose maximum is attained only at its interior knot
/// (`Q_1^u = 1.6` against `0.4`): a uniform grid can miss it, which is
/// why [`sampled_net_speeds`] reads both sides of every interior knot.
fn interior_knot_quadratic() -> geom::NurbsCurve2<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(0.1, 0.3),
        Point2::new(0.9, 0.4),
        Point2::new(1.0, 0.7),
    ];
    geom::NurbsCurve2::new(kv, pts, vec![1.0; 4]).unwrap()
}

/// `|u′|` and `|v′|` at `samples + 1` parameters across the domain plus
/// both sides of every interior knot.
fn sampled_net_speeds(c: &geom::NurbsCurve2<f64>, samples: usize) -> (f64, f64) {
    let (d0, d1) = c.domain();
    let span = d1 - d0;
    #[allow(clippy::cast_precision_loss)]
    let mut ts: Vec<f64> = (0..=samples)
        .map(|i| d0 + span * i as f64 / samples as f64)
        .collect();
    for k in c.knot_values().breakpoints().filter(|&k| k > d0 && k < d1) {
        ts.push(k - span * 1e-9);
        ts.push(k + span * 1e-9);
    }
    ts.into_iter().fold((0.0f64, 0.0f64), |(su, sv), t| {
        let d = c.deriv(t.clamp(d0, d1));
        (su.max(d.x.abs()), sv.max(d.y.abs()))
    })
}

/// **The general image's certified sup dominates the sampled speeds,
/// and where it is attained it is not padded.** Three legs: the max on
/// an end coefficient (attained), on an interior coefficient at its
/// knot (attained at one point), and mid-span (not attained, loose by
/// more than 20%). A bound folded over the end coefficients alone passes
/// the first leg and fails the other two; a scaled-down one fails all
/// three.
#[test]
fn general_speeds_dominate_the_sampled_image_speeds() {
    for (name, image, attained) in [
        (
            "degree-1 polyline, max at an end coefficient",
            polyline_image(),
            true,
        ),
        (
            "degree-2, max at the interior knot",
            interior_knot_quadratic(),
            true,
        ),
        ("degree-3, max mid-span", interior_max_cubic(), false),
    ] {
        let (d0, d1) = image.domain();
        let (su, sv) = general_speeds(&image, (d0, d1)).expect("a polynomial image");
        let (wu, wv) = sampled_net_speeds(&image, 4096);
        assert!(
            wu > 0.0 && wv > 0.0,
            "{name}: the fixture moves on both axes"
        );
        assert!(
            wu <= su && wv <= sv,
            "{name}: certified ({su:.17e}, {sv:.17e}) is dominated by sampled \
             ({wu:.17e}, {wv:.17e})"
        );
        if attained {
            // Each side's outward rounding only: `1e-14` relative is ~45
            // ulps, measured at ~6e-16.
            assert!(
                su <= wu * (1.0 + 1e-14) && sv <= wv * (1.0 + 1e-14),
                "{name}: an attained bound ({su:.17e}, {sv:.17e}) this far above the \
                 sampled ({wu:.17e}, {wv:.17e}) is slack"
            );
        } else {
            assert!(
                su > wu * 1.2,
                "{name}: convexity-loose here, {su} against {wu}"
            );
        }
    }
}

/// **A constant weight answers as its polynomial; a varying one, a read
/// past the domain and a fitted image refuse; a C⁰ kink is admitted.**
#[test]
fn the_general_bounds_refusals_and_admissions() {
    let base = interior_max_cubic();
    let (d0, d1) = base.domain();
    let plain = general_speeds(&base, (d0, d1)).unwrap();
    let with = |w: Vec<f64>| {
        geom::NurbsCurve2::new(interior_max_knots(), base.control().to_vec(), w).unwrap()
    };
    assert_eq!(
        general_speeds(&with(vec![2.0; 5]), (d0, d1)),
        Ok(plain),
        "constant weights cancel"
    );
    assert_eq!(
        general_speeds(&with(vec![1.0, 1.0, 0.5, 1.0, 1.0]), (d0, d1)),
        Err(NoSpeedBound::RationalGeneral)
    );
    assert_eq!(
        general_speeds(&base, (d0, d1 + 0.25)),
        Err(NoSpeedBound::OutsideDomain)
    );
    assert_eq!(
        general_speeds(&base, (d0, d1 * 0.5)),
        Ok(plain),
        "a sub-range reads the whole domain's bound"
    );
    assert_eq!(
        Pcurve::Fitted(Arc::new(base)).speed_bound(d0, d1),
        Err(NoSpeedBound::Fitted)
    );
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(0.3, 0.1),
        Point2::new(0.5, 0.9),
        Point2::new(0.7, 0.2),
        Point2::new(1.0, 1.0),
    ];
    let kink = geom::NurbsCurve2::new(kv, pts, vec![1.0; 5]).unwrap();
    let (su, sv) = general_speeds(&kink, (0.0, 1.0)).expect("a C⁰ kink is Lipschitz");
    let (wu, wv) = sampled_net_speeds(&kink, 4096);
    assert!(wu > 0.0 && wv > 0.0 && wu <= su && wv <= sv);
    // What the bound does not guard because the knot vector refuses it:
    // degree 0, and an interior multiplicity above the degree.
    assert!(KnotVector::clamped(vec![0.0, 1.0], 0).is_err(), "degree 0");
    assert!(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0], 2).is_err(),
        "interior multiplicity p + 1"
    );
}

/// **A harmonic image's bound is the sup it reaches, and an iso line's
/// is its velocity.** `√(pa² + pb²) + |pl|` per axis is attained where the
/// sinusoid's derivative peaks with the linear term's sign, which it does
/// once a period; so the row pins it from below at 0.999 on both axes, on
/// channels with and without a linear term.
#[test]
fn a_harmonic_images_bound_is_its_sup_and_an_iso_lines_is_its_velocity() {
    for (pa, pb, pl) in [
        (
            Vec2::new(0.7, -0.2),
            Vec2::new(0.3, 1.1),
            Vec2::new(0.0, 0.0),
        ),
        (
            Vec2::new(0.7, -0.2),
            Vec2::new(0.3, 1.1),
            Vec2::new(0.5, -0.9),
        ),
        (
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, -2.0),
        ),
    ] {
        let image = Pcurve::free_harmonic(Point2::new(0.1, 0.2), pa, pb, pl);
        let what = format!("harmonic pa {pa:?} pb {pb:?} pl {pl:?}");
        let bound = image.speed_bound(0.0, TAU).unwrap();
        let (ru, rv) = dominates(
            &what,
            sampled(&image, (0.0, TAU), 16_000, (false, false)),
            bound,
        );
        assert!(
            ru > 0.999 && rv > 0.999,
            "{what}: attained, not ({ru}, {rv})"
        );
    }
    let line = Pcurve::IsoLine {
        p0: Point2::new(0.0, 1.0),
        pl: Vec2::new(-0.25, 3.0),
    };
    assert_eq!(
        line.speed_bound(0.0, 2.0),
        Ok((0.25, 3.0)),
        "an iso line moves at |pl|"
    );
}

/// **A span is read forward, and a net is read inside its domain.** A
/// reversed or non-finite span refuses on every form (a projected line
/// reads its lever at the span's clamped vertex, and a reversed projected
/// circle reads its height's range backwards); a projected net read past
/// its knot domain refuses as a general image does.
#[test]
fn a_span_that_is_not_one_and_a_net_past_its_domain_refuse() {
    let circle = projected(
        ProjectedChart::Sphere,
        FramedCarrier::Conic {
            centre: Vec3::new(0.0, 0.0, 0.6),
            a: Vec3::new(0.8, 0.0, 0.0),
            b: Vec3::new(0.0, 0.8, 0.0),
            origin: 0.0,
            span: TAU,
        },
        0.0,
        1.0,
    );
    let line = projected(
        ProjectedChart::Cylinder,
        FramedCarrier::Line {
            point: Vec3::new(1.0, -2.0, 0.3),
            dir: Vec3::new(0.0, 1.0, 0.2),
            origin: 0.0,
            span: 4.0,
        },
        0.0,
        1.0,
    );
    for (t0, t1) in [(2.2, 1.0), (f64::NAN, 1.0), (0.0, f64::INFINITY)] {
        assert_eq!(circle.speed_bound(t0, t1), Err(NoSpeedBound::NotASpan));
        assert_eq!(line.speed_bound(t0, t1), Err(NoSpeedBound::NotASpan));
    }
    let net = degree_one_net();
    let (d0, d1) = net.domain();
    let image = projected(ProjectedChart::Cylinder, FramedCarrier::Net(net), d0, d1);
    assert_eq!(
        image.speed_bound(d0 - 2.0, d1 + 2.0),
        Err(NoSpeedBound::OutsideDomain)
    );
    assert!(image.speed_bound(d0, d1).is_ok());
}

/// The two-oval spiric poses the wall and cap rows sweep: `(R, r, d)`
/// from a thin tube to one whose hole is `10⁻⁵ R`, the cut from the
/// equator to near the tube's inner edge, both signs.
fn spiric_poses() -> Vec<(f64, f64, f64)> {
    let mut poses = Vec::new();
    for (major, minor) in [
        (3.0, 1.0),
        (2.0, 1.5),
        (5.0, 0.4),
        (1.0, 0.1),
        (1.0, 0.9),
        (1.0, 0.999),
        (1.0, 0.99999),
    ] {
        for frac in [0.0, 0.1, 0.5, 0.9, 0.99] {
            for sign in [1.0, -1.0] {
                poses.push((major, minor, sign * frac * (major - minor)));
            }
        }
    }
    poses
}

/// **A spiric wall image's bound dominates it, at every pose and on
/// both tori.** The tube angle's bound `|σ|` is its exact speed. The
/// azimuth's bound is a sup over graded pieces (`speed::spiric_breaks`),
/// so not attained, and how close it comes is the pieces' schedule's, not
/// a certified ratio: measured on this row's poses (the near-tangent cuts
/// and the tube whose hole is `10⁻⁵ R` included), the sampled speed
/// reaches at least 0.77 of it, and the row pins 0.75 so a padded,
/// unpaired or ungraded rewrite reds (the one-piece
/// `r·|d|/((R − r)·f_min)` reads 0.07 on a tube of hole `R/4`, and 64
/// equal pieces 0.34 on the finest hole). Off these poses it can sit
/// lower (reviewers measured 0.44 on one). The two tori and two senses
/// differ only in sign, so the row reads one of each.
#[test]
fn a_spiric_wall_images_bound_dominates_it() {
    let mut fewest = f64::INFINITY;
    for (major, minor, offset) in spiric_poses() {
        for (torus, sense) in [(WallTorus::Own, 1.0), (WallTorus::Mirror, -1.0)] {
            {
                let image = Pcurve::Spiric {
                    major,
                    minor,
                    offset,
                    image: SpiricImage::Wall {
                        u0: 0.3,
                        v0: -0.2,
                        sense,
                        torus,
                    },
                };
                let what = format!("wall R {major} r {minor} d {offset} {torus:?} σ {sense}");
                let bound = image.speed_bound(0.0, TAU).expect("two-oval regime");
                // The azimuth turns fastest within `√(2(R − r)/r)` of
                // `t = π`, which on a fat tube a schedule over the period
                // steps over: the window about it is read finely too.
                let (wide, near) = (
                    sampled(&image, (0.0, TAU), 6_000, (false, false)),
                    sampled(&image, (PI - 0.03, PI + 0.03), 6_000, (false, false)),
                );
                let (ru, rv) = dominates(&what, (wide.0.max(near.0), wide.1.max(near.1)), bound);
                assert!(
                    rv > 1.0 - 1e-9,
                    "{what}: the tube angle moves at exactly |σ|"
                );
                if offset != 0.0 {
                    fewest = fewest.min(ru);
                } else {
                    assert!(
                        bound.0 < 1e-8,
                        "{what}: an equatorial cut's azimuth is fixed, read {}",
                        bound.0
                    );
                }
            }
        }
    }
    assert!(
        fewest > 0.75,
        "the azimuth bound has grown looser than measured: the sampled speed reaches only \
         {fewest} of it on some pose"
    );
}

/// **A spiric cap image's bound dominates it.** Per axis
/// `|pm|·sup|f′| + |pa|`; the two terms peak apart, so it is not
/// attained, and how close it comes is the pieces' schedule's: measured
/// on this row's poses (the tube whose hole is `10⁻⁵ R` included), the
/// sampled speed reaches at least 0.71 of it on every axis, and the row
/// pins 0.65 (the carrier's speed sup in place of `sup|f′|` reads 0.15
/// near a tangent cut).
#[test]
fn a_spiric_cap_images_bound_dominates_it() {
    let mut fewest = f64::INFINITY;
    for (major, minor, offset) in spiric_poses() {
        for (pm, pa) in [
            (Vec2::new(1.0, 0.0), Vec2::new(0.0, minor)),
            (Vec2::new(0.6, 0.8), Vec2::new(-0.8 * minor, 0.6 * minor)),
        ] {
            let image = Pcurve::Spiric {
                major,
                minor,
                offset,
                image: SpiricImage::Cap {
                    p0: Point2::new(0.5, -1.0),
                    pm,
                    pa,
                },
            };
            let what = format!("cap R {major} r {minor} d {offset} pm {pm:?}");
            let bound = image.speed_bound(0.0, TAU).expect("two-oval regime");
            let (ru, rv) = dominates(
                &what,
                sampled(&image, (0.0, TAU), 6_000, (false, false)),
                bound,
            );
            fewest = fewest.min(ru).min(rv);
        }
    }
    assert!(
        fewest > 0.65,
        "the cap bound has grown looser than measured: the sampled speed reaches only \
         {fewest} of it on some pose"
    );
    // Off a ring torus's two-oval regime: a cut past the tube's inner
    // edge, and a spindle (`r > R`), whose `f` is real on part of the
    // period only.
    for (major, minor, offset) in [(3.0, 1.0, 2.0), (1.0, 2.0, 0.5)] {
        let off = Pcurve::Spiric {
            major,
            minor,
            offset,
            image: SpiricImage::Cap {
                p0: Point2::new(0.0, 0.0),
                pm: Vec2::new(1.0, 0.0),
                pa: Vec2::new(0.0, 1.0),
            },
        };
        assert_eq!(
            off.speed_bound(0.0, TAU),
            Err(NoSpeedBound::OffRegime),
            "R {major} r {minor} d {offset}"
        );
    }
}

/// **A focal section image's bound dominates it and is attained.** The
/// azimuth's `(1 + |β|)/(1 − |β|)` is reached at the vertex nearer the
/// axis, and the second channel's harmonic sup on both minted shapes
/// (a cone section: `vl = 0`; a Villarceau circle: `va = vb = 0`), so
/// the row pins both from below at 0.999.
#[test]
fn a_focal_section_images_bound_dominates_it_and_is_attained() {
    for beta in [0.0, 0.2, -0.5, 0.8, -0.95] {
        for sense in [1.0, -1.0] {
            for (va, vb, vl) in [(0.7, -0.4, 0.0), (0.0, 0.0, 1.0), (0.0, 0.0, -1.0)] {
                let image = Pcurve::FocalSection(FocalImage {
                    u0: 1.0,
                    t0: 0.4,
                    v0: 2.0,
                    va,
                    vb,
                    vl,
                    beta,
                    sense,
                });
                let what = format!("β {beta} σ {sense} v ({va}, {vb}, {vl})");
                let bound = image.speed_bound(0.0, TAU).expect("|β| < 1");
                let (ru, rv) = dominates(
                    &what,
                    sampled(&image, (0.0, TAU), 16_000, (false, false)),
                    bound,
                );
                assert!(
                    ru > 0.999 && rv > 0.999,
                    "{what}: the bound is attained, so ({ru}, {rv}) of it is slack"
                );
            }
        }
    }
    let off = Pcurve::FocalSection(FocalImage {
        u0: 0.0,
        t0: 0.0,
        v0: 0.0,
        va: 0.0,
        vb: 0.0,
        vl: 1.0,
        beta: 1.0,
        sense: 1.0,
    });
    assert_eq!(off.speed_bound(0.0, TAU), Err(NoSpeedBound::OffRegime));
}

/// A one-piece projected image of `carrier` on `chart`, its branch
/// centres at zero.
fn projected(
    chart: ProjectedChart<f64>,
    carrier: FramedCarrier<f64>,
    lo: f64,
    hi: f64,
) -> Pcurve<f64> {
    Pcurve::Projected(Box::new(ProjectedImage {
        chart,
        carrier,
        breaks: KnotVector::clamped(vec![lo, lo, hi, hi], 1).unwrap(),
        azimuth: vec![Quarter(0)],
        tube: vec![Quarter(0)],
        u_off: 0.25,
        v_off: -0.5,
        v_sign: -1.0,
    }))
}

/// The charts the projected rows sweep, each with which of its channels
/// are angles.
fn projected_charts() -> Vec<(&'static str, ProjectedChart<f64>, (bool, bool))> {
    let (sin, cos) = 0.4f64.sin_cos();
    vec![
        ("plane", ProjectedChart::Plane, (false, false)),
        ("cylinder", ProjectedChart::Cylinder, (true, false)),
        (
            "cone",
            ProjectedChart::Cone {
                sin,
                cos,
                nappe: Nappe::Opening,
            },
            (true, false),
        ),
        (
            "mirror cone",
            ProjectedChart::Cone {
                sin,
                cos,
                nappe: Nappe::Mirror,
            },
            (true, false),
        ),
        ("sphere", ProjectedChart::Sphere, (true, false)),
        ("torus", ProjectedChart::Torus { major: 2.0 }, (true, true)),
    ]
}

/// The conics the projected rows sweep, in the chart frame: tilted
/// circles and ellipses about the axis and off it, a flat circle round
/// the axis, one standing beside it.
fn projected_conics() -> Vec<(Vec3<f64>, Vec3<f64>, Vec3<f64>)> {
    let (s, c) = 0.7f64.sin_cos();
    vec![
        (
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0 * c, 2.0 * s),
        ),
        (
            Vec3::new(0.3, -0.2, 0.4),
            Vec3::new(1.5, 0.4, 0.2),
            Vec3::new(-0.3, 1.2, 0.9),
        ),
        (
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            Vec3::new(3.0, 0.5, 0.0),
            Vec3::new(0.0, 0.0, 0.8),
            Vec3::new(0.5, 0.5, 0.0),
        ),
        (
            Vec3::new(1.0, 1.0, 0.3),
            Vec3::new(0.2, 0.0, 0.1),
            Vec3::new(0.0, 0.4, -0.1),
        ),
    ]
}

/// **A projected image's bound dominates it on every chart**, for
/// conics over whole and partial turns, lines, and nets. Each channel's
/// bound is a Lipschitz one, the whole `|C′|` over the channel's least
/// lever arm, attained where the carrier moves along the channel's
/// gradient at that lever, and only there. Three legs are built to: a
/// circle round the axis, a line at its nearest approach to the axis and
/// a degree-1 net through its control box's nearest point, each moving
/// across the axis at its least lever; on every chart with an azimuth the
/// row pins their azimuth bound from below at 0.99, so a loosened bound
/// or a lever read above the curve's reds. Elsewhere the bound is not
/// attained and the row pins only domination: a channel the carrier moves
/// across (a flat circle's height on a cylinder) reads 0 against a
/// positive bound. On a circle of a sphere both channels are reached
/// (`a_projected_circle_of_a_sphere_reaches_its_bound`).
#[test]
fn a_projected_images_bound_dominates_it_on_every_chart() {
    let mut answered = 0;
    let mut refused = 0;
    for (name, chart, angles) in projected_charts() {
        for (k, &(c, a, b)) in projected_conics().iter().enumerate() {
            for (t0, t1) in [(0.0, TAU), (0.3, 1.9), (-2.0, -0.5)] {
                let carrier = FramedCarrier::Conic {
                    centre: c,
                    a,
                    b,
                    origin: t0,
                    span: t1 - t0,
                };
                let image = projected(chart, carrier, 0.0, 1.0);
                let what = format!("{name} conic {k} over [{t0}, {t1}]");
                match image.speed_bound(t0, t1) {
                    Ok(bound) => {
                        let (ru, _) =
                            dominates(&what, sampled(&image, (t0, t1), 6_000, angles), bound);
                        if k == ROUND && angles.0 {
                            assert!(ru > 0.99, "{what}: attained, not {ru}");
                        }
                        answered += 1;
                    }
                    Err(NoSpeedBound::ThroughSingularSet) => refused += 1,
                    Err(e) => panic!("{what}: a conic refused {e:?}"),
                }
            }
        }
        for (k, (point, dir)) in [
            (Vec3::new(1.0, -2.0, 0.3), Vec3::new(0.0, 1.0, 0.2)),
            (Vec3::new(0.5, 0.5, -1.0), Vec3::new(0.1, -0.2, 1.0)),
            (Vec3::new(2.5, 0.0, 0.0), Vec3::new(0.0, 0.7, 0.0)),
        ]
        .into_iter()
        .enumerate()
        {
            let carrier = FramedCarrier::Line {
                point,
                dir,
                origin: 0.0,
                span: 4.0,
            };
            let image = projected(chart, carrier, 0.0, 1.0);
            let what = format!("{name} line {k}");
            match image.speed_bound(0.0, 4.0) {
                Ok(bound) => {
                    let (ru, _) =
                        dominates(&what, sampled(&image, (0.0, 4.0), 6_000, angles), bound);
                    if k == 2 && angles.0 {
                        assert!(ru > 0.99, "{what}: attained at t = 0, not {ru}");
                    }
                    answered += 1;
                }
                Err(NoSpeedBound::ThroughSingularSet) => refused += 1,
                Err(e) => panic!("{what}: a line refused {e:?}"),
            }
        }
        let pts: Vec<Point3<f64>> = (0..7)
            .map(|i| {
                let t = f64::from(i) / 6.0;
                Point3::new(1.5 + 0.4 * (3.0 * t).cos(), 1.0 + t, 0.3 + 0.5 * t * t)
            })
            .collect();
        let net = Arc::new(geom::NurbsCurve3::interpolate(&pts, 3).unwrap());
        let (d0, d1) = net.domain();
        let image = projected(chart, FramedCarrier::Net(net), d0, d1);
        let bound = image
            .speed_bound(d0, d1)
            .expect("a net clear of every axis");
        dominates(
            &format!("{name} net"),
            sampled(&image, (d0, d1), 6_000, angles),
            bound,
        );
        let net = degree_one_net();
        let (d0, d1) = net.domain();
        let image = projected(chart, FramedCarrier::Net(net), d0, d1);
        let bound = image.speed_bound(d0, d1).expect("clear of every axis");
        let what = format!("{name} degree-1 net");
        let (ru, _) = dominates(&what, sampled(&image, (d0, d1), 6_000, angles), bound);
        if angles.0 {
            assert!(
                ru > 0.99,
                "{what}: attained at its box's nearest point, not {ru}"
            );
        }
        answered += 2;
    }
    assert!(
        answered >= 100 && refused >= 1,
        "the sweep answers {answered} poses and refuses {refused} through the singular set"
    );
}

/// The index in [`projected_conics`] of the circle round the axis.
const ROUND: usize = 2;

/// A degree-1 net, the segment `x = 2`, `z = 0.3` from `y = −1` to `1`:
/// its control box's nearest point to the axis is the curve's, at `y = 0`,
/// where it moves across the axis, so its azimuth bound is attained.
fn degree_one_net() -> Arc<geom::NurbsCurve3<f64>> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let pts = vec![Point3::new(2.0, -1.0, 0.3), Point3::new(2.0, 1.0, 0.3)];
    Arc::new(geom::NurbsCurve3::new(kv, pts, vec![1.0; 2]).unwrap())
}

/// **Near a sphere's pole the projected bound dominates in floats.** A
/// circle of the unit sphere whose highest point stands `10⁻⁷` from the
/// axis: there `ρ² = |p|² − z²` is a difference of two squares near 1,
/// which an `f64` difference reads a few percent low (and the azimuth
/// bound with it). Read over a span about the highest point, where the
/// azimuth turns fastest.
#[test]
fn a_circle_passing_a_pole_closely_is_bounded_in_floats() {
    for gap in [1e-7, 1e-5, 1e-3] {
        let (sr, cr) = 0.6f64.sin_cos();
        let (sl, cl) = (core::f64::consts::FRAC_PI_2 - 0.6 - gap).sin_cos();
        let d = Vec3::new(cl, 0.0, sl);
        let e1 = Vec3::new(0.0, 1.0, 0.0);
        let carrier = FramedCarrier::Conic {
            centre: d * cr,
            a: e1 * sr,
            b: d.cross(e1) * sr,
            origin: 0.0,
            span: TAU,
        };
        let image = projected(ProjectedChart::Sphere, carrier, 0.0, 1.0);
        let reach = 20.0 * gap;
        let span = (FRAC_PI_2 - reach, FRAC_PI_2 + reach);
        let what = format!("a circle {gap} from the pole");
        let bound = image
            .speed_bound(span.0, span.1)
            .expect("clear of the pole");
        let (ru, _) = dominates(&what, sampled(&image, span, 16_000, (true, false)), bound);
        assert!(ru > 0.9, "{what}: the azimuth bound is reached, not {ru}");
    }
}

/// **On a circle of a sphere the projected bound is the sup**: the
/// azimuth's at the arc's highest latitude, the latitude's anywhere
/// (`|C′|/|p|` is the circle's radius over the sphere's).
#[test]
fn a_projected_circle_of_a_sphere_reaches_its_bound() {
    for tilt in [0.3, 0.8, 1.2] {
        for lat in [-0.5, 0.0, 0.4] {
            // The circle of the unit sphere at angular radius 0.6 about
            // the direction at latitude `lat`, tilted `tilt` off the axis
            // plane; its centre `cos 0.6` along that direction.
            let (sl, cl) = (lat + tilt * 0.1).sin_cos();
            let d = Vec3::new(cl, 0.0, sl);
            let e1 = Vec3::new(0.0, 1.0, 0.0);
            let e2 = d.cross(e1);
            let (sr, cr) = 0.6f64.sin_cos();
            let carrier = FramedCarrier::Conic {
                centre: d * cr,
                a: e1 * sr,
                b: e2 * sr,
                origin: 0.0,
                span: TAU,
            };
            let image = projected(ProjectedChart::Sphere, carrier, 0.0, 1.0);
            let what = format!("sphere circle tilt {tilt} lat {lat}");
            let bound = image.speed_bound(0.0, TAU).expect("clear of the poles");
            let (ru, rv) = dominates(
                &what,
                sampled(&image, (0.0, TAU), 16_000, (true, false)),
                bound,
            );
            assert!(
                ru > 0.99 && rv > 0.99,
                "{what}: on a circle of the sphere the bound is reached, not ({ru}, {rv})"
            );
        }
    }
}

/// **`harmonic_range` reads an extremum inside the interval.** A
/// tilted circle's height turns inside its edge's interval, so the
/// ends alone under-read its range. Each case's range must hold a
/// dense sampling of the function and reach the extremum the
/// interval holds: `cos t` over `[−1, 1]` peaks at 1 with both ends at
/// `cos 1`, and the shifted forms put the turn off zero.
#[test]
fn harmonic_range_holds_a_turn_inside_the_interval() {
    let (lo, hi) = harmonic_range(0.0, 1.0, 0.0, (-1.0, 1.0));
    assert!((hi - 1.0).abs() < 1e-15, "the peak at t = 0: {hi}");
    assert!((lo - 1f64.cos()).abs() < 1e-15, "the ends: {lo}");
    for (c, a, b, t) in [
        (0.3, 0.4, -0.7, (0.5, 3.0)),
        (-0.2, -0.6, 0.1, (2.0, 4.5)),
        (0.0, 0.5, 0.5, (-2.5, 2.5)),
        (1.0, -0.3, -0.9, (4.0, 7.5)),
    ] {
        let (lo, hi) = harmonic_range(c, a, b, t);
        let f = |x: f64| c + a * x.cos() + b * x.sin();
        let (mut smin, mut smax) = (f64::INFINITY, f64::NEG_INFINITY);
        for i in 0..=4096 {
            let x = t.0 + (t.1 - t.0) * f64::from(i) / 4096.0;
            smin = smin.min(f(x));
            smax = smax.max(f(x));
        }
        assert!(
            lo <= smin + 1e-12 && hi >= smax - 1e-12,
            "{c} {a} {b} {t:?}"
        );
        assert!(
            smin - lo < 1e-6 && hi - smax < 1e-6,
            "{c} {a} {b} {t:?}: the range is the function's, not wider: \
             [{lo}, {hi}] against [{smin}, {smax}]"
        );
        let ends = (f(t.0).min(f(t.1)), f(t.0).max(f(t.1)));
        assert!(
            lo < ends.0 - 1e-3 || hi > ends.1 + 1e-3,
            "{c} {a} {b} {t:?}: the case turns inside its interval"
        );
    }
}

/// **A projected circle through the sphere's pole has no azimuth bound**
/// (the sphere arm refuses it `SphereTrimShape { ThroughPole }`): the unit
/// great circle in the `xz` plane passes `z = ±1`, where `ρ = 0`. Tilted
/// off the pole it answers finite bounds.
#[test]
fn a_circle_through_the_pole_has_no_azimuth_bound() {
    let circle = |b: Vec3<f64>| FramedCarrier::Conic {
        centre: Vec3::new(0.0, 0.0, 0.0),
        a: Vec3::new(1.0, 0.0, 0.0),
        b,
        origin: 0.0,
        span: TAU,
    };
    let through = projected(
        ProjectedChart::Sphere,
        circle(Vec3::new(0.0, 0.0, 1.0)),
        0.0,
        1.0,
    );
    assert_eq!(
        through.speed_bound(0.0, TAU),
        Err(NoSpeedBound::ThroughSingularSet)
    );
    let tilted = projected(
        ProjectedChart::Sphere,
        circle(Vec3::new(0.0, 0.6, 0.8)),
        0.0,
        1.0,
    );
    let (su, sv) = tilted
        .speed_bound(0.0, TAU)
        .expect("a circle clear of the pole is bounded");
    assert!(su.is_finite() && sv.is_finite() && su >= 1.0 && sv > 0.0);
}

/// A spiric wall image of `(R, r, d)`.
fn wall(major: f64, minor: f64, offset: f64) -> Pcurve<f64> {
    Pcurve::Spiric {
        major,
        minor,
        offset,
        image: SpiricImage::Wall {
            u0: 0.3,
            v0: -0.2,
            sense: 1.0,
            torus: WallTorus::Own,
        },
    }
}

/// **A spiric's work is bounded whatever its span.** The image is
/// periodic, so a span of a period or more reads one period's pieces:
/// a span of `10⁶` and one of ten radians at `10¹⁵` answer the whole
/// period's bound exactly, and a radian at `10¹⁵`, past where a parameter
/// step of one piece still advances in `f64` from its start's multiple,
/// answers at all. A span of no width reads the piece of its one point,
/// so it bounds the image's speed there.
#[test]
fn a_spirics_bound_reads_at_most_a_period_and_a_point_reads_its_piece() {
    let image = wall(3.0, 1.0, 0.8);
    let period = image.speed_bound(0.0, TAU).unwrap();
    assert_eq!(image.speed_bound(0.0, 1e6), Ok(period), "a long span");
    assert_eq!(
        image.speed_bound(1e15, 1e15 + 10.0),
        Ok(period),
        "a period far out"
    );
    let far = image
        .speed_bound(1e15, 1e15 + 1.0)
        .expect("a radian far out answers");
    assert!(far.0.is_finite() && far.0 > 0.0, "{far:?}");
    for t in [1.0, 2.5, 4.0] {
        let point = image.speed_bound(t, t).unwrap();
        let (du, _) = sampled(&image, (t - 1e-6, t + 1e-6), 2, (false, false));
        assert!(
            du > 0.0 && point.0 >= du * (1.0 - 1e-6),
            "at t = {t}: a point's bound {point:?} against the image's speed {du} there"
        );
    }
}

/// **The cap's `sup|f′|` reads `inf cos²t` on each piece.** At `d = 0`,
/// `f = ρ` and `f′ = −r·sin t` exactly, so a cap whose first axis is `f`
/// alone moves at `r·|sin t|`, and its bound is reached over a period
/// and over spans that stop short of `t = π/2`: pinned at 0.999, which a
/// bound reading `sup cos²t` falls under on the short spans.
#[test]
fn an_equatorial_caps_radial_rate_is_reached() {
    for (major, minor) in [(3.0, 1.0), (1.0, 0.999)] {
        let image = Pcurve::Spiric {
            major,
            minor,
            offset: 0.0,
            image: SpiricImage::Cap {
                p0: Point2::new(0.0, 0.0),
                pm: Vec2::new(1.0, 0.0),
                pa: Vec2::new(0.0, minor),
            },
        };
        // Over a period the piece holding `t = π/2` holds `cos t = 0`; a
        // span ending short of it reads `sup|f′|` at a piece whose
        // `cos t` is not zero, where `inf cos²t` is what the bound needs.
        for span in [(0.0, TAU), (0.3, 1.0), (2.2, 2.9)] {
            let what = format!("equatorial cap R {major} r {minor} over {span:?}");
            let bound = image.speed_bound(span.0, span.1).unwrap();
            let (ru, _) = dominates(&what, sampled(&image, span, 16_000, (false, false)), bound);
            assert!(ru > 0.999, "{what}: reached, not {ru}");
        }
    }
}

/// **A projected image's second channel is reached off the sphere.** A
/// line along the cylinder's axis, along a cone's generator on either
/// nappe, and radially through the point above a torus's core circle each
/// move along their second channel's gradient at its least lever, so the
/// `v` bound is the sup: pinned at 0.99.
#[test]
fn a_projected_images_second_channel_is_reached_off_the_sphere() {
    let (sin, cos) = 0.4f64.sin_cos();
    for (name, chart, point, dir, angles) in [
        (
            "cylinder",
            ProjectedChart::Cylinder,
            Vec3::new(1.0, 0.2, 0.0),
            Vec3::new(0.0, 0.0, 1.3),
            (true, false),
        ),
        (
            "cone",
            ProjectedChart::Cone {
                sin,
                cos,
                nappe: Nappe::Opening,
            },
            Vec3::new(sin, 0.0, cos),
            Vec3::new(sin, 0.0, cos),
            (true, false),
        ),
        (
            "mirror cone",
            ProjectedChart::Cone {
                sin,
                cos,
                nappe: Nappe::Mirror,
            },
            Vec3::new(sin, 0.0, -cos),
            Vec3::new(sin, 0.0, -cos),
            (true, false),
        ),
        (
            "torus",
            ProjectedChart::Torus { major: 2.0 },
            Vec3::new(1.5, 0.0, 0.5),
            Vec3::new(1.0, 0.0, 0.0),
            (true, true),
        ),
    ] {
        let carrier = FramedCarrier::Line {
            point,
            dir,
            origin: 0.0,
            span: 1.0,
        };
        let image = projected(chart, carrier, 0.0, 1.0);
        let bound = image
            .speed_bound(0.0, 1.0)
            .expect("clear of the singular set");
        let (_, rv) = dominates(name, sampled(&image, (0.0, 1.0), 6_000, angles), bound);
        assert!(rv > 0.99, "{name}: the second channel is reached, not {rv}");
    }
}

/// **A non-finite coefficient refuses on every form**, rather than
/// answering a NaN bound or a lever read as `0`.
#[test]
fn a_non_finite_coefficient_refuses_on_every_form() {
    let nan = f64::NAN;
    let breaks = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let images = [
        Pcurve::free_harmonic(
            Point2::new(0.0, 0.0),
            Vec2::new(nan, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0),
        ),
        Pcurve::IsoLine {
            p0: Point2::new(0.0, 0.0),
            pl: Vec2::new(nan, 1.0),
        },
        Pcurve::IsoArc {
            p0: Point2::new(0.0, 0.0),
            pd: Vec2::new(nan, 0.0),
            t0: 0.0,
            angle: FRAC_PI_2,
            breaks,
            ends: Vec::new().into(),
        },
        wall(3.0, nan, 0.5),
        Pcurve::FocalSection(FocalImage {
            u0: 0.0,
            t0: 0.0,
            v0: 0.0,
            va: nan,
            vb: 0.0,
            vl: 1.0,
            beta: 0.2,
            sense: 1.0,
        }),
        projected(
            ProjectedChart::Torus { major: nan },
            FramedCarrier::Line {
                point: Vec3::new(1.5, 0.0, 0.5),
                dir: Vec3::new(1.0, 0.0, 0.0),
                origin: 0.0,
                span: 1.0,
            },
            0.0,
            1.0,
        ),
        projected(
            ProjectedChart::Sphere,
            FramedCarrier::Conic {
                centre: Vec3::new(0.0, 0.0, nan),
                a: Vec3::new(1.0, 0.0, 0.0),
                b: Vec3::new(0.0, 1.0, 0.0),
                origin: 0.0,
                span: TAU,
            },
            0.0,
            1.0,
        ),
    ];
    for image in images {
        assert_eq!(
            image.speed_bound(0.0, 1.0),
            Err(NoSpeedBound::NonFinite),
            "{image:?}"
        );
    }
}
