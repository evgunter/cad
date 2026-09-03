//! **R2 review probes for CERT-6 (PR 1366, frozen head ce567bad).**
//!
//! Own fixtures through the props public door `nurbs_patch_face`:
//! honest faces at both patch lanes (the gauge must stay silent), the
//! ε bit-identity of the area enclosure the calibration's ε-invariance
//! claim rests on, and the fallback-arm reachability question — can a
//! face whose whole rectangle boundary collapses to a point reach a
//! certified return (PR body: "per-face, not per-lane" fallback with
//! zero corpus coverage).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::props::PropsError;
use geom_brep::props::quad::nurbs_patch_face;
use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;
use geom_core::{Band, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn pt(x: f64) -> RingInterval {
    RingInterval::from_bounds(x, x)
}

fn p3(x: f64, y: f64, z: f64) -> [RingInterval; 3] {
    [pt(x), pt(y), pt(z)]
}

/// A mildly curved single-span biquadratic dome, my own authoring.
fn dome() -> (KnotVector, KnotVector, Vec<[RingInterval; 3]>) {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut net = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            #[allow(clippy::cast_precision_loss)]
            let (x, y) = (i as f64, j as f64);
            let z = if i == 1 && j == 1 { 0.9 } else { 0.0 };
            net.push(p3(x, y, z));
        }
    }
    (k.clone(), k, net)
}

fn drive(weights: &[f64], eps: f64) -> Result<geom_brep::props::quad::FaceCutBounds, PropsError> {
    let (ku, kv, net) = dome();
    let (a, b) = ku.domain();
    let (c, d) = kv.domain();
    nurbs_patch_face::<f64>(&ku, &kv, &net, weights, (a, b, c, d), 8.0, 0.0, eps, band())
}

/// Everything this dome is read for, in ONE row: the gauge's silence
/// at both patch lanes, and the ε-invariance of the area enclosure's
/// bits.
///
/// One row rather than three because nextest is process-per-test, so
/// three rows drive the same dome at overlapping ε and each pays in
/// full; `at` below evaluates each distinct (lane, ε) pair once, which
/// also means a leg whose own ε equals one of the literals here pays
/// for that drive once instead of twice. Every assertion names the
/// reading it makes.
///
/// **ε posture.** The premise of the gauge readings is per band, not
/// unconditional: the fixed area schedule and the ε-scaled flux target
/// mean a tighter band legitimately refuses at the round budget — the
/// superset direction the unit's posture argument names, and observed
/// live on the rational lane at ε = 1e-12. Either outcome is honest;
/// what the gauge readings check is that the GAUGE stayed silent,
/// which is true of a refusal too (reaching the line at all is the
/// reading, since a fire is a panic). The bit readings are taken only
/// where the face certifies, for the same reason.
#[test]
fn r2_dome_gauge_silence_and_area_bit_invariance() {
    let unit = [1.0; 9];
    let mut rational = [1.0; 9];
    rational[4] = 1.25;
    let lane_eps = Tol::witness().get().eps;

    type Reading = Result<geom_brep::props::quad::FaceCutBounds, PropsError>;
    fn at<'a>(memo: &'a mut Vec<((bool, u64), Reading)>, w: &[f64; 9], eps: f64) -> &'a Reading {
        let key = (w[4] != 1.0, eps.to_bits());
        let idx = match memo.iter().position(|(k, _)| *k == key) {
            Some(i) => i,
            None => {
                memo.push((key, drive(w, eps)));
                memo.len() - 1
            }
        };
        &memo[idx].1
    }
    let mut memo: Vec<((bool, u64), Reading)> = Vec::new();

    // Reading 1 — integral (unit-weight) lane at the run's own ε: an
    // honest face and a silent gauge.
    match at(&mut memo, &unit, lane_eps) {
        Ok(out) => {
            println!(
                "R2 nurbs dome: area [{:e},{:e}] width {:e}",
                out.area.lo(),
                out.area.hi(),
                out.area.width()
            );
            assert!(
                out.area.width().is_finite() && out.area.lo() > 0.0,
                "integral-lane gauge reading: a certified dome must return a finite, \
                 strictly positive area enclosure, got [{:e},{:e}]",
                out.area.lo(),
                out.area.hi()
            );
        }
        Err(PropsError::QuadratureBudget { .. }) => {
            println!("R2 nurbs dome: honest budget refusal at this band — gauge silent");
        }
        Err(e) => panic!("the mild dome should certify or refuse at the budget, got {e}"),
    }

    // Reading 2 — rational lane at the run's own ε: non-unit weights
    // route to `rational_patch_face`, so a posture here means that
    // arm's gauge (and its `Ladder::point` perimeter) ran silently.
    match at(&mut memo, &rational, lane_eps) {
        Ok(out) => {
            println!(
                "R2 rational dome: area [{:e},{:e}] width {:e}",
                out.area.lo(),
                out.area.hi(),
                out.area.width()
            );
            assert!(
                out.area.width().is_finite() && out.area.lo() > 0.0,
                "rational-lane gauge reading: a certified dome must return a finite, \
                 strictly positive area enclosure, got [{:e},{:e}]",
                out.area.lo(),
                out.area.hi()
            );
        }
        Err(PropsError::QuadratureBudget { .. }) => {
            println!("R2 rational dome: honest budget refusal at this band — gauge silent");
        }
        Err(e) => panic!("the mild rational dome should certify or refuse at the budget, got {e}"),
    }

    // Reading 3 — the calibration's ε-invariance premise: the area
    // enclosure is built BEFORE the round loop, so its bits cannot
    // move with ε. Integral lane, against the run's ε as the baseline.
    let (a_lo, a_hi) = {
        let a = at(&mut memo, &unit, lane_eps)
            .as_ref()
            .expect("integral-lane ε-invariance baseline: the dome certifies at the run's ε");
        (a.area.lo().to_bits(), a.area.hi().to_bits())
    };
    {
        let b = at(&mut memo, &unit, 1e-6)
            .as_ref()
            .expect("integral-lane ε-invariance: the dome certifies at ε = 1e-6");
        assert_eq!(
            (a_lo, a_hi),
            (b.area.lo().to_bits(), b.area.hi().to_bits()),
            "integral-lane area enclosure bits moved between the run's ε and 1e-6"
        );
    }
    match at(&mut memo, &unit, 1e-12) {
        Ok(c) => assert_eq!(
            (a_lo, a_hi),
            (c.area.lo().to_bits(), c.area.hi().to_bits()),
            "integral-lane area enclosure bits moved between the run's ε and 1e-12"
        ),
        Err(e) => println!("R2 eps=1e-12: honest refusal ({e}) — superset direction observed"),
    }

    // Reading 4 — the same premise on the rational lane.
    let (r_lo, r_hi) = {
        let ra = at(&mut memo, &rational, 1e-6)
            .as_ref()
            .expect("rational-lane ε-invariance baseline: the dome certifies at ε = 1e-6");
        (ra.area.lo().to_bits(), ra.area.hi().to_bits())
    };
    match at(&mut memo, &rational, 1e-12) {
        Ok(rb) => assert_eq!(
            (r_lo, r_hi),
            (rb.area.lo().to_bits(), rb.area.hi().to_bits()),
            "rational-lane area enclosure bits moved between ε = 1e-6 and ε = 1e-12"
        ),
        Err(e) => println!("R2 rational eps=1e-12: honest refusal ({e})"),
    }
}

