//! **A stated image on an analytic chart is bounded against its carrier
//! over its span** (`certify.rs` `stated_bound`, through
//! `stated::StatedDoor`): a carrier that lies on its plane, or on its
//! cylinder's or cone's ruling, and leaves its stated image between the
//! samples does not get an edge certificate. Its miss ends as the stated
//! image's ([`CertCheck::StatedImage`]): at a build in its lever, at rest
//! the stored culprit, at the import door the damaged file, never with a
//! tolerance; in band, the lever at every door. A carrier on its image
//! still certifies.
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
    DAMAGED_FILE_ENDING, FileCoincidence, Interval, KERNEL_OR_FILE_DEFECT_ENDING, Point2, Point3,
    Real, Vec3,
};
use test_utils::fuzz;

use super::Pcurve;
use super::spline_door_rows::{band, in_band};
use crate::recourse::{ReadAt, Reading};
use crate::{CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, NurbsLane, SurfaceKey};

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
    General(NurbsCurve2<f64>),
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
        Stated::General => Image::General(
            NurbsCurve2::new(
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![
                    Point2::new(p0[0], p0[1]),
                    Point2::new(p0[0] + pl[0], p0[1] + pl[1]),
                ],
                vec![1.0, 1.0],
            )
            .unwrap(),
        ),
    }
}

/// The edge certificate of `carrier` over `[t0, t1]` stated by `image`
/// on `surface`, at one scalar.
type Certify =
    fn(&Surface<f64>, &Image, &NurbsCurve3<f64>, (f64, f64)) -> Result<(), CertifyError>;

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
        (self.certify)(&chart.surface(), &image_of(chart, stated), &net.curve(chart), (0.0, 1.0))
    }
}

/// How a refusal ended: the stated image's miss shown, its read in band,
/// or anything else.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ended {
    Certified,
    Missed,
    InBand,
    Other,
}

fn ended(got: &Result<(), CertifyError>) -> Ended {
    match got {
        Ok(()) => Ended::Certified,
        Err(CertifyError::ResidualExceeded {
            check: CertCheck::StatedImage,
            ..
        }) => Ended::Missed,
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

/// **The witness's refusal ends as a stated image's**: at a build in its
/// lever, at rest the kernel's or the file's defect, at the import door
/// the damaged file; read in band, the lever at every door. No ending
/// offers a tolerance.
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
            let want = [None, Some(KERNEL_OR_FILE_DEFECT_ENDING), Some(DAMAGED_FILE_ENDING)];
            for (at, want) in doors.into_iter().zip(want) {
                let ending = past.ending(at).unwrap();
                match want {
                    None => assert!(ending.contains(lever), "{stated:?} at {}, {at:?}: {ending}", door.name),
                    Some(want) => assert!(ending.contains(want), "{stated:?} at {}, {at:?}: {ending}", door.name),
                }
                assert!(!ending.contains("tolerance"), "{stated:?} at {}, {at:?}: {ending}", door.name);
            }
            let in_band = door
                .net(Chart::Plane, &witness(0.0).missing(in_band()), stated)
                .unwrap_err();
            assert_eq!(ended(&Err(in_band.clone())), Ended::InBand, "{stated:?} at {}", door.name);
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
                    Chart::Plane => Point2::new(along * cos - across * sin, along * sin + across * cos),
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
        let images = std::iter::once(Image::General(image)).chain(iso);
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
