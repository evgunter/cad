//! **The loft/sweep BODY assembly** (M6-3, executing the design
//! banked as M5 PR 9c item 6).
//!
//! The topology is EXTRUDE'S with different geometry (item 6(i)):
//! bottom cap from section 0, top cap from section k−1, one NURBS wall
//! per profile segment ([`crate::skin::body_walls`]) cut at every C⁰
//! knot its `u` carries, the sections' sub-arc joints
//! ([`fn@sub_arc_walls`], [`fn@segment_pieces`]), wall–wall seams as the
//! walls' `u ∈ {0, 1}` boundary iso-curves, struts swept per vertex and
//! per joint. At v-degree 1 through `k ≥ 3` sections that wall is one
//! per slab, built from its two sections' rows: the body is `k − 1` such laminae swept
//! one on the next, each interior section a ring of rims between two
//! slabs' walls, each in the row `v = 1` of the wall below it. The three
//! edge classes:
//!
//! - **Cap–wall rims** need no new GEOMETRY (item 6(ii)): the wall's
//!   `v = 0` / `v = 1` iso IS the placed sketch segment (degree
//!   elevation and knot refinement are exact), so the carrier stays
//!   the `Curve3::Line`/`Circle` it was minted as, verbatim, and
//!   certifies today. Its DESCRIPTION still moves: minted through the
//!   scaffolding door as `MappedSource::PlacedSegment` (the cap plane
//!   is fitted through the rim, so it does not exist yet), it is
//!   re-stated as an image in the cap's own chart once the plane does
//!   ([`crate::swept::describe_face_rim_at_rest`] — D3's transience
//!   fence). No cap–wall dihedral is classified here: unlike extrude,
//!   loft does not upgrade these rims to `Intersection`, because the
//!   plane × NURBS certificate refuses many of their at-rest rows
//!   (`work/ssiedge/plane-nurbs-certificate-refuses-at-rest-rows-of-lofts-and-sweeps.md`).
//! - **Wall–wall seams** are the genuinely new class (item 6(iii)):
//!   an iso image of over the wall's boundary
//!   row (`geom_brep::boundary_iso_u` — a control-net copy, no
//!   arithmetic), certified through the metric residual
//!   `|C(t) − S(u, v(t))|` at the CERT schedule.
//! - **Struts** are minted as scaffolding lines and UPGRADED to the
//!   seam class once their walls' surface keys exist — the extrude
//!   Phase-6 rim-upgrade idiom, one class over.
//!
//! The final pass re-mints the whole body's pcurves
//! ([`topo::mint_pcurves`]): every wall boundary stores the exact
//! line-in-UV image ([`geom_brep::Pcurve`]'s iso lane) — tier-3 green
//! at rest is the BUILDER's acceptance, not a follow-up.
//!
//! # Orientation (the canonical stacking arm)
//!
//! Before the skin, the stacking door ([`stacking::stacking_door`])
//! reads each slab's two sections, slab by slab: the corners they pin
//! (a profile vertex at one world point in both), whether they touch
//! beyond those corners, and each slab's displacement, which must point
//! ALONG the plane normals of both sections it spans. That is a
//! PER-SLAB statement, and a refusal names the earliest pair it stopped
//! at. The clearance refuses only contact it verified: an arc pair the
//! curve-pair reader cannot settle decides nothing there, and is the
//! embedding certificate's. The
//! displacement check is a conservative orientation check, not an
//! embedding one: it refuses some embedded lofts whose far section
//! leans back across the stack, and nothing at this door yet certifies
//! that the walls do not cross each other or the caps away from the
//! sections.
//!
//! The caps then orient exactly as extrude's (bottom reversed, top
//! forward) and every wall's chart normal `S_u × S_v` points out of
//! the material — the u direction follows the material-left profile
//! traversal and v the stacking, so `t̂ × v̂` is material-right —
//! giving `sense = true` on every wall, holes and concave arcs
//! included (unlike a cylinder chart, the skinned chart's normal
//! FOLLOWS the traversal; there is no unconditionally-radial frame to
//! fight). Every rim reads the normal of the section it lies in: the
//! bottom cap and its rims read section 0's, the top cap and its rims
//! section `k − 1`'s, and an interior section's ring its own
//! (`assemble`'s `rim_spec`). The door has decided each of those normals
//! against the displacement of the slab or slabs it bounds, and its
//! margins are not read again.
//!
//! # Scalar posture (C6)
//!
//! The sections and placements arrive in the body's scalar `T`. A
//! section or placement holding a value that is not finite refuses
//! first, at every scalar. Structure — each section's canonical form,
//! the skin's degrees, multiplicities and parameter order — is what
//! every value the inputs stand for shares, or a typed refusal. Where
//! it is decided, and so what is built, depends on which side of the
//! projective partition `T` is on ([`fn@build`]):
//!
//! - at `f64`, `Probe`, and `Dual` and `Sym` over them, every value is
//!   a point: each section's structure is selected once at `f64` on its
//!   value channel, the sections are validated at `T` along that record
//!   and the walls are skinned in `T` ([`fn@crate::skin`]), so a wall, a cap and the
//!   stacking door's margins are the functions of the sections'
//!   variables the sections are;
//! - at `Interval`, and the wrappers over it, the body encloses the
//!   family its sections stand for: the sections are validated and
//!   skinned in `T`, with each decision certified there and no `f64`
//!   record read, and the walls enclose the family. A decision the
//!   `f64` arithmetic takes and the lane cannot certify refuses, so the
//!   two sides need not build the same body. Sections and
//!   placements that are points are a family of one, built the same
//!   way, so its walls enclose the `f64` loft by the rounding of their
//!   own arithmetic. The skin's structure is what every value of the
//!   family shares, and a carrier's weights and knots are points, so
//!   an arc section (its weight `cos(Δ/2)` is an enclosure from
//!   rounding alone), or an interior knot wider than a point, refuses
//!   typed
//!   (`work/inlane/an-enclosure-carrier-holds-point-weights-and-knots.md`).
//!   A loft's interior v knots are averages of chord-length parameters
//!   read from its sections, so they are such knots; a path sweep's are
//!   averages of its station shares, a count's fractions, and are
//!   constants at every scalar ([`mod@crate::skin`]'s `stated_knots`).

mod stacking;

use core::fmt;
use std::sync::Arc;
use topo::construction::Construct;

use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::spline::{ProjectiveScalar, ProjectiveSplit, SplineError};
use geom_core::{Affine3, Band, BandError, Decide, Indeterminate, Point2, Point3, Real, Tol};
use profile::{SketchPlane, ValidatedProfile};
use topo::construction::{FaceSurface, MefSite, MevSite};
use topo::{Body, EdgeKey, EulerOpError, FaceKey, PcurveMintError, ShellKey, SolidKey};

use crate::skin::{
    PathCarry, Section, SectionLoop, SkinError, body_walls, check_counts, check_shapes,
    compatible_strips, validate_at_t, validate_guided, validate_loft,
};
use crate::swept::{
    CapEnd, SweptSeg, cap_plane, describe_face_rim_at_rest, face_surface_key, placed_segment_spec,
    swept_segments,
};

/// Everything [`loft_body`]/[`sweep_body`] built, keyed — the
/// [`crate::Extruded`] bundle one operation over.
#[derive(Debug)]
pub struct Lofted<T: topo::AtRestPolicy, B = <T as topo::AtRestPolicy>::Leaf> {
    /// The closed body, the door's leaf ([`topo::AtRestPolicy::leaf`]:
    /// finished, tiers 1–3, at `f64` and `Probe`; tier 2 alone at the box
    /// scalars, where the consumer finishes it); construction state (`Body<T>`) inside the builder. Tier 3
    /// does not test embedding, and neither does this door, so a body
    /// whose walls cross each other or a cap is not refused here.
    pub body: B,
    /// The solid.
    pub solid: SolidKey,
    /// Its one shell.
    pub shell: ShellKey,
    /// The top cap (section k−1's plane, outward along the stacking).
    pub top: FaceKey,
    /// The bottom cap (section 0's plane, outward against it).
    pub bottom: FaceKey,
    /// Wall faces `[loop][segment][sub-arc][slab]`, loops in canonical
    /// order (outer first), segments in canonical order, sub-arcs along
    /// the segment, slabs in stacking order. A segment's wall is cut at
    /// every C⁰ knot its skinned surface carries in `u`, the sub-arc
    /// joints of an arc wider than the sub-arc ceiling in any section
    /// (`crate::skin::segment_curve`), so a lofted circle is four walls
    /// and a segment with no such joint is one. At v-degree ≥ 2, and
    /// through two sections, each sub-arc has one wall,
    /// `side_faces[l][j][q][0]`, on the stretch of
    /// `LoftGeometry.walls[l][j]` between its joints; at v-degree 1
    /// through `k ≥ 3` sections it has one per slab,
    /// `side_faces[l][j][q][s]`, the stretch of the ruled wall whose rows
    /// are sections `s` and `s + 1`.
    /// `None` where the segment is pinned at both ends and is the same
    /// curve in both sections of the slab: it has no wall there, and the
    /// walls beside it share its edge.
    pub side_faces: Vec<Vec<Vec<Vec<Option<FaceKey>>>>>,
    /// Wall–wall seam edges at the profile's vertices,
    /// `[loop][vertex][slab]`: `seams[l][j][s]` is the strut at vertex
    /// `j` across slab `s` — the `u = 0` boundary iso of segment `j`'s
    /// first sub-arc wall in slab `s` (and the `u = 1` of segment
    /// `j − 1 mod n`'s last). Struts of adjacent slabs meet at an
    /// interior section's vertex, and the edges of that section's ring
    /// are the rims between adjacent slabs' walls. `None` where the
    /// corner is pinned in the slab: one vertex in both sections, with
    /// no strut.
    pub seam_edges: Vec<Vec<Vec<Option<EdgeKey>>>>,
    /// The seams inside a segment, `[loop][segment][joint][slab]`:
    /// `joints[l][j][q][s]` is the seam between segment `j`'s
    /// sub-arc walls `q` and `q + 1` across slab `s`, the `u = 0`
    /// boundary iso of wall `q + 1`. It is a crease in general: smooth
    /// where every section's joint there is one of arcs of one sweep
    /// (one weight), as a sweep's are, or at v-degree 1
    /// (`crates/sweep/README.md`, the sub-arc walls). Empty for a
    /// segment of one wall;
    /// `None` where the segment is held flat in the slab, its joints
    /// one point in both sections.
    pub joint_edges: Vec<Vec<Vec<Vec<Option<EdgeKey>>>>>,
    /// **The v-parameter each input section sits at** —
    /// [`LoftGeometry::section_params`](crate::skin::LoftGeometry::section_params) carried out to the caller
    /// (`[0] = 0`, `[k − 1] = 1`) instead of dropped with the
    /// geometry, so "what spacing did the skin choose for my
    /// sections?" is answered by the result rather than re-derived by
    /// hand. It is the SKIN's parameter: every skinned wall
    /// (`LoftGeometry.walls`) agrees on it, and section `i` is that
    /// wall's iso row `v = section_params[i]`. The body's walls carry
    /// it only where each wall spans every section (v-degree ≥ 2, or
    /// two sections). At v-degree 1 through `k ≥ 3` sections each body
    /// wall is one slab on its own `v ∈ [0, 1]` (`side_faces`), built
    /// from its two sections' rows, so no body wall holds these values
    /// as knots and none is built that does. They still say where
    /// along the stack the skin puts each section.
    ///
    /// This is a re-read of what the kernel chose, not a measurement
    /// — the produced surface IS the definition (DESIGN Q8), so no
    /// residual pad accompanies it. A [`loft_body`]'s is
    /// [`crate::loft_parameters`]' answer for its sections, askable
    /// BEFORE the body is built; a [`sweep_body`]'s is each station's
    /// normalised path parameter, `i / (stations − 1)`. Each is held in
    /// `T`, so at an enclosure scalar an interior parameter over a box
    /// reads as its bracket.
    pub section_params: Vec<T>,
}

