//! **A stated image on an analytic chart is bounded against its carrier
//! over its span** (`certify.rs` `stated_bound`, through
//! `stated::StatedDoor`): a carrier that lies on its plane, or on its
//! cylinder's or cone's ruling, and leaves its stated image between the
//! samples does not get an edge certificate. Its miss ends as the stated
//! image's ([`CertifyError::StatedImageMissed`], D4 ¶1 (iv)): at a build
//! the kernel's defect, at rest the stored culprit, at the import door the
//! damaged file, never with a tolerance; in band
//! ([`CertCheck::StatedImage`]), the lever at every door. A carrier on its
//! image still certifies.
//!
//! The carrier is a net whose controls are the chart's points at the
//! ruling's image at their Greville abscissae, so at height zero it is
//! the image mapped through the chart, on the image's parameter. One
//! control is moved WITHIN the chart, across the ruling: on the plane it
//! stays on the plane, and on a cylinder or cone it leaves the chart by
//! the chord's sagitta alone, far inside the band at the heights read
//! there, so the carrier's incidence read shows it on and only the
//! image's fidelity can see the move.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT=analytic-image-sampled` (every
//! analytic stated image metered at the schedule's samples again), the
//! witness rows and the narrowed bump's past and in-band rows go red.

test_utils::gated_to![
    "crates/geom-brep/src/certify.rs",
    "crates/geom-brep/src/pcurve_cache.rs",
    "crates/geom-brep/src/pcurve_cache/patch.rs",
    "crates/geom-brep/src/pcurve_cache/stated.rs",
];

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{
    DAMAGED_FILE_ENDING, FileCoincidence, Interval, KERNEL_DEFECT_ENDING,
    KERNEL_OR_FILE_DEFECT_ENDING, Point2, Point3, Real, Vec3,
};
use test_utils::fuzz;

use super::Pcurve;
use super::spline_door_rows::{band, in_band};
use crate::recourse::{ReadAt, Reading};
use crate::{
    CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, NurbsLane, SurfaceKey,
};

/// The cone's half-angle.
const ALPHA: f64 = 0.3;

/// A chart affine along the rows' image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Chart {
    Plane,
    Cylinder,
    Cone,
}

impl Chart {
    const ALL: [Self; 3] = [Self::Plane, Self::Cylinder, Self::Cone];

    fn surface(self) -> Surface<f64> {
        match self {
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
        }
    }

    /// The image `P(t) = p0 + pl·t`, `t ∈ [0, 1]`: on the plane the line
    /// `v = 0.3` moving in `u`; on the cylinder and cone the ruling
    /// `u = 0` moving in `v` (the cone's from slant 1 to 2).
    fn image(self) -> ([f64; 2], [f64; 2]) {
        match self {
            Self::Plane => ([0.0, 0.3], [1.0, 0.0]),
            Self::Cylinder => ([0.0, 0.0], [0.0, 1.0]),
            Self::Cone => ([0.0, 1.0], [0.0, 1.0]),
        }
    }

    /// The chart move, in `(u, v)`, that carries the chart's point at
    /// image parameter `t` a distance `h` across the image.
    fn across(self, t: f64, h: f64) -> [f64; 2] {
        let (p0, pl) = self.image();
        match self {
            Self::Plane => [0.0, h],
            Self::Cylinder => [h, 0.0],
            Self::Cone => [h / ((p0[1] + pl[1] * t) * ALPHA.sin()), 0.0],
        }
    }
}

/// A net under the image: degree `degree`, interior knots `interior`,
/// its controls the chart's points at the image's Greville abscissae,
/// the one at `bumped` moved `height` across the image within the chart.
#[derive(Clone, Debug)]
struct Net {
    degree: usize,
    interior: Vec<f64>,
    bumped: usize,
    height: f64,
}

