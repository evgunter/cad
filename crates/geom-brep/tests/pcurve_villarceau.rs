//! A Villarceau circle's exact chart image on its torus
//! (`Pcurve::FocalSection`'s torus instance): derived by `chart_pcurve`
//! on either family and traversal, exact against the carrier through
//! the chart map, certified by its envelope alone at `f64` and at
//! `Interval`, refused when any of its numbers is wrong; and a circle
//! the torus's incidence test reads on the torus that is none of its
//! circles refuses as grazing it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use core::f64::consts::TAU;
use geom::{Curve3, Surface};
use geom_brep::{
    CERT_SAMPLES, EnvelopeStatement, FocalImage, Grazer, Pcurve, PcurveCache, PcurveCertifyError,
    chart_pcurve,
};
use geom_core::{Bounds, Interval, Point3, Real, Tol, Vec3};

const MAJOR: f64 = 2.0;
const MINOR: f64 = 0.7;

/// A ring torus off the origin about a skew axis.
fn torus() -> Surface<f64> {
    let axis = Vec3::new(0.1, 0.2, 1.0).normalize();
    let seed = Vec3::unit_x();
    Surface::Torus {
        center: Point3::new(0.3, -0.2, 0.5),
        axis,
        major_radius: MAJOR,
        minor_radius: MINOR,
        u_ref: (seed - axis * seed.dot(axis)).normalize(),
    }
}

/// The Villarceau circle of `torus()` whose centre is `r` along the
/// equatorial direction at azimuth `phi`, on `family` (`±1`, the side
/// its upper half leans to), traversed with `traversal` (`±1`) from a
/// start `psi` past the point farthest from the axis.
fn villarceau(phi: f64, family: f64, traversal: f64, psi: f64) -> Curve3<f64> {
    let Surface::Torus {
        center,
        axis,
        u_ref,
        ..
    } = torus()
    else {
        unreachable!()
    };
    let d = u_ref * phi.cos() + axis.cross(u_ref) * phi.sin();
    let tilt = (MINOR / MAJOR).asin();
    let lean = d.cross(axis) * tilt.cos() + axis * (family * tilt.sin());
    Curve3::Circle {
        center: center + d * MINOR,
        axis: d.cross(lean) * traversal,
        radius: MAJOR,
        u_ref: d * psi.cos() + lean * psi.sin(),
    }
}

/// The worst distance of a carrier's points from `torus()`.
fn off_torus(carrier: &Curve3<f64>) -> f64 {
    let Surface::Torus {
        center,
        axis,
        major_radius,
        minor_radius,
        ..
    } = torus()
    else {
        unreachable!()
    };
    (0..256)
        .map(|i| {
            let q = carrier.eval(f64::from(i) * TAU / 256.0) - center;
            let h = q.dot(axis);
            let rho = (q - axis * h).norm();
            ((rho - major_radius).hypot(h) - minor_radius).abs()
        })
        .fold(0.0, f64::max)
}

/// The sampled sup of `|S(P(t)) − C(t)|` over `[t0, t1]`.
fn sampled_sup(image: &Pcurve<f64>, carrier: &Curve3<f64>, t0: f64, t1: f64) -> f64 {
    let surface = torus();
    (0..=4096)
        .map(|i| {
            let t = t0 + (t1 - t0) * f64::from(i) / 4096.0;
            let uv = image.eval(t);
            surface.eval(uv.x, uv.y).distance(carrier.eval(t))
        })
        .fold(0.0, f64::max)
}

/// The Villarceau circles of every pose the rows read.
fn poses() -> Vec<(String, Curve3<f64>)> {
    let mut out = Vec::new();
    for family in [1.0, -1.0] {
        for traversal in [1.0, -1.0] {
            for (phi, psi) in [(0.4, 0.0), (2.9, -1.3), (-1.7, 2.6)] {
                out.push((
                    format!("family {family}, traversal {traversal}, phi {phi}, psi {psi}"),
                    villarceau(phi, family, traversal, psi),
                ));
            }
        }
    }
    out
}

