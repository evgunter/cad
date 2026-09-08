//! **The teapot** — `shell`'s designated demo, and what it measured.
//!
//! The register has named this scene since 2026-08-09: *"the Utah
//! teapot is this verb's designated demo — a vessel is a shelled
//! revolve; the demo queues behind the verb."* The verb landed at
//! #1048. This is the demo, and its job is not to look like a teapot:
//! it is to be the first thing outside `shell`'s own acceptance corpus
//! that asks the verb for a real part and to report, in numbers, what
//! came back.
//!
//! Four bodies, and they are FOUR SOLIDS rather than one. That is the
//! honest exit shape (the lily precedent), and everything below says
//! which parts of it are the kernel's answer and which are the
//! modeller's:
//!
//! - **the pot** — one `revolve` of one meridian, hollowed by `shell`
//!   into a wall and a cavity in one solid. Drawn see-through, because
//!   a cavity is invisible in an opaque render at every camera (the
//!   hollow ring's founding reason, on this verb's own shape).
//! - **the lid** — a second `revolve`, its knob's top rim rolled
//!   through the one-edge annulus band. Rendered LIFTED above the
//!   mouth: an exploded view. No mate is authored and none is implied
//!   — declared contact is M9's territory, and a scene that faked one
//!   here would be claiming a certification nothing issued.
//! - **the spout** — a cone-frustum tube, built about its own axis and
//!   placed by `Node::Transform`.
//! - **the handle** — one `Node::Tube` window, its two roots
//!   driven through the belly wall — 11.2 mm past it, measured, which
//!   is what makes the union a real request and is also why a teapot
//!   built this way would leak (see `HANDLE_OVER`).
//!
//! # Findings this scene records (the demo-purpose rule)
//!
//! 1. **`shell`'s sealed arm survived exactly ONE junction shape, it
//!    was never about curvature, and it is now REPAIRED.** The verb
//!    replaced one chart at a time and re-anchored the neighbours'
//!    edges on carriers that had not moved yet, so a junction survived
//!    exactly when the neighbouring surface was invariant under the
//!    moved face's own offset motion: a plane normal to a cylinder's
//!    axis, both ways, and nothing else. A right prism on a TRIANGLE
//!    refused the same way as a cone frustum, which is what ruled
//!    curvature out.
//!
//!    **#1081 made the offsets SIMULTANEOUS**, in two halves. PR-2a
//!    solves an all-planar body's corner as `nᵢ·x = cᵢ` over every
//!    moved plane meeting it, so the hexagon, the bevel, the kite and
//!    the triangular prism hollow. PR-2b solves a body of REVOLUTION's
//!    corners in its own meridian half-plane, where a plane normal to
//!    the axis is a line, a cylinder is a line, a cone is a line and a
//!    sphere is a circle — so a corner is one line/line or line/circle
//!    meeting, closed form, with every further surface verified against
//!    it and the seam's azimuth carried as the conventional datum it
//!    is. **That is what un-squared this pot.** Its belly was a SPHERE
//!    ZONE and therefore on the curved side, and the meridian this
//!    scene ships is now the one the model always wanted: base, foot,
//!    ONE ARC, mouth. The three squared segments — shoulder, belly,
//!    shoulder — are gone, and so is the wall that pinned their
//!    refusal.
//!
//!    **The frontier moved rather than vanished**, and the scene's
//!    wall 1 moved with it — twice, and it is now gone. Pushing the
//!    belly's arc centre OFF the axis makes its wall a TORUS, which
//!    the meridian reduction did not know; the body fell to the
//!    per-chart loop and the C5 table refused its plane×torus pair.
//!    The reduction knows the kind now — a coaxial torus's meridian is
//!    a circle centred `(R, h_c)`, the sphere's circle with one more
//!    number — so that pot hollows and the probe below asserts it
//!    rather than pinning a refusal. A curved
//!    junction can also be TANGENT, and a tangent junction has no
//!    transversal corner to solve at all: the conditioning meter says
//!    so in the geometry's own terms, which is why the bullet still
//!    refuses where the pot's own foot-to-belly junction — the same
//!    surface pair — does not. `tests/verbs_teapot.rs` is that table,
//!    its sweep, and the sweep's stated blind spot. None of this was a
//!    gap the verb announced: `shell`'s acceptance corpus is a box, a
//!    cylinder between two caps and a tube between two caps — every
//!    fixture in the surviving class, and the class was never named.
//! 2. **The OPENED arm was wrong on every solid of revolution, and is
//!    the fixed thing this scene now ships.** This scene was specified
//!    to be `shell_open`'s first consumer past acceptance, opening the
//!    pot at its mouth so the wall's thickness shows as an annular
//!    rim. What it found was a body that passed tiers 1, 2 and 3 while
//!    each designated half-disc carried a RING that was its own cavity
//!    counterpart's boundary: reaching the axis apex the outer loop
//!    already owned, running back along that loop's seam legs. Genus 1
//!    where `topo::shell`'s docs say *"one opening gives a cup, which
//!    is genus 0"*, and the CDT refused an insertion on a mouth
//!    half-disc. That was wall 2 (#1082); the pot shipped sealed and
//!    this teapot had no opening.
//!
//!    **The mechanism was NOT "a revolve's cap is two half-discs".**
//!    The adopted review fixtures falsified that: a revolved TUBE's
//!    mouth is ONE face and was wrong too, and a partial revolve's cap
//!    is one face and touches the axis. The class is *a designated
//!    face whose cavity counterpart's boundary cannot become an
//!    interior-disjoint RING of it*, and what put a revolved cap out
//!    of reach was the REVOLVE's SEAM — an axis apex two half-discs
//!    share, or a radial slit an annular cap's loop walks twice. That
//!    is a fact about the sweep and not about the mouth, so
//!    `shell_open` retires it before the glue, through the Euler doors
//!    alone; on an annular cap the rim it then builds is TWO DISJOINT
//!    ANNULI, a face split, which the same doors express. The
//!    invariant is stated at rest as well: tier 3 refuses a ring
//!    standing on its own face's outer loop.
//!
//!    Nor was the path unvisited before:
//!    `offd2_r1_probes::probe_opened_vessel_cup` already opened a
//!    revolved vessel and checked only the things that were right —
//!    tier 3, the shell count, the volume — never the rings, the genus
//!    or the mesh. That is the transferable lesson, and that probe now
//!    checks all three. **The pot ships OPENED**, one annular rim,
//!    genus 0, meshing — and it also LEAVES AS STEP, which the sealed
//!    two-shell body could not.
//! 3. **A steam vent is what makes the lid's knob filletable.** The
//!    one-edge annulus band carves a CLOSED latitude rim, and a full
//!    revolve mints one only from an ANNULAR profile; a profile that
//!    touches the axis mints half-walls whose rims are two open arcs.
//!    So the lid is bored — a vent through the finial, which is a real
//!    teapot's answer as well as the kernel's. What is measured HERE is
//!    the positive half: the bored lid's knob rim is a closed edge and
//!    it rolls, with the band's census and its two tangency lines
//!    checked below. The negative half is measured ELSEWHERE and cited
//!    rather than re-asserted — `verbs_arms1_r1_probes::the_unbored_
//!    hemisphere_equator_refuses_typed` is the axis-touching profile's
//!    own pin, and the register's ARMS-1 row states the bound. The
//!    scene is where a PART met it while trying to be a part: the pot
//!    touches the axis at both ends, so nothing on it is a candidate
//!    and the lid had to be bored to have one.
//! 4. **The teapot is four solids because the operand gate has no arm
//!    for either join.** handle ∪ pot is torus × cylinder; spout ∪ pot
//!    is cone × plane. Both refuse `CurvedPairUnsupported`, pinned in
//!    walls 2 and 3 with the payload carried verbatim into the panel's
//!    note. Read what the second one NAMES: the spout's outer cone
//!    against a PLANE of the pot — not the belly wall the spout
//!    actually pierces. The gate is pair-scoped and box-conservative,
//!    so it reports the first pair whose boxes MAY meet, and a reader
//!    who took the refusal's text for the cause would be reading the
//!    wrong pair (the wall-7 lesson). The schedule is the banked
//!    germ-chord lanes and #1057's two C5 arms. **Both refusals now
//!    arrive through the DOCUMENT**, under the same payload: each join
//!    is a `Node::Boolean` that lowers to the same kernel `union` and
//!    fails at `evaluate`, and what the note quotes is that node's own
//!    carried refusal rather than a second measurement of it.
//! 5. **A spout the shape of a spout is not authorable at all.** What
//!    a potter draws is a swept curved section — a canal or a loft
//!    along a bent spine. `sweep_body` cannot round the U-turn a real
//!    spout takes and there is no variable-section sweep, so the shape
//!    on screen is a straight cone frustum tilted into place: a spout
//!    the way a lathe would make one. Recorded as a register note
//!    rather than worked around, because a hand-built stand-in would
//!    be evidence about this file and not about the library.
//! 6. **The lid's roll takes ONE kernel call and TWO document
//!    requests, and the difference is the NAME emitter.**
//!    `fillet_edges` rolls all three rims in one request. The same
//!    three rims through `Node::Fillet` refuse `Naming(Duplicate)`,
//!    and the duplicate says which: the flange's rim and the dome's
//!    foot are the two ends of ONE meridian segment (the flange cone),
//!    so both of their bands slit THAT segment's seam meridian, and
//!    `RoleSeg::BandSlit` names a slit by *the source edge whose
//!    severed piece became it*. Two slits, one source name.
//!
//!    **The shape is "two bands slitting one meridian", not "two
//!    adjacent rims"**, and this lid is where the difference shows: a
//!    band slits exactly one of its two supports' seams, so the rims
//!    at vertices 1 and 2 collide while `{2, 3}` and `{3, 4}` —
//!    adjacent pairs both — compose in one request. Adjacency is
//!    necessary and not sufficient. `tests/teapot_document.rs` is that
//!    table, executed, beside the equality the split owes: the two
//!    requests build the kernel's one-request body, same census, same
//!    three bands bit for bit, same mass — and a different face ORDER,
//!    which is the whole of what the conversion moved.
//!
//!    The scene therefore asks TWICE — the flange's rim, then the
//!    dome's foot and the knob's top against the carried names — which
//!    is what a user would have to do. The one-request refusal is
//!    ATTEMPTED live in [`per_rim_answers`] and pinned there, so the
//!    day the vocabulary grows the discriminator this wants — one on
//!    `BandSlit` saying WHICH band slit the edge, the way `BandTrim`
//!    already carries its `RimSupport` — the scene goes red and says
//!    to go back to one request and re-cut the tess-budget baseline
//!    back with it.
//!
//! # What this scene deliberately does NOT do
//!
//! No kernel change, no route widened, no gate softened. Every one of
//! the six findings above is a live probe or an executed table, and
//! each carries the sentence that retires it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use pncad::document::{
    BooleanOp, CancelToken, Datum, Dimension, Doc, DocEdit, EvalOptions, Evaluation, Expr,
    LoopProgram, Node, NodeErrorKind, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    RecipeNodeId, TubeWindow, ValuePayload, apply, evaluate,
};
use pncad::geom::{Curve3, Surface};
use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::prelude::query;
use pncad::prelude::{
    EntityKind, NamePat, ProfileEdgeRef, ProfileVertexRef, RoleSeg, SegPat, SegTag, Selector,
    StableName,
};
use pncad::profile::ArcSweep;
use pncad::select::{edge_name, face_carrier_kind, face_frame, select, vertex_position};
use pncad::topo::{Body, BooleanError, Operand};

use crate::{SceneBody, Stop, View};

