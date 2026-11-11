//! **A closed-form image stated on an analytic chart is bounded against
//! its carrier over its span** (`certify.rs` `stated_bound`, through
//! `stated::StatedDoor::Closed` and `closed_bound`): an image that leaves
//! its carrier between the edge certificate's samples does not certify,
//! and ends as the stated image's miss ([`CertifyError::StatedImageMissed`],
//! D4 ¶1 (iv)), its lever in band; an image on its carrier certifies in
//! every spelling of it (a phase a whole period away, a mirrored winding
//! against a mirrored frame, a sphere point's other name).
//!
//! **A curve in UV on a curved chart off one ruling is bounded too**
//! (`stated::StatedDoor::Patch` with `patch::PatchClass::Enclosed`): the
//! fitted door's composite against a patch that encloses the chart over
//! the image's window, its remainder in its controls.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT=closed-image-sampled` (every
//! closed form metered at the schedule's samples again), the harmonic
//! witness, the narrowed bump's past and in-band rows and the endings row
//! go red; with `analytic-image-sampled` (every analytic stated image
//! metered at the samples again), the curve-in-UV witness and its narrowed
//! bump go red too.

test_utils::gated_to![
    "crates/geom-brep/src/certify.rs",
    "crates/geom-brep/src/pcurve_cache.rs",
    "crates/geom-brep/src/pcurve_cache/closed_bound.rs",
    "crates/geom-brep/src/pcurve_cache/patch.rs",
    "crates/geom-brep/src/pcurve_cache/stated.rs",
    "crates/geom-brep/src/pcurve_cache/trig.rs",
];

use core::f64::consts::{PI, TAU};
use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, Surface};
use geom_core::spline::KnotVector;
use geom_core::{
    DAMAGED_FILE_ENDING, FileCoincidence, Interval, KERNEL_DEFECT_ENDING,
    KERNEL_OR_FILE_DEFECT_ENDING, Point2, Point3, Real, Vec2, Vec3,
};
use test_utils::fuzz;

use super::spline_door_rows::{band, in_band};
use super::{AxisMap, Channel, ChartMap, FocalImage, Pcurve, SpiricImage, chart_pcurve_over};
use crate::recourse::{ReadAt, Reading};
use crate::{
    CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, NurbsLane, SurfaceKey,
};

/// A closed-form image held at `f64`, lifted to a scalar.
fn lift<T: Real>(image: &Pcurve<f64>) -> Pcurve<T> {
    let c = T::from_f64;
    let channel = |ch: Channel<f64>| match ch {
        Channel::Angle { phase, winding } => Channel::Angle {
            phase: c(phase),
            winding,
        },
        Channel::Free { c: k, a, b, l } => Channel::Free {
            c: c(k),
            a: c(a),
            b: c(b),
            l: c(l),
        },
    };
    let p2 = |p: Point2<f64>| Point2::new(c(p.x), c(p.y));
    let v2 = |v: Vec2<f64>| Vec2::new(c(v.x), c(v.y));
    match image {
        &Pcurve::Harmonic { u, v } => Pcurve::Harmonic {
            u: channel(u),
            v: channel(v),
        },
        &Pcurve::IsoLine { p0, pl } => Pcurve::IsoLine {
            p0: p2(p0),
            pl: v2(pl),
        },
        Pcurve::General(net) => Pcurve::General(Arc::new(net.map_scalar(c))),
        &Pcurve::Spiric {
            major,
            minor,
            offset,
            image,
        } => Pcurve::Spiric {
            major: c(major),
            minor: c(minor),
            offset: c(offset),
            image: match image {
                SpiricImage::Cap { p0, pm, pa } => SpiricImage::Cap {
                    p0: p2(p0),
                    pm: v2(pm),
                    pa: v2(pa),
                },
                SpiricImage::Wall {
                    u0,
                    v0,
                    sense,
                    torus,
                } => SpiricImage::Wall {
                    u0: c(u0),
                    v0: c(v0),
                    sense: c(sense),
                    torus,
                },
            },
        },
        Pcurve::FocalSection(f) => Pcurve::FocalSection(FocalImage {
            u0: c(f.u0),
            t0: c(f.t0),
            v0: c(f.v0),
            va: c(f.va),
            vb: c(f.vb),
            vl: c(f.vl),
            beta: c(f.beta),
            sense: c(f.sense),
        }),
        Pcurve::Fitted(_) | Pcurve::IsoArc { .. } | Pcurve::Projected(_) => {
            unreachable!("the rows state the images this lift writes")
        }
    }
}

