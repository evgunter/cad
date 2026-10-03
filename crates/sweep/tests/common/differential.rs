//! **The differential batteries' shared pieces**: the polygon oracles
//! their closed-form volumes are derived from, the one `outcome` line a
//! battery prints per pose, and the reflex-corner pose. Derived without
//! the kernel, as [`super::oracles`] is, and printed in one format, so
//! two batteries' lines can be diffed against each other.
//!
//! **Deliberately not absorbed**, and the whole of it:
//! [`super::oracles`]' closed forms, which are swept-solid volumes with
//! nothing polygonal to share; and the per-battery operands of
//! `join1_r1_probes.rs` (prisms, rods, revolves), each one battery's.

use geom_core::{Point3, Tol};
use topo::test_support::{
    FaceGeometry, describe_as_intersections, flush_declarations, prism_ops, prism_z,
};
use topo::{Body, BooleanDeclarations, BooleanError, BooleanResult};

/// The signed area of a simple polygon (positive counter-clockwise).
pub fn area(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    (0..n)
        .map(|i| {
            let (a, b) = (p[i], p[(i + 1) % n]);
            a.0 * b.1 - a.1 * b.0
        })
        .sum::<f64>()
        / 2.0
}

/// `p` wound counter-clockwise.
pub fn ccw(mut p: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    if area(&p) < 0.0 {
        p.reverse();
    }
    p
}

/// `∫ x dA`, `∫ y dA` over a simple polygon (signed by its winding).
pub fn moments(p: &[(f64, f64)]) -> (f64, f64) {
    let n = p.len();
    let (mut mx, mut my) = (0.0, 0.0);
    for i in 0..n {
        let (a, b) = (p[i], p[(i + 1) % n]);
        let c = a.0 * b.1 - b.0 * a.1;
        mx += (a.0 + b.0) * c;
        my += (a.1 + b.1) * c;
    }
    (mx / 6.0, my / 6.0)
}

/// Sutherland–Hodgman of `poly` against the axis rectangle `x × y`.
pub fn clip_rect(poly: &[(f64, f64)], x: (f64, f64), y: (f64, f64)) -> Vec<(f64, f64)> {
    let rect = [(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)];
    clip_convex(poly, &rect)
}

/// Sutherland–Hodgman of `poly` against the convex CCW polygon `clipper`.
pub fn clip_convex(poly: &[(f64, f64)], clipper: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = poly.to_vec();
    let m = clipper.len();
    for k in 0..m {
        let (a, b) = (clipper[k], clipper[(k + 1) % m]);
        let side = |p: (f64, f64)| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        let inp = std::mem::take(&mut out);
        let n = inp.len();
        for i in 0..n {
            let (s, e) = (inp[i], inp[(i + 1) % n]);
            let (ds, de) = (side(s), side(e));
            let isect = || {
                let t = ds / (ds - de);
                (s.0 + t * (e.0 - s.0), s.1 + t * (e.1 - s.1))
            };
            match (ds >= 0.0, de >= 0.0) {
                (true, true) => out.push(e),
                (true, false) => out.push(isect()),
                (false, true) => {
                    out.push(isect());
                    out.push(e);
                }
                (false, false) => {}
            }
        }
    }
    out
}

/// **One pose's line**: the refusal, or the body's tiers 2 and 3′, the
/// at-rest certificate, whether it is a legal operand (it unites with a
/// far brick, `sweep::test_support::assert_legal_operand`'s question)
/// and its volume against `want`. `SOUND` is all five; anything else
/// that builds is `BAD`.
pub fn outcome(r: Result<BooleanResult<f64>, BooleanError>, want: f64, tol: Tol) -> String {
    match r {
        Err(e) => {
            let s = format!("{e:?}");
            let cut: String = s.chars().take(110).collect();
            format!("ERR {cut}")
        }
        Ok(r) => match r.body() {
            None if want.abs() < 1e-9 => "EMPTY ok".into(),
            None => format!("EMPTY WRONG want={want}"),
            Some(bb) => {
                let t2 = topo::validate_closed(&bb.body).is_ok();
                let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol).is_ok();
                let cert = topo::validate_geometric_certificate(&bb.body, tol).is_ok();
                let far = sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol);
                let operand = topo::union(&bb.body, &far, tol).is_ok();
                match topo::mass_properties(&bb.body, tol).map(|m| m.volume) {
                    Ok(v) => {
                        let good = (v - want).abs() < 1e-7;
                        format!(
                            "OK {} t2={t2} t3p={t3} cert={cert} operand={operand} v={v:.9} \
                             want={want:.9}",
                            if good && t2 && t3 && cert && operand {
                                "SOUND"
                            } else {
                                "BAD"
                            }
                        )
                    }
                    Err(e) => format!(
                        "OK-UNMEASURED t2={t2} t3p={t3} cert={cert} operand={operand} {e:?}"
                    ),
                }
            }
        },
    }
}

/// The reflex corner's `a` profile: the 315° corner at the origin, all
/// of `[−2, 2]²` but the wedge `0 ≤ y ≤ x`.
pub const REFLEX_A: [(f64, f64); 6] = [
    (0.0, 0.0),
    (2.0, 2.0),
    (-2.0, 2.0),
    (-2.0, -2.0),
    (2.0, -2.0),
    (2.0, 0.0),
];

