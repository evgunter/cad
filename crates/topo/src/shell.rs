//! `shell` — hollowing, sealed and opened.
//!
//! **Shelling** turns a solid into a thin-walled one: the boundary is
//! offset inward by the wall thickness, and the hollow is either kept
//! closed (a cavity) or opened by designating faces whose material is
//! removed, leaving annular rims where the wall's thickness shows
//! (`crates/geom-brep/README.md`'s vocabulary, unchanged).
//!
//! # The sealed arm, and what it deliberately does not run
//!
//! **Shelling is a PER-SOLID verb, and it applies to every solid the
//! operand has.** `S − offset_inward(S, t)` is defined solid by solid,
//! so on a body of `N` solids each `Sᵢ` becomes its own thin solids —
//! one per boundary shell of `Sᵢ` — and the result body holds all of
//! them. Nothing crosses between solids: a solid's cavity is inside
//! its own material, its clearance is its own, its offset door is its
//! own, and a designation names faces on any solid. An operand with no
//! solid at all has nothing to thicken ([`ShellError::NoSolid`]).
//! Everything below is stated for one solid and runs once per solid.
//!
//! `shell(body, t)` is `body − offset_inward(body, t)` BY DEFINITION,
//! and that definition is boolean-family. Its EXECUTION is not: when
//! every face's offset certifies, the two boundaries provably do not
//! cross, so SSI, the crossing census and the classification walk have
//! nothing to do — and worse, the general path's containment
//! examination is extent-box coarse, so routing through it would refuse
//! bodies the construction has already proven nested. The sealed shell
//! therefore executes as the boolean's DEGENERATE NO-CROSSING ARM:
//!
//! 1. one clone, every boundary face moved to its inward offset — the
//!    result is the material to remove, a positively oriented closed
//!    body. **Two doors do that, and which one runs is decided by the
//!    SOLID** — a box beside a vessel is neither all-planar nor axial
//!    while each of the two is one of those, so a whole-body reading
//!    would refuse the vessel's corners it solves alone: an
//!    ALL-PLANAR solid goes through
//!    [`crate::offset_planes_together`], which moves every chart at
//!    once and solves each corner against all the moved planes meeting
//!    it; anything with a curved face goes chart by chart through
//!    [`crate::replace_faces_offset`], whose corners are transported
//!    once per chart and whose OBLIQUE ones therefore refuse
//!    (`ReanchorOffCarrier`) rather than build. The split is #1081's:
//!    the planar half of that class is repaired and the curved half is
//!    not;
//! 2. that body inserted through the shared void-insertion door
//!    ([`crate::boolean::voids::insert_voids`]) with carried evidence —
//!    every shell of it, grafted under the operand solid its own solid
//!    was cloned from, one destination per cavity solid;
//! 3. **one thin solid per operand shell.** A hollow operand's clone
//!    has a shell per operand shell, and once the door has reverted it
//!    the outer shell's eroded twin faces inward — a cavity of the outer
//!    wall — while each void's DILATED twin faces outward: it is the
//!    outer boundary of the thin wall around that void. Each operand
//!    void and its twin therefore move out of the operand's solid into
//!    a solid of their own ([`Body::move_shells_to_new_solid`]), paired
//!    off the graft map — structurally, never by a containment probe.
//!    For `k` voids the result holds `k + 1` solids and `2(k + 1)`
//!    shells, and every operand shell survives under its key. A
//!    single-shell operand takes this step vacuously;
//! 4. one closing pcurve mint, then one validation (below).
//!
//! **Which way a void moves is not a special case.** `inward` reads a
//! face's `sense` to move it into the material, and a void's faces are
//! wound with their material side outward, so the same signed distance
//! that erodes the outer shell dilates a void: `shell(S, t)` on a hollow
//! `S` thickens EVERY boundary, outer inward and each void outward, with
//! no void-specific sign anywhere in the construction.
//!
//! Cost is offset mint + certification + one structural insertion. No
//! SSI runs, and that is pinned structurally rather than asserted in
//! prose (`shell_runs_no_intersection_machinery`). **What the pin
//! actually covers**: it reads the verdict log for `bool_`-prefixed
//! predicates, which is the crossing pipeline's own vocabulary — it
//! does NOT cover the `ssi_*` or `tangent_locus_*` families, which
//! carry their own prefixes. The pin's claim is "the boolean's
//! machinery did not run", and the marching stack is reached only
//! through that machinery, so the coverage is by composition rather
//! than by the filter.
//!
//! # The evidence, and where it comes from
//!
//! The void door never derives containment. What this verb carries to
//! it is the offset construction's OWN decides: a face's inward offset
//! mints only when its d-vs-reach margin is certifiably positive (the
//! realized radius for a cylinder, sphere or torus tube; the
//! apex-window margin for a cone; unbounded for a plane), and those are
//! exactly the margins that say the offset surface has not reached the
//! spine it would fold on. Every face minting is therefore the
//! construction's own strict-inside claim, and it is passed verbatim as
//! [`VoidContainment::Carried`] with [`Sign::Positive`] — the RING
//! pattern, one dimension up from a profile's hole-inside-outer margin.
//!
//! **What that carries, stated exactly.** It is a per-face (LOCAL)
//! reach claim, made once per CLONE SHELL — the eroded outer shell and
//! every dilated void alike, since each of them is every one of its
//! faces' own margin — and on a PLANE it is vacuous: a plane's reach is
//! unbounded, so no per-face decide can see two walls marching through
//! each other. That collision class is gated separately and in closed
//! form by [`wall_clearance`], which walks every planar face of every
//! shell OF ONE SOLID — a void wall facing the outer wall, or two
//! voids facing each other, across less than `2t` refuses exactly as
//! two outer walls do, while two faces of DIFFERENT solids never gate
//! at any separation, because there is no material between two solids
//! to be too thin —
//! with each face's footprint grown by `t` before the separation test,
//! because an inward offset reaches past every concave edge by `t`: a
//! pair whose operand footprints are disjoint by less than `2t` across
//! a gap under `2t` (an S-bend's risers, two voids offset diagonally)
//! refuses too. **What the gate decides** is that no two antiparallel
//! PLANAR faces have offsets whose projected boxes meet across less
//! than `2t`; **what it still cannot see** is a curved wall (below), a
//! planar pair that is not antiparallel (two offsets meeting at an
//! angle), and the corner solves' own refusals, which are the offset
//! doors'.
//!
//! **The curved residue is an open window, and it is not caught by
//! anything downstream.** A curved thin neck — two facing cylinder or
//! spline walls closer than `2t` — still shells to a self-intersecting
//! cavity, silently. **Tier-3 validation does NOT catch it**: every
//! per-face loop stays simple and consistently wound while the walls
//! cross, so the body validates and its volume is wrong (measured on a
//! planar dumbbell before the gate above existed; the same construction
//! with curved necks is still reachable). A box-based curved gate is
//! not the answer either — a shelled tube's concentric walls overlap
//! boxes by construction, so such a gate would refuse the verb's own
//! acceptance fixtures. The general clearance certificate over a
//! parameter box is M10's machinery; the window is issue #1055. **On a
//! hollow operand the window is the whole moved clone**: a void's
//! dilated twin can march into the eroded outer wall, or into another
//! void's twin, exactly as two outer walls can, so a certificate that
//! closes it reads every non-adjacent pair of the moved body across its
//! shells, not the outer shell's pairs alone.
//!
//! **Where the loud cases actually refuse.** A wall past a curved
//! face's own reach refuses at the offset door's realized-radius floor,
//! during CONSTRUCTION. A cavity whose walls invert refuses at the
//! attach layer's certification — the interval-forward and zero-span
//! checks on a re-attached edge — also during construction, on the very
//! `set_edge_curve` call that would have stored it. Neither is tier 3:
//! the verb's closing `validate_geometric` has never been the thing
//! that catches a bad wall, and saying otherwise misattributes the
//! net that is doing the work.
//!
//! # The closing mint
//!
//! The void door's posture is `Transfers`
//! (`crate::pcurves::staleness_posture::DECLARED`, the `insert_voids`
//! row): the reverted cavity's rows go stale in content and the graft
//! copies them verbatim, and that row's contract is that the producer's
//! final mint re-derives every row of the merged body. This verb is a
//! producer and runs [`crate::pcurves::mint_pcurves`] once, on the
//! assembled body, before `validate_geometric` — the verb's own
//! whole-body pass, and it stays whole-body: it is what discharges
//! `insert_voids`'s `Transfers` row over the WHOLE merged body, which
//! no per-solid pass covers. One pass suffices: nothing between the
//! door and the validate reads a stored row, the simultaneous lift
//! doors mint the rows of their own scope (the solid they were handed)
//! and touch no other, and every other step is `Neither` for rows.
//! Two consequences are stated because nothing
//! enforces them: the pass CLEARS the map first, so **a stale or
//! missing row on the OPERAND is invisible to this verb** — an operand
//! that fails tier 3 on its own rows shells to a valid body whose rows
//! are the sound operand's (`shell9_r2_probes`, the laundering rows;
//! `work/shell/shell-launders-a-stale-operand-row.md`, a posture-table
//! question for every producer that spells this mint) — and a face
//! whose carrier class the pass cannot derive stops carrying rows
//! rather than refusing (`UnsupportedCarrier`; not known to be
//! reachable through this verb). The refusal is
//! [`ShellError::Pcurve`], a kernel finding by construction.
//!
//! # The record
//!
//! Both doors return [`Shelled`]: the thin solid and the
//! [`ShellNaming`] its consumers name entities through, written by the
//! construction's own steps as they run.
//!
//! **Two key spaces meet in it, and every row says which is which.**
//! SOURCE keys are the operand's; RESULT keys are the returned body's.
//! The result is a clone of the operand with the moved clone grafted
//! in, so every surviving operand entity — on the outer shell or on a
//! void; the operand's shells are never regrafted — keeps its operand
//! key and its row's two columns are equal; the row states the
//! correspondence anyway, so no consumer leans on the identity. Cavity
//! entities are born in the cavity clone, whose keys are the operand's
//! for the same reason, and cross into the result through
//! [`crate::boolean::voids::insert_voids`]'s graft map — the only
//! bridge, read at insertion time. [`ShellNaming::thickened`] says
//! which result SOLID each operand shell's wall became: the operand's
//! own solid for its outer shell, a minted one per void. On a
//! multi-solid operand every solid contributes its own rows, in
//! shell-arena order.
//!
//! **The rows are HISTORICAL.** A result key a row names may have died
//! in a LATER step: a designated face's inner twin is recorded when the
//! cavity is grafted and is then killed by the rim surgery's `kfmrh`.
//! Every such death is listed in [`ShellRetired`] — in every arena the
//! record names and in the two it only implies, so its faces, edges,
//! vertices, loops, surfaces and shells together are exactly what the
//! construction retired — and a consumer reads `live = recorded − dead`
//! rather than assuming every row resolves.
//!
//! **Loops are handles, not emitter targets.** [`RimNaming::ring`] and
//! [`HoleRim::ring`] are RESULT loop keys, and the document layer mints
//! no `StableName` for a loop; a rim's anchor is its `ring_edges` /
//! `ring_vertices`, whose source columns are operand edges and vertices.
//! A hole's loop in particular is a result key on every operand and an
//! operand key on only some — an extruded holed slab's mouth carries its
//! ring already, while a revolve's slit annular cap has none until
//! `kemr` mints one during the chart reduction.
//!
//! # The opened arm
//!
//! `shell_open(body, t, open_faces)` is the sealed construction plus
//! rim surgery, and it composes rather than inventing:
//!
//! 1. the sealed shell, exactly as above — so the evidence handed to
//!    the void door is the strict one, before anything is opened;
//! 2. per designated CHART, its CAVITY counterpart offset back OUTWARD
//!    by `t` (the same door ladder as the cavity's —
//!    [`crate::offset_charts_together`] for a solid of revolution,
//!    [`crate::replace_faces_offset`] otherwise), which lands it on
//!    the designated face's own surface and — because the door
//!    re-describes a moved face's boundary against its untouched
//!    neighbours — extends the cavity's side walls up to meet it;
//! 3. both charts reduced to ONE face carrying disjoint cycles
//!    ([`canonicalize_chart`]);
//! 4. `kfmrh` on the pair: the cavity counterpart dies, its outer loop
//!    becomes a RING of the designated face, and the cavity shell fuses
//!    into the outer one. A counterpart HOLE is promoted to its own rim
//!    face first (`mfkrh`) and collects the designated face's matching
//!    hole after (`ring_move`).
//!
//! The result is CLOSED, and the thin solid that carries the
//! designation has one shell: the designated face is now annular — the
//! rim, where the wall thickness shows. (Every other thin solid, of
//! this operand solid or of any other, keeps its two.)
//!
//! A designation names a face on ANY solid, and each opens the thin
//! solid that face's wall became. Which solid the surgery runs in is
//! read off the RESULT — the thin solids are partitioned before it —
//! so a void designation's face and its counterpart are found in the
//! minted solid they moved to, and the lift's door is that solid's.
//!
//! **A designation on a VOID face** of a hollow operand runs the same
//! four steps one solid over — the sealed construction has already put
//! the void and its dilated twin in a solid of their own — with the
//! glue's ROLES swapped: the counterpart's lifted boundary ENCLOSES the
//! designated face's (a dilation brought back onto the same plane), so
//! the counterpart survives as the rim, facing the gap between the
//! eroded outer wall and itself, and the designated face dies with its
//! outer loop becoming the ring. The wall around that void becomes a
//! cup opening into the gap; every other solid is untouched. Which
//! shell a designation is on is read off the roles the sealed arm
//! decided, never off the result's geometry.
//! **Nothing opens** — that is the load-bearing half, and D1's
//! manifold-first stance is untouched. Genus does not necessarily rise:
//! one opening gives a cup, which is genus 0 (the cavity shell fuses
//! into the boundary and the two Euler contributions cancel); a second
//! opening gives a tube, genus 1. The invariant is closure, not genus.
//! The surgery is the ring-topology
//! band precedent verbatim; no new machinery, and in particular no
//! ladder of quads (which would chamfer the opening rather than rim
//! it — the geometry that made step 2 necessary).
//!
//! # Why step 3 exists, and what the arm is expressible over
//!
//! The glue's only output shape is "one region per face, an outer loop
//! plus rings", so a designated face is safe exactly when its cavity
//! counterpart's boundary can become an INTERIOR-DISJOINT ring of it.
//! A revolve's chart does not arrive that way: a full revolve of an
//! axis-touching profile splits the cap into two half-discs meeting at
//! the axis apex, and a full revolve of a closed off-axis profile
//! leaves the annular cap SLIT along a radial seam. Gluing onto either
//! puts the counterpart's boundary ON the designated face's own —
//! sharing the apex, running back along the seam — and the result is a
//! body every structural tier blesses and no triangulator accepts.
//!
//! Both are facts about how the operand was swept rather than about
//! the region, and step 3 removes them through the Euler doors alone
//! (`kef`, `kev`, `kemr`). What survives step 3 is genuinely about the
//! region and is refused typed
//! ([`ShellError::OpenFaceRimNotExpressible`]): a chart whose faces are
//! not one region, a counterpart boundary that still meets the
//! designated face's, a promoted rim face whose own boundary meets the
//! hole it would carry, or more than one hole to pair. The invariant is
//! stated once more at rest by tier 3's check 9
//! ([`ValidationError::RingMeetsOuter`]), so a ring standing on its own
//! outer loop is loud wherever it is minted and not only here.
//!
//! **An UNDECIDABLE separation refuses too** and never proceeds to
//! build ([`ShellError::Escalated`]) — the glue is a write, and
//! building on a gap the predicate layer could not certify is the
//! guess D4 forbids. Through THIS verb that arm is door-shielded:
//! `shell_thickness` has already decided the wall certifiably
//! positive and every rim the arm below builds is that wall wide, so
//! no fixture of this verb reaches it. It is there for the at-rest
//! bodies OTHER producers hand the same predicate, which is where
//! check 9 does its work.