/// The edge certificate of `carrier` over `[t0, t1]` stated by `image`
/// on `surface`, at one scalar.
type Certify =
    fn(&Surface<f64>, &Pcurve<f64>, &Curve3<f64>, (f64, f64)) -> Result<(), CertifyError>;

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
            certify: |surface, image, carrier, (t0, t1)| {
                let c = <$t as Real>::from_f64;
                let mut table: slotmap::SlotMap<SurfaceKey, Surface<$t>> =
                    slotmap::SlotMap::with_key();
                let key = table.insert(surface.map_scalar(c));
                let carrier = carrier.map_scalar(c);
                let (start, end) = (carrier.eval(c(t0)), carrier.eval(c(t1)));
                EdgeCurve::certify(
                    EdgeCurveSpec {
                        description: EdgeDescriptionSpec::chart_image(key, lift::<$t>(image)),
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

/// How a certificate ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ended {
    Certified,
    Missed,
    InBand,
    Unsettled,
    /// The fitted composite's sup that clears neither way with no miss
    /// located (`LaneGap::Envelope`).
    EnvelopeGap,
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
        Err(CertifyError::NoGeneralLane {
            gap: crate::LaneGap::Envelope,
            ..
        }) => Ended::EnvelopeGap,
        Err(_) => Ended::Other,
    }
}

/// The plane `z = 0`, `u` along `x̂`.
fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// **The harmonic witness** (the row's first): the line
/// `(0, 0.3, 0) + t·x̂` over `[0, 8π]` on the plane, stated by its own
/// image with `b·sin t` added to `v`. The schedule's samples sit at
/// `t = kπ`, where the sine vanishes.
fn harmonic_witness(b: f64) -> (Surface<f64>, Pcurve<f64>, Curve3<f64>, (f64, f64)) {
    let image = Pcurve::free_harmonic(
        Point2::new(0.0, 0.3),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, b),
        Vec2::new(1.0, 0.0),
    );
    let carrier = Curve3::line(Point3::new(0.0, 0.3, 0.0), Vec3::unit_x());
    (plane(), image, carrier, (0.0, 8.0 * PI))
}

impl Door {
    fn harmonic(self, b: f64) -> Result<(), CertifyError> {
        let (surface, image, carrier, span) = harmonic_witness(b);
        (self.certify)(&surface, &image, &carrier, span)
    }
}

/// The witness ends as the stated image's miss at a millimetre and at a
/// micrometre; on its line it certifies. Red on main, where both
/// certified.
fn the_harmonic_witness(door: Door) {
    // A micrometre where it is past the band (every row but `1e-6`'s).
    for b in [1e-3, 1e-6]
        .into_iter()
        .filter(|&b| b > 4.0 * band().escalate())
    {
        let got = door.harmonic(b);
        assert_eq!(
            ended(&got),
            Ended::Missed,
            "at {}: a harmonic image {b:e} m off its line between the samples ends as its \
             miss: {got:?}",
            door.name
        );
    }
    let on = door.harmonic(0.0);
    assert!(
        on.is_ok(),
        "at {}: the image on its line certifies: {on:?}",
        door.name
    );
}

#[test]
fn a_harmonic_image_off_its_line_between_the_samples_ends_as_its_miss_f64() {
    the_harmonic_witness(F64);
}

#[test]
fn a_harmonic_image_off_its_line_between_the_samples_ends_as_its_miss_interval() {
    the_harmonic_witness(INTERVAL);
}

/// **A bump narrowed toward the band ends by its height**: past the
/// escalation, the stated image's miss; in band, its lever; within the
/// zero, certified.
fn harmonic_narrowed(door: Door) {
    let b = band();
    let rows = [
        (1e-3, Ended::Missed),
        (1e3 * b.escalate(), Ended::Missed),
        (4.0 * b.escalate(), Ended::Missed),
        (in_band(), Ended::InBand),
        (0.25 * b.zero(), Ended::Certified),
    ];
    for (height, want) in rows {
        let got = door.harmonic(height);
        assert_eq!(
            ended(&got),
            want,
            "at {}: a harmonic image {height:e} m off its line: {got:?}",
            door.name
        );
    }
}

#[test]
fn a_harmonic_bump_narrowed_toward_the_band_ends_by_its_height_f64() {
    harmonic_narrowed(F64);
}

#[test]
fn a_harmonic_bump_narrowed_toward_the_band_ends_by_its_height_interval() {
    harmonic_narrowed(INTERVAL);
}

/// **A bound that does not clear, with nothing located, is the lever**:
/// the plane image `v = 0.3 + k·(1 − cos t)` over `|t| ≤ 0.05` misses its
/// line by at most `k·(1 − cos 0.05) ≈ k/800`, but its coefficients'
/// bound reads `2k`. With `2k` past the band and `k/800` inside the zero,
/// the reading shows neither a miss nor none
/// ([`CertifyError::StatedImageUnsettled`]), and ends in the lever.
#[test]
fn a_bound_past_the_band_with_no_miss_located_is_the_lever() {
    let k = 100.0 * band().zero();
    let image = Pcurve::free_harmonic(
        Point2::new(0.0, 0.3 + k),
        Vec2::new(0.0, -k),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
    );
    let carrier = Curve3::line(Point3::new(0.0, 0.3, 0.0), Vec3::unit_x());
    for door in DOORS {
        let got = (door.certify)(&plane(), &image, &carrier, (-0.05, 0.05));
        assert_eq!(ended(&got), Ended::Unsettled, "at {}: {got:?}", door.name);
        let e = got.unwrap_err();
        for at in [
            ReadAt::Run(Reading::Build),
            ReadAt::Run(Reading::AtRest),
            FileCoincidence::new(band().zero() * 1e-3).into(),
        ] {
            let ending = e.ending(at).unwrap();
            assert!(
                ending.contains("so the chart image this edge states follows its carrier")
                    && !ending.contains("tolerance"),
                "at {}, {at:?}: {ending}",
                door.name
            );
        }
    }
}

