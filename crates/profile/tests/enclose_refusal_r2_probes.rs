//! BLEND-7 (#1267) independent review probes — R2 lane.
//!
//! Written as an OUTSIDE CONSUMER: everything here goes through the
//! public `profile` doors (`Open.arc_fillet_arc`, `Open.arc_fillet`),
//! never through `sugar`'s internals or `review_s2`'s bracketing
//! harness. The point is to see what an author actually gets.
//!
//! These are probes, not pins: most of them PRINT a measurement and
//! assert only the property under examination.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point2;
use geom_core::Tol;
use profile::{ArcSweep, Center, Open, PathError, ProfileLoop, Start};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A corner where two circular carriers cross, authored through the
/// public arc→fillet→arc door. `a_in` / `a_out` are the corner's angle
/// about each carrier centre; the anchors sit `delta` radians back
/// along each leg, so a small `delta` brackets the drawn corner.
#[allow(clippy::too_many_arguments)]
fn arc_arc_corner(
    corner: Point2<f64>,
    a_in: f64,
    r_in: f64,
    tau_in: f64,
    a_out: f64,
    r_out: f64,
    tau_out: f64,
    delta: f64,
    fillet: f64,
) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let c_in = p2(corner.x - r_in * a_in.cos(), corner.y - r_in * a_in.sin());
    let c_out = p2(corner.x - r_out * a_out.cos(), corner.y - r_out * a_out.sin());
    let head_a = a_in - tau_in * delta;
    let next_a = a_out + tau_out * delta;
    let head = p2(
        c_in.x + r_in * head_a.cos(),
        c_in.y + r_in * head_a.sin(),
    );
    let next = p2(
        c_out.x + r_out * next_a.cos(),
        c_out.y + r_out * next_a.sin(),
    );
    let wind = |tau: f64| {
        if tau > 0.0 {
            ArcSweep::Ccw
        } else {
            ArcSweep::Cw
        }
    };
    Open.arc_fillet_arc(
        Center {
            c: c_in,
            winding: wind(tau_in),
            p: head,
        },
        fillet,
        Center {
            c: c_out,
            winding: wind(tau_out),
            p: next,
        },
        Tol::witness(),
    )?
    .line_to(Start, Tol::witness())
    .map(|closed| closed.loop_)
}

/// Recover a segment's circle from the stored chord + bulge.
fn circle_from_bulge(t1: Point2<f64>, t2: Point2<f64>, b: f64) -> (Point2<f64>, f64) {
    let (cx, cy) = (t2.x - t1.x, t2.y - t1.y);
    let l = cx.hypot(cy);
    let d = l / 2.0;
    let radius = d * (1.0 + b * b) / (2.0 * b.abs());
    let h = radius - d * (2.0 * b.abs() / (1.0 + b * b)) * (1.0 + b * b) / (2.0 * b.abs())
        + (radius * radius - d * d).max(0.0).sqrt() * 0.0;
    let sag = radius - (radius * radius - d * d).max(0.0).sqrt();
    let _ = h;
    let (ux, uy) = (cx / l, cy / l);
    let (nx, ny) = (-uy, ux);
    let s = if b > 0.0 { -1.0 } else { 1.0 };
    let mid = Point2::new(t1.x + cx / 2.0, t1.y + cy / 2.0);
    let centre = Point2::new(
        mid.x + s * nx * (radius - sag),
        mid.y + s * ny * (radius - sag),
    );
    (centre, radius)
}

/// The two points where the pair of carriers cross. One is the drawn
/// corner (both carriers pass through it by construction); the other is
/// its mirror across the centre line — and the §2c lattice sees BOTH.
fn crossings(
    corner: Point2<f64>,
    o1: Point2<f64>,
    o2: Point2<f64>,
) -> [Point2<f64>; 2] {
    let (dx, dy) = (o2.x - o1.x, o2.y - o1.y);
    let l = dx.hypot(dy);
    let (ux, uy) = (dx / l, dy / l);
    let (vx, vy) = (corner.x - o1.x, corner.y - o1.y);
    let t = vx * ux + vy * uy;
    let foot = Point2::new(o1.x + t * ux, o1.y + t * uy);
    [
        corner,
        Point2::new(2.0 * foot.x - corner.x, 2.0 * foot.y - corner.y),
    ]
}