use geom_core::k_stats::decide;
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Real, Sign, Tol};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::boolean::voids::{VoidContainment, VoidEvidence, VoidInsertError, insert_voids};
use crate::entity::{
    EdgeKey, EntityId, FaceKey, HalfEdgeKey as HeKey, LoopBoundary, LoopKey, ShellKey, SolidKey,
    VertexKey,
};
use crate::euler::EulerOpError;
use crate::pcurves::{PcurveMintError, mint_pcurves};
use crate::props::{PropsQuadLane, ShellRole};
use crate::replace_face::ReplaceFaceError;
use crate::validate::{ValidationError, validate_geometric};

/// Typed refusal of the shell verb (closed enum, D4 ¶3).
#[derive(Clone, Debug)]
pub enum ShellError<T: Real> {
    /// The committed tolerance admits no ambiguity band, so no
    /// margined predicate in this verb has a verdict to give. The
    /// band is derived at the door from the tolerance witness alone;
    /// this is that derivation's own refusal, carried verbatim.
    Band {
        /// The band constructor's typed refusal.
        error: BandError,
    },
    /// The wall thickness is not certifiably positive: a zero or
    /// negative wall is not a thin solid, and the ambiguity band
    /// escalates rather than guessing.
    Thickness {
        /// The thickness as given, echoed as data.
        thickness: T,
    },
    /// The body carries no solid, so there is nothing to thicken.
    /// This is an EMPTY operand, not an unsupported arity: shelling is
    /// a per-solid verb and applies to every solid a body has, so any
    /// positive count builds and only zero has no construction to run.
    NoSolid,
    /// A solid's shells could not be told apart into one outer
    /// boundary and its voids: a solid stores no such designation, so
    /// the roles are the decided signs of the shells' signed volumes,
    /// and that decision refused. Read once per HOLLOW solid, over
    /// that solid's own shells only.
    Roles {
        /// The classifier's typed refusal, verbatim.
        error: crate::props::ShellClassifyError,
    },
    /// One of the operand's solids classifies to something other than
    /// exactly one outer shell — several material components filed
    /// under one solid, or none. Not a shape this verb thickens. The
    /// roles are read per solid, so the count is that solid's own and
    /// the refusal names which solid it is about.
    OperandOuterShells {
        /// The solid whose shells did not classify to one boundary.
        solid: SolidKey,
        /// How many of ITS shells classified as outer boundaries.
        outer: usize,
    },
    /// The re-partition of an operand void and its dilated twin into a
    /// solid of their own refused. Its preconditions hold by
    /// construction — the void is one of the sealed arm's decided void
    /// list, its twin is the graft map's answer for it, both sit under
    /// the void's OWN operand solid beside that solid's outer shell and
    /// its twin — so this is a kernel bug surfaced typed.
    Partition {
        /// The operand void whose thin solid could not be minted.
        shell: ShellKey,
        /// The ownership door's typed refusal.
        error: EulerOpError,
    },
    /// **The closed-form wall-clearance gate.** Two planar faces of the
    /// operand face each other across material thinner than `2t`, so
    /// their inward offsets cross and the cavity self-intersects. This
    /// is the collision class no per-face margin can see: a plane's
    /// reach is unbounded, so every per-face decide is vacuous while
    /// the walls march through each other.
    ///
    /// The gate is CONSERVATIVE by construction (footprint overlap is
    /// tested in projection, which can report an overlap that the true
    /// footprints do not have), so it may refuse a body that would
    /// have shelled — never the reverse.
    WallClearance {
        /// One of the two facing planar faces.
        face: FaceKey,
        /// The other.
        other: FaceKey,
        /// The measured distance between their planes, in meters.
        gap: T,
        /// The wall the two offsets would need, `2t`.
        needed: T,
    },
    /// A chart is worn by faces of two different SOLIDS. A chart moves
    /// as one and the door that moves it is its solid's, so such a
    /// chart has neither a single door nor a single corner problem.
    ///
    /// **Reachable through public doors**, so this is a refusal rather
    /// than an assertion of the impossible: a [`crate::subtract`] whose
    /// cut DISCONNECTS its operand leaves the two components under one
    /// solid still wearing the operand's own surface keys — a slab cut
    /// in half keeps one plane key across both halves — and
    /// [`crate::Body::move_shells_to_new_solid`] then files them as two
    /// solids. The result is a valid body this verb cannot thicken
    /// chart by chart, and it says so naming the pair.
    ChartSpansSolids {
        /// The chart's first face, in face-arena order.
        face: FaceKey,
        /// A face of the same chart on a different solid.
        other: FaceKey,
    },
    /// A chart worn by several faces has faces with DIFFERENT
    /// orientation bits, so "inward" is not one direction for it. The
    /// group door moves a chart as one; a mixed-sense chart has no
    /// single inward to move it by.
    ChartSenseMixed {
        /// A face of the chart.
        face: FaceKey,
        /// A face of the same chart with the opposite sense.
        other: FaceKey,
    },
    /// A face's inward offset refused. This is the validity gate AND
    /// the evidence in one: the margin that says the offset has not
    /// reached its own reach is the margin that says the cavity is
    /// strictly inside.
    Face {
        /// The face whose offset refused.
        face: FaceKey,
        /// The face-replacement door's typed refusal, verbatim.
        error: Box<ReplaceFaceError<T>>,
    },
    /// A designated open face does not resolve in the operand.
    OpenFaceStale {
        /// The unresolvable designation.
        face: FaceKey,
    },
    /// A face was designated open twice.
    OpenFaceRepeated {
        /// The repeated designation.
        face: FaceKey,
    },
    /// Every face of a shell was designated open: there would be no
    /// wall left to show a thickness, and no boundary to rim.
    OpenFacesExhaustShell {
        /// The shell whose faces were all designated.
        shell: ShellKey,
    },
    /// Removing the designated faces disconnects a shell's boundary:
    /// the remaining faces fall into more than one component, so the
    /// rims would bound separate pieces rather than one thin solid.
    OpenFacesDisconnect {
        /// The shell the designation cut in two.
        shell: ShellKey,
        /// How many components the remainder falls into.
        components: usize,
    },
    /// A designated face is not planar. Its rim would be a CURVED face
    /// carrying a ring loop, which the closed-form property inventory
    /// has no reading for (the same kernel-wide limitation the fillet
    /// band's ring-free annulus works around). Refused rather than
    /// built into a body whose volume cannot be computed.
    OpenFaceRingUnsupported {
        /// The designated face.
        face: FaceKey,
        /// Its surface kind.
        kind: geom_brep::SurfaceKind,
    },
    /// A designated face shares its chart with faces that were NOT
    /// designated. The rim surgery lifts a chart as one — the group
    /// door's own contract — so a partially designated chart has no
    /// coherent lift.
    OpenFaceChartPartial {
        /// The designated face.
        face: FaceKey,
        /// A face on the same chart that was not designated.
        other: FaceKey,
    },
    /// The rim stage's outward LIFT refused — the step that puts a
    /// cavity counterpart back onto its designated face's own surface.
    /// Distinct from [`ShellError::Face`], which is the inward offset
    /// that builds the cavity and carries the containment evidence;
    /// this one carries neither.
    Lift {
        /// The designated face whose counterpart could not be lifted.
        face: FaceKey,
        /// The face-replacement door's typed refusal, verbatim.
        error: Box<ReplaceFaceError<T>>,
    },
    /// The void-insertion door refused.
    Insert {
        /// The door's typed refusal, verbatim.
        error: VoidInsertError,
    },
    /// **The rim the designation asks for is not expressible.** The
    /// surgery's only output shape is "one region per face, bounded by
    /// an outer loop and disjoint rings", and the rim of this
    /// designated face is not that: either the chart could not be
    /// reduced to one such face, or the cavity counterpart's boundary
    /// meets the designated face's own boundary rather than sitting
    /// strictly inside it, or the two boundaries' holes do not
    /// correspond.
    ///
    /// Refused rather than answered wrongly — a body whose face
    /// carries a ring standing on its own outer loop passes every
    /// structural tier and then refuses to tessellate, which is the
    /// class this refusal closes at the door.
    OpenFaceRimNotExpressible {
        /// The designated face.
        face: FaceKey,
        /// Which of the shapes above it is.
        what: &'static str,
    },
    /// The rim surgery's Euler step refused.
    Rim {
        /// The designated face whose rim could not be minted.
        face: FaceKey,
        /// The operator's typed refusal.
        error: EulerOpError,
    },
    /// A margined predicate escalated: the margin landed in the
    /// ambiguity band or was poisoned (escalate-never-guess, D4 ¶3).
    Escalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// The body's referential coherence broke mid-construction — a
    /// kernel bug, surfaced rather than swallowed.
    Corrupt {
        /// The key that stopped resolving.
        key: EntityId,
    },
    /// The closing pcurve mint refused on the assembled body (module
    /// docs, "The closing mint"). Every gate before it accepted the
    /// body — the offsets certified, the void door grafted, the rim
    /// surgery closed — so a row that cannot be re-derived here is a
    /// kernel finding about the pcurve pass or the carriers it reads,
    /// surfaced typed rather than as the validator's report of a stale
    /// row.
    Pcurve {
        /// The mint's typed refusal, verbatim.
        source: PcurveMintError,
    },
    /// The assembled result does not validate, so it is discarded.
    NotValid {
        /// The validator's report.
        errors: Vec<ValidationError>,
    },
}