/// **The witness's refusal ends as a stated image's** (D4 ¶1 (iv)): at a
/// build the kernel's defect, at rest the kernel's or the file's, at the
/// import door the damaged file; in band, the lever at every door. No
/// ending offers a tolerance.
#[test]
fn a_closed_forms_miss_ends_by_its_door_and_offers_no_tolerance() {
    let file = FileCoincidence::new(band().zero() * 1e-3);
    let doors: [ReadAt; 3] = [
        ReadAt::Run(Reading::Build),
        ReadAt::Run(Reading::AtRest),
        file.into(),
    ];
    let want = [
        KERNEL_DEFECT_ENDING,
        KERNEL_OR_FILE_DEFECT_ENDING,
        DAMAGED_FILE_ENDING,
    ];
    for door in DOORS {
        let past = door.harmonic(1e-3).unwrap_err();
        for (at, want) in doors.into_iter().zip(want) {
            assert_eq!(past.ending(at).unwrap(), want, "at {}, {at:?}", door.name);
        }
        let lever = "so the chart image this edge states follows its carrier";
        let in_band = door.harmonic(in_band()).unwrap_err();
        for at in doors {
            let ending = in_band.ending(at).unwrap();
            assert!(
                ending.contains(lever) && !ending.contains("tolerance"),
                "at {}, in band, {at:?}: {ending}",
                door.name
            );
        }
    }
}

/// An analytic chart of every kind the rows draw.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    Spindle,
}

const KINDS: [Kind; 6] = [
    Kind::Plane,
    Kind::Cylinder,
    Kind::Cone,
    Kind::Sphere,
    Kind::Torus,
    Kind::Spindle,
];

/// A random frame: an origin, a unit axis and a unit `u_ref ⊥ axis`.
fn frame(g: &mut fuzz::Rng) -> (Point3<f64>, Vec3<f64>, Vec3<f64>) {
    let mut v = || Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0));
    let (a, b) = (v(), v());
    let axis = a.normalize();
    let u_ref = (b - axis * b.dot(axis)).normalize();
    let mut p = || g.range(-10.0, 10.0);
    (Point3::new(p(), p(), p()), axis, u_ref)
}

/// A unit vector `⊥ n`.
fn perpendicular(g: &mut fuzz::Rng, n: Vec3<f64>) -> Vec3<f64> {
    let b = Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0));
    let p = b - n * b.dot(n);
    if p.norm() < 1e-3 {
        perpendicular(g, n)
    } else {
        p.normalize()
    }
}

fn sign(g: &mut fuzz::Rng) -> f64 {
    if g.below(2) == 0 { 1.0 } else { -1.0 }
}

/// A chart of `kind` and one of its own curves over a span: a line or a
/// conic in a plane; a ruling, a rim or a tilted section on a cylinder;
/// a ruling or a rim on a cone; a parallel or a meridian on a sphere or a
/// torus; a parallel on a spindle.
fn drawn(g: &mut fuzz::Rng, kind: Kind) -> (Surface<f64>, Curve3<f64>, (f64, f64)) {
    let (o, axis, u_ref) = frame(g);
    let v_ref = axis.cross(u_ref);
    let radial = |u: f64| u_ref * u.cos() + v_ref * u.sin();
    let tangential = |u: f64| v_ref * u.cos() - u_ref * u.sin();
    let turn = |g: &mut fuzz::Rng| {
        let t0 = g.range(-7.0, 7.0);
        let t1 = t0
            + if g.below(4) == 0 {
                TAU
            } else {
                g.range(0.2, 6.0)
            };
        (t0, t1)
    };
    let length = |g: &mut fuzz::Rng| {
        let t0 = g.range(-5.0, 5.0);
        (t0, t0 + g.range(0.2, 30.0))
    };
    let circle = |g: &mut fuzz::Rng, c: Point3<f64>, n: Vec3<f64>, r: f64| {
        let x = perpendicular(g, n);
        Curve3::circle(c, n * sign(g), r, x)
    };
    match kind {
        Kind::Plane => {
            let surface = Surface::Plane {
                origin: o,
                normal: axis,
                u_ref,
            };
            let at = o + u_ref * g.range(-5.0, 5.0) + v_ref * g.range(-5.0, 5.0);
            match g.below(3) {
                0 => {
                    let dir = perpendicular(g, axis) * g.range(0.3, 3.0);
                    (surface, Curve3::line(at, dir), length(g))
                }
                1 => {
                    let r = g.range(0.1, 5.0);
                    (surface, circle(g, at, axis, r), turn(g))
                }
                _ => {
                    let x = perpendicular(g, axis);
                    let ellipse = Curve3::ellipse(
                        at,
                        axis * sign(g),
                        g.range(1.0, 5.0),
                        g.range(0.2, 0.9),
                        x,
                        band(),
                    )
                    .unwrap();
                    (surface, ellipse, turn(g))
                }
            }
        }
        Kind::Cylinder => {
            let r = g.range(0.1, 5.0);
            let surface = Surface::Cylinder {
                origin: o,
                axis,
                radius: r,
                u_ref,
            };
            let h = g.range(-5.0, 5.0);
            match g.below(3) {
                0 => {
                    let at = o + radial(g.range(-4.0, 4.0)) * r + axis * h;
                    let dir = axis * (sign(g) * g.range(0.3, 3.0));
                    (surface, Curve3::line(at, dir), length(g))
                }
                1 => (surface, circle(g, o + axis * h, axis, r), turn(g)),
                _ => {
                    // The section by a plane tilted `τ` about `e₂`: an
                    // ellipse of semi-axes `r/cos τ` along `e₁ + tan τ·axis`
                    // and `r` along `e₂`.
                    let tau = g.range(0.1, 1.2);
                    let phase = g.range(-4.0, 4.0);
                    let (e1, e2) = (radial(phase), tangential(phase));
                    let major = (e1 + axis * tau.tan()).normalize();
                    let ellipse = Curve3::ellipse(
                        o + axis * h,
                        major.cross(e2).normalize() * sign(g),
                        r / tau.cos(),
                        r,
                        major,
                        band(),
                    )
                    .unwrap();
                    (surface, ellipse, turn(g))
                }
            }
        }
        Kind::Cone => {
            let alpha = g.range(0.1, 1.2);
            let surface = Surface::Cone {
                apex: o,
                axis,
                half_angle: alpha,
                u_ref,
            };
            let slant = sign(g) * g.range(0.5, 5.0);
            if g.below(2) == 0 {
                // A ruling from slant `s₀`, clear of the apex: moving away
                // from it any distance, toward it short of `0.2`.
                let generator = axis * alpha.cos() + radial(g.range(-4.0, 4.0)) * alpha.sin();
                let (sigma, speed) = (sign(g), g.range(0.3, 3.0));
                let away = sigma * slant > 0.0;
                let reach = if away { 10.0 } else { slant.abs() - 0.2 } / speed;
                let line = Curve3::line(o + generator * slant, generator * (sigma * speed));
                (surface, line, (0.0, g.range(0.1, 1.0) * reach))
            } else {
                let c = o + axis * (slant * alpha.cos());
                (
                    surface,
                    circle(g, c, axis, slant.abs() * alpha.sin()),
                    turn(g),
                )
            }
        }
        Kind::Sphere => {
            let big_r = g.range(0.1, 5.0);
            let surface = Surface::Sphere {
                center: o,
                radius: big_r,
                axis,
                u_ref,
            };
            if g.below(2) == 0 {
                let lat = g.range(-1.4, 1.4);
                let c = o + axis * (big_r * lat.sin());
                (surface, circle(g, c, axis, big_r * lat.cos()), turn(g))
            } else {
                let n = tangential(g.range(-4.0, 4.0));
                (surface, circle(g, o, n, big_r), turn(g))
            }
        }
        Kind::Torus | Kind::Spindle => {
            let minor = g.range(0.1, 2.0);
            let major = if kind == Kind::Torus {
                minor * g.range(1.2, 4.0)
            } else {
                minor * g.range(-0.9, 0.9)
            };
            let surface = if kind == Kind::Torus {
                Surface::Torus {
                    center: o,
                    axis,
                    major_radius: major,
                    minor_radius: minor,
                    u_ref,
                }
            } else {
                Surface::Spindle {
                    center: o,
                    axis,
                    major_radius: major,
                    minor_radius: minor,
                    u_ref,
                }
            };
            if kind == Kind::Torus && g.below(2) == 0 {
                let u0 = g.range(-4.0, 4.0);
                let c = o + radial(u0) * major;
                (surface, circle(g, c, tangential(u0), minor), turn(g))
            } else {
                // A parallel on the sheet: `ρ = R + r·cos v > 0`.
                let v = loop {
                    let v = g.range(-3.0, 3.0);
                    if major + minor * v.cos() > 0.05 * minor {
                        break v;
                    }
                };
                let c = o + axis * (minor * v.sin());
                (
                    surface,
                    circle(g, c, axis, major + minor * v.cos()),
                    turn(g),
                )
            }
        }
    }
}

