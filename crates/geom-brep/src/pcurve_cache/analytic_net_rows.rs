//! **A spline net stated on an analytic chart is read bounded over its
//! span** (`certify.rs` `stated_analytic_bound`, through
//! `pcurve_cache::read_net_bound`): a net off its plane, cylinder, cone,
//! sphere or torus anywhere on the span, between every sample of the
//! edge's schedule and of the sampled read alike, does not get an edge
//! certificate. A definite miss ends read off the chart, one in the band
//! ends in the read's lever, and a net on its chart still certifies.
//!
//! The ruled charts' carrier is a ruling (`Ruled`) as a polyline or a
//! higher-degree net whose controls sit on the ruling at their Greville
//! abscissae, so at height zero it is the ruling itself, parameterised
//! as its stated image is. One control is pushed off the chart along the
//! ruling's normal; its distance from the chart is then that height times
//! the control's basis function, exactly, on all three charts.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT=net-bound-sampled` (the net read
//! at its samples alone again), the witness rows and the narrowed bump's
//! off and in-band rows go red.

test_utils::gated_to![
    "crates/geom-brep/src/certify.rs",
    "crates/geom-brep/src/pcurve_cache.rs",
    "crates/geom-brep/src/pcurve_cache/projected.rs",
    "crates/geom-brep/src/edge_nurbs.rs",
];

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Decide, Interval, Point2, Point3, Real, Vec2, Vec3};
use test_utils::fuzz;

use super::Pcurve;
use super::spline_door_rows::{band, in_band};
use crate::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, IncidenceRead, NurbsLane,
    PcurveCertifyError, SurfaceKey,
};

/// The cone's half-angle.
const ALPHA: f64 = 0.3;

/// A chart whose rulings the rows lay nets along.
#[derive(Clone, Copy, Debug)]
enum Ruled {
    Plane,
    Cylinder,
    Cone,
}

impl Ruled {
    const ALL: [Self; 3] = [Self::Plane, Self::Cylinder, Self::Cone];

    fn surface<T: Real>(self) -> Surface<T> {
        let s = match self {
            Self::Plane => Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Self::Cylinder => Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            Self::Cone => Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_z(),
                half_angle: ALPHA,
                u_ref: Vec3::unit_x(),
            },
        };
        s.map_scalar(T::from_f64)
    }

    /// The ruling `foot + dir·t`, `t ∈ [0, 1]`, its unit normal to the
    /// chart, and its chart image's ends.
    fn ruling(self) -> ([f64; 3], [f64; 3], [f64; 3], [f64; 2], [f64; 2]) {
        let (s, c) = ALPHA.sin_cos();
        match self {
            Self::Plane => (
                [0.0, 0.3, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.3],
                [1.0, 0.3],
            ),
            Self::Cylinder => (
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0],
                [0.0, 1.0],
            ),
            // The ruling from slant 1 to slant 2; the normal lies in its
            // meridian plane.
            Self::Cone => ([s, 0.0, c], [s, 0.0, c], [c, 0.0, -s], [0.0, 1.0], [0.0, 2.0]),
        }
    }
}

/// A net along a ruling: degree `degree`, interior knots `interior`,
/// its controls on the ruling at their Greville abscissae, the one at
/// `bumped` pushed `height` along the normal.
#[derive(Clone, Debug)]
struct Net {
    degree: usize,
    interior: Vec<f64>,
    bumped: usize,
    height: f64,
}

/// The witness' bump: a polyline with one vertex at `t = 1/128` and its
/// neighbours `1/256` either side, wholly between the schedule's samples
/// (`i/8`) and the sampled read's (`i/64`).
fn witness(height: f64) -> Net {
    let (a, b) = (1.0 / 256.0, 1.0 / 128.0);
    Net {
        degree: 1,
        interior: vec![b - a, b, b + a],
        bumped: 2,
        height,
    }
}

impl Net {
    fn knots(&self) -> Vec<f64> {
        let p = self.degree;
        let mut knots = vec![0.0; p + 1];
        knots.extend(&self.interior);
        knots.extend(vec![1.0; p + 1]);
        knots
    }

    /// The Greville abscissae: a linear function's controls, so at height
    /// zero the net is the ruling parameterised by `t`.
    fn greville(&self) -> Vec<f64> {
        let (p, knots) = (self.degree, self.knots());
        #[allow(clippy::cast_precision_loss)]
        (0..knots.len() - p - 1)
            .map(|i| knots[i + 1..=i + p].iter().sum::<f64>() / p as f64)
            .collect()
    }

