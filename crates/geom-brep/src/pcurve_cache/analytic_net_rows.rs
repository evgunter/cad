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
use geom_core::{Interval, Point2, Point3, Real, Vec2, Vec3};
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
    fn ruling(self) -> Ruling {
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
            Self::Cone => (
                [s, 0.0, c],
                [s, 0.0, c],
                [c, 0.0, -s],
                [0.0, 1.0],
                [0.0, 2.0],
            ),
        }
    }
}

/// A ruling's foot, direction and unit normal, and its chart image's
/// ends.
type Ruling = ([f64; 3], [f64; 3], [f64; 3], [f64; 2], [f64; 2]);

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

/// A bounded read of a net on a chart over a window, at one scalar.
type Read = fn(&Surface<f64>, &NurbsCurve3<f64>, (f64, f64)) -> Result<(), PcurveCertifyError>;

/// One certifying scalar's doors, instantiated per scalar by name (so
/// the rows hold no compound scalar bound): the edge certificate of a
/// net along a chart's ruling stated by its image, and the bounded read
/// of a net on a chart over a window.
#[derive(Clone, Copy)]
struct Door {
    name: &'static str,
    certify: fn(Ruled, &Net, Stated) -> Result<(), CertifyError>,
    read: Read,
    /// The stated-image edge certificate of a net on a sphere or torus
    /// over a window, stated by the projected image the mint derives for
    /// its on-chart twin; `None` where the mint does not image the twin.
    certify_round: fn(&Round, f64) -> Option<Result<(), CertifyError>>,
    /// `net_bound_refusal` on a witness net on the plane.
    refusal: fn(crate::edge_nurbs::AnalyticRung3Refusal) -> PcurveCertifyError,
}

macro_rules! door_at {
    ($t:ty) => {
        Door {
            name: <$t as Real>::NAME,
            certify: |chart, net, stated| {
                let c = <$t as Real>::from_f64;
                let mut table: slotmap::SlotMap<SurfaceKey, Surface<$t>> =
                    slotmap::SlotMap::with_key();
                let key = table.insert(chart.surface::<$t>());
                let carrier = Curve3::Nurbs(Arc::new(net.curve(chart).map_scalar(c)));
                let (start, end) = (carrier.eval(c(0.0)), carrier.eval(c(1.0)));
                EdgeCurve::certify(
                    spec_of(key, chart, stated, carrier),
                    start,
                    end,
                    |k| table.get(k).cloned(),
                    geom_core::test_support::fixture_lever(<$t as Real>::one()),
                    band(),
                    NurbsLane::certified(),
                )
                .map(|_| ())
            },
            read: |surface, net, (t0, t1)| {
                let c = <$t as Real>::from_f64;
                let carrier = Curve3::Nurbs(Arc::new(net.map_scalar(c)));
                super::read_net_bound(
                    &carrier,
                    (c(t0), c(t1)),
                    &surface.map_scalar(c),
                    band(),
                    super::limb2_offset_sup,
                )
            },
            certify_round: |round, miss| {
                let c = <$t as Real>::from_f64;
                let (t0, t1) = round.window;
                let surface = round.surface().map_scalar(c);
                let on = Curve3::Nurbs(Arc::new(round.net(0.0).map_scalar(c)));
                let image = crate::chart_pcurve_over(&on, c(t0), c(t1), &surface, band()).ok()?;
                let carrier = Curve3::Nurbs(Arc::new(round.net(miss).map_scalar(c)));
                let mut table: slotmap::SlotMap<SurfaceKey, Surface<$t>> =
                    slotmap::SlotMap::with_key();
                let key = table.insert(surface);
                let (start, end) = (carrier.eval(c(t0)), carrier.eval(c(t1)));
                EdgeCurve::certify(
                    EdgeCurveSpec {
                        description: EdgeDescriptionSpec::chart_image(key, image),
                        carrier,
                        param_start: c(t0),
                        param_end: c(t1),
                    },
                    start,
                    end,
                    |k| table.get(k).cloned(),
                    geom_core::test_support::fixture_lever(<$t as Real>::one()),
                    band(),
                    NurbsLane::certified(),
                )
                .map(|_| ())
                .into()
            },
            refusal: |refusal| {
                let c = <$t as Real>::from_f64;
                let surface = Ruled::Plane.surface::<$t>();
                let carrier =
                    Curve3::Nurbs(Arc::new(witness(0.0).curve(Ruled::Plane).map_scalar(c)));
                super::net_bound_refusal(refusal, &surface, &carrier, band())
            },
        }
    };
}

