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
//! - the fuzz keeps the review's 20k corners; it runs in well under a
//!   second at `f64`, so no iteration trim was needed.
//! - F3's gate-sequence invariance probe records at `Probe`, so it lives
//!   in `review_s2_probe.rs` behind the `probe` feature; the rest of the
//!   suite is f64 and runs in the default build.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::tol;
use geom_core::Point2;
use profile::{ArcSweep, FilletLegShape, ProfileError, ProfileLoop};

const TAU: f64 = core::f64::consts::TAU;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// xorshift64* — deterministic fuzz.
struct Rng(u64);
impl Rng {
    fn next_f64(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next_f64()
    }
    fn flip(&mut self) -> bool {
        self.next_f64() < 0.5
    }
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

fn rand_leg(rng: &mut Rng, corner: Point2<f64>, incoming: bool, enclosing_bias: bool) -> OracleLeg {
    if !enclosing_bias && rng.flip() {
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
        let tau = if rng.flip() { 1.0 } else { -1.0 };
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

#[test]
fn fuzz_offset_carrier_construction_tangency_and_bulge() {
    let mut rng = Rng(0x5EED_CAFE_F00D_0001);
    let (mut n_ok, mut n_arc_leg, mut n_arc_arc, mut n_enclosing, mut n_major) = (0, 0, 0, 0, 0);
    for i in 0..20_000 {
        let enclosing_bias = i % 5 == 0;
        let corner = p2(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
        let leg_in = rand_leg(&mut rng, corner, true, enclosing_bias);
        let leg_out = rand_leg(&mut rng, corner, false, enclosing_bias);
        let r = if enclosing_bias {
            rng.range(0.06, 0.9)
        } else {
            rng.range(0.02, 0.8)
        };
        let head = leg_in.far_point(corner);
        let next = leg_out.far_point(corner);
        let built = ProfileLoop::builder(head).fillet_corner(
            leg_in.shape(),
            corner,
            leg_out.shape(),
            next,
            r,
            tol(),
        );
        let Ok(chain) = built else { continue };
        let lp = chain.close();
        n_ok += 1;
        let nv = lp.vertices.len();
        assert!(nv == 2 || nv == 3, "chain shape: {nv} vertices");
        let t2 = lp.vertices[nv - 1].pos;
        let t1 = lp.vertices[nv - 2].pos;
        let b = lp.vertices[nv - 2].bulge;
        assert!(
            b.is_finite() && b != 0.0,
            "degenerate fillet bulge {b} at iter {i}"
        );

        // sigma re-derived from travel directions.
        let (dix, diy) = leg_in.travel_dir(corner, true);
        let (dox, doy) = leg_out.travel_dir(corner, false);
        let sigma = (dix * doy - diy * dox).signum();

        // (a) fillet circle from the emitted data.
        let (pf, rf) = circle_from_bulge(t1, t2, b);
        assert!(
            (rf - r).abs() < 1e-9,
            "iter {i}: recovered radius {rf} vs {r}"
        );

        // (b) tangent points on their carriers.
        assert!(
            leg_in.carrier_residual(corner, t1) < 1e-9,
            "iter {i}: t1 off carrier"
        );
        assert!(
            leg_out.carrier_residual(corner, t2) < 1e-9,
            "iter {i}: t2 off carrier"
        );

        // (c) fillet circle tangent to both carriers, with the SIGNED
        // rho = R - sigma*tau*r predicting the center distance (F1).
        for (leg, inc) in [(leg_in, true), (leg_out, false)] {
            match leg {
                OracleLeg::Line { .. } => {
                    let d = leg.carrier_residual(corner, pf);
                    assert!(
                        (d - r).abs() < 1e-9,
                        "iter {i}: line-carrier clearance {d} vs r {r}"
                    );
                }
                OracleLeg::Arc { .. } => {
                    n_arc_leg += 1;
                    let res = leg.center_distance_residual(pf, sigma, r);
                    assert!(res < 1e-9, "iter {i}: |P-O| vs |rho| residual {res}");
                    if leg.is_enclosing(sigma, r) {
                        // ρ < 0: the fillet swallows this leg's carrier.
                        // One case mined from here is pinned exactly by
                        // `enclosing_fillet_swallows_both_leg_carriers`.
                        n_enclosing += 1;
                    }
                }
            }
            // (d) tangent points at distance r from the fillet center.
            let t = if inc { t1 } else { t2 };
            let dt = ((t.x - pf.x).hypot(t.y - pf.y) - r).abs();
            assert!(dt < 1e-9, "iter {i}: |t-P| residual {dt}");
            // (e) corner-side extents.
            let (sb, ext) = leg.setback_extent(corner, t, inc);
            assert!(
                sb > -1e-9 && sb < ext + 1e-9,
                "iter {i}: setback {sb} outside [0, {ext}]"
            );
        }
        if matches!(leg_in, OracleLeg::Arc { .. }) && matches!(leg_out, OracleLeg::Arc { .. }) {
            n_arc_arc += 1;
        }

        // (f) F2: atan2 sweep oracle for the bulge (major arcs included).
        let ang = |p: Point2<f64>| (p.y - pf.y).atan2(p.x - pf.x);
        let theta = ((ang(t2) - ang(t1)) * sigma).rem_euclid(TAU);
        let b_ref = sigma * (theta / 4.0).tan();
        assert!(
            (b - b_ref).abs() <= 1e-9 * b_ref.abs().max(1.0),
            "iter {i}: bulge {b} vs atan2 oracle {b_ref} (theta {theta})"
        );
        if b.abs() > 1.0 {
            n_major += 1;
        }
    }
    // Coverage floor: the fuzz must actually exercise the claims.
    assert!(n_ok > 500, "only {n_ok} accepted corners");
    assert!(n_arc_arc > 50, "only {n_arc_arc} arc-by-arc corners");
    assert!(
        n_enclosing >= 10,
        "only {n_enclosing} enclosing (rho < 0) tangencies"
    );
    // `n_major` is a COVERAGE REPORT, not a gate: it comes out 0, which
    // is the fuzz corroborating the bound `fillet_bulge`'s docs argue
    // for — the corner-side extent gates keep every fillet arc below
    // half a turn, so the negative-apothem branch is unreachable through
    // this door and is unit-tested directly instead. Deliberately not
    // asserted either way: a future change that legitimately admits major
    // arcs should not fail here, it should make the branch live.
    eprintln!(
        "fuzz: ok {n_ok}, arc legs {n_arc_leg}, arc-by-arc {n_arc_arc}, enclosing {n_enclosing}, major {n_major}"
    );
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

/// F1, enclosing case pinned deterministically (mined from the fuzz):
/// a fillet larger than BOTH arc legs' carriers (r > R_i, sigma*tau_i =
/// +1) constructs with the fillet circle enclosing both: |P - O_i| =
/// r - R_i, sign carried by the signed rho with no branch.
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
    let head = p2(o1.x + r1 * fa1.cos(), o1.y + r1 * fa1.sin());
    let next = p2(o2.x + r2 * fa2.cos(), o2.y + r2 * fa2.sin());
    let lp = ProfileLoop::builder(head)
        .fillet_corner(
            FilletLegShape::Arc {
                center: o1,
                sweep: ArcSweep::Ccw,
            },
            corner,
            FilletLegShape::Arc {
                center: o2,
                sweep: ArcSweep::Ccw,
            },
            next,
            r,
            tol(),
        )
        .expect("the enclosing fillet constructs")
        .close();
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
