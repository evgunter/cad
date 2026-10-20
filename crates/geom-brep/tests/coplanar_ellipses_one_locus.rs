//! **Two coplanar ellipses are one curve when they are one locus**, at
//! any rotation of their frames, and two loci apart by more than the
//! band are not (the pose's one-curve reading, `curve_pair_pose_one_curve`,
//! on the Procrustes bound `pose::one_curve_gap`).
//!
//! A frame turned by `δφ` moves a point of the parametrization by about
//! `a·δφ` and the locus by only about `(a − b)·δφ`. The rows build each
//! side two ways the kernel does: one tilted section derived from two
//! cylinder axes a unit ulp apart across the cut (its major direction
//! then turns by about `u/sin θ`), and one ellipse stored in two frames
//! turned in its plane. The distinct rows move a locus by a few times
//! the band's escalation (centre, either semi-axis, a turn of an
//! ordinary ellipse) and must never read one curve, nor escalate the
//! one-curve reading; moved by a few `ε` they must never read one
//! curve. What a pair past the one-curve reading answers is the
//! coplanar arm's, not these rows'. The rows run at named ε with the
//! run's K, independent of the run's ε: the locus rows at three, as their
//! failures were measured at all three, and the rounding rows at the
//! 1e-12 their witnesses were built for; a row per ε of the battery would
//! run each at one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom-brep/src/curve_pair/",];

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