impl<T: Real> core::fmt::Display for ShellError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Band { error } => {
                write!(f, "shell could not form a band: {error}")
            }
            Self::Thickness { thickness } => write!(
                f,
                "shell: the wall thickness ({thickness:?} m) is not certifiably positive, so \
                 there is no thin solid to build"
            ),
            Self::NoSolid => write!(
                f,
                "shell: the operand carries no solid, so there is no material to thicken"
            ),
            Self::Roles { error } => write!(
                f,
                "shell: the operand's shells could not be told into one outer boundary and its \
                 voids: {error}"
            ),
            Self::OperandOuterShells { solid, outer } => write!(
                f,
                "shell: the operand's {solid:?} classifies to {outer} outer shells, not one — \
                 not a shape this verb thickens"
            ),
            Self::Partition { shell, error } => write!(
                f,
                "shell: the thin solid around operand void {shell:?} could not be partitioned \
                 out (kernel bug): {error}"
            ),
            Self::WallClearance {
                face,
                other,
                gap,
                needed,
            } => write!(
                f,
                "shell: {face:?} and {other:?} face each other across {gap:?} m of material and \
                 the two walls need {needed:?} m — their inward offsets cross, so the cavity \
                 would self-intersect"
            ),
            Self::ChartSpansSolids { face, other } => write!(
                f,
                "shell: chart faces {face:?} and {other:?} lie on different solids — a chart \
                 moves as one and its door is its solid's, so it cannot span two"
            ),
            Self::ChartSenseMixed { face, other } => write!(
                f,
                "shell: {face:?} and {other:?} share a chart but not an orientation, so \
                 \"inward\" is not one direction for it"
            ),
            Self::OpenFaceChartPartial { face, other } => write!(
                f,
                "shell: {face:?} was designated open but {other:?} shares its chart and was not \
                 — the rim surgery lifts a chart as one, so a partial designation has no \
                 coherent lift"
            ),
            Self::OpenFaceRimNotExpressible { face, what } => write!(
                f,
                "shell: the rim for {face:?} is not expressible as this surgery's output shape \
                 (one region per face, an outer loop plus disjoint rings): {what}. Nothing is \
                 built"
            ),
            Self::Lift { face, error } => write!(
                f,
                "shell: the rim's outward lift for {face:?} refused (this is the step that puts \
                 the cavity counterpart back on the designated face's surface — not the inward \
                 offset, and not the containment evidence): {error}"
            ),
            Self::Face { face, error } => write!(
                f,
                "shell: {face:?}'s inward offset refused, which is both the validity gate and \
                 the containment evidence: {error}"
            ),
            Self::OpenFaceStale { face } => {
                write!(
                    f,
                    "shell: the designated open face {face:?} does not resolve"
                )
            }
            Self::OpenFaceRepeated { face } => {
                write!(f, "shell: {face:?} was designated open twice")
            }
            Self::OpenFacesExhaustShell { shell } => write!(
                f,
                "shell: every face of {shell:?} was designated open — nothing would be left to \
                 carry a wall thickness, so there is no rim to mint"
            ),
            Self::OpenFacesDisconnect { shell, components } => write!(
                f,
                "shell: removing the designated faces leaves {shell:?}'s boundary in \
                 {components} components — the rims would bound separate pieces rather than one \
                 thin solid"
            ),
            Self::OpenFaceRingUnsupported { face, kind } => write!(
                f,
                "shell: the designated face {face:?} carries a {kind:?}, and its rim would be a \
                 curved face with a ring loop — a shape the closed-form property inventory has \
                 no reading for, so nothing is built"
            ),
            Self::Insert { error } => {
                write!(f, "shell: the void-insertion door refused: {error}")
            }
            Self::Rim { face, error } => {
                write!(f, "shell: the rim surgery on {face:?} refused: {error}")
            }
            Self::Escalated { source } => write!(f, "shell escalated: {source}"),
            Self::Corrupt { key } => write!(
                f,
                "shell: {key:?} stopped resolving mid-construction (kernel bug)"
            ),
            Self::Pcurve { source } => write!(
                f,
                "shell: the closing pcurve mint refused on the assembled thin solid, which \
                 every earlier gate accepted (kernel finding): {source}"
            ),
            Self::NotValid { errors } => write!(
                f,
                "shell: the assembled thin solid is not valid ({} errors); it is discarded",
                errors.len()
            ),
        }
    }
}

impl<T: Real> std::error::Error for ShellError<T> {}

// ---------------------------------------------------------------------
// The birth record
// ---------------------------------------------------------------------

/// Everything [`shell`] / [`shell_open`] built: the thin solid and the
/// birth record its consumers name entities through.
#[derive(Debug)]
pub struct Shelled<T: Real> {
    /// The thin solids — one per operand SHELL of every operand SOLID,
    /// in one body. A plain single-solid operand yields exactly one; a
    /// hollow one yields `k + 1` for its `k` voids; a multi-solid
    /// operand yields every solid's, side by side.
    pub body: Body<T>,
    /// The mint-time naming facts of the construction that built it.
    pub naming: ShellNaming,
}

/// Mint-time naming facts of one shell (source keys ← the operand,
/// result keys ← the returned body). Rows are written as the doors
/// act, in the deterministic order the construction visits entities
/// (D9); rows are historical — a result key listed here may have
/// died in a LATER step, and every such death is listed in `dead`.
///
/// **Survivors keep their operand keys**, and every one of them still
/// gets a row: the result body is a clone of the operand with the
/// moved clone grafted in, so a wall face, edge or vertex — on the
/// outer shell or on a void, since the operand's own shells are never
/// regrafted — resolves in both bodies under one key. `outer`'s two
/// columns are therefore equal on every operand, and the row shape
/// does not lean on that: a consumer reads the correspondence, never
/// the identity.
///
/// The lookup doors below are the intended reading order; the `Vec`
/// rows are the data, kept public so a consumer can walk the
/// construction's own order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ShellNaming {
    /// Wall face (result) ← the source face it is: every source face
    /// that was not designated open, of every shell of every solid —
    /// void faces included, since a void's wall is a wall too —
    /// in face-arena order.
    pub outer: Vec<(FaceKey, FaceKey)>,
    /// Inner (cavity) twin face (result) ← the source face it was
    /// offset from. Every source face, face-arena order — a designated
    /// face's twin is listed too; it dies in the rim surgery and is
    /// then in `dead`.
    pub inner: Vec<(FaceKey, FaceKey)>,
    /// Inner twin edge (result) ← source edge, edge-arena order.
    pub inner_edges: Vec<(EdgeKey, EdgeKey)>,
    /// Inner twin vertex (result) ← source vertex, vertex-arena order.
    pub inner_vertices: Vec<(VertexKey, VertexKey)>,
    /// One row per designated CHART, in designation order (the order
    /// `open_faces` first names each chart).
    pub rims: Vec<RimNaming>,
    /// Result SOLID ← the operand shell whose thin wall it is, one row
    /// per operand shell in shell-arena order: the operand's own solid
    /// for its outer shell, the minted solid for each void. A
    /// single-shell operand lists one row. Historical, like `inner`: a
    /// void whose face is designated open fuses into its twin in the
    /// rim surgery, so its row then names a shell listed in
    /// `dead.shells` — the solid column stays live; a consumer that
    /// needs a live shell checks `dead` or the body.
    pub thickened: Vec<(SolidKey, ShellKey)>,
    /// What the construction retired, result keys.
    pub dead: ShellRetired,
}

impl ShellNaming {
    /// The cavity twin of a source face, in result keys. `None` for a
    /// key the operand did not hold. A twin that has since died is
    /// still answered — the rows are historical — so a consumer that
    /// needs a LIVE key checks `dead` or the body.
    #[must_use]
    pub fn inner_of(&self, source: FaceKey) -> Option<FaceKey> {
        self.inner
            .iter()
            .find(|(_, s)| *s == source)
            .map(|&(twin, _)| twin)
    }

    /// The rim row of the chart a designated face belongs to. `None`
    /// for a face that was not designated.
    #[must_use]
    pub fn rim_of(&self, designated: FaceKey) -> Option<&RimNaming> {
        self.rims.iter().find(|r| r.sources.contains(&designated))
    }

    /// The cavity twin of a source edge, in result keys.
    #[must_use]
    pub fn twin_edge(&self, source: EdgeKey) -> Option<EdgeKey> {
        self.inner_edges
            .iter()
            .find(|(_, s)| *s == source)
            .map(|&(twin, _)| twin)
    }

    /// The cavity twin of a source vertex, in result keys.
    #[must_use]
    pub fn twin_vertex(&self, source: VertexKey) -> Option<VertexKey> {
        self.inner_vertices
            .iter()
            .find(|(_, s)| *s == source)
            .map(|&(twin, _)| twin)
    }
}

/// Which operand shell a designated chart was on — the reading every
/// other field of [`RimNaming`] takes. Decided once, in the sealed arm
/// (the operand's shell roles), and written from that decision; never
/// inferred from the result's keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RimShell {
    /// The designated chart is on the operand's OUTER shell: the
    /// designated face survives as the rim, its cavity counterpart
    /// dies, and the ring's entities are inward twins (rows verbatim
    /// from the `inner_*` rows).
    Outer,
    /// The designated chart is on an operand VOID: the counterpart's
    /// twin survives as the rim (it faces the gap), the designated
    /// face dies, the operand's void shell fuses away, and the ring's
    /// entities are the designated chart's own (each row's two columns
    /// equal; no row is an `inner_*` row).
    Void,
}

/// The rim a designated chart became.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RimNaming {
    /// The designated faces of this chart, source keys, in
    /// designation order.
    pub sources: Vec<FaceKey>,
    /// Which shell of the operand the chart was on, and so which
    /// reading `rim`, `ring`, the ring rows and `dead` take.
    pub side: RimShell,
    /// The rim face (result), now annular. With `side` `Outer` it is
    /// **`sources[0]`** — the chart's faces merge onto the first one
    /// designated, so a caller that wants a particular face to carry
    /// the rim's identity names it first. With `side` `Void` it is the
    /// cavity counterpart's twin (`inner_of(sources[0])`), the face
    /// that survives the glue there (module docs); `sources[0]` is
    /// then in `dead.faces`.
    pub rim: FaceKey,
    /// The rim's RING (a RESULT loop key): the outer loop of the face
    /// the glue killed, as `kfmrh` returned it. The kernel names no
    /// loops to the document layer, so this is a handle into the
    /// result body, not an emitter target; `ring_edges` is the anchor
    /// a `StableName` can be minted from.
    pub ring: LoopKey,
    /// Ring edge (result) ← the source boundary edge of the designated
    /// chart it stands for; ring cycle order. With `side` `Outer` the
    /// ring is the cavity counterpart's boundary, so each ring edge is
    /// an inward twin and every row appears verbatim in
    /// [`ShellNaming::inner_edges`]; with `side` `Void` the ring is the
    /// designated chart's OWN boundary, so each row's two columns are
    /// equal and no row is an inner-twin row.
    pub ring_edges: Vec<(EdgeKey, EdgeKey)>,
    /// Ring vertex (result) ← the source boundary vertex it stands for;
    /// ring cycle order, with the same two readings as `ring_edges`
    /// (twin rows verbatim from [`ShellNaming::inner_vertices`] with
    /// `side` `Outer`, equal columns with `side` `Void`).
    pub ring_vertices: Vec<(VertexKey, VertexKey)>,
    /// A designated face with a hole yields one extra rim region per
    /// hole; pairing order.
    pub holes: Vec<HoleRim>,
}

/// The extra rim region a designated face's HOLE became: the annulus
/// between that hole's boundary and its cavity twin, promoted to its
/// own face (`mfkrh`) before the glue and handed the designated face's
/// own hole after it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoleRim {
    /// The promoted rim face (result).
    pub face: FaceKey,
    /// The loop the promoted face carries as its RING (a RESULT loop
    /// key): the designated face's own hole, as the surgery left it.
    ///
    /// **A result key, not a source key**, and the distinction is not
    /// academic: on an extruded holed slab this key is the operand's
    /// own ring loop, while on a revolve's SLIT annular cap the
    /// designated face carries no ring at all in the operand and this
    /// loop is minted by `kemr` during the chart reduction. Reading it
    /// as a source key is right on one operand and wrong on the other,
    /// which is why the edge-level rows below exist.
    pub ring: LoopKey,
    /// Twin edge (result) ← the source boundary edge of the hole it is
    /// the twin of; the promoted face's CAVITY-side boundary (its
    /// outer loop) in cycle order. Every row appears verbatim in
    /// [`ShellNaming::inner_edges`], so this is the hole's anchor in a
    /// key space the document layer can name.
    pub ring_edges: Vec<(EdgeKey, EdgeKey)>,
    /// Twin vertex (result) ← the source boundary vertex of the hole;
    /// same loop, same order. Every row appears verbatim in
    /// [`ShellNaming::inner_vertices`].
    pub ring_vertices: Vec<(VertexKey, VertexKey)>,
}

/// The result keys the construction retired, in every arena the
/// record names.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ShellRetired {
    /// Faces that no longer resolve: a designated chart's merged-away
    /// members on both sides, and the face the glue kills — the cavity
    /// counterpart on an outer-shell designation, the designated face
    /// itself on a void's.
    pub faces: Vec<FaceKey>,
    /// Edges that no longer resolve: the seam and slit edges the chart
    /// reduction kills.
    pub edges: Vec<EdgeKey>,
    /// Vertices that no longer resolve: the apex vertices a spur dies
    /// with.
    pub vertices: Vec<VertexKey>,
    /// Loops that no longer resolve: the outer loop of each face a
    /// chart merge kills.
    pub loops: Vec<LoopKey>,
    /// Surfaces reaped when killing their last face orphaned them.
    pub surfaces: Vec<crate::geometry::SurfaceKey>,
    /// The shell the glue fused away: the cavity shell on an
    /// outer-shell designation, the operand's own void shell on a
    /// void's (its dilated twin is the survivor there).
    pub shells: Vec<ShellKey>,
}

// ---------------------------------------------------------------------
// The verb
// ---------------------------------------------------------------------

/// The sealed hollow: every boundary face replaced by its inward
/// offset, the offset boundary inserted as a cavity (module docs).
///
/// `thickness` is the wall thickness in meters and is a MAGNITUDE —
/// each face's own offset direction comes from its orientation, so a
/// reversed face offsets against its chart normal without the caller
/// knowing which faces those are.
///
/// **`tol` is the only tolerance this verb takes**, and it is a
/// witness rather than a value. Where a face's inward offset has no
/// closed form the cavity is a fitted approximation, and what that fit
/// must reach is ε_precision — the same ε tier 3 will re-derive the
/// certificate against — so there is nothing for a caller to choose:
/// the witness travels down the offset chain and the number is read
/// once, at the site that classifies the residual.
///
/// # Errors
///
/// [`ShellError`] — [`ShellError::Band`] when the committed tolerance
/// admits no ambiguity band, the thickness gate, the per-face offset
/// refusals (which are the containment evidence's own decides), the
/// void door's refusals, the closing pcurve mint's refusal, and a
/// result that does not validate.
/// **The scalar must be able to certify**, because this verb validates
/// what it built: its last act is [`validate_geometric`], whose +V
/// invariant is a certified claim. A scalar without certification
/// rights cannot form the call — there is no arm and no refusal — and
/// the recourse is not a weaker shell but the ordinary one, built at a
/// certifying scalar.
pub fn shell<T: Decide + PropsQuadLane + geom_core::CertifiedBounds>(
    body: &Body<T>,
    thickness: T,
    tol: Tol,
) -> Result<Shelled<T>, ShellError<T>> {
    shell_open(body, thickness, &[], tol)
}

