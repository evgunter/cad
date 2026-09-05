//! The enclosing-tangency class, exercised through the PUBLIC API only,
//! as an outside consumer would author it.
//!
//! Written as BLEND-7's r1 review probes and KEPT as pins: each row
//! below asserts a property of the shipped refusal, and the two that
//! only printed a measurement for a merge-base differential (a
//! `NoCornerSideCandidate` mining sweep, an outcome digest over an
//! unbracketed anchor grid) were deleted with the comparison they were
//! taken for — a row that asserts nothing cannot gate.
//!
//! - P1: the post-gate radius bands on the pinned row-1 geometry, and
//!   the recourse gap ("use a radius below R" vs what actually builds);
//! - P2: enclosing-demanding geometry OUTSIDE the ratifying grid's
//!   distribution (scale ratios past 15x, corners far from the origin,
//!   hairline carrier pairs, line x arc) — hunting a false negative;
//! - P3: the unbracketed other-crossing build, decoded and measured;
//! - P6: unequal carriers, where the endorsed bound is a chain.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom_core::{Point2, Tol};
use profile::path::CornerReason;
use profile::{ArcSweep, Center, Open, PathError, ProfileLoop, Start};

const PI: f64 = core::f64::consts::PI;
const FRAC_PI_2: f64 = core::f64::consts::FRAC_PI_2;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Point at `angle` on the circle about `center` through radius `r`.
fn on_circle(center: Point2<f64>, r: f64, angle: f64) -> Point2<f64> {
    p2(center.x + r * angle.cos(), center.y + r * angle.sin())
}

/// Author an arc x arc corner through the lattice door. The corner
/// sits at `corner`; each carrier has radius `r_c`, winds `tau`
/// (+1 ccw), and the corner sits at angle `a` about its centre; the
/// far anchors run `delta` radians from the corner along each leg
/// (incoming behind, outgoing ahead, in travel sense).
#[allow(clippy::too_many_arguments)]
fn author_arc_arc(
    corner: Point2<f64>,
    a_in: f64,
    r_in: f64,
    tau_in: f64,
    delta_in: f64,
    a_out: f64,
    r_out: f64,
    tau_out: f64,
    delta_out: f64,
    r: f64,
) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let c1 = p2(corner.x - r_in * a_in.cos(), corner.y - r_in * a_in.sin());
    let c2 = p2(
        corner.x - r_out * a_out.cos(),
        corner.y - r_out * a_out.sin(),
    );
    let head = on_circle(c1, r_in, a_in - tau_in * delta_in);
    let next = on_circle(c2, r_out, a_out + tau_out * delta_out);
    let w = |t: f64| if t > 0.0 { ArcSweep::Ccw } else { ArcSweep::Cw };
    let closed = Open
        .arc_fillet_arc(
            Center {
                c: c1,
                winding: w(tau_in),
                p: head,
            },
            r,
            Center {
                c: c2,
                winding: w(tau_out),
                p: next,
            },
            Tol::witness(),
        )?
        .line_to(Start, Tol::witness())?;
    Ok(closed.loop_)
}

/// Row 1 of `enclosing_cases`: sigma = tau = +1, equal carriers
/// R = 0.2, right-angle turn, with BRACKETING anchors (incoming capped
/// at 0.95*pi, outgoing clamped to 95% of the gap to the mirror
/// corner, which is pi/2 here).
fn row1_bracketed(r: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    author_arc_arc(
        p2(0.0, 0.0),
        0.0,
        0.2,
        1.0,
        0.95 * PI,
        FRAC_PI_2,
        0.2,
        1.0,
        0.95 * FRAC_PI_2,
        r,
    )
}

