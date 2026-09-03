//! S8 adversarial review probe, in two halves that check each other.
//!
//! 1. **Against the real constructor**: three fuzz-found arc×arc
//!    two-survivor corners, each built through the arc-carrier fillet
//!    door, asserting the emitted tangent point lies at distance r from
//!    the componentwise-dominant candidate's centre and NOT from the
//!    loser's. This is the only row that pins the S8 PICK against
//!    kernel code; the fuzz below re-derives the prediction but never
//!    calls it.
//! 2. **The standalone dominance fuzz**: an independent re-derivation
//!    of the offset/gate machinery (the reviewer's
//!    `review-scratch/s8/dominance_fuzz.rs`). Over every two-survivor
//!    corner it finds — arc×arc and line×arc — the smaller-total
//!    candidate dominates componentwise, so sum, max and every monotone
//!    combination agree on which candidate the S8 ladder must pick, and
//!    no enclosing tangency (ρ < 0) ever participates. None of
//!    `sugar.rs`'s private machinery is used.
//!
//! Half 1's three corners have DISTINCT far points — the arc×arc shape
//! class the §2c dissolution made authorable on the lattice: the entry
//! fused verb rides carrier 1 from `far1`, the arc arrival emits its
//! run to the hard anchor `far2`, and the loop closes with the sharp
//! straight seam `line_to(Start)`. The carriers, radii, corner and
//! predicted candidates are the fuzz dump's, verbatim; the far ANCHORS
//! are re-derived on the same carriers — each far enough behind (resp.
//! ahead of) the corner that BOTH candidates' trims fit, and close
//! enough that the mirror corner sits behind the incoming anchor —
//! because the lattice's signed-swept gates bound a side to half a
//! turn from its anchor (past-the-end classifies Negative rather than
//! wrapping) while the raw builder's dumped extents ran to nearly a
//! full turn. The pick under test — which of the corner's two
//! surviving candidates the ladder returns — is a function of the
//! carriers and r alone, so re-anchoring the extents does not move it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// The fixture coordinates are fuzz-dumped at 17 significant digits (the
// f64 round-trip length); trimming digits to appease the lint could
// change the bits under test.
#![allow(clippy::excessive_precision)]

test_utils::gated_to!["crates/profile/src/", "crates/geom-core/src/tolerance.rs"];

mod common;
use geom_core::Point2;
use geom_core::Tol;
use profile::{ArcSweep, Center, Open, Start};
fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

#[allow(clippy::too_many_arguments)]
fn check(
    o1: (f64, f64),
    s1: ArcSweep,
    o2: (f64, f64),
    s2: ArcSweep,
    corner: (f64, f64),
    far1: (f64, f64),
    far2: (f64, f64),
    r: f64,
    win: (f64, f64),
    lose: (f64, f64),
) {
    let lp = Open
        .arc_fillet_arc(
            Center {
                c: p2(o1.0, o1.1),
                winding: s1,
                p: p2(far1.0, far1.1),
            },
            r,
            Center {
                c: p2(o2.0, o2.1),
                winding: s2,
                p: p2(far2.0, far2.1),
            },
            Tol::witness(),
        )
        .expect("two-survivor corner must construct")
        .line_to(Start, Tol::witness())
        .expect("the sharp seam closes")
        .loop_;
    let t1 = lp.vertices()[1].pos();
    // The pick must round the DUMPED corner, not the pair's mirror
    // crossing (the corner is the +y-side root; the mirror is its
    // reflection across the centre line): the emitted incoming tangent
    // point sits strictly nearer the dumped corner.
    let (c, m) = {
        let d = (o2.0 - o1.0, o2.1 - o1.1);
        let len2 = d.0 * d.0 + d.1 * d.1;
        let v = (corner.0 - o1.0, corner.1 - o1.1);
        let t = (v.0 * d.0 + v.1 * d.1) / len2;
        let foot = (o1.0 + t * d.0, o1.1 + t * d.1);
        (
            p2(corner.0, corner.1),
            p2(2.0 * foot.0 - corner.0, 2.0 * foot.1 - corner.1),
        )
    };
    let (dc, dm) = (
        ((t1.x - c.x).powi(2) + (t1.y - c.y).powi(2)).sqrt(),
        ((t1.x - m.x).powi(2) + (t1.y - m.y).powi(2)).sqrt(),
    );
    assert!(
        dc < dm,
        "the emitted trim rounds the mirror crossing, not the dumped corner \
         (|t1-corner|={dc}, |t1-mirror|={dm})"
    );
    let dw = ((t1.x - win.0).powi(2) + (t1.y - win.1).powi(2)).sqrt();
    let dl = ((t1.x - lose.0).powi(2) + (t1.y - lose.1).powi(2)).sqrt();
    assert!(
        (dw - r).abs() < 1e-9,
        "picked tangent not on predicted winner circle: |t1-win|={dw}, r={r}"
    );
    assert!(
        (dl - r).abs() > 1e-3,
        "pick is ambiguous against the losing candidate: |t1-lose|={dl}, r={r}"
    );
}

