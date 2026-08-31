//! Adversarial e2e review probes for M5 S2 (arc-leg fillet sugar),
//! promoted from the review scratch into the permanent suite.
//!
//! Independent re-derivations: fuzzed tangency residuals for the
//! offset-carrier construction (F1), an atan2-based bulge oracle (F2),
//! k_stats sequence invariance and first-candidate attribution (F3),
//! and the enclosing (r > R) class the original suite lacked. The
//! oracles are written from the geometry rather than from `sugar.rs`:
//! the signed offset radius ρ = R − σ·τ·r is re-derived, the bulge is
//! checked against an `atan2` sweep, and setback/extent is recomputed
//! here. The FORWARD reductions below are `rem_euclid(TAU)` — the
//! right window for an extent, a gap or an authored sweep. The one
//! SIGNED reduction (`signed`) folds its raw difference once instead;
//! it is deliberately not a `[0, τ)` reduction with a second fold on
//! top, which is the spelling the production helper retired.
//!
//! Notes on adoption:
//!
//! - `overrun_attribution_names_the_authored_corners_candidate` was the
//!   review's MAJOR-1 *repro* — it asserted the buggy wrap-around
//!   setback. It is inverted here into the regression pin for the fix.
//! - the corner fuzz draws from `test_utils::fuzz`: a fresh seed per run
//!   (logged unconditionally) and a corner count that is a multiple of
//!   `CAD_FUZZ_EFFORT`. A pinned seed would make this a replay corpus
//!   rather than a fuzzer.
//! - F3's gate-sequence invariance probe records at `Probe`, so it lives
//!   in `review_s2_probe.rs` behind the `probe` feature; the rest of the
//!   suite is f64 and runs in the default build.
//!
//! # Why the enclosing case is a FIXTURE and not a coverage floor
//!
//! A fuzz asserts `∀ sampled x. P(x)`, which is monotone in the safe
//! direction: cutting the sample count loses detection power but can
//! never turn a green run red. A coverage FLOOR asserts `∃ sampled x.
//! C(x)` — a *witness* search. That is anti-monotone in the sample
//! count, so it turns the count into a load-bearing constant that only
//! ever ratchets upward, and it can fail for a reason that has nothing
//! to do with the code under test.
//!
//! **A witness belongs in a deterministic fixture, a fuzz belongs on
//! the counterexample search.** Mixing them makes one sample count
//! carry two obligations, and only one of the two is safe to cut.
//!
//! So the enclosing (ρ < 0) corners are built, not sampled for:
//! `enclosing_cases` inverts the ρ algebra — ρ = R − σ·τ·r < 0 ⟺
//! σ·τ = +1 and r > R — and every row DEMANDS the enclosing tangency.
//! `the_lattice_door_never_emits_an_enclosing_tangency` pins what the
//! shipped door answers on that table: the typed refusal
//! `PathError::FilletEnclosesLegCarrier`, on every band.
//!
//! **That boundary is permanent, not a finding.**
//! `docs/ENCLOSING-TANGENCY-DESIGN.md` rules the class out for good — an
//! arc whose circle contains both leg carriers contains the corner too,
//! so it cannot touch the corner it would round, and a construction that
//! cannot touch the corner is not a fillet OF it. No door emits the
//! class, and a request whose radius demands it is answered by that
//! refusal. So these pins do not describe what the ladder happens to
//! rank today; they assert a ruling, and a build here is a violation of
//! it rather than a boundary to re-pin.
//!
//! The sweep's `n_enclosing` line is therefore a coverage report, not a
//! floor: through this door the count is 0 structurally. `assert_eq!`
//! pins that 0 — monotone-safe where a `>= 1` floor was not. What it
//! guards is the NO-EMISSION half of the ruling (nothing the door builds
//! demands the class), which is exactly what a count of accepted corners
//! can see; that the demand REFUSES, and with which words, is the two
//! pins' claim, not this one's.
//!
//! ρ < 0 on one leg forces ρ < 0 on the other, so a corner with one
//! swallowed carrier and a line leg, an opposite-sense arc, or an arc
//! bigger than the fillet does not EXIST — such a request can be
//! authored (the anchors and radius are just numbers), and what it names
//! is a pair of carriers whose tangency is degenerate, so no solution is
//! there to find. Geometrically impossible, not merely rare.
//! `an_enclosing_leg_forces_an_equally_enclosing_partner` pins each
//! one's refusal with the inequality that rules it out.
//!
//! The floors that remain (accepted corners, arc-by-arc corners) are
//! FRACTIONS of the corner count, so they scale with `CAD_FUZZ_EFFORT`
//! instead of turning red at low effort, and they describe the bulk of
//! the distribution rather than a rare class.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::tol;
use geom_core::Point2;
use geom_core::Tol;
use profile::path::PathNoCornerReason;
use profile::{ArcSweep, Center, Open, PathError, ProfileLoop, Start};
use test_utils::fuzz;

const TAU: f64 = core::f64::consts::TAU;
const PI: f64 = core::f64::consts::PI;
const FRAC_PI_2: f64 = core::f64::consts::FRAC_PI_2;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn flip(rng: &mut fuzz::Rng) -> bool {
    rng.unit() < 0.5
}

/// One resolved leg for the oracle side (re-derived, not from src).
#[derive(Clone, Copy)]
enum OracleLeg {
    Line {
        far: Point2<f64>,
    },
    Arc {
        center: Point2<f64>,
        radius: f64,
        tau: f64,
        far_angle: f64,
    },
}

impl OracleLeg {
    /// The travel sense of an arc leg (chain order), for the lattice's
    /// `Center` spec.
    fn winding(&self) -> ArcSweep {
        match *self {
            OracleLeg::Line { .. } => panic!("windings are asked of arc legs only"),
            OracleLeg::Arc { tau, .. } => {
                if tau > 0.0 {
                    ArcSweep::Ccw
                } else {
                    ArcSweep::Cw
                }
            }
        }
    }
    fn far_point(&self, _corner: Point2<f64>) -> Point2<f64> {
        match *self {
            OracleLeg::Line { far } => far,
            OracleLeg::Arc {
                center,
                radius,
                far_angle,
                ..
            } => Point2::new(
                center.x + radius * far_angle.cos(),
                center.y + radius * far_angle.sin(),
            ),
        }
    }
    /// Unit travel direction at the corner (incoming legs travel toward
    /// the corner, outgoing away — matches chain order).
    fn travel_dir(&self, corner: Point2<f64>, incoming: bool) -> (f64, f64) {
        match *self {
            OracleLeg::Line { far } => {
                let (vx, vy) = if incoming {
                    (corner.x - far.x, corner.y - far.y)
                } else {
                    (far.x - corner.x, far.y - corner.y)
                };
                let n = vx.hypot(vy);
                (vx / n, vy / n)
            }
            OracleLeg::Arc {
                center,
                radius,
                tau,
                ..
            } => {
                let (rx, ry) = (corner.x - center.x, corner.y - center.y);
                // tangent = tau * perp(r)/R, independent of side.
                (-tau * ry / radius, tau * rx / radius)
            }
        }
    }
    /// Residual of `p` against the leg's carrier.
    fn carrier_residual(&self, corner: Point2<f64>, p: Point2<f64>) -> f64 {
        match *self {
            OracleLeg::Line { far } => {
                let (dx, dy) = (far.x - corner.x, far.y - corner.y);
                let n = dx.hypot(dy);
                ((dx * (p.y - corner.y) - dy * (p.x - corner.x)) / n).abs()
            }
            OracleLeg::Arc { center, radius, .. } => {
                ((p.x - center.x).hypot(p.y - center.y) - radius).abs()
            }
        }
    }
    /// Signed setback of `p` from the corner along the leg, plus extent.
    fn setback_extent(&self, corner: Point2<f64>, p: Point2<f64>, incoming: bool) -> (f64, f64) {
        match *self {
            OracleLeg::Line { far } => {
                let (dx, dy) = self.travel_dir(corner, incoming);
                let sb = if incoming {
                    (corner.x - p.x) * dx + (corner.y - p.y) * dy
                } else {
                    (p.x - corner.x) * dx + (p.y - corner.y) * dy
                };
                (sb, (far.x - corner.x).hypot(far.y - corner.y))
            }
            OracleLeg::Arc {
                center,
                radius,
                tau,
                far_angle,
            } => {
                let ca = (corner.y - center.y).atan2(corner.x - center.x);
                let pa = (p.y - center.y).atan2(p.x - center.x);
                let (sb_ang, ext_ang) = if incoming {
                    (
                        ((ca - pa) * tau).rem_euclid(TAU),
                        ((ca - far_angle) * tau).rem_euclid(TAU),
                    )
                } else {
                    (
                        ((pa - ca) * tau).rem_euclid(TAU),
                        ((far_angle - ca) * tau).rem_euclid(TAU),
                    )
                };
                (radius * sb_ang, radius * ext_ang)
            }
        }
    }
    /// The F1 crux: expected |P_fillet - O| for an arc leg = |R - sigma*tau*r|.
    fn center_distance_residual(&self, pf: Point2<f64>, sigma: f64, r: f64) -> f64 {
        match *self {
            OracleLeg::Line { .. } => 0.0, // handled by carrier_residual(pf) - r elsewhere
            OracleLeg::Arc {
                center,
                radius,
                tau,
                ..
            } => {
                let d = (pf.x - center.x).hypot(pf.y - center.y);
                (d - (radius - sigma * tau * r).abs()).abs()
            }
        }
    }
    fn is_enclosing(&self, sigma: f64, r: f64) -> bool {
        match *self {
            OracleLeg::Line { .. } => false,
            OracleLeg::Arc { radius, tau, .. } => radius - sigma * tau * r < 0.0,
        }
    }
}