fn tag(e: &PathError<f64>) -> String {
    match e {
        // The envelope reports EVERY refusing crossing, so the tag does
        // too: one entry per derived corner, in the reported order.
        // The enclosing payload names the TIGHTEST class bound (a `None`
        // side meaning both carriers are swallowed) and carries the
        // existence bound the message endorses, so both numbers show.
        PathError::NoCornerOfPair { radius, corners } => {
            let entries: Vec<String> = corners.iter().map(|c| entry_tag(&c.reason)).collect();
            format!("Pair(r={radius},[{}])", entries.join(" "))
        }
        PathError::NoCornerForFillet { reason, .. } => format!("NoCorner({reason:?})"),
        PathError::FilletOffsetLeverTooShort { .. } => "LeverShort".to_string(),
        other => format!("{other:?}").chars().take(60).collect(),
    }
}

/// One envelope entry's reason, tagged.
fn entry_tag(reason: &CornerReason<f64>) -> String {
    match reason {
        CornerReason::EnclosesLegCarrier {
            side,
            carrier_radius,
            offset_radius,
            largest_tangent_radius,
        } => format!(
            "Encloses({side:?},R={carrier_radius},rho={offset_radius},bound={largest_tangent_radius:?})"
        ),
        CornerReason::NoTangentCircle(reason) => format!("NoTangent({reason:?})"),
        CornerReason::AnchorOutsideTrimmedExtent { side, .. } => format!("AnchorFit({side:?})"),
        CornerReason::OutsideAnchors(window) => format!("Outside({window:?})"),
    }
}

/// Recover the fillet circle (center, radius) from chord + bulge.
fn circle_from_bulge(t1: Point2<f64>, t2: Point2<f64>, b: f64) -> (Point2<f64>, f64) {
    let (cx, cy) = (t2.x - t1.x, t2.y - t1.y);
    let l = cx.hypot(cy);
    let d = l / 2.0;
    let radius = d * (1.0 + b * b) / (2.0 * b.abs());
    let k = d * (1.0 - b * b) / (2.0 * b);
    let (mx, my) = ((t1.x + t2.x) / 2.0, (t1.y + t2.y) / 2.0);
    let (ux, uy) = (cx / l, cy / l);
    (Point2::new(mx - uy * k, my + ux * k), radius)
}

/// Find the segment whose recovered circle radius matches `r`.
fn fillet_arc(lp: &ProfileLoop<f64>, r: f64) -> Option<(Point2<f64>, Point2<f64>, f64)> {
    let n = lp.vertices().len();
    for i in 0..n {
        let a = lp.vertices()[i];
        let b = lp.vertices()[(i + 1) % n];
        if a.bulge() == 0.0 {
            continue;
        }
        let (_, rf) = circle_from_bulge(a.pos(), b.pos(), a.bulge());
        if (rf - r).abs() < 1e-6 * r.max(1.0) {
            return Some((a.pos(), b.pos(), a.bulge()));
        }
    }
    None
}

/// P1: the radius sweep on row-1 geometry AFTER the gate, printed, and
/// the recourse gap asserted: the refusal names "below 0.2" but the
/// corner only BUILDS below (R1+R2-d)/2 = 0.0586.
#[test]
fn p1_row1_radius_bands_and_recourse_gap() {
    let mut bands: Vec<(f64, String, Vec<String>)> = Vec::new();
    for i in 1..=24 {
        let r = 0.025 * f64::from(i);
        let (out, entries) = match row1_bracketed(r) {
            Ok(_) => ("BUILDS".to_string(), Vec::new()),
            Err(ref e) => (
                tag(e),
                crate::common::corners(e)
                    .iter()
                    .map(|c| entry_tag(&c.reason))
                    .collect(),
            ),
        };
        bands.push((r, out, entries));
    }
    for (r, out, _) in &bands {
        println!("P1 r={r:.3} -> {out}");
    }
    // The enclosing band: every r > 0.2 (off-band) must be the typed
    // enclosing refusal now — no disjoint-offset / corner-side /
    // anchor-fit costumes left above the bound. EXACTLY, not by
    // substring: one entry, and it is the enclosing class. The
    // window-discarded crossing is not listed beside it, so a second
    // entry here would be a change in what the refusal claims.
    for (r, out, entries) in &bands {
        if *r >= 0.225 {
            assert_eq!(entries.len(), 1, "r={r}: {out}");
            assert!(
                entries[0].starts_with("Encloses("),
                "r={r}: the one entry is {}, not the enclosing class",
                entries[0]
            );
        }
    }
    // The recourse gap: the refusal at r=0.5 names "below 0.2", but
    // r=0.19 and r=0.1 still refuse (the ordinary branch ends at
    // 0.0586); only r below that builds.
    assert!(row1_bracketed(0.19).is_err(), "r=0.19 builds?");
    assert!(row1_bracketed(0.10).is_err(), "r=0.10 builds?");
    assert!(row1_bracketed(0.05).is_ok(), "r=0.05 must build");
}

