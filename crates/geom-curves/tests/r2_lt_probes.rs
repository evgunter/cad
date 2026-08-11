//! R2 review probes for M8-14 (#222, PR #369): the joined integral
//! speed meter's SOUNDNESS FUZZ and JOIN-LATTICE pins.
//!
//! Why these exist (mutation evidence from the concurrent review):
//! three certifying-path mutants survived the entire shipped
//! geom-curves battery —
//!
//! - shrinking the per-span ACTIVE window by one (`lo_i+1..span`):
//!   UNSOUND (bound above true min speed by up to +2.3 on random
//!   degree-2 nets), undetected;
//! - dropping the LAST span from the per-span scan: UNSOUND (up to
//!   +1.4), undetected;
//! - replacing the join's one-sided recovery `(global poison,
//!   per-span sound) => per-span` with poison: undetected — the
//!   corpus's interpolated closed circle does NOT collapse the
//!   global chord exactly (the fit solve leaves an ulp-scale chord),
//!   so no shipped row exercises the abstain-recovery cell at all.
//!
//! The shipped suite's carriers are all smooth dense interpolants
//! whose per-span minima sit far from the window edges; these probes
//! close both gaps: a seeded random-net soundness fuzz (kills both
//! unsound mutants) and four explicit join-lattice pins (kills the
//! lattice mutant).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// Refusal rows accept EITHER a non-positive bound or poison, and
// `m <= 0.0` would let a NaN pass — same posture as the M8-14 suite.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

use geom_core::Point3;
use geom_core::spline::KnotVector;
use geom_curves::NurbsCurve3;

fn dense_min_speed(c: &NurbsCurve3<f64>, n: usize) -> f64 {
    let (lo, hi) = c.domain();
    let mut smin = f64::INFINITY;
    for i in 0..=n {
        #[allow(clippy::cast_precision_loss)]
        let t = lo + (hi - lo) * (i as f64) / (n as f64);
        let s = c.deriv(t).norm();
        if s < smin {
            smin = s;
        }
    }
    smin
}

/// Deterministic LCG (Knuth MMIX constants) so the fuzz is a fixed
/// schedule, not a decision — reruns bit-identically.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let v = ((self.0 >> 11) as f64) / ((1_u64 << 53) as f64);
        v
    }
}

/// A random integral net on a random clamped knot vector, with ~30%
/// of interior knots raised to multiplicity ≤ p (C0 kinks included —
/// the window-edge geometry the smooth corpus never produces).
fn random_curve(seed: u64, deg: usize, npts: usize) -> Option<NurbsCurve3<f64>> {
    let mut r = Lcg(seed);
    let control: Vec<Point3<f64>> = (0..npts)
        .map(|_| {
            Point3::new(
                r.next().mul_add(4.0, -2.0),
                r.next().mul_add(4.0, -2.0),
                r.next().mul_add(4.0, -2.0),
            )
        })
        .collect();
    let interior = npts - deg - 1;
    let mut knots = vec![0.0; deg + 1];
    let mut v = 0.0;
    let mut i = 0;
    while i < interior {
        v += 0.2 + r.next();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let mult = if seed.is_multiple_of(2) && r.next() < 0.3 {
            (1 + (r.next() * (deg as f64)) as usize)
                .min(deg)
                .min(interior - i)
        } else {
            1
        };
        for _ in 0..mult {
            knots.push(v);
        }
        i += mult;
    }
    v += 0.2 + r.next();
    for _ in 0..=deg {
        knots.push(v);
    }
    let kv = KnotVector::clamped(knots, deg).ok()?;
    NurbsCurve3::<f64>::new(kv, control, vec![1.0; npts]).ok()
}