/// σ and both signed offset radii ρ = R − σ·τ·r at one crossing,
/// re-derived from the drawn geometry (travel dir on an arc leg at
/// angle `a` is τ·(−sin a, cos a)).
fn sigma_rho_at(
    at: Point2<f64>,
    o1: Point2<f64>,
    r1: f64,
    tau1: f64,
    o2: Point2<f64>,
    r2: f64,
    tau2: f64,
    fillet: f64,
) -> Option<(f64, f64, f64)> {
    let a1 = (at.y - o1.y).atan2(at.x - o1.x);
    let a2 = (at.y - o2.y).atan2(at.x - o2.x);
    let d_in = (-tau1 * a1.sin(), tau1 * a1.cos());
    let d_out = (-tau2 * a2.sin(), tau2 * a2.cos());
    let cross = d_in.0 * d_out.1 - d_in.1 * d_out.0;
    if cross.abs() < 1e-9 {
        return None;
    }
    let sigma = cross.signum();
    Some((
        sigma,
        r1 - sigma * tau1 * fillet,
        r2 - sigma * tau2 * fillet,
    ))
}

/// Does the emitted loop contain a fillet arc of radius `r` that
/// SWALLOWS one of the leg carriers? That is the ruling's own property
/// (`|P − O| + R < r`), checked on the emitted geometry rather than on
/// any bookkeeping.
fn emitted_swallows_a_carrier(
    lp: &ProfileLoop<f64>,
    r: f64,
    carriers: &[(Point2<f64>, f64)],
) -> Option<String> {
    let (t1, t2) = fillet_endpoints(lp, r)?;
    // Both perpendicular-bisector candidates; the true centre is the one
    // consistent with tangency to both carriers.
    let mid = Point2::new((t1.x + t2.x) / 2.0, (t1.y + t2.y) / 2.0);
    let (cx, cy) = (t2.x - t1.x, t2.y - t1.y);
    let l = cx.hypot(cy);
    let half = l / 2.0;
    let ap = (r * r - half * half).max(0.0).sqrt();
    let (nx, ny) = (-cy / l, cx / l);
    for s in [1.0_f64, -1.0] {
        let p = Point2::new(mid.x + s * nx * ap, mid.y + s * ny * ap);
        let tangent_to_all = carriers.iter().all(|(o, big)| {
            let d = (p.x - o.x).hypot(p.y - o.y);
            (d - (big - r).abs()).abs() < 1e-7 || (d - (big + r)).abs() < 1e-7
        });
        if !tangent_to_all {
            continue;
        }
        for (o, big) in carriers {
            let d = (p.x - o.x).hypot(p.y - o.y);
            if d + big < r - 1e-9 {
                return Some(format!(
                    "|P-O| + R = {} < r = {r} (carrier R = {big})",
                    d + big
                ));
            }
        }
        return None;
    }
    None
}

/// The emitted fillet arc's two endpoints, located by recovered radius.
fn fillet_endpoints(lp: &ProfileLoop<f64>, r: f64) -> Option<(Point2<f64>, Point2<f64>)> {
    let n = lp.vertices().len();
    for i in 0..n {
        let a = lp.vertices()[i];
        let b = lp.vertices()[(i + 1) % n];
        if a.bulge() == 0.0 {
            continue;
        }
        let (_, rf) = circle_from_bulge(a.pos(), b.pos(), a.bulge());
        if (rf - r).abs() < 1e-6 * r.max(1.0) {
            return Some((a.pos(), b.pos()));
        }
    }
    None
}

/// One-word outcome label for a sweep row.
fn label(res: &Result<ProfileLoop<f64>, PathError<f64>>) -> String {
    match res {
        Ok(_) => "BUILDS".to_string(),
        Err(PathError::FilletEnclosesLegCarrier {
            carrier_radius,
            offset_radius,
            ..
        }) => format!("ENCLOSING(R={carrier_radius}, rho={offset_radius})"),
        Err(e) => {
            let d = format!("{e:?}");
            d.split('{').next().unwrap_or("?").trim().to_string()
        }
    }
}

