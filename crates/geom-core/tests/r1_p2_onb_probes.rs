//! **PCURVE P-2, R1 consumer probes — `Vec3::orthonormal_basis` (#1157).**
//!
//! Independent, adversarial rows exercising the unit's two claims from
//! outside its own tests:
//!
//! 1. the `f64` path is BIT-IDENTICAL to the Duff spelling it replaced
//!    — probed here over inputs the unit's own sweep does not draw
//!    (non-unit magnitudes across ~600 decades, subnormals, signed
//!    zeros in `x` and `y` as well as `z`, and an LCG sweep seeded
//!    independently of proptest);
//! 2. the `interval` path no longer manufactures an unbounded / `Trv`
//!    frame for a vertical plane — probed at NEAR-vertical enclosures
//!    (sign-definite tiny, straddling tiny, straddling wide) rather
//!    than only the exact `[0, 0]` the unit pins.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Real, Vec3};

/// The Duff et al. §3 spelling exactly as the kernel carried it before
/// #1157 — `a = −1/(s + n.z)` with the sum written literally. Kept
/// verbatim and PRIVATE to this suite so the comparison does not lean
/// on anything the unit wrote.
fn duff(n: Vec3<f64>) -> (Vec3<f64>, Vec3<f64>) {
    let s = <f64 as Real>::copysign(1.0, n.z);
    let a = -1.0 / (s + n.z);
    let b = (n.x * n.y) * a;
    (
        Vec3::new(
            1.0 + (s * <f64 as Real>::powi(n.x, 2)) * a,
            s * b,
            -(s * n.x),
        ),
        Vec3::new(b, s + <f64 as Real>::powi(n.y, 2) * a, -n.y),
    )
}

fn assert_bits_match(n: Vec3<f64>) {
    let (g1, g2) = n.orthonormal_basis();
    let (w1, w2) = duff(n);
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

/// **The f64 path did not move — adversarial sweep.** Unit vectors,
/// NON-unit vectors across ~600 decades of magnitude (the constructor
/// documents non-unit inputs as well-defined), subnormal components,
/// and every signed-zero placement in every coordinate. The unit's own
/// bitwise row enumerates signed zeros in `z` only; `b1.y`, `b2.x` and
/// `b2.z` carry signed-zero products of `x` and `y` too.
#[test]
fn r1_onb_bits_match_duff_on_inputs_the_unit_did_not_draw() {
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
        for (x, y) in [(1.0, 0.0), (0.0, 1.0), (0.6, 0.8), (1e-200, 1e200), (1e300, 1e-300)] {
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
                assert!(
                    val.abs() < 1e-15,
                    "{what} = {val:e} at n = ({x}, {y}, {z})"
                );
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

    fn components(
        b1: Vec3<Interval>,
        b2: Vec3<Interval>,
    ) -> [(Interval, &'static str); 6] {
        [
            (b1.x, "b1.x"),
            (b1.y, "b1.y"),
            (b1.z, "b1.z"),
            (b2.x, "b2.x"),
            (b2.y, "b2.y"),
            (b2.z, "b2.z"),
        ]
    }

    /// **Near-vertical, not merely vertical.** #1157's fix is pinned by
    /// the unit at `n.z = [±0.0, ±0.0]` exactly. These rows ask about
    /// the neighbourhood: a sign-definite tiny `z` (both sides), a
    /// STRADDLING tiny enclosure (where `copysign` must hull to
    /// `[−1, 1]`), and point enclosures at subnormal `z`. Every
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

    /// **The exact vertical plane must enclose BOTH signed-zero f64
    /// frames.** `[0, 0]` carries no sign bit, `+0.0` and `−0.0` give
    /// different (both valid) f64 frames, and the interval answer must
    /// contain both — this is the hull claim the unit asserts one side
    /// of, checked from outside for both sides.
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

    /// **Where the fix genuinely ends** — measured, so the boundary is
    /// on record rather than implied. A `z` enclosure that straddles
    /// zero AND reaches magnitude 1 makes `1 + s·z` contain zero again
    /// (`s = [−1, 1]`, `s·z ⊇ [−1, 1]`), so the frame is unbounded for
    /// such inputs under the NEW spelling too. An enclosure that wide
    /// is a whole hemisphere-to-hemisphere hull, not a chart question;
    /// this row records the behaviour rather than demanding one.
    #[test]
    fn r1_onb_interval_full_straddle_boundary_recorded() {
        // Straddling but strictly inside (−1, 1): must stay bounded.
        let z = Interval::from_bounds(-0.9, 0.9);
        let n = Vec3::new(iv(0.1), iv(0.2), z);
        let (b1, b2) = n.orthonormal_basis();
        for (e, which) in components(b1, b2) {
            assert!(
                e.lo().is_finite() && e.hi().is_finite(),
                "{which} unbounded at a [-0.9, 0.9] straddle: [{}, {}]",
                e.lo(),
                e.hi()
            );
        }
        // Straddling and reaching ±1: record whether it is bounded.
        let z = Interval::from_bounds(-1.0, 1.0);
        let n = Vec3::new(iv(0.0), iv(0.1), z);
        let (b1, _) = n.orthonormal_basis();
        println!(
            "R1 note: full-straddle z = [-1, 1] gives b1.x = [{}, {}] (bounded: {})",
            b1.x.lo(),
            b1.x.hi(),
            b1.x.lo().is_finite() && b1.x.hi().is_finite()
        );
    }
}