#[test]
fn fuzz_found_two_survivor_corners_pick_the_dominant_candidate() {
    check(
        (0.0, 0.0),
        ArcSweep::Cw,
        (0.95562179434455674, 0.0),
        ArcSweep::Ccw,
        (-0.22946874909608578, 1.65519209137498335),
        (-0.76140938632480071, -1.48747185270125493),
        (0.11472866679104776, -1.85391456209179362),
        0.15189083980143034,
        (-0.81863298895661929, 1.27968806714547267),
        (-0.81863298895661929, -1.27968806714547267),
    );
    check(
        (0.0, 0.0),
        ArcSweep::Ccw,
        (3.82055744659083052, 0.0),
        ArcSweep::Ccw,
        (1.17316720402785357, 1.21354460418236210),
        (1.47583209002557170, -0.81910404528204495),
        (4.09329211202486132, -2.89947950588176573),
        0.31664440369683866,
        (1.27464246658060087, 0.50560149083164663),
        (1.27464246658060087, -0.50560149083164663),
    );
    check(
        (0.0, 0.0),
        ArcSweep::Cw,
        (1.68543221900967244, 0.0),
        ArcSweep::Ccw,
        (0.24842682255474419, 0.89584175565974999),
        (-0.71733306129772123, -0.59133883393316000),
        (1.62811319906494045, -1.69240405653517167),
        0.32427394642951701,
        (-0.25623799695610200, 0.54847219060938479),
        (-0.25623799695610200, -0.54847219060938479),
    );
}

// ------------------------------------------------------------------
// The review's standalone dominance fuzz, trimmed into one committed
// row (independent re-implementation — none of `sugar.rs`'s private
// machinery). Two sampled classes: uniform arc×arc corners and
// line×arc corners. Asserts, over every two-survivor corner found:
// componentwise dominance of the smaller-total candidate (the S8
// monotone-combination claim) and that no enclosing tangency (ρ < 0)
// participates. The review's full-scale artifacts (5M-trial phases,
// the targeted both-enclosing construction, and the hill-climb search
// that failed to force an enclosing two-survivor corner) live in the
// S8 review scratch; this row keeps the claim continuously exercised
// at a CI-friendly trial count that rides `CAD_FUZZ_EFFORT`.

mod dominance {
    use std::f64::consts::TAU;

    /// The oracle's own signed sweep. Spelled HERE rather than called
    /// from `sugar.rs` on purpose — this suite's oracles are written
    /// from the geometry, and an oracle that calls the code it checks
    /// checks nothing. What it must not be is the shape the production
    /// helper retired: reducing into `[0, τ)` first and folding that
    /// puts a jump at a difference of zero, which is the value this
    /// function exists to read, and an oracle carrying the retired
    /// shape is a copy waiting to be cited as precedent.
    pub(super) fn signed_swept(from: f64, to: f64, turn: f64) -> f64 {
        let d = (to - from) * turn;
        d - TAU * (d / TAU + 0.5).floor()
    }

    /// `+1` or `−1` with equal probability.
    pub(super) fn sign(rng: &mut test_utils::fuzz::Rng) -> f64 {
        if rng.unit() < 0.5 { 1.0 } else { -1.0 }
    }

    #[derive(Clone, Copy)]
    pub(super) struct Arc {
        pub(super) ox: f64,
        pub(super) oy: f64,
        pub(super) r: f64,
        pub(super) turn: f64,
        pub(super) corner_angle: f64,
        pub(super) len: f64,
    }

    #[derive(Debug, Clone, Copy)]
    pub(super) struct Survivor {
        pub(super) sb_in: f64,
        pub(super) sb_out: f64,
    }