/// Recover the fillet circle (center, radius) from the emitted chord +
/// bulge — the independent decoding of the stored form.
fn circle_from_bulge(t1: Point2<f64>, t2: Point2<f64>, b: f64) -> (Point2<f64>, f64) {
    let (cx, cy) = (t2.x - t1.x, t2.y - t1.y);
    let l = cx.hypot(cy);
    let d = l / 2.0;
    let radius = d * (1.0 + b * b) / (2.0 * b.abs());
    let k = d * (1.0 - b * b) / (2.0 * b); // signed apothem along left normal
    let (mx, my) = ((t1.x + t2.x) / 2.0, (t1.y + t2.y) / 2.0);
    let (ux, uy) = (cx / l, cy / l);
    (Point2::new(mx - uy * k, my + ux * k), radius)
}

/// The corner's turn side σ, re-derived from the two travel directions.
fn turn_sign(corner: Point2<f64>, leg_in: OracleLeg, leg_out: OracleLeg) -> f64 {
    let (dix, diy) = leg_in.travel_dir(corner, true);
    let (dox, doy) = leg_out.travel_dir(corner, false);
    (dix * doy - diy * dox).signum()
}

/// The pair's OTHER carrier intersection (the mirror corner), when the
/// carrier pair admits one: circle x circle by reflection across the
/// centre line, ray x circle by reflection about the foot of the
/// centre on the line. `None` for line x line (a unique corner).
fn mirror_corner(
    corner: Point2<f64>,
    leg_in: OracleLeg,
    leg_out: OracleLeg,
) -> Option<Point2<f64>> {
    match (leg_in, leg_out) {
        (OracleLeg::Arc { center: o1, .. }, OracleLeg::Arc { center: o2, .. }) => {
            let d = (o2.x - o1.x, o2.y - o1.y);
            let len2 = d.0 * d.0 + d.1 * d.1;
            if len2 == 0.0 {
                return None;
            }
            let v = (corner.x - o1.x, corner.y - o1.y);
            let t = (v.0 * d.0 + v.1 * d.1) / len2;
            let foot = (o1.x + t * d.0, o1.y + t * d.1);
            Some(p2(2.0 * foot.0 - corner.x, 2.0 * foot.1 - corner.y))
        }
        (OracleLeg::Line { .. }, OracleLeg::Arc { center: o, .. }) => {
            let (dx, dy) = leg_in.travel_dir(corner, true);
            let v = (o.x - corner.x, o.y - corner.y);
            let t = v.0 * dx + v.1 * dy;
            let foot = (corner.x + t * dx, corner.y + t * dy);
            Some(p2(2.0 * foot.0 - corner.x, 2.0 * foot.1 - corner.y))
        }
        (OracleLeg::Arc { center: o, .. }, OracleLeg::Line { .. }) => {
            let (dx, dy) = leg_out.travel_dir(corner, false);
            let v = (o.x - corner.x, o.y - corner.y);
            let t = v.0 * dx + v.1 * dy;
            let foot = (corner.x + t * dx, corner.y + t * dy);
            Some(p2(2.0 * foot.0 - corner.x, 2.0 * foot.1 - corner.y))
        }
        (OracleLeg::Line { .. }, OracleLeg::Line { .. }) => None,
    }
}

/// Signed angular difference into [−π, π).
///
/// Spelled here rather than called from `sugar.rs` — see this file's
/// header on oracle independence — but NOT as the composition the
/// production helper retired: the raw difference is folded once, into
/// the window centred on zero. Reducing into `[0, τ)` first would put a
/// jump exactly at the coincidence this oracle's callers measure.
fn signed(a: f64) -> f64 {
    a - TAU * (a / TAU + 0.5).floor()
}

/// The signed travel of `p` measured from `q` along an arc leg's
/// carrier (metres), in the leg's travel sense.
fn arc_travel(center: Point2<f64>, radius: f64, tau: f64, q: Point2<f64>, p: Point2<f64>) -> f64 {
    let a = |x: Point2<f64>| (x.y - center.y).atan2(x.x - center.x);
    radius * signed((a(p) - a(q)) * tau)
}

/// Clamp the ARRIVAL leg's anchor to sit strictly BEFORE the mirror
/// corner along its own carrier (95% of the gap; arc extents also
/// capped below the signed half-turn), so the reach gate reads the
/// mirror as Negative. Legs whose mirror is behind the corner pass
/// through — the incoming side's gate is checked separately.
fn clamp_arrival(corner: Point2<f64>, m: Point2<f64>, leg: OracleLeg) -> OracleLeg {
    match leg {
        OracleLeg::Arc {
            center,
            radius,
            tau,
            far_angle,
        } => {
            let thc = (corner.y - center.y).atan2(corner.x - center.x);
            let thm = (m.y - center.y).atan2(m.x - center.x);
            let gap = ((thm - thc) * tau).rem_euclid(TAU);
            let delta = ((far_angle - thc) * tau).rem_euclid(TAU);
            let lim = (0.95 * gap).min(0.95 * PI);
            let new_delta = delta.min(lim);
            OracleLeg::Arc {
                center,
                radius,
                tau,
                far_angle: thc + tau * new_delta,
            }
        }
        OracleLeg::Line { far } => {
            let (dx, dy) = ((far.x - corner.x), (far.y - corner.y));
            let len = dx.hypot(dy);
            if len == 0.0 {
                return leg;
            }
            let g = ((m.x - corner.x) * dx + (m.y - corner.y) * dy) / len;
            if g <= 0.0 {
                return leg;
            }
            let new_len = len.min(0.95 * g);
            OracleLeg::Line {
                far: p2(corner.x + dx / len * new_len, corner.y + dy / len * new_len),
            }
        }
    }
}

/// Cap an incoming ARC leg's extent below the signed half-turn (the
/// lattice's advance gate reads past-the-half-turn as Negative).
fn cap_incoming(corner: Point2<f64>, leg: OracleLeg) -> OracleLeg {
    let OracleLeg::Arc {
        center,
        radius,
        tau,
        far_angle,
    } = leg
    else {
        return leg;
    };
    let thc = (corner.y - center.y).atan2(corner.x - center.x);
    let delta = ((thc - far_angle) * tau).rem_euclid(TAU);
    let new_delta = delta.min(0.95 * PI);
    OracleLeg::Arc {
        center,
        radius,
        tau,
        far_angle: thc - tau * new_delta,
    }
}

/// Whether the pair's mirror corner is EXCLUDED by the lattice's
/// signed gates for these (already clamped) anchors: negative advance
/// from the incoming anchor, or negative reach from the arrival
/// anchor. A drawn corner whose mirror passes both gates is not the
/// corner this harness's oracle describes (the lifted ladder may
/// legitimately round the other crossing), so such draws are SKIPPED
/// rather than mis-asserted.
fn mirror_excluded(
    m: Point2<f64>,
    leg_in: OracleLeg,
    leg_out: OracleLeg,
    corner: Point2<f64>,
) -> bool {
    let adv = match leg_in {
        OracleLeg::Arc {
            center,
            radius,
            tau,
            ..
        } => arc_travel(center, radius, tau, leg_in.far_point(corner), m),
        OracleLeg::Line { far } => {
            let (dx, dy) = leg_in.travel_dir(corner, true);
            (m.x - far.x) * dx + (m.y - far.y) * dy
        }
    };
    let reach = match leg_out {
        OracleLeg::Arc {
            center,
            radius,
            tau,
            ..
        } => arc_travel(center, radius, tau, m, leg_out.far_point(corner)),
        OracleLeg::Line { .. } => {
            let (dx, dy) = leg_out.travel_dir(corner, false);
            let a = leg_out.far_point(corner);
            (a.x - m.x) * dx + (a.y - m.y) * dy
        }
    };
    adv <= 0.0 || reach <= 0.0
}

/// Author the corner through the PATHS lattice and close it. The
/// lattice derives the corner from the two anchored carriers (it is
/// never authored), so this harness AUTHORS THE DRAWN CORNER the one
/// way the lattice offers: each far anchor is placed to bracket it —
/// past the corner along its own side, and strictly inside the pair's
/// mirror corner, so the signed advance/reach gates admit exactly the
/// corner the oracle's claims are about. §2c dissolution: an arc
/// arrival lands on an ordinary directed point, so every leg-kind pair
/// closes with a sharp straight seam or a short straight run.
fn build_corner(
    corner: Point2<f64>,
    leg_in: OracleLeg,
    leg_out: OracleLeg,
    r: f64,
) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let (leg_in, leg_out) = match mirror_corner(corner, leg_in, leg_out) {
        Some(m) => (
            cap_incoming(corner, leg_in),
            clamp_arrival(corner, m, leg_out),
        ),
        None => (leg_in, leg_out),
    };
    let head = leg_in.far_point(corner);
    let next = leg_out.far_point(corner);
    let closed = match (leg_in, leg_out) {
        (OracleLeg::Arc { center: c1, .. }, OracleLeg::Arc { center: c2, .. }) => Open
            .arc_fillet_arc(
                Center {
                    c: c1,
                    winding: leg_in.winding(),
                    p: head,
                },
                r,
                Center {
                    c: c2,
                    winding: leg_out.winding(),
                    p: next,
                },
                Tol::witness(),
            )?
            .line_to(Start, Tol::witness())?,
        (OracleLeg::Arc { center: c1, .. }, OracleLeg::Line { .. }) => {
            let (dx, dy) = leg_out.travel_dir(corner, false);
            Open.arc_fillet(
                Center {
                    c: c1,
                    winding: leg_in.winding(),
                    p: head,
                },
                r,
                Tol::witness(),
            )?
            .at(next, Tol::witness())?
            .toward(dx, dy, Tol::witness())?
            .line(0.25, Tol::witness())?
            .line_to(Start, Tol::witness())?
        }
        (OracleLeg::Line { .. }, OracleLeg::Arc { center: c2, .. }) => {
            let (dx, dy) = leg_in.travel_dir(corner, true);
            Open.at(head)
                .toward(dx, dy, Tol::witness())?
                .fillet_arc(
                    r,
                    Center {
                        c: c2,
                        winding: leg_out.winding(),
                        p: next,
                    },
                    Tol::witness(),
                )?
                .line_to(Start, Tol::witness())?
        }
        (OracleLeg::Line { .. }, OracleLeg::Line { .. }) => {
            let (dx1, dy1) = leg_in.travel_dir(corner, true);
            let (dx2, dy2) = leg_out.travel_dir(corner, false);
            Open.at(head)
                .toward(dx1, dy1, Tol::witness())?
                .fillet(r, Tol::witness())?
                .at(next, Tol::witness())?
                .toward(dx2, dy2, Tol::witness())?
                .line(0.25, Tol::witness())?
                .line_to(Start, Tol::witness())?
        }
    };
    Ok(closed.loop_)
}