/// **Every family, traversal and start images exactly.** The image is
/// the focal section with `|vl| = 1`, `va = vb = 0` and
/// `β = −r/(R + √(R² − r²))`; it maps onto the carrier at rounding
/// scale over more than a turn; the two families are told apart by the
/// sign of `vl·sense` alone; and the row certifies over a part span and
/// a whole turn with a rounding-scale envelope, its schedule run as the
/// cross-check.
#[test]
fn every_family_and_traversal_images_exactly_and_certifies() {
    let surface = torus();
    let want_beta = -MINOR / (MAJOR + (MAJOR * MAJOR - MINOR * MINOR).sqrt());
    for (what, carrier) in poses() {
        assert!(off_torus(&carrier) < 1e-14, "{what}: on the torus");
        let image = chart_pcurve(&carrier, &surface, band())
            .unwrap_or_else(|e| panic!("{what}: no image: {e}"));
        let Pcurve::FocalSection(FocalImage {
            t0,
            va,
            vb,
            vl,
            beta,
            sense,
            ..
        }) = image
        else {
            panic!("{what}: expected the focal-section image, got {image:?}");
        };
        assert_eq!((va, vb), (0.0, 0.0), "{what}");
        assert_eq!((vl.abs(), sense.abs()), (1.0, 1.0), "{what}");
        assert_eq!(beta, want_beta, "{what}");
        // The vertex is the point farthest from the axis, on the outer
        // equator, where the tube angle is zero.
        let vertex = image.eval(t0);
        let outer = surface.eval(vertex.x, 0.0);
        assert!(
            carrier.eval(t0).distance(outer) < 1e-12 && vertex.y.abs() < 1e-12,
            "{what}: the vertex {vertex:?} is not the outer-equator point"
        );
        let sup = sampled_sup(&image, &carrier, -3.0, 9.0);
        assert!(sup < 1e-12, "{what}: off the carrier by {sup:e}");
        for (a, b) in [(0.3, 4.0), (t0 - 1.0, t0 - 1.0 + TAU)] {
            let cache = PcurveCache::certify(image.clone(), a, b, &carrier, &surface, band())
                .unwrap_or_else(|e| panic!("{what}, [{a}, {b}]: {e}"));
            let cert = cache.certificate();
            assert!(
                cert.envelope < 1e-12,
                "{what}: envelope {:e}",
                cert.envelope
            );
            assert!(
                matches!(cert.statement, EnvelopeStatement::MapResidualClosedForm),
                "{what}"
            );
            assert_eq!(
                cert.samples, CERT_SAMPLES,
                "{what}: the schedule cross-checks"
            );
        }
    }
    // The family is the sign of `vl·sense`, whatever the traversal (a
    // reversed traversal flips both).
    let signature = |family: f64, traversal: f64| {
        let Ok(Pcurve::FocalSection(FocalImage { vl, sense, .. })) =
            chart_pcurve(&villarceau(0.4, family, traversal, 0.0), &surface, band())
        else {
            unreachable!()
        };
        vl * sense
    };
    assert_eq!(signature(1.0, 1.0), signature(1.0, -1.0));
    assert_eq!(signature(-1.0, 1.0), signature(-1.0, -1.0));
    assert_eq!(signature(1.0, 1.0), -signature(-1.0, 1.0));
}