// ---------------------------------------------------------------------
// The vessel's meridian. Every station is a dyadic rational in meters,
// so the profile's own arithmetic is exact and the closed forms below
// are compared against numbers no rounding entered.
// ---------------------------------------------------------------------

/// The foot's radius — the cylinder the pot stands on.
const R_FOOT: f64 = 4.0 / 64.0;
/// The belly's sphere radius — the pot's widest wall, at `Y_BELLY_C`.
const R_BELLY: f64 = 5.0 / 64.0;
/// The mouth's radius, which the lid's rim reads.
const R_NECK: f64 = 3.0 / 64.0;
/// Where the foot ends and the belly's arc begins.
const Y_FOOT: f64 = 1.0 / 64.0;
/// The belly sphere's centre, on the axis. The two junction stations
/// are the sphere's own 3-4-5 points: `(4/64, 1/64)` at the foot and
/// `(3/64, 8/64)` at the mouth, both with an exactly zero residual.
const Y_BELLY_C: f64 = 4.0 / 64.0;
/// The mouth's plane.
const Y_MOUTH: f64 = 8.0 / 64.0;

/// The wall thickness. A tenth of the belly's radius, and an eighth of
/// the narrowest wall the shell has to survive (the neck's), so every
/// per-face reach margin below is definite by a wide margin rather
/// than by a hair.
const WALL: f64 = 1.0 / 128.0;

// ---------------------------------------------------------------------
// The lid: a second solid of revolution, lifted.
// ---------------------------------------------------------------------

/// How far above the mouth the lid renders — the exploded-view gap.
/// No mate is authored and none is implied; see the module docs.
const LIFT: f64 = 1.0 / 32.0;
/// The lid's underside plane.
const LID_BASE: f64 = Y_MOUTH + LIFT;
/// The flange's outer radius — the lid overhangs its mouth. With the
/// dome's foot at [`R_NECK`], the skirt between them is a cone of
/// `Δr = −2/256` over `Δy = +6/256`.
///
/// **That slope is a CONVEXITY choice.** The dome's foot is the
/// sphere's 5-12-13 point, where the meridian's own slope is
/// `|dy/dr| = 12/5 = 2.4`. A cone shallower than that meets the sphere
/// in a VALLEY — a concave rim, whose fillet band ADDS material (the
/// closed-rim carve builds either side) — and a lid whose skirt fills
/// into its own dome is not this lid. At `|dy/dr| = 3` the junction is
/// convex and all three rims roll outward, the way a lid's skirt
/// should. (A 45° skirt, `Δr = −4` over `Δy = +4`, authors and builds;
/// the variant that keeps the dome's 5-12-13 junctions exact puts
/// `DOME_C` at `LID_BASE − 1/256`, and against THIS centre it refuses
/// one door earlier, at the arc's own equidistance check.)
const R_FLANGE: f64 = 14.0 / 256.0;
/// Where the conical flange ends and the dome's sphere begins.
const Y_FLANGE: f64 = LID_BASE + 6.0 / 256.0;
/// The dome's sphere radius, and its centre's station: the 5-12-13
/// triple. What that buys is not that any of these decimals is a
/// binary-exact float — 3/64 and 13/256 are, 0.6 and 0.8 would not be
/// — but that the RESIDUALS are exactly zero: `|c − p|² − r²` for the
/// rim and for the knob's foot evaluate to 0.0 in f64, so the arc door
/// has nothing to round when it checks equidistance.
const DOME_R: f64 = 13.0 / 256.0;
/// The dome sphere's centre, one unit ABOVE the lid's underside —
/// the flange lifts the dome's foot, and the centre with it.
const DOME_C: f64 = LID_BASE + 1.0 / 256.0;
/// The knob's radius — the 5-12-13 point of the dome circle.
const R_KNOB: f64 = 5.0 / 256.0;
/// Where the dome ends and the knob's wall begins.
const Y_KNOB: f64 = LID_BASE + 13.0 / 256.0;
/// The knob's top.
const Y_TOP: f64 = LID_BASE + 18.0 / 256.0;
/// The steam vent bored through the finial — which is also what makes
/// the lid's profile ANNULAR, and therefore what makes its latitude
/// rims closed edges. See the module docs' finding 3.
const R_VENT: f64 = 1.0 / 256.0;
/// The roll, one radius for all three of the lid's rims — per
/// REQUEST rather than per edge, which is what lets the two requests
/// the naming gap forces (the module docs' sixth finding) be one
/// parameter.
const ROLL: f64 = 2.0 / 256.0;

// ---------------------------------------------------------------------
// The spout and the handle.
// ---------------------------------------------------------------------

/// The spout's length along its own axis.
const SPOUT_LEN: f64 = 8.0 / 64.0;
/// The spout's outer radius at the root, and at the tip.
const SPOUT_R0: f64 = 6.0 / 256.0;
const SPOUT_R1: f64 = 3.0 / 256.0;
/// The spout's wall.
const SPOUT_WALL: f64 = 1.0 / 256.0;
/// The spout's root, inside the belly.
const SPOUT_ROOT: Point3<f64> = Point3 {
    x: -1.0 / 32.0,
    y: 3.0 / 64.0,
    z: 0.0,
};
/// The spout's axis: the 3-4-5 direction. 0.6 and 0.8 are NOT
/// binary-exact floats; what is exact is the residual — the rotation
/// matrix built from them satisfies `cᵢ·cⱼ − δᵢⱼ == 0.0` in f64, so
/// `transform_rigid`'s orthonormality decide has nothing to round.
const SPOUT_DIR: Vec3<f64> = Vec3 {
    x: -0.8,
    y: 0.6,
    z: 0.0,
};

/// The handle's spine radius: half the chord between its two roots on
/// the belly wall, so the unextended window is exactly a semicircle and
/// the handle stands one radius clear of the pot at its widest. The
/// roots span 46.875 mm of the belly's 78.125 mm of height.
const HANDLE_R: f64 = 6.0 / 256.0;
/// The handle's tube radius.
const HANDLE_TUBE: f64 = 1.0 / 128.0;
/// The handle's spine centre, ON the belly wall at the belly's own
/// mid-height — so the semicircle's two ends land on the wall and its
/// far side stands `HANDLE_R` proud of it.
const HANDLE_C: Point3<f64> = Point3 {
    x: R_BELLY,
    y: Y_BELLY_C,
    z: 0.0,
};
/// How far past the semicircle each end of the handle runs, in radians
/// of its own spine, so that each root PENETRATES the belly rather than
/// touching it — which is what makes the union attempted below a real
/// request rather than a tangency.
///
/// **It penetrates all the way through, and the geometry says it must.**
/// The spine's centre sits ON the outer wall, so at the semicircle's own
/// ends the tube already reaches `HANDLE_TUBE` inward — and this tube's
/// radius IS the wall thickness, so the cap is flush with the cavity at
/// zero overshoot and inside it at any positive one. At 0.5 rad the
/// deepest material stands `HANDLE_R·sin(0.5) + HANDLE_TUBE` = 19.0 mm
/// below the outer wall, which is 11.2 mm into the tea. Nothing
/// asserted here depends on that (the union refuses at the operand
/// gate, before any intersection work), and the scene keeps the
/// overshoot rather than thinning the handle to hide it — but a teapot
/// built this way would leak, and re-cutting it is the FIRST thing the
/// wall-3 retire note asks for if the union ever composes.
const HANDLE_OVER: f64 = 0.5;

/// The scene's chord budget.
const DELTA: f64 = 2e-4;

// ---------------------------------------------------------------------
// Construction — ONE recipe document, and the scene's bodies are its
// values. Every station below is still the `const` it was; what the
// document changes is the SEAT: a meridian is a `LoopProgram` over
// `Expr::literal`, the mouth and the lid's rims are named by ROLE
// rather than found by a numeric scan, and the two unions the scene
// cannot compose are nodes that refuse at `evaluate`.
// ---------------------------------------------------------------------

/// A length in the canonical metres this document is authored in.
fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a finite length")
}

/// A dimensionless component — the spelling a direction takes.
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("a finite scalar")
}

/// An angle in radians, the unit this scene's turns are written in.
fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("a finite angle")
}

/// One authored point of a meridian, in the sketch's own coordinates.
fn lpt(x: f64, y: f64) -> [Expr; 2] {
    [len(x), len(y)]
}

/// A straight meridian step to `(x, y)`.
fn line_to(x: f64, y: f64) -> ProgramStep {
    ProgramStep::LineTo(ProgramTarget::Point(lpt(x, y)))
}

/// A meridian arc about `(cx, cy)` to `(x, y)`.
fn arc_to(cx: f64, cy: f64, winding: ArcSweep, x: f64, y: f64) -> ProgramStep {
    ProgramStep::ArcTo(ProgramArcData::Center {
        c: lpt(cx, cy),
        winding,
        target: ProgramTarget::Point(lpt(x, y)),
    })
}

/// The `[0, π)` face swept from meridian segment `seg` of `node`.
fn band(node: RecipeNodeId, seg: u32) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Band(ProfileEdgeRef {
            loop_index: 0,
            segment: seg,
        })],
    }
}

/// The `[π, 2π)` face swept from meridian segment `seg` of `node`.
fn band_pi(node: RecipeNodeId, seg: u32) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::BandPi(ProfileEdgeRef {
            loop_index: 0,
            segment: seg,
        })],
    }
}

/// The closed latitude rim swept from meridian VERTEX `vertex` of
/// `node` — the edge between the bands of segments `vertex − 1` and
/// `vertex`.
fn band_rim(node: RecipeNodeId, vertex: u32) -> StableName {
    StableName {
        kind: EntityKind::Edge,
        node,
        path: vec![RoleSeg::BandRim(ProfileVertexRef {
            loop_index: 0,
            vertex,
        })],
    }
}

/// A name for an entity `node` carried through unchanged from its
/// target — what a survivor of a blend is called one op later.
fn carried(node: RecipeNodeId, inner: StableName) -> StableName {
    StableName {
        kind: inner.kind,
        node,
        path: vec![RoleSeg::FromTarget(Box::new(inner))],
    }
}

/// The meridian VERTEX itself, on the seam — the point a `band_rim`'s
/// circle passes through, and what ties a rim's ROLE to the station
/// and radius the scene authored it at.
fn meridian_vertex(node: RecipeNodeId, vertex: u32) -> StableName {
    StableName {
        kind: EntityKind::Vertex,
        node,
        path: vec![RoleSeg::MeridianVertex(
            pncad::prelude::MeridianEnd::Seam,
            ProfileVertexRef {
                loop_index: 0,
                vertex,
            },
        )],
    }
}

/// **The vessel's meridian**: base disc, foot, belly, mouth disc —
/// the shoulders and the belly are ONE ARC about a centre on the axis,
/// which makes the wall a SPHERE ZONE and the pot the shape a potter
/// would throw.
///
/// This is the meridian the model always wanted. Until #1081's PR-2b it
/// was wall 1, pinned by its refusal: `shell` moved one chart at a time
/// and a corner where a plane meets a sphere is not where transporting
/// it under either one alone puts it. The simultaneous door solves that
/// corner against both surfaces at once, so the arc ships.
///
/// Every station is a dyadic rational and the sphere is the 3-4-5
/// triple twice over: radius `5/64` about `(0, 4/64)`, meeting the foot
/// cylinder at `(4/64, 1/64)` and the mouth at `(3/64, 8/64)`, so both
/// junctions' residuals `|c − p|² − r²` are exactly `0.0` in f64 and
/// the closed forms below compare against numbers no rounding entered.
///
/// Station for station the corpus document `vessel`'s meridian
/// (`crates/editor-core/tests/corpus/vessel.rs`), which is the
/// document spelling of this shape.
fn vessel_meridian() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        line_to(R_FOOT, 0.0),
        line_to(R_FOOT, Y_FOOT),
        arc_to(0.0, Y_BELLY_C, ArcSweep::Ccw, R_NECK, Y_MOUTH),
        line_to(0.0, Y_MOUTH),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// The spout meridian's own segment indices, in program order: the