/// Locate the emitted fillet arc: the unique segment whose recovered
/// circle has radius `r` (draws that would put a LEG carrier's radius
/// within the match window of `r` are skipped at the draw, so the
/// match cannot be ambiguous). Returns (t1, t2, bulge).
fn fillet_segment(
    lp: &ProfileLoop<f64>,
    r: f64,
    ctx: &dyn Fn() -> String,
) -> (Point2<f64>, Point2<f64>, f64) {
    let n = lp.vertices().len();
    let mut found = None;
    for i in 0..n {
        let a = lp.vertices()[i];
        let b = lp.vertices()[(i + 1) % n];
        let bl = a.bulge();
        if bl == 0.0 {
            continue;
        }
        let (_, rf) = circle_from_bulge(a.pos(), b.pos(), bl);
        if (rf - r).abs() < 1e-6 * r.max(1.0) {
            assert!(
                found.is_none(),
                "two segments recover the fillet radius — {}",
                ctx()
            );
            found = Some((a.pos(), b.pos(), bl));
        }
    }
    found.unwrap_or_else(|| {
        panic!(
            "no emitted segment recovers the fillet radius {r} — {}",
            ctx()
        )
    })
}

/// What one checked corner contributed to the sweep's coverage report.
#[derive(Clone, Copy)]
struct CornerCounts {
    arc_legs: u64,
    arc_arc: u64,
    enclosing: u64,
    major: u64,
}

/// **The ruling, asserted — the one home of that check.** For a corner
/// whose geometry demands the enclosing (ρ < 0) tangency, no door emits
/// one (`docs/ENCLOSING-TANGENCY-DESIGN.md`), so whatever a door built
/// must swallow NEITHER carrier: |P − O| + R stays above r. Called from
/// `check_corner`'s enclosing arm and from `report_moved_refuse_pin`,
/// which is what both enclosing pins call; the three copies of this
/// arithmetic and this message that used to sit at those sites are gone.
fn assert_swallows_nothing(
    pf: Point2<f64>,
    center: Point2<f64>,
    radius: f64,
    r: f64,
    ctx: &dyn Fn() -> String,
) {
    let d = (pf.x - center.x).hypot(pf.y - center.y);
    assert!(
        d + radius > r - 1e-9,
        "the door emitted an ENCLOSING tangency (|P-O| + R = {} < r = {r}) — the class \
         docs/ENCLOSING-TANGENCY-DESIGN.md rules permanently out — {}",
        d + radius,
        ctx()
    );
}

/// **What both enclosing pins do with an `Ok` — which is now a report of
/// a broken ruling, not of a moved boundary.** One function, because
/// they were one function written twice.
/// `the_lattice_door_never_emits_an_enclosing_tangency` (the six-row
/// table) and `enclosing_fillet_swallows_both_leg_carriers` (the mined
/// corner) pin the same refusal at the same door, and their `Ok` arms
/// carried the same `fillet_segment` → `circle_from_bulge` → per-carrier
/// boundary check → same panic, with neither site naming the other.
/// Nothing in either declared the copy, which is why no marker
/// vocabulary could have found it (smell-scan S133).
///
/// The boundary check runs BEFORE the panic on purpose: a failure here
/// should report what got built, and a build that swallows a carrier —
/// the ruled-out class itself, emitted — is a different and much larger
/// failure than a build that merely serves some other tangency where the
/// ruling says the request must refuse.
fn report_moved_refuse_pin(
    lp: &ProfileLoop<f64>,
    r: f64,
    carriers: &[(Point2<f64>, f64)],
    what: &str,
    ctx: &dyn Fn() -> String,
) -> ! {
    let (t1, t2, b) = fillet_segment(lp, r, ctx);
    let (pf, _) = circle_from_bulge(t1, t2, b);
    for (center, radius) in carriers {
        assert_swallows_nothing(pf, *center, *radius, r, ctx);
    }
    panic!(
        "{}: {what} now BUILDS (a non-swallowing fillet, verified above) where \
         docs/ENCLOSING-TANGENCY-DESIGN.md rules that a radius demanding the enclosing \
         class must refuse typed; this is a violation of that ruling, not a boundary to \
         re-pin",
        ctx()
    );
}

/// **The oracle battery for one constructed fillet corner.** One
/// function, so a caller that runs it cannot check fewer properties
/// than the sweep does. **Its oracle is about the DRAWN corner**, which
/// decides who may call it: the fuzz, for every accepted draw (it skips
/// draws whose mirror crossing survives the gates), and
/// `an_ill_conditioned_corner_lands_its_tangent_point_on_the_carrier`,
/// the one fixture whose built corner is the drawn one. The fixtures
/// that pin a REFUSAL have no fillet to check and assert against the
/// typed error instead, and
/// `an_uncertifiable_tangent_point_refuses_instead_of_being_returned`
/// builds on a hairline lens's TWIN crossing, so this battery would be
/// wrong there and it says so at its own site.
///
/// `ctx` is called only when an assertion is about to fail, so the fuzz
/// can hand it a formatter that names the iteration and the replay
/// command without paying for it per corner.
fn check_corner(
    corner: Point2<f64>,
    leg_in: OracleLeg,
    leg_out: OracleLeg,
    r: f64,
    lp: &ProfileLoop<f64>,
    ctx: &dyn Fn() -> String,
) -> CornerCounts {
    let mut counts = CornerCounts {
        arc_legs: 0,
        arc_arc: 0,
        enclosing: 0,
        major: 0,
    };
    let (t1, t2, b) = fillet_segment(lp, r, ctx);
    assert!(
        b.is_finite() && b != 0.0,
        "degenerate fillet bulge {b} — {}",
        ctx()
    );

    // sigma re-derived from travel directions.
    let sigma = turn_sign(corner, leg_in, leg_out);

    // (a) fillet circle from the emitted data.
    let (pf, rf) = circle_from_bulge(t1, t2, b);
    assert!(
        (rf - r).abs() < 1e-9,
        "recovered radius {rf} vs {r} — {}",
        ctx()
    );

    // (b) tangent points on their carriers. The residual is REPORTED,
    // not just thresholded: "off carrier" without a magnitude cannot
    // distinguish a borderline tolerance from a gross geometric error,
    // and that is precisely the question a failure here raises.
    let (res_in, res_out) = (
        leg_in.carrier_residual(corner, t1),
        leg_out.carrier_residual(corner, t2),
    );
    assert!(
        res_in < 1e-9,
        "t1 off carrier by {res_in:e} (tol 1e-9) — {}",
        ctx()
    );
    assert!(
        res_out < 1e-9,
        "t2 off carrier by {res_out:e} (tol 1e-9) — {}",
        ctx()
    );

    // (c) fillet circle tangent to both carriers, with the SIGNED
    // rho = R - sigma*tau*r predicting the center distance (F1).
    for (leg, inc) in [(leg_in, true), (leg_out, false)] {
        match leg {
            OracleLeg::Line { .. } => {
                let d = leg.carrier_residual(corner, pf);
                assert!(
                    (d - r).abs() < 1e-9,
                    "line-carrier clearance {d} vs r {r} — {}",
                    ctx()
                );
            }
            OracleLeg::Arc { center, radius, .. } => {
                counts.arc_legs += 1;
                let res = leg.center_distance_residual(pf, sigma, r);
                assert!(res < 1e-9, "|P-O| vs |rho| residual {res} — {}", ctx());
                if leg.is_enclosing(sigma, r) {
                    // rho < 0 on this leg: the corner's geometry DEMANDS
                    // the enclosing tangency. What is asserted here is
                    // the BOUNDARY, not the enclosing algebra — the
                    // lattice door never emits an enclosing tangency, so
                    // whatever it built must swallow neither carrier.
                    // Asserting the enclosing relation instead (the shape
                    // this arm carried while the raw builder existed)
                    // would pass on exactly the emission that would mean
                    // the boundary had moved.
                    counts.enclosing += 1;
                    assert_swallows_nothing(pf, center, radius, r, ctx);
                }
            }
        }
        // (d) tangent points at distance r from the fillet center.
        let t = if inc { t1 } else { t2 };
        let dt = ((t.x - pf.x).hypot(t.y - pf.y) - r).abs();
        assert!(dt < 1e-9, "|t-P| residual {dt} — {}", ctx());
        // (e) corner-side extents.
        let (sb, ext) = leg.setback_extent(corner, t, inc);
        assert!(
            sb > -1e-9 && sb < ext + 1e-9,
            "setback {sb} outside [0, {ext}] — {}",
            ctx()
        );
    }
    if matches!(leg_in, OracleLeg::Arc { .. }) && matches!(leg_out, OracleLeg::Arc { .. }) {
        counts.arc_arc += 1;
    }

    // (f) F2: atan2 sweep oracle for the bulge (major arcs included).
    let ang = |p: Point2<f64>| (p.y - pf.y).atan2(p.x - pf.x);
    let theta = ((ang(t2) - ang(t1)) * sigma).rem_euclid(TAU);
    let b_ref = sigma * (theta / 4.0).tan();
    assert!(
        (b - b_ref).abs() <= 1e-9 * b_ref.abs().max(1.0),
        "bulge {b} vs atan2 oracle {b_ref} (theta {theta}) — {}",
        ctx()
    );
    if b.abs() > 1.0 {
        counts.major = 1;
    }
    counts
}

