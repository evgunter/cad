//! The pose's one-curve reading ([`super::pose::one_curve`]): two conics
//! the band reads in one plane are one curve only where each lies within
//! the band of the other, a tilt out of the plane counted like any other
//! move, and [`meet`] reads it alike on every conic pair.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::Curve3;
use geom_core::{Band, Point3, Tol, Vec3};

use super::meet_rows::{circle, ellipse};
use super::pose::{one_curve, one_curve_gap};
use super::{MeetFault, MeetRefusal, Meetings, Span, meet};
use crate::Conic;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The meetings of two carriers over most of a turn each.
fn met(a: &Curve3<f64>, b: &Curve3<f64>, band: Band) -> Result<Meetings<f64>, MeetRefusal<f64>> {
    meet(&Span::new(a, (-3.0, 3.0)), &Span::new(b, (-3.0, 3.0)), band)
}

/// **Two sections read coplanar are one curve only where they are.**
/// The unit cylinder cut by `z = m·x` and `z = m(1 + d)·x`, `m = 100`:
/// the two ellipses cross at `(0, ±1, 0)`. With `d` set so the planes'
/// tilt carries `k·z` across the larger semi-axis (`z` the band's zero,
/// `k` = 0.1, 0.5, 0.9), the band reads the planes parallel and through
/// one point, while the larger semi-axes differ by about `m·k·z`. That is
/// no certified absence of meetings, nor two certified crossings: one's
/// level on the other stays within the band over long arcs about the
/// crossings, so the coplanar subdivision refuses typed (`Unsettled`), or
/// the reading escalates. Red where the pair is decided.
#[test]
fn two_sections_read_coplanar_in_band_are_one_curve_only_where_they_are() {
    let b = band();
    let z = b.zero();
    let m = 100.0f64;
    let section = |slope: f64| {
        ellipse(
            [0.0; 3],
            [-slope, 0.0, 1.0],
            [1.0, 0.0, slope],
            slope.hypot(1.0),
            1.0,
        )
    };
    let tilt_reach = |d: f64| {
        let (e, f) = (section(m), section(m * (1.0 + d)));
        let (
            Curve3::Ellipse { axis: n1, .. },
            Curve3::Ellipse {
                axis: n2, semi_u, ..
            },
        ) = (e, f)
        else {
            unreachable!("two ellipses")
        };
        n1.cross(n2).norm() * semi_u
    };
    for k in [0.1, 0.5, 0.9] {
        let d = k * z * (k * z / tilt_reach(k * z));
        assert!(
            (tilt_reach(d) / z - k).abs() < 1e-3,
            "k = {k}: the tilt carries {} z",
            tilt_reach(d) / z
        );
        let (e, f) = (section(m), section(m * (1.0 + d)));
        for (x, y, order) in [(&e, &f, "e × f"), (&f, &e, "f × e")] {
            let got = met(x, y, b);
            let fault = got.as_ref().err().map(|r| r.fault);
            let two = matches!(fault, Some(MeetFault::Unsettled(_)));
            let escalated = matches!(fault, Some(MeetFault::Escalated(_) | MeetFault::Touch(_)));
            assert!(
                two || escalated,
                "k = {k}, {order}: two crossing sections are not one curve: {got:?}"
            );
        }
    }
}

