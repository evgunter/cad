//! **A curved body against an identical copy is that body.** Nothing
//! crosses and each shell lies on the other, so the boolean takes the
//! no-crossings path: the sphere extent scan and the section pass hold
//! each refusing pair the reduction settled one carrier for the shells'
//! `On` verdict, and read the sweep's touches as their events. A cone is
//! read by the carrier ladder, so the glue door settles a cone face with
//! its copy rather than asking the tangency lane about it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::revert_ops::subtract_both_orders_and_intersect;
use core::f64::consts::{FRAC_1_SQRT_2, PI};
use geom_core::{Affine3, Tol, Vec3};
use sweep::test_support::{
    ball_poled_z, corners, dome, finished, lantern, revolved_about_y, waisted,
};
use topo::{Body, BooleanError, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

/// The bodies that build against a copy, each with its closed-form
/// volume.
fn bodies() -> [(&'static str, Body<f64>, f64); 4] {
    let a = FRAC_1_SQRT_2;
    [
        (
            "a ball",
            ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
            4.0 * PI / 3.0,
        ),
        // The unit sphere's zone over `[0, 1/√2]` less the bore of
        // radius ½.
        (
            "a dome",
            dome(1.0, tol()),
            PI * (0.75 * a - a.powi(3) / 3.0),
        ),
        // The unit sphere's zone over `[0, 0.6]`, then the frustum from
        // radius 0.8 to 0.2 over 0.6.
        (
            "a lantern",
            lantern(tol()),
            PI * (0.6 - 0.6f64.powi(3) / 3.0) + PI * 0.6 / 3.0 * (0.64 + 0.16 + 0.04),
        ),
        (
            "a frustum",
            revolved_about_y(
                corners(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]),
                sweep::Revolution::Full,
                tol(),
            ),
            7.0 * PI / 12.0,
        ),
    ]
}

fn shifted(body: &Body<f64>, by: Vec3<f64>) -> Body<f64> {
    topo::transform_rigid(
        body,
        &geom_core::test_support::rigid(&Affine3::translation(by), tol()),
        tol(),
    )
    .unwrap()
}

/// Every op on `(a, b)`, `∪` first.
fn every_op(
    a: &Body<f64>,
    b: &Body<f64>,
) -> Vec<(&'static str, Result<BooleanResult<f64>, BooleanError>)> {
    let (fa, fb) = (
        finished("A", a.clone(), tol()),
        finished("B", b.clone(), tol()),
    );
    let mut got = vec![("A ∪ B", topo::union(&fa, &fb, tol()))];
    got.extend(subtract_both_orders_and_intersect(a, b));
    got
}

/// `r` is one sound body of volume `want`: tier 3 and 3′, one shell.
fn one_body(label: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) {
    let Ok(BooleanResult::Body(bb)) = r else {
        panic!("{label}: one body: {r:?}");
    };
    assert_eq!(
        topo::validate_geometric(&bb.body, tol()),
        Ok(()),
        "{label}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
        Ok(()),
        "{label}: tier 3′"
    );
    assert_eq!(bb.body.shells().count(), 1, "{label}: one shell");
    let m = topo::test_support::pinned_volume(label, &bb.body, tol());
    assert!(
        (m.volume - want).abs() <= m.volume_pad + 1e-9 * want,
        "{label}: volume {} ± {} against the closed form {want}",
        m.volume,
        m.volume_pad
    );
}

/// `body` against an identical copy: `∪` and `∩` are the body at its
/// closed-form volume `want`, through tier 3 and 3′; each difference is
/// nothing.
fn against_its_copy(label: &str, body: &Body<f64>, want: f64) {
    for (op, r) in every_op(body, body) {
        let label = format!("{label} against its copy, {op}");
        if op.contains('∖') {
            assert!(
                matches!(r, Ok(BooleanResult::Empty)),
                "{label}: one body less itself is nothing: {r:?}"
            );
        } else {
            one_body(&label, r, want);
        }
    }
}

/// **A ball against its copy is the ball**: every sphere pair is settled
/// one carrier, and the extent scan holds each for the `On` verdict.
#[test]
fn a_ball_against_its_copy_is_one_body() {
    let [(label, body, want), ..] = bodies();
    against_its_copy(label, &body, want);
}

/// **A cone frustum against its copy is the frustum**: the ladder reads
/// the cone face one carrier with its copy.
#[test]
fn a_frustum_against_its_copy_is_one_body() {
    let [.., (label, body, want)] = bodies();
    against_its_copy(label, &body, want);
}

/// **A dome and a lantern against their copies**: a sphere zone meets a
/// plane face of the copy along their shared rim, which the section
/// certificate clears by the sweep's touch there (W4); the lantern's
/// cone and sphere are settled one carrier each.
#[test]
fn a_dome_and_a_lantern_against_their_copies_are_one_body() {
    let [_, dome, lantern, _] = bodies();
    for (label, body, want) in [dome, lantern] {
        against_its_copy(label, &body, want);
    }
}

/// **The waisted body against its copy refuses at the line × cone
/// lane.** Its cones are settled one carrier with their copies, so the
/// sweep reads each edge against each face: a ruling of the upper cone
/// runs parallel to a generator of the lower cone's other nappe, and
/// meets it only at the waist. The quadratic degenerates there, which
/// the lane does not answer
/// (`work/roots/a-line-parallel-to-a-cone-generator-has-no-root-lane.md`).
#[test]
fn the_waisted_body_against_its_copy_refuses_at_the_line_cone_lane() {
    let body = waisted(tol());
    for (op, r) in every_op(&body, &body) {
        let label = format!("the waisted body against its copy, {op}");
        let Err(BooleanError::CurvedPierceUnsupported { face, edge, .. }) = r else {
            panic!("{label}: {r:?}");
        };
        // Both operands are the one body, so either's keys read it.
        let surface = body
            .get_face(face)
            .and_then(|f| body.get_surface(f.surface))
            .cloned();
        let carrier = body
            .get_edge(edge)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(|g| g.certified())
            .map(|c| c.carrier().clone());
        let (
            Some(geom::Surface::Cone {
                axis, half_angle, ..
            }),
            Some(geom::Curve3::Line { dir, .. }),
        ) = (surface, carrier)
        else {
            panic!("{label}: a line edge against a cone face");
        };
        // The line × cone quadratic's lead coefficient is
        // `(d̂·a)² − cos²α`: zero, the lane's degenerate pose.
        let lead = (dir.normalize().dot(axis)).powi(2) - half_angle.cos().powi(2);
        assert!(
            lead.abs() < 1e-12,
            "{label}: the refused edge runs parallel to a generator of the face's cone: {lead}"
        );
    }
}

/// **A curved body against its copy moved by half the band or by ten
/// bands** builds or refuses typed, and what it builds is right. Within
/// the band the copy is the body: `∪` and `∩` are the body at its closed
/// form, and a difference is nothing; the ball, the frustum and the dome
/// moved across its axis build every op. Ten bands off, an answer
/// that builds is a sound body: the union and the intersection sum to
/// the two volumes, and a difference with the intersection to one.
#[test]
fn a_copy_moved_within_or_past_the_band_builds_right_or_refuses() {
    let eps = tol().eps();
    for (label, body, want) in bodies() {
        for (by, within) in [
            (Vec3::new(0.5 * eps, 0.0, 0.0), true),
            (Vec3::new(0.0, 0.5 * eps, 0.0), true),
            (Vec3::new(10.0 * eps, 0.0, 0.0), false),
            (Vec3::new(0.0, 10.0 * eps, 0.0), false),
        ] {
            // Within the band the ball and the frustum build either
            // way, the dome moved across its axis; the rest refuse in
            // band today.
            let builds = within
                && (matches!(label, "a ball" | "a frustum") || (label == "a dome" && by.x > 0.0));
            let copy = shifted(&body, by);
            let mut volumes = Vec::new();
            for (op, r) in every_op(&body, &copy) {
                let label = format!("{label} against its copy moved {by:?}, {op}");
                match r {
                    Err(BooleanError::ClassificationInvariant { what }) => {
                        panic!("{label}: a kernel invariant: {what}")
                    }
                    Err(e) => assert!(!builds, "{label}: builds: {e:?}"),
                    Ok(BooleanResult::Empty) => {
                        assert!(op.contains('∖'), "{label}: only a difference is nothing");
                        volumes.push((op, 0.0));
                    }
                    r @ Ok(BooleanResult::Body(_)) if within => {
                        assert!(!op.contains('∖'), "{label}: within the band, nothing");
                        one_body(&label, r, want);
                    }
                    Ok(BooleanResult::Body(bb)) => {
                        assert_eq!(
                            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
                            Ok(()),
                            "{label}: tier 3′"
                        );
                        let m = topo::test_support::pinned_volume(&label, &bb.body, tol());
                        volumes.push((op, m.volume));
                    }
                }
            }
            if within {
                continue;
            }
            let v = |op| volumes.iter().find(|(o, _)| *o == op).map(|&(_, v)| v);
            let slack = 1e-6 * want;
            if let (Some(u), Some(i)) = (v("A ∪ B"), v("A ∩ B")) {
                assert!(
                    (u + i - 2.0 * want).abs() <= slack,
                    "{label} moved {by:?}: ∪ {u} and ∩ {i} against twice {want}"
                );
            }
            for d in ["A ∖ B", "B ∖ A"] {
                if let (Some(d), Some(i)) = (v(d), v("A ∩ B")) {
                    assert!(
                        (d + i - want).abs() <= slack,
                        "{label} moved {by:?}: a difference {d} and ∩ {i} against {want}"
                    );
                }
            }
        }
    }
}

/// **A curved body against its copy turned about its own axis**, the
/// same point set in another chart (the ball's pole turned off its axis
/// too): `∪` and `∩` are the body, each difference nothing.
#[test]
fn a_curved_body_against_its_copy_in_another_chart_is_one_body() {
    use geom_core::Point3;
    let about =
        |axis: Vec3<f64>, angle: f64| Affine3::rotation_about_axis(Point3::origin(), axis, angle);
    let [ball, ..] = bodies();
    let pole_turned = (
        ball.0,
        ball.1,
        ball.2,
        about(Vec3::new(1.0, 1.0, 0.0).normalize(), 0.7),
    );
    let cases = bodies().into_iter().map(|(label, body, want)| {
        let axis = if label == "a ball" {
            Vec3::unit_z()
        } else {
            Vec3::unit_y()
        };
        (label, body, want, about(axis, 1.0))
    });
    for (label, body, want, turn) in cases.chain([pole_turned]) {
        let turned =
            topo::transform_rigid(&body, &geom_core::test_support::rigid(&turn, tol()), tol())
                .unwrap();
        for (op, r) in every_op(&body, &turned) {
            let label = format!("{label} against its copy turned, {op}");
            if op.contains('∖') {
                assert!(
                    matches!(r, Ok(BooleanResult::Empty)),
                    "{label}: one body less itself is nothing: {r:?}"
                );
            } else {
                one_body(&label, r, want);
            }
        }
    }
}
