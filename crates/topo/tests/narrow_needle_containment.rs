//! **Point containment reads a needle narrower than `2Kε` right**
//! (`work/flush/a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face.md`).
//! A ray within the band of an edge meets it, half a crossing for each
//! face the edge bounds (D10, Booleans); a face it meets at two edges is
//! a wedge and reads nothing. Without the wedge, the half rule counts
//! such a face once at each edge, and a point outside a turned needle
//! reads `In` (`r1s_witnesses_outside_a_turned_needle_read_outside`, red
//! without it), and pins the rule. `narrow_needles_read_by_point_containment`
//! reads the needles at an enumeration of points about their edges and
//! corners: it pins the reading, not the wedge rule, and stays green
//! without it.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol};
use topo::{Body, SolidContainment};

/// A rotation by `ang` about `axis`.
fn rot(axis: [f64; 3], ang: f64) -> [[f64; 3]; 3] {
    let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
    let (s, c) = ang.sin_cos();
    let k = 1.0 - c;
    [
        [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
        [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
        [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
    ]
}

/// `p` under the pose `(r, t)`.
fn place((r, t): ([[f64; 3]; 3], [f64; 3]), p: [f64; 3]) -> Point3<f64> {
    let at = |i: usize| r[i][0] * p[0] + r[i][1] * p[1] + r[i][2] * p[2] + t[i];
    Point3::new(at(0), at(1), at(2))
}

/// A right prism over `profile` (counter-clockwise), `z ∈ [0, h]`,
/// placed by `pose`.
fn prism(profile: &[(f64, f64)], h: f64, pose: ([[f64; 3]; 3], [f64; 3]), tol: Tol) -> Body<f64> {
    let mut body = Body::<f64>::new();
    let _ = topo::test_support::prism_ops(
        &mut body,
        profile,
        (0.0, h),
        |x, y, z| place(pose, [x, y, z]),
        topo::test_support::FaceGeometry::Certified,
        tol,
    );
    topo::test_support::describe_as_intersections(&mut body, tol);
    body
}

/// The distance from `p` to the segment `a → b`.
fn segment_distance(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let t = (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    (a.0 + t * dx - p.0).hypot(a.1 + t * dy - p.1)
}

/// Whether `p` is inside the convex `profile`.
fn in_convex(p: (f64, f64), profile: &[(f64, f64)]) -> bool {
    (0..profile.len()).all(|i| {
        let (a, b) = (profile[i], profile[(i + 1) % profile.len()]);
        (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0) > 0.0
    })
}

/// Whether `p` is in the prism, and its distance from the boundary.
fn truth(profile: &[(f64, f64)], h: f64, p: [f64; 3]) -> (bool, f64) {
    let xy = (p[0], p[1]);
    let side = (0..profile.len())
        .map(|i| segment_distance(xy, profile[i], profile[(i + 1) % profile.len()]))
        .fold(f64::INFINITY, f64::min);
    let (in_xy, in_z) = (in_convex(xy, profile), p[2] > 0.0 && p[2] < h);
    if in_xy && in_z {
        (true, side.min(p[2]).min(h - p[2]))
    } else {
        let a = if in_xy { 0.0 } else { side };
        let b = if in_z { 0.0 } else { (-p[2]).max(p[2] - h) };
        (false, a.hypot(b))
    }
}

/// **No point about a narrow needle reads wrong**, at every ε: each
/// corner of the needle's section and each side's middle, at its foot and
/// half way up,
/// pushed off along eight diagonals by 2.5Kε and 8Kε, at rest and
/// turned, both ways out. Every point more than 2Kε off the boundary that
/// reads must read its side.
#[test]
fn narrow_needles_read_by_point_containment() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let (e, ke) = (tol.eps(), tol.k() * tol.eps());
    let needles: [(&str, Vec<(f64, f64)>); 2] = [
        (
            "the 12ε square needle",
            vec![
                (0.0, 0.0),
                (12.0 * e, 0.0),
                (12.0 * e, 12.0 * e),
                (0.0, 12.0 * e),
            ],
        ),
        (
            "the trapezoid needle",
            vec![
                (0.0, 0.0),
                (19.0 * e, 0.0),
                (15.0 * e, 14.0 * e),
                (4.0 * e, 14.0 * e),
            ],
        ),
    ];
    // The needle's length and its pose's offset scale with ε, `1e8·ε`
    // (0.1 at ε 1e-9): a needle 12ε across turned off the axes holds its
    // faces' Newell normals to rounding only while its length over its
    // width stays bounded, and one a unit long is no planar prism at
    // ε 1e-12.
    let h = 1e8 * e;
    let poses = [
        ("at rest", (rot([0.0, 0.0, 1.0], 0.0), [0.0; 3])),
        (
            "turned",
            (rot([37.0, -21.0, 13.0], 0.7), [0.3 * h, -0.2 * h, 0.5 * h]),
        ),
    ];
    let diagonals: Vec<[f64; 3]> = (0..8)
        .map(|k| {
            let s = |b: usize| if k & b == 0 { 1.0 } else { -1.0 };
            let r = 3.0_f64.sqrt();
            [s(1) / r, s(2) / r, s(4) / r]
        })
        .collect();
    let mut wrong = Vec::new();
    let mut read = 0;
    for (name, profile) in &needles {
        let n = profile.len();
        let features: Vec<(f64, f64)> = (0..n)
            .flat_map(|i| {
                let (a, b) = (profile[i], profile[(i + 1) % n]);
                [a, (0.5 * (a.0 + b.0), 0.5 * (a.1 + b.1))]
            })
            .collect();
        for &(pose_name, pose) in &poses {
            for inverted in [false, true] {
                let body = prism(profile, h, pose, tol);
                let body = if inverted {
                    body.revert().expect("the needle reverts")
                } else {
                    body
                };
                for &(x, y) in &features {
                    for z in [0.0, 0.5 * h] {
                        for d in &diagonals {
                            for scale in [2.5 * ke, 8.0 * ke] {
                                let p = [x + d[0] * scale, y + d[1] * scale, z + d[2] * scale];
                                let (inside, gap) = truth(profile, h, p);
                                if gap < 2.0 * ke {
                                    continue;
                                }
                                let want = if inside != inverted {
                                    SolidContainment::In
                                } else {
                                    SolidContainment::Out
                                };
                                if let Ok(got @ (SolidContainment::In | SolidContainment::Out)) =
                                    topo::point_in_solid(&body, place(pose, p), band, tol)
                                {
                                    read += 1;
                                    if got != want {
                                        wrong.push(format!(
                                            "{name}, {pose_name}, inside out {inverted}: \
                                             {p:?} reads {got:?}, is {want:?}"
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "{} of {read} read wrong: {wrong:#?}",
        wrong.len()
    );
    assert!(read > 0, "some point read");
}

/// **R1's witnesses** (PR 5422's review R1, `fuzz_point_in_solid_against_truth`
/// on `origin/flush/edge-meeting-ray-review-r1`): three points outside a
/// turned needle, more than 2Kε off its boundary, that read `In` without
/// the wedge. Measured at ε 1e-9, where the points are given; the needles
/// scale with ε and the points do not.
#[test]
fn r1s_witnesses_outside_a_turned_needle_read_outside() {
    let tol = Tol::witness();
    if (tol.eps() - 1e-9).abs() > 1e-15 {
        test_utils::vacuity::stood_down(
            "eps other than 1e-9",
            "the witnesses were measured at 1e-9, in metres",
        );
        return;
    }
    let band = Band::linear(tol).expect("the run's band");
    let e = tol.eps();
    let square = vec![
        (0.0, 0.0),
        (12.0 * e, 0.0),
        (12.0 * e, 12.0 * e),
        (0.0, 12.0 * e),
    ];
    let trapezoid = vec![
        (0.0, 0.0),
        (19.0 * e, 0.0),
        (15.0 * e, 14.0 * e),
        (4.0 * e, 14.0 * e),
    ];
    let turned = (rot([37.0, -21.0, 13.0], 0.7), [0.3, -0.2, 0.5]);
    let tilted = (rot([1.0, 2.0, 3.0], 1.1), [0.1, 0.2, -0.3]);
    for (what, profile, pose, p) in [
        (
            "the 12ε square needle, turned",
            &square,
            turned,
            [
                -4.632_809_437_986_311_5e-8,
                8.828_154_719_988_122e-9,
                6.274_754_525_383_434e-8,
            ],
        ),
        (
            "the trapezoid needle, turned",
            &trapezoid,
            turned,
            [
                -4.259_446_897_067_895e-8,
                1.082_815_471_998_812_2e-8,
                6.274_754_525_383_434e-8,
            ],
        ),
        (
            "the trapezoid needle, tilted",
            &trapezoid,
            tilted,
            [
                -5.712_814_531_145_78e-9,
                4.367_194_692_847_797_6e-8,
                0.999_999_937_693_998_3,
            ],
        ),
    ] {
        let (inside, gap) = truth(profile, 1.0, p);
        assert!(
            !inside && gap > 2.0 * tol.k() * e,
            "{what}: {p:?} is a witness outside"
        );
        let body = prism(profile, 1.0, pose, tol);
        for (inverted, body, outside) in [
            (false, body.clone(), SolidContainment::Out),
            (true, body.revert().expect("reverts"), SolidContainment::In),
        ] {
            let got = topo::point_in_solid(&body, place(pose, p), band, tol);
            assert!(
                matches!(got, Ok(side) if side == outside),
                "{what}, inside out {inverted}: {p:?} reads {got:?}"
            );
        }
    }
}
