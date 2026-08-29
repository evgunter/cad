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
//!
//!    **The door survived U2's description collapse, and moved ARMS**
//!    (PCURVE P-1b). A conventional locus written as a 3-space
//!    pushforward has to be carried bodily with the face it hangs off.
//!    U2 restated such loci as chart IMAGES, in the chart's own
//!    coordinates, which an offset re-parameterizes without moving —
//!    so for an edge the kernel DERIVED (a seam, an iso boundary, a
//!    cap rim) the door genuinely has nothing left to refuse. But U2
//!    did not delete the pushforward: it moved it beside the image as
//!    the AUTHORITY record, and a DECLARED locus still owes the
//!    transport. So the same `what` is raised from the other arm, and
//!    the table distinguishes them.
//!
//!    An intermediate state of that unit read `ReanchorOffCarrier`
//!    here and published it as a moved verdict. That was reading a
//!    bug — the offset lane was dropping the declaration, so nothing
//!    was left to ask the question of — and the correction is recorded
//!    at `the_not_a_rigid_translation_door_is_unreachable_at_rest`
//!    rather than quietly reverted.
//! 2. **the opened rim is an annulus on every solid of revolution**,
//!    as it always was on a box.
//!    `the_opened_rim_is_an_annulus_on_every_revolve` sweeps five wall
//!    thicknesses, mouth radii at two scales (41–47 mm and 1 m) and
//!    three chord budgets, and every one comes back one rim face, one
//!    ring, genus 0, meshing, at the closed-form volume;
//!    `the_annular_mouth_opens_to_two_disjoint_rims` pins the other
//!    shape of the same class — a revolved TUBE's mouth, ONE face on a
//!    body with no axis apex, whose correct rim is TWO disjoint annuli
//!    and therefore a face SPLIT.
//!
//!    The class was *a designated face whose cavity counterpart's
//!    boundary cannot become an interior-disjoint RING of it*, and on
//!    a revolved cap it could not because the cap arrives carrying the
//!    REVOLVE's seam — an axis apex both half-discs own, or a radial
//!    slit the annulus walks twice. That is a fact about the sweep and
//!    not about the mouth, and `shell_open` now retires it through the
//!    Euler doors before the glue. The invariant is also stated at
//!    rest: tier 3's check 9 refuses a ring standing on its own face's
//!    outer loop.
//!
//! **Two claims this file used to make and no longer does**, both
//! retracted on measurement rather than on argument: that the tangent
//! row's door could not be separated from tangency (`lifted_dome`
//! separates it), and that the opened rim's discriminator was "one
//! face versus two half-discs on a chart" (the revolved tube falsifies
//! it). The fixtures that did it were contributed by the review of
//! #1078 and are merged beside this file as
//! `verbs_teapot_r1_probes.rs` and `verbs_teapot_r2_probes.rs`.
//!
//! The junction table (finding 1) is a PLANTED RED in the direction of
//! the fix: each refusal is asserted by its exact variant and, at the
//! second door, its exact `what`. A door that grows any of those cases
//! fails here and sends the reader to the retire note beside the
//! assertion. The rim rows (finding 2) are the opposite — they were
//! planted red and have FLIPPED, so they now pin what the fixed
//! surgery builds, closed forms included.
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

/// The same barrel bulged about a centre OFF the axis: the wall is a
/// TORUS rather than a sphere zone, and that is the whole difference.
/// Its two junction stations and its 5/64 radius are the barrel's own;
/// only the centre moved, from `(0, 1/16)` to the other point on the
/// chord's perpendicular bisector.
fn torus_barrel(tol: Tol) -> Body<f64> {
    revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(3.0 / 64.0, 0.0), tol)
            .expect("base")
            .arc_to(
                Center {
                    c: Point2::new(6.0 / 64.0, 1.0 / 16.0),
                    winding: ArcSweep::Cw,
                    p: Point2::new(3.0 / 64.0, TOP),
                },
                tol,
            )
            .expect("a belly about a centre off the axis is a torus")
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
/// translation" `what` was raised on before U2.
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