/// The witness' bump: a polyline with one vertex at `t = 1/128` and its
/// neighbours `1/256` either side, wholly between the schedule's samples
/// (`i/8`).
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

    fn greville(&self) -> Vec<f64> {
        let (p, knots) = (self.degree, self.knots());
        #[allow(clippy::cast_precision_loss)]
        (0..knots.len() - p - 1)
            .map(|i| knots[i + 1..=i + p].iter().sum::<f64>() / p as f64)
            .collect()
    }

    fn curve(&self, chart: Chart) -> NurbsCurve3<f64> {
        let (p0, pl) = chart.image();
        let surface = chart.surface();
        let control: Vec<Point3<f64>> = self
            .greville()
            .iter()
            .enumerate()
            .map(|(i, &t)| {
                let h = if i == self.bumped { self.height } else { 0.0 };
                let [du, dv] = chart.across(t, h);
                surface.eval(p0[0] + pl[0] * t + du, p0[1] + pl[1] * t + dv)
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
    /// walk: the net's miss from its image peaks at `height` times it
    /// (the chart is affine along the image, so the miss is the bumped
    /// control's chord times its basis function).
    fn peak(&self) -> f64 {
        let knots = KnotVector::clamped(self.knots(), self.degree).unwrap();
        let count = self.greville().len();
        let mut unit = vec![Point2::new(0.0, 0.0); count];
        unit[self.bumped] = Point2::new(1.0, 0.0);
        let basis = NurbsCurve2::new(knots, unit, vec![1.0; count]).unwrap();
        let n = 1u32 << 14;
        (0..=n)
            .map(|i| f64::from(i) / f64::from(n))
            .chain(self.greville())
            .map(|t| basis.eval(t).x)
            .fold(0.0, f64::max)
    }

    /// This net with its height set so its miss from the image peaks at
    /// `miss` (to first order on a curved chart, where the move is an arc).
    fn missing(mut self, miss: f64) -> Self {
        self.height = miss / self.peak();
        self
    }
}

/// How the edge states its image.
#[derive(Clone, Copy, Debug)]
enum Stated {
    Iso,
    General,
}

const STATED: [Stated; 2] = [Stated::Iso, Stated::General];

/// A stated image, held at `f64` and lifted to each scalar.
#[derive(Clone, Debug)]
enum Image {
    /// `p0 + pl·t`.
    Iso([f64; 2], [f64; 2]),
    General(Arc<NurbsCurve2<f64>>),
}

impl Image {
    fn at<T: Real>(&self) -> Pcurve<T> {
        let c = T::from_f64;
        match self {
            Self::Iso(p0, pl) => Pcurve::IsoLine {
                p0: Point2::new(c(p0[0]), c(p0[1])),
                pl: geom_core::Vec2::new(c(pl[0]), c(pl[1])),
            },
            Self::General(net) => Pcurve::General(Arc::new(net.map_scalar(c))),
        }
    }
}

fn image_of(chart: Chart, stated: Stated) -> Image {
    let (p0, pl) = chart.image();
    match stated {
        Stated::Iso => Image::Iso(p0, pl),
        Stated::General => Image::General(Arc::new(
            NurbsCurve2::new(
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![
                    Point2::new(p0[0], p0[1]),
                    Point2::new(p0[0] + pl[0], p0[1] + pl[1]),
                ],
                vec![1.0, 1.0],
            )
            .unwrap(),
        )),
    }
}

/// The edge certificate of `carrier` over `[t0, t1]` stated by `image`
/// on `surface`, at one scalar.
type Certify = fn(&Surface<f64>, &Image, &NurbsCurve3<f64>, (f64, f64)) -> Result<(), CertifyError>;

/// One certifying scalar's door, instantiated per scalar by name.
#[derive(Clone, Copy)]
struct Door {
    name: &'static str,
    certify: Certify,
}

macro_rules! door_at {
    ($t:ty) => {
        Door {
            name: <$t as Real>::NAME,
            certify: |surface, image, net, (t0, t1)| {
                let c = <$t as Real>::from_f64;
                let mut table: slotmap::SlotMap<SurfaceKey, Surface<$t>> =
                    slotmap::SlotMap::with_key();
                let key = table.insert(surface.map_scalar(c));
                let carrier = Curve3::Nurbs(Arc::new(net.map_scalar(c)));
                let (start, end) = (carrier.eval(c(t0)), carrier.eval(c(t1)));
                EdgeCurve::certify(
                    EdgeCurveSpec {
                        description: EdgeDescriptionSpec::chart_image(key, image.at::<$t>()),
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
            },
        }
    };
}

const F64: Door = door_at!(f64);
const INTERVAL: Door = door_at!(Interval);
const DOORS: [Door; 2] = [F64, INTERVAL];

impl Door {
    fn net(self, chart: Chart, net: &Net, stated: Stated) -> Result<(), CertifyError> {
        (self.certify)(
            &chart.surface(),
            &image_of(chart, stated),
            &net.curve(chart),
            (0.0, 1.0),
        )
    }
}

/// How a refusal ended: the stated image's miss shown, its read in band,
/// its bound over the span past the band with no miss located, or
/// anything else.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ended {
    Certified,
    Missed,
    InBand,
    Unsettled,
    Other,
}

fn ended(got: &Result<(), CertifyError>) -> Ended {
    match got {
        Ok(()) => Ended::Certified,
        Err(CertifyError::StatedImageMissed { .. }) => Ended::Missed,
        Err(CertifyError::StatedImageUnsettled) => Ended::Unsettled,
        Err(CertifyError::Escalated {
            check: CertCheck::StatedImage,
            ..
        }) => Ended::InBand,
        Err(_) => Ended::Other,
    }
}

/// **The witness**: a net on its plane, a millimetre across its stated
/// image between the samples (on a cylinder or cone, a hundred times the
/// escalation across its ruling, whose sagitta stays inside the band),
/// stated as an iso line or a degree-1 curve in UV, ends as the stated
/// image's miss; the same net on its image certifies. Red on main, where
/// both images certified.
fn the_witness(door: Door) {
    for chart in Chart::ALL {
        let height = match chart {
            Chart::Plane => 1e-3,
            Chart::Cylinder | Chart::Cone => 100.0 * band().escalate(),
        };
        for stated in STATED {
            let got = door.net(chart, &witness(height), stated);
            assert_eq!(
                ended(&got),
                Ended::Missed,
                "{chart:?} {stated:?} at {}: a net {height:e} m across its image between the \
                 samples ends as the stated image's miss: {got:?}",
                door.name
            );
            let on = door.net(chart, &witness(0.0), stated);
            assert!(
                on.is_ok(),
                "{chart:?} {stated:?} at {}: the net on its image certifies: {on:?}",
                door.name
            );
        }
    }
}

#[test]
fn a_net_on_its_plane_a_millimetre_off_its_image_between_the_samples_ends_as_its_miss_f64() {
    the_witness(F64);
}

#[test]
fn a_net_on_its_plane_a_millimetre_off_its_image_between_the_samples_ends_as_its_miss_interval() {
    the_witness(INTERVAL);
}

/// **The witness's refusal ends as a stated image's** (D4 ¶1 (iv)): at a
/// build the kernel's defect, at rest the kernel's or the file's, at the
/// import door the damaged file; read in band, the lever at every door.
/// No ending offers a tolerance.
#[test]
fn a_stated_images_miss_ends_by_its_door_and_offers_no_tolerance() {
    let file = FileCoincidence::new(band().zero() * 1e-3);
    let doors: [ReadAt; 3] = [
        ReadAt::Run(Reading::Build),
        ReadAt::Run(Reading::AtRest),
        file.into(),
    ];
    for door in DOORS {
        for stated in STATED {
            let past = door.net(Chart::Plane, &witness(1e-3), stated).unwrap_err();
            let lever = "so the chart image this edge states follows its carrier";
            let want = [
                KERNEL_DEFECT_ENDING,
                KERNEL_OR_FILE_DEFECT_ENDING,
                DAMAGED_FILE_ENDING,
            ];
            for (at, want) in doors.into_iter().zip(want) {
                let ending = past.ending(at).unwrap();
                assert_eq!(ending, want, "{stated:?} at {}, {at:?}", door.name);
            }
            let in_band = door
                .net(Chart::Plane, &witness(0.0).missing(in_band()), stated)
                .unwrap_err();
            assert_eq!(
                ended(&Err(in_band)),
                Ended::InBand,
                "{stated:?} at {}",
                door.name
            );
            for at in doors {
                let ending = in_band.ending(at).unwrap();
                assert!(
                    ending.contains(lever) && !ending.contains("tolerance"),
                    "{stated:?} at {}, in band, {at:?}: {ending}",
                    door.name
                );
            }
        }
    }
}

/// **A bump narrowed toward the band ends by its height**: past the
/// escalation, the stated image's miss; in band, its lever; within the
/// zero, certified — on each chart, at each scalar.
fn narrowed(door: Door) {
    let b = band();
    let rows = [
        (1e-3, Ended::Missed),
        (1e3 * b.escalate(), Ended::Missed),
        (4.0 * b.escalate(), Ended::Missed),
        (in_band(), Ended::InBand),
        (0.25 * b.zero(), Ended::Certified),
    ];
    for chart in Chart::ALL {
        for (miss, want) in rows {
            // A curved chart's chord leaves it by its sagitta: the rows
            // there stop where it would reach the band.
            if chart != Chart::Plane && miss * miss > b.zero() {
                continue;
            }
            for stated in STATED {
                let got = door.net(chart, &witness(0.0).missing(miss), stated);
                assert_eq!(
                    ended(&got),
                    want,
                    "{chart:?} {stated:?} at {}, a miss of {miss:e}: {got:?}",
                    door.name
                );
            }
        }
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

/// A random frame: an origin in a ten-metre box and an orthonormal pair.
fn frame(g: &mut fuzz::Rng) -> (Point3<f64>, Vec3<f64>, Vec3<f64>) {
    let mut v = || Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0));
    let (a, b) = (v(), v());
    let normal = a.normalize();
    let u_ref = (b - normal * b.dot(normal)).normalize();
    let mut p = || g.range(-10.0, 10.0);
    (Point3::new(p(), p(), p()), normal, u_ref)
}

/// Clamped knots of `degree` with up to six interior knots at random.
fn knots(g: &mut fuzz::Rng, degree: usize) -> Vec<f64> {
    let mut interior: Vec<f64> = (0..g.below(7)).map(|_| g.range(0.05, 0.95)).collect();
    interior.sort_by(f64::total_cmp);
    interior.dedup_by(|a, b| (*a - *b).abs() < 1e-3);
    let mut knots = vec![0.0; degree + 1];
    knots.extend(interior);
    knots.extend(vec![1.0; degree + 1]);
    knots
}

/// **No correct image is refused**: a curve in UV of degree one to
/// three, rational or not, on a random plane, or an iso line or a curve
/// in UV on a random cylinder's or cone's ruling, stated against the net
/// that IS its image mapped through the chart, on its own parameter (the
/// chart is affine along it, so the net's controls are the chart's
/// points at the image's), over the whole span or a random sub-span,
/// certifies at both scalars.
#[test]
fn a_stated_image_on_its_carrier_certifies() {
    let mut g = fuzz::start("analytic_image_on_its_carrier");
    for _ in 0..fuzz::scaled(16) {
        let (origin, axis, u_ref) = frame(&mut g);
        let chart = Chart::ALL[g.below(3)];
        let surface = match chart {
            Chart::Plane => Surface::Plane {
                origin,
                normal: axis,
                u_ref,
            },
            Chart::Cylinder => Surface::Cylinder {
                origin,
                axis,
                radius: g.range(0.1, 5.0),
                u_ref,
            },
            Chart::Cone => Surface::Cone {
                apex: origin,
                axis,
                half_angle: g.range(0.1, 1.2),
                u_ref,
            },
        };
        let degree = 1 + g.below(3);
        let knots = knots(&mut g, degree);
        let count = knots.len() - degree - 1;
        let rational = g.below(2) == 0;
        let weights: Vec<f64> = (0..count)
            .map(|_| if rational { g.range(0.8, 1.25) } else { 1.0 })
            .collect();
        let u0 = g.range(-3.0, 3.0);
        // Controls that step forward along a direction, so the image and
        // its net run steadily forward (the edge's span meter). On a
        // curved chart the image keeps its ruling `u0` and steps in `v`,
        // the cone's slant clear of its apex.
        let (mut along, mut across) = (g.range(-5.0, 0.0), g.range(-5.0, 5.0));
        let heading = g.range(0.0, core::f64::consts::TAU);
        let (sin, cos) = heading.sin_cos();
        let control: Vec<Point2<f64>> = (0..count)
            .map(|_| {
                along += g.range(0.2, 2.0);
                across += g.range(-0.1, 0.1);
                match chart {
                    Chart::Plane => {
                        Point2::new(along * cos - across * sin, along * sin + across * cos)
                    }
                    Chart::Cylinder => Point2::new(u0, along),
                    Chart::Cone => Point2::new(u0, 6.0 + along),
                }
            })
            .collect();
        let kv = KnotVector::clamped(knots, degree).unwrap();
        let image = NurbsCurve2::new(kv.clone(), control.clone(), weights.clone()).unwrap();
        let net = NurbsCurve3::new(
            kv,
            control.iter().map(|p| surface.eval(p.x, p.y)).collect(),
            weights,
        )
        .unwrap();
        let (t0, t1) = if g.below(2) == 0 {
            (0.0, 1.0)
        } else {
            let (a, b) = (g.range(0.0, 0.45), g.range(0.55, 1.0));
            (a, b)
        };
        // The iso line where the curve in UV is one: degree one,
        // unweighted, its two ends the line's.
        let iso = (degree == 1 && !rational && count == 2).then(|| {
            let (a, b) = (control[0], control[1]);
            Image::Iso([a.x, a.y], [b.x - a.x, b.y - a.y])
        });
        let images = std::iter::once(Image::General(Arc::new(image))).chain(iso);
        for stated in images {
            for door in DOORS {
                let got = (door.certify)(&surface, &stated, &net, (t0, t1));
                assert!(
                    got.is_ok(),
                    "{chart:?} {stated:?} over [{t0}, {t1}] at {}: a correct image certifies: \
                     {got:?}; {}",
                    door.name,
                    fuzz::replay()
                );
            }
        }
    }
}

/// A cylinder off the origin and off the axes.
fn tilted_cylinder() -> Surface<f64> {
    let axis = Vec3::new(0.2, 0.3, 0.9).normalize();
    Surface::Cylinder {
        origin: Point3::new(1.0, 2.0, 3.0),
        axis,
        radius: 2.5,
        u_ref: (Vec3::unit_x() - axis * axis.x).normalize(),
    }
}

/// A polyline under the image `uv(t)` on `surface`: its controls the
/// chart's points at the image's controls, with knots clustered about
/// `t = 3/16` (between the samples `1/8` and `2/8`), the control nearest
/// it moved `bump` in `(u, v)`. Returns the image as a curve in UV and
/// the carrier.
fn polyline_under(
    surface: &Surface<f64>,
    uv: impl Fn(f64) -> [f64; 2],
    bump: [f64; 2],
) -> (NurbsCurve2<f64>, Curve3<f64>) {
    let knots = vec![
        0.0, 0.0, 0.05, 0.17, 0.18, 0.1875, 0.195, 0.205, 0.5, 0.7, 1.0, 1.0,
    ];
    let at: Vec<f64> = knots[1..knots.len() - 1].to_vec();
    let bumped = at
        .iter()
        .enumerate()
        .min_by(|a, b| (a.1 - 0.1875).abs().total_cmp(&(b.1 - 0.1875).abs()))
        .map(|(i, _)| i)
        .unwrap();
    let image: Vec<Point2<f64>> = at
        .iter()
        .map(|&t| {
            let [u, v] = uv(t);
            Point2::new(u, v)
        })
        .collect();
    let net: Vec<Point3<f64>> = image
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let d = if i == bumped { bump } else { [0.0, 0.0] };
            surface.eval(p.x + d[0], p.y + d[1])
        })
        .collect();
    let count = at.len();
    let kv = KnotVector::clamped(knots, 1).unwrap();
    (
        NurbsCurve2::new(kv.clone(), image, vec![1.0; count]).unwrap(),
        Curve3::Nurbs(Arc::new(
            NurbsCurve3::new(kv, net, vec![1.0; count]).unwrap(),
        )),
    )
}

/// The edge certificate of `image` (at `f64`, and `image_i` at
/// `Interval`) against `carrier` on `surface` over `[0, 1]`.
fn certify_both(
    surface: &Surface<f64>,
    image: Pcurve<f64>,
    image_i: Pcurve<Interval>,
    carrier: &Curve3<f64>,
) -> [Result<(), CertifyError>; 2] {
    macro_rules! at {
        ($t:ty, $image:expr) => {{
            let c = <$t as Real>::from_f64;
            let mut table: slotmap::SlotMap<SurfaceKey, Surface<$t>> = slotmap::SlotMap::with_key();
            let key = table.insert(surface.map_scalar(c));
            let carrier = carrier.map_scalar(c);
            let (start, end) = (carrier.eval(c(0.0)), carrier.eval(c(1.0)));
            EdgeCurve::certify(
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::chart_image(key, $image),
                    carrier,
                    param_start: c(0.0),
                    param_end: c(1.0),
                },
                start,
                end,
                |k| table.get(k).cloned(),
                geom_core::test_support::fixture_lever(<$t as Real>::one()),
                band(),
                NurbsLane::certified(),
            )
            .map(|_| ())
        }};
    }
    [at!(f64, image), at!(Interval, image_i)]
}

