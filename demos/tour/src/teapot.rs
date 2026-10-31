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
//! - **the lid** — a second `revolve` of a solid profile, its three
//!   rims rolled in one request, each band carved over the rim's two
//!   half-arcs. Rendered LIFTED above the
//!   mouth: an exploded view. No mate is authored and none is implied
//!   — declared contact is M9's territory, and a scene that faked one
//!   here would be claiming a certification nothing issued.
//! - **the spout** — a CANAL: seven annular sections standing on the
//!   tangent frames of a circular arc, skinned by ONE `Node::Loft`,
//!   built about its own spine and placed against the pot. It
//!   bends, it tapers, and its wall thins with it, so no revolve
//!   reaches it at any axis. That is finding 5, and the shape and the
//!   finding moved together.
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
//! 3. **The lid is solid, and its rims roll whole.** A full revolve
//!    of a profile that touches the axis mints half-walls, and each
//!    latitude rim is two open arcs meeting at two seam vertices. One
//!    arc alone refuses `UnsupportedCorner` at a seam vertex; the pair
//!    carves as ONE torus band
//!    (`verbs_arms1_r1_probes::the_unbored_hemisphere_equator_carves_as_one_band`).
//!    So the lid is what a potter turns — a solid knob and no vent —
//!    and its three rims are asked for whole, two names each. A vent
//!    through the finial would buy nothing a lid wants: a real lid's
//!    steam hole sits off the axis, which no revolve reaches.
//! 4. **The teapot is four solids because both joins refuse — at TWO
//!    DIFFERENT DOORS, each of which has moved.**
//!
//!    handle ∪ pot used to be torus × SPHERE at the operand gate. The
//!    torus is on the union's KIND roster now, so the join gets past
//!    the gate. It then stopped at the maximal-faces precondition on
//!    the POT, whose full revolve split its planar walls in two; a full
//!    revolve sweeps them whole now (BAND, one wall per run), and the
//!    join reaches the pot's sphere face meeting the handle's torus. The
//!    section value builds that pair now: its pose decided once, its two
//!    loops about the handle's tube marched and fitted, each germ matched
//!    to its loop by foot, and each closed loop seated at its first site,
//!    so the segment lays them off the edges and the union builds. The
//!    result then fails its own validation: its volume is not computed,
//!    the vessel's sphere face now trimmed by a conic loop that no
//!    sphere-chart quadrature reads yet (`ResultInvalid` with
//!    `VolumeUncomputable` from `QuadratureUnsupported`), pinned in wall
//!    2.
//!
//!    spout ∪ pot used to be the same shape of refusal — cone × plane
//!    — and it is not any more. A loft's walls are `Nurbs`, and the
//!    pair gate has an arm for `Nurbs`, so the request gets PAST the
//!    pair rung and into the sweep, where it stops at the crossing
//!    layer: the belly's revolve seam — a quarter meridian of its
//!    sphere, on which the spout's axis leaves the belly — read against
//!    one of the spout's NURBS walls rising there, meets that wall's
//!    boundary, since the meridian and the spout's seams lie in the
//!    pot's one plane of symmetry. A meeting on a boundary edge splits
//!    both edges, and the spout's seam is a spline with no point
//!    parameter to split at: `PointSplitCarrierUnsupported { operand: B }`.
//!    That is the pierce the join actually needs, the spout's walls entering
//!    the belly, and it is the NURBS face against a curved face that
//!    the section object's rung 3 builds. Making the spout the shape a
//!    potter draws did not make it joinable; it moved the refusal off a
//!    pair the model does not care about and onto the meeting itself.
//!    Wall 3 pins it and the note carries the payload.
//!
//!    Wall 2 is past the operand gate, the maximal-faces precondition
//!    and the section, and stops at the segment the join cannot yet
//!    lay along the vessel's sphere face where it meets the handle's
//!    torus. **Both refusals arrive through the
//!    DOCUMENT**: each join is a `Node::Union` that lowers to the
//!    same kernel `union` and fails at `evaluate`, and what the note
//!    quotes is that node's own carried refusal rather than a second
//!    measurement of it.
//!
//!    Neither wall's probe ever pinned an arena KEY, and that is why
//!    both survived a re-authoring that moved one: the same physical
//!    face, the mouth-rim annulus at `y = 1/8`, was `FaceKey(1v1)`
//!    when the cup came from a plane scan and became `FaceKey(2v1)`
//!    once it came from the document. Wall 2 matches on `operand`
//!    (the vessel's split planar walls); wall 3 matches on `operand` alone,
//!    because the variant it pins carries no kinds — the edge key it
//!    does carry is exactly the sort of value a probe must not pin.
//! 5. **A spout the shape of a spout IS authorable — and this row was
//!    half wrong when it was written.** It used to read: *"there is no
//!    variable-section sweep, so the shape on screen is a straight
//!    cone frustum tilted into place: a spout the way a lathe would
//!    make one."* Both halves are re-measured here.
//!
//!    **The false half.** `Node::Loft` takes its sections as profile
//!    NODES, so each one carries its own sketch plane, and a
//!    `ProfileProgram` carries a LIST of loops — which is a variable
//!    section, an annular one, placed anywhere. That is exactly the
//!    door "no variable-section sweep" said did not exist, and it has
//!    been bound since LIB-PYG23A. The spout below is seven annuli on
//!    the tangent frames of a circular arc, skinned in ONE node: a
//!    canal that bends through half a right angle, tapers to half its
//!    root radius, and thins as it goes — reachable by no revolve
//!    about any axis, and by no extrude.
//!
//!    **The true half, narrowed to what it is.** The RECIPE door is
//!    what is missing: `wire_sweep` refuses unconditionally, and that
//!    door is banked (`SWEEP_FRONTIER`, U4/LQ3), so a swept spout
//!    cannot be said in a document and a loft of enough sections is an
//!    approximation of that rather than the thing. The bend is not
//!    the obstacle — the library's `sweep_body` turns a section
//!    through any bend short of doubling back, and refuses only where
//!    a station's tangent opposes the start tangent (the klein scene's
//!    U-turn loop is lofted through stated normal planes for that
//!    reason), and this spout turns half a right angle — so what a
//!    recipe door would buy here is the spine in the document, not a
//!    shape the kernel cannot build.
//!
//!    **What the loft costs instead, and it is a different debt.** The
//!    SPINE IS NOT IN THE DOCUMENT. `spout_stations` computes six
//!    stations from [`SPOUT_BEND`] and writes their spins and slides as
//!    literals, so a reader of the saved document sees six stations and
//!    no arc, and changing the bend means re-deriving
//!    all six rather than editing one number. That is
//!    `twisted_tube`'s own star one scene over, and a spine datum a
//!    loft could read its placements off is what would close it.
//!
//!    **What the shape cost in EVIDENCE, said out loud.** Less than it
//!    looks, and the reason is worth having: **a bend about a spine
//!    through the sections' own centres is volume-neutral.** The
//!    tube's volume element is `(1 − κ·x) dA ds` with `x` toward the
//!    curvature centre, and `∫x dA` over a centred section is zero, so
//!    the curvature term integrates away and what is left is the
//!    STRAIGHTENED tube's volume — the same frustum difference this
//!    scene used before the spout bent. The same cancellation runs
//!    over the lateral area, where `∮(1 − κ r cosθ) dθ = 2π`.
//!
//!    So the closed form survives the bend, but it is the closed form
//!    of the swept tube, and the body is a cubic skin through
//!    [`SPOUT_STATIONS`] stations that INTERPOLATES that tube. **On the
//!    volume the two differ by the LOFT FIT, 2.1e-6 of it**, inside the
//!    cubic's own error scale `2·(5/384)·Δ⁴` = 7.6e-6 at the stations'
//!    angular step `Δ`. On the area they differ by 2.2e-5, which is not
//!    a fit measurement, and the scene reports it rather than gating it.
//!
//!    **So the scene keeps two references apart.** The skin's OWN
//!    volume and area, integrated over the body's own surfaces by a
//!    dense Gauss–Legendre rule (`patch_quadrature`), are what the
//!    kernel's certified enclosure must hold. The frustum is held to
//!    that reference at the fit's scale. The fit's miss is some
//!    fourteen thousand of the enclosure's half-widths at ε = 1e-12, so
//!    an enclosure asked to hold the frustum is asked to hold the wrong
//!    body.
//!
//!    **The sections are ROUND, so the volume is an ENCLOSURE.** A
//!    circle is RATIONAL, so these lateral walls are rational patches,
//!    and a rational patch is a QUADRATURE face whose certified
//!    enclosure is chased to the tour's printed precision. Where it is
//!    not met, tier 3 still certifies the body on the
//!    enclosure's SIGN — the tier's own docs say deciding that sign "is
//!    an act of certification rather than a measurement" — and the
//!    scene measures through the certificate the tier hands back, so it
//!    holds the skin's reference to the bracket the measurement
//!    reached. A polygon's sides are
//!    straight, so octagonal sections would make polynomial walls with
//!    the integral lane's exact per-span rule, the one `twisted_tube`
//!    next door takes; the shape a potter draws is round.
//!
//! # What this scene deliberately does NOT do
//!
//! No kernel change, no route widened, no gate softened. Every one of
//! the five findings above is a live probe or an executed table, and
//! each carries the sentence that retires it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};
use pncad::prelude::AuthoredNode;

use pncad::document::{
    Axis3, Bodies, CancelToken, Coord, Dimension, Doc, DocEdit, EvalOptions, Evaluation, Formula,
    LoftSection, LoopProgram, Node, NodeErrorKind, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget, RecipeNodeId, StationMove, TargetAxis, TubeWindow, ValuePayload, apply,
    evaluate,
};
use pncad::geom::{Curve3, Surface};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::prelude::SurfaceKind;
use pncad::prelude::query;
use pncad::prelude::{
    CapEnd, EntityKind, MeridianEnd, NamePat, SegPat, SegTag, Selector, StableName,
};
use pncad::profile::ArcSweep;
use pncad::select::{
    ProfileEdgeRef, ProfilePieces, ProfileVertexRef, band, band_rim, band_rim_pi, edge_name,
    face_carrier_kind, face_frame, meridian_vertex, select, vertex_position,
};
use pncad::topo::readback::euler_counts;
use pncad::topo::{Body, BooleanError, Operand};

