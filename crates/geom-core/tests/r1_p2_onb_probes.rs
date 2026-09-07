//! **Consumer probes — `Vec3::orthonormal_basis`.**
//!
//! Independent, adversarial rows exercising the constructor's two
//! claims from outside its own tests:
//!
//! 1. the `f64` path is the world-axis comparison's, BITWISE — probed
//!    here over inputs the unit's own sweep does not draw (non-unit
//!    magnitudes across ~600 decades, subnormals, signed zeros in `x`
//!    and `y` as well as `z`, and an LCG sweep seeded independently of
//!    proptest);
//! 2. the `interval` path answers a vertical plane with a DECIDED,
//!    exact frame — probed at NEAR-vertical enclosures (sign-definite
//!    tiny, straddling tiny, straddling wide) rather than only at the
//!    exact `[0, 0]` the unit pins.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-core/src/linalg/",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/ring_interval.rs",
    "interval-transcendentals/src/",
];

use geom_core::Vec3;

/// The world-axis comparison written out with a raw branch — the
/// spelling the constructor may not use, since a value branch does not
/// survive an enclosure scalar. PRIVATE to this suite, so the
/// comparison does not lean on anything the unit wrote.
fn reference(n: Vec3<f64>) -> (Vec3<f64>, Vec3<f64>) {
    let axis = if n.z * n.z <= n.x * n.x + n.y * n.y {
        Vec3::new(-n.y, n.x, 0.0)
    } else {
        Vec3::new(n.z, 0.0, -n.x)
    };
    let b1 = axis.normalize();
    (b1, n.cross(b1))
}

fn assert_bits_match(n: Vec3<f64>) {
    let (w1, w2) = reference(n);
    // Poison bits are not a contract (the constructor's own rows say
    // so): a case whose reference frame is not finite is skipped.
    let finite = |a: Vec3<f64>| a.x.is_finite() && a.y.is_finite() && a.z.is_finite();
    if !finite(w1) || !finite(w2) {
        return;
    }
    let (g1, g2) = n.orthonormal_basis();
    for (got, want, which) in [
        (g1.x, w1.x, "b1.x"),
        (g1.y, w1.y, "b1.y"),
        (g1.z, w1.z, "b1.z"),
        (g2.x, w2.x, "b2.x"),
        (g2.y, w2.y, "b2.y"),
        (g2.z, w2.z, "b2.z"),
    ] {
        assert_eq!(
            got.to_bits(),
            want.to_bits(),
            "{which} moved at n = ({:e}, {:e}, {:e}): {got:e} vs {want:e}",
            n.x,
            n.y,
            n.z
        );
    }
}

/// A dependency-free xorshift64* — seeded independently of the unit's
/// proptest sweep, so this row's draws are not the unit's draws.
struct Rng(u64);
impl Rng {
    fn next_f64_signed(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        let x = self.0.wrapping_mul(0x2545_F491_4F6C_DD1D);
        // Uniform in [-1, 1).
        ((x >> 11) as f64) / ((1u64 << 52) as f64) - 1.0
    }
}

/// **The f64 path is the comparison's — adversarial sweep.** Unit vectors,
/// NON-unit vectors across ~600 decades of magnitude (the constructor
/// documents non-unit inputs as well-defined), subnormal components,
/// and every signed-zero placement in every coordinate. The unit's own
/// bitwise row enumerates signed zeros in `z` only; `b1.y`, `b2.x` and
/// `b2.z` carry signed-zero products of `x` and `y` too.
#[test]
fn r1_onb_bits_match_the_reference_on_inputs_the_unit_did_not_draw() {
    // Signed zeros and axis values in EVERY coordinate, full cross
    // product: 7^3 = 343 cases including (0,0,0) and all-zero mixes.
    let vals = [0.0f64, -0.0, 1.0, -1.0, 0.6, -0.8, f64::MIN_POSITIVE];
    for x in vals {
        for y in vals {
            for z in vals {
                assert_bits_match(Vec3::new(x, y, z));
            }
        }
    }
    // Subnormals and magnitude extremes, near-vertical from both sides.
    for z in [
        5e-324, -5e-324, 1e-308, -1e-308, 1e-30, -1e-30, 1e30, -1e30, 1e300, -1e300,
    ] {
        for (x, y) in [
            (1.0, 0.0),
            (0.0, 1.0),
            (0.6, 0.8),
            (1e-200, 1e200),
            (1e300, 1e-300),
        ] {
            assert_bits_match(Vec3::new(x, y, z));
            assert_bits_match(Vec3::new(-x, y, z));
            assert_bits_match(Vec3::new(x, -y, z));
        }
    }
    // LCG sweep: unit and deliberately non-unit draws.
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for i in 0..200_000 {
        let v = Vec3::new(
            rng.next_f64_signed(),
            rng.next_f64_signed(),
            rng.next_f64_signed(),
        );
        // Every 4th draw stays raw (non-unit, possibly tiny); every
        // 4th is scaled huge; every 4th tiny; the rest normalized.
        let v = match i % 4 {
            0 => v,
            1 => v * 1e155,
            2 => v * 1e-155,
            _ => v.normalize(),
        };
        assert_bits_match(v);
        // And squashed near-vertical: z scaled to subnormal range.
        assert_bits_match(Vec3::new(v.x, v.y, v.z * 1e-320));
    }
}