    fn curve(&self, chart: Ruled) -> NurbsCurve3<f64> {
        let (foot, dir, n, _, _) = chart.ruling();
        let ts = self.greville();
        let control: Vec<Point3<f64>> = ts
            .iter()
            .enumerate()
            .map(|(i, &t)| {
                let h = if i == self.bumped { self.height } else { 0.0 };
                Point3::new(
                    foot[0] + dir[0] * t + n[0] * h,
                    foot[1] + dir[1] * t + n[1] * h,
                    foot[2] + dir[2] * t + n[2] * h,
                )
            })
            .collect();
        let count = control.len();
        NurbsCurve3::new(
            KnotVector::clamped(self.knots(), self.degree).unwrap(),
            control,
            vec![1.0; count],
        )
        .unwrap()
    }

    /// The bumped control's basis function's largest value, by a dense
    /// walk: the net's distance from every ruled chart is `|height|`
    /// times it.
    fn peak(&self) -> f64 {
        let unit = Self {
            height: 1.0,
            ..self.clone()
        }
        .curve(Ruled::Plane);
        let n = 1u32 << 14;
        (0..=n)
            .map(|i| f64::from(i) / f64::from(n))
            .chain(self.greville())
            .map(|t| unit.eval(t).z.abs())
            .fold(0.0, f64::max)
    }

    /// This net with its height set so its distance from the chart peaks
    /// at `miss`.
    fn missing(mut self, miss: f64) -> Self {
        self.height = 1.0;
        let peak = self.peak();
        self.height = miss / peak;
        self
    }
}

/// How the edge states its chart image.
#[derive(Clone, Copy, Debug)]
enum Stated {
    Iso,
    General,
}

fn spec_of<T: Real>(
    key: SurfaceKey,
    chart: Ruled,
    stated: Stated,
    carrier: Curve3<T>,
) -> EdgeCurveSpec<T> {
    let c = T::from_f64;
    let (_, _, _, p0, p1) = chart.ruling();
    let image = match stated {
        Stated::Iso => Pcurve::IsoLine {
            p0: Point2::new(c(p0[0]), c(p0[1])),
            pl: Vec2::new(c(p1[0] - p0[0]), c(p1[1] - p0[1])),
        },
        Stated::General => {
            let line = NurbsCurve2::new(
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![Point2::new(p0[0], p0[1]), Point2::new(p1[0], p1[1])],
                vec![1.0, 1.0],
            )
            .unwrap();
            Pcurve::General(Arc::new(line.map_scalar(T::from_f64)))
        }
    };
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart_image(key, image),
        carrier,
        param_start: c(0.0),
        param_end: c(1.0),
    }
}

/// The edge certificate of `net` along `chart`'s ruling, stated by its
/// image, at `T`.
fn certify<T: Decide + geom_core::CertifiedBounds>(
    chart: Ruled,
    net: &Net,
    stated: Stated,
) -> Result<(), CertifyError> {
    let c = T::from_f64;
    let mut table: slotmap::SlotMap<SurfaceKey, Surface<T>> = slotmap::SlotMap::with_key();
    let key = table.insert(chart.surface::<T>());
    let carrier = Curve3::Nurbs(Arc::new(net.curve(chart).map_scalar(T::from_f64)));
    let (start, end) = (carrier.eval(c(0.0)), carrier.eval(c(1.0)));
    EdgeCurve::certify(
        spec_of(key, chart, stated, carrier),
        start,
        end,
        |k| table.get(k).cloned(),
        geom_core::test_support::fixture_lever(T::one()),
        band(),
        NurbsLane::certified(),
    )
    .map(|_| ())
}

/// What a refusal read: off, in band, or not an incidence read at all.
fn read(got: &Result<(), CertifyError>) -> Option<IncidenceRead> {
    got.as_ref().err().and_then(CertifyError::incidence_read)
}

fn is_off(got: &Result<(), CertifyError>) -> bool {
    matches!(read(got), Some(IncidenceRead::Off(_)))
}

fn is_lever(got: &Result<(), CertifyError>) -> bool {
    matches!(read(got), Some(IncidenceRead::Undecided(_)))
}