/// The world reflection through the plane through `q` normal to `n`.
#[derive(Clone, Copy)]
struct Mirror {
    q: Point3<f64>,
    n: Vec3<f64>,
}

impl Mirror {
    fn point(self, p: Point3<f64>) -> Point3<f64> {
        p - self.n * (2.0 * (p - self.q).dot(self.n))
    }

    fn vector(self, v: Vec3<f64>) -> Vec3<f64> {
        v - self.n * (2.0 * v.dot(self.n))
    }

    /// The reflected chart: every frame vector reflected, so its second
    /// leg `axis × u_ref` turns against the reflected one and the chart
    /// is the reflected chart read with its first angle (a plane, its
    /// second coordinate) negated.
    fn surface(self, s: &Surface<f64>) -> Surface<f64> {
        let (p, v) = (|x| self.point(x), |x| self.vector(x));
        match *s {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => Surface::Plane {
                origin: p(origin),
                normal: v(normal),
                u_ref: v(u_ref),
            },
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Surface::Cylinder {
                origin: p(origin),
                axis: v(axis),
                radius,
                u_ref: v(u_ref),
            },
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Surface::Cone {
                apex: p(apex),
                axis: v(axis),
                half_angle,
                u_ref: v(u_ref),
            },
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Surface::Sphere {
                center: p(center),
                radius,
                axis: v(axis),
                u_ref: v(u_ref),
            },
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Surface::Torus {
                center: p(center),
                axis: v(axis),
                major_radius,
                minor_radius,
                u_ref: v(u_ref),
            },
            Surface::Spindle {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Surface::Spindle {
                center: p(center),
                axis: v(axis),
                major_radius,
                minor_radius,
                u_ref: v(u_ref),
            },
            Surface::Nurbs(_) | Surface::Approx(_) => unreachable!("the rows draw analytic charts"),
        }
    }

    /// The reflected carrier on the same parameter: a conic's second leg
    /// is `axis × u_ref`, so its reflected axis is negated.
    fn carrier(self, c: &Curve3<f64>) -> Curve3<f64> {
        match c {
            Curve3::Line(l) => Curve3::line(self.point(l.origin), self.vector(l.dir)),
            Curve3::Circle(k) => Curve3::circle(
                self.point(k.frame.origin),
                -self.vector(k.frame.axis),
                k.radius,
                self.vector(k.frame.u_ref),
            ),
            Curve3::Ellipse(e) => Curve3::ellipse(
                self.point(e.frame.origin),
                -self.vector(e.frame.axis),
                e.semi_u,
                e.semi_v,
                self.vector(e.frame.u_ref),
                band(),
            )
            .unwrap(),
            Curve3::Spiric { .. } | Curve3::Nurbs(_) => unreachable!("the rows draw conics"),
        }
    }

    /// The image's map onto the reflected chart ([`Mirror::surface`]).
    fn image_map(s: &Surface<f64>) -> ChartMap<f64> {
        match s {
            Surface::Plane { .. } => ChartMap {
                u: AxisMap::identity(),
                v: AxisMap::reflect(),
            },
            _ => ChartMap {
                u: AxisMap::reflect(),
                v: AxisMap::identity(),
            },
        }
    }
}

/// A spelling of an image: its name, chart, carrier and image.
type Spelled = (&'static str, Surface<f64>, Curve3<f64>, Pcurve<f64>);

/// A drawn case: its name, chart, carrier and span.
type Case = (String, Surface<f64>, Curve3<f64>, (f64, f64));

/// The spellings of one image the rows certify: as derived, shifted a
/// whole period on each angle channel, the sphere's twin, and on the
/// reflected chart both mapped through the reflection and derived there.
fn spellings(
    g: &mut fuzz::Rng,
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    image: &Pcurve<f64>,
) -> Vec<Spelled> {
    let periods = surface.angle_periods().unwrap();
    let whole = |g: &mut fuzz::Rng, p: Option<_>| match p {
        Some(_) => AxisMap::shift(TAU * f64::from(i32::try_from(g.below(7)).unwrap() - 3)),
        None => AxisMap::identity(),
    };
    let shift = ChartMap {
        u: whole(g, periods.0),
        v: whole(g, periods.1),
    };
    let mut out = vec![
        ("derived", surface.clone(), carrier.clone(), image.clone()),
        (
            "shifted",
            surface.clone(),
            carrier.clone(),
            image.map_affine(shift).unwrap(),
        ),
    ];
    if matches!(surface, Surface::Sphere { .. }) {
        let twin = image.map_affine(ChartMap::sphere_involution()).unwrap();
        out.push(("twin", surface.clone(), carrier.clone(), twin));
    }
    let (q, _, n) = frame(g);
    let mirror = Mirror { q, n };
    let (s, c) = (mirror.surface(surface), mirror.carrier(carrier));
    let mapped = image.map_affine(Mirror::image_map(surface)).unwrap();
    out.push((
        "mirrored",
        s.clone(),
        c.clone(),
        mapped.map_affine(shift).unwrap(),
    ));
    if let Ok(derived) = chart_pcurve_over(&c, span.0, span.1, &s, band())
        && matches!(derived, Pcurve::Harmonic { .. })
    {
        out.push(("derived on the mirror", s, c, derived));
    }
    out
}

/// **No correct image is refused** (shape 1): on every chart kind, a
/// chart's own curve stated by its derived harmonic image, in every
/// spelling ([`spellings`]), certifies at both scalars.
#[test]
fn a_closed_form_on_its_carrier_certifies_in_every_spelling() {
    let mut g = fuzz::start("closed_form_on_its_carrier");
    let mut read = 0usize;
    for _ in 0..fuzz::scaled(24) {
        for kind in KINDS {
            let (surface, carrier, span) = drawn(&mut g, kind);
            let Ok(image) = chart_pcurve_over(&carrier, span.0, span.1, &surface, band()) else {
                continue;
            };
            if !matches!(image, Pcurve::Harmonic { .. }) {
                continue;
            }
            for (how, surface, carrier, image) in
                spellings(&mut g, &surface, &carrier, span, &image)
            {
                for door in DOORS {
                    let got = (door.certify)(&surface, &image, &carrier, span);
                    assert!(
                        got.is_ok(),
                        "{kind:?} {how} at {}: {image:?} on {carrier:?} over {span:?}: {got:?} \
                         ({})",
                        door.name,
                        fuzz::replay()
                    );
                    read += 1;
                }
            }
        }
    }
    println!("read {read} stated images");
}

/// The unit cylinder about `ẑ`.
fn unit_cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    }
}

