//! R1 review probes for BOOL-11 (PR #1520). ADDITIVE — probe branch
//! `bool/11r1-probes` only. Nothing here is proposed for the PR.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Open, PathError, Profile, SketchPlane, Start};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Author `line(1)` from the origin along +x, then `continue_to((2, dy))`.
/// `at = (1,0)`, `û = (1,0)`, so `across == dy` EXACTLY (perp_dot of a
/// unit +x ray with `(1, dy)`), which is what makes ulp probing sound.
fn attempt(dy: f64) -> Result<(), PathError<f64>> {
    let t = Tol::witness();
    Open.at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(2.0, dy), t)
        .map(|_| ())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Regime {
    Accept,
    Escalate,
    Refuse,
    Other,
}

fn regime(dy: f64) -> Regime {
    match attempt(dy) {
        Ok(()) => Regime::Accept,
        Err(PathError::Escalated { .. }) => Regime::Escalate,
        Err(PathError::ContinuationTargetOffRay { .. }) => Regime::Refuse,
        Err(_) => Regime::Other,
    }
}

fn up(x: f64) -> f64 {
    f64::from_bits(x.to_bits() + 1)
}
fn down(x: f64) -> f64 {
    f64::from_bits(x.to_bits() - 1)
}

/// **P1 — the three regimes at both band edges, ±1 ulp.** The funnel
/// contract is |m| ≤ ε ⇒ Zero, |m| ≥ K·ε ⇒ definite, strictly between ⇒
/// Indeterminate. If `continue_to` really rides the funnel on the linear
/// band, the transitions must land on exactly those ulps.
#[test]
fn r1_band_edges_to_the_ulp() {
    let t = Tol::witness();
    let (eps, kin) = (t.eps(), t.k() * t.eps());
    println!("R1: eps={eps:e} K={} eps_input={kin:e}", t.k());
    let cases: [(f64, Regime, &str); 8] = [
        (0.0, Regime::Accept, "exactly on the ray"),
        (down(eps), Regime::Accept, "eps - 1ulp"),
        (eps, Regime::Accept, "exactly eps (closed below)"),
        (up(eps), Regime::Escalate, "eps + 1ulp"),
        (down(kin), Regime::Escalate, "K*eps - 1ulp"),
        (kin, Regime::Refuse, "exactly K*eps (closed above)"),
        (up(kin), Regime::Refuse, "K*eps + 1ulp"),
        (1e3 * kin, Regime::Refuse, "far off"),
    ];
    for (dy, want, name) in cases {
        let got = regime(dy);
        println!("R1:   dy={dy:e} ({name}) -> {got:?}");
        assert_eq!(got, want, "band edge {name} at dy={dy:e}");
        // Negative side must mirror: the miss is signed, the band is not.
        assert_eq!(regime(-dy), want, "mirrored band edge {name}");
    }
}

/// **P1b — a BARE comparison against K·ε would have swallowed the
/// escalation band.** Measured, not argued: count the ulps in (ε, K·ε)
/// that escalate, and confirm a `|across| < K·eps` accept-test would
/// have accepted every one of them.
#[test]
fn r1_the_funnel_is_what_saves_the_escalation_band() {
    let t = Tol::witness();
    let (eps, kin) = (t.eps(), t.k() * t.eps());
    let mut swallowed = 0usize;
    let mut probe = up(eps);
    for _ in 0..64 {
        assert_eq!(regime(probe), Regime::Escalate, "at {probe:e}");
        assert!(probe < kin, "still inside the band");
        swallowed += 1;
        probe = up(probe);
    }
    // The middle of the band, and just inside its top.
    for dy in [0.5 * (eps + kin), down(kin)] {
        assert_eq!(regime(dy), Regime::Escalate);
        assert!(dy < kin, "a bare `< K*eps` test would have ACCEPTED {dy:e}");
        swallowed += 1;
    }
    println!("R1: {swallowed} probed misses in (eps, K*eps) escalate; all < K*eps");
}