impl<T: topo::AtRestPolicy> Lofted<T, Body<T>> {
    /// The built body as its scalar's leaf ([`topo::AtRestPolicy::leaf`]),
    /// every key kept: the one end every loft door ends in.
    ///
    /// # Errors
    ///
    /// [`LoftError::NotFinished`], every finding whole.
    pub(crate) fn leaf(self, tol: Tol) -> Result<Lofted<T>, LoftError> {
        let Self {
            body,
            solid,
            shell,
            top,
            bottom,
            side_faces,
            seam_edges,
            joint_edges,
            section_params,
        } = self;
        Ok(Lofted {
            body: T::leaf(body, tol).map_err(|findings| LoftError::NotFinished { findings })?,
            solid,
            shell,
            top,
            bottom,
            side_faces,
            seam_edges,
            joint_edges,
            section_params,
        })
    }
}

/// Typed refusal of the loft/sweep body assembly (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum LoftError {
    /// The run's tolerance could not form a classification band.
    Band(BandError),
    /// The lofted body is not a finished body
    /// ([`topo::AtRestPolicy::finish`] at the door's end): every finding,
    /// each read through its family.
    NotFinished {
        /// The finish's findings.
        findings: topo::NotFinished,
    },
    /// The §10.3/§10.4 geometry construction refused (compatibility,
    /// degree, interpolation — every [`SkinError`] reason).
    Skin(SkinError),
    /// An Euler operator or certified attach refused mid-assembly
    /// (D4 ¶2 reports surface inside
    /// [`EulerOpError::DescriptionUncertified`]).
    Euler(EulerOpError),
    /// The whole-body pcurve mint pass refused: a wall boundary's
    /// exact line-in-UV image failed its certification.
    Pcurve(PcurveMintError),
    /// The wall-boundary carrier extraction failed to re-wrap — a
    /// structurally corrupt skinned surface (unreachable from
    /// [`loft_geometry`](crate::loft_geometry) output; surfaced rather than swallowed).
    ///
    /// The payload is `geom_brep::boundary_iso_u`'s own refusal, which
    /// says WHICH structural invariant the extracted row broke; that
    /// door's `# Errors` section promises it is surfaced, and a
    /// discarded one would make a kernel-bug report say only that a
    /// kernel bug happened.
    SeamStructure {
        /// The iso-extraction refusal, carried rather than discarded.
        source: geom_core::spline::Built<SplineError>,
    },
    /// A section's loop/segment structure disagrees with the skinned
    /// geometry's, or two sections disagree with each other —
    /// unreachable when they all come from the same inputs; surfaced
    /// rather than swallowed.
    SectionStructure,
    /// Section `section` holds a value that is not finite for every
    /// value it stands for: poison, an infinity, or an enclosure
    /// reaching one.
    SectionNotFinite {
        /// The section.
        section: usize,
    },
    /// Section `section`'s placement has an entry that is not finite
    /// for every value it stands for: poison, an infinity, or an
    /// enclosure reaching one.
    PlacementNotFinite {
        /// The section whose placement it is.
        section: usize,
    },
    /// An interior section's rim found no image on the row `v = 1` of the
    /// slab wall below it (degree 1 through `k ≥ 3` sections only).
    ///
    /// `None`: no arc-rim image holds the rim. No input is known to
    /// reach this: the loft cuts every wall at each sub-arc joint any
    /// section puts in it, so a rim is one rational-quadratic span, at
    /// end weights the arc rim's image states, and the case that did
    /// reach it (sections whose arcs split into different numbers of
    /// sub-arcs) builds
    /// (`work/isotail/a-loft-whose-arc-bulge-varies-per-section-refuses-its-arc-rim.md`).
    /// It stays typed rather than assumed away.
    ///
    /// `Some`: the pick was undecided in band.
    SectionRim {
        /// The section.
        section: usize,
        /// The pick's escalation, where it was undecided.
        source: Option<Indeterminate>,
    },
    /// One SLAB definitely stacks AGAINST its own base section's plane
    /// normal. The canonical assembly orients caps and walls by the
    /// forward stacking (module docs) and does not guess: the named
    /// pair is where the stack turns back on itself.
    ReversedStacking {
        /// The slab — the pair [`SlabPair`] names — and the first one
        /// in section order that is not definitely forward.
        slab: usize,
    },
    /// One SLAB does not step along its base section's normal at
    /// tolerance, so the loft has no direction there. Two cases land
    /// here: coincidence, where every corner of every loop is pinned
    /// ([`Self::PinnedCorner`]'s predicate) and every segment is the same
    /// curve in both sections, at every scale down to bit-equal
    /// sections; and sections apart whose moving corners (the outer
    /// loop's not pinned in the slab, or every loop's where the outer
    /// loop is all pinned) have no mean displacement along the base
    /// normal — an in-plane pair. Sections that touch are
    /// [`Self::SectionsTouch`], read before this.
    DegenerateStacking {
        /// The slab — the pair [`SlabPair`] names.
        slab: usize,
    },
    /// A corner is pinned in one slab and moves in another, under a
    /// wall that spans both (v-degree 2 or more through three sections
    /// or more): the wall's boundary over that corner passes through
    /// one point twice, so it retraces itself and is not embedded.
    /// A corner is pinned where it is one world point in the slab's two
    /// sections (`loft_corner_pinned`); two adjacent pinned corners are
    /// a hinge, named as their segment.
    PinnedCorner {
        /// The loop, in canonical order (the outer first).
        loop_index: usize,
        /// The corner, or the segment hinged on two pinned corners.
        at: PinnedSite,
        /// The first slab that pins it — the pair [`SlabPair`] names.
        slab: usize,
        /// The first slab it moves in.
        moving: usize,
    },
    /// Whether a corner is the same world point in a slab's two
    /// sections escalated (`loft_corner_pinned`).
    CornerEscalated {
        /// The loop, in canonical order (the outer first).
        loop_index: usize,
        /// The corner: the start of segment `corner`.
        corner: usize,
        /// The slab — the pair [`SlabPair`] names.
        slab: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// Whether a sub-arc joint of a segment (a C⁰ knot its compatible
    /// strip carries, where the loft cuts the wall) is the same world
    /// point in a slab's two sections escalated (`loft_corner_pinned`,
    /// the corner's one predicate).
    JointEscalated {
        /// The loop, in canonical order (the outer first).
        loop_index: usize,
        /// The segment whose wall the joint cuts.
        segment: usize,
        /// The joint, counting from zero along the segment.
        joint: usize,
        /// The slab — the pair [`SlabPair`] names.
        slab: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// Two segments of a slab's two sections meet, a meeting the door
    /// verified: off every pinned corner they share, or anywhere where
    /// they share none. A hinge slid along its own edge, sections that
    /// cross, or two lines leaving a shared pinned corner along one ray. Both sections' loops lie on the
    /// body's boundary, so the boundary would meet itself there off its
    /// own edges.
    SectionsTouch {
        /// The slab — the pair [`SlabPair`] names.
        slab: usize,
        /// The segment of section `slab`, then the segment of section
        /// `slab + 1`.
        segments: [LoopSegment; 2],
        /// How many pinned corners the two share: none, one, or both
        /// ends (a lune on a pinned chord).
        shared: usize,
    },
    /// Whether two segments of a slab's two sections are apart, off any
    /// pinned corner they share, escalated: a pair of lines, whose
    /// readings converge, or a segment's two section curves read in band
    /// of one curve. A pair with an arc in it that the curve-pair reader
    /// cannot settle decides nothing here: the door refuses only contact
    /// it verified, and an unsettled arc pair is the embedding
    /// certificate's
    /// (`work/lathe/self-overlapping-spines-build-and-validate.md`).
    ClearanceUndecided {
        /// The slab — the pair [`SlabPair`] names.
        slab: usize,
        /// The segment of section `slab`, then the segment of section
        /// `slab + 1`.
        segments: [LoopSegment; 2],
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// The curve-pair reader's readings of two segments of a slab's two
    /// sections contradicted each other, or it misread a carrier's kind
    /// (`geom_brep::curve_pair::MeetFault`'s `Contradiction` and
    /// `Desync`): a kernel fault, which their definitions rule out but
    /// for rounding, and never a pose of the sections.
    ClearanceFault {
        /// The slab — the pair [`SlabPair`] names.
        slab: usize,
        /// The segment of section `slab`, then the segment of section
        /// `slab + 1`.
        segments: [LoopSegment; 2],
        /// The readings that disagreed.
        readings: &'static str,
    },
    /// One SLAB stacks along its base section's plane normal but not
    /// along its FAR section's: section `slab + 1`'s normal is
    /// definitely against the slab's displacement, or edge-on to it
    /// (Zero lands here, not in [`Self::DegenerateStacking`]: the base
    /// decide was already `Positive`, so the sections are apart).
    ///
    /// The check is conservative. It refuses every two-section loft
    /// whose rings fold between sections facing opposite ways along
    /// the stack, and it also refuses some embedded, correctly
    /// oriented lofts: a 10×10 square base under a 4×4 top turned 100°
    /// about `y` (its normal leaning back across the stack) is one. It
    /// retires when the loft door certifies embedding
    /// (`work/lathe/self-overlapping-spines-build-and-validate.md`).
    FarSectionNotForward {
        /// The slab — the pair [`SlabPair`] names; the far section is
        /// its second.
        slab: usize,
    },
    /// The far half of a SLAB's stacking decide escalated: section
    /// `slab + 1`'s plane normal against the slab's displacement is
    /// too close to call (named predicate on the diagnostic). The
    /// base half was already `Positive`.
    FarStackingEscalated {
        /// The slab — the pair [`SlabPair`] names; the far section is
        /// its second.
        slab: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// One SLAB's stacking classification against its base section's
    /// normal escalated (named predicate on the diagnostic).
    StackingEscalated {
        /// The slab — the pair [`SlabPair`] names.
        slab: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
}

/// **Where a pinned corner is**, within its loop: a corner alone, or the
/// segment between two adjacent pinned corners (a hinge).
///
/// **Not carried to the façade's curated list, by decision**
/// (`scripts/payload-rung-sweep.py` names this rung, and its disposition
/// table cites this paragraph). A Rust caller names and matches it
/// through `pncad::sweep`; what the prelude would add is the CUR3
/// matchability row over a new payload and a Python word for which
/// site refused, both the façade crate's to write. The falsifier is a
/// Python caller that must branch on a corner against a hinge rather
/// than read it off the sentence; when the façade carries it, delete
/// this paragraph and the disposition row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinnedSite {
    /// The corner that starts segment `j`.
    Corner(usize),
    /// Segment `j`, both of whose corners are pinned.
    Segment(usize),
    /// Sub-arc joint `joint` of segment `segment`: a C⁰ knot of the
    /// segment's compatible strip, where the loft cuts its wall.
    Joint {
        /// The segment.
        segment: usize,
        /// The joint, counting from zero along the segment.
        joint: usize,
    },
}

impl fmt::Display for PinnedSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Corner(j) => write!(f, "corner {j}"),
            Self::Segment(j) => write!(f, "segment {j} (both its corners)"),
            Self::Joint { segment, joint } => {
                write!(f, "sub-arc joint {joint} of segment {segment}")
            }
        }
    }
}

