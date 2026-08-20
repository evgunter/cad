//! Reviewer consumer dry-runs for M2 PR 2 (2026-07-18), promoted from
//! the reviewer's session worktree into CI per the standing convention:
//! does a PR 4-shaped sweep consumer and a PR 3-shaped dihedral
//! consumer actually compose with ValidatedProfile + SketchPlane +
//! `geom`'s Circle conventions? Independent derivations — keep
//! verbatim (promotion adapted the header only).
//!
//! `geom` is a dev-dependency here (acyclic — profile does not depend
//! on `geom`). NOTE: the axis-from-turn convention these dry-runs
//! exercise (axis = +plane normal for a CCW turn, −normal for CW, so
//! increasing parameter runs start → end per the he_plus forward
//! contract) is **PR 4's convention to own and document** — this suite
//! demonstrates it composes, it does not ratify it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{profile, rounded_rect, tol};
use geom::Curve3;
use geom_core::{Point2, Point3, Vec3};
use profile::{LoopRole, Profile, SegmentKind, SketchPlane};

fn close3(p: Point3<f64>, q: Point3<f64>, tol: f64) -> bool {
    (p.x - q.x).abs() < tol && (p.y - q.y).abs() < tol && (p.z - q.z).abs() < tol
}

/// PR 4 dry-run: lift a rounded rectangle onto a tilted sketch plane,
/// validate, and for every Arc segment reconstruct the 3-D circle
/// carrier per `geom`'s curve conventions:
///
/// - center3 = plane.to_world(center2);
/// - axis = plane normal for turn = Positive, -normal for Negative
///   (so increasing theta always runs start(he_plus) -> end);
/// - u_ref = (to_world(start) - center3)/radius, putting t_start = 0 at
///   the start vertex (the seam is conventional data);
/// - the parameter span is theta_end = 4*atan(|bulge|), derived from
///   the stored bulge (equivalently from endpoints via turn-signed
///   atan2 reduction; the chord >= K*eps rule keeps the span bounded
///   away from 2*pi so the seam is never ambiguous).
///
/// Verifies: eval(0) = start vertex, eval(theta_end) = end vertex,
/// eval(theta_end/2) = the 2-D apex mapped to world, and the tangent at
/// 0 agrees with the he_plus-forward chord side.
#[test]
fn pr4_dry_run_rounded_rect_arc_frames_on_a_tilted_plane() {
    // A non-trivial rigid placement: orthonormal right-handed frame.
    let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
    let u = Vec3::new(inv_sqrt2, inv_sqrt2, 0.0);
    let v = Vec3::new(-inv_sqrt2 / 3.0, inv_sqrt2 / 3.0, (8.0f64 / 9.0).sqrt());
    let origin = Point3::new(10.0, -5.0, 2.0);
    let plane = SketchPlane::from_frame(origin, u, v);

    let base = rounded_rect(4.0, 3.0, 0.5);
    let p = Profile::new(plane, vec![base]);
    let vp = p.validate(tol()).expect("rounded rect validates");
    assert_eq!(vp.loops()[0].role(), LoopRole::Outer);

    let normal = u.cross(v);
    let mut arcs_seen = 0;
    for seg in vp.loops()[0].segments() {
        let SegmentKind::Arc {
            center,
            radius,
            turn,
        } = seg.kind
        else {
            continue;
        };
        arcs_seen += 1;
        let center3 = vp.plane().to_world(center);
        let start3 = vp.plane().to_world(seg.start);
        let end3 = vp.plane().to_world(seg.end);
        let axis = match turn {
            geom_core::Sign::Positive => normal,
            geom_core::Sign::Negative => normal * -1.0,
            geom_core::Sign::Zero => panic!("Arc turn is never Zero"),
        };
        let u_ref = (start3 - center3) / radius;
        let circle = Curve3::Circle {
            center: center3,
            axis,
            radius,
            u_ref,
        };
        // t_start = 0 at the start vertex.
        assert!(close3(circle.eval(0.0), start3, 1e-12));
        // Span from the stored bulge: theta = 4*atan(|b|).
        let theta_end = 4.0 * seg.bulge.abs().atan();
        assert!(close3(circle.eval(theta_end), end3, 1e-12));
        // Midpoint = the 2-D apex through the same placement.
        // (Apex from the sagitta identity: apex = mid - n_left*(L*b/2).)
        let chord = seg.end - seg.start;
        let len = seg.start.distance(seg.end);
        let unit = chord / len;
        let n_left = geom_core::Vec2::new(-unit.y, unit.x);
        let mid2 = seg.start.lerp(seg.end, 0.5);
        let apex2 = mid2 - n_left * (len * seg.bulge / 2.0);
        let apex3 = vp.plane().to_world(Point2::new(apex2.x, apex2.y));
        assert!(close3(circle.eval(theta_end / 2.0), apex3, 1e-12));
        // he_plus-forward: the tangent at 0 points into the segment
        // (positive dot with start->end chord).
        let tan0 = circle.deriv(0.0);
        let chord3 = end3 - start3;
        assert!(tan0.dot(chord3) > 0.0, "tangent must run start -> end");
    }
    assert_eq!(arcs_seen, 4, "all four corner arcs reconstructed");
}