/// **A wrong image through its own pcurve door ends as the stated
/// image's miss, as the patch's does**: a projected image of a ruling on
/// a cylinder against a carrier moved a millimetre ALONG the ruling (so
/// it stays on the chart, and only the image's fidelity sees it); an iso
/// line and a curve in UV on a spline chart (a bilinear wall) against a
/// carrier moved a millimetre along the line. Each certifies on its
/// carrier.
#[test]
fn a_wrong_image_through_its_pcurve_door_ends_as_its_miss() {
    let fi = Interval::from_f64;
    let cylinder = tilted_cylinder();
    let ruling = |t: f64| [0.1, -1.0 + 2.5 * t];
    for (h, want) in [(1e-3, Ended::Missed), (0.0, Ended::Certified)] {
        let (_, on) = polyline_under(&cylinder, ruling, [0.0, 0.0]);
        let (_, carrier) = polyline_under(&cylinder, ruling, [0.0, h]);
        let image = super::projected::project(&on, Some((0.0, 1.0)), &cylinder, band()).unwrap();
        let interval = super::projected::project(
            &on.map_scalar(fi),
            Some((fi(0.0), fi(1.0))),
            &cylinder.map_scalar(fi),
            band(),
        )
        .unwrap();
        let got = certify_both(
            &cylinder,
            Pcurve::Projected(Box::new(image)),
            Pcurve::Projected(Box::new(interval)),
            &carrier,
        );
        for got in &got {
            assert_eq!(ended(got), want, "projected ruling, h = {h}: {got:?}");
            // Its fidelity term is read over the span: the refusal names
            // no sample, where the image meets its carrier at `t = 0`.
            if let Err(e) = got {
                assert!(
                    matches!(
                        e,
                        CertifyError::StatedImageMissed {
                            sample: crate::certify::NOT_A_SAMPLE,
                            ..
                        }
                    ) && e.to_string().contains("not a sampled check")
                        && !e.to_string().contains("at sample"),
                    "projected ruling, h = {h}: {e}"
                );
            }
        }
    }
    let (o, a, b) = (
        Point3::origin(),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 3.0, 0.5),
    );
    let kv = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let wall = Surface::Nurbs(Arc::new(
        geom::NurbsSurface::new(kv(), kv(), vec![o, o + b, o + a, o + a + b], vec![1.0; 4])
            .unwrap(),
    ));
    // The iso lane reads a carrier in its column's own spline space (a
    // polyline under it refuses as the lane's gap), so its wrong image is
    // a column beside the carrier's: `u` off by `du`, a miss of `2·du`.
    let column = |u: f64| {
        Curve3::Nurbs(Arc::new(
            NurbsCurve3::new(kv(), vec![o + a * u, o + a * u + b], vec![1.0; 2]).unwrap(),
        ))
    };
    for (du, want) in [(5e-4, Ended::Missed), (0.0, Ended::Certified)] {
        let iso = Image::Iso([0.3 + du, 0.0], [0.0, 1.0]);
        let got = certify_both(&wall, iso.at(), iso.at(), &column(0.3));
        for got in &got {
            assert_eq!(ended(got), want, "spline wall iso line, du = {du}: {got:?}");
        }
    }
    for (h, want) in [(1e-3, Ended::Missed), (0.0, Ended::Certified)] {
        let (net, carrier) = polyline_under(&wall, |t| [0.3, 0.1 + 0.8 * t], [0.0, h]);
        let net = Arc::new(net);
        let got = certify_both(
            &wall,
            Pcurve::General(Arc::clone(&net)),
            Pcurve::General(Arc::new(net.map_scalar(fi))),
            &carrier,
        );
        for got in &got {
            assert_eq!(ended(got), want, "spline wall General, h = {h}: {got:?}");
        }
    }
}