/// The opened hollow: [`shell`], then the designated faces re-authored
/// as annular rims (module docs). An empty `open_faces` is exactly
/// [`shell`].
///
/// The result is a CLOSED thin solid in every case — the designated
/// faces do not become holes, they become rims.
///
/// # Errors
///
/// [`ShellError::Band`] when the committed tolerance admits no
/// ambiguity band — this is the door that derives it, for both verbs.
/// Then [`ShellError`] — [`shell`]'s, plus the designation gates (a face
/// must resolve, be named once, leave a nonempty and connected
/// remainder) and the rim surgery's own refusal.
/// The certification bound is [`shell`]'s, for [`shell`]'s reason.
pub fn shell_open<T: Decide + PropsQuadLane + geom_core::CertifiedBounds>(
    body: &Body<T>,
    thickness: T,
    open_faces: &[FaceKey],
    tol: Tol,
) -> Result<Shelled<T>, ShellError<T>> {
    let mut naming = ShellNaming::default();
    // `shell` reaches this door, so both verbs derive here, once.
    let band = Band::linear(tol).map_err(|error| ShellError::Band { error })?;

    // ---- Decide: the thickness. ----
    match decide("shell_thickness", Margin::of(thickness), band) {
        Ok(Sign::Positive) => {}
        _ => return Err(ShellError::Thickness { thickness }),
    }

    // ---- Decide: there is a solid to thicken. ----
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    if solids.is_empty() {
        return Err(ShellError::NoSolid);
    }
    // The operand's own solid partition, read once: which solid every
    // face, edge and vertex belongs to. Every per-solid step below is
    // scoped through it, and the simultaneous doors take the same
    // reading of the same body.
    let partition = crate::offset_together::Scope::whole(body).ok_or(ShellError::Corrupt {
        key: EntityId::Solid(solids[0]),
    })?;

    // ---- Decide: which operand shells are VOIDS, per solid. ----
    //
    // A solid stores no outer designation (`ShellRole`'s docs): a
    // shell's role is the decided sign of its signed volume, and it is
    // read here, once, off the operand — the one flux read this verb
    // makes, and only where some solid is hollow. A single-shell solid's
    // shell is its boundary by arity, so a body of only those reads
    // nothing and its verdict log is untouched. Everything downstream
    // that tells a void from the outer shell reads THIS list, never the
    // result's geometry.
    // The whole body's voids, gathered from the PER-SOLID reads. A
    // solid's roles are its own: one outer boundary each, decided over
    // its own shells and never over another solid's — so a plain
    // neighbour's shell is never classified at all, and a
    // classification that escalated on one could not refuse a solid
    // this verb can answer for.
    let mut voids: Vec<ShellKey> = Vec::new();
    for &solid in &solids {
        let shells = body
            .get_solid(solid)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Solid(solid),
            })?
            .shells
            .clone();
        // A single-shell solid's shell is its boundary by arity, so
        // that solid reads nothing and its verdict log is untouched.
        if shells.len() == 1 {
            continue;
        }
        let roles = crate::props::classify_shells_of(body, &shells, tol)
            .map_err(|error| ShellError::Roles { error })?;
        let outer = roles.iter().filter(|c| c.role == ShellRole::Outer).count();
        if outer != 1 {
            return Err(ShellError::OperandOuterShells { solid, outer });
        }
        voids.extend(
            roles
                .iter()
                .filter(|c| c.role == ShellRole::Void)
                .map(|c| c.shell),
        );
    }

    // ---- Decide: every chart has ONE orientation, and ONE solid. ----
    //
    // A chart moves as one, and the door that moves it is its solid's,
    // so a chart worn by faces of two solids has no single door and no
    // single corner problem. The state is reachable — a disconnecting
    // subtract leaves both components wearing one surface key, and the
    // ownership door can file them as two solids — so the second gate
    // is a refusal, not an assertion.
    let charts = chart_groups(body);
    let chart_solid = |group: &[FaceKey]| -> Result<SolidKey, ShellError<T>> {
        partition.solid_of(group[0]).ok_or(ShellError::Corrupt {
            key: EntityId::Face(group[0]),
        })
    };
    for group in &charts {
        let sense = |f: FaceKey| -> Result<bool, ShellError<T>> {
            Ok(body
                .get_face(f)
                .ok_or(ShellError::Corrupt {
                    key: EntityId::Face(f),
                })?
                .sense)
        };
        let first = sense(group[0])?;
        let home = chart_solid(group)?;
        for &member in &group[1..] {
            if sense(member)? != first {
                return Err(ShellError::ChartSenseMixed {
                    face: group[0],
                    other: member,
                });
            }
            if partition.solid_of(member) != Some(home) {
                return Err(ShellError::ChartSpansSolids {
                    face: group[0],
                    other: member,
                });
            }
        }
    }

    // ---- Decide: the walls are thick enough to hold two offsets. ----
    wall_clearance(body, &partition, thickness, band)?;

    // ---- Decide: the designation. ----
    check_designation(body, open_faces)?;

    // ---- The record: the outer wall, one row per undesignated face.
    // Written HERE, off the operand's own face walk, because this is
    // the last moment the designation and the operand's arena are both
    // in hand and nothing has been built yet.
    for (face, _) in body.faces() {
        if !open_faces.contains(&face) {
            naming.outer.push((face, face));
        }
    }

    // ---- The cavity: one clone, every CHART inward. ----
    //
    // By chart, not by face: a full revolve splits its wall into two
    // bands over one cylinder, and such a surface has to move as one
    // (the face-replacement door's own group form says why). Grouping
    // is by surface key, in face-arena order, so the walk is
    // deterministic.
    let mut cavity = body.clone();
    // **All-planar and AXIAL bodies move SIMULTANEOUSLY; everything
    // else still moves chart by chart.** Composing the per-chart door over a body
    // cannot offset an OBLIQUE junction: a corner is visited once per
    // chart and transported rigidly each time, so it accumulates
    // `Σ dᵢ·nᵢ` where the offset body needs the point satisfying every
    // `nᵢ·x = nᵢ·oᵢ + dᵢ` at once. Those agree exactly when the normals
    // are mutually perpendicular — which is why a box was always right
    // — and diverge otherwise. `ReanchorOffCarrier` is what has been
    // refusing the difference rather than building it, and it stays
    // exactly where it was for every body neither branch takes — a
    // cylinder skew to the body's own axis, a NURBS. The curved
    // corners of a body of REVOLUTION are no longer among them, its
    // torus walls included:
    // `offset_charts_together` solves those in the meridian
    // half-plane, and the branch below picks it.
    //
    // **The door is ONE decision PER SOLID.** A body with a box beside
    // a vessel is neither all-planar nor a body of revolution, and a
    // whole-body reading would put both on the per-chart door — which
    // refuses the vessel's corners it solves alone. The ladder is
    // unchanged; what it reads is the solid's own faces. The cavity
    // and the rim lift read the same ladder over the same solid, so a
    // solid is on the same door on the way in and on the way back out.
    // The operand's partition serves every solid: `cavity` is a clone,
    // so it carries the same keys, and re-aiming the scope at one solid
    // is a `Vec` swap rather than another walk over the whole body.
    //
    // **What that sharing buys is one walk here, not one walk per
    // call.** Each simultaneous door the loop reaches builds its own
    // one-solid scope from its move set (`scope_of_moves`), so the
    // solids ARE walked again, once each: eight solid-walks on the
    // hollow-hollow-open body, nine on box-beside-vessel opened. What
    // is saved is this verb's own reading, which is a whole-body walk
    // and would otherwise be one per solid.
    let mut scope = partition.clone();
    for &solid in &solids {
        scope.re_scope(body, &[solid]).ok_or(ShellError::Corrupt {
            key: EntityId::Solid(solid),
        })?;
        let mine: Vec<&Vec<FaceKey>> = charts.iter().filter(|g| scope.holds_face(g[0])).collect();
        let fallback =
            mine.first()
                .and_then(|g| g.first())
                .copied()
                .ok_or(ShellError::Corrupt {
                    key: EntityId::Solid(solid),
                })?;
        let door = offset_door(&cavity, &scope, band).map_err(|error| ShellError::Face {
            face: offending_face(&cavity, &error).unwrap_or(fallback),
            error: Box::new(error),
        })?;
        match door {
            OffsetDoor::PlanesTogether | OffsetDoor::ChartsTogether => {
                let mut moves: Vec<crate::offset_together::ChartMove<T>> =
                    Vec::with_capacity(mine.len());
                for group in &mine {
                    moves.push(crate::offset_together::ChartMove {
                        faces: (*group).clone(),
                        distance: inward(&cavity, group[0], thickness)?,
                    });
                }
                // `ShellError::Face` carries ONE face, and on this branch the
                // honest one is the face the door's own refusal is about — not
                // the first chart's first face, which names the operand's arena
                // order and nothing about the failure. The door's typed
                // refusals carry a face, a vertex or an edge; the last two are
                // resolved to a face they touch.
                let outcome = if door == OffsetDoor::ChartsTogether {
                    crate::offset_charts_together(&mut cavity, &moves, band, tol)
                } else {
                    crate::offset_planes_together(&mut cavity, &moves, band, tol)
                };
                outcome.map_err(|error| ShellError::Face {
                    face: offending_face(&cavity, &error).unwrap_or(fallback),
                    error: Box::new(error),
                })?;
            }
            OffsetDoor::PerChart => {
                for group in &mine {
                    let face = group[0];
                    let d = inward(&cavity, face, thickness)?;
                    crate::replace_faces_offset(&mut cavity, group, d, band, tol).map_err(
                        |error| ShellError::Face {
                            face,
                            error: Box::new(error),
                        },
                    )?;
                }
            }
        }
    }

    // ---- The evidence: the construction's own decides, carried. ----
    //
    // Every face above minted, which is exactly to say every d-vs-reach
    // margin decided `Positive`. That IS the strict-inside claim (module
    // docs on what it carries and what it does not), so it is stated
    // once, per cavity shell, rather than re-derived by the door.
    let evidence = VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    VoidContainment::Carried {
                        sign: Sign::Positive,
                    },
                )
            })
            .collect(),
    };

    // ---- The insertion (the degenerate no-crossing arm). ----
    //
    // The cavity's arenas are read HERE, before the door consumes the
    // body by value: the offset doors write geometry through
    // `set_face_surface` / `set_edge_curve` and run no Euler operator,
    // so a cavity key is the operand key it was cloned from, and these
    // three walks are the record's source columns in arena order.
    let cavity_faces: Vec<FaceKey> = cavity.faces().map(|(k, _)| k).collect();
    let cavity_edges: Vec<EdgeKey> = cavity.edges().map(|(k, _)| k).collect();
    let cavity_vertices: Vec<VertexKey> = cavity.vertices().map(|(k, _)| k).collect();
    let mut out = body.clone();
    // The cavity is a CLONE of the operand, so a designated face's
    // counterpart carries the same key in the cavity's key space —
    // which is the space `VoidInserted` maps from.
    // **One destination per cavity solid, positionally.** The cavity is
    // a CLONE of the operand and `out` is another, so all three arenas
    // carry the same solid keys in the same slot order — which is the
    // graft's positional contract exactly.
    //
    // This is an INVARIANT, not a refusal, and it is spelled as one: a
    // `SlotMap` clone preserves every slot and version, so the three
    // walks cannot disagree unless `Body::clone` itself stops being a
    // clone. A typed refusal here would advertise a reachable state
    // that is not one, and D9 forbids a panic exactly where a refusal
    // is owed — which this is not.
    debug_assert!(
        cavity.solids().map(|(k, _)| k).eq(solids.iter().copied())
            && out.solids().map(|(k, _)| k).eq(solids.iter().copied()),
        "a clone reordered its solid arena",
    );
    let inserted = insert_voids(&mut out, &solids, cavity, &evidence, tol)
        .map_err(|error| ShellError::Insert { error })?;

    // ---- The record: the inner twins, read off the graft map at the
    // insertion rather than matched afterwards. A cavity entity the
    // map does not carry, or one whose key does not name a source
    // entity, would leave an entity of the result unnameable; both are
    // announced rather than skipped.
    for face in cavity_faces {
        if body.get_face(face).is_none() {
            return Err(ShellError::Corrupt {
                key: EntityId::Face(face),
            });
        }
        let twin = inserted.face(face).ok_or(ShellError::Corrupt {
            key: EntityId::Face(face),
        })?;
        naming.inner.push((twin, face));
    }
    for edge in cavity_edges {
        if body.get_edge(edge).is_none() {
            return Err(ShellError::Corrupt {
                key: EntityId::Edge(edge),
            });
        }
        let twin = inserted.edge(edge).ok_or(ShellError::Corrupt {
            key: EntityId::Edge(edge),
        })?;
        naming.inner_edges.push((twin, edge));
    }
    for vertex in cavity_vertices {
        if body.get_vertex(vertex).is_none() {
            return Err(ShellError::Corrupt {
                key: EntityId::Vertex(vertex),
            });
        }
        let twin = inserted.vertex(vertex).ok_or(ShellError::Corrupt {
            key: EntityId::Vertex(vertex),
        })?;
        naming.inner_vertices.push((twin, vertex));
    }

    // ---- The thin solids: one per operand shell (module docs). ----
    //
    // Every clone shell now sits under the operand's solid, and that is
    // not the ruled shape: a void's dilated twin faces OUTWARD after the
    // door's reversal — it is the outer boundary of the wall around
    // that void — so each operand void and its twin move into a solid
    // of their own. The pairing is the graft map's, and the twin goes
    // first so the minted solid lists its outer shell before its
    // cavity, as a solid born through the void door does.
    for (shell, data) in body.shells() {
        let owner = if voids.contains(&shell) {
            let twin = inserted.shell(shell).ok_or(ShellError::Corrupt {
                key: EntityId::Shell(shell),
            })?;
            out.move_shells_to_new_solid(&[twin, shell])
                .map_err(|error| ShellError::Partition { shell, error })?
        } else {
            data.solid
        };
        naming.thickened.push((owner, shell));
    }

    // The ring walks below ask "which source is this twin of?" once per
    // ring entity. The rows are the data; this is the index over them,
    // built once rather than re-scanned per lookup.
    let twins = TwinIndex::of(&naming);

    // ---- The rim surgery, per designated CHART. ----
    //
    // Per chart, ONCE — not once per designated face. The rim a
    // designation asks for is one region of the mouth plane, and how
    // many faces the operand spent on that region is a fact about the
    // operand's construction (a full revolve's seam) rather than about
    // the rim. Both sides of the glue are reduced to one face carrying
    // proper, mutually disjoint loops first
    // ([`canonicalize_chart`]) — which is exactly the condition that
    // makes the counterpart's boundary an interior-disjoint RING of
    // the designated face instead of a second copy of its own seam.
    //
    // The grouping is read ONCE, before any surgery: a chart's faces
    // merge into one, so a designation read after its own chart's turn
    // would name a key that no longer resolves.
    //
    // The RESULT's own partition, read once here rather than per
    // designation: the thin solids have just been minted, so this is
    // the first moment it exists, and every designation's lift is a
    // re-aiming of it.
    let result_partition =
        crate::offset_together::Scope::whole(&out).ok_or(ShellError::Corrupt {
            key: EntityId::Solid(solids[0]),
        })?;
    // A designation naming a face that no longer resolves is not a
    // silent skip: `check_designation` has already refused a stale one,
    // so every key here groups.
    for &designated in open_faces {
        if out.get_face(designated).is_none() {
            return Err(ShellError::Corrupt {
                key: EntityId::Face(designated),
            });
        }
    }
    for (_, group) in group_by_chart(&out, open_faces.iter().copied()) {
        let designated = group[0];
        let sources: Vec<FaceKey> = group
            .iter()
            .map(|&f| {
                inserted.face(f).ok_or(ShellError::Corrupt {
                    key: EntityId::Face(f),
                })
            })
            .collect::<Result<_, _>>()?;
        // Which shell the designation is on, read off the operand and
        // the roles decided in the sealed arm: this is what assigns the
        // glue's roles below, and what the record reports as `side`.
        let designated_shell = body
            .get_face(designated)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(designated),
            })?
            .shell;
        let side = if voids.contains(&designated_shell) {
            RimShell::Void
        } else {
            RimShell::Outer
        };
        // The solid the surgery happens in, read on the RESULT: the
        // thin solids are already partitioned, so a void designation's
        // face and its counterpart live in the minted solid the void
        // and its twin moved to, not in the operand solid they came
        // from. Both sides of the glue are in it by construction.
        let lift_solid = result_partition
            .solid_of(designated)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(designated),
            })?;

        // Lift the cavity's counterpart chart back onto the designated
        // face's own surface. The distance is read from the two PLANES
        // rather than negated from the way in: the graft's reversal
        // negates a stored plane normal (`revert`'s own contract), so
        // "the way back" is not the arithmetic negation of "the way
        // in", and deriving it from geometry is shorter and sign-safe.
        let counterpart_chart = out
            .get_face(sources[0])
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(sources[0]),
            })?
            .surface;
        let lift_group = faces_wearing(&out, counterpart_chart);
        let back = lift_to(&out, sources[0], designated)?;
        // **The lift is the same corner problem as the cavity**, with
        // one chart moving instead of all of them: the counterpart's
        // rim has to land where the moved plane meets the cavity walls
        // it shares an edge with, and on a CURVED wall that is not
        // where translating the rim puts it. Measured on the bellied
        // pot: the per-chart lift leaves the rim 6.2 mm off the
        // cavity's own sphere and refuses. So an axial body's lift goes
        // through the same simultaneous door, with every OTHER chart
        // named at distance zero — which is what makes it a corner
        // solve rather than a transport, and what keeps those charts
        // and their corners untouched.
        // The same decision as the cavity's (`offset_door`), read on the
        // result body. An ALL-PLANAR body's lift takes the per-chart
        // door on purpose: only ONE chart moves here, and that door
        // re-describes the moved plane's boundary against its UNTOUCHED
        // neighbours — one plane against two fixed ones is exact at
        // every corner, oblique or not, so the composed-door defect
        // (a corner transported once per MOVING chart) cannot arise.
        // Measured on the oblique prisms' caps (`verbs_shell`'s
        // `oblique_planar_prisms_open_at_their_cap`, closed forms).
        //
        // **The solid is the designated face's own, read on the result.**
        // The sealed arm has already partitioned the thin solids, so a
        // void designation's face and its counterpart sit in a minted
        // solid of their own — and the door that lifts them is that
        // solid's, over that solid's charts, exactly as the cavity's
        // door was its solid's.
        let mut lift_scope = result_partition.clone();
        lift_scope
            .re_scope(&out, &[lift_solid])
            .ok_or(ShellError::Corrupt {
                key: EntityId::Solid(lift_solid),
            })?;
        let lift_door = offset_door(&out, &lift_scope, band).map_err(|error| ShellError::Lift {
            face: designated,
            error: Box::new(error),
        })?;
        let outcome = match lift_door {
            OffsetDoor::ChartsTogether => {
                let mut moves: Vec<crate::offset_together::ChartMove<T>> = Vec::new();
                for group in chart_groups(&out) {
                    if !lift_scope.holds_face(group[0]) {
                        continue;
                    }
                    let key = out
                        .get_face(group[0])
                        .ok_or(ShellError::Corrupt {
                            key: EntityId::Face(group[0]),
                        })?
                        .surface;
                    let distance = if key == counterpart_chart {
                        back
                    } else {
                        T::zero()
                    };
                    moves.push(crate::offset_together::ChartMove {
                        faces: group,
                        distance,
                    });
                }
                crate::offset_charts_together(&mut out, &moves, band, tol)
            }
            OffsetDoor::PlanesTogether | OffsetDoor::PerChart => {
                crate::replace_faces_offset(&mut out, &lift_group, back, band, tol)
            }
        };
        outcome.map_err(|error| ShellError::Lift {
            face: designated,
            error: Box::new(error),
        })?;
        // One face per side, loops disjoint. Both reductions retire
        // keys through the Euler doors, and each door's own result is
        // what fills `dead` — recorded at the call, never inferred
        // afterwards from what stopped resolving.
        // The designated chart reduced to one face — the mouth — and
        // its counterpart chart reduced to one.
        let mouth = canonicalize_chart(&mut out, &group, band, &mut naming.dead)?;
        let counterpart = canonicalize_chart(&mut out, &sources, band, &mut naming.dead)?;

        // **The glue's roles.** `kfmrh(host, guest)` kills `guest` and
        // makes its outer loop a ring of `host`, so `host` must be the
        // face whose boundary ENCLOSES the other's. On the outer shell
        // that is the mouth: its counterpart was offset inward and
        // lifted back, so the counterpart's boundary sits strictly
        // inside. On a void the counterpart was DILATED and lifted
        // back, so it is the counterpart's boundary that encloses — the
        // counterpart survives as the rim, facing the gap, and the
        // mouth dies. The role is read off the sealed arm's decided
        // shell list and nothing re-derives it: `ring_outer_contact`
        // below decides CONTACT between the two loops, not which
        // encloses which, and tier 3 states no ring-inside-outer check
        // — an inverted assignment glues the larger loop in as a ring
        // of the smaller face and validates with the right volume
        // (`work/topo/tier3-accepts-a-ring-outside-its-outer-loop.md`).
        // What pins the assignment is structural: the void-ceiling row
        // asserts the designated void face DIES, and the pairing row
        // reads each thin solid's twin through the record.
        let (host, guest) = match side {
            RimShell::Void => (counterpart, mouth),
            RimShell::Outer => (mouth, counterpart),
        };
        let (host_surface, host_sense) = {
            let data = out.get_face(host).ok_or(ShellError::Corrupt {
                key: EntityId::Face(host),
            })?;
            (data.surface, data.sense)
        };
        // Read AFTER the lift and the reduction: on an outer-shell
        // designation `FaceSurface::New` minted a fresh key for the
        // lifted chart, and that key — not the one the graft brought in
        // — is what the guest's descriptions now name.
        let guest_surface = out
            .get_face(guest)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(guest),
            })?
            .surface;

        // The guest's rings are the counterparts of the host's OWN
        // rings — an annular mouth's correct rim is not one region but
        // one per hole plus one for the outer boundary, and a face is
        // one region. Each guest ring is therefore promoted to the
        // outer loop of its own rim face BEFORE the glue (which
        // requires a ring-free guest), and after the glue takes the
        // host's matching ring with it.
        let pairs = pair_rings(&out, host, guest, band)?;
        // ONE list, two readers: `ring_move` walks it to hand each
        // promoted face its hole, and the record's hole rows are read
        // off the same pairs.
        let mut promoted: Vec<(FaceKey, LoopKey)> = Vec::new();
        for &(guest_ring, host_ring) in &pairs {
            let made = out
                .mfkrh(guest_ring, crate::euler::FaceSurface::Shared(host_surface))
                .map_err(|error| ShellError::Rim {
                    face: designated,
                    error,
                })?;
            // The promoted face inherits the HOST's orientation, not
            // the guest's: `mfkrh` with a `Shared` surface mints
            // `sense: true`, and the guest faces the other way. The
            // winding works out by construction — a ring of the guest
            // is wound opposite to the guest's outer loop, i.e. the way
            // an outer loop of a host-facing face must be — and it is
            // not asserted here on that argument alone: tier 3's check
            // 6 reads `sense` against the stored loop windings on every
            // planar face, and check 7 reads the volume the same
            // windings integrate, so a flip either way reds at the
            // verb's own closing `validate_geometric` rather than
            // shipping.
            out.set_face_sense(made.face, host_sense)
                .map_err(|error| ShellError::Rim {
                    face: designated,
                    error,
                })?;
            rename_loop_surface(
                &mut out,
                guest_ring,
                guest_surface,
                host_surface,
                tol,
                designated,
            )?;
            promoted.push((made.face, host_ring));
        }

        // The guest's boundary must now be an interior-disjoint ring
        // of the host — the invariant the validator states as check 9.
        // Refused HERE, naming the shape, rather than left to arrive
        // as a generic at-rest report on a body already built.
        let guest_outer = out
            .get_face(guest)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(guest),
            })?
            .outer;
        let host_outer = out
            .get_face(host)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(host),
            })?
            .outer;
        //
        // An UNDECIDABLE separation refuses too, and does not proceed:
        // the glue is a write, and building on a gap the predicate
        // layer could not certify is exactly the guess D4 forbids.
        // **On the way in through this verb the band is door-shielded**
        // — `shell_thickness` has already decided the wall certifiably
        // positive and every rim it builds is that wall wide — so the
        // escalation arm is not reachable from `shell_open`'s own
        // fixtures. It is here for the bodies OTHER producers hand the
        // same predicate at rest, which is where check 9 does its work.
        match crate::validate::ring_outer_contact(&out, host_outer, guest_outer, band) {
            crate::validate::RingOuterVerdict::Disjoint => {}
            crate::validate::RingOuterVerdict::Contact(_) => {
                return Err(ShellError::OpenFaceRimNotExpressible {
                    face: designated,
                    what: "the cavity counterpart's boundary meets the designated face's own \
                           boundary, so neither can become an interior-disjoint ring of the \
                           other",
                });
            }
            crate::validate::RingOuterVerdict::Escalated(source) => {
                return Err(ShellError::Escalated { source });
            }
        }

        // The SPLIT path's promoted pair, checked the same way and
        // before the glue rather than after it: a promoted rim face's
        // own outer loop and the hole it is about to be handed must be
        // disjoint too, or the second rim region is the first one's
        // defect one ring in. Refused typed, naming the shape, rather
        // than arriving as a generic `NotValid` on a built body.
        for &(guest_ring, host_ring) in &pairs {
            match crate::validate::ring_outer_contact(&out, guest_ring, host_ring, band) {
                crate::validate::RingOuterVerdict::Disjoint => {}
                crate::validate::RingOuterVerdict::Contact(_) => {
                    return Err(ShellError::OpenFaceRimNotExpressible {
                        face: designated,
                        what: "a promoted rim face's own boundary meets the hole it would \
                               carry, so that rim region is not a ring inside a region either",
                    });
                }
                crate::validate::RingOuterVerdict::Escalated(source) => {
                    return Err(ShellError::Escalated { source });
                }
            }
        }

        // The connected sum: the guest dies, its outer loop becomes the
        // host's RING, and the guest's shell fuses into the host's (the
        // first chart does the fusion; any further chart is same-shell
        // genus surgery).
        let fused = out.kfmrh(host, guest).map_err(|error| ShellError::Rim {
            face: designated,
            error,
        })?;
        naming.dead.faces.push(fused.killed_face);
        naming.dead.surfaces.extend(fused.killed_surface);
        naming.dead.shells.extend(fused.killed_shell);
        // The ring's edges still NAME the surface that just died. They
        // lie on the host's surface now — the lift put the two planes
        // on top of each other — so the re-description is a key swap
        // with the carrier untouched, and the attach layer certifies it
        // against the geometry rather than taking the swap on trust.
        rename_loop_surface(
            &mut out,
            fused.ring,
            guest_surface,
            host_surface,
            tol,
            designated,
        )?;
        // The record's ring rows, walked off the ring `kfmrh` just
        // returned. On an outer-shell designation each ring entity is
        // a cavity twin, so its source is the row the graft map wrote
        // for it above — the designated chart's own boundary edge or
        // vertex; on a void designation the ring IS that boundary, and
        // each entity is its own source. An entity with neither reading
        // is a mint this record cannot explain, and it says so rather
        // than leaving a gap.
        let rows = match side {
            RimShell::Void => RingSource::Operand(body),
            RimShell::Outer => RingSource::Twins(&twins),
        };
        let (ring_edges, ring_vertices) = ring_rows(&out, fused.ring, &rows)?;

        // The hole rows read the same way, off each promoted face's
        // guest-side boundary — its outer loop, which `mfkrh` made from
        // the guest's ring — so a hole gets the same edge-level anchor
        // the outer rim has.
        let mut holes = Vec::with_capacity(promoted.len());
        for &(face, host_ring) in &promoted {
            let outer = out
                .get_face(face)
                .ok_or(ShellError::Corrupt {
                    key: EntityId::Face(face),
                })?
                .outer;
            let (ring_edges, ring_vertices) = ring_rows(&out, outer, &rows)?;
            holes.push(HoleRim {
                face,
                ring: host_ring,
                ring_edges,
                ring_vertices,
            });
        }

        // Each promoted rim face takes its matching hole with it: the
        // fusion put every face in one shell, which is `ring_move`'s
        // precondition.
        for (face, host_ring) in promoted {
            out.ring_move(host_ring, face)
                .map_err(|error| ShellError::Rim {
                    face: designated,
                    error,
                })?;
        }

        naming.rims.push(RimNaming {
            sources: group,
            side,
            rim: host,
            ring: fused.ring,
            ring_edges,
            ring_vertices,
            holes,
        });
    }

    // ---- The closing mint (module docs, "The closing mint"). ----
    //
    // Placed where the boolean places its own, after the last write.
    // The position is NOT pinned by any row: moved to just before the
    // partition the whole suite stays green (every step after the door
    // is `Neither` for rows and the lift doors mint their clone
    // whole-body). What would pin it is a designated CURVED chart,
    // whose rim surgery would write rows after the door; that
    // designation refuses `OpenFaceRingUnsupported` today.
    mint_pcurves(&mut out, tol).map_err(|source| ShellError::Pcurve { source })?;

    // ---- One validation. ----
    validate_geometric(&out, tol).map_err(|errors| ShellError::NotValid { errors })?;
    Ok(Shelled { body: out, naming })
}

