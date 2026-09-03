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
use test_utils::vacuity;

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
/// reading, since a fire is a panic).
///
/// The bit readings carry that same posture rather than fighting it:
/// their baseline is whichever ε row certifies first, so no particular
/// band is required to certify, and where fewer than two certify the
/// row stands down by name through `test_utils::vacuity::stood_down`.
/// Both halves of that are deliberate — the baseline moves so a
/// legitimately refusing band cannot red a claim that is not about
/// postures, and the stand-down is loud so a band that quietly stopped
/// asserting ε-invariance is visible in the battery log.
#[test]
fn r2_dome_gauge_silence_and_area_bit_invariance() {
    let unit = [1.0; 9];
    let mut rational = [1.0; 9];
    rational[4] = 1.25;
    let lane_eps = Tol::witness().get().eps;

    type Reading = Result<geom_brep::props::quad::FaceCutBounds, PropsError>;
    type Key = ([u64; 9], u64);
    // Keyed on the WHOLE weight net, not on a lane flag derived from one
    // entry: two nets that differ anywhere are two different faces.
    fn at<'a>(memo: &'a mut Vec<(Key, Reading)>, w: &[f64; 9], eps: f64) -> &'a Reading {
        let key: Key = (w.map(f64::to_bits), eps.to_bits());
        let idx = match memo.iter().position(|(k, _)| *k == key) {
            Some(i) => i,
            None => {
                memo.push((key, drive(w, eps)));
                memo.len() - 1
            }
        };
        &memo[idx].1
    }
    let mut memo: Vec<(Key, Reading)> = Vec::new();

    // The ε rows this run drives, deduplicated: on the ε = 1e-6 and
    // ε = 1e-12 legs the run's own ε IS one of the literals, and the
    // memo above then pays for that drive once rather than twice.
    //
    // 1e-4 is here to keep the bit reading below LIVE on every band.
    // The dome refuses at 1e-12, so on the ε = 1e-12 leg the two
    // tighter rows leave at most one certified enclosure and the
    // reading would stand down with nothing to compare; a band loose
    // enough that the fixed area schedule always closes it gives every
    // leg a second certified enclosure. It is the cheapest row here
    // for the same reason — a loose target closes in few rounds.
    let mut eps_rows: Vec<f64> = vec![lane_eps];
    for e in [1e-4f64, 1e-6f64, 1e-12f64] {
        if !eps_rows.iter().any(|x| x.to_bits() == e.to_bits()) {
            eps_rows.push(e);
        }
    }

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

    // Readings 3 and 4 — the calibration's ε-invariance premise, per
    // lane: the area enclosure is built BEFORE the round loop, so its
    // bits cannot move with ε.
    //
    // The baseline is taken from an ε at which the dome CERTIFIES,
    // never from a fixed one: Readings 1 and 2 above treat a budget
    // refusal as an honest posture, and a baseline that demanded a
    // certified return at one particular ε would contradict them —
    // one legitimately refusing band would take down all four
    // readings. When fewer than two ε rows certify there is no second
    // enclosure to compare against, and the row stands down BY NAME
    // through the tree's door rather than asserting nothing in
    // silence or reddening on a posture that is not its subject.
    for (lane, w) in [("integral lane", &unit), ("rational lane", &rational)] {
        let mut certified: Vec<(f64, (u64, u64))> = Vec::new();
        for eps in &eps_rows {
            if let Ok(fb) = at(&mut memo, w, *eps) {
                certified.push((*eps, (fb.area.lo().to_bits(), fb.area.hi().to_bits())));
            }
        }
        if certified.len() < 2 {
            vacuity::stood_down(
                &format!("r2 dome area-bit ε-invariance, {lane}"),
                &format!(
                    "the dome certifies at {} of the {} ε rows this run drives, so there is \
                     no second certified enclosure to compare against — the ε-invariance of \
                     the area enclosure is NOT asserted for this lane on this band",
                    certified.len(),
                    eps_rows.len()
                ),
            );
            continue;
        }
        let (e0, bits0) = certified[0];
        for (e, bits) in &certified[1..] {
            assert_eq!(
                bits0, *bits,
                "{lane}: the area enclosure bits moved between ε = {e0:e} and ε = {e:e}, but \
                 the enclosure is built before the round loop and cannot depend on ε"
            );
        }
        println!(
            "R2 dome {lane}: area bits identical across {} certified ε rows",
            certified.len()
        );
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