const F64: Door = door_at!(f64);
const INTERVAL: Door = door_at!(Interval);

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
fn the_witness(door: Door) {
    for chart in Ruled::ALL {
        for stated in [Stated::Iso, Stated::General] {
            let got = (door.certify)(chart, &witness(1e-3), stated);
            assert!(
                is_off(&got),
                "{chart:?} {stated:?} at {}: a net a millimetre off its chart between the \
                 samples ends off the chart: {got:?}",
                door.name
            );
            let on = (door.certify)(chart, &witness(0.0), stated);
            assert!(
                on.is_ok(),
                "{chart:?} {stated:?} at {}: the net on its chart certifies: {on:?}",
                door.name
            );
        }
    }
}

#[test]
fn a_net_a_millimetre_off_its_analytic_chart_between_the_samples_ends_off_f64() {
    the_witness(F64);
}

#[test]
fn a_net_a_millimetre_off_its_analytic_chart_between_the_samples_ends_off_interval() {
    the_witness(INTERVAL);
}

/// **The bump narrowed toward the band**: at four times the band's
/// escalation it ends off; in the band it ends in the lever, neither off
/// nor certified; under half the band's zero it certifies.
fn narrowed(door: Door) {
    let b = band();
    for chart in Ruled::ALL {
        let off = (door.certify)(chart, &witness(4.0 * b.escalate()), Stated::Iso);
        assert!(
            is_off(&off),
            "{chart:?} at {}: a bump four times the band's escalation ends off: {off:?}",
            door.name
        );
        let lever = (door.certify)(chart, &witness(in_band()), Stated::Iso);
        assert!(
            is_lever(&lever),
            "{chart:?} at {}: a bump in the band ends in the lever: {lever:?}",
            door.name
        );
        let on = (door.certify)(chart, &witness(0.5 * b.zero()), Stated::Iso);
        assert!(
            on.is_ok(),
            "{chart:?} at {}: a bump under the band's zero certifies: {on:?}",
            door.name
        );
    }
}

#[test]
fn a_bump_narrowed_toward_the_band_ends_by_its_height_f64() {
    narrowed(F64);
}