/// **The curve-in-UV witness** (the row's second): the rim circle
/// `z = 0.5` of the unit cylinder, its angle the parameter, stated over
/// `[0, 1]` by the degree-1 curve in UV `(t, 0.5)` with knots `0, 1/256,
/// 1/128, 3/256, 1` and its `1/128` vertex slid `du` along `u`. The image
/// stays on the chart and leaves the rim by `du` between the samples
/// (`i/8`).
fn uv_witness(du: f64) -> (Surface<f64>, Pcurve<f64>, Curve3<f64>, (f64, f64)) {
    let breaks = [0.0, 1.0 / 256.0, 1.0 / 128.0, 3.0 / 256.0, 1.0];
    let mut knots = vec![0.0];
    knots.extend(breaks);
    knots.push(1.0);
    let control: Vec<Point2<f64>> = breaks
        .iter()
        .enumerate()
        .map(|(i, &g)| Point2::new(g + if i == 2 { du } else { 0.0 }, 0.5))
        .collect();
    let image = Pcurve::General(Arc::new(
        NurbsCurve2::new(
            KnotVector::clamped(knots, 1).unwrap(),
            control,
            vec![1.0; 5],
        )
        .unwrap(),
    ));
    let carrier = Curve3::circle(
        Point3::new(0.0, 0.0, 0.5),
        Vec3::unit_z(),
        1.0,
        Vec3::unit_x(),
    );
    (unit_cylinder(), image, carrier, (0.0, 1.0))
}