/// Two loci: never one curve, and, `definite`, the one-curve reading
/// does not escalate either.
fn assert_two(label: &str, e: &Curve3<f64>, f: &Curve3<f64>, band: Band, definite: bool) {
    for (k, got) in read(e, f, band).iter().enumerate() {
        let one = matches!(got, Ok(Pose::Coincident { .. }));
        let one_curve_escalated = matches!(
            got,
            Err(PoseRefusal::Closed(MeetFault::Escalated(d)))
                if d.predicate == Some("curve_pair_pose_one_curve")
        );
        assert!(
            !one && !(definite && one_curve_escalated),
            "{label}, order {k}: two loci are not one curve{}: {got:?}",
            if definite { ", definitely" } else { "" }
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
/// escalation edge the one-curve reading is definite; at three times the
/// zero it is not one curve.
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

/// Two conics from their bits: `centre, normal, u_ref` and the two
/// semi-axes of each, 22 `f64` bit patterns in hex.
fn from_bits(line: &str) -> (Curve3<f64>, Curve3<f64>) {
    let x: Vec<f64> = line
        .split_whitespace()
        .map(|h| f64::from_bits(u64::from_str_radix(h, 16).unwrap()))
        .collect();
    assert_eq!(x.len(), 22, "two conics of eleven numbers: {line}");
    let conic = |x: &[f64]| Curve3::Ellipse {
        center: Point3::new(x[0], x[1], x[2]),
        axis: Vec3::new(x[3], x[4], x[5]),
        u_ref: Vec3::new(x[6], x[7], x[8]),
        semi_u: x[9],
        semi_v: x[10],
    };
    (conic(&x[..11]), conic(&x[11..]))
}

/// Never one curve, at the zero's own ε: the pair's Hausdorff distance
/// is past it.
fn assert_not_one(label: &str, e: &Curve3<f64>, f: &Curve3<f64>, band: Band) {
    for (k, got) in read(e, f, band).iter().enumerate() {
        assert!(
            !matches!(got, Ok(Pose::Coincident { .. })),
            "{label}, order {k}: two loci past the zero are not one curve: {got:?}"
        );
    }
}

/// **The gap's rounding is charged.** Each pair is an ellipse of
/// semi-axes about 100 against itself with `u_ref` one ulp off in one
/// component, chosen so the computed semi-axis vectors agree bit for
/// bit while the exact ones do not, then moved in its plane until the
/// computed gap is just under ε = 1e-12. Their Hausdorff distance
/// (mpmath) is up to `1.0105·ε`: read on the bare gap each is one
/// curve.
#[test]
fn ellipses_whose_computed_frames_agree_past_their_exact_ones_are_not_one_curve() {
    const PAIRS: [&str; 8] = [
        "3f2346ac31b63b00 bf3c2fa15606776c bf4c9bea8b69546d 3fea07585466e9ed bfc7864f8531e30a bfe1a9468c17bae2 bfb2cd38a443b356 bfef27fe7d671bbd 3fcba52b9634056c 405a6d57d8e39cc9 4055931b9ec20ef6 3f2346ac318ce29d bf3c2fa157188502 bf4c9bea8b4aeeff 3fea07585466e9ed bfc7864f8531e30a bfe1a9468c17bae2 bfb2cd38a443b356 bfef27fe7d671bbd 3fcba52b9634056d 405a6d57d8e39cc9 4055931b9ec20ef6",
        "bf37094be99039a6 3f20c57274810b00 3f457fec1935a520 bfe8a8d4b4ba7851 bfac1fdecd669c63 3fe45165cc901ced bfd4c2611c110575 3fec8648f38f0085 bfd44275b624bb16 4059513e4006f2de 405340ab2c08857a bf37094be9eb8689 3f20c5727676da28 3f457fec190917fb bfe8a8d4b4ba7851 bfac1fdecd669c63 3fe45165cc901ced bfd4c2611c110575 3fec8648f38f0086 bfd44275b624bb16 4059513e4006f2de 405340ab2c08857a",
        "bf399adc7a7f4312 bf12a9704450e118 3f40d966bdfba7ea 3fe024283b52ad9b 3fe9ba34788cbeea bfd427dcd0c5a6f2 3fdb70c0b80bf0d9 3fb5580d3752b457 3fecc93b5b0fa59c 4059ae973d14abbf 4045950b7e8c9797 bf399adc7af7f243 bf12a97044aec03f 3f40d966bd7d0db1 3fe024283b52ad9b 3fe9ba34788cbeea bfd427dcd0c5a6f2 3fdb70c0b80bf0d9 3fb5580d3752b456 3fecc93b5b0fa59c 4059ae973d14abbf 4045950b7e8c9797",
        "bf11f5437e3ee158 3f4f69255ff97718 3f42b67eb319a9bc bfef226f6a8e907c bfc8752a28ad2745 3fc09dbb7ebbe8a9 bfcbecec5e4ef714 3fee3ba493839609 bfcf4c96954987d8 40591687b3e40019 404302d60e99675a bf11f5437f3483f0 3f4f6925607e6e67 3f42b67eb2f73fcd bfef226f6a8e907c bfc8752a28ad2745 3fc09dbb7ebbe8a9 bfcbecec5e4ef714 3fee3ba49383960a bfcf4c96954987d8 40591687b3e40019 404302d60e99675a",
        "bf48ebdcf65929f0 3f4b1aadb6483e6c bf2af648158dd580 bfd39eafeb31d66a 3fe7db5cbd00e6d5 3fe2efe6a48bf545 bfed5e376c58169d bfd95f1d2b281694 3f9893c25f9d1e9b 4059861f5731f920 404c9241ca1374b0 bf48ebdcf5d80079 3f4b1aadb6800954 bf2af648159b586e bfd39eafeb31d66a 3fe7db5cbd00e6d5 3fe2efe6a48bf545 bfed5e376c58169d bfd95f1d2b281694 3f9893c25f9d1e9a 4059861f5731f920 404c9241ca1374b0",
        "3f46fe138fc0acc8 bf0747a8a3469c70 3f149d9a26a11ca0 bfca1ba89da3e732 bfeee92ba96f7fb3 bfc45d467ad76e57 bfd7407b4fc7e241 3fcce05b1d4f65e7 bfeced1823f27eb0 405d3a34ee144569 4047ce01c82e8d43 3f46fe138f8d8b23 bf0747a8a14a9cbd 3f149d9a22a75d08 bfca1ba89da3e732 bfeee92ba96f7fb3 bfc45d467ad76e57 bfd7407b4fc7e241 3fcce05b1d4f65e8 bfeced1823f27eb0 405d3a34ee144569 4047ce01c82e8d43",
        "bef992054080cd00 bf4ce2ba2049d840 bf24844f0c70ac20 3fb0610d6327ee18 bfe71ff5b97329c1 3fe6061ad2158e23 bfe659647594be73 3fdd73d1f43704b5 3fe18a3f0688df9b 405a69c1be055b4a 405584fe7e2f4538 bef992054cca2b66 bf4ce2ba20091403 bf24844f0b3c1ae1 3fb0610d6327ee18 bfe71ff5b97329c1 3fe6061ad2158e23 bfe659647594be73 3fdd73d1f43704b6 3fe18a3f0688df9b 405a69c1be055b4a 405584fe7e2f4538",
        "3f253978e49120b0 bf4c35c55c563f77 3f30fe5037ad22c4 3fef6513213b6ed7 bfc11378eb9d0cc5 bfc1f388a8e745d4 3fa656ebc57b71d1 bfe1b6156d61279e 3fea9d74e4546842 405aca01697a0a6c 4053810faa29a473 3f253978e4a9b0b5 bf4c35c55ca42493 3f30fe5038973e90 3fef6513213b6ed7 bfc11378eb9d0cc5 bfc1f388a8e745d4 3fa656ebc57b71d2 bfe1b6156d61279e 3fea9d74e4546842 405aca01697a0a6c 4053810faa29a473",
    ];
    let band = band_at(1e-12);
    for (k, line) in PAIRS.iter().enumerate() {
        let (e, f) = from_bits(line);
        assert_not_one(&format!("pair {k}"), &e, &f, band);
    }
}

/// **An ellipse a hair off round against a circle**: semi-axes
/// `100 + 1e-12` and `100` in the `xy` plane with `u_ref` one ulp off
/// unit, `(0.6⁺, 0.8)`, which `v̂ = n̂ × û` repeats, against the circle of
/// radius `100` on its centre. The ellipse's radius runs from `100·|û|`
/// to `(100 + 1e-12)·|û|`, so the pair's Hausdorff distance is exactly
/// `1.0036e-12`, past ε = 1e-12: read on the bare gap it is one curve.
#[test]
fn an_ellipse_with_its_frame_an_ulp_off_unit_past_the_zero_of_a_circle_is_not_one_curve() {
    let band = band_at(1e-12);
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 100.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for (a, b) in [(100.0 + 1e-12, 100.0), (100.0, 100.0 + 1e-12)] {
        let e = Curve3::Ellipse {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(0.6f64.next_up(), 0.8, 0.0),
            semi_u: a,
            semi_v: b,
        };
        assert_not_one(&format!("a {a}, b {b}"), &e, &circle, band);
    }
}