/// The same composition for a HOLE loop (clockwise canonical traversal,
/// negative bulges): the axis flip keeps t increasing start -> end.
#[test]
fn pr4_dry_run_hole_arcs_flip_axis() {
    let plane = SketchPlane::xy();
    let p = Profile::new(
        plane,
        vec![
            common::rect(-3.0, -3.0, 6.0, 6.0),
            common::circle_h(0.0, 0.0, 1.0),
        ],
    );
    let vp = p.validate(tol()).expect("plate with hole validates");
    let hole = &vp.loops()[1];
    assert_eq!(hole.role(), LoopRole::Hole);
    for seg in hole.segments() {
        let SegmentKind::Arc {
            center,
            radius,
            turn,
        } = seg.kind
        else {
            panic!("hole is all arcs");
        };
        assert_eq!(turn, geom_core::Sign::Negative, "hole canonicalized CW");
        assert!(seg.bulge < 0.0);
        let center3 = vp.plane().to_world(center);
        let start3 = vp.plane().to_world(seg.start);
        let end3 = vp.plane().to_world(seg.end);
        let axis = Vec3::new(0.0, 0.0, -1.0); // -normal for CW turn
        let u_ref = (start3 - center3) / radius;
        let circle = Curve3::Circle {
            center: center3,
            axis,
            radius,
            u_ref,
        };
        let theta_end = 4.0 * seg.bulge.abs().atan();
        assert!(close3(circle.eval(0.0), start3, 1e-12));
        assert!(close3(circle.eval(theta_end), end3, 1e-12));
    }
}

/// PR 3 dry-run: the joins a validated profile hands the dihedral
/// predicate. For the rounded rectangle every line->arc join is an
/// EXACT carrier tangency (tangent directions agree to rounding); for
/// the L-profile every join is a definite corner (tangents differ by
/// at least 90 deg). Nothing in between survives validation (the near-tangent
/// case escalates — see review_attacks::near_tangent_join_escalates).
#[test]
fn pr3_dry_run_joins_are_definitely_smooth_or_definitely_corner() {
    // Tangent direction of segment k at its START vertex, in 2-D.
    let tangent_at_start = |seg: &profile::ValidatedSegment<f64>| -> geom_core::Vec2<f64> {
        let chord = seg.end - seg.start;
        let len = seg.start.distance(seg.end);
        let unit = chord / len;
        match seg.kind {
            SegmentKind::Line => unit,
            SegmentKind::Arc { .. } => {
                // Tangent-chord angle = theta/2 = 2*atan(b): rotate the
                // chord by -theta/2... for CCW (b>0) the tangent at the
                // start is the chord rotated by +? Derive: inscribed
                // angle: tangent at A makes angle theta/2 with chord,
                // rotated toward the arc side. Signed: rotate unit by
                // -2*atan(b)? Check with quarter arc (0,0)->(1,1),
                // b=tan(pi/8): tangent at start is +x: chord at 45 deg,
                // rotated by -45 deg = -theta/2. So: rotate by
                // -2*atan(b).
                let half_theta = 2.0 * seg.bulge.atan();
                let (s, c) = (-half_theta).sin_cos();
                geom_core::Vec2::new(c * unit.x - s * unit.y, s * unit.x + c * unit.y)
            }
        }
    };
    let tangent_at_end = |seg: &profile::ValidatedSegment<f64>| -> geom_core::Vec2<f64> {
        let chord = seg.end - seg.start;
        let len = seg.start.distance(seg.end);
        let unit = chord / len;
        match seg.kind {
            SegmentKind::Line => unit,
            SegmentKind::Arc { .. } => {
                let half_theta = 2.0 * seg.bulge.atan();
                let (s, c) = half_theta.sin_cos();
                geom_core::Vec2::new(c * unit.x - s * unit.y, s * unit.x + c * unit.y)
            }
        }
    };

    // Rounded rect: every join smooth (angle < 1e-12 rad).
    let vp = profile(vec![rounded_rect(4.0, 3.0, 0.5)])
        .validate(tol())
        .expect("validates");
    let segs = vp.loops()[0].segments();
    let n = segs.len();
    for k in 0..n {
        let t_out = tangent_at_end(&segs[k]);
        let t_in = tangent_at_start(&segs[(k + 1) % n]);
        let angle = t_out.perp_dot(t_in).atan2(t_out.dot(t_in));
        assert!(
            angle.abs() < 1e-12,
            "rounded-rect join {k} not smooth: {angle:e} rad"
        );
    }

    // L-profile: every join a definite corner (|angle| = pi/2).
    let vp = profile(vec![common::l_profile()])
        .validate(tol())
        .expect("validates");
    let segs = vp.loops()[0].segments();
    let n = segs.len();
    for k in 0..n {
        let t_out = tangent_at_end(&segs[k]);
        let t_in = tangent_at_start(&segs[(k + 1) % n]);
        let angle = t_out.perp_dot(t_in).atan2(t_out.dot(t_in));
        assert!(
            (angle.abs() - std::f64::consts::FRAC_PI_2).abs() < 1e-12,
            "L-profile join {k}: {angle} rad"
        );
    }
}