/// **A segment of a section**, by loop (canonical order, the outer
/// first) and segment within it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoopSegment {
    /// The loop.
    pub loop_index: usize,
    /// The segment.
    pub segment: usize,
}

impl fmt::Display for LoopSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "segment {} of loop {}", self.segment, self.loop_index)
    }
}

/// **The two sections a slab spans**, and the one place this crate
/// spells that arithmetic: slab `k` is the pair `(k, k + 1)`, and
/// every stacking refusal names its pair through this.
struct SlabPair(usize);

impl SlabPair {
    /// The slab's far section.
    fn far(&self) -> usize {
        self.0 + 1
    }
}

impl fmt::Display for SlabPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sections {} and {}", self.0, self.far())
    }
}

impl LoftError {
    fn write(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        at: Option<geom_brep::recourse::ReadAt>,
    ) -> core::fmt::Result {
        match self {
            Self::Band(e) => write!(f, "{e}"),
            Self::NotFinished { findings } => {
                write!(f, "the lofted body is not a finished body: {findings}")
            }
            Self::Skin(e) => write!(f, "{e}"),
            Self::Euler(e) => write!(
                f,
                "an Euler operation of the assembly refused: {}",
                geom_brep::recourse::read_at(e, at)
            ),
            Self::Pcurve(e) => write!(f, "{}", geom_brep::recourse::read_at(e, at)),
            Self::SeamStructure { source } => write!(
                f,
                "a wall's boundary curve failed to re-wrap: {source}. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::SectionStructure => write!(
                f,
                "a section's loop or segment structure disagrees with the skinned \
                 geometry or with another section (kernel bug, not an input fault)"
            ),
            Self::SectionRim {
                section,
                source: None,
            } => write!(
                f,
                "loft section {section}'s arc rim has no image on the degree-1 wall below it, \
                 though the wall is one rational-quadratic span along it. Recourse: loft at \
                 v-degree 2 or more, whose walls span every section, so the section is no rim"
            ),
            Self::SectionRim {
                section,
                source: Some(source),
            } => write!(
                f,
                "{}",
                source.undecided(
                    format_args!(
                        "which row of the wall below loft section {section} holds its rim"
                    ),
                    source.ending(geom_core::COINCIDENCE_RECOURSE),
                )
            ),
            Self::SectionNotFinite { section } => write!(
                f,
                "section {section} holds a value that is not finite, so it has no locus to \
                 loft. Recourse: give the section finite values"
            ),
            Self::PlacementNotFinite { section } => write!(
                f,
                "section {section}'s placement is not finite, so the section has no place in \
                 space. Recourse: give the placement finite values"
            ),
            Self::ReversedStacking { slab } => write!(
                f,
                "loft {} stack AGAINST section {slab}'s plane normal, and a loft does not \
                 guess its direction. Recourse: reorder the sections so they stack \
                 forward; this is the first reversed pair, and a wholly reversed list \
                 lofts the same solid once reversed",
                SlabPair(*slab)
            ),
            Self::DegenerateStacking { slab } => write!(
                f,
                "loft {} do not step along section {slab}'s normal at tolerance (they \
                 coincide, or the corners that move between them have no mean step along \
                 it), \
                 so the loft has no direction. Recourse: move section {far} along section \
                 {slab}'s normal",
                SlabPair(*slab),
                far = SlabPair(*slab).far()
            ),
            Self::PinnedCorner {
                loop_index,
                at,
                slab,
                moving,
            } => write!(
                f,
                "loft {} hold {at} of loop {loop_index} at one point, and {} do not, so a \
                 wall through every section passes that point twice and retraces itself. \
                 Recourse: move it apart between sections {slab} and {far}, or loft at \
                 v-degree 1",
                SlabPair(*slab),
                SlabPair(*moving),
                far = SlabPair(*slab).far()
            ),
            Self::CornerEscalated {
                loop_index,
                corner,
                slab,
                source,
            } => write!(
                f,
                "{}",
                source.undecided(
                    format_args!(
                        "whether loft {} hold corner {corner} of loop {loop_index} at one point",
                        SlabPair(*slab)
                    ),
                    source.ending(geom_core::COINCIDENCE_RECOURSE),
                )
            ),
            Self::JointEscalated {
                loop_index,
                segment,
                joint,
                slab,
                source,
            } => write!(
                f,
                "{}",
                source.undecided(
                    format_args!(
                        "whether loft {} hold sub-arc joint {joint} of segment {segment} of loop \
                         {loop_index} at one point",
                        SlabPair(*slab)
                    ),
                    source.ending(geom_core::COINCIDENCE_RECOURSE),
                )
            ),
            Self::SectionsTouch {
                slab,
                segments,
                shared,
            } => write!(
                f,
                "loft section {slab}'s {} meets section {far}'s {}{}, so the body's \
                 boundary would meet itself there. Recourse: move section {far} clear of \
                 section {slab}",
                segments[0],
                segments[1],
                match shared {
                    0 => "",
                    1 => " off the pinned corner they share",
                    _ => " off the pinned corners they share",
                },
                far = SlabPair(*slab).far()
            ),
            Self::ClearanceUndecided {
                slab,
                segments,
                source,
            } => write!(
                f,
                "{}",
                source.undecided(
                    format_args!(
                        "whether loft section {slab}'s {} and section {far}'s {} are apart",
                        segments[0],
                        segments[1],
                        far = SlabPair(*slab).far()
                    ),
                    source.ending(geom_core::COINCIDENCE_RECOURSE),
                )
            ),
            Self::ClearanceFault {
                slab,
                segments,
                readings,
            } => write!(
                f,
                "the kernel's readings of where loft section {slab}'s {} and section {far}'s \
                 {} meet contradict each other ({readings}), which their definitions rule \
                 out: a kernel fault, not a fault in the sections. Recourse: report it with \
                 the sections; moving section {far} slightly may avoid it",
                segments[0],
                segments[1],
                far = SlabPair(*slab).far()
            ),
            Self::FarSectionNotForward { slab } => write!(
                f,
                "loft section {far}'s plane does not face along the stack from section \
                 {slab}. Recourse: author section {far} on a plane facing along the stack \
                 with the others",
                far = SlabPair(*slab).far()
            ),
            Self::FarStackingEscalated { slab, source } => write!(
                f,
                "{}",
                source.undecided(
                    format_args!(
                        "whether loft section {far}'s plane faces along the stack from \
                         section {slab}",
                        far = SlabPair(*slab).far()
                    ),
                    source.ending(geom_core::COINCIDENCE_RECOURSE),
                )
            ),
            Self::StackingEscalated { slab, source } => write!(
                f,
                "{}",
                source.undecided(
                    format_args!("whether loft {} stack forward", SlabPair(*slab)),
                    source.ending(geom_core::COINCIDENCE_RECOURSE),
                )
            ),
        }
    }
}

geom_brep::ended_by_write!(LoftError);

impl std::error::Error for LoftError {}

impl From<EulerOpError> for LoftError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e.from_driver())
    }
}

/// One loop's compatible strips as the stacking door reads them
/// ([`stacking::Strips`]): each section curve's control points.
fn strip_points<T: Real>(strips: &[Vec<geom::NurbsCurve3<T>>]) -> Vec<Vec<Vec<Point3<T>>>> {
    strips
        .iter()
        .map(|strip| strip.iter().map(|c| c.control().to_vec()).collect())
        .collect()
}