/// **P1 — the existence GAP, and what the refusal's recourse names.**
///
/// `enclosing_cases` row 1: equal carriers R = 0.2 whose centres sit
/// d = 0.2828 apart, a right-angle turn, σ = τ = +1. The largest
/// ordinary fillet is (R₁+R₂−d)/2 = 0.0586. At r = 0.5 the door now
/// refuses `FilletEnclosesLegCarrier` and its message says *"use a
/// radius below that side's carrier radius (0.2 m)"*.
///
/// This probe asks whether that sentence names radii that BUILD.
#[test]
fn p1_the_recourse_bound_against_the_measured_existence_gap() {
    let corner = p2(0.0, 0.0);
    let big = 0.5;
    let refusal = arc_arc_corner(
        corner,
        0.0,
        0.2,
        1.0,
        core::f64::consts::FRAC_PI_2,
        0.2,
        1.0,
        0.6,
        big,
    )
    .unwrap_err();
    println!("P1 at r = {big}: {}", label(&Err(refusal.clone())));
    println!("P1 message: {refusal}");
    let PathError::FilletEnclosesLegCarrier { carrier_radius, .. } = refusal else {
        panic!("expected the enclosing refusal at r = {big}, got {refusal:?}");
    };
    println!("P1 the recourse names: a radius below {carrier_radius}");

    // Now walk DOWN from the named bound and record what an author who
    // followed that sentence actually gets.
    let mut built = Vec::new();
    let mut refused = Vec::new();
    for step in 1..40 {
        let r = carrier_radius * f64::from(step) / 40.0;
        let res = arc_arc_corner(
            corner,
            0.0,
            0.2,
            1.0,
            core::f64::consts::FRAC_PI_2,
            0.2,
            1.0,
            0.6,
            r,
        );
        println!("P1   r = {r:.5} -> {}", label(&res));
        if res.is_ok() {
            built.push(r);
        } else {
            refused.push(r);
        }
    }
    println!(
        "P1 SUMMARY: below the named bound {carrier_radius}, {} of {} sampled radii BUILD; \
         {} refuse",
        built.len(),
        built.len() + refused.len(),
        refused.len()
    );
    if let (Some(lo), Some(hi)) = (built.last(), refused.iter().find(|r| *r > built.last().unwrap_or(&0.0)))
    {
        println!("P1 GAP: ordinary fillets end near {lo:.5}; {hi:.5} is already refused, and the \
                  message's bound is {carrier_radius}");
    }
    // The probe's own assertion: SOME radius below the named bound must
    // build, or the recourse is empty.
    assert!(
        !built.is_empty(),
        "the recourse names a bound below which nothing builds"
    );
}