/// How many of `body`'s edges carry a DECLARED locus — a sketch entity
/// under a sweep map recorded as the authority (U2 Q3), which since the
/// collapse is where the 3-space pushforward lives and therefore what
/// the offset door still has to carry.
fn declared_descriptions(body: &Body<f64>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(pncad::topo::CurveGeom::certified)
                .is_some_and(|c| c.authority().is_declared())
        })
        .count()
}

/// **Which ARM of the second door answers — and, since #1081's PR-2b,
/// whether either of these bodies still reaches that door at all**
/// (PCURVE P-1b, re-derived at the merge).
///
/// The row's own claim, unchanged: an edge of a body AT REST always
/// says which chart it lies in, so the SCAFFOLDING arm the
/// *"not a rigid translation"* door used to hang off cannot be
/// entered; and these two bodies DO carry declared loci, which is what
/// keeps the obstruction reachable through the authority record
/// instead. Both halves are asserted on the fixtures rather than
/// reasoned about, and both still hold — they are facts about the
/// bodies, not about which door offsets them.
///
/// **What moved is which door they reach.** `shell` now hands a body
/// of REVOLUTION to the simultaneous axial door, and neither of these
/// gets as far as a carrier lane:
///
/// - the LIFTED DOME hollows. Its `cylinder ∩ sphere` junction is
///   transversal and coaxial, so the meridian solve answers it — which
///   RETIRES this fixture's old job as the tangency discriminator by
///   answering the question it was asked to separate.
/// - the TANGENT bullet refuses at the corner's own transversality
///   meter (`TogetherAxialCorner`), which is a statement about the
///   angle between the two surfaces rather than about a lane. The
///   discrimination the old pair of rows made — same surfaces, same
///   authoring route, different answer — is now made by these two
///   bodies against each other on the SAME door.
///
/// The declared arm is still real and still reachable: a body outside
/// the axial kinds keeps the per-face door and everything it refuses.
/// This row no longer claims to be where that is measured.
#[test]
fn the_not_a_rigid_translation_door_is_unreachable_at_rest() {
    let tol = Tol::witness();
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
             SCAFFOLD arm the door used to hang off cannot be entered"
        );
        assert!(
            declared_descriptions(&body) > 0,
            "{what}: and these bodies do carry declared loci — which is what keeps the \
             obstruction reachable through the authority record instead"
        );
    }
    // The tangent one refuses at the CORNER; the non-tangent one
    // hollows. Same surfaces, same authoring route, and the angle
    // between them is the whole difference.
    let e = pncad::topo::shell(&bullet(tol), 1.0 / 128.0, FIT_TOL, band(tol), tol)
        .expect_err("a tangent junction has no transversal corner to solve");
    assert_eq!(
        offset_refusal(&e),
        "TogetherAxialCorner",
        "the tangent bullet refuses at the corner it is about, not at a carrier lane"
    );
    pncad::topo::shell(&lifted_dome(tol), 1.0 / 128.0, FIT_TOL, band(tol), tol)
        .expect("the non-tangent dome's junction is transversal, so it hollows");
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
            //
            // **Two arms raise that statement, and they are reported
            // apart** (PCURVE P-1b). The pushforward that owes the
            // transport used to BE the description and now sits in the
            // authority record beside a chart image; both homes still
            // owe it. Which home an edge's payload sits in is exactly
            // what the narrowed retirement is about, so the rows must
            // not be able to confuse them.
            //
            // Keyed on the opening NOUN, because that is the whole
            // discriminator and it is the half of the sentence that
            // cannot drift without the finding itself changing;
            // matching the full literal would red this row on a rewrap
            // of the source's line continuations, which is not a
            // finding.
            ReplaceFaceError::CarrierLaneUnsupported { what, .. } => {
                let arm = if what.starts_with("a mapped description") {
                    "scaffold"
                } else if what.starts_with("a declared chart image") {
                    "declared"
                } else {
                    panic!(
                        "this door's OTHER `what`s (a carrier that is neither a line nor \
                         a circle; a rotation-family trajectory) would be a different \
                         finding: {what}"
                    )
                };
                assert!(
                    what.contains("is not a rigid translation"),
                    "both arms make the same statement about the neighbouring SURFACE, \
                     not about how the meridian was authored: {what}"
                );
                format!("CarrierLaneUnsupported({arm})")
            }
            // The axial door's own two survivors, each named for what is
            // wrong with the geometry rather than for a lane it fell
            // off: a TANGENT junction has no transversal corner to
            // solve, and a TORUS is a kind the meridian reduction has
            // no curve for, so the body never reaches the door and
            // keeps the C5 table's refusal about the pair.
            ReplaceFaceError::TogetherAxialCorner { what, .. } => {
                assert!(
                    what.contains("tangent"),
                    "this door's other `what`s would be different findings, got {what}"
                );
                "TogetherAxialCorner".to_string()
            }
            ReplaceFaceError::NeighborPairUnroutable {
                kind, other_kind, ..
            } => format!("NeighborPairUnroutable({kind:?} x {other_kind:?})"),
            other => panic!("an unexpected face-offset refusal: {other}"),
        },
        other => panic!("the refusal is not the offset door's: {other}"),
    }
}