/// **The certifying scalar.** The same rows, lifted to `Interval`,
/// certify with their envelope alone (no schedule is run at an exact
/// witness), and the envelope's upper end stays at rounding scale.
#[test]
fn the_rows_certify_at_the_interval_scalar() {
    let lift = <Interval as Real>::from_f64;
    let surface = torus().map_scalar(lift);
    for (what, carrier) in poses() {
        let Ok(Pcurve::FocalSection(FocalImage {
            u0,
            t0,
            v0,
            va,
            vb,
            vl,
            beta,
            sense,
        })) = chart_pcurve(&carrier, &torus(), band())
        else {
            panic!("{what}: no image")
        };
        let image = Pcurve::FocalSection(FocalImage {
            u0: lift(u0),
            t0: lift(t0),
            v0: lift(v0),
            va: lift(va),
            vb: lift(vb),
            vl: lift(vl),
            beta: lift(beta),
            sense: lift(sense),
        });
        let cache = PcurveCache::certify(
            image,
            lift(0.3),
            lift(4.0),
            &carrier.map_scalar(lift),
            &surface,
            band(),
        )
        .unwrap_or_else(|e| panic!("{what}: {e}"));
        let cert = cache.certificate();
        assert_eq!(cert.samples, 0, "{what}: no schedule at an exact witness");
        assert!(
            cert.envelope.hi() < 1e-11,
            "{what}: {:e}",
            cert.envelope.hi()
        );
    }
}

/// **Each number of the image is load-bearing**: one moved by `64·K·ε`,
/// or the image offered against a torus of another radius, refuses.
#[test]
fn a_wrong_number_in_the_image_refuses() {
    let surface = torus();
    let carrier = villarceau(0.4, 1.0, 1.0, 0.7);
    let image = chart_pcurve(&carrier, &surface, band()).unwrap();
    let Pcurve::FocalSection(FocalImage {
        u0,
        t0,
        v0,
        va,
        vb,
        vl,
        beta,
        sense,
    }) = image
    else {
        unreachable!()
    };
    let h = 64.0 * Tol::witness().k() * eps();
    let wrong = [
        ("u0", [u0 + h, t0, v0, va, vb, vl, beta, sense]),
        ("t0", [u0, t0 + h, v0, va, vb, vl, beta, sense]),
        ("v0", [u0, t0, v0 + h, va, vb, vl, beta, sense]),
        ("va", [u0, t0, v0, va + h, vb, vl, beta, sense]),
        ("vb", [u0, t0, v0, va, vb + h, vl, beta, sense]),
        ("vl", [u0, t0, v0, va, vb, vl + h, beta, sense]),
        ("beta", [u0, t0, v0, va, vb, vl, beta + h, sense]),
        ("sense", [u0, t0, v0, va, vb, vl, beta, -sense]),
    ];
    for (name, [u0, t0, v0, va, vb, vl, beta, sense]) in wrong {
        let bad = Pcurve::FocalSection(FocalImage {
            u0,
            t0,
            v0,
            va,
            vb,
            vl,
            beta,
            sense,
        });
        assert!(
            PcurveCache::certify(bad, 0.3, 4.0, &carrier, &surface, band()).is_err(),
            "{name} moved by {h} still certifies"
        );
    }
    let Surface::Torus {
        center,
        axis,
        major_radius,
        minor_radius,
        u_ref,
    } = surface
    else {
        unreachable!()
    };
    for (major_radius, minor_radius) in [
        (major_radius + h, minor_radius),
        (major_radius, minor_radius + h),
    ] {
        let other = Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        };
        assert!(
            PcurveCache::certify(image.clone(), 0.3, 4.0, &carrier, &other, band()).is_err(),
            "the image certified against another torus ({major_radius}, {minor_radius})"
        );
    }
}