/// Each compatible strip's sub-arc joints as the stacking door reads
/// them ([`stacking::Joints`]): the control index of every C⁰ knot of
/// the strip's knot vector, which its curves share
/// ([`crate::skin::crease_columns`]).
///
/// # Errors
///
/// [`LoftError::SectionStructure`] for a strip with no curve, or a knot
/// vector the skin's continuous sections cannot have.
fn strip_joints<S: geom_core::spline::SpanLocate>(
    strips: &[Vec<Vec<geom::NurbsCurve3<S>>>],
) -> Result<stacking::Joints, LoftError> {
    strips
        .iter()
        .map(|lp| {
            lp.iter()
                .map(|strip| {
                    let first = strip.first().ok_or(LoftError::SectionStructure)?;
                    crate::skin::crease_columns(first.knot_values())
                        .map(|joints| joints.into_iter().map(|(_, c)| c).collect())
                        .ok_or(LoftError::SectionStructure)
                })
                .collect()
        })
        .collect()
}

/// The world point of a sketch-plane point under a lifted placement.
fn world<T: Real>(place: &Affine3<T>, p: geom_core::Point2<T>) -> Point3<T> {
    place.transform_point(Point3::new(p.x, p.y, T::zero()))
}

/// The shared engine of [`loft_body`], [`sweep_body`] and
/// [`loft_profiles`]: refuse the counts, then a section or placement
/// with no finite value, then run the half of the projective partition `T` is on
/// ([`SpanLocate::split_projective`](geom_core::spline::SpanLocate::split_projective)),
/// which [`LoftBuild`]'s two halves state.
///
/// Each half runs the stacking door ([`stacking::stacking_door`])
/// BEFORE the skin, because it decides, under the run's tolerance,
/// whether two adjacent sections are apart and which corners they pin;
/// a pair it refuses (down to exact coincidence) never reaches the
/// skin's parameterization.
fn build<T: Decide + topo::AtRestPolicy, L: SectionLoop<T>>(
    sections: SectionsAtT<'_, T, L>,
    places: &[SketchPlane<T>],
    channel_places: ChannelPlaces<'_>,
    params: Option<&[f64]>,
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T, Body<T>>, LoftError> {
    check_counts(sections.len(), places.len(), v_degree).map_err(LoftError::Skin)?;
    if let Some(section) = places
        .iter()
        .position(|p| !p.placement().components().iter().all(|c| c.is_bounded()))
    {
        return Err(LoftError::PlacementNotFinite { section });
    }
    if let Some(section) = sections.first_not_finite() {
        return Err(LoftError::SectionNotFinite { section });
    }
    T::split_projective(LoftBuild {
        sections,
        places,
        channel_places,
        params,
        v_degree,
        tol,
    })
}

/// Where a loft's sections at `T` come from.
enum SectionsAtT<'a, T: Real, L> {
    /// Loops: [`loft_body`]'s and [`sweep_body`]'s.
    Loops(&'a [Section<L>]),
    /// Canonical forms the caller validated at `T`
    /// ([`loft_profiles`]).
    Validated(&'a [ValidatedProfile<T>]),
}

impl<T: Decide, L: SectionLoop<T>> SectionsAtT<'_, T, L> {
    fn len(&self) -> usize {
        match self {
            Self::Loops(s) => s.len(),
            Self::Validated(s) => s.len(),
        }
    }

    /// The first section holding a value that is not finite for every
    /// value it stands for ([`Real::is_bounded`]): poison, an
    /// infinity, or an enclosure reaching one. A value wider than a
    /// point is finite.
    fn first_not_finite(&self) -> Option<usize> {
        match self {
            Self::Loops(sections) => sections.iter().position(|loops| {
                !loops
                    .iter()
                    .all(|lp| loop_all(lp.as_profile_loop(), T::is_bounded))
            }),
            Self::Validated(sections) => {
                sections.iter().position(|p| !profile_all(p, T::is_bounded))
            }
        }
    }
}

/// The placements' `f64` value channel, `None` where one has no point
/// value: computed where it is read, by the projective arm selecting a
/// loop section's structure, and nowhere else.
type ChannelPlaces<'a> = &'a dyn Fn() -> Result<Option<Vec<SketchPlane<f64>>>, LoftError>;

/// The two halves of [`fn@build`], after the counts and the finite
/// check.
struct LoftBuild<'a, T: Real, L> {
    sections: SectionsAtT<'a, T, L>,
    places: &'a [SketchPlane<T>],
    channel_places: ChannelPlaces<'a>,
    /// The v each section sits at where the caller states it (a path
    /// sweep's path parameters); otherwise the loft's chord-length rule.
    params: Option<&'a [f64]>,
    v_degree: usize,
    tol: Tol,
}

impl<T: Decide + topo::AtRestPolicy, L: SectionLoop<T>> LoftBuild<'_, T, L> {
    /// **The loft in `T`**, over `canonical`, the sections validated at
    /// `T`: the stacking door, the walls ([`body_walls`]: the section
    /// curves, their compatibility, the parameters and the solve) and the caps
    /// all read those, so a section or placement value that is a
    /// function of the variables gives walls and caps that are
    /// functions of them, and one wider than a point gives walls and
    /// caps that enclose its family.
    fn in_t(&self, canonical: Vec<ValidatedProfile<T>>) -> Result<Lofted<T, Body<T>>, LoftError> {
        let (walls, section_params, pinned) = loft_walls(
            self.places,
            &canonical,
            self.v_degree,
            self.params,
            self.tol,
        )?;
        assemble(
            self.places,
            walls,
            &canonical,
            section_params,
            &pinned,
            self.tol,
        )
    }
}

/// A loft's walls `[loop][segment][slab]`, the v each section sits at
/// and the stacking door's verdict, over sections validated at `T`: the
/// compatible strips and their sub-arc joints ([`strip_joints`]), the
/// door over them, then the walls ([`body_walls`]) with the corners and
/// joints the door pinned.
#[allow(clippy::type_complexity)]
fn loft_walls<T: Decide + topo::AtRestPolicy>(
    places: &[SketchPlane<T>],
    canonical: &[ValidatedProfile<T>],
    v_degree: usize,
    shares: Option<&[f64]>,
    tol: Tol,
) -> Result<
    (
        Vec<Vec<Vec<Arc<NurbsSurface<T>>>>>,
        Vec<T>,
        stacking::Pinned,
    ),
    LoftError,
> {
    let band = Band::linear(tol).map_err(LoftError::Band)?;
    let compat = compatible_strips(canonical, places).map_err(LoftError::Skin)?;
    let joints = strip_joints(&compat)?;
    let strips = compat.iter().map(|lp| strip_points(lp)).collect();
    let pinned = stacking::stacking_door(places, canonical, &strips, &joints, v_degree, band, tol)?;
    let (walls, section_params) = body_walls(
        compat,
        v_degree,
        shares,
        &|l, j, s| pinned.at(l, j, s),
        &|l, j, i, s| pinned.joint_at(l, j, i, s),
        &joints,
    )
    .map_err(LoftError::Skin)?;
    Ok((walls, section_params, pinned))
}

impl<T: Decide + topo::AtRestPolicy, L: SectionLoop<T>> ProjectiveSplit<T> for LoftBuild<'_, T, L> {
    type Out = Result<Lofted<T, Body<T>>, LoftError>;

    /// **At a projective scalar** every finite value is a point, so the
    /// structure is selected on the sections' value channel at `f64`
    /// ([`validate_loft`]), each section of loops is validated at `T`
    /// along its `f64` record ([`validate_guided`]) or arrives
    /// validated ([`loft_profiles`]), and the loft is built in `T`
    /// ([`Self::in_t`]).
    fn projective(self) -> Self::Out
    where
        T: ProjectiveScalar,
    {
        let canonical = match self.sections {
            SectionsAtT::Loops(sections) => {
                // Every finite value at a projective scalar is a point,
                // and `build` refused the placements that are not finite.
                let channel_places = (self.channel_places)()?.unwrap_or_else(|| {
                    unreachable!("a finite placement at a projective scalar has a point value")
                });
                let channel: Vec<Section<L::Channel>> = sections
                    .iter()
                    .map(|s| s.iter().map(SectionLoop::channel).collect())
                    .collect();
                let (_, structures) =
                    validate_loft(&channel, &channel_places, self.v_degree, self.tol)
                        .map_err(LoftError::Skin)?;
                validate_guided(sections, self.places, &structures, self.tol)
                    .map_err(LoftError::Skin)?
            }
            SectionsAtT::Validated(sections) => {
                check_shapes(sections).map_err(LoftError::Skin)?;
                sections.to_vec()
            }
        };
        self.in_t(canonical)
    }

    /// **At an enclosure scalar** the body encloses the family its
    /// sections stand for: the loft is built in `T` ([`Self::in_t`]),
    /// each section of loops validated at `T` with its decisions
    /// certified there, a family of one included. The skin's structure
    /// is what every value of the family shares, and where it is not (an
    /// arc, whose weights are no points; an interior knot wider than a
    /// point) the skin refuses typed ([`SkinError`]).
    fn enclosure(self) -> Self::Out {
        let canonical = match self.sections {
            SectionsAtT::Loops(sections) => {
                validate_at_t(sections, self.places, self.tol).map_err(LoftError::Skin)?
            }
            SectionsAtT::Validated(sections) => sections.to_vec(),
        };
        check_shapes(&canonical).map_err(LoftError::Skin)?;
        self.in_t(canonical)
    }
}