/// **Soundness fuzz**: on 400 seeded random integral nets (degree
/// 2–5, 4–15 control points, interior multiplicities included), the
/// meter never exceeds the densely sampled true minimum speed. The
/// two surviving unsound mutants (active window off by one; last
/// span dropped) each fail this within the first hundred seeds.
#[test]
fn the_meter_is_sound_on_random_integral_nets() {
    let mut built = 0_usize;
    for seed in 0..400_u64 {
        #[allow(clippy::cast_possible_truncation)]
        let deg = 2 + (seed % 4) as usize;
        #[allow(clippy::cast_possible_truncation)]
        let npts = deg + 2 + (seed % 9) as usize;
        let Some(c) = random_curve(seed * 7919 + 13, deg, npts) else {
            continue;
        };
        built += 1;
        let m = c.speed_lower_bound();
        if m.is_nan() {
            // Poison is an honest abstention, never an unsound number.
            continue;
        }
        let smin = dense_min_speed(&c, 4000);
        assert!(
            m <= smin + 1e-9,
            "seed {seed} (deg {deg}, {npts} pts): meter {m} exceeds the \
             densely sampled true min speed {smin} — the bound is UNSOUND"
        );
    }
    assert!(built >= 300, "fuzz rot: only {built}/400 nets constructed");
}

/// **The join lattice, all four cells** (doc: "The join" on
/// `speed_lower_bound`) — plus the no-laundering row. Every cell is
/// pinned on a fixture that produces it EXACTLY (control points equal
/// bitwise), which the interpolated corpus never does: the fit solve
/// leaves an ulp-scale global chord on the closed circle, so before
/// this test the `(poison, sound)` recovery cell had no coverage.
#[test]
fn the_join_lattice_is_pinned_cell_by_cell() {
    // (sound, sound): a generic open quadratic — real and positive.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.2, 0.0),
            Point3::new(2.0, 0.0, 0.0),
        ],
        vec![1.0; 3],
    )
    .unwrap();
    let m = c.speed_lower_bound();
    assert!(
        m > 0.0,
        "(sound, sound): expected a positive meter, got {m}"
    );

    // (poison, sound): an EXACTLY closed loop — first == last control
    // point bitwise, so the global chord is 0/0 and that assembly
    // abstains; the per-span scan carries the whole answer. The loop's
    // speed never collapses, so the join must be real AND positive —
    // and sound (this square loop's per-span bound is exactly √2/2,
    // the true minimum).
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ],
        vec![1.0; 5],
    )
    .unwrap();
    let m = c.speed_lower_bound();
    assert!(
        m > 0.0,
        "(poison, sound): the abstain-recovery cell must return the \
         per-span assembly's real bound, got {m}"
    );
    let smin = dense_min_speed(&c, 4000);
    assert!(
        m <= smin + 1e-9,
        "recovered bound {m} above true min {smin}"
    );

    // (sound, poison): one span's own chord collapses (P0 == P2
    // bitwise, degree 2) while the global chord does not — the
    // per-span assembly abstains and the global one carries. The
    // fixture doubles back, so real-but-non-positive is the honest
    // global answer; the cell's pin is REAL (not poison).
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(2.0, -1.0, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let m = c.speed_lower_bound();
    assert!(
        !m.is_nan(),
        "(sound, poison): the global assembly still stands, so the join \
         must be real, got {m}"
    );

    // (poison, poison): out-and-back A-B-A — global chord AND the one
    // span chord both collapse; both assemblies abstain; the join is
    // poison, never a fabricated number.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ],
        vec![1.0; 3],
    )
    .unwrap();
    let m = c.speed_lower_bound();
    assert!(
        m.is_nan(),
        "(poison, poison): both abstain => poison, got {m}"
    );

    // No laundering (D4 ¶2): a poisoned INPUT (non-finite control
    // point) poisons the projections of BOTH assemblies — the join's
    // one-sided recovery must not resurrect a number from corrupt
    // data.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
    if let Ok(c) = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(f64::NAN, 1.0, 0.0),
            Point3::new(1.0, 2.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
        ],
        vec![1.0; 4],
    ) {
        let m = c.speed_lower_bound();
        assert!(m.is_nan(), "a poisoned input must stay poison, got {m}");
    }
}
