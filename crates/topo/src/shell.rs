//! `shell` — hollowing, sealed and opened.
//!
//! **Shelling** turns a solid into a thin-walled one: the boundary is
//! offset inward by the wall thickness, and the hollow is either kept
//! closed (a cavity) or opened by designating faces whose material is
//! removed, leaving annular rims where the wall's thickness shows
//! (`crates/geom-brep/README.md`'s vocabulary, unchanged).
//!
//! # The operand is at rest
//!
//! Both doors take a finished body ([`AtRestBody`]), the Boolean's and
//! the split's operand type: tier 3 passed on these bits. The verb
//! answers for the material the operand bounds, so what tier 3 decides
//! is what the construction reads: an inside-out solid (check 7,
//! [`ValidationError::NegativeVolume`]) or an inside-out shell (check
//! 10) would be hollowed as its complement, and a stale or missing
//! pcurve row would vanish under the closing mint below. Neither
//! reaches the verb, because neither is an [`AtRestBody`].
//!
//! The door reads no second gate where the Boolean and the split read
//! [`AtRestBody::gate_unverdicted`]: that read is for an operand
//! carrying no verdict, which only a dual's
//! [`crate::AtRestPolicy::gate_at_rest_kept`] keeps, and this door is
//! bounded on the certification right a dual does not hold.
//!
//! [`crate::replace_faces_offset`] keeps its `&mut Body`: it is also
//! this verb's chart-by-chart step over a clone that is mid-construction
//! between charts, so it cannot be the place a verdict is read.
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
//!    SOLID** — a vessel beside a box tilted off its axis is neither
//!    all-planar nor axial while each of the two is one of those, so a
//!    whole-body reading would refuse the vessel's corners it solves
//!    alone: an ALL-PLANAR solid goes through
//!    [`crate::offset_planes_together`], which moves every chart at
//!    once and solves each corner against all the moved planes meeting
//!    it; anything with a curved face goes chart by chart through
//!    [`crate::replace_faces_offset`], whose corners are transported
//!    once per chart and whose OBLIQUE ones therefore refuse
//!    (`ReanchorOffCarrier`) rather than build. The split is #1081's:
//!    the planar half of that class is repaired and the curved half is
//!    not;
//! 2. that body inserted through the shared void-insertion door
//!    ([`crate::boolean::voids::insert_hollow_voids`]) with carried evidence —
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
//! PLANAR faces — to within a drift of `t` across the pair, or a
//! cosine antiparallel to the band — have offsets whose projected boxes
//! meet across less than `2t`; **what it still cannot see** is a curved wall (below), a planar pair tilted
//! further than that (two offsets meeting at an angle,
//! `work/shell/shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel.md`),
//! and the corner solves' own refusals, which are the offset doors'.
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
//! row): `Body::revert` carries the cavity's rows key for key (the
//! plane faces' rows re-stated with their frames, the curved faces'
//! untouched), the graft copies them verbatim onto fresh keys, and
//! that row's contract is that the producer's final mint re-derives
//! every row of the merged body. This verb is a
//! producer and runs [`crate::pcurves::mint_pcurves`] once, on the
//! assembled body, before `validate_geometric` — the verb's own
//! whole-body pass, and it stays whole-body: it is what discharges
//! `insert_voids`'s `Transfers` row over the WHOLE merged body, which
//! no per-solid pass covers. One pass suffices: nothing between the
//! door and the validate reads a stored row, the simultaneous lift
//! doors mint the rows of their own scope (the solid they were handed)
//! and touch no other, and every other step is `Neither` for rows.
//! The pass CLEARS the map first, so it reads none of the operand's
//! rows; that they were sound is the operand type's promise (above),
//! and a stale-row operand refuses where it is gated, not here
//! (`shell9_r2_probes`). A face whose carrier class the pass cannot
//! derive stops carrying rows rather than refusing
//! (`UnsupportedCarrier`; not known to be reachable through this
//! verb). The refusal is [`ShellError::Pcurve`], a kernel finding by
//! construction.
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
//! [`crate::boolean::voids::insert_hollow_voids`]'s graft map — the only
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
//! operand key on only some — an extruded holed slab's mouth or a
//! revolve's annular cap carries its ring already, while a cap whose
//! hole is joined to its outer cycle by a slit has none until `kemr`
//! mints one during the chart reduction.
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
//!    ([`canonicalize_chart`]) — except the surviving side of a
//!    periodic chart, which keeps its faces (below, "A curved
//!    designated face");
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
//! # A curved designated face
//!
//! No surface kind is refused: the rim takes its chart's own form.
//! The lift distance is the inverse of the offset mint
//! ([`geom_brep::offset_distance`]), read along the doors' convention,
//! so a cylinder, sphere or torus comes back by its change of radius
//! and a cone by its apex slide. Then, on a PERIODIC chart:
//!
//! - **A chart that wraps its period** — its surviving side carries an
//!   interior edge, a seam between two branches or one a face walks
//!   twice — becomes a SEAMED BAND, the shape the full revolve mints.
//!   The surviving side is not reduced: the glue makes the dying side's
//!   boundary a ring of its first face, and each seam's pole end is then
//!   re-anchored on the ring corner that stands for its boundary end
//!   ([`seamed_band`]), so every face and seam of the operand's chart
//!   survives under its key and no face carries a ring. Built through a
//!   POLE today (a cap, two branches or one); a band that wraps between
//!   two boundaries refuses [`ShellError::OpenFaceRimNotExpressible`].
//! - **A window that does not wrap** is a ring, exactly as on a plane.
//!   What its readers cannot yet read is theirs and refuses where they
//!   read it: a ringed sphere or cone face at tier 3's check 7
//!   ([`ValidationError::VolumeUncomputable`]), through
//!   [`ShellError::NotValid`] as the boolean's does, and a ringed
//!   cylinder wall — which props reads — at the mesh.
//!
//! Every nesting question the rim stage asks is read in the chart
//! ([`encloses`]); check 9's contact arm reads only planes, so a curved
//! ring's contact with its outer loop is not decided here.
//!
//! # Why step 3 exists, and what the arm is expressible over
//!
//! The glue's only output shape is "one region per face, an outer loop
//! plus rings", so a designated face is safe exactly when its cavity
//! counterpart's boundary can become an INTERIOR-DISJOINT ring of it.
//! A chart need not arrive that way: it may be several faces meeting
//! along edges only they share — two half-discs meeting at an apex —
//! or one face SLIT, its loop walking one edge both ways to join a hole
//! to its outer cycle. A full revolve mints neither (each plane wall is
//! one face, a disc or an annulus carrying its hole as a ring), but the
//! glue's hypothesis does not name where the operand came from. Gluing
//! onto either puts the counterpart's boundary ON the designated face's
//! own — sharing the apex, running back along the slit — and the result
//! is a body every structural tier blesses and no triangulator accepts.
//!
//! Both are facts about how the operand was built rather than about
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
//! **Check 9 has a second half now, and it states the other half of
//! the same sentence**: a ring must lie strictly INSIDE its face's
//! outer loop, not merely stand clear of it
//! ([`ValidationError::RingOutsideOuter`], with
//! [`ValidationError::RingNestingUndecided`] for the pair it cannot
//! certify). That is the statement an inverted host/guest pick at the
//! rim glue below falsifies, and it reaches every planar face whose
//! outer loop carries lines, circle arcs or ellipse arcs; check 9's own
//! banner enumerates what it leaves out.
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