/// root annulus, the outer cone, the TIP annulus, the bore.
const SEG_SPOUT_ROOT: u32 = 0;
/// The tip annulus's segment — the face the placement's rotation moves.
const SEG_SPOUT_TIP: u32 = 2;

/// The mouth disc's segment index in [`vessel_meridian`]'s program
/// order — base disc, foot, belly, MOUTH, axis chord.
const SEG_MOUTH: u32 = 3;

/// The mouth disc's segment index READ OFF the program: a meridian's
/// segments are its steps after the opening `At`, so the mouth disc is
/// the step that runs to the axis at the mouth's own station — and it
/// has to be the only one, or the index would be a choice rather than
/// the program's answer.
fn mouth_segment(program: &LoopProgram) -> u32 {
    let LoopProgram::Chain(steps) = program else {
        panic!("the meridian is a chain program");
    };
    let want = line_to(0.0, Y_MOUTH);
    let hits: Vec<usize> = steps
        .iter()
        .skip(1)
        .enumerate()
        .filter(|(_, s)| **s == want)
        .map(|(i, _)| i)
        .collect();
    match hits[..] {
        [only] => u32::try_from(only).expect("a meridian has few segments"),
        ref many => panic!("the mouth disc is ONE meridian step, got {many:?}"),
    }
}

/// **The wall-1 meridian, re-planted one step out**: the SAME two
/// junction stations and the SAME `5/64` radius, about the OTHER
/// centre on their perpendicular bisector — `(7/64, 5/64)`, off the
/// axis, so the revolve mints a TORUS. Both residuals are still
/// exactly zero (3-4-5 twice again).
fn torus_belly_meridian() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        line_to(R_FOOT, 0.0),
        line_to(R_FOOT, Y_FOOT),
        arc_to(7.0 / 64.0, 5.0 / 64.0, ArcSweep::Cw, R_NECK, Y_MOUTH),
        line_to(0.0, Y_MOUTH),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// **The lid's meridian**: underside annulus, conical flange, dome,
/// knob wall, knob top annulus, vent bore.
///
/// The flange is what a lid has and is also what puts the CURVED-support
/// fillet family on this part: it mints two closed latitude rims whose
/// supports are a plane and a cone, and a cone and a sphere. With the
/// knob's cylinder × plane rim that is three DIFFERENT coaxial arms on
/// one body — the three `bud` records, on a part rather than on a study.
///
/// Every station stays dyadic, which is what keeps the arc door's
/// equidistance residuals exactly zero: the flange runs `14/256 → 12/256`
/// over `40/256 → 46/256`, and the dome's centre sits at `41/256` so
/// its two junctions are still the sphere's own 5-12-13 points —
/// `(12, 46)` is `(12, 5)` from the centre and `(5, 53)` is `(5, 12)`.
fn lid_meridian() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(R_VENT, LID_BASE)),
        line_to(R_FLANGE, LID_BASE),
        line_to(R_NECK, Y_FLANGE),
        arc_to(0.0, DOME_C, ArcSweep::Ccw, R_KNOB, Y_KNOB),
        line_to(R_KNOB, Y_TOP),
        line_to(R_VENT, Y_TOP),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// The lid's three rolled rims, as the meridian VERTEX each stands at
/// — the flange's rim, the dome's foot, the knob's top. A vertex `v`
/// is the start of segment `v`, so these are the stations
/// `(R_FLANGE, LID_BASE)`, `(R_NECK, Y_FLANGE)` and `(R_KNOB, Y_TOP)`.
const LID_RIMS: [(u32, f64, f64, &str); 3] = [
    (1, R_FLANGE, LID_BASE, "the flange's rim (cone x plane)"),
    (2, R_NECK, Y_FLANGE, "the dome's foot (sphere x cone)"),
    (4, R_KNOB, Y_TOP, "the knob's top (cylinder x plane)"),
];

/// **The spout's meridian**: an annular trapezoid — a cone frustum
/// with a cone frustum bored out of it, one wall thick.
fn spout_meridian() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(SPOUT_R0 - SPOUT_WALL, 0.0)),
        line_to(SPOUT_R0, 0.0),
        line_to(SPOUT_R1, SPOUT_LEN),
        line_to(SPOUT_R1 - SPOUT_WALL, SPOUT_LEN),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// **The turn that takes the spout's own `+y` axis onto
/// [`SPOUT_DIR`]**, as the document's placement vocabulary says it: a
/// rotation about `+z` by `atan2(0.8, 0.6)`.
///
/// The 3-4-5 direction is not a binary-exact float and this angle is
/// not one either — but the rotation it builds is exact all the same,
/// and the scene asserts that rather than assuming it: `cos` and `sin`
/// of this angle return `0.6` and `0.8` bit for bit, and Rodrigues
/// about a coordinate axis consumes nothing else, so every entry of
/// the matrix is one of `0.6`, `±0.8`, `0.0` and `1.0`.
fn spout_turn() -> f64 {
    (-SPOUT_DIR.x).atan2(SPOUT_DIR.y)
}

/// The scene's recipe: one document, and the node each body is read
/// out of.
struct Recipe {
    doc: Doc<ProfileProgram>,
    /// The pot's revolve — the OPERAND both hollows are taken of, and
    /// the node whose bands name the mouth.
    bellied: RecipeNodeId,
    /// The sealed hollow: a wall and a cavity in ONE solid.
    pot: RecipeNodeId,
    /// The opened hollow — the body the montage draws.
    cup: RecipeNodeId,
    /// The lid before its rims roll.
    plain_lid: RecipeNodeId,
    /// The lid with its three rims rolled — in TWO fillet requests at
    /// one radius, for the reason the sixth finding gives.
    lid: RecipeNodeId,
    /// The spout about its OWN axis — the node whose bands name the
    /// root annulus the placement is measured on.
    spout_body: RecipeNodeId,
    /// The spout, placed.
    spout: RecipeNodeId,
    /// The handle.
    handle: RecipeNodeId,
    /// The two joins the operand gate has no arm for.
    handle_union: RecipeNodeId,
    spout_union: RecipeNodeId,
}

fn insert(doc: &mut Doc<ProfileProgram>, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
    *doc = applied.doc;
    applied.record.minted.expect("insert mints an id")
}

/// One full revolve of `loop_` about the sketch frame's own `+v`,
/// which this document places on world `+y`.
fn revolved(
    doc: &mut Doc<ProfileProgram>,
    plane: RecipeNodeId,
    axis: RecipeNodeId,
    loop_: LoopProgram,
    tol: Tol,
) -> RecipeNodeId {
    let profile = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![loop_],
        }),
        tol,
    );
    insert(
        doc,
        Node::Revolve {
            profile,
            axis,
            angle: ang(TAU),
        },
        tol,
    )
}

/// **The sketch frame and the axis every meridian here turns about.**
///
/// u = +X (the radius), v = +Y (the axis), so a meridian point
/// `(x, y)` is the world point `(x, y, 0)` and the pot stands on +Y —
/// the world placement every camera, cell and budget row of this scene
/// was taken from. The axis is written in the frame's own 2-D
/// coordinates, its own +v through the origin, so it cannot leave the
/// plane the meridian is drawn on.
fn frame_and_axis(doc: &mut Doc<ProfileProgram>, tol: Tol) -> (RecipeNodeId, RecipeNodeId) {
    let plane = insert(
        doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
        tol,
    );
    let axis = insert(
        doc,
        Node::Datum(Datum::AxisInPlane {
            plane,
            origin: [len(0.0), len(0.0)],
            direction: [scl(0.0), scl(1.0)],
        }),
        tol,
    );
    (plane, axis)
}

fn build_doc(tol: Tol) -> Recipe {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("teapot", tol);
    let (plane, axis) = frame_and_axis(&mut doc, tol);

    // ---- the vessel ----
    let bellied = revolved(&mut doc, plane, axis, vessel_meridian(), tol);
    // The mouth is the two half-faces of the meridian's mouth-disc
    // segment, NAMED. A full revolve cuts each wall of an
    // axis-touching profile at the two seam meridians, so the mouth
    // disc is two half-discs on one plane; the kernel's rim surgery
    // lifts a chart as a whole and the document names both halves,
    // the `Band` half first — which is the half that carries the rim.
    let mouth = vec![band(bellied, SEG_MOUTH), band_pi(bellied, SEG_MOUTH)];
    let pot = insert(&mut doc, Node::shell(bellied, len(WALL), Vec::new()), tol);
    let cup = insert(&mut doc, Node::shell(bellied, len(WALL), mouth), tol);

    // ---- the lid ----
    let plain_lid = revolved(&mut doc, plane, axis, lid_meridian(), tol);
    // THREE rims, THREE DIFFERENT coaxial arms. The radius
    // is per REQUEST, not per edge, and each later rim's seam-piece
    // identities are re-read against the partially-carved body, so the
    // convenient spelling is the door's grain.
    //
    // Each rim is ONE name because the lid's profile is ANNULAR: it
    // touches the axis nowhere, so the full revolve mints one whole
    // wall per segment and one CLOSED latitude rim per vertex, where
    // the pot's axis-touching profile mints half-walls and a `Band` /
    // `BandPi` pair.
    //
    // GAP (the module docs' sixth finding): the KERNEL door rolls all
    // three rims in one request and the document layer cannot NAME
    // that output. The flange's rim and the dome's foot stand at the
    // two ends of the flange cone, so both bands slit THAT segment's
    // seam meridian, and `RoleSeg::BandSlit` carries only the source
    // edge it severed — two slits, one name, `Naming(Duplicate)`. The
    // test is which meridian a band slits, not whether two rims are
    // adjacent: `{2, 3}` and `{3, 4}` are adjacent and compose in one
    // request (`tests/teapot_document.rs`). So the roll is TWO
    // requests at one radius, which is what a user would have to
    // write; the second names its rims as the first carried them
    // through, since a survivor is `FromTarget` of the name it had.
    let first = insert(
        &mut doc,
        Node::fillet(
            plain_lid,
            len(ROLL),
            vec![band_rim(plain_lid, LID_RIMS[0].0)],
        ),
        tol,
    );
    let lid = insert(
        &mut doc,
        Node::fillet(
            first,
            len(ROLL),
            vec![
                carried(first, band_rim(plain_lid, LID_RIMS[1].0)),
                carried(first, band_rim(plain_lid, LID_RIMS[2].0)),
            ],
        ),
        tol,
    );

    // ---- the spout: built about its own axis, then placed ----
    let spout_body = revolved(&mut doc, plane, axis, spout_meridian(), tol);
    let spout = insert(
        &mut doc,
        Node::Transform {
            input: spout_body,
            translation: [len(SPOUT_ROOT.x), len(SPOUT_ROOT.y), len(SPOUT_ROOT.z)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(spout_turn()),
        },
        tol,
    );

    // ---- the handle ----
    let spine = insert(
        &mut doc,
        Node::Datum(Datum::Axis {
            origin: [len(HANDLE_C.x), len(HANDLE_C.y), len(HANDLE_C.z)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        }),
        tol,
    );
    let handle = insert(
        &mut doc,
        Node::Tube {
            spine,
            u_ref: [scl(1.0), scl(0.0), scl(0.0)],
            major_radius: len(HANDLE_R),
            window: TubeWindow::Arc {
                t0: ang(-(FRAC_PI_2 + HANDLE_OVER)),
                t1: ang(FRAC_PI_2 + HANDLE_OVER),
            },
            minor_radius: len(HANDLE_TUBE),
        },
        tol,
    );

    // ---- the two joins, as document requests ----
    let union_node = |doc: &mut Doc<ProfileProgram>, b| {
        insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Union,
                a: cup,
                b,
                declare: None,
            },
            tol,
        )
    };
    let handle_union = union_node(&mut doc, handle);
    let spout_union = union_node(&mut doc, spout);

    Recipe {
        doc,
        bellied,
        pot,
        cup,
        plain_lid,
        lid,
        spout_body,
        spout,
        handle,
        handle_union,
        spout_union,
    }
}

