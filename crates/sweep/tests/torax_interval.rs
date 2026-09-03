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
//!
//! **VERBS-RIMCAP's sites live here too**, under the same law: the
//! sphere lune row executes the meridian-pair arm and the off-axis rim
//! mint at the certified scalar, and the klein elbow row executes the
//! carried-datum arm on its way to the carrier mint's refusal — each
//! new `decide` site of that unit reached where escalation is real.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::tolerance::DEFAULT_EPS;
use geom_core::{Bounds, Interval, MarginDiag, Point2, Real, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use test_utils::vacuity::stood_down;
use topo::{Body, ShellError, ValidationError};

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
///
/// **Per band, honestly** (the issue-1356 practice). At ε = 1e-12 the
/// hollow does not build: its tier-3 pass cannot decide ONE sliver
/// dihedral — the margin's enclosure is `[0, ~1.2e-12]` against a
/// `1e-12` band — and the certified scalar escalates rather than
/// answers, which is exactly the law this file's header states. So the
/// row takes the band it is run at: where the hollow builds, the
/// corners are checked as enclosures; where it escalates, the
/// escalation's exact shape is pinned (one error, that class, that
/// predicate, the band the run committed to, a margin inside the
/// escalation band — a REAL widening clears `escalate` and reds here)
/// and the corner claims are stood down BY NAME. MEASURED at this
/// head: green at ε ∈ {1e-6, 1e-9}; at 1e-12 `EdgeKey(17v1)`,
/// `tangent_second_order`, enclosure `[0, 1.1625e-12]`.
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

    let hollow = match topo::shell(&body, iv(T), FIT_TOL, tol) {
        Ok(hollow) => hollow,
        Err(ShellError::NotValid { errors }) if tol.eps() < DEFAULT_EPS => {
            let [ValidationError::SliverDihedral { edge, cause }] = errors.as_slice() else {
                panic!(
                    "at ε = {:e} the hollow refused with something other than ONE sliver \
                     dihedral: {errors:?}",
                    tol.eps()
                );
            };
            assert_eq!(
                cause.predicate,
                Some("tangent_second_order"),
                "the escalation at {edge:?} is not the dihedral classifier's own"
            );
            assert_eq!(
                cause.band.zero(),
                tol.eps(),
                "the band is the run's own ε, not a number the fixture carries"
            );
            let MarginDiag::Enclosure { lo, hi } = cause.margin else {
                panic!(
                    "the sliver margin at {edge:?} is not an enclosure: {:?}",
                    cause.margin
                );
            };
            assert!(
                lo <= cause.band.zero() && hi >= cause.band.zero() && hi < cause.band.escalate(),
                "the margin [{lo:e}, {hi:e}] at {edge:?} is not an in-band escalation against \
                 zero {:e} / escalate {:e} — a real widening would clear the escalation \
                 threshold, and this row must red on that",
                cause.band.zero(),
                cause.band.escalate()
            );
            stood_down(
                &format!("the torus barrel's interval hollow, eps = {:e}", tol.eps()),
                "the hollow escalated on one sliver dihedral before any corner was \
                 reachable, so THIS RUN ASSERTS NEITHER the two-shell count NOR the corner \
                 enclosures — only that the escalation is the certified scalar's honest \
                 one, at the run's own band, on a margin inside the escalation band",
            );
            return;
        }
        Err(e) => panic!("the torus barrel hollows at the certified scalar: {e:?}"),
    };
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

/// **The sphere lune's rim at `T = Interval`** — VERBS-RIMCAP's new
/// decide sites, executed at the scalar that can escalate (this file's
/// own law). The direct door is the row's subject for the same reason
/// it is at f64: `shell`'s closing tier 3 needs a volume the sphere
/// flux arm's `props_band_coplanar` premise cannot yet give a lune —
/// the operand's own standing wall — so the door is asked directly and
/// the corner enclosures are the claim.
///
/// One hollow executes the meridian-pair arm (`offset_axial_cap_pair`,
/// `offset_axial_cap_line`, and the shared corner/concurrence/residual
/// meters at its new call sites) and the off-axis rim mint
/// (`offset_axial_rim_concentric` / `_great` / `_plane` / `_reach`).
/// The carried-datum site (`offset_axial_datum_arm`) is the klein
/// elbow's and is executed by the refusal row below. Corners checked
/// as ENCLOSURES that contain the closed form and stay tight:
///
/// ```text
/// ρ = t·√2          h = ±√((r − t)² − 2t²)      (r, t) = (0.3, 0.05)
/// ```
///
/// Per band, honestly: where the certified scalar escalates instead of
/// answering, the escalation is pinned as the door's own typed
/// `Escalated` and the corner claims are stood down by name.
#[test]
fn interval_the_sphere_lune_rim_encloses_its_corners() {
    let tol = Tol::witness();
    let profile = Profile::new(
        SketchPlane::<Interval>::xy(),
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -0.3), iv(0.0)),
            ProfileVertex::new(p2(0.0, 0.3), iv(-1.0)),
        ])],
    )
    .validate(tol)
    .expect("the lune's cross-section validates");
    let body = revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(iv(0.0), iv(1.0)),
        },
        Revolution::Partial(iv(core::f64::consts::FRAC_PI_2)),
        tol,
    )
    .expect("the lune revolves")
    .body;

    let mut charts: Vec<(topo::SurfaceKey, Vec<topo::FaceKey>)> = Vec::new();
    for (k, f) in body.faces() {
        match charts.iter_mut().find(|(s, _)| *s == f.surface) {
            Some((_, v)) => v.push(k),
            None => charts.push((f.surface, vec![k])),
        }
    }
    let moves: Vec<topo::ChartMove<Interval>> = charts
        .into_iter()
        .map(|(_, faces)| {
            let sense = body.get_face(faces[0]).expect("face").sense;
            topo::ChartMove {
                faces,
                distance: if sense { iv(-0.05) } else { iv(0.05) },
            }
        })
        .collect();
    let mut cavity = body.clone();
    let band = geom_core::Band::linear(tol).expect("band");
    match topo::offset_charts_together(&mut cavity, &moves, band, tol) {
        Ok(()) => {}
        Err(topo::ReplaceFaceError::Escalated { source }) if tol.eps() < DEFAULT_EPS => {
            stood_down(
                &format!("the sphere lune's interval rim, eps = {:e}", tol.eps()),
                &format!(
                    "the certified scalar escalated ({source:?}) before the corners were \
                     reachable, so THIS RUN ASSERTS NO corner enclosure — only that the \
                     escalation is the door's own typed one"
                ),
            );
            return;
        }
        Err(e) => panic!("the lune's rim solves at the certified scalar: {e:?}"),
    }
    let rho = 0.05 * 2.0f64.sqrt();
    let h = (0.25f64.powi(2) - 2.0 * 0.05 * 0.05).sqrt();
    encloses_corner(&cavity, rho, h, "the lune's upper rim corner");
    encloses_corner(&cavity, rho, -h, "the lune's lower rim corner");
}