/// **A wall cut at its creases in `u`**: one wall per stretch between
/// consecutive C⁰ knots of `wall`'s `u` ([`geom_core::spline::Knots::crease_values`]),
/// each on its own `u ∈ [0, 1]`, with the crease values the cuts fall
/// at (ascending, the domain's ends left out). The skin writes an arc
/// wider than the sub-arc ceiling as sub-arcs joined at a double knot
/// ([`crate::skin::segment_curve`]), and the compatible net carries
/// every section's joints in every row, so a segment's wall is C⁰ at
/// the union of its sections' joints; a face chart is C¹ by its knot
/// structure strictly inside its cell (DESIGN `[topology]`), so the loft
/// cuts there. A wall with no crease is itself, uncut.
///
/// # Errors
///
/// [`LoftError::SectionStructure`] for an interior run of the knot
/// vector that would leave the net discontinuous, which no skin of
/// continuous sections has; [`LoftError::SeamStructure`] where the
/// spline door refuses a piece.
#[allow(clippy::type_complexity)]
fn sub_arc_walls<T: geom_core::spline::SpanLocate>(
    wall: &Arc<NurbsSurface<T>>,
) -> Result<(Vec<Arc<NurbsSurface<T>>>, Vec<f64>), LoftError> {
    let knots = wall.knot_values_u();
    let creases: Vec<usize> = knots.structure().crease_runs().collect();
    if creases.is_empty() {
        return Ok((vec![Arc::clone(wall)], Vec::new()));
    }
    let Some(joints) = crate::skin::crease_columns(knots) else {
        return Err(LoftError::SectionStructure);
    };
    let p = knots.degree();
    let runs: Vec<(T, usize)> = knots.runs().collect();
    let last = runs.len() - 1;
    let (nu, nv) = wall.control_counts();
    // The control column the net interpolates at run `r`.
    let column = |r: usize| match r {
        0 => 0,
        r if r == last => nu - 1,
        r => {
            joints[creases.iter().position(|&c| c == r).unwrap_or_else(|| {
                unreachable!("run {r} bounds a piece only where it is a crease")
            })]
            .1
        }
    };
    let seam = |e| LoftError::SeamStructure {
        source: geom_core::spline::Built::new(e),
    };
    let clamp = core::num::NonZeroUsize::MIN.saturating_add(p);
    let bounds: Vec<usize> = core::iter::once(0)
        .chain(creases.iter().copied())
        .chain(core::iter::once(last))
        .collect();
    let walls = bounds
        .windows(2)
        .map(|w| {
            let (a, b) = (w[0], w[1]);
            let (ua, ub) = (runs[a].0, runs[b].0);
            let mut piece = vec![(T::zero(), clamp)];
            for &(v, m) in &runs[a + 1..b] {
                piece.push((
                    (v - ua) / (ub - ua),
                    core::num::NonZeroUsize::MIN.saturating_add(m - 1),
                ));
            }
            piece.push((T::one(), clamp));
            let knots_u = geom_core::spline::Knots::new(piece, p).map_err(seam)?;
            let at = (column(a)..=column(b)).flat_map(|iu| (0..nv).map(move |iv| iu * nv + iv));
            let control = at.clone().map(|i| wall.control()[i]).collect();
            let weights = at.map(|i| wall.weights()[i]).collect();
            NurbsSurface::new(knots_u, wall.knot_values_v().clone(), control, weights)
                .map(Arc::new)
                .map_err(seam)
        })
        .collect::<Result<_, _>>()?;
    Ok((walls, joints.iter().map(|&(u, _)| u).collect()))
}

/// **A section's segment cut where its wall is** ([`sub_arc_walls`]):
/// one traversal per sub-arc wall, the first from the segment's start
/// and the last to its end, each cut at the point the section's own
/// curve ([`crate::skin::segment_curve`]) reaches at that crease. A
/// line's curve is affine in `u`, so its cut is `a + u·(b − a)`. An
/// arc's own sub-arc joint `k/m` is `Arc2::point_from` at `k/m`, the
/// on-arc control point itself; a crease another section put inside
/// one of its sub-arcs falls at that sub-arc's rational-quadratic
/// parameter `s`, located on the arc rim's own sub-arc breaks
/// ([`geom_brep::sub_arc_at`]), whose angle from the sub-arc's middle is
/// the rational-quadratic identity the arc rim's map inverts
/// ([`geom_brep::sub_arc_angle_at_fraction`]).
///
/// # Errors
///
/// [`LoftError::SectionStructure`] where the arc's sweep has no bracket
/// or a crease lies off `[0, 1]`: the skin admitted both before the
/// arc reached a wall.
fn segment_pieces<T: geom_core::spline::SpanLocate>(
    seg: SweptSeg<T>,
    cuts: &[f64],
) -> Result<Vec<SweptSeg<T>>, LoftError> {
    if cuts.is_empty() {
        return Ok(vec![seg]);
    }
    let piece = |a: Point2<T>, b: Point2<T>, kind| SweptSeg { a, b, kind, ..seg };
    match seg.kind.get() {
        profile::SegmentKind::Line { .. } => {
            let at = |u: f64| seg.a + (seg.b - seg.a) * T::from_computed(u);
            let points: Vec<Point2<T>> = core::iter::once(seg.a)
                .chain(cuts.iter().map(|&u| at(u)))
                .chain(core::iter::once(seg.b))
                .collect();
            Ok(points
                .windows(2)
                .map(|w| piece(w[0], w[1], seg.kind))
                .collect())
        }
        profile::SegmentKind::Arc {
            arc,
            radials: [start_radial, end_radial],
            ..
        } => {
            // The skin decided the sub-arc count over the sweep's bracket
            // (`segment_curve`) before this arc reached a wall, so a
            // sweep with no bracket here is the two disagreeing.
            let (lo, _) = arc
                .sweep
                .abs()
                .structure_bracket()
                .ok_or(LoftError::SectionStructure)?;
            let m = crate::skin::sub_arc_count(lo);
            #[allow(clippy::cast_precision_loss)]
            let steps = T::from_f64(m as f64);
            let delta = arc.sweep / steps;
            let tan_q = geom_brep::sub_arc_tan_quarter(delta);
            let fraction = |u: f64| -> Result<T, LoftError> {
                let (k, s) = geom_brep::sub_arc_at(u, m).ok_or(LoftError::SectionStructure)?;
                // The arc's own joint is the on-arc control point itself.
                if s == 0.0 {
                    return Ok(T::from_f64(geom_brep::sub_arc_joint(k, m)));
                }
                let from_middle = geom_brep::sub_arc_angle_at_fraction(T::from_computed(s), tan_q);
                #[allow(clippy::cast_precision_loss)]
                let into = T::from_f64(k as f64 + 0.5) + from_middle / delta;
                Ok(into / steps)
            };
            let fractions: Vec<T> = core::iter::once(Ok(T::zero()))
                .chain(cuts.iter().map(|&u| fraction(u)))
                .chain(core::iter::once(Ok(T::one())))
                .collect::<Result<_, _>>()?;
            let last = fractions.len() - 2;
            // The unit radial at a cut: the start radial turned by the
            // cut's angle, the rotation `point_from` applies about a
            // unit circle at the origin, so nothing divides.
            let unit = geom_core::Arc2 {
                centre: Point2::new(T::zero(), T::zero()),
                radius: T::one(),
                ..arc
            };
            let radial_at = |f: T| {
                unit.point_from(Point2::new(start_radial.x, start_radial.y), f)
                    - Point2::new(T::zero(), T::zero())
            };
            Ok(fractions
                .windows(2)
                .enumerate()
                .map(|(i, f)| {
                    let (a, ra) = if i == 0 {
                        (seg.a, start_radial)
                    } else {
                        (arc.point_from(seg.a, f[0]), radial_at(f[0]))
                    };
                    let (b, rb) = if i == last {
                        (seg.b, end_radial)
                    } else {
                        (arc.point_from(seg.a, f[1]), radial_at(f[1]))
                    };
                    piece(a, b, seg.kind.part(arc.sweep * (f[1] - f[0]), [ra, rb]))
                })
                .collect())
        }
    }
}

/// Re-describes the rim `edge` an interior section `section` laid, the
/// row `v = 1` of the slab wall `wall` below it (on the chart `chart`),
/// as that row's image: a line along the row's own parameter, which is
/// linear in a placed line's, or a circle through the row's
/// rational-quadratic parameter (`geom_brep::pick_arc_rim`, the mint's
/// own pick). Its carrier is the placed sketch segment, kept verbatim,
/// and the sketch segment that declared it stays its authority.
///
/// # Errors
///
/// [`LoftError::SectionRim`] where no arc-rim image holds the circle or
/// the pick is undecided; the certified setter's refusals.
fn describe_section_rim<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    edge: EdgeKey,
    chart: topo::SurfaceKey,
    wall: &Arc<NurbsSurface<T>>,
    section: usize,
    band: Band,
    tol: Tol,
) -> Result<(), LoftError> {
    let Some(curve) = body.edge(edge).and_then(topo::EdgeView::certified) else {
        unreachable!(
            "an interior section's rim {edge:?} is a certified edge the loft minted and still holds"
        )
    };
    let spec = curve.restated_spec();
    let EdgeDescriptionSpec::Scaffold(declared) = spec.description else {
        unreachable!(
            "an interior section's rim is minted through the scaffolding door, and only this \
             pass describes it"
        )
    };
    let (t0, t1) = (spec.param_start, spec.param_end);
    let image = match spec.carrier {
        Curve3::Circle(..) => geom_brep::pick_arc_rim(
            &spec.carrier,
            (t0, t1 - t0),
            &Surface::Nurbs(Arc::clone(wall)),
            band,
        )
        .map_err(|source| LoftError::SectionRim {
            section,
            source: Some(source),
        })?
        .ok_or(LoftError::SectionRim {
            section,
            source: None,
        })?,
        _ => geom_brep::Pcurve::iso_row(T::one(), (T::zero(), T::one()), (t0, t1)),
    };
    body.restate_stored_edge(
        edge,
        EdgeDescriptionSpec::chart_image(chart, image).declared_by(declared),
        tol,
    )?;
    Ok(())
}

/// **A hinge's edge at rest**, between the two caps that hold it: the
/// description its wedge demands ([`topo::edge_wedge_description`]), as
/// extrude's cap rims rest. The wedge is the caps' dihedral about the
/// hinge, the angle the stack turns about it, so one in band is the
/// stack's step escalating ([`LoftError::StackingEscalated`]); the door
/// has read that step for every slab already, so no hinge built at rest
/// reaches it.
fn hinge_at_rest<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    edge: EdgeKey,
    band: Band,
    tol: Tol,
) -> Result<(), LoftError> {
    let demanded = topo::edge_wedge_description(body, edge, band).unwrap_or_else(|| {
        unreachable!("hinge {edge:?} was minted by this loft with a certified carrier")
    });
    match demanded {
        Ok(geom_brep::MustCarryDescription::Intrinsic(description)) => {
            body.restate_stored_edge(edge, description, tol)?;
        }
        Ok(geom_brep::MustCarryDescription::Conventional) => body.describe_at_rest(edge, tol)?,
        Err(geom_brep::WedgeRefusal::InBand(escalation)) => {
            return Err(LoftError::StackingEscalated {
                slab: 0,
                source: escalation.diag(),
            });
        }
        Err(geom_brep::WedgeRefusal::Mixed(mixed)) => unreachable!(
            "a hinge is a line between two planes, whose dihedral is one at every station: {mixed}"
        ),
    }
    Ok(())
}