/// **A correct image far from the origin certifies or ends in the
/// lever**: on a tilted plane and on a cylinder's ruling, from 100 m to
/// 10 km out, an iso line and a curve in UV stated against their own
/// polyline (and, on the plane, against the line they are), at both
/// scalars. The patch's corners and the composite carry the coordinates'
/// rounding, which past some distance outgrows a fine band (as the
/// endpoint pins' and the net's incidence read's rounding do): the stated
/// image then reads in band, its lever, never a miss
/// (`work/pcert/a-correct-stated-image-far-from-the-origin-reads-in-band-at-a-fine-band.md`).
#[test]
fn a_correct_image_far_out_certifies_or_ends_in_its_lever() {
    let fi = Interval::from_f64;
    let n = Vec3::new(0.3, -0.5, 0.8).normalize();
    let u = (Vec3::new(1.0, 0.2, 0.0) - n * Vec3::new(1.0, 0.2, 0.0).dot(n)).normalize();
    let mut levers = 0usize;
    for far in [1e2, 1e3, 1e4] {
        let plane = Surface::Plane {
            origin: Point3::new(far, -far, 0.7 * far),
            normal: n,
            u_ref: u,
        };
        let (p0, pl) = ([0.37 * far, -0.21 * far], [0.6e-2 * far, 0.8e-2 * far]);
        let (net, polyline) = polyline_under(
            &plane,
            |t| [p0[0] + pl[0] * t, p0[1] + pl[1] * t],
            [0.0, 0.0],
        );
        let o = plane.eval(p0[0], p0[1]);
        let line = Curve3::line(o, plane.eval(p0[0] + pl[0], p0[1] + pl[1]) - o);
        let cylinder = Surface::Cylinder {
            origin: Point3::new(far, far, -far),
            axis: n,
            radius: 0.5 * far,
            u_ref: u,
        };
        let (cnet, cpolyline) = polyline_under(
            &cylinder,
            |t| [1.3, -0.2 * far + 1e-2 * far * t],
            [0.0, 0.0],
        );
        let rows = [
            (&plane, [p0, pl], &net, &polyline, "plane, polyline"),
            (&plane, [p0, pl], &net, &line, "plane, line"),
            (
                &cylinder,
                [[1.3, -0.2 * far], [0.0, 1e-2 * far]],
                &cnet,
                &cpolyline,
                "cylinder",
            ),
        ];
        for (surface, [p0, pl], net, carrier, name) in rows {
            let iso = Image::Iso(p0, pl);
            let got = [
                certify_both(surface, iso.at(), iso.at(), carrier),
                certify_both(
                    surface,
                    Pcurve::General(Arc::new(net.clone())),
                    Pcurve::General(Arc::new(net.map_scalar(fi))),
                    carrier,
                ),
            ];
            for got in got.iter().flatten() {
                // The stated image's own reading: never a miss, and in band
                // its lever. (Far out, the endpoint pins and the net's
                // incidence read meet their own rounding first.)
                let e = ended(got);
                levers += usize::from(e == Ended::InBand);
                assert!(
                    e != Ended::Missed,
                    "{name} {far:e} m out, a correct image: {got:?}"
                );
                if let Err(
                    e @ CertifyError::Escalated {
                        check: CertCheck::StatedImage,
                        ..
                    },
                ) = got
                {
                    let ending = e.ending(Reading::Build).unwrap();
                    assert!(
                        ending.starts_with(geom_core::MOVE_RECOURSE),
                        "{name} {far:e} m out, in band: {ending}"
                    );
                }
            }
        }
    }
    eprintln!(
        "a correct image far out: {levers} read in band at ε {:e}",
        band().zero()
    );
}