use crate::{SceneBody, TourStop, View};

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
pub const R_NECK: f64 = 3.0 / 64.0;
/// Where the foot ends and the belly's arc begins.
const Y_FOOT: f64 = 1.0 / 64.0;
/// The belly sphere's centre, on the axis. The two junction stations
/// are the sphere's own 3-4-5 points: `(4/64, 1/64)` at the foot and
/// `(3/64, 8/64)` at the mouth, both with an exactly zero residual.
const Y_BELLY_C: f64 = 4.0 / 64.0;
/// The mouth's plane.
pub const Y_MOUTH: f64 = 8.0 / 64.0;

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
pub const LIFT: f64 = 1.0 / 32.0;
/// The lid's underside plane.
pub const LID_BASE: f64 = Y_MOUTH + LIFT;
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
pub const R_FLANGE: f64 = 14.0 / 256.0;
/// Where the conical flange ends and the dome's sphere begins.
pub const Y_FLANGE: f64 = LID_BASE + 6.0 / 256.0;
/// The dome's sphere radius, and its centre's station: the 5-12-13
/// triple. What that buys is not that any of these decimals is a
/// binary-exact float — 3/64 and 13/256 are, 0.6 and 0.8 would not be
/// — but that the RESIDUALS are exactly zero: `|c − p|² − r²` for the
/// rim and for the knob's foot evaluate to 0.0 in f64, so the arc door
/// has nothing to round when it checks equidistance.
const DOME_R: f64 = 13.0 / 256.0;
/// The dome sphere's centre, one unit ABOVE the lid's underside —
/// the flange lifts the dome's foot, and the centre with it.
pub const DOME_C: f64 = LID_BASE + 1.0 / 256.0;
/// The knob's radius — the 5-12-13 point of the dome circle.
pub const R_KNOB: f64 = 5.0 / 256.0;
/// Where the dome ends and the knob's wall begins.
pub const Y_KNOB: f64 = LID_BASE + 13.0 / 256.0;
/// The knob's top.
pub const Y_TOP: f64 = LID_BASE + 18.0 / 256.0;
/// The roll, one radius for all three of the lid's rims — per
/// REQUEST rather than per edge, and the lid rolls in one request.
pub const ROLL: f64 = 2.0 / 256.0;

// ---------------------------------------------------------------------
// The spout and the handle.
// ---------------------------------------------------------------------

/// The spout's length ALONG ITS SPINE — an arc now, not an axis.
const SPOUT_LEN: f64 = 8.0 / 64.0;
/// The spout's outer radius at the root, and at the tip.
const SPOUT_R0: f64 = 6.0 / 256.0;
const SPOUT_R1: f64 = 3.0 / 256.0;
/// **The bore, as a FRACTION of the outer radius, at every station**
/// — so the wall thins with the spout instead of holding one
/// thickness, which is what a pulled spout does and what a lathe
/// cannot do.
///
/// Load-bearing rather than decorative: a section that scales
/// UNIFORMLY keeps its centroid on the spine, and a centred section is
/// what makes the bend volume-neutral — which is what leaves the
/// straightened frustum as this canal's closed form. A wall of
/// constant absolute thickness would move the centroid off the spine
/// as the section shrank, and the closed form would go with it.
const SPOUT_BORE: f64 = 3.0 / 4.0;
/// How many sections the spout's skin is fitted through.
const SPOUT_STATIONS: usize = 7;
// The stop's `ops` line is a `&'static str` and spells this count.
const _: () = assert!(SPOUT_STATIONS == 7);
/// **How many arcs each circular section is authored as.**
///
/// FOUR, not a plain `LoopProgram::Circle`: a `Circle` loop is two
/// segments, so each lateral wall would span a semicircle — two
/// rational Béziers joined at an interior knot of multiplicity =
/// degree, a C⁰ crease the TESSELLATOR refuses by name. That is
/// `work/mesh`'s
/// `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`,
/// and `CircleSplit(4)` is the door through it.
///
/// **The sections are ROUND, so this body's volume is an ENCLOSURE.**
/// A circle is RATIONAL, so these lateral walls are rational patches,
/// and a rational patch is a QUADRATURE face whose certified enclosure
/// is chased to the tour's printed precision. Where it is met the scene
/// holds the enclosure to the skin's own integrated volume
/// (`patch_quadrature`); where it is not, tier 3 still certifies the
/// body on the enclosure's SIGN, and the scene holds the same reference
/// to the bracket the measurement reached.
const SPOUT_ARCS: u32 = 4;
/// **The spout's total bend**, root tangent to tip tangent: half a
/// right angle. The canal leaves the belly on [`SPOUT_DIR`] and
/// arrives pointing that much further up, which is the shape a spout
/// has and the shape a revolve does not reach.
const SPOUT_BEND: f64 = FRAC_PI_4;
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
// `Formula::literal`, the mouth and the lid's rims are named by ROLE
// rather than found by a numeric scan, and the two unions the scene
// cannot compose are nodes that refuse at `evaluate`.
// ---------------------------------------------------------------------

/// A length in the canonical metres this document is authored in.
fn len(v: f64) -> Formula {
    Formula::literal(v, Dimension::Length).expect("a finite length")
}

/// An angle in radians, the unit this scene's turns are written in.
fn ang(v: f64) -> Formula {
    Formula::literal(v, Dimension::Angle).expect("a finite angle")
}

/// One authored point of a meridian, in the sketch's own coordinates.
fn lpt(x: f64, y: f64) -> [Formula; 2] {
    [len(x), len(y)]
}

/// A straight meridian step to `(x, y)`.
fn line_to(x: f64, y: f64) -> ProgramStep<Formula> {
    ProgramStep::LineTo(ProgramTarget::Point(lpt(x, y)))
}

/// A meridian arc about `(cx, cy)` to `(x, y)`.
fn arc_to(cx: f64, cy: f64, winding: ArcSweep, x: f64, y: f64) -> ProgramStep<Formula> {
    ProgramStep::ArcTo(ProgramArcData::Center {
        c: lpt(cx, cy),
        winding,
        target: ProgramTarget::Point(lpt(x, y)),
    })
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
fn vessel_meridian() -> LoopProgram<Formula> {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        line_to(R_FOOT, 0.0),
        line_to(R_FOOT, Y_FOOT),
        arc_to(0.0, Y_BELLY_C, ArcSweep::Ccw, R_NECK, Y_MOUTH),
        line_to(0.0, Y_MOUTH),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// The mouth disc's segment index in [`vessel_meridian`]'s program
/// order — base disc, foot, belly, MOUTH, axis chord.
const SEG_MOUTH: u32 = 3;

/// The mouth disc's segment index READ OFF the program: a meridian's
/// segments are its steps after the opening `At`, so the mouth disc is
/// the step that runs to the axis at the mouth's own station — and it
/// has to be the only one, or the index would be a choice rather than
/// the program's answer.
fn mouth_segment(program: &LoopProgram<Formula>) -> u32 {
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
fn torus_belly_meridian() -> LoopProgram<Formula> {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, 0.0)),
        line_to(R_FOOT, 0.0),
        line_to(R_FOOT, Y_FOOT),
        arc_to(7.0 / 64.0, 5.0 / 64.0, ArcSweep::Cw, R_NECK, Y_MOUTH),
        line_to(0.0, Y_MOUTH),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// **The lid's meridian**: underside disc, conical flange, dome, knob
/// wall, knob top disc, closed down the axis — a solid knob.
///
/// The flange is what a lid has and is also what puts the CURVED-support
/// fillet family on this part: it mints two latitude rims whose
/// supports are a plane and a cone, and a cone and a sphere. With the
/// knob's cylinder × plane rim that is three DIFFERENT coaxial arms on
/// one body — the three `bud` records, on a part rather than on a study.
///
/// Every station stays dyadic, which is what keeps the arc door's
/// equidistance residuals exactly zero: the flange runs `14/256 → 12/256`
/// over `40/256 → 46/256`, and the dome's centre sits at `41/256` so
/// its two junctions are still the sphere's own 5-12-13 points —
/// `(12, 46)` is `(12, 5)` from the centre and `(5, 53)` is `(5, 12)`.
fn lid_meridian() -> LoopProgram<Formula> {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(0.0, LID_BASE)),
        line_to(R_FLANGE, LID_BASE),
        line_to(R_NECK, Y_FLANGE),
        arc_to(0.0, DOME_C, ArcSweep::Ccw, R_KNOB, Y_KNOB),
        line_to(R_KNOB, Y_TOP),
        line_to(0.0, Y_TOP),
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

/// **The spout's spine, as a list of section PLANES** — built where the
/// canal leaves its root along the spout's `+y` and bends toward `+x`
/// through [`SPOUT_BEND`]. The placement below carries `+y` onto
/// [`SPOUT_DIR`], so the root arrives on the 3-4-5 direction and the tip
/// a half right angle further up.
///
/// The spine is a circular ARC of radius `SPOUT_LEN / SPOUT_BEND`, so
/// the stations are equally spaced along it by construction rather
/// than by a resampling step.
///
/// **Each plane's normal is the spine's tangent, exactly.** Station
/// `a`'s plane stands at `(r(1 − cos a), r sin a, 0)` with axes
/// `u = +z` and `v = (cos a, −sin a, 0)`, whose normal `u × v` is
/// `(sin a, cos a, 0)` — the tangent at station `a`. The first section
/// is drawn on the spout's own `xy` ([`spout_values`] turns it to where the
/// first plane stands); a later one's station is that plane in the
/// first's own axes — a spin of `−a` about its `x`, then slides of
/// `r(1 − cos a)` along its `y` and `r sin a` along its `z` — so a
/// section is perpendicular to the spine because of what the station
/// says, not because a fitting step made it nearly so.
///
/// Returned with each station's OUTER radius.
fn spout_stations() -> Vec<(Vec<StationMove<Formula>>, f64)> {
    let r_spine = SPOUT_LEN / SPOUT_BEND;
    (0..SPOUT_STATIONS)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f64 / (SPOUT_STATIONS - 1) as f64;
            let a = t * SPOUT_BEND;
            let (sa, ca) = a.sin_cos();
            let station = if i == 0 {
                Vec::new()
            } else {
                vec![
                    StationMove::Spin {
                        about: Axis3::X,
                        by: ang(-a),
                    },
                    StationMove::Slide {
                        along: Axis3::Y,
                        by: len(r_spine * (1.0 - ca)),
                    },
                    StationMove::Slide {
                        along: Axis3::Z,
                        by: len(r_spine * sa),
                    },
                ]
            };
            (station, SPOUT_R1.mul_add(t, SPOUT_R0 * (1.0 - t)))
        })
        .collect()
}