/// **The frame is right, not merely bit-stable, at the vertical
/// planes** — an f64 sanity row this reviewer wants beside the bitwise
/// one: for unit normals with `n.z = ±0.0`, both frame vectors are
/// unit, mutually orthogonal, and orthogonal to `n` within 1e-15.
#[test]
fn r1_onb_frame_is_orthonormal_at_vertical_normals() {
    for z in [0.0f64, -0.0] {
        for (x, y) in [
            (1.0, 0.0),
            (-1.0, 0.0),
            (0.0, 1.0),
            (0.0, -1.0),
            (0.6, 0.8),
            (-0.28, 0.96),
        ] {
            let n = Vec3::new(x, y, z);
            let (b1, b2) = n.orthonormal_basis();
            for (val, what) in [
                (b1.dot(n), "b1.n"),
                (b2.dot(n), "b2.n"),
                (b1.dot(b2), "b1.b2"),
                (b1.dot(b1) - 1.0, "|b1|-1"),
                (b2.dot(b2) - 1.0, "|b2|-1"),
            ] {
                assert!(val.abs() < 1e-15, "{what} = {val:e} at n = ({x}, {y}, {z})");
            }
        }
    }
}

#[cfg(feature = "interval")]
mod interval_lane {
    use geom_core::interval::Interval;
    use geom_core::{Bounds, Real, Vec3};

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    fn components(b1: Vec3<Interval>, b2: Vec3<Interval>) -> [(Interval, &'static str); 6] {
        [
            (b1.x, "b1.x"),
            (b1.y, "b1.y"),
            (b1.z, "b1.z"),
            (b2.x, "b2.x"),
            (b2.y, "b2.y"),
            (b2.z, "b2.z"),
        ]
    }

    /// **Near-vertical, not merely vertical.** The unit pins
    /// `n.z = [±0.0, ±0.0]` exactly; these rows ask about the
    /// neighbourhood, where `n.z²` is still far below `n.x² + n.y²`
    /// and the axis choice therefore still DECIDES: a
    /// sign-definite tiny `z` (both sides), a straddling tiny
    /// enclosure, and point enclosures at subnormal `z`. Every
    /// component must stay bounded and certified, and must enclose the
    /// f64 frame of a representative point of the enclosure.
    #[test]
    fn r1_onb_interval_near_vertical_is_bounded_and_encloses_f64() {
        let zs: Vec<(Interval, f64)> = vec![
            (iv(1e-300), 1e-300),
            (iv(-1e-300), -1e-300),
            (iv(5e-324), 5e-324),
            (iv(-5e-324), -5e-324),
            (iv(1e-15), 1e-15),
            (iv(-1e-15), -1e-15),
            // Straddling tiny: the hull case, just off the unit's pin.
            (Interval::from_bounds(-1e-300, 1e-300), 0.0),
            (Interval::from_bounds(-1e-15, 1e-15), 0.0),
            (Interval::from_bounds(-1e-15, 1e-15), -0.0),
        ];
        for (z, z_rep) in zs {
            for (x, y) in [(0.0f64, 1.0f64), (1.0, 0.0), (0.6, 0.8), (0.0, -1.0)] {
                let n = Vec3::new(iv(x), iv(y), z);
                let (b1, b2) = n.orthonormal_basis();
                let f = Vec3::new(x, y, z_rep).orthonormal_basis();
                let fv = [f.0.x, f.0.y, f.0.z, f.1.x, f.1.y, f.1.z];
                for (i, (e, which)) in components(b1, b2).into_iter().enumerate() {
                    assert!(
                        e.lo().is_finite() && e.hi().is_finite(),
                        "{which} unbounded at n = ({x}, {y}, {z:?}): [{}, {}]",
                        e.lo(),
                        e.hi()
                    );
                    assert!(
                        e.is_certified(),
                        "{which} cannot decide at n = ({x}, {y}, {z:?})"
                    );
                    assert!(
                        e.lo() <= fv[i] && fv[i] <= e.hi(),
                        "{which} at n = ({x}, {y}, {z:?}) does not enclose the f64 \
                         frame {:e}: [{}, {}]",
                        fv[i],
                        e.lo(),
                        e.hi()
                    );
                }
            }
        }
    }

    /// **The exact vertical plane, and the sign a zero does not
    /// carry.** `[0, 0]` carries no sign bit — and it does not need
    /// one: the comparison reads `n.z²`, so `+0.0` and `−0.0` give the
    /// SAME `f64` frame and the point enclosure decides. Both are checked from outside anyway, which is what
    /// makes this a measurement of the claim rather than a restatement.
    #[test]
    fn r1_onb_interval_vertical_encloses_both_signed_zero_frames() {
        for (x, y) in [(0.0f64, 1.0f64), (1.0, 0.0), (0.6, 0.8), (-0.6, 0.8)] {
            let n = Vec3::new(iv(x), iv(y), iv(0.0));
            let (b1, b2) = n.orthonormal_basis();
            for z in [0.0f64, -0.0] {
                let f = Vec3::new(x, y, z).orthonormal_basis();
                let fv = [f.0.x, f.0.y, f.0.z, f.1.x, f.1.y, f.1.z];
                for (i, (e, which)) in components(b1, b2).into_iter().enumerate() {
                    assert!(
                        e.lo() <= fv[i] && fv[i] <= e.hi(),
                        "{which} at n = ({x}, {y}, [0,0]) drops the z = {z:?} frame \
                         {:e}: [{}, {}]",
                        fv[i],
                        e.lo(),
                        e.hi()
                    );
                }
            }
        }
    }

    /// **Where the construction genuinely ends** — measured, so the
    /// boundary is on record rather than implied. `normalize` reads
    /// each candidate's own norm, so an enclosure wide enough to leave
    /// the `n.z² ≤ n.x² + n.y²` comparison undecided AND to reach a
    /// direction parallel to the candidate axis it is choosing away
    /// from hulls in that candidate's zero vector. That takes a box
    /// spanning most of a meridian; a tight enclosure of a real normal
    /// never reaches it.
    #[test]
    fn r1_onb_interval_wide_box_boundary_recorded() {
        // A tight enclosure straddling the 45° cone: still bounded,
        // because both candidates are conditioned at least ‖n‖²/2.
        let half = core::f64::consts::FRAC_1_SQRT_2;
        let z = Interval::from_bounds(half - 1e-9, half + 1e-9);
        let n = Vec3::new(iv(0.6 * half), iv(0.8 * half), z);
        let (b1, b2) = n.orthonormal_basis();
        for (e, which) in components(b1, b2) {
            assert!(
                e.lo().is_finite() && e.hi().is_finite(),
                "{which} unbounded at a tight cone straddle: [{}, {}]",
                e.lo(),
                e.hi()
            );
        }
        // A whole meridian at the azimuth whose `e_y` candidate
        // degenerates. Recorded, not demanded.
        let n = Vec3::new(iv(0.0), iv(1.0), Interval::from_bounds(0.0, 1.0));
        let (b1, _) = n.orthonormal_basis();
        println!(
            "note: n = (0, 1, [0, 1]) gives b1.x = [{}, {}] (bounded: {}, certified: {})",
            b1.x.lo(),
            b1.x.hi(),
            b1.x.lo().is_finite() && b1.x.hi().is_finite(),
            b1.x.is_certified()
        );
    }
}
