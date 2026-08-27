//! **What `shell` survives, tabulated** — the teapot scene's two
//! findings, generalized past the teapot.
//!
//! `demos/tour/src/teapot.rs` pins both of them LIVE, on the teapot's
//! own bodies: wall 1 is the bellied pot's hollow and wall 2 is the
//! opened pot's mesh. A pin on one body says the door refused THAT
//! body. This file is the other half — the claim the scene's prose
//! makes about the CLASS, executed:
//!
//! 1. **the hollow refuses at OBLIQUE junctions, and curvature has
//!    nothing to do with it.** A box and a right prism on an L hollow;
//!    a right prism on a TRIANGLE does not. A cylinder between two
//!    caps normal to its axis hollows; a cone between the same two
//!    caps does not, and neither does a sphere zone. A TANGENT
//!    junction refuses too, at a second door — and the test says why
//!    that row cannot be attributed to tangency alone. The surviving
//!    class is stated in `the_hollow_survives_exactly_the_square_junction`.
//! 2. **the opened rim is wrong on every solid of revolution**, and
//!    right on a box. Not a tolerance lottery: `the_opened_rim_is_wrong_
//!    on_every_revolve` sweeps five wall thicknesses, three mouth radii
//!    spanning a factor of 24, and three chord budgets, and the answer
//!    is the same every time — including on the simplest possible
//!    fixture, a cylindrical drum.
//!
//! Both tables are PLANTED REDS in the direction of the fix: each
//! refusal is asserted by its exact variant and each defect by its
//! exact wrong number, so a door that grows either case fails here and
//! sends the reader to the scene's own retire notes.
//!
//! # The sweeps, and what they could not match
//!
//! The fixtures are solids of revolution and right prisms, built
//! through `revolve` and `extrude`. What that leaves untested, stated
//! rather than implied: NURBS walls (the offset door's approximating
//! lane, which no fixture here enters); junctions between two CURVED
//! faces on one body; multi-designation openings; and operands whose
//! charts are shared by adoption rather than by a revolve's seam.
//! A tangent junction IS covered (`bullet`), because the reasoning
//! that predicts the oblique class predicts that one too, and a
//! prediction nothing executes is a claim — but it is a two-variable
//! row, and the test says so rather than reading it as a third
//! confirmation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::{p2, validated};
use pncad::geom::Surface;
use pncad::geom_core::{Band, Point2, Tol, Vec2};
use pncad::prelude::{Open, Start};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use pncad::topo::{Body, ReplaceFaceError, ShellError};

/// The fit tolerance the offset door's NURBS lane would use. Unread on
/// every fixture here — all analytic.
const FIT_TOL: f64 = 1e-6;
/// Every fixture's mouth plane.
const TOP: f64 = 8.0 / 64.0;

fn band(tol: Tol) -> Band {
    Band::linear(tol).expect("the run's band")
}

fn revolved(lp: ProfileLoop<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("the meridian validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the meridian fully revolves")
    .body
}

fn extruded(lp: ProfileLoop<f64>, h: f64, tol: Tol) -> Body<f64> {
    extrude(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("the footprint validates"),
        Extrusion::Distance(h),
        tol,
    )
    .expect("the footprint extrudes")
    .body
}

/// A cylinder of radius `r` between two caps normal to its axis.
fn drum(r: f64, tol: Tol) -> Body<f64> {
    revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(r, 0.0), tol)
            .expect("base")
            .line_to(Point2::new(r, TOP), tol)
            .expect("wall")
            .line_to(Point2::new(0.0, TOP), tol)
            .expect("mouth")
            .line_to(Start, tol)
            .expect("axis")
            .into(),
        tol,
    )
}

