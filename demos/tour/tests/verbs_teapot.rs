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
//!    caps does not, and neither does a sphere zone, and neither do a
//!    partial revolve's meridian caps — planes CONTAINING the axis
//!    rather than normal to it, which is the same rule from the
//!    direction the first cut of this table omitted. A CURVED
//!    neighbour refuses at a SECOND door, and that door is about the
//!    neighbour's offset not being a rigid translation rather than
//!    about tangency: `lifted_dome` is a definitely-non-tangent
//!    fixture that refuses at the identical site with the identical
//!    `what` string, which the table now asserts.
//! 2. **the opened rim is wrong on every solid of revolution**, and
//!    right on a box. Not a tolerance lottery:
//!    `the_opened_rim_is_wrong_on_every_revolve` sweeps five wall
//!    thicknesses, mouth radii at two scales (41–47 mm and 1 m) and
//!    three chord budgets, and the answer is the same every time —
//!    including on the simplest possible fixture, a cylindrical drum.
//!    And the MECHANISM is not the one this file first asserted:
//!    `the_seam_split_is_not_the_mechanism` designates a revolved
//!    TUBE's mouth, which is ONE face on a body with no axis apex, and
//!    the rim is wrong there too — in a different shape (genus 2, one
//!    ring). The class both shapes belong to is *a designated face
//!    whose cavity counterpart's boundary cannot become an
//!    interior-disjoint RING of it*, and on an annular cap the correct
//!    rim is two disjoint annuli, a face SPLIT the surgery cannot
//!    express.
//!
//! **Two claims this file used to make and no longer does**, both
//! retracted on measurement rather than on argument: that the tangent
//! row's door could not be separated from tangency (`lifted_dome`
//! separates it), and that the opened rim's discriminator was "one
//! face versus two half-discs on a chart"
//! (`the_seam_split_is_not_the_mechanism` falsifies it). The fixtures
//! that did it were contributed by the review of #1078 and are merged
//! beside this file as `verbs_teapot_r1_probes.rs` and
//! `verbs_teapot_r2_probes.rs`.
//!
//! Every table here is a PLANTED RED in the direction of the fix: each
//! refusal is asserted by its exact variant (and, at the second door,
//! its exact `what`), each defect by its exact wrong numbers. A door
//! that grows any of these cases fails here and sends the reader to
//! the retire note beside the assertion.
//!
//! # The sweeps, and what they could not match
//!
//! The fixtures are solids of revolution and right prisms, built
//! through `revolve` and `extrude`. What that leaves untested, stated
//! rather than implied: NURBS walls (the offset door's approximating
//! lane, which no fixture here enters); junctions between two CURVED
//! faces on one body; multi-designation openings; and operands whose
//! charts are shared by adoption rather than by a revolve's seam.
//! A tangent junction IS covered (`bullet`), and so is its
//! non-tangent twin (`lifted_dome`), which is what turns that row from
//! a two-variable observation into a one-variable one.

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

/// A cylinder capped by a dome that is definitely NOT tangent to it:
/// the dome's centre is lifted `d` above the wall's top, so the
/// meridian turns through a real angle there, and the arc is authored
/// through the ordinary `Center` door rather than the tangent one.
/// The discriminator for the second door (see the table's docs).
fn lifted_dome(tol: Tol) -> Body<f64> {
    let r: f64 = 3.0 / 64.0;
    let d: f64 = 0.02;
    let rr = (r * r + d * d).sqrt();
    revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(r, 0.0), tol)
            .expect("base")
            .line_to(Point2::new(r, TOP), tol)
            .expect("wall")
            .arc_to(
                Center {
                    c: Point2::new(0.0, TOP + d),
                    winding: ArcSweep::Ccw,
                    p: Point2::new(0.0, TOP + d + rr),
                },
                tol,
            )
            .expect("a dome whose centre is off the wall's top is not tangent to it")
            .line_to(Start, tol)
            .expect("axis")
            .into(),
        tol,
    )
}