use geom_core::k_stats::{decide, gate_measured};
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Real, Sign, Tol};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::boolean::voids::{VoidContainment, VoidEvidence, VoidInsertError, insert_hollow_voids};
use crate::chart_groups::ChartGroups;
use crate::entity::{
    EdgeKey, EntityId, FaceKey, HalfEdgeKey as HeKey, LoopBoundary, LoopKey, ShellKey, SolidKey,
    VertexKey,
};
use crate::euler::EulerOpError;
use crate::face_normal::plane_outward_normal;
use crate::live::{BoundaryMember, NAMES_ONLY_LIVE, linked, proven};
use crate::pcurves::{PcurveMintError, mint_pcurves};
use crate::props::ShellRole;
use crate::replace_face::ReplaceFaceError;
use crate::validate::{AtRestBody, ValidationError, validate_geometric};

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
    /// The operand could not be sorted into pieces ([`crate::pieces`])
    /// before it is thickened: the verb takes a body, and a solid
    /// holding several pieces is sorted first, so the piece a shell
    /// belongs to has to be readable.
    Pieces {
        /// The sort's typed refusal, verbatim.
        error: crate::pieces::PieceSortError,
    },
    /// One of the operand's solids, once sorted into pieces, has no
    /// outer shell: only cavities, which bound no material. Not a shape
    /// this verb thickens. More than one cannot reach here: the sort
    /// reads roles through [`crate::props::shell_role`], and this verb
    /// classifies through the same lane at the reporting target, which
    /// reads a role only where that walk read the same one
    /// (`props::role_at_target`). So the sort leaves no solid with a
    /// second decided `Outer`, and a shell the sort left undecided
    /// refuses [`Self::Roles`].
    OperandOuterShells {
        /// The solid with no outer shell.
        solid: SolidKey,
    },
    /// The re-partition of an operand void and its dilated twin into a
    /// solid of their own refused. The keys are the shell op's own —
    /// the void is one of the sealed arm's decided void list, its twin
    /// is the graft map's answer for it — so the ownership door's
    /// refusal is an operation refusal, never an argument miss
    /// ([`EulerOpError::from_driver`]).
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
    /// A chart worn by several faces of ONE solid has faces with
    /// DIFFERENT orientation bits, so "inward" is not one direction for
    /// it. A solid moves its wearers of a chart as one; a mixed-sense
    /// group has no single inward to move them by. Wearers on different
    /// solids are different groups, so two solids resting on one chart
    /// with opposed senses shell independently.
    ChartSenseMixed {
        /// A face of the chart.
        face: FaceKey,
        /// A face of the same chart and solid with the opposite sense.
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
    /// A designated face shares its chart with faces of its own solid
    /// that were NOT designated. The rim surgery lifts a solid's wearers
    /// of a chart as one — the group door's own contract — so a
    /// partially designated group has no coherent lift.
    OpenFaceChartPartial {
        /// The designated face.
        face: FaceKey,
        /// A face of the same chart and solid that was not designated.
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
            Self::Band { error } => write!(f, "{error}"),
            Self::Thickness { thickness } => write!(
                f,
                "the wall thickness ({thickness:?} m) is not certifiably positive. Recourse: \
                 supply a positive thickness"
            ),
            Self::NoSolid => write!(
                f,
                "the body carries no solid, so there is no material to thicken"
            ),
            Self::Roles { error } => write!(
                f,
                "the body's shells could not be sorted into one outer boundary and its \
                 voids: {error}"
            ),
            Self::Pieces { error } => write!(
                f,
                "the body could not be sorted into solids before it is thickened: {error}"
            ),
            Self::OperandOuterShells { .. } => write!(
                f,
                "a solid of the body has no outer shell, only cavities, which bound no \
                 material to thicken"
            ),
            Self::Partition { shell, error } => write!(
                f,
                "the thin solid around void {shell:?} could not be partitioned out: {error}"
            ),
            Self::WallClearance { gap, needed, .. } => write!(
                f,
                "two faces face each other across {gap:?} m of material and the two walls \
                 need {needed:?} m, so the cavity would self-intersect. Recourse: use a \
                 thinner wall"
            ),
            Self::ChartSenseMixed { .. } => write!(
                f,
                "two faces share a chart but not an orientation, so \"inward\" is not one \
                 direction for it"
            ),
            Self::OpenFaceChartPartial { .. } => write!(
                f,
                "a face was designated open but another face on its chart was not, and a \
                 chart opens as one. Recourse: designate every face of that chart, or none"
            ),
            Self::OpenFaceRimNotExpressible { what, .. } => write!(
                f,
                "the rim of an open face is not a shape the shell op can build: {what}"
            ),
            Self::Lift { error, .. } => write!(
                f,
                "lifting the rim back onto a designated open face refused: {error}"
            ),
            Self::Face { error, .. } => {
                write!(f, "offsetting a face inward refused: {error}")
            }
            Self::OpenFaceStale { .. } => {
                write!(f, "a designated open face does not resolve in the body")
            }
            Self::OpenFaceRepeated { .. } => write!(
                f,
                "a face was designated open twice. Recourse: designate each face once"
            ),
            Self::OpenFacesExhaustShell { .. } => write!(
                f,
                "every face of a shell was designated open, leaving nothing to carry a wall. \
                 Recourse: leave at least one face closed"
            ),
            Self::OpenFacesDisconnect { components, .. } => write!(
                f,
                "removing the designated faces splits a shell's boundary into {components} \
                 pieces, which would not make one thin solid. Recourse: designate faces whose \
                 removal leaves one connected boundary"
            ),
            Self::Insert { error } => write!(f, "inserting the cavity refused: {error}"),
            Self::Rim { face, error } => {
                write!(f, "the rim surgery on face {face:?} refused: {error}")
            }
            Self::Escalated { source } => {
                write!(
                    f,
                    "a classification of the shell is too close to call: {source}"
                )
            }

            Self::Pcurve { source } => write!(
                f,
                "the finished thin solid could not be parametrized (kernel finding): {source}"
            ),
            Self::NotValid { errors } => write!(
                f,
                "the assembled thin solid is not valid ({} errors) and is discarded",
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
    /// **`sources[0]`** — a plane chart's faces merge onto the first one
    /// designated, so a caller that wants a particular face to carry
    /// the rim's identity names it first; a chart that wraps its period
    /// keeps every face as a branch of the seamed band, and `sources[0]`
    /// is the first of them. With `side` `Void` it is the
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
    /// own ring loop, while on a SLIT cap — its hole joined to its outer
    /// cycle by a slit — the designated face carries no ring at all in
    /// the operand and this loop is minted by `kemr` during the chart
    /// reduction. Reading it
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
/// record names. Scaffolding a rim's surgery mints and kills within
/// itself (a seamed band's struts and its pole's copy) was never in a
/// row and is not listed.
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
/// The operand is a finished body (module docs, "The operand is at
/// rest"): an inside-out or stale-row body refuses where it is gated,
/// at [`AtRestBody::validate`], and never reaches the verb.
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
pub fn shell<T: Decide + geom_core::CertifiedBounds + crate::props::AtRestPolicy>(
    body: &AtRestBody<T>,
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
pub fn shell_open<T: Decide + geom_core::CertifiedBounds + crate::props::AtRestPolicy>(
    body: &AtRestBody<T>,
    thickness: T,
    open_faces: &[FaceKey],
    tol: Tol,
) -> Result<Shelled<T>, ShellError<T>> {
    let body: &Body<T> = body;
    let mut naming = ShellNaming::default();
    // `shell` reaches this door, so both verbs derive here, once.
    let band = Band::linear(tol).map_err(|error| ShellError::Band { error })?;

    // ---- Decide: the thickness. ----
    match decide("shell_thickness", Margin::of(thickness), band) {
        Ok(Sign::Positive) => {}
        _ => return Err(ShellError::Thickness { thickness }),
    }

    // ---- Decide: one piece of material per solid. ----
    //
    // A finished operand is already one piece per solid (tier 3's check
    // 10 refuses two `Outer` shells under one solid), so no operand that
    // can reach this door is changed by the sort below; whether it and
    // `ShellError::Pieces` are reachable at all is
    // `work/shell/shell-operand-shape-arms-behind-the-at-rest-gate.md`.
    // It runs on a clone, so every key the caller holds still names the
    // same face, edge and vertex; a body whose every solid has one shell
    // is not read.
    let sorted;
    let body = if body.solids().any(|(_, s)| s.shells.len() > 1) {
        let mut clone = body.clone();
        crate::pieces::sort_into_pieces(&mut clone, band, tol, T::quad_lane(), None)
            .map_err(|error| ShellError::Pieces { error })?;
        sorted = clone;
        &sorted
    } else {
        body
    };

    // ---- Decide: there is a solid to thicken. ----
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    if solids.is_empty() {
        return Err(ShellError::NoSolid);
    }
    // The operand's own solid partition, read once: which solid every
    // face, edge and vertex belongs to. Every per-solid step below is
    // scoped through it, and the simultaneous doors take the same
    // reading of the same body.
    let partition = crate::offset_together::Scope::whole(body);

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
        let shells = &proven(&body.solids, solid, EntityId::Solid).shells;
        // A single-shell solid's shell is its boundary by arity, so
        // that solid reads nothing and its verdict log is untouched.
        if shells.len() == 1 {
            continue;
        }
        let roles = crate::props::classify_shells_through(body, shells, tol, T::quad_lane())
            .map_err(|error| ShellError::Roles { error })?;
        match roles.iter().filter(|c| c.role == ShellRole::Outer).count() {
            0 => return Err(ShellError::OperandOuterShells { solid }),
            1 => {}
            outer => unreachable!(
                "{outer} decided outer shells under one solid after the sort, whose \
                 sign walk reads every role the classification reads"
            ),
        }
        voids.extend(
            roles
                .iter()
                .filter(|c| c.role == ShellRole::Void)
                .map(|c| c.shell),
        );
    }

    // ---- Decide: each solid's charts, each with ONE orientation. ----
    //
    // A chart is body-wide — faces of several solids may wear one key —
    // and the door that moves a chart is its solid's, so what moves as
    // one is a solid's OWN wearers of it. Two solids resting on one
    // chart with opposed senses are two groups, each moved inward by
    // its own solid.
    let mut scope = partition.clone();
    let mut solid_charts: Vec<(SolidKey, ChartGroups)> = Vec::with_capacity(solids.len());
    for &solid in &solids {
        scope.re_scope(body, &[solid]);
        let charts = ChartGroups::within(body, scope.faces_in_scope()).unwrap_or_else(|face| {
            unreachable!("{face:?}, walked out of its solid's shells, resolved in that walk")
        });
        for (_, group) in charts.iter() {
            let sense = |f: FaceKey| proven(&body.faces, f, EntityId::Face).sense;
            let first = sense(group[0]);
            for &member in &group[1..] {
                if sense(member) != first {
                    return Err(ShellError::ChartSenseMixed {
                        face: group[0],
                        other: member,
                    });
                }
            }
        }
        solid_charts.push((solid, charts));
    }

    // ---- Decide: the walls are thick enough to hold two offsets. ----
    wall_clearance(body, &partition, thickness, band)?;

    // ---- Decide: the designation. ----
    check_designation(body, &solid_charts, open_faces)?;

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
    // (the face-replacement door's own group form says why). The groups
    // are each solid's own, read above in face-arena order, so the walk
    // is deterministic.
    // The cavity is built under one surgery scope (`crate::surgery`):
    // the offset doors it runs each preserve tier 1, and what certifies
    // the cavity is the transplant's own postcondition in
    // `insert_voids` plus this door's closing tier-3 validation. The
    // guard owns the borrow, so a refusal on the way closes the scope
    // by dropping it.
    let mut cavity_body = body.clone();
    let mut cavity = cavity_body.begin_surgery();
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
    // **The door is ONE decision PER SOLID.** A body with a box tilted
    // off a vessel's axis beside it is neither all-planar nor axial, and a
    // whole-body reading would put both on the per-chart door — which
    // refuses the vessel's corners it solves alone. The ladder is
    // unchanged; what it reads is the solid's own faces. The cavity
    // and the rim lift read the same ladder over the same solid, and
    // the lift reads it on the body the cavity's door BUILT — so the
    // two answer alike because the axial gate's roster is closed under
    // that door's output: an offset keeps a coaxial wall coaxial, a
    // plane normal to the axis normal to it, and a plane parallel to
    // the axis parallel to it at any stand-off (`offset_axial::classify`),
    // and the planar ladder's all-planes answer is closed the same way.
    // A solid is therefore on the same door on the way in and on the
    // way back out.
    // The operand's partition serves every solid: `cavity` is a clone,
    // so it carries the same keys, and re-aiming the scope at one solid
    // is a `Vec` swap rather than another walk over the whole body.
    //
    // **What that sharing buys is one walk here, not one walk per
    // call.** Each simultaneous door the loop reaches builds its own
    // one-solid scope from its move set (`scope_of_moves`), so the
    // solids ARE walked again. What is saved is this verb's own
    // reading, which is a whole-body walk and would otherwise be one
    // per solid.
    for (solid, charts) in &solid_charts {
        let solid = *solid;
        scope.re_scope(body, &[solid]);
        let mine: Vec<&[FaceKey]> = charts.iter().map(|(_, group)| group).collect();
        let Some(&fallback) = mine.first().and_then(|g| g.first()) else {
            unreachable!(
                "{solid:?} has no face: on a tier-1-valid body a solid has a shell and a \
                 shell a face"
            )
        };
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
                        faces: group.to_vec(),
                        distance: inward(&cavity, group[0], thickness),
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
                    let d = inward(&cavity, face, thickness);
                    crate::replace_faces_offset(&mut cavity, group, d, tol).map_err(|error| {
                        ShellError::Face {
                            face,
                            error: Box::new(error),
                        }
                    })?;
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
    cavity.sweep_and_close();
    let cavity = cavity_body;
    // The result is built under one surgery scope too — see the
    // cavity's, and the closing tier-3 validation is this door's own
    // whole-body check.
    let mut out_body = body.clone();
    let mut out = out_body.begin_surgery();
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
    let inserted = insert_hollow_voids(&mut out, &solids, cavity, &evidence)
        .map_err(|error| ShellError::Insert { error })?;

    // ---- The record: the inner twins, read off the graft map at the
    // insertion rather than matched afterwards. The cavity is a clone
    // of the operand, so each of its keys names a source entity, and
    // the door grafts every entity it is handed; a miss of either would
    // leave an entity of the result unnameable and is a kernel bug.
    for face in cavity_faces {
        proven(&body.faces, face, EntityId::Face);
        let twin = inserted
            .face(face)
            .unwrap_or_else(|| ungrafted(EntityId::Face(face)));
        naming.inner.push((twin, face));
    }
    for edge in cavity_edges {
        proven(&body.edges, edge, EntityId::Edge);
        let twin = inserted
            .edge(edge)
            .unwrap_or_else(|| ungrafted(EntityId::Edge(edge)));
        naming.inner_edges.push((twin, edge));
    }
    for vertex in cavity_vertices {
        proven(&body.vertices, vertex, EntityId::Vertex);
        let twin = inserted
            .vertex(vertex)
            .unwrap_or_else(|| ungrafted(EntityId::Vertex(vertex)));
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
            let twin = inserted
                .shell(shell)
                .unwrap_or_else(|| ungrafted(EntityId::Shell(shell)));
            out.move_shells_to_new_solid(&[twin, shell])
                .map_err(|error| ShellError::Partition {
                    shell,
                    error: error.from_driver(),
                })?
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
    // designation asks for is one region of the mouth chart, and how
    // many faces the operand spent on that region is a fact about the
    // operand's construction rather than about
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
    // The RESULT's own partition, read once here: the thin solids have
    // just been minted, so this is the first moment it exists, and it
    // names the solid each designation's surgery happens in.
    let result_partition = crate::offset_together::Scope::whole(&out);
    // The designations grouped by chart within the solid each lies in on
    // the result, in the order their first face was designated.
    // `check_designation` refused a stale designation on the operand,
    // and the result is a clone of it that the rim stage has not yet
    // touched, so every designated face resolves here.
    let mut by_solid: Vec<(SolidKey, Vec<FaceKey>)> = Vec::new();
    for &designated in open_faces {
        let solid = result_partition
            .solid_of(designated)
            .unwrap_or_else(|| undesignated(designated));
        match by_solid.iter_mut().find(|(s, _)| *s == solid) {
            Some((_, faces)) => faces.push(designated),
            None => by_solid.push((solid, vec![designated])),
        }
    }
    let mut rims: Vec<Vec<FaceKey>> = Vec::new();
    for (_, faces) in by_solid {
        let charts = ChartGroups::within(&out, faces).unwrap_or_else(|face| undesignated(face));
        rims.extend(charts.iter().map(|(_, group)| group.to_vec()));
    }
    rims.sort_by_key(|group| open_faces.iter().position(|f| *f == group[0]));
    for group in rims {
        let designated = group[0];
        let sources: Vec<FaceKey> = group
            .iter()
            .map(|&f| {
                inserted
                    .face(f)
                    .unwrap_or_else(|| ungrafted(EntityId::Face(f)))
            })
            .collect();
        // Which shell the designation is on, read off the operand and
        // the roles decided in the sealed arm: this is what assigns the
        // glue's roles below, and what the record reports as `side`.
        let designated_shell = proven(&body.faces, designated, EntityId::Face).shell;
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
            .unwrap_or_else(|| undesignated(designated));

        // Lift the cavity's counterpart chart back onto the designated
        // face's own surface. The distance is read from the two
        // SURFACES (`lift_to`) rather than negated from the way in: the
        // graft's reversal negates a stored plane normal (`revert`'s own
        // contract), so "the way back" is not the arithmetic negation of
        // "the way in", and deriving it from geometry is sign-safe.
        let counterpart_chart = proven(&out.faces, sources[0], EntityId::Face).surface;
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
        // Walked on the body as it stands at this rim, after the
        // earlier rims' surgery: the door, its move set and its
        // counterpart group all read this one scope.
        let lift_scope = crate::offset_together::Scope::of_solids(&out, &[lift_solid]);
        let lift_charts =
            ChartGroups::within(&out, lift_scope.faces_in_scope()).unwrap_or_else(|face| {
                unreachable!("{face:?}, walked out of its solid's shells, resolved in that walk")
            });
        let back = lift_to(&out, lift_charts.of(counterpart_chart), designated, band).map_err(
            |error| ShellError::Lift {
                face: designated,
                error: Box::new(error),
            },
        )?;
        let lift_door = offset_door(&out, &lift_scope, band).map_err(|error| ShellError::Lift {
            face: designated,
            error: Box::new(error),
        })?;
        let outcome = match lift_door {
            OffsetDoor::ChartsTogether => {
                let moves: Vec<crate::offset_together::ChartMove<T>> = lift_charts
                    .iter()
                    .map(|(key, group)| crate::offset_together::ChartMove {
                        faces: group.to_vec(),
                        distance: if key == counterpart_chart {
                            back
                        } else {
                            T::zero()
                        },
                    })
                    .collect();
                crate::offset_charts_together(&mut out, &moves, band, tol)
            }
            OffsetDoor::PlanesTogether | OffsetDoor::PerChart => {
                crate::replace_faces_offset(&mut out, lift_charts.of(counterpart_chart), back, tol)
            }
        };
        outcome.map_err(|error| ShellError::Lift {
            face: designated,
            error: Box::new(error),
        })?;
        // The rim stage runs row-free: the closing mint clears the map
        // and re-derives every row of the assembled body, so a row the
        // lift wrote, or one a surgery op would mint here, is never
        // read. Clearing it keeps every op below on its keys-only door.
        out.pcurves.clear();
        out.joints.clear();
        // One face per side, loops disjoint. Both reductions retire
        // keys through the Euler doors, and each door's own result is
        // what fills `dead` — recorded at the call, never inferred
        // afterwards from what stopped resolving.
        // The designated chart reduced to one face — the mouth — and
        // its counterpart chart reduced to one. On a PERIODIC chart the
        // side that survives keeps every face it has: its chart
        // branches and its seams are D1's, and the rim takes them over.
        let periodic = matches!(
            out.face_surface_linked(designated, proven(&out.faces, designated, EntityId::Face)),
            geom::Surface::Cylinder { .. }
                | geom::Surface::Cone { .. }
                | geom::Surface::Sphere { .. }
                | geom::Surface::Torus { .. }
        );
        let keeps = |which: RimShell| periodic && side == which;
        let mouth = if keeps(RimShell::Outer) {
            None
        } else {
            Some(canonicalize_chart(
                &mut out,
                &group,
                band,
                &mut naming.dead,
            )?)
        };
        let counterpart = if keeps(RimShell::Void) {
            None
        } else {
            Some(canonicalize_chart(
                &mut out,
                &sources,
                band,
                &mut naming.dead,
            )?)
        };
        let reduced = |face: Option<FaceKey>, chart: &[FaceKey]| {
            face.map_or_else(|| chart.to_vec(), |face| vec![face])
        };
        let (host_faces, guest) = match side {
            RimShell::Void => (reduced(counterpart, &sources), mouth),
            RimShell::Outer => (reduced(mouth, &group), counterpart),
        };
        let Some(guest) = guest else {
            unreachable!("the side that dies in the glue is always reduced to one face")
        };
        let rows = match side {
            RimShell::Void => RingSource::Operand(body),
            RimShell::Outer => RingSource::Twins(&twins),
        };
        // **A chart whose surviving side carries interior edges wraps
        // its period**, and its rim is a SEAMED BAND rather than a ring:
        // a ring on a periodic chart is a face whose domain closes with
        // no wrap edge to close it across.
        let seams = interior_edges(&out, &host_faces);
        if !seams.is_empty() {
            let rim = seamed_band(
                &mut out,
                &host_faces,
                guest,
                &seams,
                &twins,
                &rows,
                designated,
                band,
                tol,
                &mut naming.dead,
            )?;
            naming.rims.push(RimNaming {
                sources: group,
                side,
                rim: rim.face,
                ring: rim.ring,
                ring_edges: rim.ring_edges,
                ring_vertices: rim.ring_vertices,
                holes: Vec::new(),
            });
            continue;
        }
        let [host] = host_faces[..] else {
            return Err(ShellError::OpenFaceRimNotExpressible {
                face: designated,
                what: "the designated chart's faces neither meet along a seam nor merge into one \
                       region",
            });
        };

        // **The glue's roles.** `kfmrh(host, guest)` kills `guest` and
        // makes its outer loop a ring of `host`, so `host` must be the
        // face whose boundary ENCLOSES the other's. On the outer shell
        // that is the mouth: its counterpart was offset inward and
        // lifted back, so the counterpart's boundary sits strictly
        // inside. On a void the counterpart was DILATED and lifted
        // back, so it is the counterpart's boundary that encloses — the
        // counterpart survives as the rim, facing the gap, and the
        // mouth dies. The role is read off the sealed arm's decided
        // shell list and nothing re-derives it HERE: `ring_outer_contact`
        // below decides CONTACT between the two loops, not which
        // encloses which.
        //
        // What an inverted assignment meets first is not a validator
        // but the NAMING RECORD: `ring_rows` walks the glued ring's
        // entities for the source each one came from, and on an
        // inverted pick the ring is the wrong boundary, so no entity
        // has one and the verb panics naming it before its closing
        // `validate_geometric` is reached at all. The
        // statement that an inverted glue is WRONG, rather than merely
        // unexplainable, is tier 3's check 9: its nesting half says a
        // ring lies strictly inside its face's outer loop and refuses
        // the inverted body by name, on the shapes that half reaches
        // — every planar face whose outer loop carries lines, circle
        // arcs or ellipse arcs, which covers the annular rim of every
        // shelled vessel of revolution and a rim that mixes arcs with
        // lines (check 9's banner enumerates the rest). Nothing in
        // this verb relies on that arm; what it buys is the class
        // being loud wherever else it is minted. On a rim outside its
        // reach — a non-planar rim, or an outer loop carrying a spiric
        // or spline edge — the assignment here is pinned only
        // structurally: the void-ceiling row asserts the designated
        // void face DIES, and the pairing row reads each thin solid's
        // twin through the record.
        let (host_surface, host_sense) = {
            let data = proven(&out.faces, host, EntityId::Face);
            (data.surface, data.sense)
        };
        // Read AFTER the lift and the reduction: on an outer-shell
        // designation `FaceSurface::New` minted a fresh key for the
        // lifted chart, and that key — not the one the graft brought in
        // — is what the guest's descriptions now name.
        let guest_surface = proven(&out.faces, guest, EntityId::Face).surface;

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
            // The promoted face faces the HOST's way on the host's
            // surface: a ring of the guest is wound opposite to the
            // guest's outer loop, i.e. the way an outer loop of a
            // host-facing face must be. It is minted on the guest's
            // chart, which its ring's descriptions name, then moved
            // onto the host's with them: the lift re-charted the
            // counterpart onto a surface of its own, so the host's key
            // is never the guest's chart, and the move writes the
            // host's bit as stated. The ring lies on the host's surface
            // too — the lift put the two charts on top of each other —
            // so each re-description is a key swap with the carrier
            // untouched, certified against the geometry.
            // Tier 3's check 6 reads `sense` against the stored loop
            // windings on every planar face, and check 7 reads the
            // volume the same windings integrate, so a flip either way
            // reds at the verb's own closing `validate_geometric`.
            let rim_error = |error: EulerOpError| ShellError::Rim {
                face: designated,
                error: error.from_driver(),
            };
            let made = out
                .mfkrh(guest_ring, crate::euler::FaceSurface::Inherit)
                .map_err(rim_error)?;
            let specs = loop_rekeyed(&out, guest_ring, guest_surface, host_surface);
            out.set_face_surfaces_describing(
                vec![crate::attach::Rechart::shared(
                    host_surface,
                    made.face,
                    host_sense,
                )],
                &specs,
                tol,
            )
            .map_err(rim_error)?;
            promoted.push((made.face, host_ring));
        }

        // The guest's boundary must now be an interior-disjoint ring
        // of the host — the invariant the validator states as check 9.
        // Refused HERE, naming the shape, rather than left to arrive
        // as a generic at-rest report on a body already built.
        let guest_outer = proven(&out.faces, guest, EntityId::Face).outer;
        let host_outer = proven(&out.faces, host, EntityId::Face).outer;
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
            error: error.from_driver(),
        })?;
        naming.dead.faces.push(fused.killed_face);
        naming.dead.surfaces.extend(fused.killed_surface);
        naming.dead.shells.extend(fused.killed_shell);
        // The ring's edges still NAME the surface that just died. They
        // lie on the host's surface now — the lift put the two charts
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
        let (ring_edges, ring_vertices) = ring_rows(&out, fused.ring, &rows);

        // The hole rows read the same way, off each promoted face's
        // guest-side boundary — its outer loop, which `mfkrh` made from
        // the guest's ring — so a hole gets the same edge-level anchor
        // the outer rim has.
        let mut holes = Vec::with_capacity(promoted.len());
        for &(face, host_ring) in &promoted {
            let outer = proven(&out.faces, face, EntityId::Face).outer;
            let (ring_edges, ring_vertices) = ring_rows(&out, outer, &rows);
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
                    error: error.from_driver(),
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
    // The rim stage clears the map before each chart's surgery, so on
    // an opened body with a curved face every such face reaches this
    // line row-free: moved ahead of the rim stage, the pass would leave
    // them `Unminted` at the validation below (`shell_curved_mouth`'s
    // cap row and `verbs_shell`'s revolved cups).
    mint_pcurves(&mut out, tol).map_err(|source| ShellError::Pcurve { source })?;

    // ---- One validation. ----
    out.sweep_and_close();
    validate_geometric(&out_body, tol).map_err(|errors| ShellError::NotValid { errors })?;
    Ok(Shelled {
        body: out_body,
        naming,
    })
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
/// own — so a miss is a ring entity the record cannot explain, a
/// kernel bug, and panics naming it rather than dropping it.
#[track_caller]
fn ring_rows<T: Real>(body: &Body<T>, ring: LoopKey, source: &RingSource<'_, T>) -> RingRows {
    let unexplained = |key: EntityId| -> ! {
        unreachable!(
            "{key}, on the rim ring {ring:?}, has no source row: a fused rim's ring is a \
             cavity twin's boundary or the operand's own, and the record holds a row for each"
        )
    };
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
    let LoopBoundary::Cycle { first } = proven(&body.loops, ring, EntityId::Loop).boundary else {
        return (Vec::new(), Vec::new());
    };
    let cycle = body.loop_walk(first).closed("loop", first);
    let mut edges = Vec::with_capacity(cycle.len());
    let mut vertices = Vec::with_capacity(cycle.len());
    for he in cycle {
        let half = proven(&body.half_edges, he, EntityId::HalfEdge);
        let source =
            source_edge(half.edge).unwrap_or_else(|| unexplained(EntityId::Edge(half.edge)));
        edges.push((half.edge, source));
        let source =
            source_vertex(half.start).unwrap_or_else(|| unexplained(EntityId::Vertex(half.start)));
        vertices.push((half.start, source));
    }
    (edges, vertices)
}

/// The panic for a cavity entity the void door's graft map does not
/// carry: the door grafts every entity of the cavity it is handed.
#[track_caller]
fn ungrafted(key: EntityId) -> ! {
    unreachable!(
        "{key}, an entity of the cavity, has no twin in the void door's graft map: the door \
         grafts every entity of the body it is handed"
    )
}

/// The panic for a designated face the result does not hold where the
/// rim stage reads it: `check_designation` resolved every designation
/// on the operand, and the result is a clone of it.
#[track_caller]
fn undesignated(face: FaceKey) -> ! {
    unreachable!(
        "{face:?}, a designation the operand resolved, is not on the result's partition: the \
         result is a clone of the operand, and {NAMES_ONLY_LIVE}"
    )
}

/// **One face per chart, loops disjoint** — the shape the rim glue's
/// only output form needs on both sides of it.
///
/// A chart may arrive carrying edges that are no fact about the region:
/// several faces meeting along edges only they share (two half-discs
/// meeting along a diameter), or one face SLIT, its loop walking one
/// edge in both directions to join a hole to its outer cycle. Both are
/// facts about how the operand was built, and both are exactly
/// what makes a counterpart's boundary land ON the designated face's
/// boundary instead of strictly inside it. This reduces them, through
/// the Euler doors and nothing else:
///
/// 1. the chart's faces merge across the edges only they share
///    (`kef`), leaving the merged loop walking each killed edge's
///    surviving partner twice;
/// 2. a SPUR — such a duplicate whose far vertex the merge left with
///    one edge on it, the apex of two half-discs — dies with that
///    vertex (`kev`);
/// 3. a SLIT — a duplicate still anchored at both ends, joining a hole
///    to the outer cycle — splits the loop in two (`kemr`), the
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
    let Some(&anchor) = faces.first() else {
        unreachable!("a chart group holds the face it was grouped from")
    };
    let not_expressible =
        |what: &'static str| ShellError::OpenFaceRimNotExpressible { face: anchor, what };

    // ---- 1: one face. ----
    let mut alive: Vec<FaceKey> = faces.to_vec();
    while alive.len() > 1 {
        let edges: Vec<crate::entity::EdgeKey> = body.edges().map(|(k, _)| k).collect();
        let mut acted = false;
        for edge in edges {
            let data = proven(&body.edges, edge, EntityId::Edge);
            let (fp, fm) = crate::readback::edge_sides_of(body, edge, data).faces();
            if fp == fm || !alive.contains(&fp) || !alive.contains(&fm) {
                continue;
            }
            // `kef` kills the face of the half-edge it is given, and
            // refuses a dying face that carries rings.
            let ring_free = |body: &Body<T>, f: FaceKey| {
                proven(&body.faces, f, EntityId::Face).rings.is_empty()
            };
            let (dying, he) = if fm != anchor && ring_free(body, fm) {
                (fm, data.he_minus)
            } else if fp != anchor && ring_free(body, fp) {
                (fp, data.he_plus)
            } else {
                continue;
            };
            let killed = body.kef(he).map_err(|error| ShellError::Rim {
                face: anchor,
                error: error.from_driver(),
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
        // Whether `he` ends at a valence-one tip, which `kev` kills; a
        // far vertex whose valence cannot be read refuses.
        let tip = |body: &Body<T>, he: HeKey| valence(body, body.proven_half_edge_end(he)) == 1;
        if tip(body, he1) {
            let killed = body.kev(he1).map_err(|error| ShellError::Rim {
                face: anchor,
                error: error.from_driver(),
            })?;
            dead.edges.push(killed.killed_edge);
            dead.vertices.push(killed.killed_vertex);
            continue;
        }
        if tip(body, he2) {
            let killed = body.kev(he2).map_err(|error| ShellError::Rim {
                face: anchor,
                error: error.from_driver(),
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
        let (side1, side2) = split_cycle(body, r#loop, he1, he2);
        let (p1, p2) = (
            half_edge_points(body, &side1),
            half_edge_points(body, &side2),
        );
        let chart = body
            .face_surface_linked(anchor, proven(&body.faces, anchor, EntityId::Face))
            .clone();
        let ring_first = if encloses(&chart, &p1, &p2, band) {
            true
        } else if encloses(&chart, &p2, &p1, band) {
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
            error: error.from_driver(),
        })?;
        dead.edges.push(made.killed_edge);
        // The role assignment is verified, not assumed: the ring must
        // be the enclosed side.
        let (ring_pts, outer_pts) = {
            let outer = proven(&body.faces, anchor, EntityId::Face).outer;
            (loop_points(body, made.ring), loop_points(body, outer))
        };
        if !encloses(&chart, &ring_pts, &outer_pts, band) {
            return Err(not_expressible(
                "the chart's slit loop split with the enclosing side as the ring",
            ));
        }
    }
    Ok(anchor)
}

/// The edges both of whose sides bound faces of `faces` — the seams a
/// periodic chart's branches meet along, or the one a single face
/// walks twice — in edge-arena order.
fn interior_edges<T: Real>(body: &Body<T>, faces: &[FaceKey]) -> Vec<EdgeKey> {
    body.edges()
        .filter(|&(edge, data)| {
            let (fp, fm) = crate::readback::edge_sides_of(body, edge, data).faces();
            faces.contains(&fp) && faces.contains(&fm)
        })
        .map(|(edge, _)| edge)
        .collect()
}

/// What [`seamed_band`] built: the rim face that carries the record's
/// identity, and the ring the glue made before the seams absorbed it.
struct SeamedRim {
    face: FaceKey,
    ring: LoopKey,
    ring_edges: Vec<(EdgeKey, EdgeKey)>,
    ring_vertices: Vec<(VertexKey, VertexKey)>,
}

/// **The rim of a chart that wraps its period through a pole**, built
/// as a seamed band: the surviving side's own faces, each bounded by
/// its outer boundary, its seams cut short, and the dying side's
/// boundary.
///
/// `host_faces` are the surviving chart's faces, unreduced, and `seams`
/// the edges they meet along: one seam a single face walks twice, or
/// two seams between two branches. Both run from the host's boundary
/// to one POLE, a vertex nothing else reaches, and the guest — reduced
/// to one ring-free face — lies over the pole on the same surface.
///
/// The surgery keeps every surviving edge: the glue (`kfmrh`) makes the
/// guest's boundary a ring of the first host face, and each seam's pole
/// end is then RE-ANCHORED on the ring vertex that corresponds to its
/// boundary end, through a strut along the seam's own carrier that
/// `kev` collapses. Two seams cannot both be re-anchored through one
/// pole, so with two the second is first moved off it (`mev`) onto a
/// station of its own, and the strut that would reach its ring vertex
/// splits the first face (`mef`) along the first seam; the piece that
/// holds the pole dies into the second face (`kef`). What is left is
/// each host face bounded by its own boundary, its seams' boundary
/// pieces, and its share of the ring — the convention the full revolve
/// mints — with the pole and every scaffold edge gone. The ring's loop
/// is absorbed into the first face's outer loop and is listed in
/// `dead`; the scaffolding the surgery mints and kills is in no row and
/// is not.
#[allow(clippy::too_many_arguments)]
fn seamed_band<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    host_faces: &[FaceKey],
    guest: FaceKey,
    seams: &[EdgeKey],
    twins: &TwinIndex,
    rows: &RingSource<'_, T>,
    designated: FaceKey,
    band: Band,
    tol: Tol,
    dead: &mut ShellRetired,
) -> Result<SeamedRim, ShellError<T>> {
    let not_expressible = |what: &'static str| ShellError::OpenFaceRimNotExpressible {
        face: designated,
        what,
    };
    let rim_error = |error: EulerOpError| ShellError::Rim {
        face: designated,
        error: error.from_driver(),
    };
    let ends = |body: &Body<T>, edge: EdgeKey| {
        let data = proven(&body.edges, edge, EntityId::Edge);
        let start = proven(&body.half_edges, data.he_plus, EntityId::HalfEdge).start;
        (start, body.proven_half_edge_end(data.he_plus))
    };
    // The pole: the one vertex every seam reaches and nothing else does.
    let pole = {
        let (a, b) = ends(body, seams[0]);
        [a, b].into_iter().find(|&v| {
            valence(body, v) == seams.len()
                && seams.iter().all(|&e| {
                    let (s, t) = ends(body, e);
                    s == v || t == v
                })
        })
    };
    let (Some(pole), 1 | 2) = (pole, seams.len()) else {
        return Err(not_expressible(
            "the designated chart wraps its period, but its seams do not meet at one pole",
        ));
    };
    let host = host_faces[0];
    let host_surface = proven(&body.faces, host, EntityId::Face).surface;
    let guest_surface = proven(&body.faces, guest, EntityId::Face).surface;
    // The host face's corner at the pole: `into` arrives along one seam,
    // `out_of` leaves along the other (along the same seam, on a spur).
    let host_cycle = cycle_of(body, proven(&body.faces, host, EntityId::Face).outer);
    let at = |body: &Body<T>, he: HeKey| proven(&body.half_edges, he, EntityId::HalfEdge).start;
    let Some(&out_of) = host_cycle.iter().find(|&&he| at(body, he) == pole) else {
        return Err(not_expressible(
            "the designated chart wraps its period, but its first face does not reach the pole",
        ));
    };
    let into = proven(&body.half_edges, out_of, EntityId::HalfEdge).prev;
    let edge_of = |body: &Body<T>, he: HeKey| proven(&body.half_edges, he, EntityId::HalfEdge).edge;
    let (ea, eb) = (edge_of(body, into), edge_of(body, out_of));
    let (ba, bb) = (at(body, into), body.proven_half_edge_end(out_of));
    // The guest's corner standing for each boundary end: its twin on an
    // outer designation, its source on a void's.
    let corresponding = |body: &Body<T>, v: VertexKey| -> Option<VertexKey> {
        let guest_outer = proven(&body.faces, guest, EntityId::Face).outer;
        cycle_of(body, guest_outer)
            .into_iter()
            .map(|he| proven(&body.half_edges, he, EntityId::HalfEdge).start)
            .find(|&g| twins.vertices.get(g) == Some(&v) || twins.vertices.get(v) == Some(&g))
    };
    let (Some(ga), Some(gb)) = (corresponding(body, ba), corresponding(body, bb)) else {
        return Err(not_expressible(
            "a seam of the designated chart has no corner of the cavity counterpart to end on",
        ));
    };
    let point = |body: &Body<T>, v: VertexKey| crate::chord_join::vertex_point(body, v);
    let (p_pole, p_ga, p_gb) = (point(body, pole), point(body, ga), point(body, gb));
    // Each corner the seams will end on lies on its seam strictly
    // between the boundary and the pole: the band between the boundary
    // and the ring is then the designated face's own, and not a stretch
    // of its surface past the boundary (a junction the cavity's wall
    // meets below the designated face's own).
    for (seam, boundary, corner) in [(ea, ba, p_ga), (eb, bb, p_gb)] {
        let reach = along(
            body,
            seam,
            point(body, boundary),
            p_pole,
            host_surface,
            band,
            designated,
        )?;
        let (s0, s1) = (reach.param_start, reach.param_end);
        let scale = (p_pole - point(body, boundary)).norm() / (s1 - s0);
        let inside = reach
            .carrier
            .param_near(corner, (s0 + s1) / T::from_f64(2.0))
            .is_some_and(|s| {
                [s - s0, s1 - s].into_iter().all(|gap| {
                    matches!(
                        decide("shell_seam_corner_inside", Margin::of(gap * scale), band),
                        Ok(Sign::Positive)
                    )
                })
            });
        if !inside {
            return Err(not_expressible(
                "the cavity counterpart's corner does not lie on the designated chart's seam \
                 between its boundary and the pole, so the rim is not a band of the \
                 designated face",
            ));
        }
    }

    // Two seams: the second leaves the pole for a copy of it (a null
    // edge), so the pole can be collapsed onto one ring vertex and the
    // copy onto the other.
    let second = if seams.len() == 2 {
        let made = body
            .mev_null(
                crate::euler::MevSite::Fan {
                    he1: out_of,
                    he2: {
                        let data = proven(&body.edges, ea, EntityId::Edge);
                        if data.he_plus == into {
                            data.he_minus
                        } else {
                            data.he_plus
                        }
                    },
                },
                crate::null::NewVertexSide::Above,
            )
            .map_err(rim_error)?;
        Some((made.vertex, made.he_plus))
    } else {
        None
    };

    // The glue: the guest's boundary becomes a ring of the host face.
    let fused = body.kfmrh(host, guest).map_err(rim_error)?;
    dead.faces.push(fused.killed_face);
    dead.surfaces.extend(fused.killed_surface);
    dead.shells.extend(fused.killed_shell);
    rename_loop_surface(
        body,
        fused.ring,
        guest_surface,
        host_surface,
        tol,
        designated,
    )?;
    let (ring_edges, ring_vertices) = ring_rows(body, fused.ring, rows);
    let ring_cycle = cycle_of(body, fused.ring);
    let leaving = |v: VertexKey| {
        ring_cycle
            .iter()
            .copied()
            .find(|&he| proven(&body.half_edges, he, EntityId::HalfEdge).start == v)
            .unwrap_or_else(|| unreachable!("{v:?} is a corner of the ring it was read off"))
    };
    let (ra, rb) = (leaving(ga), leaving(gb));

    // The strut that re-anchors the first seam, and the half of it that
    // runs from the ring vertex to the pole, which `kev` collapses.
    let collapse = |body: &Body<T>, plus: HeKey, minus: HeKey, tip: VertexKey| {
        if body.proven_half_edge_end(plus) == tip {
            plus
        } else {
            minus
        }
    };
    let to_pole = match second {
        None => {
            let made = body
                .mekr(
                    crate::euler_ring::MekrSite::Cycles {
                        target: out_of,
                        ring: ra,
                    },
                    along(body, ea, p_pole, p_ga, host_surface, band, designated)?,
                    tol,
                )
                .map_err(rim_error)?;
            dead.loops.push(made.killed_ring);
            collapse(body, made.he_plus, made.he_minus, pole)
        }
        Some((station, off_pole)) => {
            let joined = body
                .mekr(
                    crate::euler_ring::MekrSite::Cycles {
                        target: out_of,
                        ring: rb,
                    },
                    along(body, eb, p_pole, p_gb, host_surface, band, designated)?,
                    tol,
                )
                .map_err(rim_error)?;
            dead.loops.push(joined.killed_ring);
            let split = body
                .mef(
                    crate::euler::MefSite::Chords {
                        he1: off_pole,
                        he2: ra,
                    },
                    along(body, ea, p_pole, p_ga, host_surface, band, designated)?,
                    crate::euler::FaceSurface::Inherit,
                    tol,
                )
                .map_err(rim_error)?;
            // The new face holds the pole and the scaffold between the
            // two seams; it dies into the second host face.
            body.kef(off_pole).map_err(rim_error)?;
            let to_station = collapse(body, joined.he_plus, joined.he_minus, station);
            let to_pole = collapse(body, split.he_plus, split.he_minus, pole);
            let moved = re_anchored(body, eb, station, p_gb, designated)?;
            body.kev_describing(to_station, &[(eb, moved)], tol)
                .map_err(rim_error)?;
            to_pole
        }
    };
    let moved = re_anchored(body, ea, pole, p_ga, designated)?;
    let killed = body
        .kev_describing(to_pole, &[(ea, moved)], tol)
        .map_err(rim_error)?;
    dead.vertices.push(killed.killed_vertex);
    Ok(SeamedRim {
        face: host,
        ring: fused.ring,
        ring_edges,
        ring_vertices,
    })
}

/// A scaffold edge's spec: the stretch of `edge`'s own carrier from
/// `from` to `to`, described in `surface`'s chart and oriented so its
/// parameter runs forward from `from`.
fn along<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    from: geom_core::Point3<T>,
    to: geom_core::Point3<T>,
    surface: crate::geometry::SurfaceKey,
    band: Band,
    designated: FaceKey,
) -> Result<geom_brep::EdgeCurveSpec<T>, ShellError<T>> {
    let data = proven(&body.edges, edge, EntityId::Edge);
    let Some(curve) = body.edge_curve_linked(edge, data).certified() else {
        unreachable!("{edge:?} is a seam of a finished wall, and a finished wall has no null edge")
    };
    let (t0, t1) = curve.params();
    let near = (t0 + t1) / T::from_f64(2.0);
    let stretch = |carrier: &geom::Curve3<T>, near: T| -> Option<(T, T)> {
        let a = carrier.param_near(from, near)?;
        let b = carrier.param_near(to, near)?;
        matches!(
            decide("shell_seam_stretch", Margin::of(b - a), band),
            Ok(Sign::Positive)
        )
        .then_some((a, b))
    };
    let forward = curve.carrier().clone();
    let found = stretch(&forward, near)
        .map(|ab| (forward.clone(), ab))
        .or_else(|| {
            let reversed = forward.reversed()?;
            let mid = reversed.param_near(forward.eval(near), T::zero())?;
            stretch(&reversed, mid).map(|ab| (reversed, ab))
        });
    let Some((carrier, (param_start, param_end))) = found else {
        return Err(ShellError::OpenFaceRimNotExpressible {
            face: designated,
            what: "a seam of the designated chart has no carrier stretch to the counterpart's \
                   corner",
        });
    };
    Ok(geom_brep::EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::chart(surface),
        carrier,
        param_start,
        param_end,
    })
}

/// `edge`'s own description with its end at `moved` carried to `onto`
/// along its carrier: the spec a `kev` that merges `moved` away hands
/// the merged seam.
fn re_anchored<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    moved: VertexKey,
    onto: geom_core::Point3<T>,
    designated: FaceKey,
) -> Result<geom_brep::EdgeCurveSpec<T>, ShellError<T>> {
    let data = proven(&body.edges, edge, EntityId::Edge);
    let Some(curve) = body.edge_curve_linked(edge, data).certified() else {
        unreachable!("{edge:?} is a seam of a finished wall, which has no null edge")
    };
    let (t0, t1) = curve.params();
    let carrier = curve.carrier();
    let starts_there = proven(&body.half_edges, data.he_plus, EntityId::HalfEdge).start == moved;
    let landed = carrier.param_near(onto, if starts_there { t0 } else { t1 });
    let Some(landed) = landed else {
        return Err(ShellError::OpenFaceRimNotExpressible {
            face: designated,
            what: "a seam of the designated chart has no carrier stretch to the counterpart's \
                   corner",
        });
    };
    let (param_start, param_end) = if starts_there {
        (landed, t1)
    } else {
        (t0, landed)
    };
    // A declared source is restricted with the interval, as a split
    // restricts it; a chart image is a function of the carrier's own
    // parameter and travels verbatim.
    let span = t1 - t0;
    let description = match curve.restated_description() {
        geom_brep::EdgeDescriptionSpec::Chart {
            surface,
            image,
            seam,
            declared,
        } => geom_brep::EdgeDescriptionSpec::Chart {
            surface,
            image,
            seam,
            declared: declared
                .map(|mc| mc.restrict((param_start - t0) / span, (param_end - t0) / span)),
        },
        other => other,
    };
    Ok(geom_brep::EdgeCurveSpec {
        description,
        carrier: carrier.clone(),
        param_start,
        param_end,
    })
}

/// The half-edges of `r#loop` in cycle order. Every loop the seamed
/// band reads bounds a face of a finished wall or is a ring the glue
/// just made of one, so it is a cycle.
#[track_caller]
fn cycle_of<T: Real>(body: &Body<T>, r#loop: LoopKey) -> Vec<HeKey> {
    let LoopBoundary::Cycle { first } = proven(&body.loops, r#loop, EntityId::Loop).boundary else {
        unreachable!(
            "{:?} bounds a face of a finished wall, so it is a cycle",
            r#loop
        )
    };
    body.loop_walk(first).closed("loop", first)
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
    let rings_of = |face: FaceKey| proven(&body.faces, face, EntityId::Face).rings.clone();
    let rim_rings = rings_of(rim);
    let source_rings = rings_of(source);
    let not_expressible =
        |what: &'static str| ShellError::OpenFaceRimNotExpressible { face: rim, what };
    let chart = body
        .face_surface_linked(rim, proven(&body.faces, rim, EntityId::Face))
        .clone();
    match (&rim_rings[..], &source_rings[..]) {
        ([], []) => Ok(Vec::new()),
        ([rim_ring], [source_ring]) => {
            if encloses(
                &chart,
                &loop_points(body, *source_ring),
                &loop_points(body, *rim_ring),
                band,
            ) {
                Err(not_expressible(
                    "the designated face's hole does not sit inside the cavity counterpart's",
                ))
            } else if encloses(
                &chart,
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
/// and a slit joining a hole to its outer cycle.
/// `face` is one this call resolved; its loops, their walks, each
/// member's edge and a lone vertex's point are links, so a miss panics
/// naming the record.
#[track_caller]
fn duplicate_in_loop<T: Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Option<(crate::entity::LoopKey, HeKey, HeKey)> {
    let data = proven(&body.faces, face, EntityId::Face);
    for (r#loop, members) in body.face_boundary_by_loop(face, data) {
        let mut cycle = Vec::new();
        for member in members {
            let BoundaryMember::Edge { he, ek, .. } = member else {
                continue;
            };
            cycle.push((he, ek));
        }
        for (i, &(he1, e1)) in cycle.iter().enumerate() {
            for &(he2, e2) in &cycle[i + 1..] {
                if e2 == e1 {
                    return Some((r#loop, he1, he2));
                }
            }
        }
    }
    None
}

/// The two runs a loop's cycle falls into when `he1` and `he2` are
/// removed: the halves strictly after `he1` up to `he2`, and the
/// halves strictly after `he2` up to `he1`. Both are members of the
/// loop's cycle, as [`duplicate_in_loop`] answered them.
#[track_caller]
fn split_cycle<T: Real>(
    body: &Body<T>,
    r#loop: crate::entity::LoopKey,
    he1: HeKey,
    he2: HeKey,
) -> (Vec<HeKey>, Vec<HeKey>) {
    let LoopBoundary::Cycle { first } = proven(&body.loops, r#loop, EntityId::Loop).boundary else {
        unreachable!(
            "{:?} holds the cycle {he1:?} was walked from",
            EntityId::Loop(r#loop)
        )
    };
    let cycle = body.loop_walk(first).closed("loop", first);
    let position = |he: HeKey| {
        cycle.iter().position(|&m| m == he).unwrap_or_else(|| {
            unreachable!(
                "{he:?}, walked as a member of {:?}'s cycle, is in it",
                r#loop
            )
        })
    };
    let (i, j) = (position(he1), position(he2));
    let (lo, hi) = if i < j { (i, j) } else { (j, i) };
    let between: Vec<HeKey> = cycle[lo + 1..hi].to_vec();
    let around: Vec<HeKey> = cycle[hi + 1..]
        .iter()
        .chain(&cycle[..lo])
        .copied()
        .collect();
    if i < j {
        (between, around)
    } else {
        (around, between)
    }
}

/// How many edges emanate from a vertex this call read out of a record
/// ([`Body::vertex_orbit_linked`]: an orbit that does not walk panics
/// naming the hop, since it has no valence to answer).
#[track_caller]
fn valence<T: Real>(body: &Body<T>, vertex: crate::entity::VertexKey) -> usize {
    body.vertex_orbit_linked(vertex).len()
}

/// Sampled points along a run of half-edges — each edge at the
/// certification schedule's own parameters, so a full-period arc is
/// read as the arc rather than as its (collapsed) chord.
fn half_edge_points<T: Decide>(body: &Body<T>, run: &[HeKey]) -> Vec<geom_core::Point3<T>> {
    let mut out = Vec::new();
    for &he in run {
        let key = proven(&body.half_edges, he, EntityId::HalfEdge).edge;
        let edge = linked(
            &body.edges,
            key,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        // A null edge has no carrier to sample, and no extent either.
        let Some(geom) = body.edge_curve_linked(key, edge).certified() else {
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
    let LoopBoundary::Cycle { first } = proven(&body.loops, r#loop, EntityId::Loop).boundary else {
        return Vec::new();
    };
    half_edge_points(body, &body.loop_walk(first).closed("loop", first))
}

/// Whether `inner` is the ENCLOSED one of two nested loops on the
/// chart `surface`, by mean radius about the pair's common centroid,
/// read in that chart ([`chart_read`]).
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
    surface: &geom::Surface<T>,
    inner: &[geom_core::Point3<T>],
    outer: &[geom_core::Point3<T>],
    band: Band,
) -> bool {
    if inner.is_empty() || outer.is_empty() {
        return false;
    }
    let Some(read) = chart_read(surface, &[inner, outer], band) else {
        return false;
    };
    let (inner, outer) = read.split_at(inner.len());
    let centre = centroid_of(&[inner, outer]);
    let gap = mean_radius(outer, centre) - mean_radius(inner, centre);
    matches!(
        decide("shell_rim_nesting", Margin::of(gap), band),
        Ok(Sign::Positive)
    )
}

/// Point runs read into `surface`'s chart, laid flat in metres: a
/// plane's points as they are (its chart is the plane), a surface of
/// revolution's as `(u·ρ, v·λ, 0)` — `u` unwrapped about the first
/// point's, `ρ` that point's distance from the axis and `λ` the length
/// one unit of `v` spans (`Chart::v_lever`'s). One scale for every
/// point, so the two runs are compared in one flat picture. `None` on a
/// chart with no closed-form read (a spline), or a first point on the
/// axis, where `u` is not defined.
fn chart_read<T: Decide>(
    surface: &geom::Surface<T>,
    runs: &[&[geom_core::Point3<T>]],
    band: Band,
) -> Option<Vec<geom_core::Point3<T>>> {
    use geom::Surface as S;
    let points = runs.iter().flat_map(|run| run.iter().copied());
    let (anchor, axis, u_ref) = match surface {
        S::Plane { .. } => return Some(points.collect()),
        S::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        } => (*origin, *axis, *u_ref),
        S::Cone {
            apex, axis, u_ref, ..
        } => (*apex, *axis, *u_ref),
        S::Sphere {
            center,
            axis,
            u_ref,
            ..
        }
        | S::Torus {
            center,
            axis,
            u_ref,
            ..
        } => (*center, *axis, *u_ref),
        S::Nurbs(_) | S::Approx(_) => return None,
    };
    let v_ref = axis.cross(u_ref);
    let pi = T::from_f64(core::f64::consts::PI);
    let sign = |name: &'static str, x: T| decide(name, Margin::of(x), band).ok();
    let u_v = |p: geom_core::Point3<T>| -> Option<(T, T, T)> {
        let w = p - anchor;
        let h = w.dot(axis);
        let (x, y) = (w.dot(u_ref), w.dot(v_ref));
        let radial = (x.powi(2) + y.powi(2)).sqrt();
        let mut u = y.atan2(x);
        let v = match surface {
            S::Cone { half_angle, .. } => {
                // The mirror nappe's chart u sits half a turn round.
                if sign("shell_chart_nappe", h)? == Sign::Negative {
                    u = u + pi;
                }
                h / half_angle.cos()
            }
            S::Sphere { radius, .. } => *radius * (h / *radius).asin(),
            S::Torus {
                major_radius,
                minor_radius,
                ..
            } => *minor_radius * h.atan2(radial - *major_radius),
            _ => h,
        };
        Some((u, v, radial))
    };
    let mut points = points.peekable();
    let (u0, _, lever) = u_v(*points.peek()?)?;
    if sign("shell_chart_lever", lever)? != Sign::Positive {
        return None;
    }
    points
        .map(|p| {
            let (u, v, _) = u_v(p)?;
            // `u − u0` brought into (−π, π].
            let (sin, cos) = (u - u0).sin_cos();
            Some(geom_core::Point3::new(sin.atan2(cos) * lever, v, T::zero()))
        })
        .collect()
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
    /// An axial body — every face of revolution about one axis, or a
    /// plane normal or parallel to it (a bored box and a D-shaft as
    /// much as a vessel): [`crate::offset_charts_together`], each
    /// corner solved in the meridian half-plane.
    ChartsTogether,
    /// Anything else: [`crate::replace_faces_offset`] chart by chart,
    /// whose oblique corners refuse rather than build.
    PerChart,
}

/// The door for the solids `scope` names. A door is a property of a
/// SOLID — a vessel beside a box tilted off its axis is neither
/// all-planar nor axial while each of the two is one of those — so the
/// ladder reads that solid's
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
        .all(|(k, f)| matches!(body.face_surface_linked(k, f), geom::Surface::Plane { .. }));
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
/// or names an entity that touches one. The door refused on its own
/// clone and left `body` as it read it, so a vertex or edge it names is
/// a record of `body`, and its links resolve.
#[track_caller]
fn offending_face<T: Real>(body: &Body<T>, error: &ReplaceFaceError<T>) -> Option<FaceKey> {
    let face_of_he = |he| Some(body.face_of_linked(he));
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
            face_of_he(proven(&body.vertices, *vertex, EntityId::Vertex).emanating?)
        }
        ReplaceFaceError::TogetherEdgeDisagreement { edge, .. }
        | ReplaceFaceError::TogetherAxialEdge { edge, .. }
        | ReplaceFaceError::ReanchorOffCarrier { edge, .. }
        | ReplaceFaceError::ReanchorPastCarrierEnd { edge, .. }
        | ReplaceFaceError::ReanchorInconclusive { edge, .. }
        | ReplaceFaceError::NurbsLaneUnsupported { edge, .. }
        | ReplaceFaceError::NeighborPoseUnroutable { edge, .. } => {
            face_of_he(proven(&body.edges, *edge, EntityId::Edge).he_plus)
        }
        _ => None,
    }
}

/// The signed distance the offset doors move `from`'s chart by to land
/// it on `onto`'s surface, along the doors' own convention: the chart
/// normal AT `from`'s faces ([`crate::offset_together::ChartMove`]).
///
/// One home: [`geom_brep::offset_distance`], the inverse of the mint,
/// turned by the group's nappe as the doors turn every distance they
/// are handed (only a cone has two). `from` is the cavity counterpart
/// of `onto`'s chart, minted from it by the cavity's door and reverted
/// by the graft, so the two are an offset pair by construction.
fn lift_to<T: Decide>(
    body: &Body<T>,
    from: &[FaceKey],
    onto: FaceKey,
    band: Band,
) -> Result<T, ReplaceFaceError<T>> {
    let surface = |face: FaceKey| {
        let data = proven(&body.faces, face, EntityId::Face);
        body.face_surface_linked(face, data)
    };
    let d = match geom_brep::offset_distance(surface(from[0]), surface(onto)) {
        Ok(d) => d,
        Err(geom_brep::OffsetDistanceError::Offset(error)) => {
            return Err(ReplaceFaceError::Offset {
                face: from[0],
                error,
            });
        }
        Err(geom_brep::OffsetDistanceError::KindsDiffer {
            from: kind,
            onto: other,
        }) => {
            unreachable!(
                "{:?}'s chart, a {kind:?}, is the cavity door's offset of {onto:?}'s {other:?}, \
                 and an offset keeps its kind",
                from[0]
            )
        }
    };
    if !matches!(surface(from[0]), geom::Surface::Cone { .. }) {
        return Ok(d);
    }
    Ok(crate::offset_nappe::group_nappe(body, from, band)?.turn(d))
}

/// Re-points every description on `r#loop` that names `dead` at
/// `live`, re-certifying each through the attach layer. `rim` names
/// the designated face in any refusal and is otherwise unread.
fn rename_loop_surface<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    r#loop: crate::entity::LoopKey,
    dead: crate::geometry::SurfaceKey,
    live: crate::geometry::SurfaceKey,
    tol: Tol,
    rim: FaceKey,
) -> Result<(), ShellError<T>> {
    for (edge, spec) in loop_rekeyed(body, r#loop, dead, live) {
        body.set_edge_curve(edge, spec, tol)
            .map_err(|error| ShellError::Rim {
                face: rim,
                error: error.from_driver(),
            })?;
    }
    Ok(())
}

/// Every edge on `r#loop`, in cycle order, with its stored description
/// restated naming `live` wherever it names `dead`, carrier and
/// interval verbatim. `r#loop` is one an Euler door just returned; its
/// walk and its edges are links, so a miss panics naming the record.
#[track_caller]
fn loop_rekeyed<T: Decide>(
    body: &Body<T>,
    r#loop: crate::entity::LoopKey,
    dead: crate::geometry::SurfaceKey,
    live: crate::geometry::SurfaceKey,
) -> Vec<(crate::entity::EdgeKey, geom_brep::EdgeCurveSpec<T>)> {
    let LoopBoundary::Cycle { first } = proven(&body.loops, r#loop, EntityId::Loop).boundary else {
        return Vec::new();
    };
    let mut specs = Vec::new();
    for he in body.loop_walk(first).closed("loop", first) {
        let edge = proven(&body.half_edges, he, EntityId::HalfEdge).edge;
        let data = linked(
            &body.edges,
            edge,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        let Some(curve) = body.edge_curve_linked(edge, data).certified() else {
            unreachable!(
                "{edge:?} bounds a rim face the shell op built from certified walls, and the \
                 surgery mints no null edge"
            )
        };
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
    specs
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
/// on projected bounding boxes of each face's whole boundary, arcs
/// included ([`footprint`]), GROWN by `thickness` on every side —
/// the footprint an inward offset has past a concave edge
/// ([`footprints_may_overlap`]) — and an ambiguous or escalating box
/// comparison counts as OVERLAPPING. A pair tilted off antiparallel
/// is read when its planes drift apart by at most `t` across its
/// extent, or when its normals' cosine is antiparallel to the band, and
/// its gap is then taken short by that drift. The gate may therefore
/// refuse a staircase body whose faces do not really face each other,
/// or a convex-edged pair whose offsets would have cleared; it cannot
/// miss a planar pair within either window that crosses. A pair tilted
/// further is not read (module docs).
fn wall_clearance<T: Decide>(
    body: &Body<T>,
    partition: &crate::offset_together::Scope,
    thickness: T,
    band: Band,
) -> Result<(), ShellError<T>> {
    let two_t = thickness + thickness;
    let planes = planar_faces(body, partition);
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
            // Facing each other. `|n_a + n_b|` is the chord between one
            // outward normal and the other's reverse, exactly `2·sin(δ/2)`
            // for a tilt `δ` off antiparallel; levered by `L`, a bound on
            // how far apart two points of the pair stand, it is how far
            // one plane drifts from the other across the pair. A pair is
            // read when that drift is within one wall, OR when the cosine
            // is antiparallel to the band — the window the gate read
            // before the lever, which is the wider one when `t/L` is
            // below `√(2ε)`. A pair outside both is the tilted residue
            // (module docs).
            let lever = gate_measured(
                "shell_walls_extent",
                a.reach + b.reach + (b.origin - a.origin).norm(),
                band,
            )
            .map_err(|source| ShellError::Escalated { source })?;
            let drift = (a.normal + b.normal).norm() * lever;
            let within_a_wall = !matches!(
                decide(
                    "shell_walls_antiparallel",
                    Margin::of(drift - thickness),
                    band,
                ),
                Ok(Sign::Positive)
            );
            let antiparallel_cosine = || {
                matches!(
                    decide(
                        "shell_walls_antiparallel_cosine",
                        Margin::of(-(a.normal.dot(b.normal)) - T::one()),
                        band,
                    ),
                    Ok(Sign::Zero)
                )
            };
            if !within_a_wall && !antiparallel_cosine() {
                continue;
            }
            if face_neighbours(body, a.face).contains(&b.face) {
                continue;
            }
            if !footprints_may_overlap(a, b, thickness, band) {
                continue;
            }
            // `gap - drift` bounds the wall from below: for `q` on `b`,
            // `(q − o_a)·n_a = ±gap + (q − o_b)·(n_a + n_b)`, since
            // `(q − o_b)·n_b = 0`, and `|q − o_b| ≤ L`. Together the two
            // offsets close that separation by `t·(1 − n_a·n_b) ≤ 2t`, so
            // a wall of at least `2t` everywhere keeps them apart. Read in either window, so a tilted pair the cosine
            // admits is not measured as parallel.
            let gap = (b.origin - a.origin).dot(a.normal).abs();
            match decide(
                "shell_wall_clearance",
                Margin::of(gap - drift - two_t),
                band,
            )
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
/// OUTWARD normal, a point on it, an in-plane frame, the projected
/// footprint of its boundary in that frame, and how far that footprint
/// reaches from the point.
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
    /// The farthest corner of the footprint box from `origin`. Poison
    /// when a boundary term was, so the gate escalates on it rather
    /// than folding it away.
    reach: T,
}

/// Every planar face of `body`, with its outward normal and projected
/// footprint. Every face is read off the arena, so its records are
/// links and a miss panics naming one.
#[track_caller]
fn planar_faces<T: Decide>(
    body: &Body<T>,
    partition: &crate::offset_together::Scope,
) -> Vec<PlanarFace<T>> {
    let mut out = Vec::new();
    for (face, data) in body.faces() {
        let geom::Surface::Plane {
            origin,
            normal,
            u_ref,
        } = body.face_surface_linked(face, data)
        else {
            continue;
        };
        let normal = plane_outward_normal(data, *normal).vec();
        let frame = InPlane {
            origin: *origin,
            u: *u_ref,
            v: normal.cross(*u_ref),
            n: normal,
        };
        let Some((box_u, box_v)) = footprint(body, face, &frame) else {
            continue;
        };
        let far = |(lo, hi): (T, T)| lo.abs().max(hi.abs());
        out.push(PlanarFace {
            face,
            solid: partition.solid_of(face).unwrap_or_else(|| {
                unreachable!(
                    "{face:?} is in no solid's walk: the partition walks every solid, and on a \
                     tier-1-valid body every face is in a shell a solid owns"
                )
            }),
            origin: *origin,
            normal,
            u_ref: frame.u,
            v_ref: frame.v,
            box_u,
            box_v,
            reach: (far(box_u).powi(2) + far(box_v).powi(2)).sqrt(),
        });
    }
    out
}

/// A plane's frame: `origin`, the in-plane `u` and `v`, and the normal
/// `n`, all three unit and orthogonal.
struct InPlane<T: Real> {
    origin: geom_core::Point3<T>,
    u: geom_core::Vec3<T>,
    v: geom_core::Vec3<T>,
    n: geom_core::Vec3<T>,
}

impl<T: Real> InPlane<T> {
    fn point(&self, p: geom_core::Point3<T>) -> geom_core::Point3<T> {
        let w = p - self.origin;
        geom_core::Point3::new(w.dot(self.u), w.dot(self.v), w.dot(self.n))
    }
}

/// **`face`'s footprint in `frame`**: the `(u, v)` box holding its
/// whole boundary — every vertex, and every edge's arc on its carrier
/// ([`carrier_box`]). The vertices alone are not enough: an arc bowing
/// out of the vertex hull carries region past it, and an extruded
/// disc's cap has its two vertices on one diameter. A poisoned term
/// poisons all four ends. `None` for a face with no boundary member to
/// fold.
#[track_caller]
fn footprint<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    frame: &InPlane<T>,
) -> Option<((T, T), (T, T))> {
    let mut points = Vec::new();
    for he in body.face_cycles_linked(face) {
        let h = proven(&body.half_edges, he, EntityId::HalfEdge);
        let start = frame.point(body.linked_vertex_point(h.start, EntityId::HalfEdge(he), "start"));
        points.push((start.x, start.y));
        let edge = linked(
            &body.edges,
            h.edge,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        // Null scaffolding is a zero-length chord its vertex holds.
        if let Some(curve) = body.edge_curve_linked(h.edge, edge).certified() {
            points.extend(
                carrier_box(curve.carrier(), curve.params(), frame)
                    .into_iter()
                    .flatten(),
            );
        }
    }
    let poison = points
        .iter()
        .flat_map(|&(u, v)| [u, v])
        .find(|x| x.is_poison());
    if let Some(p) = poison {
        return Some(((p, p), (p, p)));
    }
    let (&(u0, v0), rest) = points.split_first()?;
    Some(
        rest.iter()
            .fold(((u0, u0), (v0, v0)), |((ulo, uhi), (vlo, vhi)), &(u, v)| {
                ((ulo.min(u), uhi.max(u)), (vlo.min(v), vhi.max(v)))
            }),
    )
}

/// The `(u, v)` box of `frame` holding `carrier`'s arc over `params`:
/// the projection of [`carrier_ball`]'s ball, which holds the whole arc.
/// `None` for a line, whose two vertices hold it.
///
/// [`carrier_ball`]: crate::splitting::containment::carrier_ball
fn carrier_box<T: Decide>(
    carrier: &geom::Curve3<T>,
    params: (T, T),
    frame: &InPlane<T>,
) -> Option<[(T, T); 2]> {
    if matches!(carrier, geom::Curve3::Line { .. }) {
        return None;
    }
    let (center, radius) = crate::splitting::containment::carrier_ball(carrier, params)
        .unwrap_or_else(|| unreachable!("a certified spline carrier has control points"));
    let c = frame.point(center);
    Some([(c.x - radius, c.y - radius), (c.x + radius, c.y + radius)])
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

/// The signed distance that moves `face` INTO the material: the
/// chart normal points out of the solid on a positively-sensed face and
/// into it on a reversed one, so the caller's thickness magnitude never
/// has to know which is which. `face` is one of the operand's charts,
/// read out of its walk.
#[track_caller]
fn inward<T: Real>(body: &Body<T>, face: FaceKey, thickness: T) -> T {
    if proven(&body.faces, face, EntityId::Face).sense {
        -thickness
    } else {
        thickness
    }
}

/// The designation gates: every named face resolves, is named once,
/// takes its solid's whole group on its chart (`solid_charts`, each
/// solid's own), and leaves its shell with a nonempty, connected
/// remainder.
fn check_designation<T: Decide>(
    body: &Body<T>,
    solid_charts: &[(SolidKey, ChartGroups)],
    open_faces: &[FaceKey],
) -> Result<(), ShellError<T>> {
    for (i, face) in open_faces.iter().enumerate() {
        if body.get_face(*face).is_none() {
            return Err(ShellError::OpenFaceStale { face: *face });
        }
        if open_faces[..i].contains(face) {
            return Err(ShellError::OpenFaceRepeated { face: *face });
        }
    }
    if open_faces.is_empty() {
        return Ok(());
    }
    // A solid's wearers of a chart are lifted as ONE by the rim stage
    // (the group door's own contract), so a partially designated group
    // has no coherent lift.
    for &face in open_faces {
        let key = proven(&body.faces, face, EntityId::Face).surface;
        let group = solid_charts
            .iter()
            .map(|(_, charts)| charts.of(key))
            .find(|group| group.contains(&face))
            .unwrap_or_else(|| {
                unreachable!(
                    "{face:?} is in no solid's chart groups: the groups are read off every \
                     solid's walk, and on a tier-1-valid body every face is in a shell a \
                     solid owns"
                )
            });
        if let Some(&other) = group.iter().find(|f| !open_faces.contains(f)) {
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
        let components = count_components(body, &remaining);
        if components != 1 {
            return Err(ShellError::OpenFacesDisconnect { shell, components });
        }
    }
    Ok(())
}

/// How many edge-adjacency components `faces` falls into — the
/// validator's own pass-11 relation, restricted to a subset.
#[track_caller]
fn count_components<T: Decide>(body: &Body<T>, faces: &[FaceKey]) -> usize {
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
            for neighbour in face_neighbours(body, face) {
                if faces.contains(&neighbour) && !seen.contains(&neighbour) {
                    seen.push(neighbour);
                    work.push(neighbour);
                }
            }
        }
    }
    components
}

/// The faces `face` shares an edge with, for a face a shell's record
/// lists: every hop is a link, so a miss panics naming the record.
#[track_caller]
fn face_neighbours<T: Decide>(body: &Body<T>, face: FaceKey) -> Vec<FaceKey> {
    let mut out = Vec::new();
    for he in body.face_cycles_linked(face) {
        let mate = body.proven_mate(he, crate::live::link(EntityId::Face(face), "cycle"));
        let parent = body.face_of_linked(mate.mate);
        if parent != face && !out.contains(&parent) {
            out.push(parent);
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::test_support::finished;

    #[allow(clippy::panic)]
    mod footprint_fuzz;

    /// **Nesting on a curved chart is read in the chart.** Two windows
    /// on a unit cylinder about `z`, both straddling `u = π`, where
    /// the raw azimuth jumps a turn, so the read has to unwrap it: the
    /// inner spans `π ± 0.2` rad and `z ∈ [0.2, 0.8]`, the outer `π ± 0.4` rad and `z ∈ [0, 1]`.
    /// The inner is inside and the reverse question answers no. A run
    /// whose first point is on the axis has no `u`, and the read refuses
    /// rather than guessing.
    #[test]
    fn nesting_on_a_cylinder_is_read_in_its_chart() {
        let band = Band::linear(Tol::witness()).unwrap();
        let cylinder = geom::Surface::Cylinder {
            origin: geom_core::Point3::new(0.0, 0.0, 0.0),
            axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
        };
        let window = |du: f64, z0: f64, z1: f64| -> Vec<geom_core::Point3<f64>> {
            let mut out = Vec::new();
            for i in 0..=8 {
                let u = core::f64::consts::PI - du + 2.0 * du * f64::from(i) / 8.0;
                out.push(geom_core::Point3::new(u.cos(), u.sin(), z0));
                out.push(geom_core::Point3::new(u.cos(), u.sin(), z1));
            }
            out
        };
        let (inner, outer) = (window(0.2, 0.2, 0.8), window(0.4, 0.0, 1.0));
        assert!(encloses(&cylinder, &inner, &outer, band), "inner in outer");
        assert!(
            !encloses(&cylinder, &outer, &inner, band),
            "not the reverse"
        );
        let on_axis = vec![geom_core::Point3::new(0.0, 0.0, 0.5)];
        assert!(chart_read(&cylinder, &[&on_axis, &inner], band).is_none());
    }

    /// **The duplicate scan panics on a ring link that does not
    /// resolve**, where it stepped over it.
    #[test]
    fn the_duplicate_scan_panics_on_a_torn_ring_link() {
        use crate::live::OPERATORS_KEEP_LINKS;
        use crate::review_d18::{ROW_FOUR, assert_torn_op_panics, tear_ring};
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        assert!(
            duplicate_in_loop(&body, face).is_none(),
            "a cube face has no slit"
        );
        let named = tear_ring(&mut body, face);
        assert_torn_op_panics(
            "duplicate_in_loop",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| duplicate_in_loop(b, face),
        );
    }

    /// A vertex whose orbit does not walk has no valence to answer, so
    /// the read panics naming the walk rather than answer zero, which
    /// the spur test reads as "not a tip" and sends to the slit's
    /// `kemr`. A lone vertex meets no edge and answers zero.
    #[test]
    fn valence_panics_on_a_vertex_whose_orbit_does_not_walk() {
        let (body, halves, v) = crate::fixtures::torn_cube_closing_through_another_vertex();
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            let _ = valence(&body, v);
        }));
        assert!(
            report.contains("orbit walk from") && report.contains(crate::body::WALKS_CLOSE),
            "the torn walk: {report}"
        );
        let cube = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness()).body;
        let start = body.get_half_edge(halves[11]).unwrap().start;
        assert_eq!(start, v);
        assert_eq!(valence(&cube, v), 3, "the same vertex untorn");
        let lone = crate::fixtures::mvfs_state();
        assert_eq!(valence(&lone.body, lone.vertex), 0, "a lone vertex");
    }

    /// A torn operand is refused where the verb's operand is gated,
    /// naming the torn record, and so never reaches the verb.
    #[test]
    fn a_torn_operand_is_refused_at_the_gate_naming_the_record() {
        let tol = Tol::witness();
        let mut body = crate::splitting::reassembly::quad_prism(
            &crate::test_support_fixtures::UNIT_SQUARE,
            1.0,
            tol,
        );
        let face = body.faces().nth(2).unwrap().0;
        let dead = body.add_surface(geom::Surface::Plane {
            origin: geom_core::Point3::new(0.0, 0.0, 0.0),
            normal: geom_core::Vec3::new(0.0, 0.0, 1.0),
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
        });
        body.surfaces.remove(dead);
        body.get_face_mut(face).unwrap().surface = dead;
        let errors = AtRestBody::validate(body, tol).expect_err("a torn body is not finished");
        assert!(
            errors.contains(&ValidationError::DanglingGeometry {
                from: EntityId::Face(face),
                to: crate::entity::GeomRef::Surface(dead),
            }),
            "{errors:?}"
        );
    }

    /// A neighbour is read through the mate's loop, and the face that
    /// loop names must list it: a loop torn to name another face would
    /// join two components through a loop that face does not own, so
    /// the hop panics naming the loop rather than counting it.
    #[test]
    fn face_neighbours_panics_on_a_loop_its_face_does_not_list() {
        let tol = Tol::witness();
        let mut body = crate::splitting::reassembly::quad_prism(
            &crate::test_support_fixtures::UNIT_SQUARE,
            1.0,
            tol,
        );
        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        let (torn, seed) = (faces[1], faces[0]);
        let neighbours = face_neighbours(&body, torn);
        let probe = *neighbours
            .iter()
            .find(|&&f| f != seed)
            .expect("a side face has a neighbour other than the seed");
        let outer = body.get_face(torn).unwrap().outer;
        body.get_loop_mut(outer).unwrap().face = seed;
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            let _ = face_neighbours(&body, probe);
        }));
        assert!(
            report.contains(&format!(
                "loop {outer:?} names face {seed:?}, which does not list it"
            )),
            "{report}"
        );
    }

    /// A designation is the caller's key: a stale one stays typed.
    #[test]
    fn a_stale_designation_stays_typed() {
        let tol = Tol::witness();
        let body = crate::test_support_fixtures::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let stale = FaceKey::default();
        assert!(matches!(
            shell_open(&finished("the brick", body, tol), 0.1, &[stale], tol),
            Err(ShellError::OpenFaceStale { face }) if face == stale
        ));
    }
}