/// **The witness**: a net a millimetre off its plane, cylinder or cone
/// between both sample sets, stated as an iso line or a degree-1 curve in
/// UV, ends read off the chart; the same net on its chart certifies.
fn the_witness<T: Decide + geom_core::CertifiedBounds>() {
    for chart in Ruled::ALL {
        for stated in [Stated::Iso, Stated::General] {
            let got = certify::<T>(chart, &witness(1e-3), stated);
            assert!(
                is_off(&got),
                "{chart:?} {stated:?} at {}: a net a millimetre off its chart between the \
                 samples ends off the chart: {got:?}",
                T::NAME
            );
            let on = certify::<T>(chart, &witness(0.0), stated);
            assert!(
                on.is_ok(),
                "{chart:?} {stated:?} at {}: the net on its chart certifies: {on:?}",
                T::NAME
            );
        }
    }
}

#[test]
fn a_net_a_millimetre_off_its_analytic_chart_between_the_samples_ends_off_f64() {
    the_witness::<f64>();
}

#[test]
fn a_net_a_millimetre_off_its_analytic_chart_between_the_samples_ends_off_interval() {
    the_witness::<Interval>();
}

/// **The bump narrowed toward the band**: at four times the band's
/// escalation it ends off; in the band it ends in the lever, neither off
/// nor certified; under half the band's zero it certifies.
fn narrowed<T: Decide + geom_core::CertifiedBounds>() {
    let b = band();
    for chart in Ruled::ALL {
        let off = certify::<T>(chart, &witness(4.0 * b.escalate()), Stated::Iso);
        assert!(
            is_off(&off),
            "{chart:?} at {}: a bump four times the band's escalation ends off: {off:?}",
            T::NAME
        );
        let lever = certify::<T>(chart, &witness(in_band()), Stated::Iso);
        assert!(
            is_lever(&lever),
            "{chart:?} at {}: a bump in the band ends in the lever: {lever:?}",
            T::NAME
        );
        let on = certify::<T>(chart, &witness(0.5 * b.zero()), Stated::Iso);
        assert!(
            on.is_ok(),
            "{chart:?} at {}: a bump under the band's zero certifies: {on:?}",
            T::NAME
        );
    }
}

#[test]
fn a_bump_narrowed_toward_the_band_ends_by_its_height_f64() {
    narrowed::<f64>();
}

#[test]
fn a_bump_narrowed_toward_the_band_ends_by_its_height_interval() {
    narrowed::<Interval>();
}

/// A height relative to the band: on the chart, in the band, or four
/// times its escalation.
#[derive(Clone, Copy, Debug)]
enum Height {
    On,
    InBand,
    Off,
}

impl Height {
    const ALL: [Self; 3] = [Self::On, Self::InBand, Self::Off];

    fn miss(self) -> f64 {
        match self {
            Self::On => 0.0,
            Self::InBand => in_band(),
            Self::Off => 4.0 * band().escalate(),
        }
    }

    /// Whether `got` is what a carrier this far off its chart ends as:
    /// on certifies, in the band ends in the lever (its supremum is past
    /// the zero and short of the escalation, so neither on nor off can be
    /// shown), off ends off.
    fn ends<E: std::fmt::Debug>(self, got: &Result<(), E>, read: Option<IncidenceRead>) -> bool {
        match self {
            Self::On => got.is_ok(),
            Self::InBand => matches!(read, Some(IncidenceRead::Undecided(_))),
            Self::Off => matches!(read, Some(IncidenceRead::Off(_))),
        }
    }
}

/// A knot-rich net along a ruling: degree one to three, up to forty
/// interior knots at random, one control pushed.
fn draw_ruled(g: &mut fuzz::Rng) -> Net {
    let degree = 1 + g.below(3);
    let mut interior: Vec<f64> = (0..1 + g.below(40))
        .map(|_| g.range(1e-6, 1.0 - 1e-6))
        .collect();
    interior.sort_by(f64::total_cmp);
    interior.dedup();
    let count = interior.len() + degree + 1;
    Net {
        degree,
        interior,
        bumped: g.below(count),
        height: if g.below(2) == 0 { 1.0 } else { -1.0 },
    }
}