/// P2a: enclosing demand at geometry the PR's mining grids could not
/// draw: scale ratio 200x, corner far from the origin, deep radius.
#[test]
fn p2a_enclosing_extremes_still_refuse_typed() {
    // 200x scale ratio, corner at (1e4, -3e3).
    let c = p2(1.0e4, -3.0e3);
    let err = author_arc_arc(c, 0.3, 0.01, 1.0, 2.9, 1.7, 2.0, 1.0, 0.9, 10.0)
        .expect_err("r=10 vs R=0.01/2.0 demands the enclosing class");
    // The drawn corner, alone: the envelope is about the crossing the
    // anchors bracket and does not add the other one beside it.
    crate::common::assert_corners(&err, &[(c.x, c.y)], "the drawn corner");
    assert!(crate::common::is_enclosing(&err), "got {err:?}");
    // Hairline pair: carriers nearly internally tangent (centres
    // 1e-4 apart at matched radii), r far above both.
    let c2 = p2(0.0, 0.0);
    let err2 = author_arc_arc(c2, 0.0, 0.2, 1.0, 2.9, 1.0e-3, 0.2, 1.0, 4.7e-4, 0.7)
        .expect_err("hairline enclosing demand");
    println!("P2a hairline -> {}", tag(&err2));
    // Tiny everything: micron-scale carriers, millimetre radius.
    let err3 = author_arc_arc(
        p2(0.0, 0.0),
        0.0,
        1.0e-6,
        1.0,
        2.9,
        1.4,
        3.0e-6,
        1.0,
        1.2,
        1.0e-3,
    )
    .expect_err("micron-scale enclosing demand");
    println!("P2a micro -> {}", tag(&err3));
    // ADOPTED, and made ε-KEYED: a micron carrier is not resolvable at
    // every shipped band. At ε = 1e-6 the arc centre sits within the
    // tolerance of its own carrier, so the door refuses that first and
    // is right to — the enclosing question is downstream of whether the
    // carrier exists at all. Asserting the class refusal unconditionally
    // made this row red at 1e-6 for a reason that is not about the gate,
    // which is the ε-blindness this suite removes elsewhere.
    let resolvable = 1.0e-6 > 10.0 * Tol::witness().eps();
    if resolvable {
        assert!(crate::common::is_enclosing(&err3), "got {err3:?}");
    } else {
        assert!(
            matches!(err3, PathError::DegenerateArcCenter { .. }),
            "below the band the carrier itself must be refused first, got {err3:?}"
        );
    }
}