/// **The envelope bounds what it certifies.** An image with one of its
/// numbers moved by a quarter of ε certifies where the envelope reads
/// the move under the band (and refuses, soundly, where it does not),
/// and then the envelope covers the residual sampled densely over the
/// span.
#[test]
fn the_envelope_covers_an_admitted_error() {
    let surface = torus();
    let h = 0.25 * eps();
    let mut compared = Vec::new();
    for (what, carrier) in poses().into_iter().step_by(5) {
        let image = chart_pcurve(&carrier, &surface, band()).unwrap();
        let Pcurve::FocalSection(FocalImage {
            u0,
            t0,
            v0,
            va,
            vb,
            vl,
            beta,
            sense,
        }) = image
        else {
            unreachable!()
        };
        let moved = [
            ("u0", [u0 + h, t0, v0, va, vb, vl, beta, sense]),
            ("t0", [u0, t0 + h, v0, va, vb, vl, beta, sense]),
            ("v0", [u0, t0, v0 + h, va, vb, vl, beta, sense]),
            ("va", [u0, t0, v0, va + h, vb, vl, beta, sense]),
            ("vb", [u0, t0, v0, va, vb + h, vl, beta, sense]),
            ("vl", [u0, t0, v0, va, vb, vl * (1.0 + h), beta, sense]),
            ("beta", [u0, t0, v0, va, vb, vl, beta + h, sense]),
        ];
        for (name, [u0, t0, v0, va, vb, vl, beta, sense]) in moved {
            let image = Pcurve::FocalSection(FocalImage {
                u0,
                t0,
                v0,
                va,
                vb,
                vl,
                beta,
                sense,
            });
            let (a, b) = (0.3, 0.3 + 0.5 * TAU);
            let what = format!("{what}, {name} moved by eps/4");
            let Ok(cache) = PcurveCache::certify(image.clone(), a, b, &carrier, &surface, band())
            else {
                // A move the envelope reads over the band refuses:
                // sound, and no bound to compare.
                continue;
            };
            let sup = sampled_sup(&image, &carrier, a, b);
            assert!(sup > 0.1 * h, "{what}: off its carrier by only {sup:e}");
            let envelope = cache.certificate().envelope;
            assert!(
                envelope >= sup,
                "{what}: envelope {envelope:e} under the sampled residual {sup:e}"
            );
            compared.push(name);
        }
    }
    // The tube-angle offsets are under the band at every pose, so each
    // is compared there rather than refused; the rate's drift grows with
    // the reach, and is under the band at one pose at least.
    for (name, poses) in [("v0", 3), ("va", 3), ("vb", 3), ("vl", 1)] {
        assert!(
            compared.iter().filter(|&&n| n == name).count() >= poses,
            "{name} compared at {poses} poses at least: {compared:?}"
        );
    }
}

/// **A circle the incidence test reads on the torus that is none of its
/// circles grazes it**, and one the test reads off it is off the chart.
/// A circle ⊥ the axis through the tube's crest centred `2·K·ε` off the
/// axis leaves the torus by only the square of that, so it passes the
/// incidence test while the centring gate fails; a Villarceau circle
/// moved `5·K·ε` along the axis is off the torus by that much at its
/// crest; one tilted `1e-3` off its family's plane is off it outright.
#[test]
fn a_grazing_circle_refuses_and_a_moved_villarceau_circle_is_off_the_chart() {
    let surface = torus();
    let Surface::Torus {
        center,
        axis,
        u_ref,
        ..
    } = surface
    else {
        unreachable!()
    };
    let k = Tol::witness().k();
    let cv = axis.cross(u_ref);
    let delta = 2.0 * k * eps();
    let crest = Curve3::Circle {
        center: center + axis * MINOR + u_ref * delta,
        axis,
        radius: MAJOR,
        u_ref: cv,
    };
    assert!(off_torus(&crest) < 0.5 * eps(), "the crest circle grazes");
    let got = chart_pcurve(&crest, &surface, band());
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::CarrierGrazesChart {
                grazer: Grazer::TorusCircle,
                ..
            })
        ),
        "a crest circle centred off the axis grazes the torus: {got:?}"
    );
    let msg = got.unwrap_err().to_string();
    assert!(msg.contains("Recourse: state the crest parallel"), "{msg}");

    let Curve3::Circle {
        center: c,
        axis: n,
        radius,
        u_ref: e,
    } = villarceau(0.4, 1.0, 1.0, 0.0)
    else {
        unreachable!()
    };
    let lifted = Curve3::Circle {
        center: c + axis * (5.0 * k * eps()),
        axis: n,
        radius,
        u_ref: e,
    };
    let rot = |v: Vec3<f64>, by: f64| {
        // Rotate about the circle's own `u_ref` direction.
        v * by.cos() + e.cross(v) * by.sin() + e * (e.dot(v) * (1.0 - by.cos()))
    };
    let tilted = Curve3::Circle {
        center: c,
        axis: rot(n, 1e-3),
        radius,
        u_ref: e,
    };
    for (what, carrier) in [("lifted", lifted), ("tilted", tilted)] {
        let got = chart_pcurve(&carrier, &surface, band());
        assert!(
            matches!(got, Err(PcurveCertifyError::CarrierOffChart { .. })),
            "a {what} Villarceau circle is off the torus: {got:?}"
        );
    }
}