/// **A route decided once** (the review's interval iso line): an iso
/// line at `Interval` whose `u` is an enclosure, not a point, but does
/// not move, routes to the ruling's patch and certifies there against
/// its carrier on the ruling; its patch reads the enclosure as the
/// ruling's `u`.
#[test]
fn an_iso_line_whose_u_is_an_enclosure_certifies_on_its_ruling() {
    let cylinder = tilted_cylinder();
    let (_, carrier) = polyline_under(&cylinder, |t| [0.1, -1.0 + 2.5 * t], [0.0, 0.0]);
    let fi = Interval::from_f64;
    let image: Pcurve<Interval> = Pcurve::IsoLine {
        p0: Point2::new(Interval::from_bounds(0.1, 0.1 + 1e-15), fi(-1.0)),
        pl: geom_core::Vec2::new(fi(0.0), fi(2.5)),
    };
    let surface = cylinder.map_scalar(fi);
    assert!(
        matches!(
            super::StatedDoor::of(&surface, &image),
            Some(super::StatedDoor::Patch { .. })
        ),
        "the iso line routes to the ruling's patch"
    );
    let [_, got] = certify_both(
        &cylinder,
        Pcurve::IsoLine {
            p0: Point2::new(0.1, -1.0),
            pl: geom_core::Vec2::new(0.0, 2.5),
        },
        image,
        &carrier,
    );
    assert!(got.is_ok(), "the iso line certifies on its ruling: {got:?}");
}