/// **What survives now, and what the variable was.**
///
/// The old form of this row read "the hollow survives exactly the
/// square junction". It did, for two waves, and the reason was never
/// curvature: `shell` replaced ONE chart at a time and re-anchored the
/// neighbours' edges on carriers that had not moved, so a junction
/// survived exactly when the neighbouring surface was invariant under
/// the moved face's own offset — a plane normal to a cylinder's axis,
/// both ways, and nothing else. A right prism on a TRIANGLE refused
/// exactly like a cone frustum, which is what ruled curvature out.
///
/// **#1081 made the offsets SIMULTANEOUS and the class is gone.**
/// PR-2a solves an all-planar corner against every moved plane at
/// once. PR-2b solves a body of REVOLUTION's corners in its meridian
/// half-plane, where a plane normal to the axis is a line, a cylinder
/// is a line, a cone is a line and a sphere is a circle — so the cone
/// frustum, the sphere zone, the quarter-revolve wedge and the lifted
/// dome all hollow, and so does the teapot's own belly.
///
/// **Two rows survive, and each names a different reason** — which is
/// why this table is still worth running:
///
/// - a **TORUS** wall (the barrel bulged about a centre OFF the axis)
///   is outside the axial kinds, so the body never reaches the door
///   and keeps the C5 table's own refusal, naming the PAIR. Nothing in
///   either PR widened `intersect::route`, and this row is what says
///   so on a body rather than in a sentence.
/// - a **TANGENT** junction has no transversal corner to solve at all,
///   and the conditioning meter says so in the geometry's own terms.
///   The bullet's `cylinder ∩ sphere` is the SAME surface pair as the
///   bellied pot's foot-to-belly junction, which hollows: the variable
///   is the angle between them, and this pair of rows is the only
///   place that is measured.
///
/// The lifted dome was the discriminator for the old third door
/// (`CarrierLaneUnsupported`, about the neighbour's offset not being a
/// rigid translation). It now HOLLOWS, because that door is no longer
/// what a coaxial curved junction reaches — which retires the
/// discriminator by answering it.
#[test]
fn the_hollow_now_survives_every_axial_junction() {
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
        (
            "a right prism on a triangle (58/58/64)",
            triangular_prism(tol),
            0.02,
        ),
        // FLIPPED by #1081's PR-2b: every one of these has a CURVED
        // face at the junction, and every one of them was on the
        // refusing list until the meridian solve landed.
        ("a cone frustum between two caps", frustum(tol), t),
        ("a sphere zone between two caps", barrel(tol), t),
        ("a quarter-revolve WEDGE", wedge(tol), t),
        (
            "a dome whose centre is lifted clear of the wall's top",
            lifted_dome(tol),
            t,
        ),
    ] {
        pncad::topo::shell(&body, thickness, FIT_TOL, band(tol), tol)
            .unwrap_or_else(|e| panic!("{what} hollows, got {e}"));
    }

    for (what, body, thickness, door) in [
        (
            "a belly bulged about a centre OFF the axis: a TORUS wall",
            torus_barrel(tol),
            t,
            "NeighborPairUnroutable(Plane x Torus)",
        ),
        (
            "a hemisphere TANGENT to its cylinder",
            bullet(tol),
            t,
            // main renamed this door's payload to
            // `CarrierLaneUnsupported(declared)` while this branch was
            // open. The rename is moot for the bullet: a tangent
            // junction no longer reaches a carrier lane at all, it
            // refuses at the corner's own transversality meter. The
            // lifted dome and the quarter-revolve wedge, which main
            // still lists here, are on the hollowing list above.
            "TogetherAxialCorner",
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

/// **The opened rim on a REVOLVE is an annulus, everywhere.** The same
/// sweep the defect row ran — five wall thicknesses over a factor of
/// 5, mouth radii at two scales (41–47 mm, the small pair 14% apart,
/// and 1 m), the pot's stepped meridian against the simplest fixture
/// there is, three chord budgets over a factor of 50 — and every one
/// now comes back as ONE rim face carrying ONE ring, genus 0, and
/// triangulating at every budget. The volume is checked against the
/// closed form on the drums, so "it meshes" is not standing in for
/// "it is the right solid".
///
/// # What was wrong, and what fixed it
///
/// The class was *a designated face whose cavity counterpart's
/// boundary cannot become an interior-disjoint RING of it*, and on a
/// revolved cap it could not because the cap arrives carrying the
/// REVOLVE's seam: an axis-touching cap is two half-discs meeting at
/// the axis apex, so the counterpart's boundary reached that same apex
/// and ran back along the outer loop's own seam legs. That contact —
/// not a ring count — is what the CDT refused.
///
/// The seam is a fact about how the operand was swept, not about the
/// mouth, and `shell_open` now removes it before the glue: the chart
/// is reduced to one face with disjoint cycles on both sides
/// (`kef`/`kev`/`kemr`, no new machinery), after which the
/// counterpart's boundary IS strictly inside. The invariant is stated
/// at rest as well — tier 3's check 9 refuses any ring standing on its
/// face's outer loop — and `verbs_shell::a_ring_standing_on_its_outer_
/// loop_refuses_at_tier_3` builds the old body through the same public
/// doors to show that net firing.
#[test]
fn the_opened_rim_is_an_annulus_on_every_revolve() {
    let tol = Tol::witness();
    let cases: Vec<(String, Body<f64>, f64, Option<f64>)> =
        [1.0 / 128.0, 1.0 / 100.0, 1.0 / 64.0, 0.003, 0.0055]
            .into_iter()
            .map(|t| {
                (
                    format!("drum, t = {t}"),
                    drum(3.0 / 64.0, tol),
                    t,
                    Some(3.0 / 64.0),
                )
            })
            .chain([0.041_25, 3.0 / 64.0, 1.0].into_iter().map(|r| {
                (
                    format!("drum, mouth radius {r}"),
                    drum(r, tol),
                    1.0 / 128.0,
                    Some(r),
                )
            }))
            .chain(core::iter::once((
                "the teapot's own stepped meridian".to_string(),
                teapot_pot(tol),
                1.0 / 128.0,
                None,
            )))
            .collect();
    for (what, body, t, drum_radius) in cases {
        let chart = plane_chart_at(&body, TOP);
        assert_eq!(
            chart.len(),
            2,
            "{what}: a full revolve's cap is two half-discs"
        );
        let cup = pncad::topo::shell_open(&body, t, &chart, FIT_TOL, band(tol), tol)
            .unwrap_or_else(|e| panic!("{what}: the opened arm must build the rim, got {e}"));
        assert_eq!(
            pncad::topo::validate_geometric(&cup, tol),
            Ok(()),
            "{what}: tier 3, which now also carries the ring-vs-outer invariant"
        );
        assert_eq!(
            (rings(&cup), genus(&cup)),
            (1, 0),
            "{what}: ONE rim annulus with one ring, and a cup is genus 0"
        );
        assert_eq!(
            plane_chart_at(&cup, TOP).len(),
            1,
            "{what}: the two half-discs became one rim face"
        );
        for delta in [1e-2, 1e-3, 2e-4] {
            pncad::mesh::tessellate(&cup, delta, tol)
                .unwrap_or_else(|e| panic!("{what}: delta = {delta} must triangulate, got {e:?}"));
        }
        // The drums have a closed form; the stepped pot does not get a
        // number invented for it here (the scene owns its census).
        if let Some(r) = drum_radius {
            let props = pncad::topo::mass_properties(&cup, tol).expect("props");
            let want = core::f64::consts::PI * (r * r * TOP - (r - t) * (r - t) * (TOP - t));
            assert!(
                (props.volume - want).abs() <= 1e-9 + props.volume_pad,
                "{what}: cup volume {} (pad {}), want {want}",
                props.volume,
                props.volume_pad
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
/// **One of NINE copies of this helper across five crates (#1123).**
/// `demos/tour` is a separate workspace and an integration test cannot
/// import a binary's module, so no existing home covers them all; the
/// issue carries the list and the shared-test-support fix.
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
/// **One of NINE copies of this helper across five crates (#1123).**
/// `demos/tour` is a separate workspace and an integration test cannot
/// import a binary's module, so no existing home covers them all; the
/// issue carries the list and the shared-test-support fix.
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

/// **The ANNULAR mouth: two disjoint rims, not one ring.**
///
/// This row's fixture is the one that falsified the first reading of
/// the class: a revolved TUBE's meridian is a closed off-axis loop, so
/// it closes its own seam and the mouth chart is exactly ONE face,
/// with no axis apex anywhere on the body — and the rim was wrong here
/// too, in a different shape (genus 2, one ring, untessellatable).
///
/// The correct rim here is not a ring at all but TWO DISJOINT ANNULI —
/// `[ri, ri+t]` and `[ro-t, ro]` — which is a face SPLIT, and it is
/// built: the counterpart's hole is promoted to its own rim face
/// before the glue (`mfkrh`) and takes the designated face's matching
/// hole with it after (`ring_move`). Both are existing doors; the
/// surgery gained no new machinery. What the operand contributed was
/// the seam again, in its other form — the annulus arrives SLIT along
/// a radial edge its own loop walks twice — and `kemr` retires that
/// before the glue for the same reason `kef`/`kev` retire the axis
/// apex above.
///
/// (`verbs_teapot_r2_probes::r2_revolved_tube_separates_seam_from_axis`
/// and `r2_annular_mouth_anatomy` are where the wrong shape was first
/// measured; this row is where the right one is pinned.)
#[test]
fn the_annular_mouth_opens_to_two_disjoint_rims() {
    let tol = Tol::witness();
    let (ri, ro, h, t) = (0.30, 0.50, 0.40, 0.05);
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
    let cup = pncad::topo::shell_open(&body, t, &chart, FIT_TOL, band(tol), tol)
        .expect("the annular mouth opens");
    assert_eq!(
        pncad::topo::validate_geometric(&cup, tol),
        Ok(()),
        "tiers 1-3, the third of which now carries the ring-vs-outer invariant"
    );
    assert_eq!(
        (rings(&cup), genus(&cup)),
        (2, 1),
        "TWO rim annuli, one ring each; the bore runs through, so the cup is genus 1"
    );
    assert_eq!(
        plane_chart_at(&cup, h).len(),
        2,
        "the mouth plane is worn by two faces — the SPLIT, which is the finding"
    );
    pncad::mesh::tessellate(&cup, 1e-3, tol).expect("and it triangulates");
    let props = pncad::topo::mass_properties(&cup, tol).expect("props");
    let want = core::f64::consts::PI
        * ((ro * ro - ri * ri) * h - ((ro - t).powi(2) - (ri + t).powi(2)) * (h - t));
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "annular cup volume {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
}