impl Door {
    fn uv(self, du: f64) -> Result<(), CertifyError> {
        let (surface, image, carrier, span) = uv_witness(du);
        (self.certify)(&surface, &image, &carrier, span)
    }
}

/// The witness ends as the stated image's miss at a millimetre and at a
/// micrometre; on the rim it certifies. Red on main, where both
/// certified.
fn the_uv_witness(door: Door) {
    for du in [1e-3, 1e-6]
        .into_iter()
        .filter(|&b| b > 4.0 * band().escalate())
    {
        let got = door.uv(du);
        assert_eq!(
            ended(&got),
            Ended::Missed,
            "at {}: a curve in UV {du:e} rad off its rim between the samples ends as its \
             miss: {got:?}",
            door.name
        );
    }
    let on = door.uv(0.0);
    assert!(
        on.is_ok(),
        "at {}: the curve on its rim certifies: {on:?}",
        door.name
    );
}

#[test]
fn a_curve_in_uv_off_its_rim_between_the_samples_ends_as_its_miss_f64() {
    the_uv_witness(F64);
}

#[test]
fn a_curve_in_uv_off_its_rim_between_the_samples_ends_as_its_miss_interval() {
    the_uv_witness(INTERVAL);
}

/// **The curve-in-UV bump narrowed toward the band**: the vertex's
/// slide `du` is the miss's peak on the unit rim (to first order in
/// `du`, which these heights make exact far past the band's resolution).
fn uv_narrowed(door: Door) {
    let b = band();
    let rows = [
        (1e3 * b.escalate(), Ended::Missed),
        (4.0 * b.escalate(), Ended::Missed),
        (in_band(), Ended::InBand),
        (0.25 * b.zero(), Ended::Certified),
    ];
    for (du, want) in rows {
        let got = door.uv(du);
        assert_eq!(
            ended(&got),
            want,
            "at {}: a curve in UV {du:e} rad off its rim: {got:?}",
            door.name
        );
    }
}

#[test]
fn a_curve_in_uv_bump_narrowed_toward_the_band_ends_by_its_height_f64() {
    uv_narrowed(F64);
}

#[test]
fn a_curve_in_uv_bump_narrowed_toward_the_band_ends_by_its_height_interval() {
    uv_narrowed(INTERVAL);
}

/// Certifies `image` on every door, or names the read that refused. At
/// a band finer than `1e-10` a correct image of metres' extent may read in
/// band, its rounding there the band's size
/// (`work/pcert/a-correct-stated-image-far-from-the-origin-reads-in-band-at-a-fine-band.md`):
/// it ends in its lever, or as the fitted composite's gap, never as a
/// miss.
fn certifies_everywhere(
    what: &str,
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    image: &Pcurve<f64>,
) {
    for door in DOORS {
        let got = (door.certify)(surface, image, carrier, span);
        let fine = band().zero() < 1e-10;
        assert!(
            got.is_ok()
                || (fine
                    && matches!(
                        ended(&got),
                        Ended::InBand | Ended::Unsettled | Ended::EnvelopeGap
                    )),
            "{what} at {}: {image:?} on {carrier:?} over {span:?}: {got:?} ({})",
            door.name,
            fuzz::replay()
        );
        if got.is_err() {
            println!("{what} at {}: in band at a fine band: {got:?}", door.name);
        }
    }
}

/// A spiric oval on a random ring torus, and the three charts that hold
/// it: its torus, the torus's mirror through the cutting plane, and the
/// plane.
fn spiric(g: &mut fuzz::Rng) -> (Curve3<f64>, [Surface<f64>; 3]) {
    let (o, axis, _) = frame(g);
    let n = perpendicular(g, axis);
    let minor = g.range(0.1, 1.0);
    let major = minor * g.range(1.5, 4.0);
    let offset = sign(g) * (major - minor) * g.range(0.1, 0.9);
    let carrier = Curve3::spiric(o, axis, n, major, minor, offset, band()).unwrap();
    let mut torus = |center| Surface::Torus {
        center,
        axis: axis * sign(g),
        major_radius: major,
        minor_radius: minor,
        u_ref: perpendicular(g, axis),
    };
    let own = torus(o);
    let mirror = torus(o + n * (2.0 * offset));
    let normal = n * sign(g);
    let plane = Surface::Plane {
        origin: o + n * offset + axis * g.range(-1.0, 1.0),
        normal,
        u_ref: perpendicular(g, normal),
    };
    (carrier, [own, mirror, plane])
}