fn rand_leg(
    rng: &mut fuzz::Rng,
    corner: Point2<f64>,
    incoming: bool,
    enclosing_bias: bool,
) -> OracleLeg {
    if !enclosing_bias && flip(rng) {
        let a = rng.range(0.0, TAU);
        let len = rng.range(0.5, 3.0);
        OracleLeg::Line {
            far: Point2::new(corner.x + len * a.cos(), corner.y + len * a.sin()),
        }
    } else {
        let radius = if enclosing_bias {
            rng.range(0.05, 0.3)
        } else {
            rng.range(0.3, 2.5)
        };
        let a = rng.range(0.0, TAU);
        let center = Point2::new(corner.x - radius * a.cos(), corner.y - radius * a.sin());
        let tau = if flip(rng) { 1.0 } else { -1.0 };
        let delta = rng.range(0.4, 2.8);
        let far_angle = if incoming {
            a - tau * delta
        } else {
            a + tau * delta
        };
        OracleLeg::Arc {
            center,
            radius,
            tau,
            far_angle,
        }
    }
}

/// [`rand_leg`]'s arc branch with the draws replaced by chosen numbers:
/// the carrier of radius `radius` through `corner`, which sits at angle
/// `a` as seen from the center, winding `tau` in chain order and running
/// `delta` radians away from the corner.
fn arc_leg(
    corner: Point2<f64>,
    a: f64,
    radius: f64,
    tau: f64,
    delta: f64,
    incoming: bool,
) -> OracleLeg {
    OracleLeg::Arc {
        center: Point2::new(corner.x - radius * a.cos(), corner.y - radius * a.sin()),
        radius,
        tau,
        far_angle: if incoming {
            a - tau * delta
        } else {
            a + tau * delta
        },
    }
}

/// [`rand_leg`]'s line branch with the draws chosen: the far end sits
/// `len` from the corner in direction `a`.
fn line_leg(corner: Point2<f64>, a: f64, len: f64) -> OracleLeg {
    OracleLeg::Line {
        far: Point2::new(corner.x + len * a.cos(), corner.y + len * a.sin()),
    }
}

