//! Adversarial e2e review probes for M5 S2 (arc-leg fillet sugar),
//! promoted from the review scratch into the permanent suite.
//!
//! Independent re-derivations: fuzzed tangency residuals for the
//! offset-carrier construction (F1), an atan2-based bulge oracle (F2),
//! k_stats sequence invariance and first-candidate attribution (F3),
//! and enclosing-case (r > R) coverage the original suite lacked. The
//! oracles are written from the geometry rather than from `sugar.rs`:
//! the signed offset radius ρ = R − σ·τ·r is re-derived, the bulge is
//! checked against an `atan2` sweep, and setback/extent is recomputed
//! with `rem_euclid`.
//!
//! Two notes on adoption:
//!
//! - `overrun_attribution_names_the_authored_corners_candidate` was the
//!   review's MAJOR-1 *repro* — it asserted the buggy wrap-around
//!   setback. It is inverted here into the regression pin for the fix.
//! - the corner fuzz draws from `test_utils::fuzz`: a fresh seed per run
//!   (logged unconditionally) and a corner count that is a multiple of
//!   `CAD_FUZZ_EFFORT`. It no longer keeps the review's fixed 20k
//!   corners — a pinned seed made this a replay corpus rather than a
//!   fuzzer. `CAD_FUZZ_EFFORT` restores that depth and more.
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
//! This file carried exactly that mistake. The enclosing (ρ < 0)
//! tangency was claimed by an absolute floor, `n_enclosing >= 1`. Those
//! corners occur ~1 per 1 000 random draws (measured 58/60 000,
//! 60/60 000, 56/60 000) even though one corner in five is already drawn
//! under `enclosing_bias`, so the floor forced the count to 12 500 —
//! roughly 8x what the counterexample search itself asks for. The row
//! paid 8x to STUMBLE ONTO a class it can simply BUILD.
//!
//! So the class is built. [`enclosing_tangency_is_constructed_not_stumbled_upon`]
//! inverts the ρ algebra — ρ = R − σ·τ·r < 0 ⟺ σ·τ = +1 and r > R — and
//! lays out six corners spanning both sign pairs (σ = τ_in = τ_out = +1
//! and = −1), equal and unequal leg carriers, and ρ from just past the
//! sign flip to an order of magnitude past it. Those rows run the SAME
//! oracle battery the sweep runs: [`check_corner`] is one function
//! called from both sides, so a fixture cannot quietly assert less than
//! the fuzz did. The floor is gone and the count is `fuzz::scaled(1_500)`.
//!
//! Building the class also settled its SHAPE, which sampling never did:
//! ρ < 0 on one leg forces ρ < 0 on the other, so a swallowed carrier
//! never appears beside a line leg, an opposite-sense arc, or an arc
//! bigger than the fillet. Those three are geometrically impossible, not
//! merely rare, and [`an_enclosing_leg_forces_an_equally_enclosing_partner`]
//! pins each one's refusal with the inequality that rules it out.
//!
//! The principle, since it generalizes past this file: **a witness
//! belongs in a deterministic fixture, a fuzz belongs on the
//! counterexample search.** Mixing them makes one sample count carry two
//! obligations, and only one of the two is safe to cut.
//!
//! The floors that remain (accepted corners, arc-by-arc corners) are
//! FRACTIONS of the corner count, so they scale with `CAD_FUZZ_EFFORT`
//! instead of turning red at low effort, and they describe the bulk of
//! the distribution rather than a rare class.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::tol;
use geom_core::Point2;
use profile::{ArcSweep, FilletLegShape, ProfileError, ProfileLoop};
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
    fn shape(&self) -> FilletLegShape<f64> {
        match *self {
            OracleLeg::Line { .. } => FilletLegShape::Line,
            OracleLeg::Arc { center, tau, .. } => FilletLegShape::Arc {
                center,
                sweep: if tau > 0.0 {
                    ArcSweep::Ccw
                } else {
                    ArcSweep::Cw
                },
            },
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

/// Author the corner through the public sugar and close it.
fn build_corner(
    corner: Point2<f64>,
    leg_in: OracleLeg,
    leg_out: OracleLeg,
    r: f64,
) -> Result<ProfileLoop<f64>, ProfileError> {
    let head = leg_in.far_point(corner);
    let next = leg_out.far_point(corner);
    Ok(ProfileLoop::builder(head)
        .fillet_corner(leg_in.shape(), corner, leg_out.shape(), next, r, tol())?
        .close())
}

/// What one checked corner contributed to the sweep's coverage report.
#[derive(Clone, Copy)]
struct CornerCounts {
    arc_legs: u64,
    arc_arc: u64,
    enclosing: u64,
    major: u64,
}

/// **The oracle battery for one constructed fillet corner.** One
/// function, called by the fuzz for every accepted draw and by the
/// deterministic fixtures for every hand-built row — so a fixture
/// literally cannot check fewer properties than the sweep does.
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
    let nv = lp.vertices.len();
    assert!(nv == 2 || nv == 3, "chain shape: {nv} vertices — {}", ctx());
    let t2 = lp.vertices[nv - 1].pos;
    let t1 = lp.vertices[nv - 2].pos;
    let b = lp.vertices[nv - 2].bulge;
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
                    // rho < 0: the fillet swallows this leg's carrier, so
                    // the tangency is INTERNAL with the carrier inside the
                    // fillet — |P-O| + R = r, not |P-O| = R + r. Implied by
                    // the residual above, asserted separately because it is
                    // the claim the sign of rho is actually making.
                    counts.enclosing += 1;
                    let d = (pf.x - center.x).hypot(pf.y - center.y);
                    assert!(
                        d + radius < r + 1e-9,
                        "enclosing leg: |P-O| {d} + R {radius} exceeds r {r} — {}",
                        ctx()
                    );
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
    // now lives in `enclosing_tangency_is_constructed_not_stumbled_upon`
    // as built geometry — see the module docs. Nothing forces the count
    // upward any more, so it is a smoke level like every other sweep's,
    // and `CAD_FUZZ_EFFORT` buys depth when depth is wanted.
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
    // safe floor when the class it names is common; the rare class that
    // used to sit here alongside them is a fixture now. Measured at 1 500
    // corners over 22 fresh-seed runs: ~955 accepted against a floor of
    // 38, ~277 arc-by-arc against a floor of 4. Neither is a witness
    // search in disguise.
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
    // `n_enclosing` and `n_major` are a COVERAGE REPORT, not a gate.
    //
    // `n_enclosing` used to be gated by an absolute floor, which is what
    // forced this count to 12 500; the class is now constructed
    // deterministically, so what the sweep stumbles onto is information
    // about the draw distribution and nothing has to hold.
    //
    // `n_major` comes out 0, which is the fuzz corroborating the bound
    // `fillet_bulge`'s docs argue for — the corner-side extent gates keep
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

/// The enclosing table, derived rather than sampled.
///
/// ρ = R − σ·τ·r is negative exactly when σ·τ = +1 **and** r > R, so a
/// row is authored by picking the two carrier angles that force the
/// wanted σ and then giving both carriers a radius below the fillet's.
/// For an arc leg the travel direction at the corner is τ·(−sin a, cos a)
/// where `a` is the corner's angle about the center, so
/// σ = sign(τ_in·τ_out·sin(a_out − a_in)); a line leg's is
/// ±(cos a, sin a). Every row states the σ it expects, and the test
/// re-derives σ and both ρ signs from the built geometry, so a change to
/// the sign convention makes the table RED rather than vacuous.
///
/// **Both legs, always.** A ρ < 0 leg forces its partner to be an arc
/// that is also swallowed — see
/// [`an_enclosing_leg_forces_an_equally_enclosing_partner`], which pins
/// the three shapes that are ruled out. So the class the table spans is
/// σ = τ_in = τ_out = ±1 with r > max(R_in, R_out), and what varies
/// across rows is the turn side, equal vs unequal carriers, the turn
/// magnitude, and how close ρ sits to 0.
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

/// F1's enclosing (ρ < 0) class, **constructed** — the witness the fuzz
/// used to be asked to stumble onto.
///
/// Each row is checked by [`check_corner`], the same battery the sweep
/// runs on every accepted draw: recovered radius, both tangent points on
/// their carriers, the signed ρ = R − σ·τ·r predicting |P − O| (with the
/// internal-tangency claim |P − O| + R = r spelled out on the swallowed
/// legs), both tangent points at distance r from the fillet center, the
/// corner-side setback/extent bounds, and the atan2 bulge oracle.
#[test]
fn enclosing_tangency_is_constructed_not_stumbled_upon() {
    let cases = enclosing_cases();
    let (mut saw_ccw, mut saw_cw, mut saw_unequal, mut total) = (false, false, false, 0u64);
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
        let lp = build_corner(case.corner, case.leg_in, case.leg_out, case.r)
            .unwrap_or_else(|e| panic!("{name}: the enclosing fillet must construct, got {e:?}"));
        let c = check_corner(case.corner, case.leg_in, case.leg_out, case.r, &lp, &|| {
            name.to_string()
        });
        assert_eq!(
            c.enclosing, 2,
            "{name}: the oracle battery classified {} legs as enclosing, not both",
            c.enclosing
        );
        total += c.enclosing;
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
    assert!(
        total >= 8,
        "only {total} enclosing legs across {} rows — the table has been thinned",
        cases.len()
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
/// named for. The sugar refuses all three with `OffsetCarriersDisjoint`,
/// which is the same inequality read from the offset side.
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
            Err(ProfileError::NoCornerForFillet {
                reason: profile::NoCornerReason::OffsetCarriersDisjoint,
                ..
            }) => {}
            other => panic!(
                "{name}: a swallowed carrier next to this partner is geometrically \
                 impossible, so the sugar must refuse with OffsetCarriersDisjoint; got \
                 {other:?}"
            ),
        }
    }
}

/// F3 attack, now the MAJOR-1 regression pin: on the vesica's BOTTOM
/// corner with short legs, both carrier intersections exist, and the
/// first candidate in fixed order is the one rounding the OTHER (top)
/// corner. Before the fix the arc setback was reduced into [0, 2π), so
/// that candidate's tangent point read as a huge POSITIVE setback (the
/// long way round the circle), passed the corner-side test, and the
/// refusal rendered 8.15 m against a 0.14 m leg — numbers belonging to a
/// corner the author never named.
///
/// With a signed setback the top candidate classifies Negative on
/// `fillet_leg_reach` and is skipped, so the refusal describes the
/// bottom corner's own candidate: a setback of the same order as the
/// legs, not four circumferences' worth.
#[test]
fn overrun_attribution_names_the_authored_corners_candidate() {
    let s3 = 3.0f64.sqrt();
    let deg = |d: f64| d.to_radians();
    // corner (0, -s3) on both circles (centers (-1,0),(1,0), R=2);
    // both legs only 10 degrees long.
    let far_in = p2(-1.0 + 2.0 * deg(296.0).cos(), 2.0 * deg(296.0).sin());
    let far_out = p2(1.0 + 2.0 * deg(244.0).cos(), 2.0 * deg(244.0).sin());
    let err = ProfileLoop::builder(far_in)
        .fillet_corner(
            FilletLegShape::Arc {
                center: p2(-1.0, 0.0),
                sweep: ArcSweep::Ccw,
            },
            p2(0.0, -s3),
            FilletLegShape::Arc {
                center: p2(1.0, 0.0),
                sweep: ArcSweep::Ccw,
            },
            far_out,
            0.5,
            tol(),
        )
        .expect_err("short legs must refuse");
    match err {
        ProfileError::FilletDoesNotFit {
            leg,
            setback,
            leg_length,
            ..
        } => {
            // The bottom corner's OWN candidate: it overruns the 8°
            // (0.1396 m) leg, but only by a factor of ~1.6 — not by the
            // 4.4 m the top corner's wrap-around reading produced.
            assert_eq!(leg, profile::FilletLeg::Incoming);
            assert!(
                (leg_length - 0.139_626_340_159_546_53).abs() < 1e-15,
                "leg length {leg_length}"
            );
            assert!(
                setback > leg_length && setback < 1.0,
                "setback {setback}: expected the near candidate's own overrun, not a \
                 wrap-around distance to the corner the author never named"
            );
            // Half the carrier's circumference is the hard ceiling a
            // signed setback can never exceed (|Δθ| ≤ π, R = 2).
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
/// that difference; the tangent point is recovered by projecting the
/// fillet centre back over exactly that lever, and the corner's carriers
/// are 2.27 m apart, so the centre's last-place rounding arrives at the
/// carrier magnified past ε. `fillet_offset_lever` now says so:
/// |ρ| = 2.89e-4 against a least supported lever of 6.44e-3, short by a
/// factor of 22.
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
    // The gate's least lever is `scale * sqrt(C * R2 / band.zero())`, so
    // a looser ambient band affords MORE conditioning: this corner's
    // 2.29e-9 residual is uncertifiable against a 1e-9 band and entirely
    // fine against a 1e-6 one. Asserting the refusal unconditionally
    // would be exactly the eps-blindness this suite has been removing
    // elsewhere — and CI caught it doing so on the 1e-6 row.
    //
    // So: at a band this corner's conditioning cannot support, it must
    // REFUSE typed; at a band that can support it, it must BUILD and the
    // tangent point it returns must actually sit on its carrier within
    // that band. Both halves are claims; neither row is a skip.
    let eps = tol().eps;
    if 2.291e-9 < eps {
        let lp = build_corner(corner, leg_in, leg_out, r).unwrap_or_else(|e| {
            panic!(
                "at eps = {eps:e} this corner's conditioning IS supported, so it must \
                 build rather than refuse; got {e:?}"
            )
        });
        let nv = lp.vertices.len();
        let t2 = lp.vertices[nv - 1].pos;
        let res = leg_out.carrier_residual(corner, t2);
        assert!(
            res < eps,
            "at eps = {eps:e} the corner builds, so its tangent point must sit on the \
             carrier within the band it was certified against: off by {res:e}"
        );
        return;
    }
    match build_corner(corner, leg_in, leg_out, r) {
        Err(ProfileError::FilletOffsetLeverTooShort {
            leg,
            offset_radius,
            least_lever,
            margin,
            ..
        }) => {
            assert_eq!(leg, profile::FilletLeg::Outgoing);
            assert!(
                (offset_radius - rho).abs() < 1e-15,
                "the refusal reports offset lever {offset_radius}, not the corner's {rho}"
            );
            // Short by more than a decade: the gate is not deciding a
            // hairline here, it is refusing a corner that misses the
            // supported conditioning by 22x.
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
            "a tangent point this unconditioned must refuse typed rather than be returned \
             2.29e-9 off its carrier; got {other:?}"
        ),
    }
}

/// F1, enclosing case pinned deterministically (mined from the fuzz back
/// when the sweep was the only door to this class):
/// a fillet larger than BOTH arc legs' carriers (r > R_i, sigma*tau_i =
/// +1) constructs with the fillet circle enclosing both: |P - O_i| =
/// r - R_i, sign carried by the signed rho with no branch.
///
/// Kept alongside the constructed table because it is a real mined case
/// at coordinates nobody would author, and it holds its residuals to
/// 1e-11 rather than the battery's 1e-9. It also runs the full
/// [`check_corner`] battery, which the mined pin originally did not.
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
    let lp = build_corner(corner, leg_in, leg_out, r).expect("the enclosing fillet constructs");
    let c = check_corner(corner, leg_in, leg_out, r, &lp, &|| {
        "enclosing_fillet_swallows_both_leg_carriers".to_string()
    });
    assert_eq!(c.enclosing, 2, "both legs must classify as enclosing");
    let nv = lp.vertices.len();
    let (t1, t2, b) = (
        lp.vertices[nv - 2].pos,
        lp.vertices[nv - 1].pos,
        lp.vertices[nv - 2].bulge,
    );
    let (pf, rf) = circle_from_bulge(t1, t2, b);
    assert!((rf - r).abs() < 1e-11, "recovered radius {rf} vs {r}");
    for (o, rl) in [(o1, r1), (o2, r2)] {
        let d = (pf.x - o.x).hypot(pf.y - o.y);
        assert!(
            (d - (r - rl)).abs() < 1e-11,
            "|P-O| = {d}, expected r - R = {}",
            r - rl
        );
        // Internal tangency with the fillet enclosing the carrier.
        assert!(d + rl < r + 1e-11);
    }
}