/// **A whole turn of the image is one period on each channel**, signed
/// by `sense` and `vl`: the element (`±1`, `±1`) the loop walk's joint
/// decision reads off a closed Villarceau edge's exit and entry, inside
/// the loop invariant's one period per channel.
#[test]
fn a_whole_turn_advances_each_channel_by_one_period() {
    let surface = torus();
    for (what, carrier) in poses() {
        let image = chart_pcurve(&carrier, &surface, band()).unwrap();
        let Pcurve::FocalSection(FocalImage { vl, sense, .. }) = image else {
            unreachable!()
        };
        for t in [-2.0, 0.0, 1.3] {
            let (a, b) = (image.eval(t), image.eval(t + TAU));
            let turns = ((b.x - a.x) / TAU, (b.y - a.y) / TAU);
            assert!(
                (turns.0 - sense).abs() < 1e-12 && (turns.1 - vl).abs() < 1e-12,
                "{what}, from {t}: a whole turn moves {turns:?}, not ({sense}, {vl})"
            );
        }
    }
}

/// **An in-band move is never read off the torus** (the reviewers'
/// one-sided-incidence probe, PR 4227). A Villarceau circle moved along
/// the torus's axis by under the band's zero half is within the band of
/// the torus everywhere: the incidence test, which decides `Off` only
/// from a sampled lower bound on the distance, does not refuse it, the
/// image derives, and its certificate covers the move. The same circle
/// moved `3·K·ε` is off the torus.
#[test]
fn an_in_band_move_is_never_read_off_the_torus() {
    let k = Tol::witness().k();
    for ratio in [0.1, 0.35, 0.6, 0.9] {
        let (major, minor) = (2.0, 2.0 * ratio);
        let surface = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: major,
            minor_radius: minor,
            u_ref: Vec3::unit_x(),
        };
        let tilt = (minor / major).asin();
        let lean = Vec3::unit_y() * tilt.cos() + Vec3::unit_z() * tilt.sin();
        let moved = |by: f64| Curve3::Circle {
            center: Point3::new(minor, 0.0, by),
            axis: Vec3::unit_x().cross(lean),
            radius: major,
            u_ref: Vec3::unit_x(),
        };
        for fraction in [0.2, 0.5, 0.9] {
            let what = format!("r/R {ratio}, moved {fraction}·ε along the axis");
            let carrier = moved(fraction * eps());
            let image = chart_pcurve(&carrier, &surface, band())
                .unwrap_or_else(|e| panic!("{what}: refused: {e}"));
            let cache = PcurveCache::certify(image.clone(), 0.0, TAU, &carrier, &surface, band())
                .unwrap_or_else(|e| panic!("{what}: the image does not certify: {e}"));
            let mut sup = 0.0_f64;
            for i in 0..=4096 {
                let t = TAU * f64::from(i) / 4096.0;
                let uv = image.eval(t);
                sup = sup.max(surface.eval(uv.x, uv.y).distance(carrier.eval(t)));
            }
            let noise = 64.0 * f64::EPSILON * (major + minor);
            assert!(
                cache.certificate().envelope >= sup - noise,
                "{what}: envelope {:e} under the residual {sup:e}",
                cache.certificate().envelope
            );
        }
        let got = chart_pcurve(&moved(3.0 * k * eps()), &surface, band());
        assert!(
            matches!(got, Err(PcurveCertifyError::CarrierOffChart { .. })),
            "r/R {ratio}, moved 3·K·ε: off the torus: {got:?}"
        );
    }
}