/// **Wall 1's re-planted pot, hollowed** — a PROBE, in its own
/// document.
///
/// It is the same two nodes the scene's own pot takes, over the same
/// two junction stations and the same tube radius about the OTHER
/// centre on their perpendicular bisector: what the wall pinned was a
/// refusal, and what stands here is the hollow. It gets a document of
/// its own for the reason the per-rim questions do — the gallery opens
/// the scene's recipe, and a body the scene measures rather than
/// models is not part of that recipe.
fn wall_one_pot(tol: Tol) -> Body<f64> {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("teapot-wall-1", tol);
    let (plane, axis) = frame_and_axis(&mut doc, tol);
    let belly = revolved(&mut doc, plane, axis, torus_belly_meridian(), tol);
    let hollow = insert(&mut doc, Node::shell(belly, len(WALL), Vec::new()), tol);
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    body_at(&ev, hollow)
}

/// **This scene's recipe, as a document the GUI can open** — the same
/// [`build_doc`] the stops walk, so the gallery cannot be a second
/// authoring of the scene.
///
/// Three sinks are deleted, and the deletion is what makes the file
/// draw the teapot rather than a pile: the SEALED hollow, which is the
/// same operand and wall as the cup and would render inside it, and
/// the two refusing unions, which produce no body at all and hold the
/// cup, the spout and the handle down out of the root set while they
/// stand. What is left is the four bodies the montage shows, as four
/// roots.
///
/// They interpenetrate, and the file says so: the handle's roots are
/// driven through the belly wall and the spout's root disc sits inside
/// it, which is what makes the two unions real requests. A separation
/// finding on this document is the scene's own subject arriving
/// through the checks registry.
pub fn gallery_document(tol: Tol) -> Doc<ProfileProgram> {
    let r = build_doc(tol);
    [r.handle_union, r.spout_union, r.pot]
        .into_iter()
        .fold(r.doc, |doc, id| {
            apply(&doc, &DocEdit::DeleteNode { id }, tol)
                .expect("each is a sink: deleting it drops a root and uncovers no body")
                .doc
        })
}