/// A cone frustum between the same two caps: the junctions are the
/// drum's, tilted.
fn frustum(tol: Tol) -> Body<f64> {
    revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(4.0 / 64.0, 0.0), tol)
            .expect("base")
            .line_to(Point2::new(2.0 / 64.0, TOP), tol)
            .expect("cone")
            .line_to(Point2::new(0.0, TOP), tol)
            .expect("mouth")
            .line_to(Start, tol)
            .expect("axis")
            .into(),
        tol,
    )
}

/// A sphere zone between the same two caps.
fn barrel(tol: Tol) -> Body<f64> {
    revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(3.0 / 64.0, 0.0), tol)
            .expect("base")
            .arc_to(
                Center {
                    c: Point2::new(0.0, 1.0 / 16.0),
                    winding: ArcSweep::Ccw,
                    p: Point2::new(3.0 / 64.0, TOP),
                },
                tol,
            )
            .expect("belly")
            .line_to(Point2::new(0.0, TOP), tol)
            .expect("mouth")
            .line_to(Start, tol)
            .expect("axis")
            .into(),
        tol,
    )
}

/// A cylinder capped by a hemisphere of its own radius: the one
/// junction on the body is TANGENT, and the base cap is the drum's.
fn bullet(tol: Tol) -> Body<f64> {
    let r = 3.0 / 64.0;
    revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(r, 0.0), tol)
            .expect("base")
            .line_to(Point2::new(r, TOP), tol)
            .expect("wall")
            // The tangency is DECLARED, not stumbled into: the
            // lattice refuses an implicit G1 junction (`JunctionTangent`)
            // and names this door, so the dome meets the wall tangentially
            // by construction rather than to within a margin.
            .tangent()
            .tangent_arc_to(Point2::new(0.0, TOP + r), tol)
            .expect("the dome is tangent to the wall at the equator")
            .line_to(Start, tol)
            .expect("axis")
            .into(),
        tol,
    )
}

/// A right prism on a rectangle: every dihedral square.
fn boxy(tol: Tol) -> Body<f64> {
    extruded(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(0.2, 0.0), tol)
            .expect("a")
            .line_to(Point2::new(0.2, 0.3), tol)
            .expect("b")
            .line_to(Point2::new(0.0, 0.3), tol)
            .expect("c")
            .line_to(Start, tol)
            .expect("d")
            .into(),
        0.25,
        tol,
    )
}

/// A right prism on an L: every dihedral square, the footprint not
/// convex.
fn l_prism(tol: Tol) -> Body<f64> {
    extruded(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(0.6, 0.0), tol)
            .expect("a")
            .line_to(Point2::new(0.6, 0.2), tol)
            .expect("b")
            .line_to(Point2::new(0.2, 0.2), tol)
            .expect("c")
            .line_to(Point2::new(0.2, 0.6), tol)
            .expect("d")
            .line_to(Point2::new(0.0, 0.6), tol)
            .expect("e")
            .line_to(Start, tol)
            .expect("f")
            .into(),
        0.3,
        tol,
    )
}

/// A right prism on a TRIANGLE: every side normal to both caps, and
/// not one of the three side-to-side dihedrals square.
fn triangular_prism(tol: Tol) -> Body<f64> {
    extruded(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(0.5, 0.0), tol)
            .expect("a")
            .line_to(Point2::new(0.25, 0.4), tol)
            .expect("b")
            .line_to(Start, tol)
            .expect("c")
            .into(),
        0.3,
        tol,
    )
}

/// The offset door's own refusal, as a two-word class name plus what
/// it measured — read off the payload, never off the message.
fn offset_refusal(e: &ShellError<f64>) -> String {
    match e {
        ShellError::Face { error, .. } => match &**error {
            ReplaceFaceError::ReanchorOffCarrier { gap, .. } => {
                assert!(*gap > 0.0, "the gap is a distance in meters, got {gap}");
                "ReanchorOffCarrier".to_string()
            }
            ReplaceFaceError::CarrierLaneUnsupported { .. } => "CarrierLaneUnsupported".to_string(),
            other => panic!("an unexpected face-offset refusal: {other}"),
        },
        other => panic!("the refusal is not the offset door's: {other}"),
    }
}

