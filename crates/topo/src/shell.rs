//! `shell` — hollowing, sealed and opened.
//!
//! **Shelling** turns a solid into a thin-walled one: the boundary is
//! offset inward by the wall thickness, and the hollow is either kept
//! closed (a cavity) or opened by designating faces whose material is
//! removed, leaving annular rims where the wall's thickness shows
//! (`docs/OFFSET-DESIGN.md`'s vocabulary, unchanged).
//!
//! # The sealed arm, and what it deliberately does not run
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
//!    body**: an ALL-PLANAR operand goes through
//!    [`crate::offset_planes_together`], which moves every chart at
//!    once and solves each corner against all the moved planes meeting
//!    it; anything with a curved face goes chart by chart through
//!    [`crate::replace_faces_offset`], whose corners are transported
//!    once per chart and whose OBLIQUE ones therefore refuse
//!    (`ReanchorOffCarrier`) rather than build. The split is #1081's:
//!    the planar half of that class is repaired and the curved half is
//!    not;
//! 2. that body inserted through the shared void-insertion door
//!    ([`crate::boolean::voids::insert_void`]) with carried evidence;
//! 3. one validation.
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
//! reach claim, and on a PLANE it is vacuous — a plane's reach is
//! unbounded, so no per-face decide can see two walls marching through
//! each other. That collision class is gated separately and in closed
//! form by [`wall_clearance`], which is what makes the carried evidence
//! sound on planar operands.
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
//! parameter box is M10's machinery; the window is issue #1055.
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
//! # The opened arm
//!
//! `shell_open(body, t, open_faces)` is the sealed construction plus
//! rim surgery, and it composes rather than inventing:
//!
//! 1. the sealed shell, exactly as above — so the evidence handed to
//!    the void door is the strict one, before anything is opened;
//! 2. per designated CHART, its CAVITY counterpart offset back OUTWARD
//!    by `t` ([`crate::replace_face_offset`] again), which lands it on
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
//! The result is a CLOSED thin solid with one shell: the designated
//! face is now annular — the rim, where the wall thickness shows.
//! **Nothing opens** — that is the load-bearing half, and D1's
//! manifold-first stance is untouched. Genus does not necessarily rise:
//! one opening gives a cup, which is genus 0 (the cavity shell fuses
//! into the boundary and the two Euler contributions cancel); a second
//! opening gives a tube, genus 1. The invariant is closure, not genus. The surgery is the ring-topology
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
use geom_core::{Band, Decide, Indeterminate, Margin, Real, Sign, Tol};

use crate::body::Body;
use crate::boolean::voids::{VoidContainment, VoidEvidence, VoidInsertError, insert_void};
use crate::entity::{EntityId, FaceKey, HalfEdgeKey as HeKey, LoopBoundary, ShellKey, SolidKey};
use crate::euler::EulerOpError;
use crate::props::PropsQuadLane;
use crate::replace_face::ReplaceFaceError;
use crate::validate::{ValidationError, validate_geometric};

