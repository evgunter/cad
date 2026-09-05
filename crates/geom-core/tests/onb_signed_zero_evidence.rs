//! Does a signed zero survive the interval backend? — the fact that
//! chooses between the two `Interval`-only narrowings of
//! `Vec3::orthonormal_basis`.
//!
//! `orthonormal_basis` opens with `s = T::one().copysign(self.z)`. At
//! `Interval`, `copysign`'s zero-containing arm returns the two-sided
//! hull, so every wall whose normal has `n.z = 0` stores a sign-hulled
//! `u_ref`. One proposal narrows that arm at a POINT enclosure of zero
//! by transferring the zero's sign BIT; it is sound only if the bit is
//! actually there — through construction, through the arithmetic that
//! mints a normal, and through `normalize`.
//!
//! So the bit is followed, not assumed. `#[ignore]`d: these assert
//! nothing and gate nothing; they print, and the printed table is the
//! measurement. The corpus is written down as literals rather than
//! described.
//!
//! ```text
//! cargo test -p geom-core --features interval --test all \
//!     -- --ignored --nocapture onb_signed_zero_evidence
//! ```

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Interval, Point3, Real, Vec3};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

/// How a zero prints when the sign bit is the whole question.
fn bit(x: f64) -> String {
    if x == 0.0 {
        (if x.is_sign_negative() { "-0.0" } else { "+0.0" }).to_string()
    } else {
        format!("{x:e}")
    }
}

fn row(op: &str, f: f64, e: Interval) {
    println!(
        "| {op} | {} | {} | {} |",
        bit(f),
        bit(e.lo()),
        bit(e.hi())
    );
}

/// THE CORPUS for the cross-sum replay, written down: four vertical
/// walls of the unit cube (outward rings, `n.z = 0` by construction)
/// and two walls of a regular hexagonal prism, one of them the
/// y-aligned wall whose `n.x = 0` — the case the hull leaves exact.
const CUBE_WALLS: [[[f64; 3]; 4]; 4] = [
    // +x wall, then +y, -x, -y; each ring wound outward.
    [[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]],
    [[1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0]],
    [[0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]],
    [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
];

/// Two hexagonal-prism walls: `[0]` has an oblique normal, `[1]` the
/// y-aligned normal `(0, -1, 0)` (`n.x = 0`).
const HEX_WALLS: [[[f64; 3]; 4]; 2] = [
    [[1.0, 0.0, 0.0], [0.5, 0.75, 0.0], [0.5, 0.75, 2.0], [1.0, 0.0, 2.0]],
    [[0.5, -0.75, 0.0], [-0.5, -0.75, 0.0], [-0.5, -0.75, 2.0], [0.5, -0.75, 2.0]],
];

/// The translate-to-origin Newell cross-sum, the exact spelling of
/// `geom_brep::newell::newell_plane`'s loop, replayed generically so
/// the same literal ring runs at `f64` and at `Interval`.
fn newell_normal_sum<T: Real>(ring: &[[f64; 3]; 4]) -> Vec3<T> {
    let points: Vec<Point3<T>> = ring
        .iter()
        .map(|p| Point3::new(T::from_f64(p[0]), T::from_f64(p[1]), T::from_f64(p[2])))
        .collect();
    let n = T::from_f64(points.len() as f64);
    let mut sum = Vec3::zero();
    for p in &points {
        sum = sum + (*p - Point3::origin());
    }
    let centroid = Point3::origin() + sum / n;
    let mut normal_sum = Vec3::zero();
    for (i, p) in points.iter().enumerate() {
        let next = points[(i + 1) % points.len()];
        normal_sum = normal_sum + (*p - centroid).cross(next - centroid);
    }
    normal_sum
}

/// **Table 1a — the sign bit through construction and arithmetic.**
#[test]
#[ignore = "signed-zero evidence instrument; run explicitly"]
fn signed_zero_through_the_interval_backend() {
    println!("| op | f64 result | Interval lo | Interval hi |");
    println!("| --- | --- | --- | --- |");
    row("from_f64(+0.0)", 0.0, iv(0.0));
    row("from_f64(-0.0)", -0.0, iv(-0.0));
    row("(-1) * [+0,+0]", -1.0 * 0.0, iv(-1.0) * iv(0.0));
    row("(-1) * [-0,-0]", -1.0 * -0.0, iv(-1.0) * iv(-0.0));
    row("[-0,-0] + [-0,-0]", -0.0 + -0.0, iv(-0.0) + iv(-0.0));
    row("[+0,+0] + [-0,-0]", 0.0 + -0.0, iv(0.0) + iv(-0.0));
    row("[+0,+0] - [+0,+0]", 0.0 - 0.0, iv(0.0) - iv(0.0));
    row("[-0,-0] - [+0,+0]", -0.0 - 0.0, iv(-0.0) - iv(0.0));
    row("-[+0,+0]", -0.0, -iv(0.0));
    row("abs([-0,-0])", (-0.0f64).abs(), iv(-0.0).abs());
    row("[-0,-0] / [2,2]", -0.0 / 2.0, iv(-0.0) / iv(2.0));
    row("copysign(1, -0.0)", 1.0f64.copysign(-0.0), Interval::one().copysign(iv(-0.0)));
    row("copysign(1, +0.0)", 1.0f64.copysign(0.0), Interval::one().copysign(iv(0.0)));
    let vf = Vec3::new(1.0, 0.0, -0.0).normalize();
    let ve = Vec3::new(iv(1.0), iv(0.0), iv(-0.0)).normalize();
    row("normalize((1,0,-0)).z", vf.z, ve.z);
}

/// **Table 1b — the sign bit through the Newell cross-sum**, on the
/// literal rings above: what `z` each mint actually carries.
#[test]
#[ignore = "signed-zero evidence instrument; run explicitly"]
fn newell_cross_sum_z_on_literal_vertical_walls() {
    println!("| ring | f64 sum.z | f64 normal.z | f64 u_ref.z | Interval normal.z | Interval u_ref.z |");
    println!("| --- | --- | --- | --- | --- | --- |");
    let named: Vec<(String, [[f64; 3]; 4])> = CUBE_WALLS
        .iter()
        .enumerate()
        .map(|(i, r)| (format!("cube wall {i}"), *r))
        .chain(
            HEX_WALLS
                .iter()
                .enumerate()
                .map(|(i, r)| (format!("hex wall {i}"), *r)),
        )
        .collect();
    for (name, ring) in named {
        let sf: Vec3<f64> = newell_normal_sum(&ring);
        let nf = sf.normalize();
        let (uf, _) = nf.orthonormal_basis();
        let se: Vec3<Interval> = newell_normal_sum(&ring);
        let ne = se.normalize();
        let (ue, _) = ne.orthonormal_basis();
        println!(
            "| {name} | {} | {} | {} | [{}, {}] | [{}, {}] |",
            bit(sf.z),
            bit(nf.z),
            bit(uf.z),
            bit(ne.z.lo()),
            bit(ne.z.hi()),
            bit(ue.z.lo()),
            bit(ue.z.hi())
        );
    }
}