/// A QUARTER revolve of the drum's own meridian: a wedge. Its two
/// meridian caps are planes CONTAINING the cylinder's axis, which is
/// the direction the surviving class's "normal to the axis" excludes
/// from the other side.
fn wedge(tol: Tol) -> Body<f64> {
    let r = 3.0 / 64.0;
    let profile = validated(
        SketchPlane::xy(),
        vec![
            Open.at(Point2::new(0.0, 0.0))
                .line_to(Point2::new(r, 0.0), tol)
                .expect("base")
                .line_to(Point2::new(r, TOP), tol)
                .expect("wall")
                .line_to(Point2::new(0.0, TOP), tol)
                .expect("top")
                .line_to(Start, tol)
                .expect("axis")
                .into(),
        ],
        tol,
    )
    .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol,
    )
    .expect("a quarter turn")
    .body
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
/// not one of the three side-to-side dihedrals square — they are the
/// footprint's own interior angles, 58°, 58° and 64°.
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

/// How many of `body`'s edges are still described through the
/// SCAFFOLDING door — the arm `CarrierLaneUnsupported`'s "not a rigid
/// translation" `what` is raised on, and the only one.
fn scaffold_descriptions(body: &Body<f64>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(pncad::topo::CurveGeom::certified)
                    .map(pncad::topo::EdgeCurve::description),
                Some(&pncad::topo::EdgeDescription::Scaffold(_))
            )
        })
        .count()
}