    /// Survivors + signed offset radii for an arc×arc corner (the
    /// reviewer's `classify`, verbatim semantics).
    pub(super) fn classify(
        a1: &Arc,
        a2: &Arc,
        sgn: f64,
        r: f64,
    ) -> Option<(Vec<Survivor>, f64, f64)> {
        let rho1 = a1.r - sgn * a1.turn * r;
        let rho2 = a2.r - sgn * a2.turn * r;
        let (r1, r2) = (rho1.abs(), rho2.abs());
        let (lx, ly) = (a2.ox - a1.ox, a2.oy - a1.oy);
        let d2 = lx * lx + ly * ly;
        let d = d2.sqrt();
        // Clearance gates with a fat margin so band details cannot matter.
        if r1 + r2 - d < 1e-9 || d - (r1 - r2).abs() < 1e-9 {
            return None;
        }
        let along = (d2 + rho1 * rho1 - rho2 * rho2) / (2.0 * d);
        let (bx, by) = (a1.ox + lx * along / d, a1.oy + ly * along / d);
        let half2 = rho1 * rho1 - along * along;
        if half2 < 1e-18 {
            return None;
        }
        let half = half2.sqrt();
        let (nx, ny) = (-ly / d, lx / d);
        let cands = [
            (bx + nx * half, by + ny * half),
            (bx - nx * half, by - ny * half),
        ];
        let mut surv = Vec::new();
        for &(px, py) in &cands {
            let t = |a: &Arc, rho: f64| {
                let (vx, vy) = (px - a.ox, py - a.oy);
                (a.ox + vx * a.r / rho, a.oy + vy * a.r / rho)
            };
            let (t1x, t1y) = t(a1, rho1);
            let (t2x, t2y) = t(a2, rho2);
            let ang1 = (t1y - a1.oy).atan2(t1x - a1.ox);
            let ang2 = (t2y - a2.oy).atan2(t2x - a2.ox);
            let sb_in = a1.r * signed_swept(ang1, a1.corner_angle, a1.turn);
            let sb_out = a2.r * signed_swept(a2.corner_angle, ang2, a2.turn);
            if sb_in >= 0.0 && sb_out >= 0.0 && a1.len - sb_in >= 0.0 && a2.len - sb_out >= 0.0 {
                surv.push(Survivor { sb_in, sb_out });
            }
        }
        Some((surv, rho1, rho2))
    }

    /// Dominance + no-enclosing bookkeeping for one classified corner.
    /// `stats` = (two-survivor count, violations, enclosing-involved).
    pub(super) fn tally(surv: &[Survivor], rho1: f64, rho2: f64, stats: &mut (u64, u64, u64)) {
        if surv.len() != 2 {
            return;
        }
        stats.0 += 1;
        if rho1 < 0.0 || rho2 < 0.0 {
            stats.2 += 1;
        }
        let (a, b) = (&surv[0], &surv[1]);
        let (win, lose) = if a.sb_in + a.sb_out <= b.sb_in + b.sb_out {
            (a, b)
        } else {
            (b, a)
        };
        if !(win.sb_in <= lose.sb_in && win.sb_out <= lose.sb_out) {
            stats.1 += 1;
        }
    }
}