/// A rim ring's two row lists: its edge rows and its vertex rows.
type RingRows = (Vec<(EdgeKey, EdgeKey)>, Vec<(VertexKey, VertexKey)>);

/// The inner-twin rows read backwards — result key to the source key
/// it twins — so a ring walk is one lookup per entity rather than a
/// scan of the rows.
struct TwinIndex {
    edges: SecondaryMap<EdgeKey, EdgeKey>,
    vertices: SecondaryMap<VertexKey, VertexKey>,
}

impl TwinIndex {
    fn of(naming: &ShellNaming) -> Self {
        let mut edges = SecondaryMap::new();
        for &(result, source) in &naming.inner_edges {
            edges.insert(result, source);
        }
        let mut vertices = SecondaryMap::new();
        for &(result, source) in &naming.inner_vertices {
            vertices.insert(result, source);
        }
        Self { edges, vertices }
    }
}

/// Where a rim ring's entities come from — the two readings of a ring
/// row's source column.
enum RingSource<'a, T: Real> {
    /// The ring is the cavity counterpart's boundary (an outer-shell
    /// designation): every entity is an inward twin, and its source is
    /// the inner-twin rows' answer.
    Twins(&'a TwinIndex),
    /// The ring is the designated chart's OWN boundary (a void
    /// designation): every entity is an operand entity surviving under
    /// its key, so its source is itself — checked to resolve in the
    /// operand rather than assumed.
    Operand(&'a Body<T>),
}

/// The rim ring's edge and vertex rows, in ring cycle order: each
/// result entity paired with the source entity it stands for, read
/// through `source`.
///
/// The lookup is total by construction — a ring of the fused rim is
/// the outer loop of the face the glue killed, whose entities are
/// either cavity entities (every one has a twin row) or the operand's
/// own — so a miss is a ring entity the record cannot explain and is
/// announced as corruption rather than dropped.
fn ring_rows<T: Real>(
    body: &Body<T>,
    ring: LoopKey,
    source: &RingSource<'_, T>,
) -> Result<RingRows, ShellError<T>> {
    let corrupt = |key| ShellError::Corrupt { key };
    let source_edge = |edge: EdgeKey| -> Option<EdgeKey> {
        match source {
            RingSource::Twins(twins) => twins.edges.get(edge).copied(),
            RingSource::Operand(operand) => operand.get_edge(edge).map(|_| edge),
        }
    };
    let source_vertex = |vertex: VertexKey| -> Option<VertexKey> {
        match source {
            RingSource::Twins(twins) => twins.vertices.get(vertex).copied(),
            RingSource::Operand(operand) => operand.get_vertex(vertex).map(|_| vertex),
        }
    };
    let LoopBoundary::Cycle { first } = body
        .get_loop(ring)
        .ok_or_else(|| corrupt(EntityId::Loop(ring)))?
        .boundary
    else {
        return Ok((Vec::new(), Vec::new()));
    };
    let cycle = body
        .loop_cycle(first)
        .ok_or_else(|| corrupt(EntityId::HalfEdge(first)))?;
    let mut edges = Vec::with_capacity(cycle.len());
    let mut vertices = Vec::with_capacity(cycle.len());
    for he in cycle {
        let half = body
            .get_half_edge(he)
            .ok_or_else(|| corrupt(EntityId::HalfEdge(he)))?;
        let source = source_edge(half.edge).ok_or_else(|| corrupt(EntityId::Edge(half.edge)))?;
        edges.push((half.edge, source));
        let source =
            source_vertex(half.start).ok_or_else(|| corrupt(EntityId::Vertex(half.start)))?;
        vertices.push((half.start, source));
    }
    Ok((edges, vertices))
}

/// **One face per chart, loops disjoint** — the shape the rim glue's
/// only output form needs on both sides of it.
///
/// A chart arrives from a revolve carrying that construction's seam:
/// an axis-touching cap is TWO faces meeting along a diameter, and an
/// annular cap is one face slit radially, its loop walking the seam
/// edge in both directions. Neither is a fact about the region — both
/// are facts about how the operand was swept — and both are exactly
/// what makes a counterpart's boundary land ON the designated face's
/// boundary instead of strictly inside it. This reduces them, through
/// the Euler doors and nothing else:
///
/// 1. the chart's faces merge across the edges only they share
///    (`kef`), leaving the merged loop walking each killed edge's
///    surviving partner twice;
/// 2. a SPUR — such a duplicate whose far vertex the merge left with
///    one edge on it, the axis apex of a revolved cap — dies with that
///    vertex (`kev`);
/// 3. a SLIT — a duplicate still anchored at both ends, an annular
///    cap's radial seam — splits the loop in two (`kemr`), the
///    inner side becoming the ring it always was.
///
/// Returns the surviving face. A chart this cannot reduce refuses
/// typed rather than gluing onto a shape it does not have.
///
/// Every operator's own result is appended to `dead` at the call, so
/// the birth record's retirement rows are what the reduction did
/// rather than a later reading of what stopped resolving.
fn canonicalize_chart<T: Decide>(
    body: &mut Body<T>,
    faces: &[FaceKey],
    band: Band,
    dead: &mut ShellRetired,
) -> Result<FaceKey, ShellError<T>> {
    let anchor = *faces.first().ok_or(ShellError::Corrupt {
        key: EntityId::Face(FaceKey::default()),
    })?;
    let not_expressible =
        |what: &'static str| ShellError::OpenFaceRimNotExpressible { face: anchor, what };

    // ---- 1: one face. ----
    let mut alive: Vec<FaceKey> = faces.to_vec();
    while alive.len() > 1 {
        let edges: Vec<crate::entity::EdgeKey> = body.edges().map(|(k, _)| k).collect();
        let mut acted = false;
        for edge in edges {
            let Some((fp, fm)) = crate::replace_face::edge_faces(body, edge) else {
                continue;
            };
            if fp == fm || !alive.contains(&fp) || !alive.contains(&fm) {
                continue;
            }
            let data = body.get_edge(edge).ok_or(ShellError::Corrupt {
                key: EntityId::Edge(edge),
            })?;
            // `kef` kills the face of the half-edge it is given, and
            // refuses a dying face that carries rings.
            let ring_free =
                |body: &Body<T>, f: FaceKey| body.get_face(f).is_some_and(|d| d.rings.is_empty());
            let (dying, he) = if fm != anchor && ring_free(body, fm) {
                (fm, data.he_minus)
            } else if fp != anchor && ring_free(body, fp) {
                (fp, data.he_plus)
            } else {
                continue;
            };
            let killed = body.kef(he).map_err(|error| ShellError::Rim {
                face: anchor,
                error,
            })?;
            dead.faces.push(killed.killed_face);
            dead.edges.push(killed.killed_edge);
            dead.loops.push(killed.killed_loop);
            dead.surfaces.extend(killed.killed_surface);
            alive.retain(|&f| f != dying);
            acted = true;
            break;
        }
        if !acted {
            return Err(not_expressible(
                "the designated chart's faces do not merge into one region through the edges \
                 they share",
            ));
        }
    }

    // ---- 2 and 3: proper loops. ----
    while let Some((r#loop, he1, he2)) = duplicate_in_loop(body, anchor) {
        let far = |he| body.half_edge_end(he);
        if far(he1).is_some_and(|v| valence(body, v) == 1) {
            let killed = body.kev(he1).map_err(|error| ShellError::Rim {
                face: anchor,
                error,
            })?;
            dead.edges.push(killed.killed_edge);
            dead.vertices.push(killed.killed_vertex);
            continue;
        }
        if far(he2).is_some_and(|v| valence(body, v) == 1) {
            let killed = body.kev(he2).map_err(|error| ShellError::Rim {
                face: anchor,
                error,
            })?;
            dead.edges.push(killed.killed_edge);
            dead.vertices.push(killed.killed_vertex);
            continue;
        }
        // The slit's two sides, in cycle order: the run strictly after
        // `he1` up to `he2`, and the run strictly after `he2` up to
        // `he1`. `kemr` makes its FIRST argument's side the ring, so
        // the argument order is the role assignment, decided by which
        // side the other encloses.
        let (side1, side2) = split_cycle(body, r#loop, he1, he2)
            .ok_or_else(|| not_expressible("the chart's slit loop does not split in two"))?;
        let (p1, p2) = (
            half_edge_points(body, &side1),
            half_edge_points(body, &side2),
        );
        let ring_first = if encloses(&p1, &p2, band) {
            true
        } else if encloses(&p2, &p1, band) {
            false
        } else {
            return Err(not_expressible(
                "the chart's slit loop splits into two sides neither of which encloses the \
                 other, so neither is the hole",
            ));
        };
        let (a, b) = if ring_first { (he1, he2) } else { (he2, he1) };
        let made = body.kemr(a, b).map_err(|error| ShellError::Rim {
            face: anchor,
            error,
        })?;
        dead.edges.push(made.killed_edge);
        // The role assignment is verified, not assumed: the ring must
        // be the enclosed side.
        let (ring_pts, outer_pts) = {
            let outer = body
                .get_face(anchor)
                .ok_or(ShellError::Corrupt {
                    key: EntityId::Face(anchor),
                })?
                .outer;
            (loop_points(body, made.ring), loop_points(body, outer))
        };
        if !encloses(&ring_pts, &outer_pts, band) {
            return Err(not_expressible(
                "the chart's slit loop split with the enclosing side as the ring",
            ));
        }
    }
    Ok(anchor)
}

/// Pair the cavity counterpart's hole with the designated face's own.
///
/// A designated face's rim is one region per BOUNDARY the counterpart
/// sits inside: the annulus between the two outer loops, plus one
/// annulus per hole. This returns the `(source_ring, rim_ring)`
/// correspondence those extra regions need.
///
/// **Scope, stated rather than implied**: zero or ONE hole. A
/// designated face with two or more holes has a pairing this door does
/// not derive — the enclosure question stops being the single
/// comparison below — and refuses typed rather than guessing at it.
fn pair_rings<T: Decide>(
    body: &Body<T>,
    rim: FaceKey,
    source: FaceKey,
    band: Band,
) -> Result<Vec<(crate::entity::LoopKey, crate::entity::LoopKey)>, ShellError<T>> {
    let rings_of = |face: FaceKey| -> Result<Vec<crate::entity::LoopKey>, ShellError<T>> {
        Ok(body
            .get_face(face)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(face),
            })?
            .rings
            .clone())
    };
    let rim_rings = rings_of(rim)?;
    let source_rings = rings_of(source)?;
    let not_expressible =
        |what: &'static str| ShellError::OpenFaceRimNotExpressible { face: rim, what };
    match (&rim_rings[..], &source_rings[..]) {
        ([], []) => Ok(Vec::new()),
        ([rim_ring], [source_ring]) => {
            if encloses(
                &loop_points(body, *source_ring),
                &loop_points(body, *rim_ring),
                band,
            ) {
                Err(not_expressible(
                    "the designated face's hole does not sit inside the cavity counterpart's",
                ))
            } else if encloses(
                &loop_points(body, *rim_ring),
                &loop_points(body, *source_ring),
                band,
            ) {
                Ok(vec![(*source_ring, *rim_ring)])
            } else {
                Err(not_expressible(
                    "the designated face's hole and the cavity counterpart's do not nest",
                ))
            }
        }
        _ => Err(not_expressible(
            "the designated face and its cavity counterpart do not each carry the same single \
             hole, and this door pairs no more than one",
        )),
    }
}

/// A loop of `face` that walks one edge in BOTH directions, with the
/// two halves in cycle order — the seam remnant a chart merge leaves,
/// and the slit a full revolve of a closed profile is born with.
fn duplicate_in_loop<T: Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Option<(crate::entity::LoopKey, HeKey, HeKey)> {
    let data = body.get_face(face)?;
    for r#loop in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(r#loop)?.boundary else {
            continue;
        };
        let cycle = body.loop_cycle(first)?;
        for (i, &he1) in cycle.iter().enumerate() {
            let e1 = body.get_half_edge(he1)?.edge;
            for &he2 in &cycle[i + 1..] {
                if body.get_half_edge(he2)?.edge == e1 {
                    return Some((r#loop, he1, he2));
                }
            }
        }
    }
    None
}

/// The two runs a loop's cycle falls into when `he1` and `he2` are
/// removed: the halves strictly after `he1` up to `he2`, and the
/// halves strictly after `he2` up to `he1`.
fn split_cycle<T: Real>(
    body: &Body<T>,
    r#loop: crate::entity::LoopKey,
    he1: HeKey,
    he2: HeKey,
) -> Option<(Vec<HeKey>, Vec<HeKey>)> {
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop)?.boundary else {
        return None;
    };
    let cycle = body.loop_cycle(first)?;
    let i = cycle.iter().position(|&he| he == he1)?;
    let j = cycle.iter().position(|&he| he == he2)?;
    let (lo, hi) = if i < j { (i, j) } else { (j, i) };
    let between: Vec<HeKey> = cycle[lo + 1..hi].to_vec();
    let around: Vec<HeKey> = cycle[hi + 1..]
        .iter()
        .chain(&cycle[..lo])
        .copied()
        .collect();
    if i < j {
        Some((between, around))
    } else {
        Some((around, between))
    }
}