/// **The retired door, demonstrated rather than argued** (PCURVE
/// P-1b).
///
/// `CarrierLaneUnsupported`'s *"a mapped description whose surface's
/// offset is not a rigid translation"* is raised on exactly ONE
/// description arm — the scaffolding door, whose payload is a
/// pushforward stated in 3-SPACE and therefore has to be translated
/// with the face it hangs off. Once U2 collapsed the conventional
/// descriptions onto a chart image, stated in CHART coordinates, there
/// is nothing left to translate: the offset re-parameterizes the chart
/// and the image drawn in it is untouched. So the door stops firing —
/// not because the obstruction went away, but because that particular
/// obstruction was an artefact of writing a conventional locus in
/// 3-space.
///
/// This asserts the PREMISE on the fixtures themselves rather than
/// reasoning about it: neither curved-neighbour body carries a
/// scaffolding description at all, so the arm cannot be entered, and
/// the refusal necessarily moves to whatever obstruction is really
/// there. (It is still a refusal — the junction is still not square,
/// which is finding 1 and is untouched.)
#[test]
fn the_not_a_rigid_translation_door_is_unreachable_at_rest() {
    let tol = Tol::witness();
    let mut doors = Vec::new();
    for (what, body) in [
        ("a hemisphere TANGENT to its cylinder", bullet(tol)),
        (
            "a dome whose centre is lifted clear of the wall's top",
            lifted_dome(tol),
        ),
    ] {
        assert_eq!(
            scaffold_descriptions(&body),
            0,
            "{what}: every edge of a body at rest says which chart it lies in, so the \
             scaffolding arm the retired door hangs off cannot be entered"
        );
        let e = pncad::topo::shell(&body, 1.0 / 128.0, FIT_TOL, band(tol), tol)
            .expect_err("this junction is not square, so the hollow must refuse");
        doors.push((what, offset_refusal(&e)));
    }
    // Both fixtures land on the obstruction that is really there —
    // the same door the table's WEDGE row has always expected.
    assert_eq!(
        doors,
        vec![
            (
                "a hemisphere TANGENT to its cylinder",
                "ReanchorOffCarrier".to_string()
            ),
            (
                "a dome whose centre is lifted clear of the wall's top",
                "ReanchorOffCarrier".to_string()
            ),
        ],
        "the retired door is unreachable, so these must refuse elsewhere"
    );
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
            // The STRING as well as the variant: this door has more than
            // one `what`, and which one fires IS the finding — "the
            // neighbour's offset is not a rigid translation" is a
            // statement about the neighbouring surface, not about how
            // the meridian was authored.
            ReplaceFaceError::CarrierLaneUnsupported { what, .. } => {
                assert_eq!(
                    *what, "a mapped description whose surface's offset is not a rigid translation",
                    "this door's OTHER `what` (a carrier that is neither a line nor a circle) \
                     would be a different finding"
                );
                "CarrierLaneUnsupported".to_string()
            }
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
/// Everything else moves its neighbour's edge off the neighbour: the
/// triangular prism's side planes (its footprint's interior angles are
/// 58°, 58° and 64°, and the dihedral between two side planes IS that
/// angle), a cone against a cap, a sphere zone against a cap. The gap
/// the refusal carries IS that distance in meters, and it is checked to
/// be a real positive length rather than a tag.
///
/// **The second door is about the NEIGHBOUR'S OFFSET, not about
/// tangency**, and that is measured rather than reasoned. A dome
/// authored by an ordinary `Center` arc with its centre lifted clear
/// of the wall's top — definitely NOT a tangent junction, and not the
/// `.tangent().tangent_arc_to(..)` route — refuses at the IDENTICAL
/// site with the IDENTICAL `what` string. So the variable is the
/// neighbouring SPHERE: its inward offset is a radius change, not a
/// rigid translation, and the door has no transport lane for a mapped
/// description on such a surface. `bullet` is kept as the table's row
/// for that door and `lifted_dome` beside it is the discriminator;
/// `verbs_teapot_r2_probes::r2_tangent_bullet_which_door` is where the
/// pair was first measured.
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
        // The discriminator, not a third confirmation: same pair,
        // same door, and NOT tangent. Tangency is not the variable.
        (
            "a dome whose centre is lifted clear of the wall's top",
            lifted_dome(tol),
            t,
            "CarrierLaneUnsupported",
        ),
        // The rule reads "a plane NORMAL to a cylinder's axis" — this
        // row comes at it from the direction the rest of the table
        // omits: a partial revolve's meridian caps are planes
        // CONTAINING that axis, and they refuse like everything else
        // outside the class.
        (
            "a quarter-revolve WEDGE",
            wedge(tol),
            t,
            "ReanchorOffCarrier",
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

/// **The opened rim on a REVOLVE is wrong, everywhere.** Sweeping five
/// wall thicknesses over a factor of 5, mouth radii at two scales
/// (41–47 mm, and 1 m — the small pair are 14% apart, so this is two
/// scales rather than a continuum), the pot's stepped meridian against
/// the simplest fixture there is, and three chord budgets over a
/// factor of 50: every one comes back genus 1 with TWO rings and
/// refuses tessellation on a half-disc of the mouth plane. There is no
/// corner of this parameter space where it is right, which is what
/// separates it from the `mesh::planar` sub-floor lottery (klein's
/// wall 7, #555) that fires at one flare angle and not its neighbour.
///
/// # What the class is NOT — and this row does not get to say it alone
///
/// The obvious discriminator, and the one this file first asserted,
/// was "an extrusion's cap is ONE face; a full revolve's is TWO
/// half-discs sharing a chart". That is FALSE as a mechanism, and two
/// adopted review fixtures are what falsify it, both in-tree and both
/// run by the same `cargo test --release`:
/// `verbs_teapot_r2_probes::r2_revolved_tube_separates_seam_from_axis`
/// designates the mouth of a revolved TUBE — one face, because a
/// closed off-axis profile closes its own seam — and the result is
/// wrong there too; `r2_partial_revolve_one_cap_face` comes at it from
/// the other side with a wedge, whose cap is one face and does touch
/// the axis.
///
/// **The class, restated from what those measure:** a designated face
/// is safe exactly when its CAVITY COUNTERPART's boundary can become
/// an interior-disjoint RING of it. A box's can — the inner rectangle
/// sits strictly inside the outer one. A revolved cap's cannot: on an
/// axis-touching cap the counterpart's boundary is a D-loop that
/// reaches the same axis apex the outer loop owns and runs back along
/// the outer loop's own seam legs (measured in
/// `verbs_teapot_r1_probes` and `r2_ring_anatomy_on_a_drum`), and that
/// CONTACT is why the CDT refuses. On an ANNULAR cap the correct
/// answer is not a ring at all but TWO DISJOINT ANNULI — a face SPLIT,
/// which `kfmrh` has no way to express (`r2_annular_mouth_anatomy`).
/// So this is not one ring-placement bug; it is a surgery whose only
/// output shape is "outer loop plus rings".
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
        let chart = plane_chart_at(&body, TOP);
        assert_eq!(
            chart.len(),
            2,
            "{what}: a full revolve's cap is two half-discs"
        );
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

/// Every planar face of `body` whose plane sits at station `y` — the
/// chart, not a face.
///
/// **Spelled twice in this PR**, here and as `teapot::plane_chart_at`
/// in the scene: an integration test cannot import a binary's module,
/// and copying five lines is the whole cost of that. The copies are
/// tied by what they are checked against — both are asserted to return
/// the face count the fixture's own geometry implies (2 for a full
/// revolve's cap, 1 for an extrusion's), so a change to what a chart
/// means fails on both sides rather than drifting on one.
fn plane_chart_at(body: &Body<f64>, y: f64) -> Vec<pncad::topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// Duplicated from the scene for the same reason as
/// [`plane_chart_at`]; see [`genus`].
fn rings(body: &Body<f64>) -> usize {
    body.faces().map(|(_, f)| f.rings.len()).sum()
}

/// The Euler–Poincaré genus. **Duplicated from `teapot::genus`**, and
/// deliberately: a binary's module cannot be imported by an
/// integration test, and this is three lines of a published identity
/// rather than a shared invariant. Both copies check the parity before
/// dividing, because an odd `v − e + f − r` is a census that does not
/// satisfy the identity at all and halving it would turn that into a
/// plausible number.
fn genus(body: &Body<f64>) -> i64 {
    let (v, e, f) = (
        body.vertices().count() as i64,
        body.edges().count() as i64,
        body.faces().count() as i64,
    );
    let chi = v - e + f - rings(body) as i64;
    assert!(
        chi % 2 == 0,
        "v - e + f - r = {chi} is ODD, so this census does not satisfy \
         Euler-Poincare and no genus follows from it"
    );
    body.shells().count() as i64 - chi / 2
}

/// **The seam split is NOT the mechanism, asserted rather than cited.**
///
/// The row above sweeps axis-touching caps, every one of which is two
/// half-discs on one chart — so on its own it cannot separate "the cap
/// is two faces" from "the cap touches the axis" from anything else.
/// This row removes both variables at once: a revolved TUBE's meridian
/// is a closed off-axis loop, so it closes its own seam and its mouth
/// chart is exactly ONE face, with no axis apex anywhere on the body.
/// The rim is still wrong, and wrong in a DIFFERENT shape — genus 2
/// with one ring, against the axis-touching cap's genus 1 with two.
///
/// What both shapes have in common is the class the register now
/// carries: the cavity counterpart's boundary cannot become an
/// interior-disjoint ring of the designated face. Here the correct rim
/// would be TWO DISJOINT ANNULI — a face SPLIT, which the `kfmrh`
/// surgery has no output shape for at all — and what the verb returns
/// instead is a single ring the CDT then refuses.
///
/// (`verbs_teapot_r2_probes::r2_revolved_tube_separates_seam_from_axis`
/// and `r2_annular_mouth_anatomy` are where this was first measured and
/// where the loop anatomy is printed face by face; this row is the
/// planted red, so a fix reds here.)
#[test]
fn the_seam_split_is_not_the_mechanism() {
    let tol = Tol::witness();
    let (ri, ro, h) = (0.30, 0.50, 0.40);
    let body = revolved(
        Open.at(Point2::new(ri, 0.0))
            .line_to(Point2::new(ro, 0.0), tol)
            .expect("base annulus")
            .line_to(Point2::new(ro, h), tol)
            .expect("outer wall")
            .line_to(Point2::new(ri, h), tol)
            .expect("mouth annulus")
            .line_to(Start, tol)
            .expect("the bore closes the meridian")
            .into(),
        tol,
    );
    let chart = plane_chart_at(&body, h);
    assert_eq!(
        chart.len(),
        1,
        "a closed OFF-AXIS meridian closes its own seam, so this cap is ONE face — \
         which is the whole point of the row"
    );
    let cup = pncad::topo::shell_open(&body, 0.05, &chart, FIT_TOL, band(tol), tol)
        .expect("the opened arm returns a body here too");
    assert_eq!(
        pncad::topo::validate_geometric(&cup, tol),
        Ok(()),
        "and tiers 1-3 bless it, exactly as they bless the axis-touching case"
    );
    assert_eq!(
        (rings(&cup), genus(&cup)),
        (1, 2),
        "MEASURED, not wanted: one ring and genus 2 on a body with no seam split and no \
         axis apex. When this stops reading (1, 2), re-derive the class in \
         docs/KERNEL-VERBS.md and #1082 from what it says instead"
    );
    let e = pncad::mesh::tessellate(&cup, 1e-3, tol)
        .expect_err("and it does not mesh, for the same reason the axis-touching one does not");
    let pncad::mesh::TessellateError::Triangulation { face } = e else {
        panic!("expected the CDT's insertion refusal, got {e:?}");
    };
    let f = cup.get_face(face).expect("the refusing face");
    assert!(
        matches!(cup.get_surface(f.surface),
            Some(Surface::Plane { origin, .. }) if (origin.y - h).abs() < 1e-12)
            && f.rings.len() == 1,
        "the face that refuses is the MOUTH annulus carrying the ring, not some other face"
    );
}