/// **An iso line on a spline wall that leaves its carrier between the
/// samples ends in its lever, never as the lane's gap** (the review's
/// column probe): an affine bilinear wall whose `v` knots crowd `3/16`,
/// the carrier its column `u₀` in the column's own spline space with one
/// control moved 1 mm along `v`. The iso door certifies the class and
/// reads the bump only through its envelope, a sum of upper bounds (the
/// control-difference hull and the snap slacks) that locates no point,
/// so the reading is unsettled: the lever at every door. On its column
/// the carrier certifies.
#[test]
fn an_iso_line_off_its_column_between_the_samples_ends_in_its_lever() {
    let knots = vec![
        0.0, 0.0, 0.05, 0.17, 0.18, 0.1875, 0.195, 0.205, 0.5, 0.7, 1.0, 1.0,
    ];
    let at: Vec<f64> = knots[1..knots.len() - 1].to_vec();
    let bumped = at
        .iter()
        .enumerate()
        .min_by(|a, b| (a.1 - 0.1875).abs().total_cmp(&(b.1 - 0.1875).abs()))
        .map(|(i, _)| i)
        .unwrap();
    let (o, a, b) = (
        Point3::origin(),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 3.0, 0.5),
    );
    let kv = KnotVector::clamped(knots, 1).unwrap();
    let mut control = Vec::new();
    for u in [0.0, 1.0] {
        for &v in &at {
            control.push(o + a * u + b * v);
        }
    }
    let count = control.len();
    let wall = Surface::Nurbs(Arc::new(
        geom::NurbsSurface::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            kv.clone(),
            control,
            vec![1.0; count],
        )
        .unwrap(),
    ));
    for u0 in [0.0, 0.3] {
        for (h, want) in [(1e-3, Ended::Unsettled), (0.0, Ended::Certified)] {
            let net: Vec<Point3<f64>> = at
                .iter()
                .enumerate()
                .map(|(j, &v)| wall.eval(u0, v + if j == bumped { h } else { 0.0 }))
                .collect();
            let carrier = Curve3::Nurbs(Arc::new(
                NurbsCurve3::new(kv.clone(), net, vec![1.0; at.len()]).unwrap(),
            ));
            let iso = Image::Iso([u0, 0.0], [0.0, 1.0]);
            for got in certify_both(&wall, iso.at(), iso.at(), &carrier) {
                assert_eq!(ended(&got), want, "column {u0}, h = {h}: {got:?}");
                if let Err(e) = got {
                    for at in [Reading::Build, Reading::AtRest] {
                        let ending = e.ending(at).unwrap();
                        assert!(
                            ending.starts_with(geom_core::MOVE_RECOURSE)
                                && !ending.contains(geom_core::NOT_YET_ENDING),
                            "column {u0}, h = {h}, {at:?}: {ending}"
                        );
                    }
                }
            }
        }
    }
}