/// **Two ellipses are one curve only where each lies within the band
/// of the other**, a tilt out of the plane counted like any other
/// move. An ellipse in a skew plane, of semi-axis ratio 1 to 100 at
/// two sizes, against itself moved so that its farthest point leaves
/// it by `d`: its centre within the plane and off it, each semi-axis,
/// a turn in the plane (`d = (a − b)·φ`), and a tilt about either
/// axis and the diagonal (`d` = the tilt times the extent across the
/// hinge). Each is stored as built, with its semi-axes in the other
/// order, and upside down. With `z` the band's zero, `d = 0.6·z` is
/// one curve, `1.5·z` is not, and twice the escalation edge is
/// definitely two, read the same in either order, the gap itself
/// bit for bit. A circle turned in its plane is itself.
#[test]
fn two_ellipses_are_one_curve_only_within_the_band_of_each_other() {
    let b = band();
    let z = b.zero();
    let turn = |x: Vec3<f64>, k: Vec3<f64>, t: f64| {
        let (s, c) = t.sin_cos();
        x * c + k.cross(x) * s + k * (k.dot(x) * (1.0 - c))
    };
    let n = Vec3::new(1.0, 2.0, 3.0).normalize();
    let u = n.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
    let v = n.cross(u);
    let read = |e: &Conic<f64>, f: &Conic<f64>| {
        assert_eq!(
            one_curve_gap(e, f).to_bits(),
            one_curve_gap(f, e).to_bits(),
            "the gap is the same either way round: {e:?} {f:?}"
        );
        let (ef, fe) = (one_curve(e, f, b), one_curve(f, e, b));
        let key = |r: &Result<bool, MeetFault>| match r {
            Ok(one) => Some(*one),
            Err(MeetFault::Escalated(_)) => None,
            Err(other) => panic!("only a band reading refuses: {other:?}"),
        };
        assert_eq!(key(&ef), key(&fe), "either order: {e:?} {f:?}");
        key(&ef)
    };
    for ratio in [1.0, 1.001, 2.0, 10.0, 100.0] {
        for size in [0.01, 1.0] {
            let (major, minor) = (ratio * size, size);
            let e = Conic {
                center: Point3::new(1.0, -2.0, 0.5),
                axis: n,
                u_ref: u,
                semi_u: major,
                semi_v: minor,
            };
            let tilt = |hinge: Vec3<f64>, d: f64| {
                let across = n.cross(hinge);
                let reach = (major * across.dot(u)).hypot(minor * across.dot(v));
                Conic {
                    axis: turn(n, hinge, d / reach),
                    u_ref: turn(u, hinge, d / reach),
                    ..e
                }
            };
            let pose = |what: &str, d: f64| match what {
                "centre in the plane" => Conic {
                    center: e.center + (u + v).normalize() * d,
                    ..e
                },
                "centre off the plane" => Conic {
                    center: e.center + n * d,
                    ..e
                },
                "major" => Conic {
                    semi_u: major + d,
                    ..e
                },
                "minor" => Conic {
                    semi_v: minor - d,
                    ..e
                },
                "tilt about û" => tilt(u, d),
                "tilt about v̂" => tilt(v, d),
                "tilt about û + v̂" => tilt((u + v).normalize(), d),
                "turn" => Conic {
                    u_ref: turn(u, n, d / (major - minor)),
                    ..e
                },
                other => panic!("no pose {other}"),
            };
            let mut poses = vec![
                "centre in the plane",
                "centre off the plane",
                "major",
                "minor",
                "tilt about û",
                "tilt about v̂",
                "tilt about û + v̂",
            ];
            if ratio > 1.1 {
                poses.push("turn");
            }
            for what in poses {
                for (d, want) in [
                    (0.6 * z, Some(true)),
                    (1.5 * z, None),
                    (2.0 * b.escalate(), Some(false)),
                ] {
                    let f = pose(what, d);
                    let stored = [
                        f,
                        Conic {
                            u_ref: f.v_ref(),
                            semi_u: f.semi_v,
                            semi_v: f.semi_u,
                            ..f
                        },
                        Conic { axis: -f.axis, ..f },
                    ];
                    for g in stored {
                        let got = read(&e, &g);
                        let fits = match want {
                            None => got != Some(true),
                            _ => got == want,
                        };
                        assert!(
                            fits,
                            "a/b = {ratio}, b = {size}, {what} by {} z: {got:?}",
                            d / z
                        );
                    }
                }
            }
            if ratio == 1.0 {
                for angle in [0.3, 2.0] {
                    let f = Conic {
                        u_ref: turn(u, n, angle),
                        ..e
                    };
                    assert_eq!(read(&e, &f), Some(true), "a circle turned {angle}");
                }
            }
        }
    }
}

/// **Two of a sphere's circles the band reads coplanar are one only
/// within it.** The unit sphere's great circle in `z = 0`, against
/// its section by that plane turned `θ = k·z` about the line
/// `x = 0.9` (`z` the band's zero): the two cross at
/// `(0.9, ±√0.19, 0)` and lie about `1.9·θ` apart at `x = −1`. The
/// planes' tilt (`0.9·θ` over the radius), the section's offset from
/// `z = 0` and its centre's from the origin (each about `0.81·θ`) all
/// read zero at `k = 0.9`, yet the circles are in band of one
/// another, and the pair escalates; at `k = 0.2` they are one within
/// the band. Between the crossings the great circle dips at most `0.1·θ`
/// below the section's plane, so at `k = 30` the two stay within the
/// band's gap of each other over that whole arc and escalate too. At
/// `θ = 0.2`, past the basin floor at every ε row, they cross
/// definitely, and meet at both crossings.
#[test]
fn two_sphere_sections_read_coplanar_are_one_circle_only_within_the_band() {
    let b = band();
    let z = b.zero();
    let great = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
    let section = |theta: f64| {
        let n = Vec3::new(-theta.sin(), 0.0, theta.cos());
        let centre = n * (-0.9 * theta.sin());
        circle(
            centre.to_array(),
            n.to_array(),
            (1.0 - (0.9 * theta.sin()).powi(2)).sqrt(),
        )
    };
    for (x, y) in [(&great, &section(0.9 * z)), (&section(0.9 * z), &great)] {
        let got = met(x, y, b);
        assert!(
            matches!(got, Err(ref r) if matches!(r.fault, MeetFault::Escalated(_))),
            "k = 0.9: in band of one circle: {got:?}"
        );
    }
    let got = met(&great, &section(0.2 * z), b).unwrap();
    assert!(got.shared.is_some(), "k = 0.2: one circle: {got:?}");
    let got = met(&great, &section(30.0 * z), b);
    assert!(
        matches!(got, Err(ref r) if matches!(r.fault, MeetFault::Escalated(_))),
        "k = 30: in band between the crossings: {got:?}"
    );
    let m = met(&great, &section(0.2), b).unwrap();
    let h = 0.19f64.sqrt();
    for w in [[0.9, h, 0.0], [0.9, -h, 0.0]] {
        assert!(
            m.points
                .iter()
                .any(|p| p.point.distance(Point3::from_array(w)) < 1e-6),
            "θ = 0.2: {w:?} in {m:?}"
        );
    }
}