/// Assembles the loft BODY from its skinned geometry (module docs),
/// over sections the stacking door has already accepted.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum.
fn assemble<T: Decide + topo::AtRestPolicy>(
    places: &[SketchPlane<T>],
    walls: Vec<Vec<Vec<Arc<NurbsSurface<T>>>>>,
    canonical: &[ValidatedProfile<T>],
    section_params: Vec<T>,
    door: &stacking::Pinned,
    tol: Tol,
) -> Result<Lofted<T, Body<T>>, LoftError> {
    let band = Band::linear(tol).map_err(LoftError::Band)?;
    let (Some(place_bottom), Some(place_top)) = (places.first(), places.last()) else {
        return Err(LoftError::SectionStructure);
    };

    // ---- The walls: one per segment per slab ([`body_walls`]), each
    // cut at its creases into one per sub-arc ([`sub_arc_walls`]). A
    // segment's slabs share its compatible strip's `u` knots, so they cut
    // alike. Pieces are numbered along each loop, every segment's
    // sub-arcs in turn; `owner[l][q]` is piece `q`'s (segment, sub-arc).
    // Slab boundary `b ∈ 0 ..= slabs` sits at section `at(b)`. ----
    let k = canonical.len();
    let segment_walls = walls;
    let mut walls: Vec<Vec<Vec<Arc<NurbsSurface<T>>>>> = Vec::with_capacity(segment_walls.len());
    let mut owner: Vec<Vec<(usize, usize)>> = Vec::with_capacity(segment_walls.len());
    let mut cuts: Vec<Vec<Vec<f64>>> = Vec::with_capacity(segment_walls.len());
    for lp in &segment_walls {
        let (mut w_l, mut o_l, mut c_l) = (Vec::new(), Vec::new(), Vec::new());
        for (j, slabs_of) in lp.iter().enumerate() {
            let cut: Vec<(Vec<Arc<NurbsSurface<T>>>, Vec<f64>)> = slabs_of
                .iter()
                .map(sub_arc_walls)
                .collect::<Result<_, _>>()?;
            let Some((first, c)) = cut.first() else {
                return Err(LoftError::SectionStructure);
            };
            if cut
                .iter()
                .any(|(subs, cs)| subs.len() != first.len() || cs != c)
            {
                return Err(LoftError::SectionStructure);
            }
            for q in 0..first.len() {
                w_l.push(cut.iter().map(|(subs, _)| Arc::clone(&subs[q])).collect());
                o_l.push((j, q));
            }
            c_l.push(c.clone());
        }
        walls.push(w_l);
        owner.push(o_l);
        cuts.push(c_l);
    }
    let slabs = walls.first().and_then(|lp| lp.first()).map_or(1, Vec::len);
    let at = |b: usize| if slabs == 1 { b * (k - 1) } else { b };
    // The door's classes over each body slab's sections: a wall through
    // every section is one slab over all of them, where the door has
    // refused a corner or segment pinned in some and not others.
    let over = |s: usize| if slabs == 1 { 0..k - 1 } else { s..s + 1 };
    // The door classes the profile's corners, segments and sub-arc
    // joints. A piece is its segment's: held flat where its segment is,
    // and its start pinned where that is its segment's corner or a joint
    // the door pinned, a corner of the two walls the joint parts.
    let flat = |li: usize, q: usize, s: usize| {
        let (j, _) = owner[li][q];
        door.flat_through(li, j, over(s))
    };
    let pinned_at = |li: usize, q: usize, s: usize| match owner[li][q] {
        (j, 0) => door.at_through(li, j, over(s)),
        (j, sub) => door.joint_through(li, j, sub - 1, over(s)),
    };

    // ---- Traversals and world points, per section, per loop, per
    // piece ([`segment_pieces`]). ----
    let loops_at: Vec<Vec<Vec<SweptSeg<T>>>> = canonical
        .iter()
        .map(|c| {
            c.loops()
                .iter()
                .zip(&cuts)
                .map(|(lp, cuts)| {
                    swept_segments(lp, false)
                        .into_iter()
                        .zip(cuts)
                        .map(|(seg, c)| segment_pieces(seg, c))
                        .collect::<Result<Vec<_>, _>>()
                        .map(|pieces| pieces.into_iter().flatten().collect())
                })
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;
    if places.len() < 2
        || places.len() != k
        || canonical.iter().any(|c| {
            c.loops().len() != cuts.len()
                || c.loops()
                    .iter()
                    .zip(&cuts)
                    .any(|(lp, c)| lp.segments().len() != c.len())
        })
        || loops_at.iter().any(|ls| {
            ls.len() != walls.len() || ls.iter().zip(&walls).any(|(segs, w)| segs.len() != w.len())
        })
        || walls.iter().flatten().any(|w| w.len() != slabs)
    {
        return Err(LoftError::SectionStructure);
    }
    let q_at: Vec<Vec<Vec<Point3<T>>>> = loops_at
        .iter()
        .zip(places)
        .map(|(ls, p)| {
            let place = p.placement();
            ls.iter()
                .map(|segs| segs.iter().map(|s| world(&place, s.a)).collect())
                .collect()
        })
        .collect();
    let bloops = &loops_at[0];
    let bq = &q_at[0];
    // Loop `li`'s segment `j` placed at section `i`, from `q_from`.
    let rim_spec = |i: usize, li: usize, j: usize, q_from| {
        placed_segment_spec(&loops_at[i][li][j], places[i].frame(), q_from, tol)
    };
    let wall_surface = |li: usize, j: usize, s: usize| FaceSurface::New {
        surface: Surface::Nurbs(Arc::clone(&walls[li][j][s])),
        // The skinned chart's normal points out of the material
        // (module docs).
        sense: true,
    };

    // ---- Phase 1: bottom lamina (the extrude shape: the seed face's
    // chain is minted at the BOTTOM vertices and survives as the TOP
    // cap after the sweep; the mef'd close face is the bottom cap,
    // its loop reversed so the outward normal opposes stacking). ----
    let outer = &bloops[0];
    let qs = &bq[0];
    let n = outer.len();
    // One surgery scope for the whole assembly: the tier-1
    // postcondition is this door's, paid once over the finished body
    // (`topo::surgery`), and the tier-2 check below subsumes it.
    let mut built = Body::<T>::new();
    let mut body = built.begin_surgery();
    let bottom_cap = FaceSurface::New {
        surface: cap_plane(place_bottom.frame(), false, CapEnd::Start),
        sense: true,
    };
    let mut bases: Vec<Vec<topo::HalfEdgeKey>> = Vec::with_capacity(bloops.len());
    let seed = body.mvfs(qs[0], true)?;
    let mut hes = Vec::with_capacity(n);
    let first = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        qs[1],
        rim_spec(0, 0, 0, qs[0]),
        tol,
    )?;
    hes.push(first.he_plus);
    let mut prev = first;
    for j in 2..n {
        let m = body.mev(
            MevSite::Fan {
                he1: prev.he_minus,
                he2: prev.he_minus,
            },
            qs[j],
            rim_spec(0, 0, j - 1, qs[j - 1]),
            tol,
        )?;
        hes.push(m.he_plus);
        prev = m;
    }
    let close = body.mef(
        MefSite::Chords {
            he1: prev.he_minus,
            he2: first.he_plus,
        },
        rim_spec(0, 0, n - 1, qs[n - 1]),
        bottom_cap,
        tol,
    )?;
    hes.push(close.he_plus);
    let (bottom_face, anchor) = (close.face, hes[0]);
    bases.push(hes);
    let top_face = seed.face;
    let bottom_surface = face_surface_key(&body, bottom_face);

    // ---- Phase 2: holes (rings in the seed face + kfmrh into the
    // bottom cap) — extrude's phase verbatim, loft rim specs. ----
    for (li, segs) in bloops.iter().enumerate().skip(1) {
        let hq = &bq[li];
        let m = segs.len();
        let (ring, _) = crate::swept::plant_hole_ring(&mut body, anchor, hq[0], tol)?;
        let mut hole_hes = Vec::with_capacity(m);
        let first = body.mev(
            MevSite::Lone { r#loop: ring },
            hq[1],
            rim_spec(0, li, 0, hq[0]),
            tol,
        )?;
        hole_hes.push(first.he_plus);
        let mut prev = first;
        for j in 2..m {
            let mv = body.mev(
                MevSite::Fan {
                    he1: prev.he_minus,
                    he2: prev.he_minus,
                },
                hq[j],
                rim_spec(0, li, j - 1, hq[j - 1]),
                tol,
            )?;
            hole_hes.push(mv.he_plus);
            prev = mv;
        }
        let close = body.mef(
            MefSite::Chords {
                he1: prev.he_minus,
                he2: first.he_plus,
            },
            rim_spec(0, li, m - 1, hq[m - 1]),
            crate::swept::transient_disc(bottom_surface),
            tol,
        )?;
        hole_hes.push(close.he_plus);
        body.kfmrh(bottom_face, close.face)?;
        bases.push(hole_hes);
    }

    // ---- Phases 3–4: per loop, slab after slab, sweep struts and
    // close the wall quads. A slab's far rims stay in the seed face as
    // the next slab's base; the last slab's are the top cap's. Struts
    // are SCAFFOLDING lines here (mev_line) and upgrade to the seam
    // class in phase 6, once their walls' keys exist. ----
    let mut side_faces: Vec<Vec<Vec<Option<FaceKey>>>> = Vec::with_capacity(bloops.len());
    let mut seam_edges: Vec<Vec<Vec<Option<EdgeKey>>>> = Vec::with_capacity(bloops.len());
    // Each interior section's rims, with the loop, segment and slab of
    // the wall below them.
    let mut section_rims: Vec<(EdgeKey, usize, usize, usize)> = Vec::new();
    for (li, base) in bases.iter().enumerate() {
        let n = base.len();
        let mut base = base.clone();
        let mut faces: Vec<Vec<Option<FaceKey>>> = vec![Vec::with_capacity(slabs); n];
        let mut seams: Vec<Vec<Option<EdgeKey>>> = vec![Vec::with_capacity(slabs); n];
        for s in 0..slabs {
            let far = &q_at[at(s + 1)][li];
            // A corner the door pinned in this slab is one vertex in
            // both sections, so it gets no strut, and the half of the
            // seed face's loop that leaves its top is its rim's: `top[j]`
            // is that half for every corner, moved on as each wall's far
            // rim takes its place.
            let mut top: Vec<topo::HalfEdgeKey> = Vec::with_capacity(n);
            for j in 0..n {
                if pinned_at(li, j, s) {
                    seams[j].push(None);
                    top.push(base[j]);
                    continue;
                }
                let m = body.mev_line(
                    MevSite::Fan {
                        he1: base[j],
                        he2: base[j],
                    },
                    far[j],
                    tol,
                )?;
                seams[j].push(Some(m.edge));
                top.push(m.he_minus);
            }
            for j in 0..n {
                // A segment pinned at both ends and the same curve in
                // both sections is the edge both hold: no wall, and its
                // rim stays the next slab's base.
                if flat(li, j, s) {
                    faces[j].push(None);
                    continue;
                }
                let mef = body.mef(
                    MefSite::Chords {
                        he1: top[j],
                        he2: top[(j + 1) % n],
                    },
                    rim_spec(at(s + 1), li, j, far[j]),
                    wall_surface(li, j, s),
                    tol,
                )?;
                faces[j].push(Some(mef.face));
                // The far rim's plus half stays in the seed face, from
                // the top of corner `j`: the next slab's base.
                base[j] = mef.he_plus;
                top[j] = mef.he_plus;
                // A rim every slab above holds flat is the top cap's,
                // which states it at rest; any other far rim below the top
                // is an interior section's.
                if !(s + 1..slabs).all(|t| flat(li, j, t)) {
                    section_rims.push((mef.edge, li, j, s));
                }
            }
        }
        side_faces.push(faces);
        seam_edges.push(seams);
    }

    // ---- Phase 5: the swept seed face survives as the top cap. ----
    body.set_face_surface(
        top_face,
        FaceSurface::New {
            surface: cap_plane(place_top.frame(), false, CapEnd::End),
            sense: true,
        },
    )?;

    // Both cap planes exist now, so both rims are at REST in them and
    // stop leaning on the scaffolding door they had to be minted
    // through (D3's transience fence — a cap's plane is fitted THROUGH
    // its own rim, so the rim cannot name it at mint time).
    describe_face_rim_at_rest(&mut body, bottom_face, tol)?;
    describe_face_rim_at_rest(&mut body, top_face, tol)?;
    // A segment held flat through every slab is a hinge: the edge both
    // caps hold, between two planes, so it rests as their wedge demands.
    for (li, base) in bases.iter().enumerate() {
        for (j, &he) in base.iter().enumerate() {
            if (0..slabs).all(|s| flat(li, j, s)) {
                let Some(edge) = body.get_half_edge(he).map(|h| h.edge) else {
                    unreachable!("a cap rim's half-edge is live until the assembly ends")
                };
                hinge_at_rest(&mut body, edge, band, tol)?;
            }
        }
    }

    // ---- Phase 6: strut upgrades to the seam class — the wall keys
    // now exist, so each strut re-describes as its slab wall's `u = 0`
    // boundary iso through the certified setter (D9 — loops in
    // canonical order, vertices in swept order, slabs in stacking
    // order) — and each interior section's rims come to rest in the
    // wall below them. ----
    for (li, seams) in seam_edges.iter().enumerate() {
        let n = seams.len();
        for j in 0..n {
            for s in 0..slabs {
                // A pinned corner has no strut to describe.
                let Some(strut) = seams[j][s] else {
                    continue;
                };
                let Some(wall) = side_faces[li][j][s] else {
                    unreachable!(
                        "a corner with a strut is not pinned, so the segment it starts has a wall"
                    )
                };
                let wall_key = face_surface_key(&body, wall);
                let carrier = geom_brep::boundary_iso_u(walls[li][j][s].as_ref(), false)
                    .map_err(|source| LoftError::SeamStructure { source })?;
                let spec = EdgeCurveSpec {
                    description: EdgeDescriptionSpec::iso(
                        wall_key,
                        T::zero(),
                        T::zero(),
                        T::one(),
                        T::zero(),
                        T::one(),
                    ),
                    carrier: Curve3::Nurbs(Arc::new(carrier)),
                    param_start: T::zero(),
                    param_end: T::one(),
                };
                body.set_edge_curve(strut, spec, tol)?;
            }
        }
    }
    for &(rim, li, j, s) in &section_rims {
        let Some(wall) = side_faces[li][j][s] else {
            unreachable!("an interior section's rim is laid by the mef of the wall below it")
        };
        let chart = face_surface_key(&body, wall);
        describe_section_rim(
            &mut body,
            rim,
            chart,
            &walls[li][j][s],
            at(s + 1),
            band,
            tol,
        )?;
    }
    // ---- Phase 7: whole-body pcurve mint (spec §1's final pass) —
    // every wall boundary stores its exact line-in-UV image. ----
    topo::mint_pcurves(&mut body, tol).map_err(LoftError::Pcurve)?;

    body.close_already_checked();

    // ---- The keys by segment: pieces regrouped under the canonical
    // segment they cut. A segment's first piece starts at its profile
    // vertex, whose strut is a seam; the struts at its later pieces are
    // its sub-arc joints. ----
    let mut by_segment: Vec<Vec<Vec<Vec<Option<FaceKey>>>>> = Vec::with_capacity(owner.len());
    let mut vertex_seams: Vec<Vec<Vec<Option<EdgeKey>>>> = Vec::with_capacity(owner.len());
    let mut joint_edges: Vec<Vec<Vec<Vec<Option<EdgeKey>>>>> = Vec::with_capacity(owner.len());
    for (li, pieces) in owner.iter().enumerate() {
        let segments = cuts[li].len();
        let (mut faces, mut seams, mut joints) = (
            vec![Vec::new(); segments],
            Vec::with_capacity(segments),
            vec![Vec::new(); segments],
        );
        for (q, &(j, sub)) in pieces.iter().enumerate() {
            faces[j].push(side_faces[li][q].clone());
            if sub == 0 {
                seams.push(seam_edges[li][q].clone());
            } else {
                joints[j].push(seam_edges[li][q].clone());
            }
        }
        by_segment.push(faces);
        vertex_seams.push(seams);
        joint_edges.push(joints);
    }

    Ok(Lofted {
        body: built,
        solid: seed.solid,
        shell: seed.shell,
        top: top_face,
        bottom: bottom_face,
        side_faces: by_segment,
        seam_edges: vertex_seams,
        joint_edges,
        section_params,
    })
}

/// **The loft body** (M6-PLAN unit 3, spec §1): skins
/// [`loft_geometry`](crate::loft_geometry) and assembles the closed solid around it.
///
/// `sections[i][l][j]` is section `i`, loop `l`, segment `j` in sketch
/// coordinates; `places[i]` its placement, rigid and right-handed by
/// its type ([`SketchPlane`] is a [`geom_core::RigidMotion`], so a
/// mirrored frame refuses where it is minted); `v_degree` the
/// skinning degree in the section direction, all in `T`. The body is
/// computed in `T`; where its structure is decided depends on the
/// scalar (module docs, "Scalar posture").
///
/// # Correspondence — the vertex order you wrote
///
/// Sections are paired **by index over the canonical loops**, and the
/// canonical form keeps each loop's AUTHORED start
/// ([`profile::Profile::validate`] normalizes only the traversal sense),
/// so segment `j` of every section is counted from the vertex you wrote
/// first ([`loft_geometry`](crate::loft_geometry), "The correspondence is the author's").
/// **A section rotated relative to its neighbour rolls the body by the
/// angle you authored**: the turning-orientation suite's authored-roll
/// row lofts a square onto the same square rotated by `theta` about its
/// own centre, each written from the image of the other's start, and
/// the body rolls by `theta`. To change the twist, start the section at
/// a different vertex.
///
/// The sections sit at [`crate::loft_parameters`]' v-parameters, a
/// function of the section set: a section spelled from a different
/// starting vertex, or rolled about its own normal by one of its own
/// symmetries, builds the same body.
///
/// `places[i]` is the caller's. For the planes normal to a path at
/// uniform shares of its parameter, [`crate::sweep_places`] carries a
/// start frame along it, and a different roll is a rotation composed
/// about the tangent onto each.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum.
///
/// # Panics
///
/// On a kernel bug: an edge the operation stored whose own span or
/// winding reads certainly refused at the run's band where it is
/// re-described ([`topo::Body::restate_stored_edge`]).
pub fn loft_body<T: Decide + topo::AtRestPolicy>(
    sections: &[Section<impl SectionLoop<T>>],
    places: &[SketchPlane<T>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    loft_body_construction(sections, places, v_degree, tol)?.leaf(tol)
}

/// **[`loft_body`] before its finish**: the built body in construction
/// state, every key kept. The one door this crate's suites reach the
/// walls through where the body is not finished at its scalar (an
/// enclosure scalar whose certificate does not run on these walls).
pub(crate) fn loft_body_construction<T: Decide + topo::AtRestPolicy>(
    sections: &[Section<impl SectionLoop<T>>],
    places: &[SketchPlane<T>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T, Body<T>>, LoftError> {
    build(
        SectionsAtT::Loops(sections),
        places,
        &|| Ok(places.iter().map(|p| p.point_value()).collect()),
        None,
        v_degree,
        tol,
    )
}

/// **The loft body over sections already validated at `T`** — each a
/// [`ValidatedProfile`] carrying its own sketch plane, which is its
/// placement. The door for a caller that holds its sections as
/// canonical forms in the lane scalar: a document's profile values,
/// which its own (guided) validation produced.
///
/// No section is validated again: the walls, caps and stacking door
/// read the given forms themselves. Everything else is [`loft_body`]'s.
///
/// # Errors
///
/// [`loft_body`]'s.
///
/// # Panics
///
/// On a kernel bug: an edge the operation stored whose own span or
/// winding reads certainly refused at the run's band where it is
/// re-described ([`topo::Body::restate_stored_edge`]).
pub fn loft_profiles<T: Decide + topo::AtRestPolicy>(
    sections: &[ValidatedProfile<T>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    loft_profiles_construction(sections, v_degree, tol)?.leaf(tol)
}

/// **[`loft_profiles`] before its finish**: the built body in construction
/// state, every key kept. The one door this crate's suites reach the
/// walls through where the body is not finished at its scalar (an
/// enclosure scalar whose certificate does not run on these walls).
pub(crate) fn loft_profiles_construction<T: Decide + topo::AtRestPolicy>(
    sections: &[ValidatedProfile<T>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T, Body<T>>, LoftError> {
    let places: Vec<SketchPlane<T>> = sections.iter().map(|s| *s.plane()).collect();
    build(
        SectionsAtT::<T, profile::ProfileLoop<T>>::Validated(sections),
        &places,
        &|| Ok(None),
        None,
        v_degree,
        tol,
    )
}

/// Whether `holds` of every value of `lp`.
fn loop_all<T: Real>(lp: &profile::ProfileLoop<T>, holds: impl Fn(T) -> bool + Copy) -> bool {
    lp.vertices().iter().all(|p| point_all(p, holds))
        && lp.segments().iter().all(|s| match s {
            profile::Segment::Line => true,
            profile::Segment::Arc(arc) => arc_all(arc, holds),
        })
}

/// [`loop_all`] for a canonical form.
fn profile_all<T: Real>(profile: &ValidatedProfile<T>, holds: impl Fn(T) -> bool + Copy) -> bool {
    profile.loops().iter().all(|lp| {
        lp.vertices().iter().all(|p| point_all(p, holds))
            && lp.segments().iter().all(|s| {
                point_all(&s.start, holds)
                    && point_all(&s.end, holds)
                    && match &s.kind {
                        profile::SegmentKind::Line { .. } => true,
                        profile::SegmentKind::Arc { arc, .. } => arc_all(arc, holds),
                    }
            })
    })
}

fn point_all<T: Real>(p: &Point2<T>, holds: impl Fn(T) -> bool) -> bool {
    holds(p.x) && holds(p.y)
}

fn arc_all<T: Real>(arc: &geom_core::Arc2<T>, holds: impl Fn(T) -> bool + Copy) -> bool {
    point_all(&arc.centre, holds) && holds(arc.radius) && holds(arc.sweep)
}

/// **The path-swept body** (§10.4 as a solid): places rigid copies of
/// `profile` along `path` ([`crate::sweep_geometry`]'s frame — same
/// machinery, no fork) and assembles the loft of those sections.
///
/// # Correspondence
///
/// [`loft_body`]'s paragraph of that name applies, and lands softly
/// here for a reason worth knowing: every section is the SAME profile,
/// so every section canonicalizes identically and the index pairing is
/// the identity whatever the profile's vertex order was. What the
/// authored start still decides is which wall of the
/// built body is which — the segment order the returned
/// [`Lofted::side_faces`] is keyed in. The body's roll comes from the
/// path's carry of the start frame ([`crate::sweep_places`];
/// `crates/sweep/README.md` `[sweep.frame]`), not from the sections.
/// Station `i` sits at `v = i / (stations − 1)`, its share of the
/// path's parameter, which neither the spelling nor the roll of the
/// start frame moves.
///
/// # The starting frame
///
/// `place` — the frame every station is carried from — is the
/// caller's. With no roll in mind, a caller writes the world frame at
/// the path's start and turns it onto the start tangent,
/// `RigidMotion::axes_xy(start).turned_onto(tangent, …)`: its local +Z
/// is the start tangent and its roll the written frame's, carried by
/// the least turn, refused typed across the band around that frame's
/// −z, where the least turn is not decided (`−z` itself included). A
/// caller wanting a different roll composes a rotation about the
/// tangent onto that frame; there is no second door.
///
/// # Errors
///
/// [`LoftError`] — every door named on the enum, with every refusal
/// [`crate::sweep_places`] makes arriving through [`LoftError::Skin`];
/// a profile with no loops is [`SkinError::EmptyProfile`] with no
/// section.
///
/// # Panics
///
/// On a kernel bug: an edge the operation stored whose own span or
/// winding reads certainly refused at the run's band where it is
/// re-described ([`topo::Body::restate_stored_edge`]).
pub fn sweep_body<T: Decide + topo::AtRestPolicy>(
    profile: &[impl SectionLoop<T>],
    plane: SketchPlane<T>,
    path: &geom::NurbsCurve3<f64>,
    stations: usize,
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T>, LoftError> {
    sweep_body_construction(profile, plane, path, stations, v_degree, tol)?.leaf(tol)
}

/// **[`sweep_body`] before its finish**: the built body in construction
/// state, every key kept. The one door this crate's suites reach the
/// walls through where the body is not finished at its scalar (an
/// enclosure scalar whose certificate does not run on these walls).
pub(crate) fn sweep_body_construction<T: Decide + topo::AtRestPolicy>(
    profile: &[impl SectionLoop<T>],
    plane: SketchPlane<T>,
    path: &geom::NurbsCurve3<f64>,
    stations: usize,
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T, Body<T>>, LoftError> {
    if profile.is_empty() {
        return Err(LoftError::Skin(SkinError::EmptyProfile { section: None }));
    }
    let carry = PathCarry::of(path, stations, tol).map_err(LoftError::Skin)?;
    let places = carry.place(plane).map_err(LoftError::Skin)?;
    let sections: Vec<Section<_>> = core::iter::repeat_n(profile.to_vec(), places.len()).collect();
    build(
        SectionsAtT::Loops(&sections),
        &places,
        // The stations at `f64`, minted from the start frame's point
        // value: read only where the structure is selected on the value
        // channel.
        &|| {
            plane
                .point_value()
                .map(|start| carry.place(start))
                .transpose()
                .map_err(LoftError::Skin)
        },
        Some(&carry.params()),
        v_degree,
        tol,
    )
}

/// **A body at `T` whose carriers hold `f64` values as points**: the
/// `f64` body [`loft_body`] builds over `sections` and `places`, each
/// wall's poles, weights and knots, each cap profile's stored scalars
/// and each placement entered at `T` as the point it is, and assembled
/// there. A family of one whose member has `f64` values. It is not the
/// loft of these sections at `T`, which is built in `T` and holds what
/// that arithmetic gives.
///
/// The `f64` half is the
/// loft's own pipeline ([`validate_loft`], [`loft_walls`]) and the `T`
/// half its assembly; what differs is that the walls, cap profiles and
/// placements are entered at `T` between the two.
///
/// # Errors
///
/// [`loft_body`]'s at `f64` short of its finish, and the assembly's at
/// `T`.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn f64_body_held_as_points<T: Decide + topo::AtRestPolicy>(
    sections: &[Section<profile::ProfileLoop<f64>>],
    places: &[SketchPlane<f64>],
    v_degree: usize,
    tol: Tol,
) -> Result<Lofted<T, Body<T>>, LoftError> {
    let (canonical, _) = validate_loft(sections, places, v_degree, tol).map_err(LoftError::Skin)?;
    let (walls, params, pinned) = loft_walls(places, &canonical, v_degree, None, tol)?;
    let held_places = crate::test_support::lift_places::<T>(places, tol);
    let held: Vec<ValidatedProfile<T>> = canonical
        .into_iter()
        .zip(&held_places)
        .map(|(c, place)| {
            c.lift_onto(*place)
                .unwrap_or_else(|e| unreachable!("{e}: a placement admitted at `T` is finite"))
        })
        .collect();
    let walls = walls
        .iter()
        .map(|lp| {
            lp.iter()
                .map(|slabs| {
                    slabs
                        .iter()
                        .map(|w| Arc::new(w.map_scalar(T::from_f64)))
                        .collect()
                })
                .collect()
        })
        .collect();
    let params = params.into_iter().map(T::from_f64).collect();
    assemble(&held_places, walls, &held, params, &pinned, tol)
}

#[cfg(test)]
mod undecided_tests {
    use super::*;

    /// **A loft's stacking escalation offers the tolerance a value in the
    /// band gives**: the decision is the sections' mean spacing along a
    /// normal, a length.
    #[test]
    fn a_stacking_escalation_ends_in_its_lengths_tolerance() {
        use crate::swept::undecided_fixtures::{SIZED, expected, readings};
        let near = expected("whether loft sections 1 and 2 stack forward", SIZED);
        let far = expected(
            "whether loft section 2's plane faces along the stack from section 1",
            SIZED,
        );
        for ((source, near), far) in readings("loft_stacking").into_iter().zip(near).zip(far) {
            assert_eq!(
                LoftError::StackingEscalated { slab: 1, source }
                    .render(geom_brep::recourse::Reading::Build),
                near
            );
            assert_eq!(
                LoftError::FarStackingEscalated { slab: 1, source }
                    .render(geom_brep::recourse::Reading::Build),
                far
            );
        }
    }

    /// **A pinned-corner or clearance escalation offers the tolerance a
    /// value in the band gives**: each decides a distance between two
    /// placed points or cells, a length.
    #[test]
    fn a_corner_or_clearance_escalation_ends_in_its_lengths_tolerance() {
        use crate::swept::undecided_fixtures::{SIZED, expected, readings};
        let corner = expected(
            "whether loft sections 1 and 2 hold corner 3 of loop 0 at one point",
            SIZED,
        );
        let apart = expected(
            "whether loft section 1's segment 2 of loop 0 and section 2's segment 0 of loop 1 \
             are apart",
            SIZED,
        );
        let at = |loop_index, segment| LoopSegment {
            loop_index,
            segment,
        };
        for ((source, corner), apart) in readings("loft_corner_pinned")
            .into_iter()
            .zip(corner)
            .zip(apart)
        {
            assert_eq!(
                LoftError::CornerEscalated {
                    loop_index: 0,
                    corner: 3,
                    slab: 1,
                    source
                }
                .render(geom_brep::recourse::Reading::Build),
                corner
            );
            assert_eq!(
                LoftError::ClearanceUndecided {
                    slab: 1,
                    segments: [at(0, 2), at(1, 0)],
                    source
                }
                .render(geom_brep::recourse::Reading::Build),
                apart
            );
        }
    }
}