/// **No spiric or focal-section image on its carrier is refused**
/// (shape 1): a spiric oval stated on its torus, its mirror torus and its
/// plane; a cone's tilted section on its cone; a Villarceau circle on its
/// torus. Each by its derived image, and shifted a whole period on each
/// angle channel, certifies at both scalars.
#[test]
fn a_spiric_or_focal_section_on_its_carrier_certifies() {
    let mut g = fuzz::start("spiric_focal_on_its_carrier");
    let mut read = [0usize; 2];
    for _ in 0..fuzz::scaled(16) {
        let mut cases = Vec::new();
        let (oval, charts) = spiric(&mut g);
        for chart in charts {
            cases.push(("spiric", chart, oval.clone()));
        }
        if let Some((cone, ellipse)) = super::envelope_lemma_fuzz::cone_section(&mut g) {
            cases.push(("cone section", cone, ellipse));
        }
        let (torus, circle) = super::envelope_lemma_fuzz::villarceau(&mut g);
        cases.push(("villarceau", torus, circle));
        for (what, surface, carrier) in cases {
            let t0 = g.range(-7.0, 7.0);
            let span = (
                t0,
                t0 + if g.below(4) == 0 {
                    TAU
                } else {
                    g.range(0.2, 6.0)
                },
            );
            let Ok(image) = chart_pcurve_over(&carrier, span.0, span.1, &surface, band()) else {
                continue;
            };
            let slot = match image {
                Pcurve::Spiric { .. } => 0,
                Pcurve::FocalSection(_) => 1,
                _ => continue,
            };
            let periods = surface.angle_periods().unwrap();
            let whole = |g: &mut fuzz::Rng, p: Option<_>| match p {
                Some(_) => AxisMap::shift(TAU * f64::from(i32::try_from(g.below(7)).unwrap() - 3)),
                None => AxisMap::identity(),
            };
            let shift = ChartMap {
                u: whole(&mut g, periods.0),
                v: whole(&mut g, periods.1),
            };
            certifies_everywhere(what, &surface, &carrier, span, &image);
            if let Ok(shifted) = image.map_affine(shift) {
                certifies_everywhere(what, &surface, &carrier, span, &shifted);
            }
            read[slot] += 1;
        }
    }
    println!(
        "read {} spiric and {} focal-section images",
        read[0], read[1]
    );
    assert!(
        read.iter().all(|&n| n > 0),
        "the sweep reads both kinds: {read:?} ({})",
        fuzz::replay()
    );
}

/// **No curve in UV on its carrier is refused on a curved chart**
/// (shape 1): a chart's own circle or line whose derived image is linear
/// in its parameter (a rim, a parallel, a meridian, a ruling), stated as
/// an iso line and as a degree-1 curve in UV with random breakpoints,
/// certifies at both scalars through the enclosing patch, or the ruling's;
/// with one control moved, it is never certified past its dense miss.
#[test]
fn a_curve_in_uv_on_its_carrier_certifies_on_a_curved_chart() {
    let mut g = fuzz::start("curve_in_uv_on_its_carrier");
    let mut read = 0usize;
    for _ in 0..fuzz::scaled(2) {
        for kind in &KINDS[1..] {
            let (surface, carrier, span) = drawn(&mut g, *kind);
            let Ok(image) = chart_pcurve_over(&carrier, span.0, span.1, &surface, band()) else {
                continue;
            };
            let Some((p0, pa, pb, pl)) = image.harmonic_form() else {
                continue;
            };
            if [pa.x, pa.y, pb.x, pb.y].iter().any(|&x| x != 0.0) {
                continue;
            }
            let iso = Pcurve::IsoLine { p0, pl };
            let mut breaks: Vec<f64> = (0..g.below(5))
                .map(|_| span.0 + (span.1 - span.0) * g.range(0.05, 0.95))
                .collect();
            breaks.sort_by(f64::total_cmp);
            breaks.dedup_by(|a, b| (*a - *b).abs() < 1e-3);
            let mut at = vec![span.0];
            at.extend(&breaks);
            at.push(span.1);
            let mut knots = vec![span.0];
            knots.extend(&at);
            knots.push(span.1);
            let control: Vec<Point2<f64>> = at.iter().map(|&t| p0 + pl * t).collect();
            let count = control.len();
            let kv = KnotVector::clamped(knots, 1).unwrap();
            let general = Pcurve::General(Arc::new(
                NurbsCurve2::new(kv.clone(), control, vec![1.0; count]).unwrap(),
            ));
            for (what, image) in [("iso line", iso), ("curve in UV", general.clone())] {
                certifies_everywhere(
                    &format!("{kind:?} {what}"),
                    &surface,
                    &carrier,
                    span,
                    &image,
                );
                read += 1;
            }
            // The curve in UV with one control moved: never certified past
            // its dense miss.
            let Pcurve::General(net) = &general else {
                unreachable!("built above")
            };
            let b = band();
            let delta = (b.zero() * 1e-2) * (1e5 * b.escalate() / b.zero()).powf(g.range(0.0, 1.0));
            let mut control = net.control().to_vec();
            let i = g.below(control.len());
            if g.below(2) == 0 {
                control[i].x += delta;
            } else {
                control[i].y += delta;
            }
            let wrong = Pcurve::General(Arc::new(
                NurbsCurve2::new(kv, control, net.weights().to_vec()).unwrap(),
            ));
            let miss = dense_miss(&surface, &wrong, &carrier, span);
            for door in DOORS {
                let got = (door.certify)(&surface, &wrong, &carrier, span);
                assert!(
                    got.is_err() || miss <= b.zero() * (1.0 + 1e-6),
                    "{kind:?} at {}: a curve in UV certified with a dense miss of {miss:e}: \
                     {wrong:?} on {carrier:?} over {span:?} ({})",
                    door.name,
                    fuzz::replay()
                );
            }
        }
    }
    println!("read {read} curves in UV");
}

/// `sup |C(t) − S(P(t))|` over the span, read densely: `4096` even points,
/// then a golden-section refinement about the largest. A lower bound on
/// the miss, short of it by `O(h²)` at the refined peak: far below the
/// band at these spans.
fn dense_miss(
    surface: &Surface<f64>,
    image: &Pcurve<f64>,
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
) -> f64 {
    let miss = |t: f64| {
        let p = image.eval(t);
        carrier.eval(t).distance(surface.eval(p.x, p.y))
    };
    let n = 4096;
    let at = |i: usize| {
        t0 + (t1 - t0)
            * (f64::from(u32::try_from(i).unwrap()) / f64::from(u32::try_from(n).unwrap()))
    };
    let (mut best, mut arg) = (0.0, t0);
    for i in 0..=n {
        let m = miss(at(i));
        if m > best {
            (best, arg) = (m, at(i));
        }
    }
    let step = (t1 - t0) / f64::from(u32::try_from(n).unwrap());
    let (mut a, mut b) = ((arg - step).max(t0), (arg + step).min(t1));
    let phi = 0.5 * (5f64.sqrt() - 1.0);
    for _ in 0..60 {
        let (c, d) = (b - phi * (b - a), a + phi * (b - a));
        if miss(c) > miss(d) {
            b = d;
        } else {
            a = c;
        }
    }
    best.max(miss(0.5 * (a + b)))
}