/// P2b: UNBRACKETED enclosing demand at extremes: whatever the door
/// serves, it must never be a fillet that swallows a leg carrier.
#[test]
fn p2b_unbracketed_extremes_never_swallow() {
    let mut n_builds = 0_u32;
    let mut n_refusals = 0_u32;
    for &(rin, rout, r) in &[
        (0.2, 0.2, 0.5),
        (0.01, 2.0, 10.0),
        (0.05, 0.08, 0.5),
        (0.2, 0.2001, 0.5),
    ] {
        for &(din, dout) in &[(4.0, 4.0), (3.2, 6.0), (6.0, 6.0), (5.9, 0.2)] {
            let c = p2(0.3, -0.4);
            let cin = p2(c.x - rin, c.y);
            let cout = p2(c.x - rout * FRAC_PI_2.cos(), c.y - rout * FRAC_PI_2.sin());
            match author_arc_arc(c, 0.0, rin, 1.0, din, FRAC_PI_2, rout, 1.0, dout, r) {
                Ok(lp) => {
                    n_builds += 1;
                    if let Some((t1, t2, b)) = fillet_arc(&lp, r) {
                        let (pf, _) = circle_from_bulge(t1, t2, b);
                        for (o, rc) in [(cin, rin), (cout, rout)] {
                            let d = (pf.x - o.x).hypot(pf.y - o.y);
                            assert!(
                                d + rc > r - 1e-9,
                                "SWALLOWED carrier: |P-O|+R = {} < r = {r} at \
                                 rin={rin},rout={rout},din={din},dout={dout}",
                                d + rc
                            );
                        }
                    }
                }
                Err(_) => n_refusals += 1,
            }
        }
    }
    println!("P2b builds={n_builds} refusals={n_refusals}");
}

/// P2c: line x arc with a swallowed arc leg, far from the origin and
/// at a scale the harness distribution does not draw.
#[test]
fn p2c_line_arc_enclosing_refuses_typed() {
    // Corner far out; incoming straight leg, outgoing tiny ccw carrier
    // crossed by it; sigma*tau = +1 arranged as in the review suite's
    // partner table (line incoming east, arc at angle pi).
    let c = p2(-2.0e3, 5.0e2);
    let r_arc = 0.003;
    let ca = p2(c.x - r_arc * PI.cos(), c.y - r_arc * PI.sin());
    let head = p2(c.x - 1.8f64 * 1.0, c.y); // line from the west
    let next = on_circle(ca, r_arc, PI + 1.0 * 2.9);
    let closed = Open
        .at(head)
        .toward(1.0, 0.0, Tol::witness())
        .and_then(|b| {
            b.fillet_arc(
                2.5,
                Center {
                    c: ca,
                    winding: ArcSweep::Ccw,
                    p: next,
                },
                Tol::witness(),
            )
        });
    match closed {
        Ok(b) => {
            // Decode: which tangency did the door serve, and does it
            // swallow the small carrier?
            let closed = b.line_to(Start, Tol::witness()).unwrap();
            let lp = closed.loop_;
            let (t1, t2, bl) = fillet_arc(&lp, 2.5).expect("an emitted r=2.5 arc");
            let (pf, _) = circle_from_bulge(t1, t2, bl);
            let d = (pf.x - ca.x).hypot(pf.y - ca.y);
            println!(
                "P2c BUILT: |P-O_arc| = {d:.6} (r+R = {:.6}, r-R = {:.6}); \
                 |P-O|+R-r = {:.6}",
                2.5 + r_arc,
                2.5 - r_arc,
                d + r_arc - 2.5
            );
            assert!(
                d + r_arc > 2.5 - 1e-9,
                "SWALLOWED the arc carrier: |P-O|+R = {} < r = 2.5",
                d + r_arc
            );
        }
        Err(e) => {
            println!("P2c -> {}", tag(&e));
        }
    }
}