/// **The spout, as one `Node::Loft` through those sections.**
///
/// Two loops per section — the outer arc chain and the bore, the bore
/// a fixed FRACTION of the outer at every station. That is what makes
/// the closed form below available at all: a section that scales
/// uniformly keeps its centroid on the spine, and a centred section is
/// what makes a bend volume-neutral.
fn spout_loft(
    doc: &mut Doc<ProfileProgram>,
    stations: &[(Vec<StationMove<Formula>>, f64)],
    tol: Tol,
) -> RecipeNodeId {
    // **Circles, split into [`SPOUT_ARCS`] arcs**, which is the shape a
    // potter draws and the shape this scene is FOR. The split is the
    // tessellator's door (a whole `Circle` loop makes a semicircular
    // wall with a C⁰ crease); the rationality that comes with it is
    // what tier 3 currently cannot certify at ε = 1e-12, and that is a
    // kernel finding this scene exhibits rather than dodges.
    let mut sections = stations.iter().map(|(station, outer)| {
        let arcs = |radius: f64| LoopProgram::CircleSplit {
            centre: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            radius: len(radius),
            n: SPOUT_ARCS,
            phase: Formula::zero(Dimension::Angle),
        };
        let loops = vec![arcs(*outer), arcs(outer * SPOUT_BORE)];
        let profile = insert(
            doc,
            Node::Profile(ProfileProgram {
                loops,
                ids: Vec::new(),
                on: None,
            }),
            tol,
        );
        (profile, station.clone())
    });
    let (first, _) = sections.next().expect("a loft has a first section");
    let then = sections
        .map(|(profile, station)| LoftSection {
            profile: profile.into(),
            station,
        })
        .collect();
    insert(
        doc,
        Node::Loft {
            first: first.into(),
            then,
            v_degree: Formula::count(3),
        },
        tol,
    )
}

/// **The spout's values on the pot's axes**: the spout's own axes
/// turned into the bent frame its spine is described in (`x` onto `+z`,
/// `y` onto `+x`, `z` onto `+y`: a quarter turn back about `x`, then
/// one back about `y`), that frame turned by [`spout_turn`] about `+z`,
/// and slid to [`SPOUT_ROOT`].
fn spout_values() -> Vec<(Coord, Formula)> {
    let mut values = vec![
        (
            Coord::Spin(Some(TargetAxis::X)),
            crate::world::turn(4, true),
        ),
        (
            Coord::Spin(Some(TargetAxis::Y)),
            crate::world::turn(4, true),
        ),
        (Coord::Spin(Some(TargetAxis::Z)), ang(spout_turn())),
    ];
    values.extend(crate::world::slides([
        SPOUT_ROOT.x,
        SPOUT_ROOT.y,
        SPOUT_ROOT.z,
    ]));
    values
}

/// **The handle's values on the pot's axes**: its spine centre slid onto
/// the belly wall, its axes the pot's.
fn handle_values() -> Vec<(Coord, Formula)> {
    crate::world::slides([HANDLE_C.x, HANDLE_C.y, HANDLE_C.z])
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
pub struct Recipe {
    pub doc: Doc<ProfileProgram>,
    /// The pot's revolve — the OPERAND both hollows are taken of, and
    /// the node whose bands name the mouth.
    bellied: RecipeNodeId,
    /// The sealed hollow: a wall and a cavity in ONE solid.
    pot: RecipeNodeId,
    /// The opened hollow — the body the montage draws.
    cup: RecipeNodeId,
    /// The lid before its rims roll.
    plain_lid: RecipeNodeId,
    /// The lid with its three rims rolled in ONE fillet request.
    lid: RecipeNodeId,
    /// The spout about its OWN axis — the node whose bands name the
    /// root annulus the placement is measured on.
    spout_body: RecipeNodeId,
    /// The spout, placed against the pot.
    spout: RecipeNodeId,
    /// The handle in its own coordinates.
    handle_body: RecipeNodeId,
    /// The handle, placed against the pot.
    handle: RecipeNodeId,
    /// The two joins the operand gate has no arm for.
    handle_union: RecipeNodeId,
    spout_union: RecipeNodeId,
}

fn insert(doc: &mut Doc<ProfileProgram>, node: AuthoredNode, tol: Tol) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        tol,
    )
    .expect("the edit applies");
    *doc = applied.doc;
    applied.record.minted.expect("insert mints an id")
}

/// One full revolve of `loop_` about the meridian's own `+y` through
/// its origin: the axis is written in the meridian's 2-D coordinates,
/// so it cannot leave the plane the meridian is drawn on. A meridian
/// point `(x, y)` is the revolve's own point `(x, y, 0)`, so the pot
/// stands on its own `+y` — the placement every camera, cell and
/// budget row of this scene was taken from.
fn revolved(doc: &mut Doc<ProfileProgram>, loop_: LoopProgram<Formula>, tol: Tol) -> RecipeNodeId {
    let profile = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![loop_],
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(TAU),
        },
        tol,
    )
}

/// **The pieces of the meridian the revolve `sweep` turns**, under the
/// document's current values: what its bands, rims and meridians are
/// named by (`editor-core/src/names/README.md`, N1).
fn pieces_of(doc: &Doc<ProfileProgram>, sweep: RecipeNodeId, tol: Tol) -> ProfilePieces {
    let Some(Node::Revolve { profile, .. }) = doc.node(sweep) else {
        panic!("node {} is a revolve", sweep.0);
    };
    let Some(Node::Profile(program)) = doc.operation_of(*profile).and_then(|p| doc.node(p)) else {
        panic!("a revolve's operand is a profile");
    };
    program
        .pieces(&doc.var_env::<f64>(), tol)
        .expect("the meridian replays")
}

/// The piece meridian segment `k` is, on the revolve `sweep`.
fn edge_at(doc: &Doc<ProfileProgram>, sweep: RecipeNodeId, k: u32, tol: Tol) -> ProfileEdgeRef {
    pieces_of(doc, sweep, tol)
        .edge(0, k as usize)
        .expect("the segment is the meridian's")
}

/// The piece starting at meridian vertex `v`, on the revolve `sweep`.
fn vertex_at(doc: &Doc<ProfileProgram>, sweep: RecipeNodeId, v: u32, tol: Tol) -> ProfileVertexRef {
    pieces_of(doc, sweep, tol)
        .vertex(0, v as usize)
        .expect("the vertex is the meridian's")
}

/// **The latitude rim at `vertex`, WHOLE**: on a full revolve of a
/// profile that touches the axis, a rim is two half-arcs meeting at
/// two seam vertices, named `BandRim` (`[0, π)`) and `BandRimPi`
/// (`[π, 2π)`). The fillet takes the pair; either arc alone ends at a
/// seam vertex and refuses `UnsupportedCorner`.
fn rim_arcs(node: RecipeNodeId, vertex: ProfileVertexRef) -> [StableName; 2] {
    [band_rim(node, vertex), band_rim_pi(node, vertex)]
}

