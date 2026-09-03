//! **VERBS-1031B — the arc-bounded winding arm**, measured at the
//! teapot cup.
//!
//! The cup is the live consumer of `merge_coplanar_faces`' coplanar
//! pair: `shell_open` on the teapot's own stepped meridian leaves three
//! full-valence coplanar pairs per side (the shoulders and their cavity
//! twins, each seam two disjoint collinear Line segments with all four
//! endpoints at valence 4) plus two pole-split base caps. The merge's
//! surgery completes on all of them; what refused was the ROLE pass,
//! because the merged annulus is bounded by circles and the winding
//! functional was line-bounded only.
//!
//! Both rows here are measurements of doors, not of a shape: one names
//! what the merge does, one names what the boolean gate says about the
//! unmerged operand. They move independently and are meant to.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2};
use profile::{Open, Profile, ProfileLoop, SketchPlane, Start};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, Surface};

const FIT_TOL: f64 = 1e-6;
const TOP: f64 = 8.0 / 64.0;

fn revolved(lp: ProfileLoop<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("the meridian validates"),
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the meridian fully revolves")
    .body
}

/// The teapot's own vessel meridian, transcribed from
/// `demos/tour/tests/verbs_teapot.rs::teapot_pot`.
fn teapot_pot(tol: Tol) -> Body<f64> {
    revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(3.0 / 64.0, 0.0), tol)
            .expect("base")
            .line_to(Point2::new(3.0 / 64.0, 1.0 / 64.0), tol)
            .expect("foot")
            .line_to(Point2::new(5.0 / 64.0, 1.0 / 64.0), tol)
            .expect("lower shoulder")
            .line_to(Point2::new(5.0 / 64.0, 6.0 / 64.0), tol)
            .expect("belly")
            .line_to(Point2::new(3.0 / 64.0, 6.0 / 64.0), tol)
            .expect("upper shoulder")
            .line_to(Point2::new(3.0 / 64.0, TOP), tol)
            .expect("neck")
            .line_to(Point2::new(0.0, TOP), tol)
            .expect("mouth")
            .line_to(Start, tol)
            .expect("axis")
            .into(),
        tol,
    )
}

fn plane_chart_at(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// The cup: the teapot pot, opened at its mouth chart.
fn teapot_cup(tol: Tol) -> Body<f64> {
    let body = teapot_pot(tol);
    let chart = plane_chart_at(&body, TOP);
    assert_eq!(chart.len(), 2, "a full revolve's cap is two half-discs");
    topo::shell_open(&body, 1.0 / 128.0, &chart, FIT_TOL, tol).expect("the cup opens")
}

/// A cutter box: `x in [0.02, 0.2]`, `y in [-0.01, 0.1]`, `z in [0, 0.3]`.
fn cutter(tol: Tol) -> Body<f64> {
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.02, -0.01))
        .line_to(Point2::new(0.2, -0.01), tol)
        .expect("south")
        .line_to(Point2::new(0.2, 0.1), tol)
        .expect("east")
        .line_to(Point2::new(0.02, 0.1), tol)
        .expect("north")
        .line_to(Start, tol)
        .expect("west")
        .into();
    sweep::extrude(
        &Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("a rectangle is a valid profile"),
        sweep::Extrusion::Distance(0.3),
        tol,
    )
    .expect("a rectangle extrudes")
    .body
}

/// **The differential**: the UNMERGED cup is a non-maximal operand and
/// the boolean gate says so, at `gate_maximal_faces`' same-surface-key
/// planar branch. This row does not move with the winding arm — it is
/// the statement that the cup's coplanar pairs are real, read by a
/// second door that never consults `loop_winding`.
#[test]
fn the_unmerged_cup_is_a_non_maximal_operand() {
    let tol = Tol::witness();
    let cup = teapot_cup(tol);
    assert!(
        matches!(
            topo::boolean::subtract(&cup, &cutter(tol), tol),
            Err(topo::BooleanError::NonMaximalFaces {
                operand: topo::Operand::A,
                ..
            })
        ),
        "the unmerged cup's own coplanar pairs are what F7 refuses"
    );
}

/// **The head measurement (VERBS-1031B's opening row).** The cup's
/// merge refuses `MergedFaceRoleAmbiguous`, naming the merged annulus,
/// because that annulus's outline and ring are both CIRCLES and
/// `loop_winding`'s `all_lines` guard answers `None` for each — the
/// role pass then sees no positively-wound cycle at all.
#[test]
fn the_cup_merge_refuses_on_an_arc_bounded_annulus() {
    let tol = Tol::witness();
    let mut cup = teapot_cup(tol);
    assert_eq!(
        (
            cup.faces().count(),
            cup.vertices().count(),
            cup.edges().count()
        ),
        (25, 26, 48),
        "the cup's census at rest"
    );
    let out = cup.merge_coplanar_faces(tol);
    assert!(
        matches!(
            out,
            Err(topo::MergeCoplanarError::MergedFaceRoleAmbiguous { .. })
        ),
        "the arc-bounded annulus has no decidable winding, got {out:?}"
    );
}