/// **P2 — the no-lever claim, part 1: the miss is scale-free in the
/// DIRECTOR's magnitude.** `toward(dx, dy)` normalizes, so `across` is a
/// true metre distance and not the director's length times something.
/// If `Dir::unit` were ever stored un-normalized (the verb table says
/// "ray stored verbatim"), this row goes red.
#[test]
fn r1_no_lever_is_scale_free_in_the_director() {
    let t = Tol::witness();
    let (_, kin) = (t.eps(), t.k() * t.eps());
    let miss = 5.0 * kin;
    for mag in [1e-6_f64, 1.0, 1e3, 1e6] {
        if mag <= t.k() * t.eps() {
            continue;
        }
        let err = Open
            .at(p2(0.0, 0.0))
            .toward(mag, 0.0, t)
            .unwrap()
            .line(1.0, t)
            .unwrap()
            .continue_to(p2(2.0, miss), t)
            .expect_err("definitely off the ray whatever the director's magnitude");
        match err {
            PathError::ContinuationTargetOffRay { across, along } => {
                println!("R1: director magnitude {mag:e} -> across={across:e} along={along:e}");
                assert!(
                    (across - miss).abs() <= 8.0 * f64::EPSILON * miss,
                    "across must be the metre miss, not scaled by the director: {across:e}"
                );
                assert!((along - 1.0).abs() < 1e-9);
            }
            other => panic!("{other:?}"),
        }
    }
}

/// **P2b — the no-lever claim, part 2: what the absolute threshold
/// actually costs in ANGLE.** Measure the largest angular deviation the
/// check tolerates as a function of how far along the ray the target
/// sits. This is the semantics question the design decision turns on,
/// reported as a number rather than an opinion.
#[test]
fn r1_no_lever_angular_sensitivity_versus_target_distance() {
    let t = Tol::witness();
    let eps = t.eps();
    println!("R1: target distance L | accepting angular half-width (rad) at |across| = eps");
    for l in [1e-3_f64, 1.0, 1e3, 1e6] {
        // A target L along the ray, tilted so the miss is exactly eps.
        let theta = (eps / l).atan();
        assert!(
            Open.at(p2(0.0, 0.0))
                .angle(0.0, t)
                .unwrap()
                .line(1.0, t)
                .unwrap()
                .continue_to(p2(1.0 + l, eps), t)
                .is_ok(),
            "a miss of exactly eps must be accepted at L={l:e}"
        );
        println!("R1:   L={l:e} -> theta={theta:e} rad");
    }
    // And the same angular error at two distances lands in two regimes:
    // the absolute miss, not the angle, is what decides.
    let theta = 1e-8_f64;
    let near = regime(theta * 1.0); // L = 1
    let far = {
        let l = 1e4;
        let t2 = Tol::witness();
        match Open
            .at(p2(0.0, 0.0))
            .angle(0.0, t2)
            .unwrap()
            .line(1.0, t2)
            .unwrap()
            .continue_to(p2(1.0 + l, theta * l), t2)
        {
            Ok(_) => Regime::Accept,
            Err(PathError::Escalated { .. }) => Regime::Escalate,
            Err(PathError::ContinuationTargetOffRay { .. }) => Regime::Refuse,
            Err(_) => Regime::Other,
        }
    };
    println!("R1: same angular error {theta:e} rad -> near(L=1) {near:?}, far(L=1e4) {far:?}");
}