/// **The hollow's surviving class, tabulated.**
///
/// The rule the table exhibits: `shell` replaces one CHART at a time,
/// and the door re-anchors every edge that ends at a moved vertex on
/// its own carrier — which has not moved yet. So a junction survives
/// exactly when the neighbouring surface is INVARIANT under the moved
/// face's offset motion. A plane's offset is a translation along its
/// normal, and a cylinder is invariant under translation along its
/// axis; a cylinder's offset is a radial shrink, and a plane normal to
/// the axis is invariant under that. That pair is the whole surviving
/// class, and the box is in it because every one of its faces is
/// normal to every neighbour.
///
/// Everything else moves its neighbour's edge off the neighbour:
/// two planes meeting at 63°, a cone against a cap, a sphere zone
/// against a cap. The gap the refusal carries IS that distance in
/// meters, and it is checked to be a real positive length rather than
/// a tag.
///
/// **The tangent row refuses at a DIFFERENT door, and the table does
/// not attribute that to tangency.** A tangent junction is only
/// reachable through the lattice's `.tangent().tangent_arc_to(..)`
/// declaration — an implicit G1 joint refuses `JunctionTangent`
/// upstream — and that door authors a MAPPED arc description. So this
/// fixture differs from its neighbours in two ways at once, and
/// nothing outside the kernel can separate them; what is recorded is
/// the outcome and which door produced it.
#[test]
fn the_hollow_survives_exactly_the_square_junction() {
    let tol = Tol::witness();
    let t = 1.0 / 128.0;
    for (what, body, thickness) in [
        (
            "a drum: cylinder between two caps normal to its axis",
            drum(3.0 / 64.0, tol),
            t,
        ),
        ("a right prism on a rectangle", boxy(tol), 0.02),
        ("a right prism on an L", l_prism(tol), 0.02),
    ] {
        pncad::topo::shell(&body, thickness, FIT_TOL, band(tol), tol)
            .unwrap_or_else(|e| panic!("{what} hollows, got {e}"));
    }
    for (what, body, thickness, door) in [
        (
            "a cone frustum between two caps",
            frustum(tol),
            t,
            "ReanchorOffCarrier",
        ),
        (
            "a sphere zone between two caps",
            barrel(tol),
            t,
            "ReanchorOffCarrier",
        ),
        (
            "a right prism on a triangle",
            triangular_prism(tol),
            0.02,
            "ReanchorOffCarrier",
        ),
        (
            "a hemisphere TANGENT to its cylinder",
            bullet(tol),
            t,
            "CarrierLaneUnsupported",
        ),
    ] {
        let e = pncad::topo::shell(&body, thickness, FIT_TOL, band(tol), tol)
            .expect_err("this junction is not square, so the hollow must refuse");
        assert_eq!(
            offset_refusal(&e),
            door,
            "{what}: the door that refuses is part of the finding, not an incidental"
        );
    }
}

/// **The opened rim, on the acceptance corpus's own shape.** A box
/// opened at its top is a cup: genus 0, ONE ring on the designated
/// face, and it meshes. This row is the control for the one below.
#[test]
fn the_opened_rim_is_right_on_a_box() {
    let tol = Tol::witness();
    let body = boxy(tol);
    let top: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9
                        && (origin.z - 0.25).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(top.len(), 1, "an extrusion's cap is ONE face");
    let cup = pncad::topo::shell_open(&body, 0.02, &top, FIT_TOL, band(tol), tol)
        .expect("a box opens at its top");
    assert_eq!(
        (rings(&cup), genus(&cup)),
        (1, 0),
        "one ring on the designated face, and a cup is genus 0"
    );
    pncad::mesh::tessellate(&cup, 1e-3, tol).expect("and it meshes");
}