/// **The klein elbow at `T = Interval`**: the carried-datum arm
/// (`offset_axial_datum_arm` and the shared azimuth solve at its new
/// call site) executes at the certified scalar on the way to the same
/// refusal f64 measures — the latitude mint's off-axis-centre payload,
/// the torus half's own boundary. An escalation at a strict band is
/// the certified scalar's honest answer and is pinned as such.
#[test]
fn interval_the_klein_elbow_rim_refuses_at_the_carrier_mint() {
    let tol = Tol::witness();
    let r = 0.275_f64;
    let profile = Profile::new(
        SketchPlane::<Interval>::xy(),
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(-r, 0.0), iv(1.0)),
            ProfileVertex::new(p2(r, 0.0), iv(1.0)),
        ])],
    )
    .validate(tol)
    .expect("the elbow's cross-section validates");
    let body = revolve(
        &profile,
        RevolveAxis {
            origin: p2(1.2, 0.0),
            dir: Vec2::new(iv(0.0), iv(-1.0)),
        },
        Revolution::Partial(iv(-core::f64::consts::FRAC_PI_2)),
        tol,
    )
    .expect("the elbow revolves")
    .body;
    let e = topo::shell(&body, iv(0.05), FIT_TOL, tol)
        .expect_err("the elbow's moved rim is a spiric section away from a carrier");
    match e {
        ShellError::Face { ref error, .. }
            if matches!(
                **error,
                topo::ReplaceFaceError::TogetherAxialEdge { what, .. }
                    if what == "a circular edge between two charts whose centre is off the axis"
            ) => {}
        ShellError::Face { ref error, .. }
            if tol.eps() < DEFAULT_EPS
                && matches!(**error, topo::ReplaceFaceError::Escalated { .. }) =>
        {
            stood_down(
                &format!("the klein elbow's interval rim, eps = {:e}", tol.eps()),
                "the certified scalar escalated before the carrier mint's refusal was \
                 reachable, so THIS RUN ASSERTS ONLY the door's own typed escalation",
            );
        }
        other => panic!("expected the latitude mint's off-axis refusal, got {other:?}"),
    }
}
