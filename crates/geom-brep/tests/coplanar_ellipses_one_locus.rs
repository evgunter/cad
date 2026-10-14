//! **Two coplanar ellipses are one curve when they are one locus**, at
//! any rotation of their frames, and two loci apart by more than the
//! band are not ([`geom_brep::Conic::locus_gap`], read by the pose's
//! two-ellipse arm).
//!
//! A frame turned by `δφ` moves a point of the parametrization by about
//! `a·δφ` and the locus by only about `(a − b)·δφ`. The rows build each
//! side two ways the kernel does: one tilted section derived from two
//! cylinder axes a unit ulp apart across the cut (its major direction
//! then turns by about `u/sin θ`), and one ellipse stored in two frames
//! turned in its plane. The distinct rows move a locus by a few times
//! the band's escalation (centre, either semi-axis, a turn of an
//! ordinary ellipse) and must never read one curve; moved by a few `ε`
//! they read distinct or escalate. Every row runs at three named ε with
//! the run's K, independent of the run's ε: the failures these rows pin
//! were measured at all three, and a row per ε of the battery would run
//! each at one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/curve_pair/",
    "crates/geom-brep/src/implicit.rs",
];

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};
use geom::Curve3;
use geom_brep::curve_pair::{MeetFault, Pose, PoseRefusal, Span, pose};
use geom_core::{Band, Point3, Tol, Vec3};

const EPSILONS: [f64; 3] = [1e-6, 1e-9, 1e-12];
const RADII: [f64; 3] = [1e-2, 1.0, 100.0];

fn band_at(eps: f64) -> Band {
    Band::linear_at(Tol::witness(), eps).unwrap()
}

/// A skew plane's normal and an orthonormal pair in it.
fn skew_frame() -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
    let n = Vec3::new(1.0, 2.0, 3.0).normalize();
    let e1 = n.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
    (n, e1, n.cross(e1))
}

fn centre() -> Point3<f64> {
    Point3::new(0.3, -0.7, 1.1)
}

/// The section of the plane through [`centre`] with [`skew_frame`]'s
/// normal by the radius-`r` cylinder through it whose axis is
/// `cos θ·n + sin θ·e₁ + across·e₂`, derived in the plane × cylinder
/// arm's arithmetic (`plane_cylinder_section`): semi-minor `r` along
/// `d̂ × n` normalized, semi-major `r/|d̂·n|` along that × `n`.
fn section(r: f64, tilt: f64, across: f64) -> Curve3<f64> {
    let (n, e1, e2) = skew_frame();
    let (s, c) = tilt.sin_cos();
    let d = (n * c + e1 * s + e2 * across).normalize();
    let sin_vec = d.cross(n);
    let minor = sin_vec / sin_vec.norm();
    Curve3::Ellipse {
        center: centre(),
        axis: n,
        semi_u: r / d.dot(n).abs(),
        semi_v: r,
        u_ref: minor.cross(n),
    }
}

/// The ellipse of semi-axes `a` along `e₁` turned by `turn` in the plane
/// and `b` square to it, stored in the frame turned by a further `frame`
/// (semi-axes kept in place, so the locus turns with the frame).
fn turned(a: f64, b: f64, turn: f64, frame: f64, at: Point3<f64>) -> Curve3<f64> {
    let (n, e1, e2) = skew_frame();
    let (s, c) = (turn + frame).sin_cos();
    Curve3::Ellipse {
        center: at,
        axis: n,
        semi_u: a,
        semi_v: b,
        u_ref: (e1 * c + e2 * s).normalize(),
    }
}

/// Every reading of the pair: the pose either way round over arcs that
/// overlap.
fn read(e: &Curve3<f64>, f: &Curve3<f64>, band: Band) -> [Result<Pose<f64>, PoseRefusal>; 2] {
    let (se, sf) = (Span::new(e, (-1.0, 2.0)), Span::new(f, (-PI, PI)));
    [pose(&se, &sf, band), pose(&sf, &se, band)]
}

fn assert_one(label: &str, e: &Curve3<f64>, f: &Curve3<f64>, band: Band) {
    for (k, got) in read(e, f, band).iter().enumerate() {
        assert!(
            matches!(got, Ok(Pose::Coincident { .. })),
            "{label}, order {k}: one locus reads one curve: {got:?}"
        );
    }
}

fn assert_two(label: &str, e: &Curve3<f64>, f: &Curve3<f64>, band: Band, definite: bool) {
    for (k, got) in read(e, f, band).iter().enumerate() {
        let distinct = matches!(got, Err(PoseRefusal::Closed(MeetFault::Unsupported { .. })));
        let escalated = matches!(got, Err(PoseRefusal::Closed(MeetFault::Escalated(_))));
        assert!(
            distinct || (!definite && escalated),
            "{label}, order {k}: two loci read distinct{}: {got:?}",
            if definite { "" } else { " or escalate" }
        );
    }
}