/// P3: the PR's measurement C — unbracketed anchors (4 rad legs) on
/// row-1 carriers at r = 0.5 BUILD an ordinary fillet at the pair's
/// other crossing. Decode and measure it.
#[test]
fn p3_unbracketed_other_crossing_build_decoded() {
    let corner = p2(0.0, 0.0);
    let lp = author_arc_arc(corner, 0.0, 0.2, 1.0, 4.0, FRAC_PI_2, 0.2, 1.0, 4.0, 0.5)
        .expect("the PR says this builds");
    let (t1, t2, b) = fillet_arc(&lp, 0.5).expect("an emitted r=0.5 arc");
    let (pf, rf) = circle_from_bulge(t1, t2, b);
    let cin = p2(-0.2, 0.0);
    let cout = p2(0.0, -0.2);
    let din = (pf.x - cin.x).hypot(pf.y - cin.y);
    let dout = (pf.x - cout.x).hypot(pf.y - cout.y);
    // Arc midpoint distance from the drawn corner.
    let mid = p2((t1.x + t2.x) / 2.0, (t1.y + t2.y) / 2.0);
    let d_corner = mid.x.hypot(mid.y);
    println!(
        "P3 fillet centre=({:.4},{:.4}) r={rf:.4} |P-O_in|={din:.4} |P-O_out|={dout:.4} \
         (r+R=0.7) tangent pts=({:.4},{:.4})-({:.4},{:.4}) mid-dist-from-corner={d_corner:.4}",
        pf.x, pf.y, t1.x, t1.y, t2.x, t2.y
    );
    for v in lp.vertices() {
        println!(
            "P3 vertex ({:.5},{:.5}) bulge {:.5}",
            v.pos().x,
            v.pos().y,
            v.bulge()
        );
    }
    // The PR's claim: externally tangent at the OTHER crossing.
    assert!((din - 0.7).abs() < 1e-9, "|P-O_in| = {din}");
    assert!((dout - 0.7).abs() < 1e-9, "|P-O_out| = {dout}");
    // The PR's "0.83 m away" is the fillet CENTRE's distance from the
    // drawn corner; the arc itself sits ~0.34 m away.
    let d_centre = pf.x.hypot(pf.y);
    println!("P3 centre-dist-from-corner={d_centre:.4}");
    assert!(d_centre > 0.8, "centre distance {d_centre}");
}

/// P6: unequal carriers (row-c4 shape, R_in 0.4 / R_out 0.15, sigma =
/// tau = -1): the refusal names the FIRST swallowed side's bound
/// (0.4), and following it to r = 0.3 lands on a second enclosing
/// refusal naming the tighter bound (0.15) — the recourse is a chain,
/// not one step.
#[test]
fn p6_unequal_carriers_bound_is_two_step() {
    let c = p2(-0.15, 0.5);
    let go = |r: f64| author_arc_arc(c, 0.0, 0.4, -1.0, 2.9, -2.4, 0.15, -1.0, 1.0, r);
    let e1 = go(0.9).expect_err("r=0.9 demands the enclosing class");
    println!("P6 r=0.9 -> {}", tag(&e1));
    let e2 = go(0.3).expect_err("r=0.3 still encloses the outgoing carrier");
    println!("P6 r=0.3 -> {}", tag(&e2));
    // ADOPTED, with the assertions moved onto the fixed behaviour: this
    // probe found the first-hit bound (r = 0.9 naming the incoming 0.4
    // while the outgoing 0.15 is the binding one, so a "smaller radius"
    // between them re-refuses). The bound is now the tightest carrier on
    // both rows, and the endorsed radius is the existence bound rather
    // than either carrier.
    crate::common::assert_corners(&e1, &[(c.x, c.y)], "r=0.9: the drawn corner");
    let Some((_, carrier_radius, _, largest_tangent_radius)) = crate::common::enclosing(&e1) else {
        panic!("r=0.9: {e1:?}")
    };
    assert!(
        (carrier_radius - 0.15).abs() < 1e-12,
        "r=0.9 names {carrier_radius}, not the tightest 0.15"
    );
    let bound = largest_tangent_radius.expect("both carriers swallowed at r = 0.9");
    assert!(
        bound < 0.15,
        "the endorsed bound {bound} is not below the class bound"
    );
    assert!(
        go(0.99 * bound).is_ok(),
        "the endorsed radius must build, or the recourse is dead again"
    );
    crate::common::assert_corners(&e2, &[(c.x, c.y)], "r=0.3: the drawn corner");
    let Some((_, carrier_radius, _, _)) = crate::common::enclosing(&e2) else {
        panic!("r=0.3: {e2:?}")
    };
    assert!((carrier_radius - 0.15).abs() < 1e-12);
}