/// A circle a uniform `miss` off a sphere or a torus, as a rational
/// quadratic net (four quarter arcs) refined at random knots, read over a
/// random window: on the sphere a great circle of radius `R + miss`, on
/// the torus a tube circle of radius `r + miss` about the core.
fn draw_round(g: &mut fuzz::Rng, torus: bool, miss: f64) -> (Surface<f64>, NurbsCurve3<f64>, (f64, f64)) {
    let (big, small) = (2.0, 0.5);
    let phi = g.range(0.0, std::f64::consts::TAU);
    let radial = Vec3::new(phi.cos(), phi.sin(), 0.0);
    let (surface, center, a, radius) = if torus {
        (
            Surface::Torus {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                major_radius: big,
                minor_radius: small,
                u_ref: Vec3::unit_x(),
            },
            Point3::origin() + radial * big,
            radial,
            small + miss,
        )
    } else {
        (
            Surface::Sphere {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: big,
                u_ref: Vec3::unit_x(),
            },
            Point3::origin(),
            radial,
            big + miss,
        )
    };
    let b = Vec3::unit_z();
    let w = std::f64::consts::FRAC_1_SQRT_2;
    let corners = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0), (1.0, -1.0), (1.0, 0.0)];
    let control = corners
        .iter()
        .map(|&(x, y)| center + a * (radius * x) + b * (radius * y))
        .collect();
    let weights = (0..9).map(|i| if i % 2 == 0 { 1.0 } else { w }).collect();
    let circle = NurbsCurve3::new(
        KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0],
            2,
        )
        .unwrap(),
        control,
        weights,
    )
    .unwrap();
    let mut add: Vec<f64> = (0..g.below(30)).map(|_| g.range(1e-6, 1.0 - 1e-6)).collect();
    add.sort_by(f64::total_cmp);
    let net = circle.refine_knots(&add).unwrap();
    let (x, y) = (g.unit(), g.unit());
    let window = (x.min(y), x.max(y).max(x.min(y) + 1e-3).min(1.0));
    (surface, net, window)
}

/// The bounded read of `net` on `surface` over `window` at `T`.
fn read_round<T: Decide + geom_core::CertifiedBounds>(
    surface: &Surface<f64>,
    net: &NurbsCurve3<f64>,
    (t0, t1): (f64, f64),
) -> Result<(), PcurveCertifyError> {
    let carrier = Curve3::Nurbs(Arc::new(net.map_scalar(T::from_f64)));
    super::read_net_bound(
        &carrier,
        (T::from_f64(t0), T::from_f64(t1)),
        &surface.map_scalar(T::from_f64),
        band(),
    )
}

fn incidence(got: &Result<(), PcurveCertifyError>) -> Option<IncidenceRead> {
    match got {
        Err(PcurveCertifyError::Incidence(refusal)) => Some(refusal.read()),
        _ => None,
    }
}

/// **No false Off, and every height ends by its supremum** (shape 1): a
/// knot-rich net on its plane, cylinder or cone, through the edge
/// certificate, or a refined circle on its sphere or torus, through the
/// bounded read, at a supremum of zero, in the band or four times the
/// escalation, ends as that supremum says ([`Height::ends`]) at `f64`
/// and at `Interval`.
#[test]
fn a_nets_read_on_an_analytic_chart_ends_by_its_supremum() {
    let mut g = fuzz::start("analytic_net_supremum");
    for _ in 0..fuzz::scaled(12) {
        let net = draw_ruled(&mut g);
        let chart = Ruled::ALL[g.below(3)];
        let stated = if g.below(2) == 0 {
            Stated::Iso
        } else {
            Stated::General
        };
        for height in Height::ALL {
            let net = net.clone().missing(height.miss());
            for (scalar, got) in [
                ("f64", certify::<f64>(chart, &net, stated)),
                ("interval", certify::<Interval>(chart, &net, stated)),
            ] {
                assert!(
                    height.ends(&got, read(&got)),
                    "{chart:?} {stated:?} {height:?} at {scalar}: {got:?} ({net:?}); {}",
                    fuzz::replay()
                );
            }
        }
        let torus = g.below(2) == 0;
        for height in Height::ALL {
            let (surface, round, window) = draw_round(&mut g, torus, height.miss());
            for (scalar, got) in [
                ("f64", read_round::<f64>(&surface, &round, window)),
                ("interval", read_round::<Interval>(&surface, &round, window)),
            ] {
                assert!(
                    height.ends(&got, incidence(&got)),
                    "{} {height:?} at {scalar} over {window:?}: {got:?}; {}",
                    if torus { "torus" } else { "sphere" },
                    fuzz::replay()
                );
            }
        }
    }
}