/// The reflex probe's twelve `b` profiles, by name: a vertex or an
/// edge of each on the corner.
pub const REFLEX_PROFILES: [(&str, [(f64, f64); 4]); 12] = [
    ("sqQ1", [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]),
    ("sqQ2", [(-1.0, 0.0), (0.0, 0.0), (0.0, 1.0), (-1.0, 1.0)]),
    ("sqQ3", [(-1.0, -1.0), (0.0, -1.0), (0.0, 0.0), (-1.0, 0.0)]),
    ("sqQ4", [(0.0, -1.0), (1.0, -1.0), (1.0, 0.0), (0.0, 0.0)]),
    ("dUp", [(0.0, 0.0), (0.5, 0.5), (0.0, 1.0), (-0.5, 0.5)]),
    (
        "dLeft",
        [(0.0, 0.0), (-0.5, 0.5), (-1.0, 0.0), (-0.5, -0.5)],
    ),
    (
        "dDown",
        [(0.0, 0.0), (-0.5, -0.5), (0.0, -1.0), (0.5, -0.5)],
    ),
    ("dRight", [(0.0, 0.0), (0.5, -0.5), (1.0, 0.0), (0.5, 0.5)]),
    ("eBot", [(-0.5, 0.0), (0.5, 0.0), (0.5, 1.0), (-0.5, 1.0)]),
    ("eTop", [(-0.5, -1.0), (0.5, -1.0), (0.5, 0.0), (-0.5, 0.0)]),
    ("eLeft", [(0.0, -0.5), (1.0, -0.5), (1.0, 0.5), (0.0, 0.5)]),
    (
        "eRight",
        [(-1.0, -0.5), (0.0, -0.5), (0.0, 0.5), (-1.0, 0.5)],
    ),
];

/// The reflex probe's four ops, in the order [`ReflexPose::want`]
/// holds their volumes.
pub const REFLEX_OPS: [&str; 4] = ["I", "U", "S_ab", "S_ba"];

/// One reflex pose: `a` the 315° prism `prism_z(REFLEX_A)`,
/// z ∈ [0, 1]; `b` a `prism_ops` prism over z ∈ (1, 3) sheared
/// `z' = z + sx·x + sy·y`, so its bottom cap passes through `a`'s
/// corner `(0, 0, 1)`; their flush declarations (for the `(a, b)`
/// order); and the closed-form volumes of [`REFLEX_OPS`].
pub struct ReflexPose {
    pub a: Body<f64>,
    pub b: Body<f64>,
    pub d: BooleanDeclarations,
    pub want: [f64; 4],
}

/// The pose of the named profile, turned `rot` degrees about the
/// corner, at shear `(sx, sy)` (not both zero).
pub fn reflex_pose(profile: &str, rot: f64, sx: f64, sy: f64, tol: Tol) -> ReflexPose {
    let a_prof = ccw(REFLEX_A.to_vec());
    let a = prism_z::<f64>(&a_prof, 0.0, 1.0, tol).body;
    let (c, s) = (rot.to_radians().cos(), rot.to_radians().sin());
    let prof = ccw(REFLEX_PROFILES
        .iter()
        .find(|(n, _)| *n == profile)
        .expect("a probe profile")
        .1
        .iter()
        .map(|&(x, y)| (c * x - s * y, s * x + c * y))
        .collect());
    let mut b = Body::<f64>::new();
    prism_ops(
        &mut b,
        &prof,
        (1.0, 3.0),
        |x, y, z| Point3::new(x, y, z + sx * x + sy * y),
        FaceGeometry::Certified,
        tol,
    );
    describe_as_intersections(&mut b, tol);
    // The overlap's height over (x, y) in both profiles is
    // min(1, max(0, −L)), L = sx·x + sy·y (b's cap may pass below a's
    // floor): ∫(0 − L)⁺ − ∫(−1 − L)⁺.
    let both = clip_convex(&a_prof, &prof);
    let len = (sx * sx + sy * sy).sqrt();
    let (ux, uy) = (-sy / len, sx / len);
    let (ix, iy) = (-sx / len, -sy / len);
    let big = 100.0;
    let below = |lvl: f64| {
        // ∫ over both ∩ {L < lvl} of (lvl − L).
        let (ox, oy) = (sx * lvl / (len * len), sy * lvl / (len * len));
        let half = ccw(vec![
            (ox + ux * big, oy + uy * big),
            (ox - ux * big, oy - uy * big),
            (ox - ux * big + ix * big, oy - uy * big + iy * big),
            (ox + ux * big + ix * big, oy + uy * big + iy * big),
        ]);
        let low = clip_convex(&both, &half);
        if low.len() < 3 {
            return 0.0;
        }
        let (mx, my) = moments(&low);
        lvl * area(&low) - (sx * mx + sy * my)
    };
    let vi = below(0.0) - below(-1.0);
    let (va, vb) = (area(&a_prof), area(&prof) * 2.0);
    let d = flush_declarations(&a, &b, tol);
    ReflexPose {
        a,
        b,
        d,
        want: [vi, va + vb - vi, va - vi, vb - vi],
    }
}

/// One of [`REFLEX_OPS`] on a pose.
pub fn reflex_run(p: &ReflexPose, op: &str, tol: Tol) -> Result<BooleanResult<f64>, BooleanError> {
    match op {
        "I" => topo::intersect_with(&p.a, &p.b, &p.d, tol),
        "U" => topo::union_with(&p.a, &p.b, &p.d, tol),
        "S_ab" => topo::subtract_with(&p.a, &p.b, &p.d, tol),
        _ => topo::subtract_with(&p.b, &p.a, &p.d, tol),
    }
}