/// How many edges emanate from a vertex.
fn valence<T: Real>(body: &Body<T>, vertex: crate::entity::VertexKey) -> usize {
    body.get_vertex(vertex)
        .and_then(|d| d.emanating)
        .and_then(|he| body.vertex_orbit(he))
        .map_or(0, |orbit| orbit.len())
}

/// Sampled points along a run of half-edges — each edge at the
/// certification schedule's own parameters, so a full-period arc is
/// read as the arc rather than as its (collapsed) chord.
fn half_edge_points<T: Decide>(body: &Body<T>, run: &[HeKey]) -> Vec<geom_core::Point3<T>> {
    let mut out = Vec::new();
    for &he in run {
        let Some(geom) = body
            .get_half_edge(he)
            .and_then(|h| body.get_edge(h.edge))
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(crate::null::CurveGeom::certified)
        else {
            continue;
        };
        for i in 0..=8 {
            out.push(geom.carrier().eval(geom.sample_param(i)));
        }
    }
    out
}

/// [`half_edge_points`] over a whole loop.
fn loop_points<T: Decide>(
    body: &Body<T>,
    r#loop: crate::entity::LoopKey,
) -> Vec<geom_core::Point3<T>> {
    let Some(LoopBoundary::Cycle { first }) = body.get_loop(r#loop).map(|l| l.boundary) else {
        return Vec::new();
    };
    let Some(cycle) = body.loop_cycle(first) else {
        return Vec::new();
    };
    half_edge_points(body, &cycle)
}