/// Perimeter-bound soundness anchor: a FLAT unit square patch has true
/// boundary perimeter exactly 4. The certified lower bound must sit at
/// or below 4 and, with straight edges (chords lie ON the curve),
/// within ring rounding of it. Read via the R2_GAUGE_TRACE line; this
/// row asserts through the door only that the face certifies.
#[test]
fn r2_flat_square_perimeter_anchor() {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut net = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            #[allow(clippy::cast_precision_loss)]
            let (x, y) = (i as f64 / 2.0, j as f64 / 2.0);
            net.push(p3(x, y, 0.0));
        }
    }
    let out = nurbs_patch_face::<f64>(
        &k,
        &k,
        &net,
        &[1.0; 9],
        (0.0, 1.0, 0.0, 1.0),
        4.0,
        0.0,
        Tol::witness().get().eps,
        band(),
    )
    .expect("the flat square certifies");
    println!(
        "R2 flat square: area [{:e},{:e}] (true 1)",
        out.area.lo(),
        out.area.hi()
    );
}

/// Claim 5's reachability question: a "balloon" patch whose ENTIRE
/// rectangle boundary maps to one point (all boundary control points
/// coincide), with real interior area. If this reaches a certified
/// return, the relative-gauge fallback arm is live; if it refuses,
/// the fallback is dead code at today's call sites. The probe records
/// the posture rather than asserting one.
#[test]
fn r2_collapsed_boundary_balloon_probe() {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let n = k.control_count(); // 4
    let mut net = Vec::new();
    for i in 0..n {
        for j in 0..n {
            let boundary = i == 0 || j == 0 || i == n - 1 || j == n - 1;
            if boundary {
                net.push(p3(0.0, 0.0, 0.0));
            } else {
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = (i as f64 - 1.5, j as f64 - 1.5);
                net.push(p3(3.0 * x, 3.0 * y, 2.0));
            }
        }
    }
    let (a, b) = k.domain();
    let out = nurbs_patch_face::<f64>(
        &k,
        &k,
        &net,
        &vec![1.0; n * n],
        (a, b, a, b),
        0.0,
        0.0,
        Tol::witness().get().eps,
        band(),
    );
    match out {
        Ok(fb) => println!(
            "R2 balloon: CERTIFIED, area [{:e},{:e}] width {:e} — fallback arm was LIVE if the \
             chord perimeter was 0",
            fb.area.lo(),
            fb.area.hi(),
            fb.area.width()
        ),
        Err(e) => println!("R2 balloon: refused {e} — fallback not reached through this shape"),
    }
}