fn build_doc(tol: Tol) -> Recipe {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("teapot", tol);

    // ---- the vessel ----
    let bellied = revolved(&mut doc, vessel_meridian(), tol);
    // The mouth is the meridian's mouth-disc segment, NAMED. A full
    // revolve sweeps a planar wall whole — no seam meridian cuts it —
    // so the mouth disc is ONE face, its `Band`.
    let lip = edge_at(&doc, bellied, SEG_MOUTH, tol);
    let mouth = vec![band(bellied, lip)];
    let pot = insert(
        &mut doc,
        Node::shell(
            pncad::document::Operand::output(bellied, 0),
            len(WALL),
            Vec::new(),
        ),
        tol,
    );
    let cup = insert(
        &mut doc,
        Node::shell(
            pncad::document::Operand::output(bellied, 0),
            len(WALL),
            mouth,
        ),
        tol,
    );

    // ---- the lid ----
    let plain_lid = revolved(&mut doc, lid_meridian(), tol);
    // THREE rims, THREE DIFFERENT coaxial arms, in ONE request. The
    // radius is per REQUEST, not per edge, and each later rim's
    // seam-piece identities are re-read against the partially-carved
    // body, so the convenient spelling is the door's grain.
    //
    // The lid's profile touches the axis, so each rim is two half-arcs
    // and is asked for whole ([`rim_arcs`]); the band carves over both.
    //
    // The flange's rim and the dome's foot stand at the two ends of the
    // flange cone, so both bands slit and cross THAT segment's meridians;
    // their names tell the two apart by the band that made each.
    let rims: Vec<StableName> = LID_RIMS
        .iter()
        .flat_map(|&(v, ..)| rim_arcs(plain_lid, vertex_at(&doc, plain_lid, v, tol)))
        .collect();
    let lid = insert(
        &mut doc,
        Node::fillet(
            pncad::document::Operand::output(plain_lid, 0),
            len(ROLL),
            rims,
        ),
        tol,
    );

    // ---- the spout: a CANAL lofted about its own bent spine, then
    // placed against the pot: the pot offers no planar face on its own
    // `xy`, so the spout's root is mated to a frame read off the pot.
    let stations = spout_stations();
    let spout_body = spout_loft(&mut doc, &stations, tol);
    let placed = crate::world::against(
        &doc,
        pncad::document::Operand::output(bellied, 0),
        spout_body,
        spout_values(),
        tol,
    );
    doc = placed.doc;
    let spout = placed.placement;

    // ---- the handle ----
    // Bent about its own `z` through its own origin, its window starting
    // from its own `x`; placed against the pot with its centre on the
    // belly wall.
    let handle_body = insert(
        &mut doc,
        Node::Tube {
            major_radius: len(HANDLE_R),
            window: TubeWindow::Arc {
                t0: ang(-(FRAC_PI_2 + HANDLE_OVER)),
                t1: ang(FRAC_PI_2 + HANDLE_OVER),
            },
            minor_radius: len(HANDLE_TUBE),
        },
        tol,
    );
    // Placed into the pot's space with its own frame at the handle's
    // pose there.
    let placed = crate::world::against(
        &doc,
        pncad::document::Operand::output(bellied, 0),
        handle_body,
        handle_values(),
        tol,
    );
    doc = placed.doc;
    let handle = placed.placement;

    // ---- the two joins, as document requests ----
    let union_node = |doc: &mut Doc<ProfileProgram>, b| {
        insert(
            doc,
            Node::Union {
                members: Bodies::Spelled(vec![cup.into(), b]),
            },
            tol,
        )
    };
    let handle_union = union_node(&mut doc, handle.into());
    let spout_union = union_node(&mut doc, spout.into());

    Recipe {
        doc,
        bellied,
        pot,
        cup,
        plain_lid,
        lid,
        spout_body,
        spout,
        handle_body,
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
    let belly = revolved(&mut doc, torus_belly_meridian(), tol);
    let hollow = insert(
        &mut doc,
        Node::shell(
            pncad::document::Operand::output(belly, 0),
            len(WALL),
            Vec::new(),
        ),
        tol,
    );
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    body_at(&ev, hollow, tol)
}

/// **This scene's recipe, as a document the GUI can open** — the same
/// [`build_doc`] the stops walk, so the gallery cannot be a second
/// authoring of the scene.
///
/// The product is the four bodies the montage shows — the handle, the
/// lid, the cup and the spout — each placed in the world once. Nothing
/// places the SEALED hollow, the same operand and wall as the cup, so
/// it is not in the product. It, the two refusing unions, which
/// produce no body, and the placements of the handle and the spout
/// against the pot that only the unions read are deleted as well, so
/// the file holds what the montage shows, no refusal and nothing
/// unused.
///
/// They interpenetrate, and the file says so: the handle's roots are
/// driven through the belly wall and the spout's root disc sits inside
/// it, which is what makes the two unions real requests. A separation
/// finding on this document is the scene's own subject arriving
/// through the checks registry.
pub fn gallery_document(tol: Tol) -> Doc<ProfileProgram> {
    let r = build_doc(tol);
    let tidied = [r.handle_union, r.spout_union, r.pot, r.handle, r.spout]
        .into_iter()
        .fold(r.doc, |doc, id| {
            apply(&doc, &DocEdit::DeleteNode { id }, tol)
                .expect("nothing reads it, so it deletes")
                .doc
        });
    let mut copies = Vec::new();
    let placed = [
        (r.handle_body, handle_values()),
        (r.lid, Vec::new()),
        (r.cup, Vec::new()),
        (r.spout_body, spout_values()),
    ]
    .into_iter()
    .fold(tidied, |doc, (body, values)| {
        let placed = crate::world::in_world_by(&doc, body, values, tol);
        copies.push(pncad::document::Operand::Node(placed.placement));
        placed.doc
    });
    // What the pot is: its four copies.
    crate::world::set_product(&placed, &copies, tol).doc
}

/// The body a node evaluated to.
fn body_at(ev: &Evaluation<f64>, id: RecipeNodeId, tol: Tol) -> Body<f64> {
    let value = ev
        .value(id)
        .unwrap_or_else(|| panic!("node {id:?} evaluated to a body, got {}", describe(ev, id)));
    match &value.payload {
        ValuePayload::Body(b) => crate::world::in_its_space(b, tol),
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

/// **Every band face `node`'s blend minted** — the lid rolls in one
/// request, so each of its bands is that request's own mint.
fn band_faces(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<StableName> {
    faces_where(ev, node, SegPat::tag(SegTag::BandFace))
}

/// **A named rim's own circle, read back OFF THE EDGES THE NAME
/// DENOTES**: its centre station and its radius.
///
/// The direction matters and is the whole strength of the row. A
/// numeric scan matched a DESCRIPTION — station and radius — and
/// answered a key; this goes the other way, from the name to the one
/// edge that carries it (`edge_name` over the body's edges is the
/// only door the façade has for that direction), out to the whole rim
/// through `query::rim_of`, and then reads the circle off each of
/// THOSE edges' own certified carriers. So a name that resolved to a
/// different rim standing at the same station cannot answer the right
/// radius: the radius is the edges', not a neighbouring vertex's.
///
/// The same read serves an annular profile (a rim of one closed edge)
/// and an axis-touching one (two half-arcs, `BandRim` and
/// `BandRimPi`). What is asserted, each a way the tie could be false:
/// the `BandRim` name denotes EXACTLY ONE edge; the rim through it is
/// named by this vertex's rim names and no others; every edge of it
/// stands on one circle centred on the axis; and the meridian VERTEX of
/// the same profile vertex lies ON that circle — which is what makes
/// the rim and the vertex two names for one place rather than two
/// independent reads that happen to agree.
fn rim_circle(
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
    body: &Body<f64>,
    vertex: u32,
) -> (f64, f64) {
    let start = vertex_at(doc, node, vertex, Tol::witness());
    let names = rim_arcs(node, start);
    let seeds: Vec<_> = query::all_edges(body)
        .into_iter()
        .filter(|&k| edge_name(ev, node, 0, k).ok() == Some(&names[0]))
        .collect();
    let [seed] = seeds[..] else {
        panic!("the rim's BandRim name denotes exactly one edge, got {seeds:?}");
    };
    let rim = query::rim_of(body, seed).expect("a latitude rim chains into one closed rim");
    let circles: Vec<(f64, f64)> = rim
        .iter()
        .map(|&k| {
            let name = edge_name(ev, node, 0, k).expect("every rim edge is named");
            assert!(
                names.contains(name),
                "the rim through {:?} carries an edge named {name:?}, not one of this \
                 vertex's rim names",
                names[0]
            );
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
    let (station, radius) = circles[0];
    assert!(
        circles.iter().all(|&c| c == (station, radius)),
        "every edge of one rim stands on one circle: {circles:?}"
    );
    let p = vertex_position(ev, node, &meridian_vertex(MeridianEnd::Seam, node, start))
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

/// **A join's answer, as one line for the panel's note.**
///
/// The germ PAIR the gate named, rendered from the payload rather
/// than dumped: `Debug` on a kernel refusal puts Rust field names and
/// arena keys into prose a person reads, and the keys are not the
/// evidence anyway — they are the operand's arena numbering, which
/// moves when the model is re-authored (the module docs' finding 4).
/// What IS the evidence is the pair, which is exactly what the
/// wall-7 lesson is about, and it survives verbatim.
fn describe_join(ev: &Evaluation<f64>, node: RecipeNodeId) -> String {
    match join_outcome(ev, node) {
        Ok(()) => "COMPOSED".to_owned(),
        Err(BooleanError::CurvedPairUnsupported {
            operand,
            kind,
            other_kind,
            ..
        }) => format!(
            "CurvedPairUnsupported, operand {operand:?}: {} x {}",
            kind.name(),
            other_kind.name()
        ),
        Err(other) => format!("{other}"),
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

/// The `n`-point Gauss–Legendre rule on `[-1, 1]`, as `(node, weight)`:
/// Newton on the three-term recurrence from Tricomi's initial guess.
fn gauss_legendre(n: usize) -> Vec<(f64, f64)> {
    let nf = n as f64;
    (1..=n)
        .map(|k| {
            let mut x = (PI * (k as f64 - 0.25) / (nf + 0.5)).cos();
            let mut dp = 0.0;
            for _ in 0..100 {
                let (mut p0, mut p1) = (1.0, x);
                for j in 2..=n {
                    let jf = j as f64;
                    (p0, p1) = (p1, ((2.0 * jf - 1.0) * x * p1 - (jf - 1.0) * p0) / jf);
                }
                dp = nf * (x * p1 - p0) / (x * x - 1.0);
                let dx = p1 / dp;
                x -= dx;
                if dx.abs() <= 1e-16 {
                    break;
                }
            }
            (x, 2.0 / ((1.0 - x * x) * dp * dp))
        })
        .collect()
}

/// **The volume of a body whose faces are two planes and any number of
/// NURBS patches, and the area of its patches**, integrated over the
/// body's OWN surfaces with an `order`-point Gauss–Legendre rule on
/// every knot span: the reference a fitted skin has where it has no
/// closed form.
///
/// The volume is the divergence theorem, `V = ⅓ ∮ (x − p)·n dA`, with
/// `p` a point on BOTH planes: `(x − p)·n` vanishes on each, so the
/// caps contribute nothing and only the patches are integrated. Every
/// span of a patch is a smooth rational piece, where the rule converges
/// geometrically in `order`.
fn patch_quadrature(body: &Body<f64>, order: usize) -> (f64, f64) {
    let rule = gauss_legendre(order);
    let planes: Vec<(Point3<f64>, Vec3<f64>)> = body
        .faces()
        .filter_map(|(_, f)| match body.get_surface(f.surface) {
            Some(Surface::Plane { origin, normal, .. }) => Some((*origin, *normal)),
            _ => None,
        })
        .collect();
    let [(o1, n1), (o2, n2)] = planes[..] else {
        panic!("a quadrature over two caps, got {} planes", planes.len())
    };
    // The point of the two planes' common line nearest the origin.
    let (c, d1, d2) = (
        n1.dot(n2),
        n1.dot(o1 - Point3::origin()),
        n2.dot(o2 - Point3::origin()),
    );
    let p = Point3::origin()
        + n1 * ((d1 - c * d2) / (1.0 - c * c))
        + n2 * ((d2 - c * d1) / (1.0 - c * c));
    let breaks = |knots: &[f64]| {
        let mut b = knots.to_vec();
        b.dedup();
        b
    };
    let (mut flux, mut area) = (0.0, 0.0);
    for (_, f) in body.faces() {
        let n = match body.get_surface(f.surface) {
            Some(Surface::Plane { .. }) => continue,
            Some(Surface::Nurbs(n)) => n,
            other => panic!("a quadrature over planes and NURBS, got {other:?}"),
        };
        let sense = if f.sense { 1.0 } else { -1.0 };
        let (bu, bv) = (
            breaks(n.knot_values_u().values()),
            breaks(n.knot_values_v().values()),
        );
        for u in bu.windows(2) {
            for v in bv.windows(2) {
                let (hu, hv) = (0.5 * (u[1] - u[0]), 0.5 * (v[1] - v[0]));
                for &(xu, wu) in &rule {
                    for &(xv, wv) in &rule {
                        let jet = n.ders(u[0] + hu * (1.0 + xu), v[0] + hv * (1.0 + xv));
                        let normal = jet.du.cross(jet.dv) * sense;
                        let w = wu * wv * hu * hv;
                        flux += w * (jet.point - p).dot(normal);
                        area += w * normal.norm();
                    }
                }
            }
        }
    }
    (flux / 3.0, area)
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

/// A body's `(vertices, edges, faces)`.
type Census = (usize, usize, usize);

/// **Each of the lid's three rims asked for ON ITS OWN**, in a
/// document of its own: the answer printed, and its census returned
/// for the scene to hold each band to its own delta.
///
/// A one-call refusal names ONE edge, and on a body where three rims
/// are new that is not enough to say which arm the door turned away —
/// so the scene asks three questions whose answers are each about one
/// rim. They are asked on a SEPARATE document because the scene's own
/// recipe is what the gallery opens, and three extra half-rolled lids
/// in it would be three bodies the scene does not model.
fn per_rim_answers(tol: Tol) -> Vec<(&'static str, String, Option<Census>)> {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("teapot-lid-rims", tol);
    let lid = revolved(&mut doc, lid_meridian(), tol);
    let rims: Vec<[StableName; 2]> = LID_RIMS
        .iter()
        .map(|&(v, ..)| rim_arcs(lid, vertex_at(&doc, lid, v, tol)))
        .collect();
    let asked: Vec<(&'static str, RecipeNodeId)> = LID_RIMS
        .iter()
        .zip(&rims)
        .map(|(&(_, _, _, what), rim)| {
            (
                what,
                insert(
                    &mut doc,
                    Node::fillet(
                        pncad::document::Operand::output(lid, 0),
                        len(ROLL),
                        rim.to_vec(),
                    ),
                    tol,
                ),
            )
        })
        .collect();
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    asked
        .into_iter()
        .map(|(what, node)| {
            let census = ev.value(node).and_then(|v| match &v.payload {
                ValuePayload::Body(b) => Some((
                    b.body().vertices().count(),
                    b.body().edges().count(),
                    b.body().faces().count(),
                )),
                _ => None,
            });
            (what, describe(&ev, node), census)
        })
        .collect()
}

/// **The teapot's two walls** — the handle union and the spout union,
/// each ATTEMPTED for real on every pass and pinned by its own typed
/// refusal, in the shape `lily::wall_probes` and `klein::wall_probes`
/// carry theirs.
///
/// It takes the scene's evaluation rather than building one of its
/// own: each union is a NODE of the document and refuses at
/// `evaluate`, with the kernel's payload carried unaltered into the
/// panel's note — so the payload the caption quotes and the payload
/// the probe pins cannot be two different measurements. That is why
/// this one takes an evaluation where `torusvessel::wall_probes` takes
/// only a tolerance and builds its own operands: nothing outside that
/// scene's probe reads what it makes.
pub fn wall_probes(ev: &Evaluation<f64>, r: &Recipe) {
    // WALL 2 — the handle joined to the pot. The handle's torus is on
    // the union's KIND roster, so the gate has nothing to say about
    // it. The join used to stop one door on, at the maximal-faces
    // precondition, on the VESSEL, whose full revolve minted split
    // planar walls; it sweeps them whole now (BAND, one wall per run),
    // so the request reaches the vessel's sphere face meeting the
    // handle's torus. The section value carries that pair's loops, the
    // segment lays them, and the union builds; its result's volume is not
    // computed (a conic trim on the sphere's chart has no quadrature yet),
    // so the result is invalid.
    crate::walls::wall(
        "teapot",
        2,
        "join the handle to the vessel (union; both roots driven 11.2 mm past the \
         belly's inner wall — a real overlap, not a tangency)",
        join_outcome(ev, r.handle_union),
        |e| {
            matches!(
                e,
                BooleanError::ResultInvalid { errors } if matches!(
                    errors.as_slice(),
                    [pncad::topo::ValidationError::VolumeUncomputable {
                        source: pncad::topo::MassPropsError::Face {
                            source: pncad::geom_brep::PropsError::QuadratureUnsupported { what },
                            ..
                        },
                        ..
                    }] if what.starts_with("conic trim on a cone/sphere/torus chart")
                )
            )
        },
        "make the teapot ONE solid: union the handle and the spout into the vessel, drop \
         walls 2 and 3, re-state the montage caption (which currently says four solids), \
         and RE-CUT THE HANDLE'S OVERSHOOT FIRST — at 0.5 rad its roots stand 11.2 mm \
         inside the cavity, which is fine for a refused request and wrong for a joined one",
    );

    // WALL 3 — the spout joined to the pot, and it is a DIFFERENT RUNG
    // of the gate from wall 2 now that the spout is a canal.
    //
    // The frustum's walls were cones, so this used to die where wall 2
    // dies: `CurvedPairUnsupported`, on a face-kind pair with no arm.
    // A loft's walls are `Nurbs` and the pair gate HAS an arm for
    // `Nurbs`, so the request gets past that rung and into the sweep,
    // which meets the belly's revolve seam, a quarter meridian on which
    // the spout's axis leaves the belly, with one of the spout's NURBS
    // walls. The sweep reads the meridian against that wall's boundary
    // first, and the meridian meets it there: the meridian and the
    // spout's seams lie in the pot's one plane of symmetry, so where
    // the meridian pierces the wall it crosses a seam. A meeting on a
    // boundary edge splits both edges, and a spline seam has no point
    // parameter to split at
    // (`work/splice/a-meeting-on-a-spline-boundary-edge-has-no-split-parameter`).
    //
    // The predicate matches on the operand and the class, deliberately
    // not the edge: an arena key moves whenever the model is re-authored,
    // and a probe that pinned one would red on a rename. What is pinned
    // is the class and the side: an edge of operand B, the spout.
    crate::walls::wall(
        "teapot",
        3,
        "join the CANAL to the vessel (union; the root disc wholly inside the belly)",
        join_outcome(ev, r.spout_union),
        |e| {
            matches!(
                e,
                BooleanError::PointSplitCarrierUnsupported {
                    operand: Operand::B,
                    ..
                }
            )
        },
        "make the teapot ONE solid: union the handle and the spout into the vessel, drop \
         walls 2 and 3, re-state the montage caption (which currently says four solids), \
         and RE-CUT THE HANDLE'S OVERSHOOT FIRST — at 0.5 rad its roots stand 11.2 mm \
         inside the cavity, which is fine for a refused request and wrong for a joined one",
    );
}

/// The scene's recipe, built and evaluated. Both the render walk and
/// the wall-probe test (`tests/teapot.rs`) go through here, so the probes are pinned
/// against the evaluation the scene actually draws from rather than a
/// second one configured by hand.
pub fn evaluated(tol: Tol) -> (Recipe, Evaluation<f64>) {
    let r = build_doc(tol);
    let ev = evaluate::<f64>(
        &r.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    (r, ev)
}

pub fn stops(tol: Tol) -> Vec<TourStop> {
    let (r, ev) = evaluated(tol);

    // ---- the vessel, before the wall ----
    let bellied = body_at(&ev, r.bellied, tol);
    assert_eq!(
        (
            bellied.vertices().count(),
            bellied.edges().count(),
            bellied.faces().count(),
        ),
        (6, 10, 6),
        "four revolved meridian segments (the fifth is the axis and sweeps nothing): \
         the two curved walls each cut at the two seam meridians into a pair of \
         half-walls, the two planar discs swept whole — one arc where the squared pot \
         spent three segments on shoulder, belly, shoulder"
    );
    // The mouth disc's segment index is read off the PROGRAM rather
    // than transcribed: it is the segment whose meridian step lands on
    // the mouth plane at the axis.
    assert_eq!(
        mouth_segment(&vessel_meridian()),
        SEG_MOUTH,
        "the mouth disc is segment {SEG_MOUTH} of the meridian in program order"
    );
    // THE MOUTH, BY NAME. One disc on its plane — the name the shell
    // node was authored with, asserted to denote exactly that: a planar
    // face on the mouth's own station.
    let lip = edge_at(&r.doc, r.bellied, SEG_MOUTH, tol);
    let mouth = [band(r.bellied, lip)];
    for name in &mouth {
        assert_eq!(
            face_carrier_kind(&ev, r.bellied, name).expect("the mouth is named"),
            SurfaceKind::Plane,
            "the mouth disc's carrier is a plane"
        );
        let origin = face_frame(&ev, r.bellied, name)
            .expect("the mouth is named")
            .origin;
        assert!(
            (origin.y - Y_MOUTH).abs() < 1e-12,
            "the mouth stands on the mouth plane: got {origin:?}"
        );
    }
    // The CHART is that face and nothing else, said in both directions:
    // the name denotes one face, and the operand has exactly one planar
    // face on the mouth's station, so a designation of it is a
    // designation of the whole chart (which is what `shell_open`
    // requires and what `OpenFaceChartPartial` refuses).
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
        (1, 1),
        "the mouth is one PLANE worn by one disc face — a full revolve sweeps a planar \
         wall whole — and the name the shell node carries is that face"
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
                let outward = pncad::geom_brep::OutwardNormal::from_chart(*normal, f.sense);
                Some((origin.y, outward.vec().y))
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
    let pot = body_at(&ev, r.pot, tol);
    let (pv, pe, pf) = (
        pot.vertices().count(),
        pot.edges().count(),
        pot.faces().count(),
    );
    assert_eq!(
        (pv, pe, pf),
        (12, 20, 12),
        "the operand's 6/10/6, twice: the cavity is the same boundary offset inward, \
         inserted whole through the shared void door"
    );
    assert_eq!(
        pot.shells().count(),
        2,
        "the outer boundary and the cavity, in ONE solid"
    );
    assert_eq!(
        euler_counts(&pot).genus(),
        Ok(0),
        "two sphere-like shells, no handles"
    );

    // The wall as a NUMBER, against the two stacks — and the cavity's
    // capacity asked for DIRECTLY rather than inferred from a
    // difference, which is the number a potter actually wants.
    let v_out = pot_volume(0.0);
    let v_cav = pot_volume(WALL);
    let v_want = v_out - v_cav;
    let a_want = pot_area(0.0) + pot_area(WALL);
    // Finished once: the props and the shell roles below both read it.
    let pot_at_rest = crate::booleans::at_rest(&pot, tol);
    let props =
        pncad::topo::mass_properties(&pot_at_rest, tol, crate::printed()).expect("the pot's props");
    assert!(
        ((props.enclosure.volume - v_want) / v_want).abs() < 1e-12,
        "pot V = {} vs the wall's own closed form {v_want}",
        props.enclosure.volume
    );
    assert!(
        ((props.enclosure.surface_area - a_want) / a_want).abs() < 1e-12,
        "pot A = {} vs the closed form {a_want}",
        props.enclosure.surface_area
    );
    assert_eq!(props.enclosure.volume_pad, 0.0, "closed forms need no pad");
    let classes =
        pncad::topo::classify_shells(&pot_at_rest, tol).expect("per-shell classification");
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
        ((voids[0].enclosure.volume + v_cav) / v_cav).abs() < 1e-12,
        "the pot holds {} m³ against the cavity stack's own {v_cav}, negated by the \
         orientation convention",
        voids[0].enclosure.volume
    );
    let outers: Vec<_> = classes
        .iter()
        .filter(|c| c.role == pncad::topo::ShellRole::Outer)
        .collect();
    assert_eq!(outers.len(), 1, "one outer boundary");
    assert!(
        ((outers[0].enclosure.volume - v_out) / v_out).abs() < 1e-12,
        "the outer shell encloses {} m³ against the outer stack's {v_out}",
        outers[0].enclosure.volume
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
    // mouth's name for the other.
    let cup = body_at(&ev, r.cup, tol);

    // ---- the lid ----
    let plain_lid = body_at(&ev, r.plain_lid, tol);
    assert_eq!(
        (
            plain_lid.vertices().count(),
            plain_lid.edges().count(),
            plain_lid.faces().count(),
        ),
        (8, 14, 8),
        "an axis-touching profile mints half-walls: the two discs whole, the cone, sphere \
         and cylinder as two faces each; four latitude rims of two half-arcs each, and two \
         meridians per curved wall"
    );
    // THE THREE RIMS, BY NAME — and the name checked against the
    // DESCRIPTION a scan would have matched on, which is the station
    // and the radius the meridian authored. That equality is what
    // retires the scan: the role names the same circle, and it names
    // it through a rebuild rather than through a coordinate.
    for &(v, radius, station, what) in &LID_RIMS {
        let (y, rho) = rim_circle(&r.doc, &ev, r.plain_lid, &plain_lid, v);
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
    //
    // Each band's own census delta is asserted here, one rim at a
    // time, so the one-request total below is the sum of THESE deltas
    // and not a mix that happens to reach it. A band adds a face, two
    // trimline feet and their edges; on a whole planar disc the foot the
    // slit does not reach has valence two, so the blend's closing join
    // takes it (+1, +2, +1). On a half-wall every foot sits on a seam
    // meridian and stays (+2, +3, +1).
    for ((what, answer, census), want) in
        per_rim_answers(tol)
            .into_iter()
            .zip([(9, 16, 9), (10, 17, 9), (9, 16, 9)])
    {
        println!("   {what}: {answer}");
        assert_eq!(
            census,
            Some(want),
            "{what}, rolled alone, is one band over its two half-arcs on the sharp 8/14/8"
        );
    }
    // THREE rims, THREE DIFFERENT coaxial arms. The lid is
    // the tour's carrier of the curved-support fillet family now that
    // `bud` is off the sheet: the flange rim is cone × plane(⊥), the
    // dome's foot is sphere × cone, and the knob's top is
    // cylinder × plane. Each is a latitude circle of two half-arcs, and
    // each band carves over both.
    //
    // The radius is per REQUEST rather than per edge, and each later
    // rim's seam-piece identities are re-read against the
    // partially-carved body.
    let rolled = body_at(&ev, r.lid, tol);
    assert_eq!(
        (
            rolled.vertices().count(),
            rolled.edges().count(),
            rolled.faces().count(),
        ),
        (12, 21, 11),
        "three bands, each over a rim's two half-arcs, each the delta it adds alone"
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
                "a band is ring-free: one cycle over its two half-arcs' trims and the \
                 meridians between them"
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
        + stack_volume(&[(R_KNOB, Y_TOP - Y_KNOB)]);
    let a_lid = a_dome
        + frustum_lateral(R_FLANGE, R_NECK, flange_h)
        + 2.0 * PI * R_KNOB * (Y_TOP - Y_KNOB)
        + PI * (R_KNOB * R_KNOB + R_FLANGE * R_FLANGE);
    let unit = 1.0_f64 / 256.0;
    let (v_note, a_note) = (
        5369.0 * PI / 3.0 * unit.powi(3),
        (453.0 + 52.0 * 10f64.sqrt()) * PI * unit * unit,
    );
    assert!(
        ((v_lid - v_note) / v_note).abs() < 1e-14 && ((a_lid - a_note) / a_note).abs() < 1e-14,
        "the note narrates V = 5369π/3·256⁻³ = {v_note} and A = (453 + 52√10)π·256⁻² = \
         {a_note}; the stations now give {v_lid} and {a_lid}"
    );
    let sharp_lid_props = pncad::topo::mass_properties(
        &crate::booleans::at_rest(&plain_lid, tol),
        tol,
        crate::printed(),
    )
    .expect("the lid's props");
    assert!(
        ((sharp_lid_props.enclosure.volume - v_lid) / v_lid).abs() < 1e-12,
        "lid V = {} vs the closed form {v_lid}",
        sharp_lid_props.enclosure.volume
    );
    assert!(
        ((sharp_lid_props.enclosure.surface_area - a_lid) / a_lid).abs() < 1e-12,
        "lid A = {} vs the closed form {a_lid}",
        sharp_lid_props.enclosure.surface_area
    );
    let lid_props = pncad::topo::mass_properties(
        &crate::booleans::at_rest(&rolled, tol),
        tol,
        crate::printed(),
    )
    .expect("the rolled lid");
    let dv_lid = sharp_lid_props.enclosure.volume - lid_props.enclosure.volume;
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
    // The placement is a mate to a frame read off the pot plus values
    // taken by `atan2` and projection, and the 3-4-5 turn is not a
    // binary-exact angle, so what the residual costs is MEASURED — off
    // the placed BODY, never re-derived from the same trigonometry the
    // placement used. The subjects are the loft's caps, named at the
    // loft and read at the placement, which carries them as they are.
    //
    // TWO subjects, the root annulus and the tip one, and the guard
    // below is why there are two and why neither is the spout's own
    // origin: a point on the rotation's fixed axis maps to `R·0 + t`
    // for any angle whatsoever, so a residual read there measures the
    // translation and would be 0 with the turn deleted. These two
    // stand off that axis and one of them stands a whole spout-length
    // up it, so both residuals are about the turn.
    //
    // The two subjects are the loft's own CAPS, which is a better
    // handle than the frustum's meridian bands were: a loft names its
    // two ends `Cap(Start)` and `Cap(End)` outright, so the name asks
    // for the end rather than for a segment index that a re-ordered
    // meridian could move under it. Both are annuli, because the
    // sections are.
    let one_cap = |node: RecipeNodeId, end: CapEnd| -> StableName {
        let mut found = faces_where(&ev, node, SegPat::tag(SegTag::Cap).side(end));
        assert_eq!(
            found.len(),
            1,
            "a loft has exactly one {end:?} cap; got {found:?}"
        );
        found.remove(0)
    };
    let root_name = one_cap(r.spout_body, CapEnd::Start);
    let tip_name = one_cap(r.spout_body, CapEnd::End);
    let tip_unplaced =
        face_frame(&ev, r.spout_body, &tip_name).expect("the tip annulus, before the placement");
    let unplaced =
        face_frame(&ev, r.spout_body, &root_name).expect("the root annulus, before the placement");
    // A copy carries its source's names as they are.
    let copied = |name: &StableName| name.clone();
    let tip_placed =
        face_frame(&ev, r.spout, &copied(&tip_name)).expect("the tip annulus, after it");
    let placed = face_frame(&ev, r.spout, &copied(&root_name)).expect("the root annulus, after it");
    // WHICH cap is the root is read rather than assumed: the spout is
    // built with station 0 AT its own origin, so the root cap is the
    // one standing there and the tip cap is a whole spine-length away.
    assert!(
        Vec3::new(unplaced.origin.x, unplaced.origin.y, unplaced.origin.z).norm() <= 1e-15,
        "`Cap(Start)` is station 0, which sits at the spout's own origin; it is at {:?}",
        unplaced.origin
    );
    // The EXACT image of that face's own frame under the placement the
    // authored direction states — the 3-4-5 turn written as the matrix
    // whose columns are the direction's own components, which is what
    // this scene used to hand `transform_rigid` and is independent of
    // the angle the document stores.
    // The spout's own point `p` in the bent frame its spine is
    // described in ([`spout_values`]), turned.
    let turn = |p: Vec3<f64>| {
        Vec3::new(SPOUT_DIR.y, -SPOUT_DIR.x, 0.0) * p.x + SPOUT_DIR * p.y + Vec3::unit_z() * p.z
    };
    let bent = |p: Point3<f64>| Vec3::new(p.y, p.z, p.x);
    let root_exact =
        turn(bent(unplaced.origin)) + Vec3::new(SPOUT_ROOT.x, SPOUT_ROOT.y, SPOUT_ROOT.z);
    let root_residual =
        (Vec3::new(placed.origin.x, placed.origin.y, placed.origin.z) - root_exact).norm();
    let tip_exact =
        turn(bent(tip_unplaced.origin)) + Vec3::new(SPOUT_ROOT.x, SPOUT_ROOT.y, SPOUT_ROOT.z);
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
    // **WHERE THE EVIDENCE COMES FROM MOVED WITH THE SHAPE, and the
    // guard that used to hold both subjects is why.** A point on the
    // rotation's fixed set — the `+z` axis through the world origin —
    // maps to `R·0 + t = t` for ANY angle, so a residual read there
    // measures the translation and says nothing about the turn. The
    // frustum's root BAND stood off that axis; the loft's root CAP is
    // the section at station 0, whose frame origin is the spine's own
    // start, and the spout is built with that start AT its origin. So
    // the root cap is exactly the degenerate subject the guard was
    // written to exclude, and the receipt is re-cut rather than the
    // guard weakened:
    //
    //   * POSITION evidence comes from the TIP cap alone, which stands
    //     a whole spine-length off the axis;
    //   * ORIENTATION evidence comes from BOTH caps' normals, and a
    //     translation cannot rotate a normal — so the root cap still
    //     carries turn evidence, of a kind its position could not.
    let off_axis = bent(tip_unplaced.origin)
        .x
        .hypot(bent(tip_unplaced.origin).y);
    assert!(
        off_axis > 1e-3,
        "the tip annulus stands at {:?}, on the rotation's own fixed axis — a residual \
         read there measures the translation and not the turn",
        tip_unplaced.origin
    );
    // The root cap's residual is still asserted, and it is now about
    // the TRANSLATION: `turn(0) + t` is `t`, so this says the placed
    // root sits exactly on SPOUT_ROOT.
    //
    // A BOUND rather than a bit, on both residuals, and the bound is
    // the argument: the angle's cosine and sine are a libm's answer,
    // so an equality here would gate this scene on one platform's last
    // ulp (the reason this workspace routes `erf` through the `libm`
    // crate). What is observed goes in the note; what is ASSERTED is
    // that the placed face stands where the direction says to within a
    // ulp of the spout's own scale.
    assert!(
        root_residual <= 1e-15,
        "the placed root annulus stands at {:?}; the direction's own matrix puts it at \
         {root_exact:?} ({root_residual:e} m away)",
        placed.origin
    );
    // And it FACES the way the direction says: the root section's
    // normal is the spine's tangent at station 0, which the sketch
    // frames put on the spout's own `+v` — so its image is SPOUT_DIR,
    // and a rotation is the only thing that could have put it there.
    let axis_cross = placed.axis.cross(SPOUT_DIR).norm();
    let axis_dot = placed.axis.dot(SPOUT_DIR).abs();
    assert!(
        axis_cross <= 1e-15 && (axis_dot - 1.0).abs() <= 1e-15,
        "the placed root annulus's normal is {:?}, which is not SPOUT_DIR ({SPOUT_DIR:?}): \
         cross {axis_cross:e}, |dot| {axis_dot}",
        placed.axis
    );
    // **The TIP's normal measures the BEND as well as the turn.** The
    // tip section's normal is the tangent at [`SPOUT_BEND`], so its
    // image under the placement is a direction neither the placement
    // nor the spine determines alone — which is what makes this the
    // one reading on the sheet that would move if either did.
    let (sb, cb) = SPOUT_BEND.sin_cos();
    let tip_axis_exact = turn(Vec3::new(sb, cb, 0.0));
    let tip_cross = tip_placed.axis.cross(tip_axis_exact).norm();
    let tip_dot = tip_placed.axis.dot(tip_axis_exact).abs();
    assert!(
        tip_cross <= 1e-15 && (tip_dot - 1.0).abs() <= 1e-15,
        "the placed tip annulus's normal is {:?}, and the bend and the turn together put \
         it at {tip_axis_exact:?}: cross {tip_cross:e}, |dot| {tip_dot}",
        tip_placed.axis
    );
    // The observed turn, REPORTED — the note quotes it and nothing
    // gates on it.
    let (sin_t, cos_t) = spout_turn().sin_cos();
    let spout = body_at(&ev, r.spout, tol);

    // **Two references, and only one of them is this body's.**
    //
    // There is no elementary volume for a cubic skin through seven
    // annuli. There IS one for the thing that skin approximates, and
    // it is the same frustum difference this scene used before the
    // spout bent: **a bend about a spine through the sections' own
    // centres is volume-neutral.** The tube's volume element is
    // `(1 − κ·x) dA ds` with `x` toward the curvature centre, and
    // `∫x dA` over a centred section is zero — so the curvature term
    // integrates away and what is left is `∫A ds`, the STRAIGHTENED
    // tube's volume. So the closed form is the straight frustum, said
    // in the canal's own three numbers: outer radii `R0 → R1`, a bore
    // that is `SPOUT_BORE` of each, and the SPINE LENGTH as the height.
    let bore = SPOUT_BORE;
    let v_spout = (1.0 - bore * bore) * frustum_volume(SPOUT_R0, SPOUT_R1, SPOUT_LEN);
    let a_spout = frustum_lateral(SPOUT_R0, SPOUT_R1, SPOUT_LEN)
        + frustum_lateral(bore * SPOUT_R0, bore * SPOUT_R1, SPOUT_LEN)
        + annulus(SPOUT_R0, bore * SPOUT_R0)
        + annulus(SPOUT_R1, bore * SPOUT_R1);

    // The skin's OWN volume is what a certified enclosure of this body
    // must hold, and the frustum is not it: the two differ by the LOFT
    // FIT. So the skin's reference is integrated over its own surfaces
    // ([`patch_quadrature`]) at two orders, and their difference is the
    // reference's error estimate. The caps are the authored annuli.
    let (v_lo, a_lo) = patch_quadrature(&spout, 12);
    let (v_skin, a_walls) = patch_quadrature(&spout, 24);
    let a_skin = a_walls + annulus(SPOUT_R0, bore * SPOUT_R0) + annulus(SPOUT_R1, bore * SPOUT_R1);
    let (v_err, a_err) = ((v_skin - v_lo).abs(), (a_walls - a_lo).abs());

    // **The frustum is held to the skin at the FIT's scale.** A cubic
    // spline through stations `Δ` apart in the bend's angle misses a
    // smooth curve by `(5/384)·Δ⁴` of its own scale, and a section's
    // area is quadratic in its scale, so the volume misses by about
    // twice that. That is a property of the geometry, and holds at
    // every ε.
    let fit_step = SPOUT_BEND / (SPOUT_STATIONS - 1) as f64;
    let fit_scale = 2.0 * (5.0 / 384.0) * fit_step.powi(4);
    let v_fit = ((v_skin - v_spout) / v_spout).abs();
    assert!(
        v_fit <= fit_scale,
        "the skin's V = {v_skin} misses the straightened frustum's {v_spout} by {v_fit:e} of \
         it, past the cubic fit's scale {fit_scale:e}"
    );
    let a_fit = ((a_skin - a_spout) / a_spout).abs();

    // These walls are RATIONAL — [`SPOUT_ARCS`] carries why, and why
    // that is the honest authoring — so they are QUADRATURE faces.
    // `mass_properties` publishes a certified ENCLOSURE rather than a
    // number, where every analytic body on this page publishes a pad
    // of exactly 0, and that enclosure is chased to the printed
    // precision. The scene measures through the certificate tier 3
    // hands back, continued by `measure`. Where the precision is met
    // the enclosure must hold the skin's own reference; where it is
    // not, the bracket the measurement reached must; a refusal is a
    // body with no volume at all, and stays fail-loud.
    let spout_certificate = pncad::topo::validate_geometric_certificate(&spout, tol)
        .unwrap_or_else(|e| panic!("the spout's tier 3 refused: {e:?}"));
    let spout_gated = spout_certificate.enclosure();
    let spout_measured = spout_certificate
        .measure(crate::printed())
        .unwrap_or_else(|refusal| panic!("the spout's mass properties: {refusal}"));
    let spout_reading = match spout_measured.stop {
        pncad::topo::Stop::Met => {
            let p = spout_measured.enclosure;
            let v_gap = (p.volume - v_skin).abs();
            let a_gap = (p.surface_area - a_skin).abs();
            assert!(
                v_gap <= p.volume_pad + v_err,
                "the canal's V = {} vs its own skin's {v_skin} (± {v_err:e}): they differ by \
                 {v_gap:e} m^3, OUTSIDE the certified enclosure's own half-width {:e}",
                p.volume,
                p.volume_pad
            );
            assert!(
                a_gap <= p.area_pad + a_err,
                "the canal's A = {} vs its own skin's {a_skin} (± {a_err:e}): they differ by \
                 {a_gap:e} m^2, OUTSIDE the certified enclosure's own half-width {:e}",
                p.surface_area,
                p.area_pad
            );
            // The pads are not vacuous, and saying so is what makes
            // the bracket a test: an analytic body publishes 0, so a
            // bracket claim on THIS body is a claim only because these
            // walls are quadrature faces.
            assert!(
                p.volume_pad > 0.0 && p.area_pad > 0.0,
                "a fitted rational skin publishes a nonzero pad; got {:e} and {:e}",
                p.volume_pad,
                p.area_pad
            );
            format!(
                "V = {:.9} m^3 against the skin's own {v_skin:.9} — inside the kernel's \
                 own certified enclosure, {v_gap:.1e} against a half-width of {:.1e}; A = {:.9} \
                 against {a_skin:.9}, {a_gap:.1e} against {:.1e}. The straightened frustum's \
                 V = {v_spout:.9} sits {v_fit:.1e} of itself off the skin's, which is the loft \
                 FIT, inside the cubic's {fit_scale:.1e}; its A = {a_spout:.9} sits {a_fit:.1e} off",
                p.volume, p.volume_pad, p.surface_area, p.area_pad
            )
        }
        stop @ (pncad::topo::Stop::Floor | pncad::topo::Stop::Wall) => {
            let b = spout_measured.enclosure.enclosure();
            // Each face keeps the narrower of the enclosures it held,
            // so the bracket is the gate's or a narrower one, and never
            // wider.
            assert!(
                b.volume_hi - b.volume_lo <= spout_gated.volume_hi - spout_gated.volume_lo,
                "the spout's bracket [{}, {}] is wider than the gate's [{}, {}]",
                b.volume_lo,
                b.volume_hi,
                spout_gated.volume_lo,
                spout_gated.volume_hi,
            );
            let half = 0.5 * (b.volume_hi - b.volume_lo);
            assert!(
                b.volume_lo <= v_skin - v_err && v_skin + v_err <= b.volume_hi,
                "the skin's own V = {v_skin} (± {v_err:e}) lies OUTSIDE the canal's certified \
                 bracket [{}, {}]",
                b.volume_lo,
                b.volume_hi
            );
            format!(
                "the measurement stopped at its {stop:?} short of the printed precision, so \
                 there is no volume NUMBER to compare. What is missing is a number and not a \
                 certificate: tier 3 admitted this body at this same eps on the SIGN of the \
                 enclosure, and the certificate it handed back, continued as far as the faces \
                 refine, keeps a bracket — V in [{:.9}, {:.9}] m^3, whose lower \
                 end clears zero by {:.1e} half-widths, and which holds the skin's own \
                 V = {v_skin:.9}. The straightened frustum's V = {v_spout:.9} sits {v_fit:.1e} \
                 of itself off the skin's, which is the loft FIT, inside the cubic's \
                 {fit_scale:.1e}",
                b.volume_lo,
                b.volume_hi,
                b.volume_lo / half,
            )
        }
        stop => unreachable!("a resolution decides nothing: {stop:?}"),
    };
    let spout_bend_deg = SPOUT_BEND.to_degrees();

    // ---- the handle ----
    let sweep = 2.0 * (FRAC_PI_2 + HANDLE_OVER);
    let handle = body_at(&ev, r.handle, tol);
    let v_handle = sweep * HANDLE_R * PI * HANDLE_TUBE * HANDLE_TUBE;
    let a_handle = sweep * HANDLE_R * 2.0 * PI * HANDLE_TUBE + 2.0 * PI * HANDLE_TUBE * HANDLE_TUBE;
    let handle_props = pncad::topo::mass_properties(
        &crate::booleans::at_rest(&handle, tol),
        tol,
        crate::printed(),
    )
    .expect("the handle's props");
    assert!(
        ((handle_props.enclosure.volume - v_handle) / v_handle).abs() < 1e-12,
        "handle V = {} vs Pappus on the disc {v_handle}",
        handle_props.enclosure.volume
    );
    assert!(
        ((handle_props.enclosure.surface_area - a_handle) / a_handle).abs() < 1e-12,
        "handle A = {} vs the closed form {a_handle}",
        handle_props.enclosure.surface_area
    );

    // ---- the walls ----
    //
    // Every one of these is ATTEMPTED here, on every pass, and pinned
    // by its own EXACT typed refusal: a different refusal, or a
    // success, fails the tour — so the findings list in the module
    // docs cannot rot behind a frontier that moved.

    // WALL 1 — RETIRED at #1081's PR-2b, and the retirement is the
    // pot above: the belly IS the arc now. What this wall pinned was
    // the sealed hollow of a sphere-zone meridian refusing at the
    // per-chart door's moved corner (a refusal since retired: the door
    // derives corners by root now), and it refused because `shell` moved one
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
    let cup_counts = euler_counts(&cup);
    assert_eq!(
        (cup_counts.r, cup_counts.genus(), cup_counts.s),
        (1, Ok(0), 1),
        "ONE rim annulus carrying ONE ring, genus 0 as `topo::shell`'s docs promise a \
         cup is, and the cavity fused into the boundary"
    );
    assert_eq!(
        faces_where(&ev, r.cup, SegPat::tag(SegTag::Rim)).len(),
        1,
        "the mouth disc comes back as ONE rim face"
    );
    let cup_props =
        pncad::topo::mass_properties(&crate::booleans::at_rest(&cup, tol), tol, crate::printed())
            .expect("the cup's props");
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
        ((cup_props.enclosure.volume - cup_want) / cup_want).abs() < 1e-12,
        "cup V = {} vs the wall's closed form less the lifted mouth disc {cup_want}",
        cup_props.enclosure.volume
    );
    let cup_mesh = pncad::mesh::tessellate(&cup, DELTA, tol)
        .unwrap_or_else(|e| panic!("the opened pot must triangulate, got {e:?}"));
    let cup_triangles: usize = cup_mesh.patches.iter().map(|q| q.triangles.len()).sum();
    assert!(cup_triangles > 0, "a mesh with no triangles is not a mesh");

    // The two unions, attempted for real. Both refusals are read off
    // THIS evaluation — the one the probes are run against below and
    // the one the note quotes — so the caption and the pin are one
    // measurement.
    let handle_refusal = describe_join(&ev, r.handle_union);
    let spout_refusal = describe_join(&ev, r.spout_union);
    wall_probes(&ev, &r);

    vec![TourStop {
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
              mouth disc BY NAME) for the vessel; Profile -> Revolve -> \
              ONE Node::Fillet over the flange rim, the dome foot and the knob top, \
              each by its two half-arcs' names, for the lid; Profile(2 circle loops) x7 -> \
              Node::Loft(v_degree 3, six stations) -> Place against the pot (its root cap \
              on a frame read off the pot) for the spout; \
              Node::Tube -> Place against the pot (its spine's axis on a frame read off \
              the pot) for the handle. Two walls pinned: both unions, \
              as Node::Union nodes that refuse at evaluate",
        delta: DELTA,
        note: Some(format!(
            "ONE RECIPE DOCUMENT, AND EVERY BODY HERE IS A NODE'S VALUE — the mouth and \
             the lid's three rims are NAMED by their role in the sweep, not found by a \
             numeric scan of the built body. THE VESSEL, SEALED THEN OPENED, is the \
             same operand and the same wall through the same node kind twice, parted \
             only by the open list: EMPTY for the sealed hollow, the mouth disc's \
             name for the cup. Sealed it is {pv} vertices, {pe} edges, \
             {pf} faces over TWO shells in one solid — the operand's 6/10/6 twice, \
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
             THE BELLY IS AN ARC, AND THAT IS THE \
             SCENE'S FIRST FINDING RETIRED — foot cylinder, ONE spherical zone, mouth, \
             where the squared pot spent three segments; the mechanism that un-squared \
             it is the module docs' finding 1 and is not re-told here. WALL 1 WAS \
             RE-PLANTED ONE STEP OUT AND IS NOW RETIRED IN TURN: it pushed the belly's \
             arc centre OFF the axis, making the wall a torus the meridian reduction \
             did not know. The reduction knows the kind now, that pot hollows, and the \
             probe above asserts the hollow instead of pinning a refusal \
             (`tests/verbs_teapot.rs` carries the junction table, the tangency \
             discriminator and the sweep's blind spot). THE MOUTH IS \
             OPEN, AND THAT WAS THE SCENE'S SECOND FINDING BEFORE IT WAS FIXED: \
             `shell_open` used to return a body that passed tiers 1-3 while each \
             designated half-disc carried its own cavity counterpart's boundary as a \
             ring, so the CDT refused it and the pot shipped sealed (#1082); the class \
             and its mechanism are the module docs' finding 2. The seam is retired \
             before the glue now, and this pot's mouth comes back as ONE \
             annular rim carrying one ring, genus {}, V = {:.9} m³ against the wall's \
             closed form less the lifted mouth disc, {cup_triangles} triangles at δ = \
             {DELTA}. On an ANNULAR mouth the same class needs two disjoint annuli — a \
             face SPLIT — and that is built too; `tests/verbs_teapot.rs` carries both \
             shapes. THE OPENED POT NOW LEAVES AS STEP, which the sealed one could not: \
             the writer refuses a void shell (VoidShellUnsupported: no \
             BREP_WITH_VOIDS yet) — and a cup is ONE shell. That gate keeps three \
             live probes (klein's wall 6, the `ring` scene's and `hollowtorus`'s); this \
             scene is no longer one of them, because the body it ships no longer \
             reaches it. THE LID, which is where the CURVED-SUPPORT fillet family \
             lives on this tour. It is what a potter turns: a SOLID knob, no vent — \
             the profile closes down the axis, so the revolve mints half-walls like \
             the pot's: 8/14/8 sharp (two whole discs, the cone, sphere and cylinder \
             as two faces each, four latitude rims of two half-arcs each), \
             V = 5369*pi/3 / 256^3 m^3 and A = (453 + 52*sqrt(10))*pi / 256^2 m^2 \
             by Archimedes' zone and the stack, both met to 1e-12. THREE rims roll \
             in ONE request, each asked for WHOLE — both its half-arcs, BandRim and \
             BandRimPi, minted by `band_rim` and `band_rim_pi` — and each band \
             carves over both arcs: 12/21/11 rolled, the sum of the lone bands' \
             deltas — (+2, +3, +1) each, less one vertex and one edge where the \
             blend's closing join makes a trimline on a WHOLE disc (the base under \
             the flange, the top under the knob) one closed circle; the dome's foot \
             runs between two half-walled supports and keeps both. The flange's rim and the \
             dome's foot are the two ends of ONE meridian segment, so both bands slit \
             and cross THAT segment's meridians, and their names tell the two apart \
             by the band that made each. Their supports are three \
             DIFFERENT coaxial arms: the flange's rim is cone x plane(perp), the dome's \
             foot is SPHERE x CONE — the arm no plane-supported scene reaches — and the \
             knob's top is cylinder x plane. Every band is a ring-free TORUS, which is \
             what sharing an axis buys, and each is re-derived here from its OWN two \
             tangency traces rather than read back from the arm that minted it: two \
             lines for the knob ({knob_major}, {ROLL}), two lines for the flange \
             ({flange_major}, {ROLL}), and for the dome's foot a line meeting a circle \
             ({foot_major}, {ROLL}) at the crossing y = {foot_y}. ΔV = {dv_lid:.9} m³, \
             inside the three corner squares' bound {pappus_cap:.9}. NO MATE IS AUTHORED — the lid renders {LIFT} m above the mouth and \
             the two bodies are strangers to the kernel; declared contact is M9's. THE \
             SPOUT AND THE HANDLE. The spout is a CANAL — {SPOUT_STATIONS} annular \
             sections standing on the tangent frames of a circular arc, skinned by ONE \
             `Node::Loft`, bending {spout_bend_deg:.0} degrees and tapering to half its root radius, so \
             no revolve reaches it at any axis and no extrude does either. A fitted skin has no \
             elementary volume, but the tube it approximates does, and it is the SAME \
             frustum difference this scene used before the spout bent: a bend about a \
             spine through the sections' own centres is VOLUME-NEUTRAL, because the \
             curvature term of the volume element weighs the section's first moment and \
             a centred section's is zero. The same cancellation runs over the lateral \
             area. So the CLOSED FORM is the STRAIGHTENED tube — outer radii R0 → R1, a \
             bore {SPOUT_BORE} of each, the spine length as the height — but the body is \
             a cubic skin that INTERPOLATES that tube, and the two differ by the loft \
             fit. So the canal is held, INSIDE THE KERNEL'S OWN CERTIFIED ENCLOSURE, to \
             its own skin's volume integrated over its own surfaces, and the frustum is \
             held to that at the fit's scale: {spout_reading}. THE SECTIONS ARE ROUND, SO \
             THE VOLUME IS AN ENCLOSURE. A circle is RATIONAL, so these lateral walls are \
             rational patches, and a rational patch is a QUADRATURE face whose enclosure \
             is chased to a REPORTING target derived from eps; where a tight enough eps \
             refuses that target on budget, tier 3 still certifies the body on the \
             enclosure's SIGN, and the reading holds the skin's volume to the bracket the \
             certificate keeps. The handle checks by \
             Pappus on its own disc. The document says a placement in AXIS-ANGLE and the \
             3-4-5 turn is not a binary-exact angle, so what that costs is MEASURED off \
             the PLACED BODY: the TIP cap, a whole spine-length off the turn's fixed \
             axis, stands {tip_residual:e} m from its own exact image and its normal \
             agrees with the bend and the turn together to a cross of {tip_cross:e}; the \
             ROOT cap stands {root_residual:e} m from where the direction's own matrix \
             puts it and faces SPOUT_DIR to a cross of {axis_cross:e}. The two carry \
             different halves ON PURPOSE: the root cap sits exactly ON the rotation's \
             fixed axis, where a position residual would measure the translation and \
             nothing else — so position evidence is the tip's, while orientation \
             evidence is both caps', because a translation cannot rotate a normal. On THIS platform the \
             angle's cosine and sine come back as {cos_t} and {sin_t}, which are the \
             direction's components bit for bit; that is reported and not asserted, \
             because a bitwise pin here would gate the scene on one libm's last ulp. Neither JOINS, and they refuse at TWO \
             DIFFERENT DOORS. handle ∪ vessel: {handle_refusal} — the handle's torus \
             is on the union's roster, so the join gets past the operand gate and stops \
             at the maximal-faces precondition on the VESSEL, whose full revolve mints \
             its planar walls split in two. spout ∪ vessel: {spout_refusal} — and THIS one moved when the spout became \
             a canal. A loft's walls are Nurbs and the pair rung HAS a Nurbs arm, so the \
             request now gets past it into the sweep, which meets the belly's revolve seam \
             (a meridian, where the spout leaves the belly) with one of the spout's NURBS walls, on that wall's seam, \
             and cannot yet split a spline seam at a point: the pierce the join needs. The shape a potter \
             draws did not become joinable; the refusal moved off a pair nobody modelled \
             and onto the meeting itself. The schedule is the banked germ-chord \
             lanes (DESIGN frontier (d)) and #1057's two C5 arms. BOTH REFUSALS NOW \
             ARRIVE THROUGH THE DOCUMENT: each union is a `Node::Union` that lowers \
             to the same kernel `union` and fails at `evaluate`, so what is quoted here \
             is the evaluation's own carried refusal. NEITHER PROBE PINS AN ARENA KEY, which is \
             why both survived a re-authoring that moved one — the mouth-rim annulus at \
             y = 1/8 was FaceKey(1v1) off the plane scan and is FaceKey(2v1) off the \
             document. Wall 2 matches on the operand; wall 3 matches on \
             the operand and the face's kind, never the face key, which is exactly the \
             sort of value a probe must not pin. The lofted canal above \
             is what a potter would draw and it IS authorable; what is not is a SWEPT \
             spout said as a recipe (`wire_sweep` refuses unconditionally, U4/LQ3 \
             banked — the library sweep itself turns a bend like this one, short of a U-turn) \
             and, one level \
             down, the SPINE ITSELF — the loft's placements are \
             {SPOUT_STATIONS} literal frames this file derives, so the saved document carries no \
             arc and moving the bend re-derives all seven",
            props.enclosure.volume,
            props.enclosure.surface_area,
            voids[0].enclosure.volume,
            2.0 * WALL,
            planes.len(),
            euler_counts(&cup)
                .genus()
                .unwrap_or_else(|refusal| panic!("{refusal}")),
            cup_props.enclosure.volume,
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