/// **P3 — the closer at the entry: the duplicate-vertex pin is load
/// bearing.** Re-measured independently of the PR's fixture: an
/// eight-vertex subdivided square keeps eight, and if the closer ever
/// minted the entry again the loop would carry nine AND a zero-length
/// segment the data gate calls degenerate. Both halves asserted here.
#[test]
fn r1_the_closer_mints_nothing_and_leaves_no_degenerate_segment() {
    use std::f64::consts::FRAC_PI_2;
    let t = Tol::witness();
    let entry = p2(0.0, 0.0);
    let loop_ = Open
        .at(entry)
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(2.0, 0.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(2.0, 2.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(0.0, 2.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Start, t)
        .unwrap();
    let lowered = loop_.loop_.clone();
    assert_eq!(lowered.vertices().len(), 8, "no ninth vertex");
    // No consecutive duplicate anywhere on the ring (the shape the
    // DegenerateSegment refusal was reading).
    let vs: Vec<Point2<f64>> = lowered.vertices().iter().map(|v| v.pos()).collect();
    let n = vs.len();
    for i in 0..n {
        let a = vs[i];
        let b = vs[(i + 1) % n];
        let d = (b - a).norm_squared().sqrt();
        assert!(d > 1e-9, "segment {i} is degenerate: {d:e}");
    }
    Profile::new(SketchPlane::xy(), vec![lowered])
        .validate(t)
        .expect("the closed square passes the data gate");
}

/// **P4 — the open-fillet asymmetry.** `continue_to(Start)` guards an
/// open pending fillet with a typed refusal; `continue_to(point)` has no
/// such guard, and neither does `line(len)`. Probe whether the guarded
/// state is even reachable, and what the point form does if it is.
#[test]
fn r1_open_fillet_before_a_declared_continuation() {
    let t = Tol::witness();
    let after_fillet = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .fillet(0.2, t);
    match after_fillet {
        Ok(_) => println!("R1: fillet(r) from a directed point is Ok — state is Open, not NoAng"),
        Err(e) => println!("R1: fillet(r) refused: {e:?}"),
    }
}

/// **P5 — the site attribution, measured from scratch** rather than
/// read off the PR's fixture. A unit square with the bottom side
/// subdivided TWICE, seam at the second subdivision: the closer departs
/// a subdivision (so the verb applies) and lands on a straight seam.
/// Independently rebuilt with `toward` rather than `turn`.
#[test]
fn r1_site_seam_is_reachable_from_the_declared_verb() {
    let t = Tol::witness();
    // Ring: (2,0) entry, (3,0) corner, (3,3), (0,3), (0,0), (1,0) sub.
    let chain = Open
        .at(p2(2.0, 0.0))
        .toward(1.0, 0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .line(3.0, t)
        .unwrap()
        .toward(-1.0, 0.0, t)
        .unwrap()
        .line(3.0, t)
        .unwrap()
        .toward(0.0, -1.0, t)
        .unwrap()
        .line(3.0, t)
        .unwrap()
        .toward(1.0, 0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap();
    match chain.continue_to(Start, t) {
        Err(PathError::TangentLineClose { site, margin }) => {
            println!("R1: site={site:?} margin={margin:e}");
            assert_eq!(site, profile::CloseSite::Seam);
        }
        other => panic!("expected a seam refusal, got {other:?}"),
    }
}

// ==================================================================
// P6 — THE LILY PARITY WALL, attacked independently.
//
// The PR's impossibility argument: the kite's corners are its TIPS
// (even indices) and the rectangle's are its SHOULDERS (odd), the sets
// are disjoint, a loft pins ONE rotation for every section, so no
// spelling closes both. The attack below widens the search past the
// PR's own 64 rings: every vertex BUDGET from 8 to 24 reachable by
// distributing extra subdivisions over the eight base segments, both
// windings, every start, both real section widths. If any (budget,
// distribution, start, winding) closes shoulder = 0 AND shoulder = 1,
// the deviation was avoidable.
// ==================================================================

/// The demo's own section formula (`lily.rs` / `bool8_r1_probes`).
fn r1_lily_base(width: f64, ridge: f64, keel: f64, shoulder: f64) -> Vec<Point2<f64>> {
    let sh = |a: (f64, f64), b: (f64, f64)| {
        let m = (0.5 * (a.0 + b.0), 0.5 * (a.1 + b.1));
        (m.0 + shoulder * m.0, m.1 + shoulder * m.1)
    };
    let right = (0.5 * width, 0.0);
    let ridge_p = (0.0, ridge);
    let left = (-0.5 * width, 0.0);
    let keel_p = (0.0, -keel);
    [
        right,
        sh(right, ridge_p),
        ridge_p,
        sh(ridge_p, left),
        left,
        sh(left, keel_p),
        keel_p,
        sh(keel_p, right),
    ]
    .iter()
    .map(|&(x, y)| p2(x, y))
    .collect()
}

/// Insert `extra[i]` evenly-spaced vertices into base segment i. A
/// vertex added at index i lies on an EDGE of both sections (the
/// formula is affine in `shoulder`), so it shifts BOTH corner sets by
/// the same amount — which is exactly the claim under attack.
fn r1_ring_with_extras(base: &[Point2<f64>], extra: &[usize]) -> Vec<Point2<f64>> {
    let n = base.len();
    let mut out = Vec::new();
    for i in 0..n {
        out.push(base[i]);
        let a = base[i];
        let b = base[(i + 1) % n];
        for j in 1..=extra[i] {
            let f = j as f64 / (extra[i] + 1) as f64;
            out.push(p2(a.x + f * (b.x - a.x), a.y + f * (b.y - a.y)));
        }
    }
    out
}

/// Author a ring exactly as `bool8_r1_probes::try_author` does, with
/// the DECLARED closer, and require the data gate too.
fn r1_try_close(ring: &[Point2<f64>], t: Tol) -> bool {
    let n = ring.len();
    if n < 3 {
        return false;
    }
    let straight_at = |i: usize| -> bool {
        let a = ring[(i + n - 1) % n];
        let b = ring[i];
        let c = ring[(i + 1) % n];
        let d1 = b - a;
        let d2 = c - b;
        let cross = (d1.x * d2.y - d1.y * d2.x).abs();
        let l1 = d1.norm_squared().sqrt();
        let l2 = d2.norm_squared().sqrt();
        l1 > 0.0 && l2 > 0.0 && cross / (l1 * l2) < 1e-12
    };
    let dist = |a: Point2<f64>, b: Point2<f64>| (b - a).norm_squared().sqrt();
    let d0 = ring[1] - ring[0];
    let mut chain = match Open
        .at(ring[0])
        .toward(d0.x, d0.y, t)
        .and_then(|c| c.line(dist(ring[0], ring[1]), t))
    {
        Ok(c) => c,
        Err(_) => return false,
    };
    for i in 1..n - 1 {
        let len = dist(ring[i], ring[i + 1]);
        let next = if straight_at(i) {
            chain.line(len, t)
        } else {
            let d = ring[i + 1] - ring[i];
            match chain.toward(d.x, d.y, t) {
                Ok(c) => c.line(len, t),
                Err(e) => Err(e),
            }
        };
        chain = match next {
            Ok(c) => c,
            Err(_) => return false,
        };
    }
    let closed = match chain.continue_to(Start, t) {
        Ok(c) => c,
        Err(_) => return false,
    };
    Profile::new(SketchPlane::xy(), vec![closed.loop_])
        .validate(t)
        .is_ok()
}

/// The widened hunt. If it finds a (budget, distribution, start,
/// winding) that closes BOTH shoulder extremes, the PR's "lily cannot
/// migrate" is wrong.
#[test]
fn r1_hunt_a_rotation_that_closes_both_parities() {
    let t = Tol::witness();
    let mut both: Vec<String> = Vec::new();
    let mut rings = 0usize;
    // Distributions of extra subdivisions over the 8 base segments:
    // uniform 0/1/2, and every single-segment and adjacent-pair bump.
    let mut dists: Vec<[usize; 8]> = vec![[0; 8], [1; 8], [2; 8]];
    for i in 0..8 {
        let mut d = [0usize; 8];
        d[i] = 1;
        dists.push(d);
        let mut d2 = [0usize; 8];
        d2[i] = 2;
        dists.push(d2);
        let mut d3 = [0usize; 8];
        d3[i] = 1;
        d3[(i + 1) % 8] = 1;
        dists.push(d3);
        let mut d4 = [1usize; 8];
        d4[i] = 0;
        dists.push(d4);
    }
    for (w, r, k) in [(0.170, 0.028, 0.020), (0.420, 0.034, 0.016)] {
        for dist in &dists {
            let n = 8 + dist.iter().sum::<usize>();
            for start in 0..n {
                for rev in [false, true] {
                    let make = |shoulder: f64| -> Vec<Point2<f64>> {
                        let base = r1_lily_base(w, r, k, shoulder);
                        let full = r1_ring_with_extras(&base, dist);
                        let m = full.len();
                        (0..m)
                            .map(|i| {
                                let idx = if rev {
                                    (start + m - i) % m
                                } else {
                                    (start + i) % m
                                };
                                full[idx]
                            })
                            .collect()
                    };
                    rings += 1;
                    let kite = r1_try_close(&make(0.0), t);
                    let rect = r1_try_close(&make(1.0), t);
                    if kite && rect {
                        both.push(format!(
                            "w={w} extras={dist:?} n={n} start={start} rev={rev}"
                        ));
                    }
                }
            }
        }
    }
    println!(
        "R1: widened hunt — {rings} (ring, rotation) pairs; both-parity closures: {}",
        both.len()
    );
    assert!(
        both.is_empty(),
        "MAJOR: a uniform rotation closes BOTH lily parities — the migration was avoidable: {both:?}"
    );
}

/// The control: the widened hunt DOES find closures, per parity, so the
/// negative above is not a broken harness.
#[test]
fn r1_widened_hunt_positive_control() {
    let t = Tol::witness();
    let mut kite_hits = 0usize;
    let mut rect_hits = 0usize;
    let base_k = r1_lily_base(0.170, 0.028, 0.020, 0.0);
    let base_r = r1_lily_base(0.170, 0.028, 0.020, 1.0);
    for start in 0..8 {
        let rot = |b: &[Point2<f64>]| -> Vec<Point2<f64>> {
            (0..8).map(|i| b[(start + i) % 8]).collect()
        };
        if r1_try_close(&rot(&base_k), t) {
            kite_hits += 1;
        }
        if r1_try_close(&rot(&base_r), t) {
            rect_hits += 1;
        }
    }
    println!("R1: control — kite closes {kite_hits}/8 starts, rectangle {rect_hits}/8");
    assert!(
        kite_hits > 0 && rect_hits > 0,
        "the harness must find SOME closures"
    );
}

/// **P7 — the accepted band is per-LEG, so does it accumulate?** The
/// tip carries out the DECLARED ray (bitwise inherited), while the
/// emitted vertex is the authored target, which may sit up to ε off it.
/// The next leg's check measures from the NEW vertex along the SAME
/// ray, so an author who biases every target to one side moves the run
/// off the line it declared, one ε at a time, with every individual
/// check passing. Measure the drift.
#[test]
fn r1_in_band_misses_accumulate_along_a_declared_run() {
    let t = Tol::witness();
    let eps = t.eps();
    let bias = 0.5 * eps; // an accepted miss, every leg, the same side
    let n = 40usize;
    let mut chain = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap();
    for i in 0..n {
        let x = 1.0 + (i as f64 + 1.0);
        let y = (i as f64 + 1.0) * bias;
        chain = chain
            .continue_to(p2(x, y), t)
            .unwrap_or_else(|e| panic!("leg {i} refused: {e:?}"));
    }
    let closed = chain
        .line_to(p2(1.0 + n as f64, 50.0), t)
        .unwrap()
        .line_to(p2(0.0, 50.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap();
    let lowered = closed.loop_;
    let last = lowered.vertices()[n + 1].pos();
    println!(
        "R1: after {n} declared continuations each within eps={eps:e}, the run's end sits \
         {:e} m off the declared ray ({} eps)",
        last.y,
        last.y / eps
    );
    assert!(last.y > 10.0 * eps, "drift accumulated: {:e}", last.y);
    // And the data gate still takes it: nothing downstream sees the bow.
    let gate = Profile::new(SketchPlane::xy(), vec![lowered]).validate(t);
    match &gate {
        Ok(_) => println!("R1: the data gate ACCEPTS the bowed run"),
        Err(e) => println!("R1: the data gate on the bowed run -> {e}"),
    }
}

/// **P8 — D4(iv): does the new DEFINITE arm tell the same story as its
/// in-band sibling?** DESIGN D4's two-tolerance principle, consequence
/// (iv): "for every arm added to a decision, name which ε_input story
/// it belongs to, or say why it belongs to none." Print both messages a
/// user sees on either side of the escalation band.
#[test]
fn r1_the_two_sides_of_the_band_tell_two_stories() {
    let t = Tol::witness();
    let eps = t.eps();
    for (dy, label) in [
        (5.0 * eps, "in band (5 eps)"),
        (15.0 * eps, "definite (15 eps)"),
    ] {
        match attempt(dy) {
            Err(e) => println!("R1: {label}\n     {e}\n"),
            Ok(()) => println!("R1: {label} -> ACCEPTED"),
        }
    }
}