/// **P2 — false positive hunt: is any ORDINARY request now refused?**
///
/// The gate fires on ρ = R − σ·τ·r < 0 for either circular leg. This
/// sweeps a wide grid of arc×arc corners and asserts the new variant
/// appears ONLY where the probe's own independently computed ρ is
/// negative — and, conversely, that a ρ > 0 corner never receives it.
#[test]
fn p2_the_gate_fires_only_where_a_crossing_of_the_pair_is_enclosing() {
    let mut checked = 0_u32;
    let mut enclosing_refusals = 0_u32;
    let mut builds = 0_u32;
    let mut mismatched = 0_u32;
    let mut attributed_to_the_other_crossing = 0_u32;
    for (i, &r_in) in [0.15_f64, 0.4, 1.0, 2.5].iter().enumerate() {
        for &r_out in &[0.2_f64, 0.55, 1.3, 3.0] {
            for &tau_in in &[1.0_f64, -1.0] {
                for &tau_out in &[1.0_f64, -1.0] {
                    for k in 0..7 {
                        let a_out = 0.4 + 0.7 * f64::from(k);
                        for m in 0..9 {
                            let fillet = 0.05 + 0.35 * f64::from(m);
                            let corner = p2(0.3 * i as f64, -0.2);
                            let o1 = p2(corner.x - r_in, corner.y);
                            let o2 = p2(
                                corner.x - r_out * a_out.cos(),
                                corner.y - r_out * a_out.sin(),
                            );
                            if (o2.x - o1.x).hypot(o2.y - o1.y) < 1e-9 {
                                continue;
                            }
                            let res = arc_arc_corner(
                                corner, 0.0, r_in, tau_in, a_out, r_out, tau_out, 0.9, fillet,
                            );
                            let xs = crossings(corner, o1, o2);
                            let facts: Vec<(f64, f64, f64)> = xs
                                .iter()
                                .filter_map(|x| {
                                    sigma_rho_at(
                                        *x, o1, r_in, tau_in, o2, r_out, tau_out, fillet,
                                    )
                                })
                                .collect();
                            if facts.is_empty() {
                                continue;
                            }
                            checked += 1;
                            let any_enclosing =
                                facts.iter().any(|(_, a, b)| *a < 0.0 || *b < 0.0);
                            let drawn_enclosing = facts
                                .first()
                                .is_some_and(|(_, a, b)| *a < 0.0 || *b < 0.0);
                            match &res {
                                Err(PathError::FilletEnclosesLegCarrier {
                                    offset_radius,
                                    ..
                                }) => {
                                    enclosing_refusals += 1;
                                    // FALSIFIER: the gate must be
                                    // classifying a real crossing of
                                    // this pair, and a negative rho.
                                    assert!(*offset_radius < 0.0, "the payload rho is not negative");
                                    let matches_a_crossing = facts.iter().any(|(_, a, b)| {
                                        (a - offset_radius).abs() < 1e-9
                                            || (b - offset_radius).abs() < 1e-9
                                    });
                                    if !matches_a_crossing {
                                        mismatched += 1;
                                        println!(
                                            "P2 UNMATCHED rho {offset_radius}: crossings say \
                                             {facts:?} (r_in {r_in}, r_out {r_out}, tau \
                                             {tau_in}/{tau_out}, a_out {a_out}, r {fillet})"
                                        );
                                    }
                                    assert!(
                                        any_enclosing,
                                        "FALSE POSITIVE: enclosing refusal where NEITHER \
                                         crossing of the pair is enclosing ({facts:?}; r_in \
                                         {r_in}, r_out {r_out}, tau {tau_in}/{tau_out}, \
                                         a_out {a_out}, r {fillet})"
                                    );
                                    if !drawn_enclosing {
                                        attributed_to_the_other_crossing += 1;
                                    }
                                }
                                Ok(lp) => {
                                    builds += 1;
                                    if let Some(bad) = emitted_swallows_a_carrier(
                                        lp,
                                        fillet,
                                        &[(o1, r_in), (o2, r_out)],
                                    ) {
                                        panic!(
                                            "RULING VIOLATED: the door emitted an enclosing \
                                             tangency — {bad} (r_in {r_in}, r_out {r_out}, \
                                             tau {tau_in}/{tau_out}, a_out {a_out}, r \
                                             {fillet})"
                                        );
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
            }
        }
    }
    println!(
        "P2: {checked} corners; {enclosing_refusals} enclosing refusals ({mismatched} whose rho \
         matched NEITHER crossing), {builds} builds, none swallowing a carrier"
    );
    println!(
        "P2 ATTRIBUTION: {attributed_to_the_other_crossing} of {enclosing_refusals} enclosing \
         refusals describe the pair's OTHER crossing, not the corner the anchors were drawn \
         around"
    );
    assert_eq!(mismatched, 0, "a refusal named a rho no crossing has");
    assert!(enclosing_refusals > 0, "the sweep never reached the class");
    assert!(builds > 0, "the sweep never built anything");
}

/// **P3 — geometry the PR's 60×60×3 anchor grid could not reach.**
///
/// The PR's zero-emissions claim rests on a grid over row-1's carriers
/// near the origin at one radius. This attacks with what that
/// distribution cannot produce: corners far from the origin, scale
/// ratios past 15×, and near-tangent (hairline-lens) carrier pairs.
/// The property asserted is the ruling itself — no emitted fillet may
/// swallow a leg carrier.
#[test]
fn p3_no_emitted_fillet_swallows_a_carrier_off_the_prs_grid() {
    let mut emitted = 0_u32;
    let mut enclosing_refusals = 0_u32;
    let mut checked = 0_u32;
    // (label, corner, r_in, r_out)
    let scenes: [(&str, Point2<f64>, f64, f64); 6] = [
        ("far from origin", p2(1400.0, -930.0), 0.2, 0.2),
        ("very far from origin", p2(85000.0, 40000.0), 0.5, 0.7),
        ("scale ratio 40x", p2(0.0, 0.0), 0.05, 2.0),
        ("scale ratio 200x", p2(0.0, 0.0), 0.01, 2.0),
        ("large absolute scale", p2(0.0, 0.0), 120.0, 300.0),
        ("tiny absolute scale", p2(0.0, 0.0), 1e-3, 3e-3),
    ];
    for (name, corner, r_in, r_out) in scenes {
        for &tau_in in &[1.0_f64, -1.0] {
            for &tau_out in &[1.0_f64, -1.0] {
                for k in 0..15 {
                    // a_out sweeps toward the near-tangent (hairline
                    // lens) configuration at the small end.
                    let a_out = 0.02 + 0.21 * f64::from(k);
                    for m in 0..13 {
                        let fillet = r_in.min(r_out) * 0.2 * f64::from(m + 1);
                        for &delta in &[0.35_f64, 1.2, 3.0] {
                            let o1 = p2(corner.x - r_in, corner.y);
                            let o2 = p2(
                                corner.x - r_out * a_out.cos(),
                                corner.y - r_out * a_out.sin(),
                            );
                            if (o2.x - o1.x).hypot(o2.y - o1.y) < 1e-12 {
                                continue;
                            }
                            let res = arc_arc_corner(
                                corner, 0.0, r_in, tau_in, a_out, r_out, tau_out, delta, fillet,
                            );
                            checked += 1;
                            match &res {
                                Ok(lp) => {
                                    emitted += 1;
                                    // The ruling, checked on the EMITTED
                                    // geometry: no bookkeeping involved.
                                    if let Some(bad) = emitted_swallows_a_carrier(
                                        lp,
                                        fillet,
                                        &[(o1, r_in), (o2, r_out)],
                                    ) {
                                        panic!(
                                            "{name}: RULING VIOLATED — {bad} (a_out {a_out}, \
                                             delta {delta}, tau {tau_in}/{tau_out})"
                                        );
                                    }
                                }
                                Err(PathError::FilletEnclosesLegCarrier {
                                    offset_radius,
                                    ..
                                }) => {
                                    enclosing_refusals += 1;
                                    assert!(
                                        *offset_radius < 0.0,
                                        "{name}: the payload rho is not negative"
                                    );
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
            }
        }
    }
    println!(
        "P3: {checked} off-grid corners; {emitted} fillets emitted, none swallowing a carrier; \
         {enclosing_refusals} enclosing refusals"
    );
    assert!(emitted > 0, "the off-grid sweep built nothing at all");
    assert!(
        enclosing_refusals > 0,
        "the off-grid sweep never reached the class"
    );
}

/// **P4 — the header's line-partner claim, exercised at the door.**
///
/// `review_s2`'s header states that a swallowed carrier "never appears
/// beside a line leg — geometrically impossible, not merely rare".
/// That is a claim about a swallowing SOLUTION existing, not about the
/// gate's input: a line×arc corner whose arc leg has ρ < 0 is perfectly
/// drawable, and the gate must fire on it. This probe draws one through
/// the public line-partner door and reports what comes back.
#[test]
fn p4_a_line_partner_corner_with_a_negative_rho_arc_leg() {
    let corner = p2(0.0, 0.0);
    let r_arc = 0.25;
    let c_arc = p2(corner.x - r_arc, corner.y);
    // Incoming along the arc (ccw, tau = +1), leaving along a straight
    // leg. The fillet radius is far above the carrier radius.
    for &fillet in &[0.05_f64, 0.2, 0.6, 1.5] {
        let head_a = -1.0_f64;
        let head = p2(
            c_arc.x + r_arc * head_a.cos(),
            c_arc.y + r_arc * head_a.sin(),
        );
        for &(dx, dy) in &[(-1.0_f64, 0.4_f64), (-1.0, -0.4), (1.0, 0.6)] {
            let res = Open
                .arc_fillet(
                    Center {
                        c: c_arc,
                        winding: ArcSweep::Ccw,
                        p: head,
                    },
                    fillet,
                    Tol::witness(),
                )
                .and_then(|b| b.at(p2(dx, dy), Tol::witness()))
                .and_then(|b| b.toward(dx, dy, Tol::witness()))
                .and_then(|b| b.line(0.25, Tol::witness()))
                .and_then(|b| b.line_to(Start, Tol::witness()))
                .map(|closed| closed.loop_);
            println!(
                "P4 r = {fillet}, dir ({dx}, {dy}) -> {}",
                label(&res)
            );
        }
    }
}

/// **P5 — the non-abort design, judged as geometry.**
///
/// With long (unbracketed) legs the pair's OTHER crossing is inside the
/// windows, and the PR reports four of six rows building an ordinary
/// fillet there. This probe measures HOW FAR the emitted fillet sits
/// from the corner the author's two anchors bracket, so the answer's
/// honesty can be judged rather than assumed.
#[test]
fn p5_what_the_other_crossing_serves_and_how_far_away_it_is() {
    let corner = p2(0.0, 0.0);
    for &delta in &[0.6_f64, 2.0, 4.0] {
        let res = arc_arc_corner(
            corner,
            0.0,
            0.2,
            1.0,
            core::f64::consts::FRAC_PI_2,
            0.2,
            1.0,
            delta,
            0.5,
        );
        match &res {
            Ok(lp) => {
                // Where did the emitted fillet actually land relative to
                // the corner the author's two anchors bracket?
                match fillet_endpoints(lp, 0.5) {
                    Some((t1, t2)) => {
                        let d1 = (t1.x - corner.x).hypot(t1.y - corner.y);
                        let d2 = (t2.x - corner.x).hypot(t2.y - corner.y);
                        println!(
                            "P5 delta = {delta}: BUILDS; the emitted fillet's tangent points \
                             are {d1:.4} m and {d2:.4} m from the DRAWN corner"
                        );
                    }
                    None => println!(
                        "P5 delta = {delta}: BUILDS, but no segment recovers r = 0.5 \
                         (vertices {})",
                        lp.vertices().len()
                    ),
                }
            }
            Err(e) => println!("P5 delta = {delta}: {}", label(&Err(e.clone()))),
        }
    }
}

/// **P6 — is `NoCornerSideCandidate` still reachable at all?**
///
/// This PR re-points the ONLY test in the workspace that asserted the
/// variant (`arc_fillet::an_arc_arc_corner_can_have_no_corner_side_
/// candidate`, the sole hit at the merge base). The PR says the reason
/// "keeps its line×arc route" but does not exhibit one. This probe
/// searches for a line×arc witness through the public doors.
#[test]
fn p6_hunt_a_surviving_no_corner_side_candidate_witness() {
    use profile::path::PathNoCornerReason;
    use profile::NoCornerReason;
    let mut hits = 0_u32;
    let mut tried = 0_u32;
    let mut seen: Vec<String> = Vec::new();
    // Arc incoming, straight leg outgoing.
    for &r_arc in &[0.15_f64, 0.4, 1.0, 2.2] {
        for &tau in &[1.0_f64, -1.0] {
            for hd in 0..12 {
                let head_a = -2.6 + 0.45 * f64::from(hd);
                for ld in 0..24 {
                    let ang = -3.0 + 0.26 * f64::from(ld);
                    for &fillet in &[0.02_f64, 0.07, 0.2, 0.5, 0.9, 1.6] {
                        for &reach in &[0.3_f64, 1.0, 2.5] {
                            let corner = p2(0.0, 0.0);
                            let c_arc = p2(corner.x - r_arc, corner.y);
                            let head =
                                p2(c_arc.x + r_arc * head_a.cos(), c_arc.y + r_arc * head_a.sin());
                            let (dx, dy) = (ang.cos(), ang.sin());
                            let away = p2(corner.x + reach * dx, corner.y + reach * dy);
                            tried += 1;
                            let res = Open
                                .arc_fillet(
                                    Center {
                                        c: c_arc,
                                        winding: if tau > 0.0 {
                                            ArcSweep::Ccw
                                        } else {
                                            ArcSweep::Cw
                                        },
                                        p: head,
                                    },
                                    fillet,
                                    Tol::witness(),
                                )
                                .and_then(|b| b.at(away, Tol::witness()))
                                .and_then(|b| b.toward(dx, dy, Tol::witness()))
                                .and_then(|b| b.line(0.3, Tol::witness()))
                                .and_then(|b| b.line_to(Start, Tol::witness()));
                            if let Err(PathError::NoCornerForFillet {
                                reason:
                                    PathNoCornerReason::NoTangentCircle(
                                        NoCornerReason::NoCornerSideCandidate,
                                    ),
                                ..
                            }) = res
                            {
                                hits += 1;
                                if seen.len() < 3 {
                                    seen.push(format!(
                                        "R={r_arc} tau={tau} head_a={head_a} ang={ang} \
                                         r={fillet} reach={reach}"
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("P6: {tried} line x arc corners tried; {hits} NoCornerSideCandidate hits");
    for s in &seen {
        println!("P6   witness: {s}");
    }
}

/// **P7 — is the NAMED bound the RIGHT bound when the carriers differ?**
///
/// `arc_fillet::an_arc_arc_radius_larger_than_both_carriers_refuses_as_
/// the_enclosing_class`'s geometry: R₁ = 1 about (0, −1), R₂ = 1/2
/// about (1/2, 0), corner at the origin, both CCW, σ = +1. At r = 2
/// both ρ are negative and the refusal names the INCOMING side, so its
/// message reads "use a radius below that side's carrier radius (1 m)".
///
/// But ρ₂ = 1/2 − r stays negative for every r above 1/2. The bound
/// that actually leaves the class is min(R₁, R₂), and the message names
/// max-of-the-first-hit instead. This probe walks the radii the message
/// endorses.
#[test]
fn p7_the_named_bound_on_a_corner_whose_carriers_differ() {
    let along = |cx: f64, cy: f64, r: f64, delta: f64| {
        p2(cx + r * delta.cos(), cy + r * delta.sin())
    };
    // Anchors one radian along each carrier, as the fixture draws them.
    let build = |r: f64| {
        Open.arc_fillet_arc(
            Center {
                c: p2(0.0, -1.0),
                winding: ArcSweep::Ccw,
                p: along(0.0, -1.0, 1.0, core::f64::consts::FRAC_PI_2 - 1.0),
            },
            r,
            Center {
                c: p2(0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: along(0.5, 0.0, 0.5, core::f64::consts::PI + 1.0),
            },
            Tol::witness(),
        )
        .and_then(|b| b.line_to(Start, Tol::witness()))
        .map(|closed| closed.loop_)
    };
    let err = build(2.0).unwrap_err();
    println!("P7 at r = 2: {}", label(&Err(err.clone())));
    let PathError::FilletEnclosesLegCarrier {
        side,
        carrier_radius,
        ..
    } = err
    else {
        panic!("expected the enclosing refusal, got {err:?}");
    };
    println!("P7 the message names the {side} side, bound {carrier_radius}");
    for &r in &[1.9_f64, 1.5, 1.2, 0.99, 0.9, 0.75, 0.6, 0.51, 0.49, 0.3, 0.2, 0.1, 0.05] {
        let res = build(r);
        let endorsed = r < carrier_radius;
        println!(
            "P7   r = {r:<5} (below the named bound: {endorsed:<5}) -> {}",
            label(&res)
        );
    }
}

/// **E2E — a realistic authored part, as an outside consumer.**
///
/// A rounded slot outline: two circular lobes joined by fillets. This
/// is the shape a user would actually draw, and the probe reports what
/// the doors say at a sensible radius and at a demanding one.
#[test]
fn e2e_a_rounded_slot_authored_through_the_public_doors() {
    // Two lobes of radius 8 mm whose centres sit 10 mm apart: they
    // cross, and the crossings are the corners a fillet would round.
    let (r_lobe, sep) = (0.008_f64, 0.010);
    let o1 = p2(-sep / 2.0, 0.0);
    let o2 = p2(sep / 2.0, 0.0);
    // The upper crossing, by symmetry.
    let y = (r_lobe * r_lobe - (sep / 2.0) * (sep / 2.0)).sqrt();
    let corner = p2(0.0, y);
    println!("E2E lobes R = {r_lobe} m, centres {sep} m apart; upper crossing at {corner:?}");
    for &fillet in &[0.0005_f64, 0.001, 0.002, 0.004, 0.008, 0.012, 0.02] {
        let a1 = (corner.y - o1.y).atan2(corner.x - o1.x);
        let a2 = (corner.y - o2.y).atan2(corner.x - o2.x);
        let res = Open
            .arc_fillet_arc(
                Center {
                    c: o1,
                    winding: ArcSweep::Ccw,
                    p: p2(
                        o1.x + r_lobe * (a1 - 1.0).cos(),
                        o1.y + r_lobe * (a1 - 1.0).sin(),
                    ),
                },
                fillet,
                Center {
                    c: o2,
                    winding: ArcSweep::Ccw,
                    p: p2(
                        o2.x + r_lobe * (a2 + 1.0).cos(),
                        o2.y + r_lobe * (a2 + 1.0).sin(),
                    ),
                },
                Tol::witness(),
            )
            .and_then(|b| b.line_to(Start, Tol::witness()))
            .map(|closed| closed.loop_);
        println!("E2E fillet r = {fillet:<7} -> {}", label(&res));
        if let Err(e @ PathError::FilletEnclosesLegCarrier { .. }) = &res {
            println!("E2E     message: {e}");
        }
    }
}
