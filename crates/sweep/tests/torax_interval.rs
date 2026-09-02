//! **VERBS-TORAX, interval lane**: the axial door's TORUS arm
//! instantiated at the certified scalar.
//!
//! `sf2b_interval_probe` carries the axial door's other four fixtures
//! and states the law this file exists to keep: *every new decide site
//! the unit added executes at the scalar that can escalate rather than
//! at the one that cannot*. No fixture in that file hollows a torus, so
//! none of them instantiates this unit's arm — and its rows are inside
//! the spec's bit-identity block, so the arm's interval coverage is
//! added HERE rather than by moving them.
//!
//! **What this row reaches, and what it cannot.** The torus arm's two
//! new `decide` sites are `offset_axial_seam_meridian` and
//! `offset_axial_pole_centre`:
//!
//! - `offset_axial_seam_meridian` is on the only path a full revolve's
//!   torus wall has. `mint_carrier` has no other arm for a torus chart's
//!   single seam — without it the barrel refuses `TogetherAxialEdge`
//!   ("a seam whose carrier and chart are not one of the closed-form
//!   pairs"), which is how the arm was found to be needed at all — so a
//!   hollow that SUCCEEDS is a proof the site executed. This row is
//!   that hollow, at `T = Interval`.
//! - `offset_axial_pole_centre` is **not instantiable from a torus at
//!   any scalar**, and that is geometry rather than a fixture gap: it
//!   fires only where a single profile constraint at an axis pole is a
//!   CIRCLE, and a ring torus's meridian stands `R − r > 0` clear of the
//!   axis, so no vertex is ever both on a torus face and at `ρ = 0`.
//!   Reaching it needs a SPHERE whose own pole is a vertex, which is a
//!   sphere-side fixture and not this unit's to plant. The module docs
//!   in `offset_axial` list it among the arms written for correctness
//!   and unreached by any fixture; this file is where that claim is
//!   restated at the scalar it matters for.
//!
//! The comparison is the corner's own closed form as an ENCLOSURE
//! claim, not a volume: a torus wall's volume of revolution is not a
//! shorter statement than the corner it is here to check.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Bounds, Interval, Point2, Real, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(iv(x), iv(y))
}

const FIT_TOL: f64 = 1e-6;

/// The tour's own wall thickness, the one `torax_axial` hollows by.
const T: f64 = 1.0 / 128.0;

fn revolved(lp: ProfileLoop<Interval>, turn: Revolution<Interval>) -> Body<Interval> {
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(iv(0.0), iv(1.0)),
        },
        turn,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// **The torus barrel at `T = Interval`** — `torax_axial`'s own first
/// fixture, built from the same dyadic numbers.
///
/// `R = 6/64`, `r = 5/64`, `h_c = 4/64`: a 3-4-5 at each junction, so
/// the operand's own residuals are exact and the only width in the
/// answer is the door's.
///
/// The wall moves the minor radius OUT to `r + T = 11/128` about the
/// UNMOVED tube centre `(12/128, 8/128)`. Each moved cap station stands
/// `7/128` from `h_c`, the circle's half-chord there is
/// `√(11² − 7²)/128 = 6√2/128`, and the root a small offset keeps is
/// the inner one:
///
/// ```text
/// ρ = (12 − 6√2)/128 = 3(2 − √2)/64     h ∈ {1/128, 15/128}
/// ```
///
/// Both stations are checked as enclosures that CONTAIN that number and
/// stay tight — containment alone gets easier as an enclosure degrades,
/// so a width bound goes with it, the `sf2b_interval_probe` rule.
/// MEASURED at this head: `ρ ∈ [0.02745873926376053,
/// 0.027458739263761793]`, width `1.26e-15` m, at both junctions; the
/// stations `h = 1/128` and `h = 15/128` come back with width EXACTLY
/// zero, which is what a dyadic cap offset should give. The bound is
/// `1e-9`, six decades of headroom, so it reds on a real widening and
/// not on a last-place wobble.
#[test]
fn interval_the_torus_barrel_hollows_and_encloses_its_corners() {
    let tol = Tol::witness();
    let c = p2(6.0 / 64.0, 1.0 / 16.0);
    let (lo, hi) = (p2(3.0 / 64.0, 0.0), p2(3.0 / 64.0, 8.0 / 64.0));
    let (u, v) = (lo - c, hi - c);
    let bulge = (u.perp_dot(v).atan2(u.dot(v)) / iv(4.0)).tan();
    let body = revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), iv(0.0)),
            ProfileVertex::new(lo, bulge),
            ProfileVertex::new(hi, iv(0.0)),
            ProfileVertex::new(p2(0.0, 8.0 / 64.0), iv(0.0)),
        ]),
        Revolution::Full,
    );
    assert!(
        body.surfaces()
            .any(|(_, s)| matches!(s, Surface::Torus { .. })),
        "the barrel's wall is a torus at this scalar too"
    );

    let hollow = topo::shell(&body, iv(T), FIT_TOL, tol)
        .expect("the torus barrel hollows at the certified scalar");
    assert_eq!(
        topo::validate_geometric(&hollow, tol),
        Ok(()),
        "the torus barrel's interval hollow: tier 3"
    );
    assert_eq!(hollow.shells().count(), 2, "outer + cavity");

    let rho = 3.0 * (2.0 - 2.0f64.sqrt()) / 64.0;
    encloses_corner(&hollow, rho, 1.0 / 128.0, "the barrel's base junction");
    encloses_corner(&hollow, rho, 15.0 / 128.0, "the barrel's mouth junction");
}

/// Some vertex of `body` encloses `(ρ, h)`, and its enclosure is tight.
///
/// The width bound is `1e-9` m ABSOLUTE on a body whose radii are
/// ~5e-2 m: the same shape of claim `sf2b_interval_probe::contains`
/// makes about a volume, written against a length here because a
/// station of `1/128` is a legitimate answer and a relative bound on it
/// would be a bound on a different quantity at each corner.
fn encloses_corner(body: &Body<Interval>, rho: f64, h: f64, what: &str) {
    let mut best: Option<(f64, f64, f64, f64, f64)> = None;
    for (_, vtx) in body.vertices() {
        let p = *body.get_point(vtx.point).expect("a vertex carries a point");
        let r = (p.x * p.x + p.z * p.z).sqrt();
        let (rlo, rhi, hlo, hhi) = (r.lo(), r.hi(), p.y.lo(), p.y.hi());
        if rlo <= rho && rho <= rhi && hlo <= h && h <= hhi {
            println!(
                "[torax-interval] {what}: ρ ∈ [{rlo}, {rhi}] (width {}), h ∈ [{hlo}, {hhi}] \
                 (width {})",
                rhi - rlo,
                hhi - hlo
            );
            assert!(
                rhi - rlo <= 1e-9 && hhi - hlo <= 1e-9,
                "{what}: the enclosure must stay TIGHT — ρ width {}, h width {}",
                rhi - rlo,
                hhi - hlo
            );
            return;
        }
        let miss = (rlo - rho).abs().min((rhi - rho).abs()) + (hlo - h).abs().min((hhi - h).abs());
        if best.is_none_or(|b| miss < b.0) {
            best = Some((miss, rlo, rhi, hlo, hhi));
        }
    }
    panic!(
        "{what}: no vertex encloses the closed form (ρ, h) = ({rho}, {h}); the nearest \
         was {best:?}"
    );
}