/// **A conic read twice is one curve at interval too.** A circle and an
/// ellipse in a plane turned off every axis, each paired with itself
/// and with itself re-seamed a third of a turn on (the circle) or half a
/// turn on (the ellipse, its `u_ref` reversed), every coordinate an
/// interval a few ulps wide. The frames' differences enclose zero, so
/// their self-dots, read as interval products, reach below it; the
/// one-curve margin squares them under a root and must still certify
/// zero. Red where that root reads a straddling enclosure, which makes
/// the margin `Invalid` and the census escalate on a body's coincident
/// rims (editor-core's `boss_union` at `Interval`).
#[test]
fn a_conic_read_twice_is_one_curve_at_interval() {
    use geom_core::Interval;
    let b = band();
    let ulps = |x: f64| {
        Interval::from_bounds(
            x - 4.0 * f64::EPSILON * x.abs(),
            x + 4.0 * f64::EPSILON * x.abs(),
        )
    };
    let iv = |v: Vec3<f64>| Vec3::new(ulps(v.x), ulps(v.y), ulps(v.z));
    let n = Vec3::new(1.0, 2.0, 3.0).normalize();
    let u = n.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
    let v = n.cross(u);
    let conic = |u_ref: Vec3<f64>, (a, c): (f64, f64)| Conic {
        center: Point3::new(ulps(1.0), ulps(-2.0), ulps(0.5)),
        axis: iv(n),
        u_ref: iv(u_ref),
        semi_u: ulps(a),
        semi_v: ulps(c),
    };
    let third = u * (2.0 * std::f64::consts::FRAC_PI_3).cos()
        + v * (2.0 * std::f64::consts::FRAC_PI_3).sin();
    for (what, e, f) in [
        (
            "a circle and itself",
            conic(u, (0.7, 0.7)),
            conic(u, (0.7, 0.7)),
        ),
        (
            "a circle re-seamed",
            conic(u, (0.7, 0.7)),
            conic(third, (0.7, 0.7)),
        ),
        (
            "an ellipse and itself",
            conic(u, (1.3, 0.4)),
            conic(u, (1.3, 0.4)),
        ),
        (
            "an ellipse re-seamed",
            conic(u, (1.3, 0.4)),
            conic(-u, (1.3, 0.4)),
        ),
    ] {
        for (order, got) in [
            ("e, f", one_curve(&e, &f, b)),
            ("f, e", one_curve(&f, &e, b)),
        ] {
            assert!(
                matches!(got, Ok(true)),
                "{what} ({order}): one curve at interval: {got:?}"
            );
        }
    }
}

/// **A near-circle at radius `10³` past the zero is not one curve.**
/// Semi-axes `1000` and `1000 − 3e-11`, re-framed and moved: the gap
/// read bare is `9.69e-13`, under ε = 1e-12, while the pair's Hausdorff
/// distance (mpmath) is `1.071e-12`. Charged, the reading escalates.
/// (The pose itself escalates earlier, on the planes' offset; this row
/// holds the one-curve reading, which every caller of the pose shares.)
#[test]
fn a_near_circle_at_radius_a_thousand_past_the_zero_is_not_one_curve() {
    let bits = "3fb999999999999c 3fc9999999999998 3fd3333333333332 3fef06032db7307e bfcef0485ef0c38b 3fa4f97e65b8a6ac 3fc3612ad532465a 3fdd5e6d2519bc63 bfec0416b5d5c693 408f3fffffffffff 408f3fffffffe0c0 3fb999999999c92d 3fc999999999f9d1 3fd3333333333585 3fef06032db73081 bfcef0485ef0c38c 3fa4f97e65b8a6b5 3fc3612ad5324668 3fdd5e6d2519bc73 bfec0416b5d5c68f 408f3ffffffffffd 408f3fffffffe0c0";
    let x: Vec<f64> = bits
        .split_whitespace()
        .map(|h| f64::from_bits(u64::from_str_radix(h, 16).unwrap()))
        .collect();
    let conic = |x: &[f64]| Conic {
        center: Point3::new(x[0], x[1], x[2]),
        axis: Vec3::new(x[3], x[4], x[5]),
        u_ref: Vec3::new(x[6], x[7], x[8]),
        semi_u: x[9],
        semi_v: x[10],
    };
    let (e, f) = (conic(&x[..11]), conic(&x[11..]));
    let b = Band::linear_at(Tol::witness(), 1e-12).unwrap();
    for (order, got) in [
        ("e, f", one_curve(&e, &f, b)),
        ("f, e", one_curve(&f, &e, b)),
    ] {
        assert!(
            !matches!(got, Ok(true)),
            "{order}: a pair 1.071e-12 apart is not one curve at 1e-12: {got:?}"
        );
    }
}