/// **Each Villarceau gate alone refuses typed.** A Villarceau circle
/// moved by `3·K·ε` in one gate's quantity and in no other's — its centre
/// along the centre's radial (offset), along its plane's own direction
/// `n × d̂` (equator, to first order), along the equator's tangent (the
/// plane through the torus centre), its plane turned about `d̂` (tilt), or
/// its radius — is never imaged: each departure is, to first order, that
/// far off the torus, so the incidence test refuses it before the gates
/// read it (a grazer that passes the incidence test fails four gates at
/// once, `a_grazing_circle_refuses_…`).
#[test]
fn each_villarceau_gate_alone_refuses_typed() {
    let surface = torus();
    let Surface::Torus {
        center: tc, axis, ..
    } = surface
    else {
        unreachable!()
    };
    let Curve3::Circle {
        center,
        axis: n,
        radius,
        u_ref,
    } = villarceau(0.4, 1.0, 1.0, 0.0)
    else {
        unreachable!()
    };
    let delta = 3.0 * Tol::witness().k() * eps();
    let d = {
        let w = center - tc;
        (w - axis * w.dot(axis)).normalize()
    };
    let sin_tilt = MINOR / MAJOR;
    let turned = |by: f64| {
        let n2 = (n * by.cos() + d.cross(n) * by.sin()).normalize();
        (n2, (u_ref - n2 * u_ref.dot(n2)).normalize())
    };
    let (n_tilt, u_tilt) = turned(delta / (MAJOR * (1.0 - sin_tilt * sin_tilt).sqrt()));
    let rows = [
        ("offset", center + d * delta, n, radius, u_ref),
        (
            "equator",
            center + n.cross(d) * (delta / sin_tilt),
            n,
            radius,
            u_ref,
        ),
        (
            "plane",
            center + axis.cross(d) * (delta / sin_tilt),
            n,
            radius,
            u_ref,
        ),
        ("tilt", center, n_tilt, radius, u_tilt),
        ("radius", center, n, radius + delta, u_ref),
    ];
    for (gate, center, axis, radius, u_ref) in rows {
        let carrier = Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        };
        let got = chart_pcurve(&carrier, &surface, band());
        assert!(
            matches!(got, Err(PcurveCertifyError::CarrierOffChart { .. })),
            "the {gate} gate failed alone by 3·K·ε: the incidence test refuses it first: {got:?}"
        );
    }
}

/// **The certificate's two tube gates refuse typed.** A stored image
/// whose tube rate is off `±1` by `3·K·ε/r` refuses at check 1
/// (`pcurve_focal_section_tube_rate`), and one whose tube angle sweeps
/// more than a period over a span its azimuth does not refuses at
/// check 2 (`pcurve_tube_period`).
#[test]
fn the_tube_rate_and_tube_period_gates_refuse_typed() {
    let surface = torus();
    let carrier = villarceau(0.4, 1.0, 1.0, 0.0);
    let Ok(Pcurve::FocalSection(image)) = chart_pcurve(&carrier, &surface, band()) else {
        panic!("a Villarceau circle images")
    };
    let rate = FocalImage {
        vl: image.vl * (1.0 + 3.0 * Tol::witness().k() * eps() / MINOR),
        ..image
    };
    let got = PcurveCache::certify(
        Pcurve::FocalSection(rate),
        0.3,
        2.0,
        &carrier,
        &surface,
        band(),
    );
    assert!(
        matches!(got, Err(PcurveCertifyError::ImageMismatch { why, .. }) if why.contains("tube rate")),
        "a tube rate off ±1: {:?}",
        got.err()
    );
    let swept = FocalImage { va: 0.5, ..image };
    let got = PcurveCache::certify(
        Pcurve::FocalSection(swept),
        0.0,
        TAU - 0.1,
        &carrier,
        &surface,
        band(),
    );
    assert!(
        matches!(got, Err(PcurveCertifyError::TubePeriodExceeded)),
        "a tube angle over a period: {:?}",
        got.err()
    );
}