/// The trimmed dominance fuzz: over every two-survivor corner the
/// independent machinery finds, the smaller-total candidate dominates
/// componentwise (so sum, max and every monotone combination agree on
/// the S8 pick) and no enclosing tangency participates.
#[test]
fn dominance_fuzz_two_survivor_corners() {
    use dominance::{Arc, classify, sign, signed_swept, tally};
    use std::f64::consts::TAU;
    use test_utils::fuzz;

    // --- arc×arc, uniform over crossing carriers -------------------
    let mut rng = fuzz::start("review_s8_probe::dominance_arc_arc");
    let trials = fuzz::scaled(18_750) as u64;
    let mut stats = (0u64, 0u64, 0u64);
    for _ in 0..trials {
        let d = rng.range(0.05, 4.0);
        let r1c = rng.range(0.05, 3.0);
        let r2c = rng.range(0.05, 3.0);
        if d >= r1c + r2c || d <= (r1c - r2c).abs() {
            continue;
        }
        let along = (d * d + r1c * r1c - r2c * r2c) / (2.0 * d);
        let h2 = r1c * r1c - along * along;
        if h2 <= 1e-14 {
            continue;
        }
        let (cx, cy) = (along, h2.sqrt());
        let (t1, t2) = (sign(&mut rng), sign(&mut rng));
        let a1 = Arc {
            ox: 0.0,
            oy: 0.0,
            r: r1c,
            turn: t1,
            corner_angle: cy.atan2(cx),
            len: r1c * rng.range(0.05, TAU - 0.05),
        };
        let a2 = Arc {
            ox: d,
            oy: 0.0,
            r: r2c,
            turn: t2,
            corner_angle: cy.atan2(cx - d),
            len: r2c * rng.range(0.05, TAU - 0.05),
        };
        let dir = |a: &Arc| {
            let (vx, vy) = (cx - a.ox, cy - a.oy);
            (-vy * a.turn / a.r, vx * a.turn / a.r)
        };
        let (d1x, d1y) = dir(&a1);
        let (d2x, d2y) = dir(&a2);
        let turn = d1x * d2y - d1y * d2x;
        if turn.abs() < 1e-9 {
            continue;
        }
        let r = rng.range(0.01, 3.0);
        if let Some((surv, rho1, rho2)) = classify(&a1, &a2, turn.signum(), r) {
            tally(&surv, rho1, rho2, &mut stats);
        }
    }
    assert_eq!(
        stats.1,
        0,
        "dominance violations (arc x arc) — {}",
        fuzz::replay()
    );
    assert_eq!(
        stats.2,
        0,
        "enclosing two-survivor corners (arc x arc) — {}",
        fuzz::replay()
    );
    println!(
        "[s8 arc x arc] {trials} trials, {} two-survivor corners",
        stats.0
    );
    // COVERAGE FLOOR as a FRACTION of the trial count (the review's own
    // density: 500 two-survivor corners per 150 000 trials), so effort
    // and floor move together.
    assert!(
        stats.0 * 300 > trials,
        "coverage floor: only {} two-survivor out of {trials} — {}",
        stats.0,
        fuzz::replay()
    );

    // --- line×arc ---------------------------------------------------
    let mut rng = fuzz::start("review_s8_probe::dominance_line_arc");
    let trials = fuzz::scaled(18_750) as u64;
    let mut stats = (0u64, 0u64, 0u64);
    for _ in 0..trials {
        // Corner at the origin; straight incoming leg travelling (dx, dy).
        let phi = rng.range(0.0, TAU);
        let (dx, dy) = (phi.cos(), phi.sin());
        let line_len = rng.range(0.1, 6.0);
        let rc = rng.range(0.05, 3.0);
        let th = rng.range(0.0, TAU);
        let (ox, oy) = (rc * th.cos(), rc * th.sin());
        let tau = sign(&mut rng);
        let ca = (-oy).atan2(-ox);
        let alen = rc * rng.range(0.05, TAU - 0.05);
        let r = rng.range(0.01, 2.5);
        let (adx, ady) = (oy * tau / rc, -ox * tau / rc);
        let turn = dx * ady - dy * adx;
        if turn.abs() < 1e-6 {
            continue;
        }
        let sgn = turn.signum();
        let (nx, ny) = (-dy, dx);
        let rho = rc - sgn * tau * r;
        let h = (ox - nx * sgn * r) * nx + (oy - ny * sgn * r) * ny;
        if rho.abs() - h.abs() < 1e-9 {
            continue;
        }
        let (fx, fy) = (ox - nx * h, oy - ny * h);
        let half = (rho * rho - h * h).sqrt();
        let mut surv = Vec::new();
        for (px, py) in [
            (fx + dx * half, fy + dy * half),
            (fx - dx * half, fy - dy * half),
        ] {
            let (t1x, t1y) = (px - nx * sgn * r, py - ny * sgn * r);
            let sb_in = -(t1x * dx + t1y * dy);
            // Tangent point O + (P − O)·R/ρ; its angle about O.
            let (wx, wy) = (px - ox, py - oy);
            let an = (wy * rc / rho).atan2(wx * rc / rho);
            let sb_out = rc * signed_swept(ca, an, tau);
            if sb_in >= 0.0 && sb_out >= 0.0 && line_len - sb_in >= 0.0 && alen - sb_out >= 0.0 {
                surv.push(dominance::Survivor { sb_in, sb_out });
            }
        }
        tally(&surv, rho, 1.0, &mut stats);
    }
    assert_eq!(
        stats.1,
        0,
        "dominance violations (line x arc) — {}",
        fuzz::replay()
    );
    assert_eq!(
        stats.2,
        0,
        "enclosing two-survivor corners (line x arc) — {}",
        fuzz::replay()
    );
    println!(
        "[s8 line x arc] {trials} trials, {} two-survivor corners",
        stats.0
    );
    // COVERAGE FLOOR as a fraction of the trial count (see above).
    assert!(
        stats.0 * 300 > trials,
        "coverage floor: only {} two-survivor out of {trials} — {}",
        stats.0,
        fuzz::replay()
    );
}