/// The body a node evaluated to.
fn body_at(ev: &Evaluation<f64>, id: RecipeNodeId) -> Body<f64> {
    let value = ev
        .value(id)
        .unwrap_or_else(|| panic!("node {id:?} evaluated to a body, got {}", describe(ev, id)));
    match &value.payload {
        ValuePayload::Body(b) => (**b).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

/// The faces of `node`'s output matching one role-segment pattern.
fn faces_where(ev: &Evaluation<f64>, node: RecipeNodeId, seg: SegPat) -> Vec<StableName> {
    select(
        ev,
        node,
        &Selector::of(NamePat::of_kind(EntityKind::Face).seg(seg)),
    )
}

/// **Every band face `node`'s lid carries** — the ones its own blend
/// minted, and the ones an earlier blend minted and this one carried
/// through. Two patterns because a survivor is `FromTarget` of the
/// name it had: the roll below is TWO requests, so one of its three
/// bands is a carried name and the other two are mints.
fn band_faces(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<StableName> {
    let faces = NamePat::of_kind(EntityKind::Face);
    select(
        ev,
        node,
        &Selector::any_of([
            faces.clone().seg(SegPat::tag(SegTag::BandFace)),
            faces.seg(
                SegPat::tag(SegTag::FromTarget)
                    .of([NamePat::any().seg(SegPat::tag(SegTag::BandFace))]),
            ),
        ]),
    )
}

/// **A named rim's own circle, read back OFF THE EDGE THE NAME
/// DENOTES**: its centre station and its radius.
///
/// The direction matters and is the whole strength of the row. A
/// numeric scan matched a DESCRIPTION — station and radius — and
/// answered a key; this goes the other way, from the name to the one
/// edge that carries it (`edge_name` over the body's edges is the
/// only door the façade has for that direction), and then reads the
/// circle off THAT edge's own certified carrier. So a name that
/// resolved to a different rim standing at the same station cannot
/// answer the right radius: the radius is the edge's, not a
/// neighbouring vertex's.
///
/// Three things are asserted and each is a way the tie could be
/// false: the name denotes EXACTLY ONE edge, that edge's circle is
/// centred on the axis, and the meridian VERTEX of the same profile
/// vertex lies ON that circle — which is what makes the rim and the
/// vertex two names for one place rather than two independent reads
/// that happen to agree.
fn rim_circle(
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
    body: &Body<f64>,
    vertex: u32,
) -> (f64, f64) {
    let want = band_rim(node, vertex);
    let carried: Vec<(f64, f64)> = query::all_edges(body)
        .into_iter()
        .filter(|&k| edge_name(ev, node, 0, k).ok() == Some(&want))
        .map(|k| {
            let c = body
                .get_edge(k)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(|g| g.certified())
                .expect("a rim carries a certified curve");
            match *c.carrier() {
                Curve3::Circle { center, radius, .. } => {
                    assert!(
                        center.x.abs() < 1e-12 && center.z.abs() < 1e-12,
                        "a latitude rim's circle is centred ON the axis: got {center:?}"
                    );
                    (center.y, radius)
                }
                ref other => panic!("a latitude rim's carrier is a circle, got {other:?}"),
            }
        })
        .collect();
    let [(station, radius)] = carried[..] else {
        panic!("the rim's name denotes exactly one edge, got {carried:?}");
    };
    let p = vertex_position(ev, node, &meridian_vertex(node, vertex))
        .expect("the meridian vertex's name denotes a vertex");
    assert!(
        (p.y - station).abs() < 1e-12 && (p.x.hypot(p.z) - radius).abs() < 1e-12,
        "the meridian vertex {p:?} does not stand on the circle its own rim carries \
         (station {station}, radius {radius})"
    );
    (station, radius)
}

/// A document node's answer, as one line for the panel's note — the
/// refusal's own payload rather than a sentence about it (the wall-7
/// lesson: a refusal's TEXT is not evidence of its cause; the payload
/// and the raising site are).
fn describe(ev: &Evaluation<f64>, node: RecipeNodeId) -> String {
    match ev.node_error(node) {
        None => "COMPOSED".to_string(),
        Some(e) => format!("{:?}", e.kind),
    }
}

/// **A join's outcome, as a `Result` the wall probe can read both
/// arms of.** `Ok(())` for a union that COMPOSED, the kernel's own
/// refusal unaltered for one that did not.
///
/// Both arms are reachable, which is the point: a probe whose success
/// arm cannot be reached is a probe that can never tell the scene its
/// wall is gone, and this scene's two walls are exactly the frontier
/// whose retirement has to be noticed.
fn join_outcome(ev: &Evaluation<f64>, node: RecipeNodeId) -> Result<(), &BooleanError> {
    match ev.node_error(node).map(|e| &e.kind) {
        None => Ok(()),
        Some(NodeErrorKind::Boolean(e)) => Err(e),
        Some(other) => panic!("the union refused, but not at the boolean door: {other:?}"),
    }
}

// ---------------------------------------------------------------------
// The closed forms — derived HERE from the authored stations, never
// read back from the bodies they check.
// ---------------------------------------------------------------------

/// The volume of a stack of coaxial cylindrical segments `(radius,
/// height)`.
fn stack_volume(segments: &[(f64, f64)]) -> f64 {
    PI * segments.iter().map(|(r, h)| r * r * h).sum::<f64>()
}

/// A spherical zone's own two closed forms about a sphere of radius
/// `r` centred at station `c`, between stations `y0` and `y1`:
/// `(∫π x² dy, Archimedes' 2πr·Δy)`.
fn zone(r: f64, c: f64, y0: f64, y1: f64) -> (f64, f64) {
    let (a, b) = (y0 - c, y1 - c);
    (
        PI * (r * r * (b - a) - (b * b * b - a * a * a) / 3.0),
        2.0 * PI * r * (y1 - y0),
    )
}

/// A right cone frustum's volume between radii `r0` and `r1` over
/// height `h`.
fn frustum_volume(r0: f64, r1: f64, h: f64) -> f64 {
    PI * h * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0
}

/// A right cone frustum's LATERAL area between radii `r0` and `r1`
/// over height `h`.
fn frustum_lateral(r0: f64, r1: f64, h: f64) -> f64 {
    PI * (r0 + r1) * ((r0 - r1) * (r0 - r1) + h * h).sqrt()
}

/// An annulus's area.
fn annulus(ro: f64, ri: f64) -> f64 {
    PI * (ro * ro - ri * ri)
}

/// The genus of `body` by the Euler–Poincaré identity
/// `v − e + f − r = 2(s − g)`, summed over shells.
///
/// The identity's left side is EVEN on any body the identity applies
/// to, so an odd one is not a body with a surprising genus — it is a
/// census that does not satisfy Euler–Poincaré at all, and halving it
/// would turn that into a plausible number. Checked before the divide
/// rather than after, because after is too late.
///
/// Duplicated, deliberately, in `tests/verbs_teapot.rs`: a binary's
/// module cannot be imported by an integration test, and the two
/// copies are three lines of a published identity rather than a shared
/// invariant. The tie between them is that both are checked against
/// the same measured censuses.
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
    let r: i64 = body.faces().map(|(_, x)| x.rings.len() as i64).sum();
    let chi = v - e + f - r;
    assert!(
        chi % 2 == 0,
        "v - e + f - r = {chi} is ODD, so this census does not satisfy \
         Euler-Poincare and no genus follows from it"
    );
    body.shells().count() as i64 - chi / 2
}

/// The station where the foot cylinder meets the belly sphere, at
/// inward offset `d`: the sphere shrinks concentrically and the
/// cylinder shrinks radially, so their meeting slides ALONG the
/// meridian. This is the corner the simultaneous door solves, written
/// independently here — and it is what a transported corner gets
/// wrong.
fn pot_junction(d: f64) -> f64 {
    let (rr, rf) = (R_BELLY - d, R_FOOT - d);
    Y_BELLY_C - (rr * rr - rf * rf).sqrt()
}

/// The pot's boundary radius on the belly sphere at station `y` and
/// inward offset `d`.
fn belly_radius(y: f64, d: f64) -> f64 {
    let rr = R_BELLY - d;
    (rr * rr - (y - Y_BELLY_C) * (y - Y_BELLY_C)).sqrt()
}

/// The volume the pot's boundary encloses at inward offset `d`: the
/// foot's cylinder up to the junction, then the SPHERICAL ZONE from
/// there to the mouth. `π∫ρ²dy` in two pieces, with the second one the
/// zone integral `π[R²y − y³/3]` about the sphere's own centre.
fn pot_volume(d: f64) -> f64 {
    let (rf, rr) = (R_FOOT - d, R_BELLY - d);
    let (y0, y1) = (pot_junction(d), Y_MOUTH - d);
    let zone = |y: f64| {
        let u = y - Y_BELLY_C;
        rr * rr * u - u * u * u / 3.0
    };
    PI * rf * rf * (y0 - d) + PI * (zone(y1) - zone(y0))
}

/// The area of the pot's boundary at inward offset `d`: the base cap,
/// the foot's wall, the belly zone's lateral area (`2πRh`, Archimedes)
/// and the mouth cap.
fn pot_area(d: f64) -> f64 {
    let (rf, rr) = (R_FOOT - d, R_BELLY - d);
    let (y0, y1) = (pot_junction(d), Y_MOUTH - d);
    PI * rf * rf
        + 2.0 * PI * rf * (y0 - d)
        + 2.0 * PI * rr * (y1 - y0)
        + PI * belly_radius(y1, d) * belly_radius(y1, d)
}

// ---------------------------------------------------------------------
// The scene
// ---------------------------------------------------------------------

/// **Each of the lid's three rims asked for ON ITS OWN**, in a
/// document of its own, and the answer printed.
///
/// A one-call refusal names ONE edge, and on a body where three rims
/// are new that is not enough to say which arm the door turned away —
/// so the scene asks three questions whose answers are each about one
/// rim. They are asked on a SEPARATE document because the scene's own
/// recipe is what the gallery opens, and three extra half-rolled lids
/// in it would be three bodies the scene does not model.
fn per_rim_answers(tol: Tol) -> Vec<(&'static str, String)> {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("teapot-lid-rims", tol);
    let plane = insert(
        &mut doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
        tol,
    );
    let axis = insert(
        &mut doc,
        Node::Datum(Datum::AxisInPlane {
            plane,
            origin: [len(0.0), len(0.0)],
            direction: [scl(0.0), scl(1.0)],
        }),
        tol,
    );
    let lid = revolved(&mut doc, plane, axis, lid_meridian(), tol);
    let mut asked: Vec<(&'static str, RecipeNodeId)> = LID_RIMS
        .iter()
        .map(|&(v, _, _, what)| {
            (
                what,
                insert(
                    &mut doc,
                    Node::fillet(lid, len(ROLL), vec![band_rim(lid, v)]),
                    tol,
                ),
            )
        })
        .collect();
    // And the fourth question, which is the sixth finding ATTEMPTED
    // rather than described: all three rims in ONE request.
    let together = insert(
        &mut doc,
        Node::fillet(
            lid,
            len(ROLL),
            LID_RIMS.iter().map(|&(v, ..)| band_rim(lid, v)).collect(),
        ),
        tol,
    );
    asked.push(("all three in ONE request", together));
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    // The refusal is PINNED, not merely printed: it is the shape the
    // scene's two-request grain exists for, and the day the naming
    // vocabulary grows the discriminator this asks for, this row goes
    // red and says what to do about it.
    let refusal = describe(&ev, together);
    assert!(
        refusal.starts_with("Naming(Duplicate")
            && refusal.contains("BandSlit")
            && refusal.contains("Meridian(Seam, ProfileEdgeRef { loop_index: 0, segment: 1 })"),
        "the one-request roll of all three rims refuses because the flange's rim and the \
         dome's foot slit ONE seam meridian and a `BandSlit` carries only the edge it \
         severed. It answered {refusal} instead. If it COMPOSED, the vocabulary grew the \
         discriminator: put the three rims back in one `Node::fillet`, delete this \
         probe and the sixth finding, and re-cut the tess-budget baseline BACK — the \
         lid's three `teapotlid` rows permute their triangle counts with the request \
         count. `tests/teapot_document.rs` is the table behind this one refusal"
    );
    asked
        .into_iter()
        .map(|(what, node)| (what, describe(&ev, node)))
        .collect()
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let r = build_doc(tol);
    let ev = evaluate::<f64>(
        &r.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );

    // ---- the vessel, before the wall ----
    let bellied = body_at(&ev, r.bellied);
    assert_eq!(
        (
            bellied.vertices().count(),
            bellied.edges().count(),
            bellied.faces().count(),
        ),
        (8, 14, 8),
        "four revolved meridian segments (the fifth is the axis and sweeps nothing), \
         each cut at the two seam meridians into a pair of half-walls — one arc where \
         the squared pot spent three segments on shoulder, belly, shoulder"
    );
    // The mouth disc's segment index is read off the PROGRAM rather
    // than transcribed: it is the segment whose meridian step lands on
    // the mouth plane at the axis.
    assert_eq!(
        mouth_segment(&vessel_meridian()),
        SEG_MOUTH,
        "the mouth disc is segment {SEG_MOUTH} of the meridian in program order"
    );
    // THE MOUTH, BY NAME. Two half-discs on ONE plane — the two names
    // the shell node was authored with, asserted to denote exactly
    // that: two planar faces, both on the mouth's own station.
    let mouth = [band(r.bellied, SEG_MOUTH), band_pi(r.bellied, SEG_MOUTH)];
    for name in &mouth {
        assert_eq!(
            face_carrier_kind(&ev, r.bellied, name).expect("the mouth half is named"),
            SurfaceKind::Plane,
            "a mouth half-disc's carrier is a plane"
        );
        let origin = face_frame(&ev, r.bellied, name)
            .expect("the mouth half is named")
            .origin;
        assert!(
            (origin.y - Y_MOUTH).abs() < 1e-12,
            "the mouth's halves stand on the mouth plane: got {origin:?}"
        );
    }
    // The CHART is those two faces and nothing else — the pin the
    // numeric plane scan used to carry as `mouth.len() == 2`, said in
    // both directions: the two names denote two DISTINCT faces, and
    // the operand has exactly two planar faces on the mouth's station,
    // so a designation of both is a designation of the whole chart
    // (which is what `shell_open` requires and what
    // `OpenFaceChartPartial` refuses).
    let named_mouth = bellied
        .faces()
        .filter(|&(k, _)| {
            pncad::select::face_name(&ev, r.bellied, 0, k).is_ok_and(|n| mouth.contains(n))
        })
        .count();
    let on_mouth_plane = bellied
        .faces()
        .filter(|(_, f)| {
            matches!(bellied.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - Y_MOUTH).abs() < 1e-12)
        })
        .count();
    assert_eq!(
        (named_mouth, on_mouth_plane),
        (2, 2),
        "the mouth is one PLANE worn by two half-disc faces — a full revolve's seam cut \
         — and the two names the shell node carries are exactly those two"
    );

    // ---- the gates, MEASURED off the operand before the verb runs ----
    //
    // A plane's own reach is unbounded, so its per-face collapse margin
    // is vacuous and `wall_clearance` is what stands in for it: every
    // antiparallel non-adjacent planar pair must clear 2t. The OUTWARD
    // normal is the stored one turned by the face's sense bit, which is
    // what the gate reads and what a scan trusting the surface alone
    // would get wrong: a revolve stores its caps' planes with one
    // normal and lets the face's orientation say which way each looks,
    // so a sense-blind scan finds NO antiparallel pair on this pot at
    // all. The count that stored `+y` goes in the note, measured.
    let planes: Vec<(f64, f64)> = bellied
        .faces()
        .filter_map(|(_, f)| match bellied.get_surface(f.surface) {
            Some(Surface::Plane { origin, normal, .. }) => {
                Some((origin.y, if f.sense { normal.y } else { -normal.y }))
            }
            _ => None,
        })
        .collect();
    let stored_plus = planes.len()
        - bellied
            .faces()
            .filter(|(_, f)| {
                matches!(bellied.get_surface(f.surface),
                    Some(Surface::Plane { normal, .. }) if normal.y < 0.0)
            })
            .count();
    // Not merely reported: EVERY planar face of this pot stores `+y`,
    // and that is the whole content of the sense-bit warning below. A
    // number only printed is a number nothing checks — if a revolve
    // ever stores a cap's plane the other way round, this fails here
    // and the note's sentence gets re-derived rather than silently
    // becoming a different true statement.
    assert_eq!(
        stored_plus,
        planes.len(),
        "every planar face of this pot stores a +y normal; only the face's own sense \
         bit says which way each looks"
    );
    let mut clearance = f64::INFINITY;
    for (i, a) in planes.iter().enumerate() {
        for b in &planes[i + 1..] {
            if a.1 * b.1 < 0.0 {
                clearance = clearance.min((b.0 - a.0).abs());
            }
        }
    }
    assert!(
        clearance.is_finite(),
        "this pot HAS antiparallel planar pairs — a scan that found none read the \
         stored normal without its face's sense bit"
    );
    assert!(
        clearance > 2.0 * WALL,
        "the closest antiparallel planar pair clears {clearance} m and two walls need {}",
        2.0 * WALL
    );
    let reach = bellied
        .faces()
        .filter_map(|(_, f)| match bellied.get_surface(f.surface) {
            Some(Surface::Cylinder { radius, .. }) => Some(*radius - WALL),
            _ => None,
        })
        .fold(f64::INFINITY, f64::min);
    assert!(
        reach > 0.0,
        "every cylinder's realized inner radius is positive; the tightest has {reach} m left"
    );

    // ---- the sealed hollow: the document's own value ----
    let pot = body_at(&ev, r.pot);
    let (pv, pe, pf) = (
        pot.vertices().count(),
        pot.edges().count(),
        pot.faces().count(),
    );
    assert_eq!(
        (pv, pe, pf),
        (16, 28, 16),
        "the operand's 8/14/8, twice: the cavity is the same boundary offset inward, \
         inserted whole through the shared void door"
    );
    assert_eq!(
        pot.shells().count(),
        2,
        "the outer boundary and the cavity, in ONE solid"
    );
    assert_eq!(genus(&pot), 0, "two sphere-like shells, no handles");

    // The wall as a NUMBER, against the two stacks — and the cavity's
    // capacity asked for DIRECTLY rather than inferred from a
    // difference, which is the number a potter actually wants.
    let v_out = pot_volume(0.0);
    let v_cav = pot_volume(WALL);
    let v_want = v_out - v_cav;
    let a_want = pot_area(0.0) + pot_area(WALL);
    let props = pncad::topo::mass_properties(&pot, tol).expect("the pot's props");
    assert!(
        ((props.volume - v_want) / v_want).abs() < 1e-12,
        "pot V = {} vs the wall's own closed form {v_want}",
        props.volume
    );
    assert!(
        ((props.surface_area - a_want) / a_want).abs() < 1e-12,
        "pot A = {} vs the closed form {a_want}",
        props.surface_area
    );
    assert_eq!(props.volume_pad, 0.0, "closed forms need no pad");
    let classes = pncad::topo::classify_shells(&pot, tol).expect("per-shell classification");
    let voids: Vec<_> = classes
        .iter()
        .filter(|c| c.role == pncad::topo::ShellRole::Void)
        .collect();
    assert_eq!(
        voids.len(),
        1,
        "one cavity, one Void shell — DECIDED, not declared"
    );
    assert!(
        ((voids[0].volume + v_cav) / v_cav).abs() < 1e-12,
        "the pot holds {} m³ against the cavity stack's own {v_cav}, negated by the \
         orientation convention",
        voids[0].volume
    );
    let outers: Vec<_> = classes
        .iter()
        .filter(|c| c.role == pncad::topo::ShellRole::Outer)
        .collect();
    assert_eq!(outers.len(), 1, "one outer boundary");
    assert!(
        ((outers[0].volume - v_out) / v_out).abs() < 1e-12,
        "the outer shell encloses {} m³ against the outer stack's {v_out}",
        outers[0].volume
    );
    let capacity_l = v_cav * 1000.0;

    // ---- the opened hollow: the body the scene SHIPS ----
    //
    // The SEALED census above is the sealed arm's own evidence and
    // stays: it is where the two-shell insertion, the decided Void
    // role and the capacity are read. What the montage draws is the
    // OPENED pot, because a teapot has a mouth. Both hollows are the
    // SAME operand and the SAME wall through the same node kind; what
    // separates them is the open list, which is empty for one and the
    // mouth's two names for the other.
    let cup = body_at(&ev, r.cup);

    // ---- the lid ----
    let plain_lid = body_at(&ev, r.plain_lid);
    assert_eq!(
        (
            plain_lid.vertices().count(),
            plain_lid.edges().count(),
            plain_lid.faces().count(),
        ),
        (6, 12, 6),
        "an ANNULAR profile mints ONE full wall per segment — six walls, six closed \
         latitude rims and six seam meridians — where the pot's axis-touching profile \
         mints half-walls and open arcs"
    );
    // THE THREE RIMS, BY NAME — and the name checked against the
    // DESCRIPTION a scan would have matched on, which is the station
    // and the radius the meridian authored. That equality is what
    // retires the scan: the role names the same circle, and it names
    // it through a rebuild rather than through a coordinate.
    for &(v, radius, station, what) in &LID_RIMS {
        let (y, rho) = rim_circle(&ev, r.plain_lid, &plain_lid, v);
        assert!(
            (y - station).abs() < 1e-12 && (rho - radius).abs() < 1e-12,
            "{what} is the rim named at meridian vertex {v}, and its circle is \
             (station {station}, radius {radius}); got (station {y}, radius {rho})"
        );
    }
    // Each rim asked for ON ITS OWN first, and the answer printed. A
    // one-call refusal names ONE edge, and on a body where three rims
    // are new that is not enough to say which arm the door turned away
    // — so the scene asks three questions whose answers are each about
    // one rim.
    for (what, answer) in per_rim_answers(tol) {
        println!("   {what}: {answer}");
    }
    // THREE rims, THREE DIFFERENT coaxial arms. The lid is
    // the tour's carrier of the curved-support fillet family now that
    // `bud` is off the sheet: the flange rim is cone × plane(⊥), the
    // dome's foot is sphere × cone, and the knob's top is
    // cylinder × plane. All three are closed latitude circles because
    // the profile is ANNULAR — which is what the steam vent buys, and
    // the reason an axis-touching profile has no candidate at all.
    //
    // The radius is per REQUEST rather than per edge, as `bud`
    // establishes, and #935 re-reads each later rim's seam-piece
    // identities against the partially-carved body — which is what
    // makes the two requests the naming gap forces (the module docs'
    // sixth finding) the same body the one-request kernel door
    // builds.
    let rolled = body_at(&ev, r.lid);
    assert_eq!(
        (
            rolled.vertices().count(),
            rolled.edges().count(),
            rolled.faces().count(),
        ),
        (9, 18, 9),
        "three annulus bands, each the same census delta: +1 vertex, +2 edges, +1 face"
    );
    let bands = band_faces(&ev, r.lid);
    assert_eq!(bands.len(), 3, "three rims, three bands");
    for name in &bands {
        assert_eq!(
            face_carrier_kind(&ev, r.lid, name).expect("the band face is named"),
            SurfaceKind::Torus,
            "every coaxial band is a TORUS — that is what sharing an axis of revolution \
             buys: the rolling ball's centre is confined to the meridian half-plane"
        );
    }
    // EACH band re-derived from its own two tangency conditions rather
    // than read back from the arm that minted it. Coaxial supports
    // confine the rolling ball's centre to the meridian half-plane,
    // where each support cuts a LINE or a CIRCLE and the centre is the
    // crossing of the two offset traces — so every one of these is a
    // closed form, and they are three DIFFERENT ones.
    //
    // Bands are selected by their spine station, never by index: the
    // arm is free to mint them in any order.
    let lid_band = |y: f64| -> (f64, f64) {
        let mut hit = None;
        for (_, f) in rolled.faces() {
            let Some(Surface::Torus {
                major_radius,
                minor_radius,
                center,
                ..
            }) = rolled.get_surface(f.surface)
            else {
                continue;
            };
            assert!(
                center.x.abs() < 1e-12 && center.z.abs() < 1e-12,
                "a coaxial band's spine circle is centred ON the axis: got {center:?}"
            );
            if (center.y - y).abs() < 1e-12 {
                assert!(hit.is_none(), "two bands share the spine station {y}");
                hit = Some((*major_radius, *minor_radius));
            }
            assert!(
                f.rings.is_empty(),
                "a curved band is ring-free: one cycle, two closed trim circles and a slit"
            );
        }
        hit.unwrap_or_else(|| panic!("no band has its spine at {y}"))
    };

    // (1) cylinder x plane, at the knob's top. On a cylinder of radius
    // R_KNOB the centre rides at R_KNOB - r; on a plane normal to the
    // axis it rides r below it. Two lines, one crossing.
    let (knob_major, knob_minor) = lid_band(Y_TOP - ROLL);
    assert!(
        (knob_minor - ROLL).abs() < 1e-12 && (knob_major - (R_KNOB - ROLL)).abs() < 1e-12,
        "the knob band is the torus (R_KNOB - roll, roll) = ({}, {ROLL}); got \
         ({knob_major}, {knob_minor})",
        R_KNOB - ROLL
    );

    // The flange's half-angle from the axis, and its inward offset
    // trace: a line parallel to the generator, one roll into the
    // material.
    let alpha = ((R_FLANGE - R_NECK) / (Y_FLANGE - LID_BASE)).atan();
    let (sin_a, cos_a) = alpha.sin_cos();
    let off0 = (R_FLANGE - ROLL * cos_a, LID_BASE - ROLL * sin_a);
    let dir = (-sin_a, cos_a);

    // (2) cone x plane(perp), at the flange's rim. The underside's own
    // offset is y = LID_BASE + roll; walk the cone's offset line to
    // that height and read the radius off it.
    let (flange_major, flange_minor) = lid_band(LID_BASE + ROLL);
    let flange_want = R_FLANGE - ROLL * (1.0 + sin_a) / cos_a;
    assert!(
        (flange_minor - ROLL).abs() < 1e-12 && (flange_major - flange_want).abs() < 1e-12,
        "the flange band is the torus (R_FLANGE - roll(1 + sin a)/cos a, roll) = \
         ({flange_want}, {ROLL}); got ({flange_major}, {flange_minor})"
    );

    // (3) sphere x cone, at the dome's foot — the arm no plane-supported
    // scene reaches. The cone's offset is the same line as above; the
    // sphere's is the circle of radius DOME_R - roll about the dome
    // centre. A line and a circle: one quadratic, and the root inside
    // the flange's span is the one the ball rolls in.
    let rho = DOME_R - ROLL;
    // |off0 + u*dir - (0, DOME_C)|^2 = rho^2, solved in u.
    let (px, py) = (off0.0, off0.1 - DOME_C);
    let b = px * dir.0 + py * dir.1;
    let c = px * px + py * py - rho * rho;
    let disc = b * b - c;
    assert!(
        disc > 0.0,
        "the cone's offset line must cut the sphere's offset circle twice"
    );
    let u = -b + disc.sqrt();
    let (foot_r, foot_y) = (off0.0 + u * dir.0, off0.1 + u * dir.1);
    let (foot_major, foot_minor) = lid_band(foot_y);
    assert!(
        (foot_minor - ROLL).abs() < 1e-12 && (foot_major - foot_r).abs() < 1e-12,
        "the dome-foot band is the torus ({foot_r}, {ROLL}) — the cone's offset line \
         meeting the sphere's offset circle at y = {foot_y}; got ({foot_major}, \
         {foot_minor})"
    );

    // The SHARP lid against its closed forms — the dome's area by
    // Archimedes, the rest by the stack — and the roll then bounded
    // against that twin rather than against a remembered number.
    let flange_h = Y_FLANGE - LID_BASE;
    let (v_dome, a_dome) = zone(DOME_R, DOME_C, Y_FLANGE, Y_KNOB);
    let v_lid = v_dome
        + frustum_volume(R_FLANGE, R_NECK, flange_h)
        + stack_volume(&[(R_KNOB, Y_TOP - Y_KNOB)])
        - stack_volume(&[(R_VENT, Y_TOP - LID_BASE)]);
    let a_lid = a_dome
        + frustum_lateral(R_FLANGE, R_NECK, flange_h)
        + 2.0 * PI * R_KNOB * (Y_TOP - Y_KNOB)
        + annulus(R_KNOB, R_VENT)
        + annulus(R_FLANGE, R_VENT)
        + 2.0 * PI * R_VENT * (Y_TOP - LID_BASE);
    let sharp_lid_props = pncad::topo::mass_properties(&plain_lid, tol).expect("the lid's props");
    assert!(
        ((sharp_lid_props.volume - v_lid) / v_lid).abs() < 1e-12,
        "lid V = {} vs the closed form {v_lid}",
        sharp_lid_props.volume
    );
    assert!(
        ((sharp_lid_props.surface_area - a_lid) / a_lid).abs() < 1e-12,
        "lid A = {} vs the closed form {a_lid}",
        sharp_lid_props.surface_area
    );
    let lid_props = pncad::topo::mass_properties(&rolled, tol).expect("the rolled lid");
    let dv_lid = sharp_lid_props.volume - lid_props.volume;
    // Three convex rims now, so the bound is the three corner squares
    // swept round their own rims.
    let pappus_cap = 2.0 * PI * ROLL * ROLL * (R_KNOB + R_FLANGE + R_NECK);
    assert!(
        dv_lid > 0.0 && dv_lid < pappus_cap,
        "each convex rim's roll removes material, and less than the corner square swept \
         round it: ΔV = {dv_lid} against the bound {pappus_cap}"
    );

    // ---- the spout: built about its own axis, then placed ----
    //
    // The document's placement vocabulary is AXIS-ANGLE and the 3-4-5
    // turn is not a binary-exact angle, so what the residual costs is
    // MEASURED — off the placed BODY, never re-derived from the same
    // trigonometry the placement used. The subject is the root
    // annulus: the band of the spout meridian's own segment 0, named
    // at the revolve that minted it and read at the TRANSFORM, since a
    // transform contributes no role segment and reading a carried name
    // there is reading the placed face.
    //
    // TWO subjects, the root annulus and the tip one, and the guard
    // below is why there are two and why neither is the spout's own
    // origin: a point on the rotation's fixed axis maps to `R·0 + t`
    // for any angle whatsoever, so a residual read there measures the
    // translation and would be 0 with the turn deleted. These two
    // stand off that axis and one of them stands a whole spout-length
    // up it, so both residuals are about the turn.
    let tip_unplaced = face_frame(&ev, r.spout_body, &band(r.spout_body, SEG_SPOUT_TIP))
        .expect("the tip annulus, before the placement");
    let unplaced = face_frame(&ev, r.spout_body, &band(r.spout_body, SEG_SPOUT_ROOT))
        .expect("the root annulus, before the placement");
    let tip_placed = face_frame(&ev, r.spout, &band(r.spout_body, SEG_SPOUT_TIP))
        .expect("the tip annulus, after it");
    let placed = face_frame(&ev, r.spout, &band(r.spout_body, SEG_SPOUT_ROOT))
        .expect("the root annulus, after it");
    // The EXACT image of that face's own frame under the placement the
    // authored direction states — the 3-4-5 turn written as the matrix
    // whose columns are the direction's own components, which is what
    // this scene used to hand `transform_rigid` and is independent of
    // the angle the document stores.
    let turn = |p: Vec3<f64>| {
        Vec3::new(SPOUT_DIR.y, -SPOUT_DIR.x, 0.0) * p.x + SPOUT_DIR * p.y + Vec3::unit_z() * p.z
    };
    let root_exact = turn(Vec3::new(
        unplaced.origin.x,
        unplaced.origin.y,
        unplaced.origin.z,
    )) + Vec3::new(SPOUT_ROOT.x, SPOUT_ROOT.y, SPOUT_ROOT.z);
    let root_residual =
        (Vec3::new(placed.origin.x, placed.origin.y, placed.origin.z) - root_exact).norm();
    let tip_exact = turn(Vec3::new(
        tip_unplaced.origin.x,
        tip_unplaced.origin.y,
        tip_unplaced.origin.z,
    )) + Vec3::new(SPOUT_ROOT.x, SPOUT_ROOT.y, SPOUT_ROOT.z);
    let tip_residual = (Vec3::new(
        tip_placed.origin.x,
        tip_placed.origin.y,
        tip_placed.origin.z,
    ) - tip_exact)
        .norm();
    assert!(
        tip_residual <= 1e-15,
        "the placed tip annulus stands at {:?}; the direction's own matrix puts it at \
         {tip_exact:?} ({tip_residual:e} m away)",
        tip_placed.origin
    );
    // Both subjects stand OFF the rotation's fixed set (the +z axis
    // through the world origin), so both residuals are about the turn
    // and neither is the `R·0 + t = t` identity a point on that axis
    // would give for any angle at all.
    for (what, o) in [("root", unplaced.origin), ("tip", tip_unplaced.origin)] {
        assert!(
            o.x.hypot(o.y) > 1e-3,
            "the {what} annulus stands at {o:?}, on the rotation's own fixed axis — a \
             residual read there measures the translation and not the turn"
        );
    }
    // A BOUND rather than a bit, and the bound is the argument: the
    // angle's cosine and sine are a libm's answer, so an equality here
    // would gate this scene on one platform's last ulp (the reason
    // this workspace routes `erf` through the `libm` crate). What is
    // observed goes in the note; what is ASSERTED is that the placed
    // face stands where the direction says to within a ulp of the
    // spout's own scale.
    assert!(
        root_residual <= 1e-15,
        "the placed root annulus stands at {:?}; the direction's own matrix puts it at \
         {root_exact:?} ({root_residual:e} m away)",
        placed.origin
    );
    // And it FACES the way the direction says: the placed chart's axis
    // is the image of the unplaced one, which for this annulus is the
    // sketch's own +v carried onto SPOUT_DIR.
    let axis_cross = placed.axis.cross(SPOUT_DIR).norm();
    let axis_dot = placed.axis.dot(SPOUT_DIR).abs();
    assert!(
        axis_cross <= 1e-15 && (axis_dot - 1.0).abs() <= 1e-15,
        "the placed root annulus's normal is {:?}, which is not SPOUT_DIR ({SPOUT_DIR:?}): \
         cross {axis_cross:e}, |dot| {axis_dot}",
        placed.axis
    );
    // The observed turn, REPORTED — the note quotes it and nothing
    // gates on it.
    let (sin_t, cos_t) = spout_turn().sin_cos();
    let spout = body_at(&ev, r.spout);
    let v_spout = frustum_volume(SPOUT_R0, SPOUT_R1, SPOUT_LEN)
        - frustum_volume(SPOUT_R0 - SPOUT_WALL, SPOUT_R1 - SPOUT_WALL, SPOUT_LEN);
    let a_spout = frustum_lateral(SPOUT_R0, SPOUT_R1, SPOUT_LEN)
        + frustum_lateral(SPOUT_R0 - SPOUT_WALL, SPOUT_R1 - SPOUT_WALL, SPOUT_LEN)
        + annulus(SPOUT_R0, SPOUT_R0 - SPOUT_WALL)
        + annulus(SPOUT_R1, SPOUT_R1 - SPOUT_WALL);
    let spout_props = pncad::topo::mass_properties(&spout, tol).expect("the spout's props");
    // Asserted AFTER the placement, which is the point: the closed
    // forms are stated in the spout's OWN frame, so the rigid map is
    // what has to have left them alone.
    assert!(
        ((spout_props.volume - v_spout) / v_spout).abs() < 1e-12,
        "spout V = {} vs the frustum difference {v_spout}",
        spout_props.volume
    );
    assert!(
        ((spout_props.surface_area - a_spout) / a_spout).abs() < 1e-12,
        "spout A = {} vs the closed form {a_spout}",
        spout_props.surface_area
    );

    // ---- the handle ----
    let sweep = 2.0 * (FRAC_PI_2 + HANDLE_OVER);
    let handle = body_at(&ev, r.handle);
    let v_handle = sweep * HANDLE_R * PI * HANDLE_TUBE * HANDLE_TUBE;
    let a_handle = sweep * HANDLE_R * 2.0 * PI * HANDLE_TUBE + 2.0 * PI * HANDLE_TUBE * HANDLE_TUBE;
    let handle_props = pncad::topo::mass_properties(&handle, tol).expect("the handle's props");
    assert!(
        ((handle_props.volume - v_handle) / v_handle).abs() < 1e-12,
        "handle V = {} vs Pappus on the disc {v_handle}",
        handle_props.volume
    );
    assert!(
        ((handle_props.surface_area - a_handle) / a_handle).abs() < 1e-12,
        "handle A = {} vs the closed form {a_handle}",
        handle_props.surface_area
    );

    // ---- the walls ----
    //
    // Every one of these is ATTEMPTED here, on every pass, and pinned
    // by its own EXACT typed refusal: a different refusal, or a
    // success, fails the tour — so the findings list in the module
    // docs cannot rot behind a frontier that moved.

    // WALL 1 — RETIRED at #1081's PR-2b, and the retirement is the
    // pot above: the belly IS the arc now. What this wall pinned was
    // the sealed hollow of a sphere-zone meridian refusing
    // `ReanchorOffCarrier`, and it refused because `shell` moved one
    // chart at a time. The simultaneous door solves each corner
    // against every surface meeting it, so the arc ships and the
    // squared shoulders are gone from the scene entirely.
    //
    // WALL 1, RE-PLANTED and now RETIRED IN TURN. The re-planted wall
    // pushed the arc's centre OFF the axis, making the belly a TORUS —
    // a kind the meridian reduction did not know, so the body fell to
    // the per-chart loop and refused at the C5 table's plane×torus
    // pair. The reduction knows the kind now: a coaxial torus's
    // meridian is a circle centred `(R, h_c)` in the `(ρ, h)`
    // half-plane, which is the sphere's circle centred `(0, h_c)` with
    // one more number, so the corner solve takes it and never asks the
    // table. The pot below is ATTEMPTED live, exactly as the wall was,
    // and it hollows — through the SAME node the scene's own pot takes.
    let torus_pot = wall_one_pot(tol);
    assert_eq!(
        pncad::topo::validate_geometric(&torus_pot, tol),
        Ok(()),
        "the torus-bellied pot: tier 3"
    );
    assert_eq!(
        torus_pot.shells().count(),
        2,
        "the torus-bellied pot's hollow is outer + cavity"
    );
    println!(
        "   wall 1 — RETIRED: the torus-bellied pot hollows ({} faces over two shells); \
         its junction corners are pinned to their closed forms by the \
         kernel's own torax_axial suite",
        torus_pot.faces().count()
    );

    // THE MOUTH, OPENED — the scene's second finding, RETIRED. This
    // was wall 2: `shell_open` returned a body that passed tiers 1-3
    // while each designated half-disc carried its cavity counterpart's
    // own boundary as a ring, and the CDT refused it (#1082). It is
    // measured here rather than cited, because the pot the montage
    // ships is now this body.
    assert_eq!(
        pncad::topo::validate_geometric(&cup, tol),
        Ok(()),
        "tier 3 on the cup, which now also refuses a ring standing on its outer loop"
    );
    let cup_rings: usize = cup.faces().map(|(_, f)| f.rings.len()).sum();
    assert_eq!(
        (cup_rings, genus(&cup), cup.shells().count()),
        (1, 0, 1),
        "ONE rim annulus carrying ONE ring, genus 0 as `topo::shell`'s docs promise a \
         cup is, and the cavity fused into the boundary"
    );
    assert_eq!(
        faces_where(&ev, r.cup, SegPat::tag(SegTag::Rim)).len(),
        1,
        "the revolve's seam is retired before the glue, so the mouth's two designated \
         halves come back as ONE rim face and not as two half-annuli"
    );
    let cup_props = pncad::topo::mass_properties(&cup, tol).expect("the cup's props");
    // The cup is the sealed wall LESS the disc of wall the mouth cap
    // was: opening lifts the cavity's cap from the mouth plane's own
    // station minus a wall, up to the plane.
    // The lifted mouth disc is NOT a cylinder any more: the cavity's
    // wall over that slab is the belly SPHERE, so the plug the lift
    // opens is the zone integral between the cavity's own top station
    // and the mouth plane. Reading it as a cylinder was right only
    // while the neck was one.
    let cup_want = v_want - {
        let rr = R_BELLY - WALL;
        let zone = |y: f64| {
            let u = y - Y_BELLY_C;
            rr * rr * u - u * u * u / 3.0
        };
        PI * (zone(Y_MOUTH) - zone(Y_MOUTH - WALL))
    };
    assert!(
        ((cup_props.volume - cup_want) / cup_want).abs() < 1e-12,
        "cup V = {} vs the wall's closed form less the lifted mouth disc {cup_want}",
        cup_props.volume
    );
    let cup_mesh = pncad::mesh::tessellate(&cup, DELTA, tol)
        .unwrap_or_else(|e| panic!("the opened pot must triangulate, got {e:?}"));
    let cup_triangles: usize = cup_mesh.patches.iter().map(|q| q.triangles.len()).sum();
    assert!(cup_triangles > 0, "a mesh with no triangles is not a mesh");

    // WALL 2 — the handle joined to the pot. A curved x curved pair at
    // the operand gate; the germ roster has no arm for it.
    //
    // Each union is a NODE of the document and refuses at `evaluate`,
    // with the kernel's payload carried unaltered into the panel's
    // note — so the payload the caption quotes and the payload the
    // probe pins cannot be two different measurements.
    let handle_refusal = describe(&ev, r.handle_union);
    crate::walls::wall(
        "teapot",
        2,
        "join the handle to the vessel (union; both roots driven 11.2 mm past the \
         belly's inner wall — a real overlap, not a tangency)",
        join_outcome(&ev, r.handle_union),
        |e| {
            matches!(
                e,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::B,
                    kind: SurfaceKind::Torus,
                    // The pot's belly is a SPHERE now, not the squared
                    // pot's cylinder: same gate, and the pair it names
                    // is the pair the geometry actually has.
                    other_kind: SurfaceKind::Sphere,
                    ..
                }
            )
        },
        "make the teapot ONE solid: union the handle and the spout into the vessel, drop \
         walls 2 and 3, re-state the montage caption (which currently says four solids), \
         and RE-CUT THE HANDLE'S OVERSHOOT FIRST — at 0.5 rad its roots stand 11.2 mm \
         inside the cavity, which is fine for a refused request and wrong for a joined one",
    );

    // WALL 3 — the spout joined to the pot. A DIFFERENT pair, and it
    // is not the pair the model cares about: the gate is pair-scoped
    // and box-conservative, so the faces it names are the first whose
    // boxes MAY meet — here the spout's outer cone against a PLANE of
    // the pot, not the belly wall the spout actually pierces.
    let spout_refusal = describe(&ev, r.spout_union);
    crate::walls::wall(
        "teapot",
        3,
        "join the spout to the vessel (union; the root disc wholly inside the belly)",
        join_outcome(&ev, r.spout_union),
        |e| {
            matches!(
                e,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::B,
                    kind: SurfaceKind::Cone,
                    other_kind: SurfaceKind::Plane,
                    ..
                }
            )
        },
        "make the teapot ONE solid: union the handle and the spout into the vessel, drop \
         walls 2 and 3, re-state the montage caption (which currently says four solids), \
         and RE-CUT THE HANDLE'S OVERSHOOT FIRST — at 0.5 rad its roots stand 11.2 mm \
         inside the cavity, which is fine for a refused request and wrong for a joined one",
    );

    vec![Stop {
        name: "teapot",
        caption: "THE TEAPOT (FOUR solids: the opened pot, the lid lifted, and the \
                  spout and handle it will not join)"
            .to_string(),
        montage: true,
        story: "shell's designated demo, and ONE recipe document: every body below is a \
                node's value. The pot is ONE revolved profile hollowed by \
                `Node::Shell` and OPENED at its mouth: a cavity the size of the tea, inside a \
                wall 7.8 mm thick, in one solid — drawn see-through, because a cavity \
                cannot be read from an opaque render at any camera. The lid is a SECOND solid, rendered lifted: \
                an exploded view, not a mate. The spout and the handle are two more, \
                and their unions with the pot are attempted on every pass and REFUSE — \
                so what the montage shows is four bodies sitting where a teapot's parts \
                sit, not a teapot",
        ops: "ONE recipe document: Profile -> Revolve -> Node::Shell(t = 7.8125 mm, the \
              mouth's two half-discs BY NAME) for the vessel; Profile -> Revolve -> \
              Node::Fillet twice (the flange rim, then the dome foot + the knob top, all \
              by name) for the lid; Profile -> Revolve -> Node::Transform for the spout; \
              Datum::Axis -> Node::Tube for the handle. Two walls pinned: both unions, \
              as Node::Boolean(Union) nodes that refuse at evaluate",
        delta: DELTA,
        note: Some(format!(
            "ONE RECIPE DOCUMENT, AND EVERY BODY HERE IS A NODE'S VALUE — the mouth and \
             the lid's three rims are NAMED by their role in the sweep, not found by a \
             numeric scan of the built body. THE VESSEL, SEALED THEN OPENED, is the \
             same operand and the same wall through the same node kind twice, parted \
             only by the open list: EMPTY for the sealed hollow, the mouth's two \
             half-disc names for the cup. Sealed it is {pv} vertices, {pe} edges, \
             {pf} faces over TWO shells in one solid — the operand's 8/14/8 twice, \
             since the cavity is that same boundary offset inward and inserted whole \
             through the shared void door. Genus 0. V = {:.9} m³ of WALL against the \
             difference of two closed-form SPHERICAL ZONES on a foot, and A = {:.9} m² \
             against the two boundaries' own; zero enclosure pad. The capacity is asked for \
             DIRECTLY of THAT body rather than inferred: `classify_shells` gives the \
             cavity the Void role and its signed volume is {:.9} m³, the cavity's own closed form, \
             its {capacity_l:.4} LITRES negated by the orientation convention — a \
             reading the OPENED pot cannot give, because its cavity is no longer a \
             void. The gates, measured on the operand before the verb \
             ran: the closest antiparallel planar pair clears {clearance} m where two \
             walls need {:.7} m, so `wall_clearance` — which is what stands in for a \
             plane's vacuous reach margin — does not bind; the tightest cylinder has \
             {reach} m of realized inner radius left. Note the sense bit in that scan: \
             {stored_plus} of this pot's {} planar FACES store a +y normal and only the \
             face's own orientation says which way each looks, so a sense-blind scan \
             finds no antiparallel pair here at all. THE BELLY IS AN ARC, AND THAT IS THE \
             SCENE'S FIRST FINDING RETIRED. It read THE BELLY IS STILL SQUARED for \
             two waves: `shell` moved ONE chart at a time and re-anchored its \
             neighbours on carriers that had not moved, so a junction survived only \
             where the neighbouring surface was invariant under the moved face's own \
             offset — and the class was OBLIQUE junctions rather than curvature, a \
             triangular prism of all planes refusing exactly like a cone frustum. \
             #1081 made the offsets SIMULTANEOUS: every corner is solved against all \
             the surfaces meeting it at once, planar corners by a 3×3 solve (PR-2a) \
             and a body of revolution's in its own meridian half-plane (PR-2b), where \
             a plane is a line, a cylinder is a line, a cone is a line, a sphere is \
             a circle centred ON the axis and a TORUS is that same circle centred one \
             number off it. So the shoulders are gone: foot cylinder, ONE spherical \
             zone, mouth. WALL 1 WAS RE-PLANTED ONE STEP OUT AND IS NOW RETIRED IN \
             TURN: it pushed the belly's arc centre OFF the axis, making the wall a \
             torus the meridian reduction did not know, so the body fell to the \
             per-chart loop and the C5 table refused its plane x torus pair. The \
             reduction knows the kind now, that pot hollows, and the probe above \
             asserts the hollow instead of pinning a refusal — the hollow rides the \
             meridian reduction, not the C5 table, and held on both sides of \
             VERBS-C5ARMS's later `intersect::route` widening for the pair \
             (`tests/verbs_teapot.rs` \
             carries the junction table, the tangency discriminator and the sweep's \
             blind spot). THE MOUTH IS \
             OPEN, AND THAT WAS THE SCENE'S SECOND FINDING BEFORE IT WAS FIXED: \
             `shell_open` used to return a body that passed tiers 1-3 while each \
             designated half-disc carried its own cavity counterpart's boundary as a \
             ring — sharing the axis apex the outer loop owned, running back along its \
             seam legs — so the CDT refused it and the pot shipped sealed (#1082). The \
             class was never \"a revolve's cap is two half-discs\": it was any \
             designated face whose cavity counterpart's boundary cannot become an \
             interior-disjoint RING of it, and what put it out of reach was the \
             REVOLVE's seam rather than the mouth. The seam is retired before the glue \
             now, through the Euler doors alone, and this pot's mouth comes back as ONE \
             annular rim carrying one ring, genus {}, V = {:.9} m³ against the wall's \
             closed form less the lifted mouth disc, {cup_triangles} triangles at δ = \
             {DELTA}. On an ANNULAR mouth the same class needs two disjoint annuli — a \
             face SPLIT — and that is built too; `tests/verbs_teapot.rs` carries both \
             shapes. THE OPENED POT NOW LEAVES AS STEP, which the sealed one could not: \
             the writer's outward/void classifier has closed forms for planar faces \
             only, so a CURVED solid of two or more shells refuses \
             CurvedShellClassification — and a cup is ONE shell. That gate keeps three \
             live probes (klein's wall 6, the `ring` scene's and `hollowtorus`'s); this \
             scene is no longer one of them, because the body it ships no longer \
             reaches it. THE LID, which is where the CURVED-SUPPORT fillet family \
             lives on this tour. 6/12/6 sharp — an ANNULAR profile mints one FULL wall \
             per segment where the pot's axis-touching profile mints half-walls — and \
             9/18/9 rolled, three annulus bands each carrying the same (+1, +2, +1). \
             THREE closed latitude rims roll, in TWO requests where the kernel door \
             takes one — the flange's rim, then the dome's foot and the knob's top — \
             because the flange's rim and the dome's foot are the two ends of ONE \
             meridian segment, both bands slit THAT segment's seam, and a `BandSlit` is \
             named by the source edge it severed, so one request cannot NAME its own \
             output. The test is the shared MERIDIAN and not adjacency: the adjacent \
             pairs at the knob compose in one request, and `tests/teapot_document.rs` \
             tabulates which do and pins the two spellings' bodies equal — same census, \
             the same three bands bit for bit, the same mass, a different face order. \
             Their supports are three \
             DIFFERENT coaxial arms: the flange's rim is cone x plane(perp), the dome's \
             foot is SPHERE x CONE — the arm no plane-supported scene reaches — and the \
             knob's top is cylinder x plane. Every band is a ring-free TORUS, which is \
             what sharing an axis buys, and each is re-derived here from its OWN two \
             tangency traces rather than read back from the arm that minted it: two \
             lines for the knob ({knob_major}, {ROLL}), two lines for the flange \
             ({flange_major}, {ROLL}), and for the dome's foot a line meeting a circle \
             ({foot_major}, {ROLL}) at the crossing y = {foot_y}. ΔV = {dv_lid:.9} m³, \
             inside the three corner squares' bound {pappus_cap:.9}. The steam vent is \
             what makes those rims CLOSED edges at all: bore the finial and the profile \
             is annular; leave it solid and each rim is two arcs over two half-discs, \
             which the annulus band does not carve — and it is also why each of these \
             three rims is ONE name: an annular profile touches the axis nowhere, so \
             the full revolve mints one whole wall per segment and one CLOSED rim per \
             meridian vertex, where the pot's axis-touching profile mints the half-wall \
             pair the mouth is named as. NO MATE IS AUTHORED — the lid renders {LIFT} m above the mouth and \
             the two bodies are strangers to the kernel; declared contact is M9's. THE \
             SPOUT AND THE HANDLE. Both build and both check against closed forms — the \
             spout as a difference of cone frusta, asserted AFTER `Node::Transform` \
             placed it, so the map's isometry is part of the receipt; the handle by \
             Pappus on its own disc. The document says a placement in AXIS-ANGLE and the \
             3-4-5 turn is not a binary-exact angle, so what that costs is MEASURED off \
             the PLACED BODY: the root annulus — the spout meridian's own segment-0 \
             band, named at the revolve and read at the transform — stands \
             {root_residual:e} m from where the direction's own matrix puts it and its \
             chart faces SPOUT_DIR to a cross of {axis_cross:e}, and the TIP annulus \
             one spout-length up stands {tip_residual:e} m from its own exact image. \
             Both subjects sit OFF the turn's fixed axis, which is what keeps either \
             residual a measurement of the ROTATION rather than of the translation. On THIS platform the \
             angle's cosine and sine come back as {cos_t} and {sin_t}, which are the \
             direction's components bit for bit; that is reported and not asserted, \
             because a bitwise pin here would gate the scene on one libm's last ulp. Neither JOINS. handle ∪ vessel: {handle_refusal}. \
             spout ∪ vessel: {spout_refusal}. Both are the pair-scoped operand gate \
             naming a germ PAIR with no wired arm, and note what the second one names — \
             the spout's outer CONE against a PLANE of the pot, not the belly wall the \
             spout actually pierces: box overlap is a MAY, and the gate reports the \
             first pair whose boxes may meet. The schedule is the banked germ-chord \
             lanes (DESIGN frontier (d)) and #1057's two C5 arms. BOTH REFUSALS NOW \
             ARRIVE THROUGH THE DOCUMENT, under the same payload: each union is a \
             `Node::Boolean` that lowers to the same kernel `union` and fails at \
             `evaluate`, so what the caption quotes is the evaluation's own carried \
             refusal. A lofted or \
             canal-swept spout — the shape a potter would draw — is not authorable at \
             all; the register carries that as a note and this scene does not hack \
             around it",
            props.volume,
            props.surface_area,
            voids[0].volume,
            2.0 * WALL,
            planes.len(),
            genus(&cup),
            cup_props.volume,
        )),
        // The pot's axis is +y and its spout, handle and lid knob all
        // lie in the world z = 0 plane, so a camera near -z sees the
        // silhouette a teapot is recognised by. Ten degrees off it
        // toward the spout, and 22 up: enough elevation that the
        // shoulders and the lid's dome read as ellipses rather than
        // lines, and little enough that the lid's lift stays a visible
        // GAP rather than foreshortening onto the mouth.
        view: View {
            elev: 22.0,
            azim: -100.0,
            up: 'y',
        },
        bodies: vec![
            // See-through for the hollow ring's reason, on this verb's
            // own shape: the subject is a CAVITY, and no camera reads
            // one from an opaque render.
            SceneBody::plain("teapotvessel", [0.72, 0.70, 0.66], cup)
                .transparent(45)
                .named(&ev, r.cup),
            SceneBody::plain("teapotlid", [0.58, 0.64, 0.72], rolled).named(&ev, r.lid),
            SceneBody::plain("teapotspout", [0.72, 0.70, 0.66], spout).named(&ev, r.spout),
            SceneBody::plain("teapothandle", [0.58, 0.64, 0.72], handle).named(&ev, r.handle),
        ],
    }]
}