/// Whether `inner` is the ENCLOSED one of two nested coplanar loops,
/// by mean radius about the pair's common centroid.
///
/// The comparison is a mean radius and not a containment proof, and
/// that is exactly the claim the callers need: the loops compared here
/// are always the two boundaries ONE offset produced from the other —
/// a slit chart's two sides, or a hole and its own offset — so they
/// are concentric by construction and the mean radius is their nesting
/// order. Anything else refuses typed at the call site rather than
/// being read off this. One margin, one decide, metered as the length
/// it is.
fn encloses<T: Decide>(
    inner: &[geom_core::Point3<T>],
    outer: &[geom_core::Point3<T>],
    band: Band,
) -> bool {
    if inner.is_empty() || outer.is_empty() {
        return false;
    }
    let centre = centroid_of(&[inner, outer]);
    let gap = mean_radius(outer, centre) - mean_radius(inner, centre);
    matches!(
        decide("shell_rim_nesting", Margin::of(gap), band),
        Ok(Sign::Positive)
    )
}

/// The centroid of several point runs, accumulated from the first
/// point so the sum stays local to the geometry's own scale.
fn centroid_of<T: Real>(runs: &[&[geom_core::Point3<T>]]) -> geom_core::Point3<T> {
    let base = runs
        .iter()
        .find_map(|run| run.first().copied())
        .unwrap_or(geom_core::Point3::new(T::zero(), T::zero(), T::zero()));
    let mut sum = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
    let mut n = 0usize;
    for run in runs {
        for p in *run {
            sum = sum + (*p - base);
            n += 1;
        }
    }
    if n == 0 {
        return base;
    }
    base + sum / T::from_f64(n as f64)
}

/// The mean distance of a point run from `centre`.
fn mean_radius<T: Real>(points: &[geom_core::Point3<T>], centre: geom_core::Point3<T>) -> T {
    let mut sum = T::zero();
    for p in points {
        sum = sum + (*p - centre).norm();
    }
    sum / T::from_f64(points.len() as f64)
}

/// Which offset door moves a body's charts — ONE decision, read on a
/// structural property of the body before anything is written, and
/// read the same way by the cavity and by the rim lift (module docs on
/// what each door solves).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OffsetDoor {
    /// Every face is a plane: [`crate::offset_planes_together`], every
    /// chart at once, each corner solved against all the moved planes.
    PlanesTogether,
    /// A body of revolution: [`crate::offset_charts_together`], each
    /// corner solved in the meridian half-plane.
    ChartsTogether,
    /// Anything else: [`crate::replace_faces_offset`] chart by chart,
    /// whose oblique corners refuse rather than build.
    PerChart,
}

/// The door for the solids `scope` names. A door is a property of a
/// SOLID — a box beside a vessel is neither all-planar nor axial while
/// each of the two is one of those — so the ladder reads that solid's
/// own faces and nothing else. An UNDECIDED axis gate is not
/// `PerChart`: it escalates typed, and the caller refuses with it
/// rather than taking the other branch (`is_axial`'s docs).
fn offset_door<T: Decide>(
    body: &Body<T>,
    scope: &crate::offset_together::Scope,
    band: Band,
) -> Result<OffsetDoor, ReplaceFaceError<T>> {
    let all_planar = body
        .faces()
        .filter(|(k, _)| scope.holds_face(*k))
        .all(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { .. })
            )
        });
    if all_planar {
        return Ok(OffsetDoor::PlanesTogether);
    }
    Ok(if crate::offset_axial::is_axial_in(body, scope, band)? {
        OffsetDoor::ChartsTogether
    } else {
        OffsetDoor::PerChart
    })
}

/// The face a simultaneous-door refusal is about, where it names one
/// or names an entity that touches one.
fn offending_face<T: Real>(body: &Body<T>, error: &ReplaceFaceError<T>) -> Option<FaceKey> {
    let face_of_he =
        |he| -> Option<FaceKey> { Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face) };
    match error {
        ReplaceFaceError::StaleFace { face }
        | ReplaceFaceError::TogetherNonPlanar { face, .. }
        | ReplaceFaceError::TogetherPartialSet { face }
        | ReplaceFaceError::TogetherChartMixed { face, .. }
        | ReplaceFaceError::TogetherFaceRepeated { face }
        | ReplaceFaceError::TogetherAxialUnsupported { face, .. }
        | ReplaceFaceError::TogetherNotAxial { face, .. }
        | ReplaceFaceError::NappeStraddles { face, .. } => Some(*face),
        ReplaceFaceError::TogetherCorner { vertex, .. }
        | ReplaceFaceError::TogetherAxialCorner { vertex, .. } => {
            face_of_he(body.get_vertex(*vertex)?.emanating?)
        }
        ReplaceFaceError::TogetherEdgeDisagreement { edge, .. }
        | ReplaceFaceError::TogetherAxialEdge { edge, .. }
        | ReplaceFaceError::ReanchorOffCarrier { edge, .. } => {
            face_of_he(body.get_edge(*edge)?.he_plus)
        }
        _ => None,
    }
}

/// The signed distance along `from`'s chart normal that lands it on
/// `onto`'s plane. Both are planar — a curved designation is refused
/// upstream — so this is one dot product and no solve.
fn lift_to<T: Real>(body: &Body<T>, from: FaceKey, onto: FaceKey) -> Result<T, ShellError<T>> {
    let plane =
        |face: FaceKey| -> Result<(geom_core::Point3<T>, geom_core::Vec3<T>), ShellError<T>> {
            let data = body.get_face(face).ok_or(ShellError::Corrupt {
                key: EntityId::Face(face),
            })?;
            match body.get_surface(data.surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => Ok((*origin, *normal)),
                _ => Err(ShellError::Corrupt {
                    key: EntityId::Face(face),
                }),
            }
        };
    let (o_from, n_from) = plane(from)?;
    let (o_onto, _) = plane(onto)?;
    Ok((o_onto - o_from).dot(n_from))
}

/// Re-points every description on `r#loop` that names `dead` at
/// `live`, re-certifying each through the attach layer. `rim` names
/// the designated face in any refusal and is otherwise unread.
fn rename_loop_surface<T: Decide>(
    body: &mut Body<T>,
    r#loop: crate::entity::LoopKey,
    dead: crate::geometry::SurfaceKey,
    live: crate::geometry::SurfaceKey,
    tol: Tol,
    rim: FaceKey,
) -> Result<(), ShellError<T>> {
    let corrupt = |key| ShellError::Corrupt { key };
    let ring = r#loop;
    let LoopBoundary::Cycle { first } = body
        .get_loop(ring)
        .ok_or_else(|| corrupt(EntityId::Loop(ring)))?
        .boundary
    else {
        return Ok(());
    };
    let cycle = body
        .loop_cycle(first)
        .ok_or_else(|| corrupt(EntityId::HalfEdge(first)))?;
    let mut specs = Vec::new();
    for he in cycle {
        let edge = body
            .get_half_edge(he)
            .ok_or_else(|| corrupt(EntityId::HalfEdge(he)))?
            .edge;
        let curve = body
            .get_curve_geom(
                body.get_edge(edge)
                    .ok_or_else(|| corrupt(EntityId::Edge(edge)))?
                    .curve,
            )
            .and_then(crate::null::CurveGeom::certified)
            .ok_or_else(|| corrupt(EntityId::Edge(edge)))?;
        let (param_start, param_end) = curve.params();
        specs.push((
            edge,
            geom_brep::EdgeCurveSpec {
                description: crate::replace_face::remap_description(
                    curve.restated_description(),
                    dead,
                    live,
                ),
                carrier: curve.carrier().clone(),
                param_start,
                param_end,
            },
        ));
    }
    for (edge, spec) in specs {
        body.set_edge_curve(edge, spec, tol)
            .map_err(|error| ShellError::Rim { face: rim, error })?;
    }
    Ok(())
}