#[test]
fn a_bump_narrowed_toward_the_band_ends_by_its_height_interval() {
    narrowed(INTERVAL);
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
    /// [`Self::ends`], save that on the chart a refusal of a check after
    /// the read is admitted where it reads neither off nor definite: at
    /// `Interval` and ε 1e-12 the schedule's residual on a sphere of
    /// radius 2 can escalate on its enclosure's width.
    fn admits(self, got: &Result<(), CertifyError>, read: Option<IncidenceRead>) -> bool {
        match self {
            Self::On => {
                got.is_ok()
                    || !matches!(read, Some(r) if r.is_definite() || matches!(r, IncidenceRead::Off(_)))
            }
            _ => self.ends(got, read),
        }
    }

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
/// random window: on the sphere a great circle tilted `tilt` from the
/// equator (clear of the poles, where the projected image has no
/// azimuth) of radius `R + miss`, on the torus a tube circle of radius `r + miss`
/// about the core. Its distance from the chart is `|miss|` everywhere.
#[derive(Clone, Debug)]
struct Round {
    torus: bool,
    phi: f64,
    tilt: f64,
    add: Vec<f64>,
    window: (f64, f64),
}

const BIG: f64 = 2.0;
const SMALL: f64 = 0.5;

impl Round {
    fn draw(g: &mut fuzz::Rng, torus: bool) -> Self {
        let phi = g.range(0.0, std::f64::consts::TAU);
        let tilt = g.range(0.2, 1.2);
        let mut add: Vec<f64> = (0..g.below(30))
            .map(|_| g.range(1e-6, 1.0 - 1e-6))
            .collect();
        add.sort_by(f64::total_cmp);
        let (x, y) = (g.unit(), g.unit());
        let window = (x.min(y), x.max(y).max(x.min(y) + 1e-3).min(1.0));
        Self {
            torus,
            phi,
            tilt,
            add,
            window,
        }
    }

    fn surface(&self) -> Surface<f64> {
        if self.torus {
            Surface::Torus {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                major_radius: BIG,
                minor_radius: SMALL,
                u_ref: Vec3::unit_x(),
            }
        } else {
            Surface::Sphere {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: BIG,
                u_ref: Vec3::unit_x(),
            }
        }
    }

    fn net(&self, miss: f64) -> NurbsCurve3<f64> {
        let a = Vec3::new(self.phi.cos(), self.phi.sin(), 0.0);
        let (center, radius, b) = if self.torus {
            (Point3::origin() + a * BIG, SMALL + miss, Vec3::unit_z())
        } else {
            let across = Vec3::new(-self.phi.sin(), self.phi.cos(), 0.0);
            let b = across * self.tilt.cos() + Vec3::unit_z() * self.tilt.sin();
            (Point3::origin(), BIG + miss, b)
        };
        let w = std::f64::consts::FRAC_1_SQRT_2;
        let corners = [
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
        let control = corners
            .iter()
            .map(|&(x, y)| center + a * (radius * x) + b * (radius * y))
            .collect();
        let weights = (0..9).map(|i| if i % 2 == 0 { 1.0 } else { w }).collect();
        NurbsCurve3::new(
            KnotVector::clamped(
                vec![
                    0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
                ],
                2,
            )
            .unwrap(),
            control,
            weights,
        )
        .unwrap()
        .refine_knots(&self.add)
        .unwrap()
    }

    fn name(&self) -> &'static str {
        if self.torus { "torus" } else { "sphere" }
    }
}

fn incidence(got: &Result<(), PcurveCertifyError>) -> Option<IncidenceRead> {
    match got {
        Err(PcurveCertifyError::Incidence(refusal)) => Some(refusal.read()),
        _ => None,
    }
}

/// **No false Off, and every height ends by its supremum** (shape 1),
/// through the edge certificate: a knot-rich net along a ruling of its
/// plane, cylinder or cone, stated by the ruling's image, and a refined
/// circle on its sphere or torus, stated by the projected image the mint
/// derives for its on-chart twin, at a supremum of zero, in the band or
/// four times the escalation, ends as that supremum says
/// ([`Height::ends`]) at `f64` and at `Interval`; the circle through the
/// bounded read alone as well.
///
/// The oracle: a circle's distance is its offset, the same at every
/// parameter. A ruled net's is `|h|` times its pushed control's basis
/// function, whose peak is found by a dense walk ([`Net::peak`]): the
/// basis is a smooth piecewise polynomial, so the walk is within its
/// spacing squared of the peak, far inside the factors between the
/// heights and the band's edges. It is no oracle where the distance has
/// a kink (beside a cone's apex, the ruled nets stay on slant `[1, 2]`);
/// the apex rows are [`beside_the_apex`] and [`through_the_apex`].
#[test]
fn a_nets_read_on_an_analytic_chart_ends_by_its_supremum() {
    let mut g = fuzz::start("analytic_net_supremum");
    let mut unimaged = 0usize;
    for _ in 0..fuzz::scaled(16) {
        let net = draw_ruled(&mut g);
        let chart = Ruled::ALL[g.below(3)];
        let stated = if g.below(2) == 0 {
            Stated::Iso
        } else {
            Stated::General
        };
        let torus = g.below(2) == 0;
        let round = Round::draw(&mut g, torus);
        for height in Height::ALL {
            let miss = height.miss();
            let net = net.clone().missing(miss);
            let surface = round.surface();
            let circle = round.net(miss);
            for door in [F64, INTERVAL] {
                let got = (door.certify)(chart, &net, stated);
                assert!(
                    height.ends(&got, read(&got)),
                    "{chart:?} {stated:?} {height:?} at {}: {got:?} ({net:?}); {}",
                    door.name,
                    fuzz::replay()
                );
                let got = (door.read)(&surface, &circle, round.window);
                assert!(
                    height.ends(&got, incidence(&got)),
                    "{} read {height:?} at {}: {got:?} ({round:?}); {}",
                    round.name(),
                    door.name,
                    fuzz::replay()
                );
                let Some(got) = (door.certify_round)(&round, miss) else {
                    unimaged += 1;
                    continue;
                };
                assert!(
                    height.admits(&got, read(&got)),
                    "{} certified {height:?} at {}: {got:?} ({round:?}); {}",
                    round.name(),
                    door.name,
                    fuzz::replay()
                );
            }
        }
    }
    eprintln!("analytic_net_supremum: {unimaged} circle certificates the mint gave no twin image");
}

/// How a net beside a cone's apex ends.
#[derive(Clone, Copy, Debug)]
enum Apex {
    Off,
    Lever,
    On,
}

/// **A net beside a cone's apex** (the P2 row's): a ruling of the double
/// cone `30·zero` to the side of the apex, read over a span that passes
/// the apex between the samples. With a knot beside the apex it ends off;
/// as one span it ends in the lever (the parts' floors beside the apex
/// read no lever, so the rounds measure no miss), never certified. The
/// ruling itself, read from the apex along either nappe, is on.
fn beside_the_apex(door: Door) {
    let zero = band().zero();
    let (s, c) = ALPHA.sin_cos();
    // The apex's parameter, between the sampled read's samples.
    let apex = 1.0 / 64.0 + 1.0 / 128.0;
    let cases = [
        (30.0 * zero, (-1.0, 1.0), true, Apex::Off),
        (30.0 * zero, (-0.3, 0.9), true, Apex::Off),
        (30.0 * zero, (-1.0, 1.0), false, Apex::Lever),
        (0.0, (apex - 0.5, apex), false, Apex::On),
        (0.0, (apex, 1.0), false, Apex::On),
    ];
    for (shift, (lo, hi), knot, ends) in cases {
        let at = |t: f64| Point3::new(s * (t - apex), shift, c * (t - apex));
        let ts: Vec<f64> = if knot {
            vec![lo, apex, hi]
        } else {
            vec![lo, hi]
        };
        let mut knots = vec![lo];
        knots.extend(&ts);
        knots.push(hi);
        let net = NurbsCurve3::new(
            KnotVector::clamped(knots, 1).unwrap(),
            ts.iter().map(|&t| at(t)).collect(),
            vec![1.0; ts.len()],
        )
        .unwrap();
        let got = (door.read)(&Ruled::Cone.surface(), &net, (lo, hi));
        let read = incidence(&got);
        assert!(
            match ends {
                Apex::Off => matches!(read, Some(IncidenceRead::Off(_))),
                Apex::Lever =>
                    matches!(read, Some(IncidenceRead::Undecided(c)) if !c.margin.is_invalid()),
                Apex::On => got.is_ok(),
            },
            "at {}, {shift:e} beside the apex over [{lo}, {hi}] (a knot there: {knot}) ends \
             {ends:?}: {got:?}",
            door.name
        );
    }
}

#[test]
fn a_net_beside_a_cones_apex_ends_off_f64() {
    beside_the_apex(F64);
}

#[test]
fn a_net_beside_a_cones_apex_ends_off_interval() {
    beside_the_apex(INTERVAL);
}

/// **A bound limb 2 cannot read ends in the lever** (`net_bound_refusal`):
/// an upper bound past the band with no miss, or no certified bound at
/// all, is a straddle read in band with a margin, never a poisoned read
/// (which ends as a defect) and never off, at every scalar.
fn a_bound_that_does_not_read(door: Door) {
    use crate::edge_nurbs::AnalyticRung3Refusal as R;
    let b = band();
    let limit = R::KernelLimit {
        operand: geom::SurfaceKind::Plane,
        bound: 4.0 * b.escalate(),
        margin: geom_core::MarginDiag::value(4.0 * b.escalate()),
        width: 0.0,
        extent: 1.0,
    };
    let unbounded = R::NoOffsetBound {
        operand: geom::SurfaceKind::Plane,
        why: "a row's",
    };
    for refusal in [limit, unbounded] {
        let got = (door.refusal)(refusal);
        let read = match &got {
            PcurveCertifyError::Incidence(r) => Some(r.read()),
            _ => None,
        };
        assert!(
            matches!(read, Some(IncidenceRead::Undecided(c)) if !c.margin.is_invalid()),
            "{refusal:?} at {} ends in the lever with a margin: {got:?}",
            door.name
        );
    }
}

#[test]
fn a_bound_that_does_not_read_ends_in_the_lever_f64() {
    a_bound_that_does_not_read(F64);
}

#[test]
fn a_bound_that_does_not_read_ends_in_the_lever_interval() {
    a_bound_that_does_not_read(INTERVAL);
}

/// **A ruling of the double cone through its apex** (the review's probe
/// of PR 6826): on the chart, crossing from one nappe to the other, the
/// composite has no bound on the nappe it does not read, and the read
/// ends in the lever with a margin, never a defect and never off. That
/// it does not certify is a completeness hole, filed
/// (`work/pcert/an-on-chart-net-through-a-double-cones-apex-ends-in-the-lever.md`).
fn through_the_apex(door: Door) {
    let (s, c) = ALPHA.sin_cos();
    for (lo, hi) in [(-1.0, 1.0), (-0.2, 1.0), (-1.0, 0.2)] {
        let at = |t: f64| Point3::new(s * t, 0.0, c * t);
        let net = NurbsCurve3::new(
            KnotVector::clamped(vec![lo, lo, hi, hi], 1).unwrap(),
            vec![at(lo), at(hi)],
            vec![1.0, 1.0],
        )
        .unwrap();
        let got = (door.read)(&Ruled::Cone.surface(), &net, (lo, hi));
        assert!(
            got.is_ok()
                || matches!(incidence(&got), Some(IncidenceRead::Undecided(c)) if !c.margin.is_invalid()),
            "at {}, the ruling through the apex over [{lo}, {hi}] is on or in the lever: {got:?}",
            door.name
        );
    }
}

#[test]
fn a_ruling_through_a_cones_apex_is_never_a_defect_f64() {
    through_the_apex(F64);
}

#[test]
fn a_ruling_through_a_cones_apex_is_never_a_defect_interval() {
    through_the_apex(INTERVAL);
}