/// One of a closed form's numbers moved by `delta`.
fn nudged(image: &Pcurve<f64>, which: usize, delta: f64) -> Pcurve<f64> {
    let mut image = image.clone();
    let nudge = |ch: &mut Channel<f64>, k: usize| match ch {
        Channel::Angle { phase, .. } => *phase += delta,
        Channel::Free { c, a, b, l } => *[c, a, b, l][k % 4] += delta,
    };
    match &mut image {
        Pcurve::Harmonic { u, v } => {
            if which.is_multiple_of(2) {
                nudge(u, which / 2);
            } else {
                nudge(v, which / 2);
            }
        }
        Pcurve::Spiric { image, .. } => match image {
            SpiricImage::Cap { p0, pm, pa } => {
                *[
                    &mut p0.x, &mut p0.y, &mut pm.x, &mut pm.y, &mut pa.x, &mut pa.y,
                ][which % 6] += delta;
            }
            SpiricImage::Wall { u0, v0, .. } => *[u0, v0][which % 2] += delta,
        },
        Pcurve::FocalSection(f) => {
            *[
                &mut f.u0,
                &mut f.v0,
                &mut f.va,
                &mut f.vb,
                &mut f.vl,
                &mut f.beta,
            ][which % 6] += delta;
        }
        _ => unreachable!("the search nudges closed forms"),
    }
    image
}

/// **No image is certified past its miss** (shape 1, a counterexample
/// search): a chart's own curve of every kind, a spiric oval and a focal
/// section, stated by its derived image with one number moved by
/// `δ ∈ [10⁻²·zero, 10³·escalate]` (log-uniform). Whatever the door
/// answers, a certificate stands only where the dense miss is within the
/// zero, and a dense miss past twice the escalation ends as the image's
/// miss, located, or as its structure's refusal where the number moved
/// is one its structure fixes.
#[test]
fn no_closed_form_is_certified_past_its_miss() {
    let mut g = fuzz::start("closed_form_past_its_miss");
    let b = band();
    let mut tally = [0usize; 3];
    for _ in 0..fuzz::scaled(24) {
        let mut cases: Vec<Case> = KINDS
            .iter()
            .map(|&kind| {
                let (s, c, span) = drawn(&mut g, kind);
                (format!("{kind:?}"), s, c, span)
            })
            .collect();
        let (oval, charts) = spiric(&mut g);
        let turn = |g: &mut fuzz::Rng| {
            let t0 = g.range(-7.0, 7.0);
            (t0, t0 + g.range(0.2, TAU))
        };
        for chart in charts {
            cases.push(("spiric".into(), chart, oval.clone(), turn(&mut g)));
        }
        if let Some((cone, ellipse)) = super::envelope_lemma_fuzz::cone_section(&mut g) {
            cases.push(("cone section".into(), cone, ellipse, turn(&mut g)));
        }
        let (torus, circle) = super::envelope_lemma_fuzz::villarceau(&mut g);
        cases.push(("villarceau".into(), torus, circle, turn(&mut g)));
        for (what, surface, carrier, span) in cases {
            let Ok(image) = chart_pcurve_over(&carrier, span.0, span.1, &surface, band()) else {
                continue;
            };
            if !matches!(
                image,
                Pcurve::Harmonic { .. } | Pcurve::Spiric { .. } | Pcurve::FocalSection(_)
            ) {
                continue;
            }
            let delta = (b.zero() * 1e-2)
                * (1e5 * b.escalate() / b.zero()).powf(g.range(0.0, 1.0))
                * sign(&mut g);
            let wrong = nudged(&image, g.below(8), delta);
            let miss = dense_miss(&surface, &wrong, &carrier, span);
            for door in DOORS {
                let got = (door.certify)(&surface, &wrong, &carrier, span);
                let end = ended(&got);
                assert!(
                    end != Ended::Certified || miss <= b.zero() * (1.0 + 1e-6),
                    "{what} at {}: certified with a dense miss of {miss:e} (δ {delta:e}): {wrong:?} \
                     on {carrier:?} over {span:?} ({})",
                    door.name,
                    fuzz::replay()
                );
                // A number the image's structure fixes (a whole winding,
                // a unit sense) moved off it ends as that structure's
                // refusal, its own defect ending.
                let structural = matches!(got, Err(CertifyError::ChartImageUnavailable { .. }));
                assert!(
                    miss <= 2.0 * b.escalate() || end == Ended::Missed || structural,
                    "{what} at {}: a dense miss of {miss:e} (δ {delta:e}) ends {end:?}, not as its \
                     miss: {got:?} ({})",
                    door.name,
                    fuzz::replay()
                );
                tally[match end {
                    Ended::Certified => 0,
                    Ended::Missed => 1,
                    _ => 2,
                }] += 1;
            }
        }
    }
    println!(
        "certified {}, missed {}, otherwise {}",
        tally[0], tally[1], tally[2]
    );
}