/// Typed refusal of the shell verb (closed enum, D4 ¶3).
#[derive(Clone, Debug)]
pub enum ShellError<T: Real> {
    /// The wall thickness is not certifiably positive: a zero or
    /// negative wall is not a thin solid, and the ambiguity band
    /// escalates rather than guessing.
    Thickness {
        /// The thickness as given, echoed as data.
        thickness: T,
    },
    /// The body is not a single solid. Shelling is a per-solid verb and
    /// the void door inserts into ONE solid; a multi-solid operand
    /// would need a designation this verb does not take.
    NotOneSolid {
        /// How many solids the body carries.
        solids: usize,
    },
    /// **The operand is already hollow.** Its boundary is more than one
    /// shell, so "the inward offset of every boundary face" would erode
    /// the CAVITY walls outward into the material as readily as it
    /// erodes the outer ones inward, and the cavity clone's shells
    /// would be inserted as new voids beside the existing one —
    /// overlapping it, with carried evidence that says nothing about
    /// the void-derived shell because it never was in material.
    ///
    /// Refused rather than answered wrongly. **The eventual semantics
    /// is ratified and is NOT "offset the outer shell only"**: shelling
    /// a hollow body must THICKEN EVERY BOUNDARY, outer inward and each
    /// void outward. See the issue cited at the gate site.
    OperandAlreadyHollow {
        /// How many shells the operand's solid carries.
        shells: usize,
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
    /// The assembled result does not validate, so it is discarded.
    NotValid {
        /// The validator's report.
        errors: Vec<ValidationError>,
    },
}

impl<T: Real> core::fmt::Display for ShellError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Thickness { thickness } => write!(
                f,
                "shell: the wall thickness ({thickness:?} m) is not certifiably positive, so \
                 there is no thin solid to build"
            ),
            Self::NotOneSolid { solids } => write!(
                f,
                "shell: the operand carries {solids} solids — shelling is a per-solid verb and \
                 this door takes no designation to pick one"
            ),
            Self::OperandAlreadyHollow { shells } => write!(
                f,
                "shell: the operand's solid already carries {shells} shells — shelling a hollow \
                 body must thicken EVERY boundary (outer inward, each void outward), which this \
                 verb does not do yet, and offsetting only the outer shell is explicitly not \
                 the answer. Nothing is built"
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
/// # Errors
///
/// [`ShellError`] — the thickness gate, the per-face offset refusals
/// (which are the containment evidence's own decides), the void door's
/// refusals, and a result that does not validate.
pub fn shell<T: Decide + PropsQuadLane>(
    body: &Body<T>,
    thickness: T,
    tolerance: f64,
    band: Band,
    tol: Tol,
) -> Result<Body<T>, ShellError<T>> {
    shell_open(body, thickness, &[], tolerance, band, tol)
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
/// [`ShellError`] — [`shell`]'s, plus the designation gates (a face
/// must resolve, be named once, leave a nonempty and connected
/// remainder) and the rim surgery's own refusal.
pub fn shell_open<T: Decide + PropsQuadLane>(
    body: &Body<T>,
    thickness: T,
    open_faces: &[FaceKey],
    tolerance: f64,
    band: Band,
    tol: Tol,
) -> Result<Body<T>, ShellError<T>> {
    // ---- Decide: the thickness. ----
    match decide("shell_thickness", Margin::of(thickness), band) {
        Ok(Sign::Positive) => {}
        _ => return Err(ShellError::Thickness { thickness }),
    }

    // ---- Decide: one solid. ----
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    let [solid] = solids[..] else {
        return Err(ShellError::NotOneSolid {
            solids: solids.len(),
        });
    };

    // ---- Decide: the operand is not already hollow. ----
    //
    // Issue #1056 carries the ratified semantics for when it is.
    let shells = body
        .get_solid(solid)
        .ok_or(ShellError::Corrupt {
            key: EntityId::Solid(solid),
        })?
        .shells
        .len();
    if shells != 1 {
        return Err(ShellError::OperandAlreadyHollow { shells });
    }

    // ---- Decide: every chart has ONE orientation. ----
    let charts = chart_groups(body);
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
        for &member in &group[1..] {
            if sense(member)? != first {
                return Err(ShellError::ChartSenseMixed {
                    face: group[0],
                    other: member,
                });
            }
        }
    }

    // ---- Decide: the walls are thick enough to hold two offsets. ----
    wall_clearance(body, thickness, band)?;

    // ---- Decide: the designation. ----
    check_designation(body, open_faces)?;

    // ---- The cavity: one clone, every CHART inward. ----
    //
    // By chart, not by face: a full revolve splits its wall into two
    // bands over one cylinder, and such a surface has to move as one
    // (the face-replacement door's own group form says why). Grouping
    // is by surface key, in face-arena order, so the walk is
    // deterministic.
    let mut cavity = body.clone();
    // **All-planar bodies move SIMULTANEOUSLY; everything else still
    // moves chart by chart.** Composing the per-chart door over a body
    // cannot offset an OBLIQUE junction: a corner is visited once per
    // chart and transported rigidly each time, so it accumulates
    // `Σ dᵢ·nᵢ` where the offset body needs the point satisfying every
    // `nᵢ·x = nᵢ·oᵢ + dᵢ` at once. Those agree exactly when the normals
    // are mutually perpendicular — which is why a box was always right
    // — and diverge otherwise. `ReanchorOffCarrier` is what has been
    // refusing the difference rather than building it, and it stays
    // exactly where it was for every body this branch does not take:
    // the curved corners are the C5-table work that follows.
    let all_planar = cavity.faces().all(|(_, f)| {
        matches!(
            cavity.get_surface(f.surface),
            Some(geom::Surface::Plane { .. })
        )
    });
    if all_planar {
        let mut moves: Vec<crate::offset_together::ChartMove<T>> = Vec::with_capacity(charts.len());
        for group in &charts {
            moves.push(crate::offset_together::ChartMove {
                faces: group.clone(),
                distance: inward(&cavity, group[0], thickness)?,
            });
        }
        let named = charts
            .first()
            .and_then(|g| g.first())
            .copied()
            .ok_or(ShellError::Corrupt {
                key: EntityId::Solid(solid),
            })?;
        crate::offset_planes_together(&mut cavity, &moves, band, tol).map_err(|error| {
            ShellError::Face {
                face: named,
                error: Box::new(error),
            }
        })?;
    } else {
        for group in &charts {
            let face = group[0];
            let d = inward(&cavity, face, thickness)?;
            crate::replace_faces_offset(&mut cavity, group, d, tolerance, band, tol).map_err(
                |error| ShellError::Face {
                    face,
                    error: Box::new(error),
                },
            )?;
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
    let mut out = body.clone();
    // The cavity is a CLONE of the operand, so a designated face's
    // counterpart carries the same key in the cavity's key space —
    // which is the space `VoidInserted` maps from.
    let inserted = insert_void(&mut out, solid, cavity, &evidence, tol)
        .map_err(|error| ShellError::Insert { error })?;

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
    let mut charts: Vec<(crate::geometry::SurfaceKey, Vec<FaceKey>)> = Vec::new();
    for &designated in open_faces {
        let chart = out
            .get_face(designated)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(designated),
            })?
            .surface;
        match charts.iter_mut().find(|(key, _)| *key == chart) {
            Some((_, faces)) => faces.push(designated),
            None => charts.push((chart, vec![designated])),
        }
    }
    for (_, group) in charts {
        let designated = group[0];
        let sources: Vec<FaceKey> = group
            .iter()
            .map(|&f| {
                inserted.face(f).ok_or(ShellError::Corrupt {
                    key: EntityId::Face(f),
                })
            })
            .collect::<Result<_, _>>()?;

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
        let lift_group: Vec<FaceKey> = out
            .faces()
            .filter(|(_, f)| f.surface == counterpart_chart)
            .map(|(k, _)| k)
            .collect();
        let back = lift_to(&out, sources[0], designated)?;
        crate::replace_faces_offset(&mut out, &lift_group, back, tolerance, band, tol).map_err(
            |error| ShellError::Lift {
                face: designated,
                error: Box::new(error),
            },
        )?;
        // Read AFTER the lift: `FaceSurface::New` minted a fresh key
        // for the moved chart, and that key — not the one the graft
        // brought in — is what the ring's descriptions now name.
        let dead_surface = out
            .get_face(sources[0])
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(sources[0]),
            })?
            .surface;

        // One face per side, loops disjoint.
        let rim = canonicalize_chart(&mut out, &group, band)?;
        let source = canonicalize_chart(&mut out, &sources, band)?;
        let (rim_surface, rim_sense) = {
            let data = out.get_face(rim).ok_or(ShellError::Corrupt {
                key: EntityId::Face(rim),
            })?;
            (data.surface, data.sense)
        };

        // The counterpart's rings are the counterparts of the
        // designated face's OWN rings — an annular mouth's correct rim
        // is not one region but one per hole plus one for the outer
        // boundary, and a face is one region. Each counterpart ring is
        // therefore promoted to the outer loop of its own rim face
        // BEFORE the glue (which requires a ring-free counterpart),
        // and after the glue takes the designated face's matching ring
        // with it.
        let pairs = pair_rings(&out, rim, source, band)?;
        let mut promoted: Vec<(FaceKey, crate::entity::LoopKey)> = Vec::new();
        for &(source_ring, rim_ring) in &pairs {
            let made = out
                .mfkrh(source_ring, crate::euler::FaceSurface::Shared(rim_surface))
                .map_err(|error| ShellError::Rim {
                    face: designated,
                    error,
                })?;
            // The promoted face inherits the RIM's orientation, not
            // the counterpart's: `mfkrh` with a `Shared` surface mints
            // `sense: true`, and the counterpart is a cavity wall whose
            // outward normal points the other way. The winding works
            // out by construction — a ring of the counterpart is wound
            // opposite to the counterpart's outer loop, i.e. the way an
            // outer loop of a rim-facing face must be — and it is not
            // asserted here on that argument alone: tier 3's check 6
            // reads `sense` against the stored loop windings on every
            // planar face, and check 7 reads the volume the same
            // windings integrate, so a flip either way reds at the
            // verb's own closing `validate_geometric` rather than
            // shipping.
            out.set_face_sense(made.face, rim_sense)
                .map_err(|error| ShellError::Rim {
                    face: designated,
                    error,
                })?;
            rename_loop_surface(&mut out, source_ring, dead_surface, rim_surface, tol, rim)?;
            promoted.push((made.face, rim_ring));
        }

        // The counterpart's boundary must now be an interior-disjoint
        // ring of the rim — the invariant the validator states as
        // check 9. Refused HERE, naming the shape, rather than left to
        // arrive as a generic at-rest report on a body already built.
        let source_outer = out
            .get_face(source)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(source),
            })?
            .outer;
        let rim_outer = out
            .get_face(rim)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(rim),
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
        match crate::validate::ring_outer_contact(&out, rim_outer, source_outer, band) {
            crate::validate::RingOuterVerdict::Disjoint => {}
            crate::validate::RingOuterVerdict::Contact(_) => {
                return Err(ShellError::OpenFaceRimNotExpressible {
                    face: designated,
                    what: "the cavity counterpart's boundary meets the designated face's own \
                           boundary, so it cannot become an interior-disjoint ring of it",
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
        for &(source_ring, rim_ring) in &pairs {
            match crate::validate::ring_outer_contact(&out, source_ring, rim_ring, band) {
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

        // The connected sum: the counterpart dies, its outer loop
        // becomes the rim's RING, and the cavity shell fuses into the
        // outer one (the first chart does the fusion; any further
        // chart is same-shell genus surgery).
        let fused = out.kfmrh(rim, source).map_err(|error| ShellError::Rim {
            face: designated,
            error,
        })?;
        // The ring's edges still NAME the surface that just died. They
        // lie on the rim's surface now — the lift put the two planes on
        // top of each other — so the re-description is a key swap with
        // the carrier untouched, and the attach layer certifies it
        // against the geometry rather than taking the swap on trust.
        rename_loop_surface(&mut out, fused.ring, dead_surface, rim_surface, tol, rim)?;
        // Each promoted rim face takes its matching hole with it: the
        // fusion put every face in one shell, which is `ring_move`'s
        // precondition.
        for (face, rim_ring) in promoted {
            out.ring_move(rim_ring, face)
                .map_err(|error| ShellError::Rim {
                    face: designated,
                    error,
                })?;
        }
    }

    // ---- One validation. ----
    validate_geometric(&out, tol).map_err(|errors| ShellError::NotValid { errors })?;
    Ok(out)
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
fn canonicalize_chart<T: Decide>(
    body: &mut Body<T>,
    faces: &[FaceKey],
    band: Band,
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
            body.kef(he).map_err(|error| ShellError::Rim {
                face: anchor,
                error,
            })?;
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
            body.kev(he1).map_err(|error| ShellError::Rim {
                face: anchor,
                error,
            })?;
            continue;
        }
        if far(he2).is_some_and(|v| valence(body, v) == 1) {
            body.kev(he2).map_err(|error| ShellError::Rim {
                face: anchor,
                error,
            })?;
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
                    *curve.description(),
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
/// on projected bounding boxes, and an ambiguous or escalating box
/// comparison counts as OVERLAPPING. The gate may therefore refuse a
/// staircase body whose faces do not really face each other; it cannot
/// miss a pair that does.
fn wall_clearance<T: Decide>(
    body: &Body<T>,
    thickness: T,
    band: Band,
) -> Result<(), ShellError<T>> {
    let two_t = thickness + thickness;
    let planes = planar_faces(body)?;
    for (i, a) in planes.iter().enumerate() {
        for b in &planes[i + 1..] {
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
            if !footprints_may_overlap(a, b, band) {
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
    origin: geom_core::Point3<T>,
    normal: geom_core::Vec3<T>,
    u_ref: geom_core::Vec3<T>,
    v_ref: geom_core::Vec3<T>,
    box_u: (T, T),
    box_v: (T, T),
}

/// Every planar face of `body`, with its outward normal and projected
/// footprint.
fn planar_faces<T: Real>(body: &Body<T>) -> Result<Vec<PlanarFace<T>>, ShellError<T>> {
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
/// in-plane frame? `true` on any ambiguity — the conservative answer.
fn footprints_may_overlap<T: Decide>(a: &PlanarFace<T>, b: &PlanarFace<T>, band: Band) -> bool {
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
    !(separated(a.box_u, re_u) || separated(a.box_v, re_v))
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
    let mut keys: Vec<crate::geometry::SurfaceKey> = Vec::new();
    let mut groups: Vec<Vec<FaceKey>> = Vec::new();
    for (key, face) in body.faces() {
        match keys.iter().position(|k| *k == face.surface) {
            Some(i) => groups[i].push(key),
            None => {
                keys.push(face.surface);
                groups.push(vec![key]);
            }
        }
    }
    groups
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