/// **The opened rim on a REVOLVE is wrong, everywhere.** Sweeping
/// wall thickness over a factor of 5, mouth radius over a factor of
/// 24 (0.041 m to 1 m), the pot's stepped meridian against the
/// simplest fixture there is, and three chord budgets over a factor of
/// 50: every one comes back genus 1 with TWO rings — one on each of
/// the designated chart's two half-discs — and refuses tessellation on
/// a half-disc of the mouth plane. There is no corner of this
/// parameter space where it is right, which is what separates it from
/// the `mesh::planar` sub-floor lottery (klein's wall 7, #555) that
/// fires at one flare angle and not its neighbour.
#[test]
fn the_opened_rim_is_wrong_on_every_revolve() {
    let tol = Tol::witness();
    let cases: Vec<(String, Body<f64>, f64)> =
        [1.0 / 128.0, 1.0 / 100.0, 1.0 / 64.0, 0.003, 0.0055]
            .into_iter()
            .map(|t| (format!("drum, t = {t}"), drum(3.0 / 64.0, tol), t))
            .chain(
                [0.041_25, 3.0 / 64.0, 1.0]
                    .into_iter()
                    .map(|r| (format!("drum, mouth radius {r}"), drum(r, tol), 1.0 / 128.0)),
            )
            .chain(core::iter::once((
                "the teapot's own stepped meridian".to_string(),
                teapot_pot(tol),
                1.0 / 128.0,
            )))
            .collect();
    for (what, body, t) in cases {
        let chart: Vec<_> = body
            .faces()
            .filter(|(_, f)| {
                matches!(body.get_surface(f.surface),
                    Some(Surface::Plane { origin, .. }) if (origin.y - TOP).abs() < 1e-12)
            })
            .map(|(k, _)| k)
            .collect();
        assert_eq!(chart.len(), 2, "{what}: a revolved cap is two half-discs");
        let cup = pncad::topo::shell_open(&body, t, &chart, FIT_TOL, band(tol), tol)
            .unwrap_or_else(|e| panic!("{what}: the opened arm still returns a body, got {e}"));
        assert_eq!(
            pncad::topo::validate_geometric(&cup, tol),
            Ok(()),
            "{what}: and it passes tier 3, which is why this table exists"
        );
        assert_eq!(
            (rings(&cup), genus(&cup)),
            (2, 1),
            "{what}: a ring on EACH half-disc, and genus 1 where a cup's is 0"
        );
        for delta in [1e-2, 1e-3, 2e-4] {
            let e = pncad::mesh::tessellate(&cup, delta, tol)
                .err()
                .unwrap_or_else(|| panic!("{what}: δ = {delta} must refuse"));
            let pncad::mesh::TessellateError::Triangulation { face } = e else {
                panic!("{what}: δ = {delta}: expected the CDT's insertion refusal, got {e:?}");
            };
            let f = cup.get_face(face).expect("the refusing face");
            assert!(
                matches!(cup.get_surface(f.surface),
                    Some(Surface::Plane { origin, .. }) if (origin.y - TOP).abs() < 1e-12)
                    && f.rings.len() == 1,
                "{what}: δ = {delta}: the face that refuses is a MOUTH half-disc \
                 carrying the spurious ring, not some other face"
            );
        }
    }
}

/// The teapot's own vessel meridian, kept in step with the scene by
/// its stations rather than by sharing code: `demos/tour/src/teapot.rs`
/// is a binary's module and a test cannot import it, so what ties the
/// two is that both are checked against the same numbers — the scene
/// asserts its census, this file asserts the defect, and a change to
/// either meridian moves one of those two.
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

fn rings(body: &Body<f64>) -> usize {
    body.faces().map(|(_, f)| f.rings.len()).sum()
}

fn genus(body: &Body<f64>) -> i64 {
    let (v, e, f) = (
        body.vertices().count() as i64,
        body.edges().count() as i64,
        body.faces().count() as i64,
    );
    body.shells().count() as i64 - (v - e + f - rings(body) as i64) / 2
}