/// **The closed-form wall-clearance gate** (module docs). Every pair of
/// non-adjacent PLANAR faces that face each other — antiparallel
/// outward normals, footprints overlapping in projection — must have at
/// least `2·thickness` of material between them, or their inward
/// offsets cross.
///
/// **Why planar only, and what that leaves open.** A plane's reach is
/// unbounded, so the per-face collapse margins are vacuous on exactly
/// the faces that can collide; the closed form here is what replaces
/// them. The CURVED residue is a documented window, not an oversight:
/// a box-based test is unusable there because a shelled tube's
/// concentric walls overlap boxes by construction, so a curved gate of
/// that shape would refuse the verb's own acceptance fixtures. **A
/// curved thin neck can still shell silently wrong** until the general
/// clearance certificate over a parameter box lands — issue #1055,
/// aimed at M10.
///
/// **Conservative in the #571 direction.** Footprint overlap is tested
/// on projected bounding boxes GROWN by `thickness` on every side —
/// the footprint an inward offset has past a concave edge
/// ([`footprints_may_overlap`]) — and an ambiguous or escalating box
/// comparison counts as OVERLAPPING. The gate may therefore refuse a
/// staircase body whose faces do not really face each other, or a
/// convex-edged pair whose offsets would have cleared; it cannot miss
/// a planar pair that crosses.
fn wall_clearance<T: Decide>(
    body: &Body<T>,
    partition: &crate::offset_together::Scope,
    thickness: T,
    band: Band,
) -> Result<(), ShellError<T>> {
    let two_t = thickness + thickness;
    let planes = planar_faces(body, partition)?;
    for (i, a) in planes.iter().enumerate() {
        for b in &planes[i + 1..] {
            // **A pair of DIFFERENT solids never gates.** The gate is
            // about a WALL — material between two offsets that would
            // cross — and there is no material between two solids: two
            // parts facing each other across space each thicken into
            // their own material, however close they stand.
            if a.solid != b.solid {
                continue;
            }
            // Facing each other: outward normals antiparallel.
            let anti = Margin::of(-(a.normal.dot(b.normal)) - T::one());
            if !matches!(
                decide("shell_walls_antiparallel", anti, band),
                Ok(Sign::Zero)
            ) {
                continue;
            }
            if face_neighbours(body, a.face)?.contains(&b.face) {
                continue;
            }
            if !footprints_may_overlap(a, b, thickness, band) {
                continue;
            }
            let gap = (b.origin - a.origin).dot(a.normal).abs();
            match decide("shell_wall_clearance", Margin::of(gap - two_t), band)
                .map_err(|source| ShellError::Escalated { source })?
            {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(ShellError::WallClearance {
                        face: a.face,
                        other: b.face,
                        gap,
                        needed: two_t,
                    });
                }
            }
        }
    }
    Ok(())
}

/// One planar face reduced to what the clearance gate reads: its
/// OUTWARD normal, a point on it, an in-plane frame, and the projected
/// footprint of its boundary in that frame.
struct PlanarFace<T: Real> {
    face: FaceKey,
    /// The solid the face belongs to: the gate is a claim about ONE
    /// solid's material, so a pair that straddles two is not a wall.
    solid: SolidKey,
    origin: geom_core::Point3<T>,
    normal: geom_core::Vec3<T>,
    u_ref: geom_core::Vec3<T>,
    v_ref: geom_core::Vec3<T>,
    box_u: (T, T),
    box_v: (T, T),
}

/// Every planar face of `body`, with its outward normal and projected
/// footprint.
fn planar_faces<T: Real>(
    body: &Body<T>,
    partition: &crate::offset_together::Scope,
) -> Result<Vec<PlanarFace<T>>, ShellError<T>> {
    let mut out = Vec::new();
    for (face, data) in body.faces() {
        let Some(geom::Surface::Plane {
            origin,
            normal,
            u_ref,
        }) = body.get_surface(data.surface)
        else {
            continue;
        };
        // Outward is the chart normal on a positively-sensed face.
        let normal = if data.sense { *normal } else { -*normal };
        let v_ref = normal.cross(*u_ref);
        let mut box_u: Option<(T, T)> = None;
        let mut box_v: Option<(T, T)> = None;
        for point in face_boundary_points(body, face)? {
            let w = point - *origin;
            let (u, v) = (w.dot(*u_ref), w.dot(v_ref));
            box_u = Some(match box_u {
                None => (u, u),
                Some((lo, hi)) => (lo.min(u), hi.max(u)),
            });
            box_v = Some(match box_v {
                None => (v, v),
                Some((lo, hi)) => (lo.min(v), hi.max(v)),
            });
        }
        let (Some(box_u), Some(box_v)) = (box_u, box_v) else {
            continue;
        };
        out.push(PlanarFace {
            face,
            solid: partition.solid_of(face).ok_or(ShellError::Corrupt {
                key: EntityId::Face(face),
            })?,
            origin: *origin,
            normal,
            u_ref: *u_ref,
            v_ref,
            box_u,
            box_v,
        });
    }
    Ok(out)
}

/// Do the two footprints overlap when both are projected into `a`'s
/// in-plane frame, once each has been GROWN by `grow` on every side?
/// `true` on any ambiguity — the conservative answer.
///
/// **Why the growth.** The gate reads the OPERAND's faces, but what
/// collides is their inward offsets, and an inward offset extends past
/// every CONCAVE edge of its face by the offset distance (the two moved
/// planes meet further out) while it retracts by that much at a convex
/// one. Two faces whose operand footprints are disjoint by less than
/// `2t` therefore have offsets whose footprints overlap, and a gate
/// that compared the operand's boxes would miss exactly the pair whose
/// walls cross — an S-bend's two risers, two box voids offset
/// diagonally (every edge of a void is concave from the material's
/// side). Growing each box by `t` reads the offset's footprint at a
/// concave edge exactly and over-reads it at a convex one, which is
/// the #571 direction: it may refuse a pair that would have cleared,
/// never pass one that crosses.
fn footprints_may_overlap<T: Decide>(
    a: &PlanarFace<T>,
    b: &PlanarFace<T>,
    grow: T,
    band: Band,
) -> bool {
    let grown = |(lo, hi): (T, T)| (lo - grow, hi + grow);
    // `b`'s box is expressed in `b`'s own frame; re-express its corners
    // in `a`'s. The two planes are parallel, so this is a 2-D rigid
    // change of basis and the box is re-hulled from the four corners.
    let mut re_u: Option<(T, T)> = None;
    let mut re_v: Option<(T, T)> = None;
    for (u, v) in [
        (b.box_u.0, b.box_v.0),
        (b.box_u.0, b.box_v.1),
        (b.box_u.1, b.box_v.0),
        (b.box_u.1, b.box_v.1),
    ] {
        let p = b.origin + b.u_ref * u + b.v_ref * v;
        let w = p - a.origin;
        let (pu, pv) = (w.dot(a.u_ref), w.dot(a.v_ref));
        re_u = Some(match re_u {
            None => (pu, pu),
            Some((lo, hi)) => (lo.min(pu), hi.max(pu)),
        });
        re_v = Some(match re_v {
            None => (pv, pv),
            Some((lo, hi)) => (lo.min(pv), hi.max(pv)),
        });
    }
    let (Some(re_u), Some(re_v)) = (re_u, re_v) else {
        return true;
    };
    // Disjoint iff definitely separated on either axis.
    let separated = |(alo, ahi): (T, T), (blo, bhi): (T, T)| {
        matches!(
            decide("shell_footprint_separation", Margin::of(blo - ahi), band),
            Ok(Sign::Positive)
        ) || matches!(
            decide("shell_footprint_separation", Margin::of(alo - bhi), band),
            Ok(Sign::Positive)
        )
    };
    !(separated(grown(a.box_u), grown(re_u)) || separated(grown(a.box_v), grown(re_v)))
}

/// Every point on `face`'s boundary loops.
fn face_boundary_points<T: Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<geom_core::Point3<T>>, ShellError<T>> {
    let corrupt = |key| ShellError::Corrupt { key };
    let data = body
        .get_face(face)
        .ok_or_else(|| corrupt(EntityId::Face(face)))?;
    let mut out = Vec::new();
    for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let loop_data = body
            .get_loop(lk)
            .ok_or_else(|| corrupt(EntityId::Loop(lk)))?;
        let LoopBoundary::Cycle { first } = loop_data.boundary else {
            continue;
        };
        for he in body
            .loop_cycle(first)
            .ok_or_else(|| corrupt(EntityId::HalfEdge(first)))?
        {
            let start = body
                .get_half_edge(he)
                .ok_or_else(|| corrupt(EntityId::HalfEdge(he)))?
                .start;
            let vertex = body
                .get_vertex(start)
                .ok_or_else(|| corrupt(EntityId::Vertex(start)))?;
            out.push(
                *body
                    .get_point(vertex.point)
                    .ok_or_else(|| corrupt(EntityId::Vertex(start)))?,
            );
        }
    }
    Ok(out)
}

/// The body's faces grouped by the surface they wear, in face-arena
/// order — the unit a chart moves in.
fn chart_groups<T: Real>(body: &Body<T>) -> Vec<Vec<FaceKey>> {
    group_by_chart(body, body.faces().map(|(k, _)| k))
        .into_iter()
        .map(|(_, faces)| faces)
        .collect()
}

/// `faces` gathered by the chart each wears, in first-appearance order
/// with each group in the order the faces arrived — the one grouping
/// this verb does, wherever it does it.
fn group_by_chart<T: Real>(
    body: &Body<T>,
    faces: impl Iterator<Item = FaceKey>,
) -> Vec<(crate::geometry::SurfaceKey, Vec<FaceKey>)> {
    let mut out: Vec<(crate::geometry::SurfaceKey, Vec<FaceKey>)> = Vec::new();
    for face in faces {
        let Some(data) = body.get_face(face) else {
            continue;
        };
        match out.iter_mut().find(|(k, _)| *k == data.surface) {
            Some((_, group)) => group.push(face),
            None => out.push((data.surface, vec![face])),
        }
    }
    out
}

/// Every face of `body` wearing `chart`, in face-arena order.
fn faces_wearing<T: Real>(body: &Body<T>, chart: crate::geometry::SurfaceKey) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| f.surface == chart)
        .map(|(k, _)| k)
        .collect()
}

/// The signed offset distance that moves `face` INTO the material: the
/// chart normal points out of the solid on a positively-sensed face and
/// into it on a reversed one, so the caller's thickness magnitude never
/// has to know which is which.
fn inward<T: Real>(body: &Body<T>, face: FaceKey, thickness: T) -> Result<T, ShellError<T>> {
    let sense = body
        .get_face(face)
        .ok_or(ShellError::Corrupt {
            key: EntityId::Face(face),
        })?
        .sense;
    Ok(if sense { -thickness } else { thickness })
}

/// The designation gates: every named face resolves, is named once, and
/// leaves its shell with a nonempty, connected remainder.
fn check_designation<T: Real>(body: &Body<T>, open_faces: &[FaceKey]) -> Result<(), ShellError<T>> {
    for (i, face) in open_faces.iter().enumerate() {
        let Some(data) = body.get_face(*face) else {
            return Err(ShellError::OpenFaceStale { face: *face });
        };
        if open_faces[..i].contains(face) {
            return Err(ShellError::OpenFaceRepeated { face: *face });
        }
        let surface = body.get_surface(data.surface).ok_or(ShellError::Corrupt {
            key: EntityId::Face(*face),
        })?;
        if !matches!(surface, geom::Surface::Plane { .. }) {
            return Err(ShellError::OpenFaceRingUnsupported {
                face: *face,
                kind: geom_brep::SurfaceKind::of(surface),
            });
        }
    }
    if open_faces.is_empty() {
        return Ok(());
    }
    // A chart is lifted as ONE by the rim stage (the group door's own
    // contract), so a partially designated chart has no coherent lift.
    for &face in open_faces {
        let key = body
            .get_face(face)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(face),
            })?
            .surface;
        if let Some((other, _)) = body
            .faces()
            .find(|(k, f)| !open_faces.contains(k) && f.surface == key)
        {
            return Err(ShellError::OpenFaceChartPartial { face, other });
        }
    }
    for (shell, data) in body.shells() {
        let remaining: Vec<FaceKey> = data
            .faces
            .iter()
            .copied()
            .filter(|f| !open_faces.contains(f))
            .collect();
        if remaining.is_empty() {
            return Err(ShellError::OpenFacesExhaustShell { shell });
        }
        let components = count_components(body, &remaining)?;
        if components != 1 {
            return Err(ShellError::OpenFacesDisconnect { shell, components });
        }
    }
    Ok(())
}

/// How many edge-adjacency components `faces` falls into — the
/// validator's own pass-11 relation, restricted to a subset.
fn count_components<T: Real>(body: &Body<T>, faces: &[FaceKey]) -> Result<usize, ShellError<T>> {
    let mut seen: Vec<FaceKey> = Vec::new();
    let mut components = 0usize;
    for seed in faces {
        if seen.contains(seed) {
            continue;
        }
        components += 1;
        let mut work = vec![*seed];
        seen.push(*seed);
        while let Some(face) = work.pop() {
            for neighbour in face_neighbours(body, face)? {
                if faces.contains(&neighbour) && !seen.contains(&neighbour) {
                    seen.push(neighbour);
                    work.push(neighbour);
                }
            }
        }
    }
    Ok(components)
}

/// The faces `face` shares an edge with.
fn face_neighbours<T: Real>(body: &Body<T>, face: FaceKey) -> Result<Vec<FaceKey>, ShellError<T>> {
    let corrupt = |key| ShellError::Corrupt { key };
    let data = body
        .get_face(face)
        .ok_or_else(|| corrupt(EntityId::Face(face)))?;
    let mut out = Vec::new();
    for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let loop_data = body
            .get_loop(lk)
            .ok_or_else(|| corrupt(EntityId::Loop(lk)))?;
        let LoopBoundary::Cycle { first } = loop_data.boundary else {
            continue;
        };
        let cycle = body
            .loop_cycle(first)
            .ok_or_else(|| corrupt(EntityId::HalfEdge(first)))?;
        for he in cycle {
            let mate = body
                .mate(he)
                .ok_or_else(|| corrupt(EntityId::HalfEdge(he)))?;
            let mate_data = body
                .get_half_edge(mate)
                .ok_or_else(|| corrupt(EntityId::HalfEdge(mate)))?;
            let parent = body
                .get_loop(mate_data.parent_loop)
                .ok_or_else(|| corrupt(EntityId::Loop(mate_data.parent_loop)))?
                .face;
            if parent != face && !out.contains(&parent) {
                out.push(parent);
            }
        }
    }
    Ok(out)
}