#[test]
fn fuzz_offset_carrier_construction_tangency_and_bulge() {
    let mut rng = fuzz::start("review_s2::offset_carrier_tangency_and_bulge");
    // The counterexample search, and nothing else. The 12 500 this used
    // to be was set by a WITNESS obligation (the enclosing floor), which
    // now lives in `enclosing_cases` as built geometry and in
    // `the_lattice_door_never_emits_an_enclosing_tangency` as the pin on
    // what the door does with it — see the module docs. Nothing forces
    // the count upward any more, so it is a smoke level like every other
    // sweep's, and `CAD_FUZZ_EFFORT` buys depth when depth is wanted.
    let corners = fuzz::scaled(1_500);
    let (mut n_ok, mut n_arc_leg, mut n_arc_arc, mut n_enclosing, mut n_major) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    for i in 0..corners {
        let enclosing_bias = i % 5 == 0;
        let corner = p2(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
        let leg_in = rand_leg(&mut rng, corner, true, enclosing_bias);
        let leg_out = rand_leg(&mut rng, corner, false, enclosing_bias);
        let r = if enclosing_bias {
            rng.range(0.06, 0.9)
        } else {
            rng.range(0.02, 0.8)
        };
        // A leg carrier whose radius sits inside `fillet_segment`'s
        // match window would make the fillet-arc extraction ambiguous;
        // the window is 1e-6, the skip band 100x it.
        let near_r = |leg: OracleLeg| matches!(leg, OracleLeg::Arc { radius, .. } if (radius - r).abs() < 1e-4);
        if near_r(leg_in) || near_r(leg_out) {
            continue;
        }
        // The oracle speaks about the DRAWN corner, so a draw whose
        // mirror corner would survive the gates even after bracketing
        // is skipped (the lifted ladder may legitimately round the
        // other crossing there).
        if let Some(m) = mirror_corner(corner, leg_in, leg_out) {
            let li = cap_incoming(corner, leg_in);
            let lo = clamp_arrival(corner, m, leg_out);
            if !mirror_excluded(m, li, lo, corner) {
                continue;
            }
        }
        let Ok(lp) = build_corner(corner, leg_in, leg_out, r) else {
            continue;
        };
        n_ok += 1;
        let c = check_corner(corner, leg_in, leg_out, r, &lp, &|| {
            format!("iter {i}: {}", fuzz::replay())
        });
        n_arc_leg += c.arc_legs;
        n_arc_arc += c.arc_arc;
        n_enclosing += c.enclosing;
        n_major += c.major;
    }
    // COVERAGE FLOORS, as fractions of the corner count so they scale
    // with `CAD_FUZZ_EFFORT` instead of turning red at low effort. The
    // fractions are the review's own (500 and 50 out of 20 000). Both
    // describe the BULK of the draw distribution — a fraction is only a
    // safe floor when the class it names is common. RE-CALIBRATED at
    // the lattice door (the mirror-survivor skip removes ~17% of draws
    // whose oracle would speak about the wrong crossing): measured at
    // 1 500 corners over 10 fresh-seed runs, accepted 741-796
    // (mean ~771, 51%) against a floor of 38, arc-by-arc 179-229
    // (mean ~206, 14%) against a floor of 4 — the unchanged fractions
    // now bind TIGHTER relative to the distribution than the raw-door
    // numbers did (~20x and ~55x slack, down from ~25x and ~74x).
    // Neither is a witness search in disguise.
    let corners = corners as u64;
    assert!(
        n_ok * 40 > corners,
        "only {n_ok} accepted corners out of {corners} — {}",
        fuzz::replay()
    );
    assert!(
        n_arc_arc * 400 > corners,
        "only {n_arc_arc} arc-by-arc corners out of {corners} — {}",
        fuzz::replay()
    );
    // `n_enclosing` used to be gated by an absolute FLOOR (`>= 1`),
    // which is what forced this count to 12 500 — a witness search
    // whose sample count only ever ratchets upward. At the lattice door
    // the count is structurally 0, so the guard inverts: pinning 0 is
    // monotone in the safe direction (more draws can only find a
    // violation, never lose one). A printed number nobody compares is
    // not a guard — `cargo test` swallows the report line below without
    // `--nocapture`.
    //
    // WHAT THIS GUARDS, precisely: the no-emission half of the ruling —
    // no corner the door BUILDS demands the enclosing class, whether the
    // demand is refused by the class gate or was never rankable in the
    // first place. It is not sensitive to the gate itself (before the
    // gate existed the count was 0 too, by the ladder's own ranking), and
    // claiming otherwise would make a passing row look like evidence for
    // a refusal it never examines. The refusal is the two enclosing
    // pins', which assert the variant and its payload.
    assert_eq!(
        n_enclosing,
        0,
        "the lattice door built {n_enclosing} corner(s) whose geometry demands the \
         enclosing tangency; the boundary this suite pins says it builds none — {}",
        fuzz::replay()
    );
    // `n_major` stays a REPORT. It comes out 0, which is the fuzz
    // corroborating the bound `fillet_bulge`'s docs argue for — the corner-side extent gates keep
    // every fillet arc below half a turn, so the negative-apothem branch
    // is unreachable through this door and is unit-tested directly
    // instead. Deliberately not asserted either way: a future change that
    // legitimately admits major arcs should not fail here, it should make
    // the branch live.
    eprintln!(
        "fuzz: ok {n_ok}, arc legs {n_arc_leg}, arc-by-arc {n_arc_arc}, enclosing {n_enclosing}, major {n_major}"
    );
}

/// One hand-built enclosing corner for the table below.
struct EnclosingCase {
    name: &'static str,
    corner: Point2<f64>,
    leg_in: OracleLeg,
    leg_out: OracleLeg,
    r: f64,
    /// The turn side the row is built to produce.
    sigma: f64,
}

/// The enclosing table: six corners whose geometry DEMANDS the
/// enclosing tangency (σ·τ = +1 on both legs with r > R on both — the
/// signed offset radius ρ = R − σ·τ·r negative on both legs). For an
/// arc leg the travel direction at the corner is τ·(−sin a, cos a)
/// where `a` is the corner's angle about the center, so
/// σ = sign(τ_in·τ_out·sin(a_out − a_in)). Every row states the σ it
/// expects, and the test re-derives σ and both ρ signs from the drawn
/// geometry, so a change to the sign convention makes the table RED
/// rather than vacuous.
///
/// **Both legs, always.** A ρ < 0 leg forces its partner to be an arc
/// that is also swallowed — see
/// [`an_enclosing_leg_forces_an_equally_enclosing_partner`], which pins
/// the three shapes that are ruled out.
fn enclosing_cases() -> Vec<EnclosingCase> {
    // sin(a_out - a_in) > 0 with tau_in = tau_out = +1 gives sigma = +1,
    // so sigma*tau = +1 on both legs and r > R swallows both.
    let c1 = p2(0.0, 0.0);
    // The mirror: both legs clockwise, sin(a_out - a_in) < 0, sigma = -1,
    // and sigma*tau = +1 again — the other sign pair that reaches rho < 0.
    let c2 = p2(-0.7, 0.35);
    let c3 = p2(0.25, 0.25);
    let c4 = p2(-0.15, 0.5);
    let c5 = p2(0.6, -0.5);
    let c6 = p2(-0.4, -0.9);
    vec![
        EnclosingCase {
            name: "sigma = tau = +1, equal carriers (R 0.2, r 0.5), right-angle turn",
            corner: c1,
            leg_in: arc_leg(c1, 0.0, 0.2, 1.0, 4.0, true),
            leg_out: arc_leg(c1, FRAC_PI_2, 0.2, 1.0, 4.0, false),
            r: 0.5,
            sigma: 1.0,
        },
        EnclosingCase {
            name: "sigma = tau = -1, equal carriers, right-angle turn (the mirror)",
            corner: c2,
            leg_in: arc_leg(c2, 0.0, 0.2, -1.0, 4.0, true),
            leg_out: arc_leg(c2, -FRAC_PI_2, 0.2, -1.0, 4.0, false),
            r: 0.5,
            sigma: -1.0,
        },
        EnclosingCase {
            name: "sigma = tau = +1, UNEQUAL carriers (0.15 in, 0.40 out, r 0.9): |rho| \
                   differs per leg, so one radius cannot stand in for the other",
            corner: c3,
            leg_in: arc_leg(c3, 0.0, 0.15, 1.0, 4.0, true),
            leg_out: arc_leg(c3, 2.4, 0.4, 1.0, 4.0, false),
            r: 0.9,
            sigma: 1.0,
        },
        EnclosingCase {
            name: "sigma = tau = -1, unequal carriers the other way round (0.40 in, \
                   0.15 out): the bigger |rho| swaps sides",
            corner: c4,
            leg_in: arc_leg(c4, 0.0, 0.4, -1.0, 4.0, true),
            leg_out: arc_leg(c4, -2.4, 0.15, -1.0, 4.0, false),
            r: 0.9,
            sigma: -1.0,
        },
        EnclosingCase {
            name: "sigma = tau = +1, SHALLOWLY enclosing: the outgoing carrier is 0.71 r, \
                   so its rho sits close to the sign flip rather than deep inside it",
            corner: c5,
            leg_in: arc_leg(c5, 0.0, 0.15, 1.0, 4.0, true),
            leg_out: arc_leg(c5, 3.0, 0.4, 1.0, 4.0, false),
            r: 0.56,
            sigma: 1.0,
        },
        EnclosingCase {
            name: "sigma = tau = -1, DEEPLY enclosing (R 0.05/0.08 against r 0.5): the \
                   fillet is an order of magnitude bigger than either carrier",
            corner: c6,
            leg_in: arc_leg(c6, 0.0, 0.05, -1.0, 4.0, true),
            leg_out: arc_leg(c6, -1.6, 0.08, -1.0, 4.0, false),
            r: 0.5,
            sigma: -1.0,
        },
    ]
}

/// **The enclosing (ρ < 0) class at the LATTICE DOOR — a permanent
/// property, pinned.** The raw builder reached the enclosing tangency
/// by AUTHORING the corner; the §2c lattice derives corners from the
/// anchored carriers and ranks every gated survivor with the lifted S8
/// ladder. For a corner whose geometry demands the enclosing tangency,
/// the pair's OTHER crossing always carries an ordinary (ρ > 0)
/// candidate pair whose setbacks are strictly smaller, and that
/// crossing cannot be excluded by the signed gates: the enclosing
/// setbacks wrap past the inter-crossing gap on both carriers, so any
/// anchors far enough out to fit the enclosing trims also leave the
/// other crossing inside its windows. So the ladder never had a reason
/// to emit an enclosing tangency, and now it has no route to one either:
/// the class is **ruled out permanently** by
/// `docs/ENCLOSING-TANGENCY-DESIGN.md`. The blend circle contains both
/// leg carriers, hence the corner, so the arc cannot touch the corner it
/// would round — which makes it no fillet OF that corner at all — and a
/// radius demanding it is answered by the typed
/// `PathError::FilletEnclosesLegCarrier`, gated on the construction's own
/// signed ρ before any candidate centre is computed.
///
/// This test pins exactly that: every table corner still DEMANDS the
/// class (σ, τ and both ρ signs re-derived from the drawn geometry), and
/// **the door refuses every one of them with that variant**, on every
/// shipped band, naming the swallowed side's carrier radius as the bound
/// and carrying the ρ this suite re-derives independently. The `Ok` arm
/// is unreachable under the ruling; it asserts the non-swallowing
/// property first (via `report_moved_refuse_pin`, the one home of that
/// arm) so a violation says what got built before it fails.
///
/// (The construction machinery underneath — signed offset radii, the
/// antipodal tangent-point flip — is unchanged and still describes the
/// enclosing candidates. It is the substrate this boundary is measured
/// against, and its ρ is what the refusal classifies — `sugar`'s
/// `offset_radius`, whose docs carry that purpose.)
///
/// Its mined twin is `enclosing_fillet_swallows_both_leg_carriers`,
/// which pins the same refusal at the same door on one corner nobody
/// would author; both `Ok` arms are `report_moved_refuse_pin`.
#[test]
fn the_lattice_door_never_emits_an_enclosing_tangency() {
    let cases = enclosing_cases();
    let (mut saw_ccw, mut saw_cw, mut saw_unequal) = (false, false, false);
    for case in &cases {
        let name = case.name;
        let sigma = turn_sign(case.corner, case.leg_in, case.leg_out);
        assert_eq!(
            sigma, case.sigma,
            "{name}: turn side came out sigma = {sigma}, but the row was authored for \
             sigma = {} — the row no longer describes the corner it was written to describe",
            case.sigma
        );
        for (side, leg) in [("incoming", case.leg_in), ("outgoing", case.leg_out)] {
            assert!(
                leg.is_enclosing(sigma, case.r),
                "{name}: the {side} leg is not rho < 0, so the row is not in the class it \
                 claims (rho = R - sigma*tau*r < 0 iff sigma*tau = +1 and r > R)"
            );
        }
        match build_corner(case.corner, case.leg_in, case.leg_out, case.r) {
            // The RULED branch: every table row refuses with the
            // enclosing-class variant, on every shipped band (measured
            // at 1e-6 / 1e-9 / 1e-12), and the refusal names the side it
            // would swallow — the payload's carrier radius is that
            // side's R and its offset radius is the row's own negative
            // ρ, re-derived here from the drawn geometry rather than
            // read back from the construction.
            Err(PathError::FilletEnclosesLegCarrier {
                side,
                carrier_radius,
                offset_radius,
                radius,
                largest_tangent_radius,
            }) => {
                assert_eq!(radius, case.r, "{name}: the refusal renamed the radius");
                // Every table row swallows BOTH carriers, so the refusal
                // says so rather than picking a side.
                assert_eq!(
                    side, None,
                    "{name}: both carriers are swallowed, so no single side is the story"
                );
                let radii = [case.leg_in, case.leg_out].map(|leg| {
                    let OracleLeg::Arc {
                        radius: big, tau, ..
                    } = leg
                    else {
                        panic!("{name}: the table is arc x arc by construction");
                    };
                    (big, big - sigma * tau * case.r)
                });
                // The CLASS bound is the TIGHTEST one: naming the other
                // carrier would endorse radii that re-refuse with this
                // same variant, now naming the smaller side.
                let tightest = radii[0].0.min(radii[1].0);
                assert!(
                    (carrier_radius - tightest).abs() < 1e-15,
                    "{name}: the refusal names carrier radius {carrier_radius}, not the \
                     tightest {tightest} — the bound it offers is the wrong number"
                );
                let rho = radii[0].1.min(radii[1].1);
                assert!(
                    offset_radius < 0.0 && (offset_radius - rho).abs() < 1e-15,
                    "{name}: the refusal reports rho {offset_radius}, not the row's own \
                     negative {rho}"
                );
                // **The endorsed radius must BUILD.** The existence
                // bound is the largest circle tangent to both carriers
                // at this corner, (R1 + R2 - d)/2, re-derived here from
                // the drawn geometry; the message endorses radii below
                // it, so this row rounds the corner at one and checks
                // the arc that comes back. A bound that endorsed the
                // class limit instead would send an author to radii
                // that refuse again — the defect this payload exists to
                // rule out.
                let bound = largest_tangent_radius
                    .unwrap_or_else(|| panic!("{name}: an arc x arc corner defines the bound"));
                let (o1, o2) = match (case.leg_in, case.leg_out) {
                    (OracleLeg::Arc { center: a, .. }, OracleLeg::Arc { center: b, .. }) => (a, b),
                    _ => panic!("{name}: the table is arc x arc by construction"),
                };
                let d = (o2.x - o1.x).hypot(o2.y - o1.y);
                let want = (radii[0].0 + radii[1].0 - d) / 2.0;
                assert!(
                    (bound - want).abs() < 1e-15,
                    "{name}: the endorsed bound {bound} is not the corner's largest tangent \
                     radius {want}"
                );
                assert!(
                    bound <= tightest,
                    "{name}: the endorsed bound {bound} is above the class bound {tightest}, \
                     so it endorses the very class this refusal rules out"
                );
                let endorsed = 0.99 * bound;
                let lp = build_corner(case.corner, case.leg_in, case.leg_out, endorsed)
                    .unwrap_or_else(|e| {
                        panic!(
                            "{name}: the refusal endorses radii below {bound}, but {endorsed} \
                             refuses with {e:?} — a dead recourse"
                        )
                    });
                let ctx = || format!("{name} @ endorsed radius");
                let (t1, t2, b) = fillet_segment(&lp, endorsed, &ctx);
                let (pf, _) = circle_from_bulge(t1, t2, b);
                for (center, big) in [(o1, radii[0].0), (o2, radii[1].0)] {
                    assert_swallows_nothing(pf, center, big, endorsed, &ctx);
                }
            }
            Err(other) => panic!(
                "{name}: a radius demanding the enclosing class must refuse with \
                 FilletEnclosesLegCarrier (docs/ENCLOSING-TANGENCY-DESIGN.md), not with \
                 {other:?}"
            ),
            Ok(lp) => {
                // A build here breaks the ruling.
                // `report_moved_refuse_pin` checks the class BEFORE it
                // reports, so the failure names what got built: whatever
                // it is, it must not be the enclosing tangency (the
                // emitted fillet swallows neither carrier).
                let carriers = [case.leg_in, case.leg_out].map(|leg| {
                    let OracleLeg::Arc { center, radius, .. } = leg else {
                        panic!("{name}: the table is arc x arc by construction");
                    };
                    (center, radius)
                });
                report_moved_refuse_pin(&lp, case.r, &carriers, "the row", &|| name.to_string());
            }
        }
        saw_ccw |= sigma > 0.0;
        saw_cw |= sigma < 0.0;
        if let (OracleLeg::Arc { radius: a, .. }, OracleLeg::Arc { radius: b, .. }) =
            (case.leg_in, case.leg_out)
        {
            saw_unequal |= a != b;
        }
    }
    // The table's SPAN is part of the claim: a row deleted or retuned
    // into a narrower shape must fail here rather than pass quietly.
    assert!(
        saw_ccw && saw_cw,
        "both turn sides (sigma = +/-1) must appear"
    );
    assert!(
        saw_unequal,
        "a row with UNEQUAL leg carriers must appear, or |rho| is the same number twice"
    );
}

/// **Why the enclosing table has no line leg, no opposite-sense partner
/// and no ρ > 0 partner: those corners do not exist.** Found while
/// building the table (each shape was a row until the sugar refused it),
/// so it is pinned rather than left as a comment.
///
/// Let leg 1 be swallowed: |P − O₁| = r − R₁ with R₁ < r, where P is the
/// fillet center. The corner is a point of carrier 1, so it sits at
/// distance R₁ from O₁. Each partner shape then collapses:
///
/// - *Line partner.* The fillet is tangent to the line, so d(P, line) =
///   r; the corner is on the line, so d(O₁, line) ≤ R₁. The triangle
///   inequality gives d(O₁, line) ≥ r − (r − R₁) = R₁. Equality forces
///   the line to touch carrier 1 exactly at the corner — the legs are
///   TANGENT there and there is no corner to round.
/// - *Opposite-sense arc partner* (σ·τ₂ = −1, ρ₂ = R₂ + r): |O₁O₂| ≥
///   (R₂ + r) − (r − R₁) = R₁ + R₂, while a shared corner forces
///   |O₁O₂| ≤ R₁ + R₂. Equality again — externally tangent carriers.
/// - *Same-sense arc partner with R₂ > r* (ρ₂ = R₂ − r > 0): |O₁O₂| ≤
///   (r − R₁) + (R₂ − r) = R₂ − R₁, while a shared corner forces
///   |O₁O₂| ≥ |R₁ − R₂| = R₂ − R₁. Equality — internally tangent
///   carriers.
///
/// So ρ < 0 on one leg means ρ < 0 on BOTH, which is the shape the table
/// spans and the shape `enclosing_fillet_swallows_both_leg_carriers` was
/// named for. The sugar refuses all three as the enclosing class it is —
/// the ρ < 0 leg is the one the refusal names, and the partner never has
/// to be examined, because a swallowed carrier is already a corner no
/// fillet of that radius can touch (`docs/ENCLOSING-TANGENCY-DESIGN.md`).
/// Before that ruling these rows came back as `OffsetCarriersDisjoint`,
/// the same inequality read from the offset side — true, and about the
/// offset carriers rather than about the fillet the author asked for.
#[test]
fn an_enclosing_leg_forces_an_equally_enclosing_partner() {
    let c = p2(0.4, -0.2);
    let r = 0.6;
    let impossible: [(&str, OracleLeg, OracleLeg); 4] = [
        (
            "line incoming, swallowed arc outgoing",
            line_leg(c, 0.0, 1.8),
            arc_leg(c, PI, 0.25, 1.0, 4.0, false),
        ),
        (
            "swallowed arc incoming, line outgoing",
            arc_leg(c, 0.0, 0.25, 1.0, 4.0, true),
            line_leg(c, PI, 1.8),
        ),
        (
            "swallowed arc incoming, OPPOSITE-sense arc outgoing (sigma*tau = -1)",
            arc_leg(c, 0.0, 0.25, 1.0, 4.0, true),
            arc_leg(c, -FRAC_PI_2, 0.3, -1.0, 4.0, false),
        ),
        (
            "swallowed arc incoming, same-sense arc outgoing but LARGER than the fillet \
             (rho > 0)",
            arc_leg(c, 0.0, 0.25, 1.0, 4.0, true),
            arc_leg(c, FRAC_PI_2, 1.5, 1.0, 1.3, false),
        ),
    ];
    for (name, leg_in, leg_out) in impossible {
        // The row only says anything if a leg really is swallowed under
        // the sigma this corner produces.
        let sigma = turn_sign(c, leg_in, leg_out);
        assert!(
            leg_in.is_enclosing(sigma, r) || leg_out.is_enclosing(sigma, r),
            "{name}: the row must contain a rho < 0 leg to be about the enclosing class"
        );
        match build_corner(c, leg_in, leg_out, r) {
            Err(PathError::FilletEnclosesLegCarrier {
                side,
                offset_radius,
                carrier_radius,
                largest_tangent_radius,
                ..
            }) => {
                assert!(
                    offset_radius < 0.0 && carrier_radius < r,
                    "{name}: the refusal must name the swallowed carrier (rho \
                     {offset_radius}, R {carrier_radius} against r {r})"
                );
                // These corners are the degenerate ones: exactly one leg
                // is swallowed, so the refusal names that side and
                // endorses NO radius — the existence bound is not
                // defined here, and a class bound alone is necessary
                // rather than sufficient.
                assert!(side.is_some(), "{name}: one leg is swallowed, not both");
                assert_eq!(
                    largest_tangent_radius, None,
                    "{name}: a degenerate partner defines no largest tangent circle"
                );
            }
            other => panic!(
                "{name}: a swallowed carrier next to this partner is geometrically \
                 impossible, and the swallowing itself is refused typed, so the sugar must \
                 refuse with FilletEnclosesLegCarrier; got {other:?}"
            ),
        }
    }
}

/// F3 attack, now the MAJOR-1 regression pin, restated at the lattice
/// door: on the vesica with SHORT legs near the BOTTOM corner, both
/// carrier intersections exist. Before the fix the arc setback was
/// reduced into [0, 2π), so the TOP corner's candidate read as a huge
/// positive setback (the long way round), passed the corner-side test,
/// and the refusal rendered 8.15 m against a 0.14 m leg — numbers
/// belonging to a corner the author never named. With SIGNED setbacks
/// (and the harness bracketing the drawn corner, as every lattice
/// author must), the refusal describes the bottom corner's own
/// candidate: a setback of the same order as the legs, never a
/// wrap-around circumference.
#[test]
fn overrun_attribution_names_the_authored_corners_candidate() {
    let s3 = 3.0f64.sqrt();
    let deg = |d: f64| d.to_radians();
    // corner (0, -s3) on both circles (centers (-1,0),(1,0), R=2);
    // both legs only a few degrees long.
    let far_in = p2(-1.0 + 2.0 * deg(296.0).cos(), 2.0 * deg(296.0).sin());
    // 70 degrees past the bottom corner: far enough that the TOP corner
    // reads Negative on the signed reach gate, close enough to keep the
    // bottom corner in its window.
    let far_out = p2(1.0 + 2.0 * deg(310.0).cos(), 2.0 * deg(310.0).sin());
    let leg = |far: Point2<f64>, centre: Point2<f64>| OracleLeg::Arc {
        center: centre,
        radius: 2.0,
        tau: 1.0,
        far_angle: (far.y - centre.y).atan2(far.x - centre.x),
    };
    let err = build_corner(
        p2(0.0, -s3),
        leg(far_in, p2(-1.0, 0.0)),
        leg(far_out, p2(1.0, 0.0)),
        0.5,
    )
    .expect_err("short legs must refuse");
    match err {
        PathError::AnchorOutsideTrimmedExtent {
            side,
            setback,
            available,
            ..
        } => {
            // The bottom corner's OWN candidate: it overruns the 4-degree
            // (0.1396 m) leg, but only by a factor of a few — not by the
            // 4.4 m the top corner's wrap-around reading produced.
            assert_eq!(side, profile::FilletLeg::Incoming);
            assert!(
                (available - 0.139_626_340_159_546_53).abs() < 1e-12,
                "leg length {available}"
            );
            assert!(
                setback > available && setback < 1.0,
                "setback {setback}: expected the near candidate's own overrun, not a \
                 wrap-around distance to the corner the author never named"
            );
            // Half the carrier's circumference is the hard ceiling a
            // signed setback can never exceed (|dtheta| <= pi, R = 2).
            assert!(setback < core::f64::consts::PI * 2.0, "setback {setback}");
        }
        other => panic!("unexpected refusal {other:?}"),
    }
}

/// **The conditioning gate's mined witness (M8).** The corner
/// `CAD_FUZZ_SEED=0x814d97e9cec0d36e CAD_FUZZ_EFFORT=1` iteration 89
/// drew, which is what turned this sweep red: `check_corner`'s (b)
/// assertion measured `t2` sitting 2.29e-9 off its carrier against a
/// 1e-9 tolerance.
///
/// The construction was not wrong — it was *unconditioned*, and nothing
/// said so. The outgoing leg's carrier radius (0.567 322 99) sits
/// 2.9e-4 from the authored fillet radius (0.567 033 69) on the side the
/// corner turns toward, so its offset radius ρ = R − σ·τ·r collapses to
/// that difference, and the corner's carriers are 2.27 m apart.
///
/// # This corner is now BUILT, not refused, at ε = 1e-9
///
/// Twice retargeted, and the history is the point of keeping it. It was
/// first returned wrong (2.29e-9 off carrier, reported as success); then
/// refused, once `fillet_offset_lever` existed; and it now BUILDS, with
/// a residual of **exactly zero** — but on the pair's TWIN crossing,
/// whose offset signs are the other ones, so the exact residual is not
/// the collapsed lever surviving the measured-spoke scaling. The body
/// says what the building bands do and do not exercise, with the
/// numbers. The mined conditioning is what the ε = 1e-12 leg decides on.
///
/// So the ε key moved rather than went away: |ρ| = 2.89e-4 against a
/// least supported lever of 7.31e-2 at ε = 1e-12 (short by 253x) but of
/// 7.31e-5 at ε = 1e-9 (comfortably clear). The crossover sits at
/// ε ≈ 2.53e-10.
///
/// That threshold is a literal below, derived from the shipped
/// `LEVER_ULPS`, and it is deliberately a tripwire: moving the constant
/// is a capability change and should require touching a witness that
/// says which corners it moves.
///
/// Pinned as a fixture rather than left to the seed (the harness's shape-2
/// rule): the witness is written down, so it is checked on every run
/// instead of once per lucky draw. The numbers below are the draw's, to
/// the bit.
#[test]
fn an_uncertifiable_tangent_point_refuses_instead_of_being_returned() {
    let corner = p2(-0.036_538_048_808_474_78, -0.639_153_338_141_905_2);
    let r = 0.567_033_689_456_740_4;
    let leg_in = OracleLeg::Arc {
        center: p2(-1.716_849_619_579_590_6, -0.386_316_949_082_364_86),
        radius: 1.699_227_240_394_869,
        tau: -1.0,
        far_angle: 6.965_749_569_640_534_5,
    };
    let leg_out = OracleLeg::Arc {
        center: p2(0.526_054_394_925_831_4, -0.712_263_623_663_518_8),
        radius: 0.567_322_987_015_324_7,
        tau: 1.0,
        far_angle: 5.717_696_914_354_667,
    };
    // The row only says anything if this really is the near-collapse it
    // was mined for: same-sense, r within 3e-4 of the outgoing carrier.
    let sigma = turn_sign(corner, leg_in, leg_out);
    let OracleLeg::Arc {
        radius: r_out, tau, ..
    } = leg_out
    else {
        panic!("the outgoing leg is an arc by construction")
    };
    let rho = r_out - sigma * tau * r;
    assert!(
        rho.abs() < 3e-4,
        "the pin's outgoing offset lever is {rho}, not the near-collapse it was mined for"
    );
    // THE OUTCOME IS eps-KEYED, and saying so is the point of the row.
    // The gate's least lever is `C * R2 * scale^2 / (d * band.zero())`,
    // so a looser ambient band affords MORE conditioning. Asserting
    // either outcome unconditionally would be exactly the eps-blindness
    // this suite has been removing elsewhere — and CI caught this very
    // row doing so on the 1e-6 leg once already.
    //
    // So: at a band this corner's conditioning cannot support it must
    // REFUSE typed; at a band that can support it, it must BUILD, and
    // the fillet it returns must be tangent to the outgoing carrier at
    // the ULP FLOOR rather than merely within the band.
    //
    // WHAT THE BUILDING BANDS DO NOT CHECK, and it is not a tolerance
    // question. This pin's two carriers sit within 8.61e-5 of external
    // tangency (|O1O2| = 2.266 464 1 against R1 + R2 = 2.266 550 2 —
    // NOT the 3e-4 asserted above, which bounds rho and is a different
    // quantity), so the pair is a hairline lens whose two crossings are
    // 1.71e-2 apart, and the harness's bracketing does not exclude the
    // twin: `mirror_excluded` is FALSE here. The ladder rounds the twin
    // crossing, where the turn side is the other one — the returned fillet has |P - O_out| =
    // R_out + r and |P - O_in| = R_in - r, i.e. the opposite offset sign
    // on BOTH legs from the rho this row re-derives above, which the
    // build arm now ASSERTS rather than merely recording. So the
    // collapsed lever is not what the building bands exercise, and
    // `check_corner`'s battery cannot run here: its oracle speaks about
    // the DRAWN corner, exactly as the sweep's own mirror-survivor skip
    // says. The mined conditioning decides the outcome on the REFUSING
    // band, where the typed error carries `offset_radius` and this row
    // checks it against rho directly.
    let eps = tol().eps();
    if 2.53e-10 < eps {
        let lp = build_corner(corner, leg_in, leg_out, r).unwrap_or_else(|e| {
            panic!(
                "at eps = {eps:e} this corner's conditioning IS supported, so it must \
                 build rather than refuse; got {e:?}"
            )
        });
        let (t1, t2, b) = fillet_segment(&lp, r, &|| "uncertifiable pin".to_string());
        // WHICH fillet came back, asserted rather than described. Every
        // leg's centre distance is the one the OTHER turn side predicts
        // — |P - O| = |R - (-sigma)*tau*r| on both — which is the twin
        // crossing and not this corner's demand. If the ladder ever
        // returns the drawn corner's own candidate here, this goes red
        // and the row must be re-mined before its ulp claim means what
        // its prose says.
        let (pf, _) = circle_from_bulge(t1, t2, b);
        for (side, leg) in [("incoming", leg_in), ("outgoing", leg_out)] {
            let twin = leg.center_distance_residual(pf, -sigma, r);
            assert!(
                twin < 1e-9,
                "at eps = {eps:e} the {side} leg's tangency is no longer the twin \
                 crossing's (residual {twin:e} against the flipped offset sign)"
            );
        }
        let res = leg_out.carrier_residual(corner, t2);
        // 8 ulps of the ~1 m scene, not `eps`: the measured residual is
        // 0.0, and holding it to the band would be a far weaker claim
        // than the construction actually supports. What it is NOT is
        // evidence about the measured-spoke scaling surviving a
        // collapsed lever — this crossing's offset radius is R_out + r,
        // so there is no 1/rho amplification here to survive. The
        // scaling's regression pin is
        // `an_ill_conditioned_corner_lands_its_tangent_point_on_the_carrier`.
        let bound = 8.0 * f64::EPSILON;
        assert!(
            res <= bound,
            "at eps = {eps:e} the corner builds, so its tangent point must sit on the \
             carrier at the ulp floor: off by {res:e}, bound {bound:e}"
        );
        return;
    }
    match build_corner(corner, leg_in, leg_out, r) {
        Err(PathError::FilletOffsetLeverTooShort {
            side,
            offset_radius,
            least_lever,
            margin,
            ..
        }) => {
            assert_eq!(side, profile::FilletLeg::Outgoing);
            assert!(
                (offset_radius - rho).abs() < 1e-15,
                "the refusal reports offset lever {offset_radius}, not the corner's {rho}"
            );
            // Short by more than a decade: the gate is not deciding a
            // hairline here, it is refusing a corner that misses the
            // supported conditioning by 253x at eps = 1e-12.
            assert!(
                least_lever > 10.0 * offset_radius.abs(),
                "least lever {least_lever} against |rho| {}: the pin no longer sits well \
                 inside the refused region",
                offset_radius.abs()
            );
            assert!(
                margin < 0.0,
                "margin {margin} must be the negative shortfall"
            );
        }
        other => panic!(
            "at eps = {eps:e} this corner's conditioning is NOT supported, so it must \
             refuse typed rather than return a tangent point it cannot place; got {other:?}"
        ),
    }
}

/// The mined enclosing corner (F1's deterministic pin, coordinates
/// nobody would author), at the permanent boundary the table test pins:
/// the geometry still DEMANDS the enclosing tangency (both rho < 0,
/// re-derived), and **the door refuses with
/// `PathError::FilletEnclosesLegCarrier`**, on every band. That is the
/// ruling of `docs/ENCLOSING-TANGENCY-DESIGN.md`, not a finding about
/// today's ladder: a blend circle that swallows both leg carriers
/// swallows the corner, so it can never touch the corner it would round.
/// The `Ok` arm is unreachable under that ruling; it checks the
/// non-swallowing property first — via `report_moved_refuse_pin`, the one
/// home of that arm — and only then reports the violation.
/// The table twin is `the_lattice_door_never_emits_an_enclosing_tangency`,
/// which pins the same refusal over six authored rows.
#[test]
fn enclosing_fillet_swallows_both_leg_carriers() {
    let corner = p2(0.4141246232685536, -0.9332926788663134);
    let r = 0.7730763477423346;
    let o1 = p2(0.33261753191949683, -1.1228461282388256);
    let fa1: f64 = -1.477819896068483;
    let o2 = p2(0.4884663916168746, -0.8611854913524928);
    let fa2: f64 = 6.685799873422015;
    let r1 = (corner.x - o1.x).hypot(corner.y - o1.y);
    let r2 = (corner.x - o2.x).hypot(corner.y - o2.y);
    assert!(r > r1 && r > r2, "the pin must be the enclosing case");
    let leg_in = OracleLeg::Arc {
        center: o1,
        radius: r1,
        tau: 1.0,
        far_angle: fa1,
    };
    let leg_out = OracleLeg::Arc {
        center: o2,
        radius: r2,
        tau: 1.0,
        far_angle: fa2,
    };
    let sigma = turn_sign(corner, leg_in, leg_out);
    for (side, leg) in [("incoming", leg_in), ("outgoing", leg_out)] {
        assert!(
            leg.is_enclosing(sigma, r),
            "the {side} leg must still demand rho < 0"
        );
    }
    match build_corner(corner, leg_in, leg_out, r) {
        // The RULED branch: the mined geometry refuses with the
        // enclosing-class variant on every shipped band (measured at
        // 1e-6 / 1e-9 / 1e-12), naming a swallowed side whose carrier
        // radius bounds the fillet and whose rho is negative.
        Err(PathError::FilletEnclosesLegCarrier {
            side,
            carrier_radius,
            offset_radius,
            radius,
            largest_tangent_radius,
        }) => {
            assert_eq!(radius, r, "the refusal renamed the radius");
            assert_eq!(side, None, "this corner swallows BOTH carriers");
            assert!(
                (carrier_radius - r1.min(r2)).abs() < 1e-15,
                "the refusal names carrier radius {carrier_radius}, not the tightest {}",
                r1.min(r2)
            );
            assert!(
                offset_radius < 0.0 && carrier_radius < r,
                "the refusal's bound must be the swallowed carrier: rho {offset_radius}, \
                 R {carrier_radius} against r {r}"
            );
            assert!(
                leg_in.is_enclosing(sigma, r) && leg_out.is_enclosing(sigma, r),
                "both legs must still demand the class the refusal names"
            );
            // The endorsed radius BUILDS: the mined corner rounds at a
            // radius below its largest tangent circle, and what comes
            // back swallows neither carrier.
            let d = (o2.x - o1.x).hypot(o2.y - o1.y);
            let bound = largest_tangent_radius.expect("an arc x arc corner defines the bound");
            assert!(
                (bound - (r1 + r2 - d) / 2.0).abs() < 1e-15,
                "the endorsed bound {bound} is not this corner's largest tangent radius"
            );
            let endorsed = 0.99 * bound;
            let lp = build_corner(corner, leg_in, leg_out, endorsed).unwrap_or_else(|e| {
                panic!("the endorsed radius {endorsed} refuses with {e:?} — a dead recourse")
            });
            let ctx = || "enclosing_fillet_swallows_both_leg_carriers @ endorsed".to_string();
            let (t1, t2, b) = fillet_segment(&lp, endorsed, &ctx);
            let (pf, _) = circle_from_bulge(t1, t2, b);
            assert_swallows_nothing(pf, o1, r1, endorsed, &ctx);
            assert_swallows_nothing(pf, o2, r2, endorsed, &ctx);
        }
        Err(other) => panic!(
            "the mined enclosing corner must refuse with FilletEnclosesLegCarrier \
             (docs/ENCLOSING-TANGENCY-DESIGN.md), not with {other:?}"
        ),
        Ok(lp) => {
            let ctx = || "enclosing_fillet_swallows_both_leg_carriers".to_string();
            report_moved_refuse_pin(&lp, r, &[(o1, r1), (o2, r2)], "the mined corner", &ctx);
        }
    }
}

/// **The regression pin for `sugar::Leg::tangent_point`'s measured
/// spoke.**
///
/// A written-down witness, not a seed (the harness's shape 2): the fuzz
/// above searches, this pins. The corner is the WORST the
/// `fillet_offset_lever` gate accepts, found by sweeping 2.4M corners
/// against the old scaling at the **tightest ε leg** — which is what makes
/// this a fixture rather than an ε-keyed claim. Accepted at ε = 1e-12, it
/// is accepted at every looser ε, so the assertion is one claim on all
/// three legs.
///
/// # The bound, and where it comes from
///
/// Over those same 2.4M gate-accepted corners, worst off-carrier residual:
///
/// | scaling | worst | on this corner |
/// |---|---|---|
/// | nominal `R/ρ` (before) | **90.6 ulps** | 90.6 ulps (2.01e-14) |
/// | measured spoke (now) | **1.5 ulps** | see below |
///
/// So 8 ulps sits 5.3x above everything the construction produced across
/// the whole accepted region, and 11x below what the old one produced
/// here. Wide enough that no rounding difference trips it; narrow enough
/// that restoring `R/ρ` fails it immediately (verified by doing exactly
/// that).
///
/// Ulps are of the **coordinate** magnitude, not of R. The residual floors
/// at an ulp of the scene because `t − O` is a difference of O(1) points,
/// however exact the arithmetic after it; a bound in ulps of R would be
/// unreachable on a small carrier.
#[test]
fn an_ill_conditioned_corner_lands_its_tangent_point_on_the_carrier() {
    // The scene's coordinate magnitude — see the docs above on units.
    const SCENE: f64 = 1.0;
    const ULPS: f64 = 8.0;

    let corner = p2(-0.417_819_980_559_473_56, -0.034_224_129_413_008_564);
    // The mined corner walked in REVERSE (legs swapped, windings
    // negated; sigma*tau per leg — and with it the collapsed offset
    // lever — is invariant): the orientation whose anchors bracket the
    // drawn corner at the lattice door. The ill-conditioned leg is now
    // the INCOMING one.
    let leg_in = arc_leg(
        corner,
        4.638_961_400_716_957,
        0.165_877_420_990_701_74,
        1.0,
        1.869_369_386_166_155,
        true,
    );
    let leg_out = arc_leg(
        corner,
        1.785_843_803_375_859_5,
        1.107_622_274_793_787_8,
        -1.0,
        2.564_546_871_371_803,
        false,
    );
    let r = 0.102_474_035_114_155_93;

    // It must BUILD, on every leg. If a future gate change refuses this
    // corner that is a capability regression, and this is where it shows:
    // the construction demonstrably places it to within ULPS.
    let lp = build_corner(corner, leg_in, leg_out, r).unwrap_or_else(|e| {
        panic!(
            "the ill-conditioned pin must still build at eps = {:e}; got {e:?}",
            tol().eps()
        )
    });

    // The full oracle battery first, so this fixture cannot assert less
    // than the sweep does — then the tight residual claim the sweep's 1e-9
    // threshold cannot make, which is the whole point of the row.
    check_corner(corner, leg_in, leg_out, r, &lp, &|| {
        "ill-conditioned tangent-point pin".to_string()
    });

    let (t1, _, _) = fillet_segment(&lp, r, &|| "ill-conditioned pin".to_string());
    let res_in = leg_in.carrier_residual(corner, t1);
    let bound = ULPS * f64::EPSILON * SCENE;
    assert!(
        res_in <= bound,
        "t1 off its carrier by {res_in:e} ({:.1} ulps), bound {bound:e} \
         ({ULPS} ulps). The nominal-rho scaling this replaced put THIS \
         corner 2.01e-14 (90.6 ulps) out — see Leg::tangent_point.",
        res_in / (f64::EPSILON * SCENE)
    );
}

/// **The collapsed offset lever refuses TYPED on every band — and the
/// band decides WHICH gate speaks, pinned exactly.**
///
/// The mined witness collapses the outgoing offset lever to
/// |ρ| = 1.2e-7, and that collapse is STRUCTURAL company: the fillet
/// centre must sit |ρ_in| from o_in and ~0 from o_out, so the carriers
/// themselves are within |ρ| of mutual tangency — a hairline lens. The
/// raw builder, handed the AUTHORED corner, sailed past that and hit
/// the lever gate. The lattice DERIVES the corner, and on a hairline
/// lens the corner-turn classification is band-keyed:
///
/// - at ε = 1e-6 and 1e-9 the derived corner's turn margin lands in
///   the tangent band, so the pair refuses one gate earlier as
///   `NoCornerForFillet { CarriersParallel }` — the same degeneracy,
///   classified at the carriers;
/// - at ε = 1e-12 the turn is definite, the construction reaches the
///   M8 conditioning gate, and `FilletOffsetLeverTooShort` fires its
///   DEFINITE arm — and, per the resolve doctrine, ABORTS the whole
///   resolve rather than being outranked by the hairline pair's twin
///   corner (the silent-build class this pin caught once already).
///
/// What must never happen — on any band — is a build.
#[test]
fn a_collapsed_offset_lever_refuses_typed_at_every_band() {
    let corner = p2(-0.466_393_541_070_097, -0.036_421_594_587_948_69);
    // The mined orientation, verbatim: the collapsed leg is the
    // OUTGOING one — the side whose offset lever the M8 gate measures.
    let leg_in = arc_leg(
        corner,
        2.166_517_959_434_531_6,
        2.271_512_339_781_247,
        1.0,
        1.205_372_593_734_55,
        true,
    );
    let leg_out = arc_leg(
        corner,
        2.166_547_354_045_76,
        0.672_869_286_050_959_2,
        1.0,
        2.679_318_364_718_840_3,
        false,
    );
    let r = 0.672_869_165_673_333_2;
    let eps = tol().eps();

    match build_corner(corner, leg_in, leg_out, r) {
        Err(PathError::FilletOffsetLeverTooShort {
            side,
            carrier_radius,
            offset_radius,
            least_lever,
            margin,
        }) => {
            // The lever gate's own definite arm (ε = 1e-12 and
            // tighter): naming the exposed leg and carrying the lever,
            // the threshold and the shortfall.
            assert!(
                eps < 1e-10,
                "at eps = {eps:e} the hairline lens should refuse at the carriers, \
                 not reach the lever gate"
            );
            assert_eq!(side, profile::FilletLeg::Outgoing);
            assert!(
                (carrier_radius - 0.672_869_286_050_959_2).abs() < 1e-15,
                "the refusal reports carrier radius {carrier_radius}, not the leg's"
            );
            assert!(
                offset_radius.abs() < 1e-6,
                "the pin's offset lever is {offset_radius}, not the collapse it was mined for"
            );
            assert!(
                least_lever > 10.0 * offset_radius.abs(),
                "least lever {least_lever} against |rho| {}: the pin must sit well inside \
                 the refused region",
                offset_radius.abs()
            );
            assert!(
                margin < 0.0,
                "margin {margin} must be the definite arm's negative shortfall"
            );
        }
        Err(PathError::NoCornerForFillet {
            reason: PathNoCornerReason::CarriersParallel,
            ..
        }) => {
            // The carrier-level tangency classification (ε = 1e-9 and
            // looser): the hairline lens's turn margin is in-band, so
            // the funnel refuses one gate earlier.
            assert!(
                eps >= 1e-10,
                "at eps = {eps:e} the turn is definite, so the lever gate — not the \
                 carrier classification — must speak"
            );
        }
        Ok(_) => panic!(
            "a collapsed offset lever must never BUILD: a tangent point placed over a \
             lever no band supports is the silent-wrong-geometry class this pin exists \
             to keep refused"
        ),
        Err(other) => panic!("expected a typed refusal of the collapsed lever, got {other:?}"),
    }
}