/// **One section derived twice**, from cylinder axes a unit ulp apart
/// across the cut: near circular (tilt `1e-4`) and ordinary.
#[test]
fn one_section_derived_from_axes_an_ulp_apart_is_one_curve() {
    for eps in EPSILONS {
        let band = band_at(eps);
        for r in RADII {
            for tilt in [1e-4, 1e-2, 0.5, 1.0] {
                let (e, f) = (section(r, tilt, 0.0), section(r, tilt, f64::EPSILON));
                assert_one(&format!("ε {eps:e}, r {r:e}, tilt {tilt:e}"), &e, &f, band);
            }
        }
    }
}

/// **One locus in two frames**: an ellipse within a tenth of the band of
/// round stored in frames turned `1e-3`, `π/4` and `π/2`; an ordinary
/// ellipse (`b = a/2`) with its semi-axes swapped onto the frame turned
/// `π/2`, and upside down; and one of ratio `1.001` turned by `20ε/a`,
/// which moves its parametrization by twice the escalation edge and its
/// locus by a fiftieth of the zero.
#[test]
fn one_ellipse_in_two_frames_is_one_curve() {
    for eps in EPSILONS {
        let band = band_at(eps);
        for r in RADII {
            let at = centre();
            let label = |what: &str| format!("ε {eps:e}, r {r:e}: {what}");
            let b = r - 0.1 * eps;
            for frame in [1e-3, FRAC_PI_4, FRAC_PI_2] {
                let (e, f) = (turned(r, b, 0.3, 0.0, at), turned(r, b, 0.3, frame, at));
                assert_one(
                    &label(&format!("near round, frame {frame:e}")),
                    &e,
                    &f,
                    band,
                );
            }
            let e = turned(r, 0.5 * r, 0.3, 0.0, at);
            let swapped = turned(0.5 * r, r, 0.3, FRAC_PI_2, at);
            assert_one(&label("semi-axes swapped"), &e, &swapped, band);
            let (n, ..) = skew_frame();
            let Curve3::Ellipse { u_ref, .. } = e else {
                unreachable!()
            };
            let upside = Curve3::Ellipse {
                center: at,
                axis: n * -1.0,
                semi_u: r,
                semi_v: 0.5 * r,
                u_ref: u_ref * -1.0,
            };
            assert_one(&label("upside down"), &e, &upside, band);
            let a = 1.001 * r;
            let (e, f) = (
                turned(a, r, 0.3, 0.0, at),
                turned(a, r, 0.3, 20.0 * eps / a, at),
            );
            assert_one(&label("ratio 1.001 turned 20ε/a"), &e, &f, band);
        }
    }
}

/// **Two loci apart**: an ordinary ellipse (`b = a/2`) against itself
/// with its centre moved in the plane, either semi-axis grown, or the
/// whole turned so its locus moves by `d`. At `d` three times the
/// escalation edge it reads distinct; at three times the zero it reads
/// distinct or escalates.
#[test]
fn two_coplanar_ellipses_a_few_band_widths_apart_are_not_one_curve() {
    for eps in EPSILONS {
        let band = band_at(eps);
        let k = Tol::witness().k();
        for r in RADII {
            let at = centre();
            let (_, e1, e2) = skew_frame();
            let (a, b) = (r, 0.5 * r);
            let e = turned(a, b, 0.3, 0.0, at);
            for (d, definite) in [(3.0 * k * eps, true), (3.0 * eps, false)] {
                let label = |what: &str| format!("ε {eps:e}, r {r:e}, d {d:e}: {what}");
                let moved = |by: Vec3<f64>| turned(a, b, 0.3, 0.0, at + by);
                assert_two(
                    &label("centre along e₁"),
                    &e,
                    &moved(e1 * d),
                    band,
                    definite,
                );
                assert_two(
                    &label("centre along e₂"),
                    &e,
                    &moved(e2 * d),
                    band,
                    definite,
                );
                let grown_a = turned(a + d, b, 0.3, 0.0, at);
                assert_two(&label("a grown"), &e, &grown_a, band, definite);
                let grown_b = turned(a, b + d, 0.3, 0.0, at);
                assert_two(&label("b grown"), &e, &grown_b, band, definite);
                let turn = turned(a, b, 0.3, d / (a - b), at);
                assert_two(&label("turned"), &e, &turn, band, definite);
            }
        }
    }
}
