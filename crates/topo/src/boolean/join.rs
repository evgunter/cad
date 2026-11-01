//! Boolean joining (ch. 15 §15.7, Programs 15.13/15.14): the ch. 14
//! join skeleton (PR 3's [`ChordJoiner`]) driven **in lockstep across
//! both solids**, with `scanjoin`'s dual requirement — a candidate
//! joins only where a topological neighbor exists in BOTH solids at
//! the SAME loose end — realized through PR 4's explicit
//! [`NullEdgePairRecord`](super::NullEdgePairRecord) correspondence
//! keys and the **germ facings** ([`HalfGerm`]) recorded at insertion
//! (F9: correspondence as data, never correlated array order — the
//! book's `ssortnulledges` ordering/orientation discipline is
//! enforced as the derived sense data below, not as a sort).
//!
//! The section frame a curved germ turns about is read here too, keyed
//! by the germ's two surface kinds ([`germ_section_loops`],
//! [`pair_section_frame_at`]): a straight pair has none, each conic pair
//! names its centre and axis, and the plane×torus pair and the equal
//! cylinder pair with meeting axes name two loops with a frame each
//! ([`SectionLoops`]), of which a germ takes the one its branch names
//! ([`germ_section_frame`]; the branch is decided where the germ is
//! minted, [`germ_branch`]).
//!
//! # Matching is germ identity, not slots or senses (the below-copy
//! # audit)
//!
//! A section-polygon edge lies in one cell of each operand — inside a
//! face, or along an edge, where it lies in both faces the edge bounds
//! ([`super::Locus`]); its two end sites offer null-edge halves
//! *facing* that germ. The neighbor test is therefore pure data:
//!
//! - the two halves' germ **loci agree** on both operands (this
//!   subsumes the book's same-face test, is immune to mid-join face
//!   divisions, and names a segment along an edge the same at both
//!   ends, whichever flank each end attributed it to);
//! - their record **parities oppose** (the book's "opposite he1/he2
//!   roles" — 15.11's IN-record→OUT-record orientation carried as
//!   data);
//! - the A-side and B-side germs of one match carry the SAME loci (one
//!   spatial polygon edge).
//!
//! A segment whose locus on a solid is an edge is that edge: the
//! solid's adjacency skip reads the locus, not the geometry
//! ([`SegmentEdge::Locus`]).
//!
//! A matched segment's chord curve is computed once per solid, before
//! any chord is minted ([`SegmentCurve`]): the joiner mints both chords
//! on it, and the ring lane winds the run the first chord closes with
//! it ([`ring_run_ccw`]). The mekr and outer lanes order the halves
//! first, topologically, and the curve is computed in that order; so
//! does a ring-face match across its segment's own edge, whose one
//! chord the two orders mint alike, and a ring of a wall face, whose
//! island winds on the face's chart closed along its section plane. Any
//! other ring-lane match — a ring of a planar face — computes the curve
//! first and orders the halves by it.
//!
//! Boolean runs mint copies of BOTH parities (In-runs mint
//! `NewVertexSide::Below` copies — the PR 4 interface fact); nothing
//! here assumes above-only: polygon-completion roles are resolved by
//! membership of the IN/OUT end-vertex sets built from the F9
//! attributes (`in_copy` = the loop through IN ends), and the germ
//! facings identify halves regardless of which side was minted.
//!
//! # The seam-orientation discipline (the derived form of the book's
//! # ssortnulledges / he1↔he2 crossover)
//!
//! Derived from the ratified conventions (outward normals, loops
//! CCW-from-outside: a half-edge with tangent `t` on a face with
//! normal `n` has interior to its LEFT, `n×t` pointing in):
//!
//! 1. **Required end state.** On the germ line of face pair (fA, fB),
//!    the boundary of fA's region inside B runs `tA(in) = nA×nB`
//!    (check: `nA×(nA×nB) ∝ proj(−nB)`, the into-B direction);
//!    `tA(out)`, `tB(in)`, `tB(out)` follow by the A↔B and in↔out
//!    mirrors, giving `tA(x) = −tB(x)`. Section loops are mates of
//!    region boundaries, so for every op — ∩ (IN,IN), ∪ (OUT,OUT),
//!    ∖ (A-OUT, revert(B-IN)) — the kept loops are antiparallel at
//!    the zip **iff each solid's section loops attach to its own
//!    regions geometrically-CCW-consistently**. Op-independent.
//!    An edge-edge germ's record may pair two COPLANAR flankers (each
//!    solid's own fold flanker, `recl::place_germ`), where `nA×nB` is
//!    zero and names no line: there the germ line is the common edge
//!    itself (`insert::plan_null_pairs`' `dirs` take the bound
//!    read On of a germ along an edge, the A flanker's where both solids
//!    hold the edge, and `fn germ_dir`'s normal cross everywhere else),
//!    and the loops' direction along it is the region boundaries' as
//!    above, read on that edge.
//! 2. **The sense theorem.** The half FACING germ `g` is UP (starts
//!    at `below_end`) iff `g`'s own-solid forward-wedge code is Out;
//!    geometrically, with the orbit-forward direction `w = σ·n_own×d`
//!    (σ the fixed orbit handedness, shared by both solids), UP ⟺
//!    `σ·det[n_own, d, n_other] > 0`. Mirror checks: a CROSSING-lane
//!    strut's two germs share the line with `d1 = −d0` ⇒ opposite
//!    senses (pierce-lane RING struts carry perpendicular germ dirs
//!    instead — their opposite senses come from the cross-solid
//!    anti-correlation below, not the shared-line mirror); the two
//!    facing germs of one polygon side ⇒ opposite senses within each
//!    solid (the neighbor test); `det[nA,d,nB] = −det[nB,d,nA]` ⇒
//!    **sense_A(g) = ¬sense_B(g) at every germ** — the cross-solid
//!    anti-correlation. Insertion mints attributes to this rule
//!    (struts included — their facing swap swaps the labels with it),
//!    so the attributes ARE the discipline; nothing rebinds later.
//!    A strut faces its germs by one rule
//!    (`insert::strut_faces_first`): a germ along the splice corner's
//!    edge names its half; otherwise the corner's walk orders them,
//!    by their sector entries, and two in one entry by their angle
//!    (`insert::strut_order`), read as distances: a germ's half-turn
//!    side at the arrival edge's reach, two germs' order at the sector's
//!    arm;
//!    where nothing orders them it refuses.
//! 3. **What the join controls.** Surgery never reverses existing
//!    halves, and chords close cycles forced by arc endpoints, so the
//!    directed cycles after every join are fixed by the senses alone:
//!    per polygon side the IN copies are chorded `up-site → down-site`
//!    on the region side (that direction CCW-bounds the IN region —
//!    the same determinant as step 2), and the section face receives
//!    the antiparallel copies. Role order moves only FACE identity:
//!    which cycle becomes the mef's new face vs stays with the old.
//!    That is orientation-neutral for outer-loop splits and mekr
//!    merges, and load-bearing exactly for RING splits, where the
//!    remainder becomes the old face's ring (a hole boundary must
//!    anti-enclose): the run must take the CCW-winding cycle —
//!    [`choose_roles`]' derived rule via [`ring_run_ccw`], decided
//!    intrinsically so multi-polygon faces cannot cross it.
//! 4. **Consistency theorem.** With (2) as data, (3)'s ring rule per
//!    solid, and matching that consumes the SAME germ in both solids
//!    ([`find_match`]'s slot lock), every completed polygon pair has
//!    A's IN loop antiparallel to B's IN loop, and the zip assertion
//!    ([`BooleanError::SeamOrientation`]) is a theorem with a runtime
//!    witness, not a hope.
//!
//! # The fixpoint sweep and lockstep discipline
//!
//! All pair records register up front, and [`matched_records`]
//! matches them: it repeatedly takes the nearest valid match in
//! deterministic scan order until quiescent (each match consumes its
//! germs, so the next round ranks only the germs still free). The criterion
//! reads only record data — loci, senses, site points, section
//! frames — and the segments are decided before any chord is minted.
//! Surgery moves no site, but a chord dividing a face moves the
//! boundary vertices a section frame is levered at ([`frame_lever`]),
//! so every frame and kind arm the join reads is read in that
//! pre-surgery state ([`SegmentPair`]). A record left whole whose two
//! germs are one closed loop through its one site, a wrap edge or a
//! closed edge's vertex, is one more segment
//! ([`one_site_segments`]), read with the matched ones
//! ([`join_segments`]). The sweep then joins them in that order. Joins,
//! retirements, and completions must occur in
//! BOTH solids together; any divergence is the typed
//! [`BooleanError::JoinDesync`] refusal, never a silent mis-join.
//! There is no geometric sort and no section-area certification here:
//! boolean intersection polygons are in general non-planar (§15.7);
//! degenerate results are netted at the component stage instead.

use geom_core::{Band, Decide, Margin, Point3, Sign, UnitVec3, Vec3};
use slotmap::SecondaryMap;
use std::collections::BTreeMap;

use super::shell_witness::{Reading, complex_side};
use super::{
    BooleanDecision, BooleanError, BooleanReduction, Coincide, DeclarationRead, HalfGerm, Operand,
    SectionUnbuilt, SelfCheck, SideCode,
};
use crate::body::Body;
use crate::chord_join::{
    ChordJoiner, CutOutcome, Datum, JoinLane, JoinPlan, Leave, RimSide, SegmentCurve, SegmentEdge,
    SplitJoinError,
};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::face_normal::face_outward_normal;
use crate::loop_winding::{RunClosing, RunMissesEnd, Unwound};
use crate::null::NullFacePair;
use crate::ring_path::LoopSplit;
use crate::validate::{decide, decide_positive};
use geom_core::Tol;

/// The K funnel name of a germ plane's normal-length decision: the
/// boolean reads a plane carrier's normal into the section lanes, and
/// decides its length there because the carrier's unit length is an
/// at-rest convention no tier certifies. Its comparand is the vector's
/// norm, a pure number, levered by a lower bound on the reach the
/// section consumes it over: the joined germ sites' reach from the
/// plane's origin ([`UnitVec3::levered`]).
pub(super) const BOOL_GERM_PLANE_NORMAL: &str = "bool_germ_plane_normal";

/// One completed section-polygon **pair**: the 2-loop null face in
/// each solid, with the loop roles as F9 data (IN copy = the loop
/// through the IN-side ends).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompletedPolygonPair {
    /// The A-clone null face.
    pub a_face: FaceKey,
    /// A's IN-copy loop.
    pub a_in_loop: LoopKey,
    /// A's OUT-copy loop.
    pub a_out_loop: LoopKey,
    /// The B-clone null face.
    pub b_face: FaceKey,
    /// B's IN-copy loop.
    pub b_in_loop: LoopKey,
    /// B's OUT-copy loop.
    pub b_out_loop: LoopKey,
}

/// The chord lane a matched segment joins by, with the section datum
/// each side's chords lie in: chosen by its germ face pair, or, where
/// no face-pair arm exists, by the conic edge its germ runs along.
#[derive(Clone, Copy)]
enum GermLane<T: geom_core::Real> {
    /// Plane × plane: the chord runs along the two planes' common line.
    Planar,
    /// A pair whose section the operation's section table carries: each
    /// side's chord is an arc of a component of the carried section,
    /// given there: the one the germ's branch names
    /// (`chord_join::JoinLane::Carried`).
    Carried(usize),
    /// A planar, B a wall: B's chords lie in A's plane.
    PlaneWall((Point3<T>, UnitVec3<T>)),
    /// A a wall, B planar: A's chords lie in B's plane.
    WallPlane((Point3<T>, UnitVec3<T>)),
    /// A sphere pair: both sides' chords lie in the radical plane.
    Radical((Point3<T>, UnitVec3<T>)),
    /// A cylinder pair with parallel axes: both rulings lie in the
    /// pair's radical plane, and each side's chord is its own wall's
    /// ruling in it.
    Rulings((Point3<T>, UnitVec3<T>)),
    /// An equal cylinder pair with intersecting axes: the segment rides
    /// the ellipse of its germ's branch, which lies in that branch's
    /// bisector plane, and each side's chord is its own wall cut by
    /// that plane.
    Bisector((Point3<T>, UnitVec3<T>)),
    /// A segment along a conic edge of the solid `on` only, lying inside
    /// the other's curved face: `on`'s chord copies its edge, and the
    /// other's is its face cut by the edge's plane, which meets that
    /// face's carrier in the edge's conic. `root` is the edge's split
    /// root, which every piece of it shares.
    EdgePlane {
        on: Operand,
        root: EdgeKey,
        plane: (Point3<T>, UnitVec3<T>),
    },
}

impl<T: geom_core::Real> GermLane<T> {
    /// How each side's ring-lane island closes, A's then B's: a planar
    /// germ face on its own plane, a wall along the section its chords
    /// lie in, a carried chord's face by the chord, and the edge's own side of an [`Self::EdgePlane`] as
    /// along its edge, since its chord copies the edge and reads no
    /// section.
    fn ring_closures(self) -> (RingClosure<T>, RingClosure<T>) {
        match self {
            Self::Planar => (RingClosure::Planar, RingClosure::Planar),
            Self::Carried(_) => (RingClosure::Chord, RingClosure::Chord),
            Self::PlaneWall(plane) => (RingClosure::Planar, RingClosure::Wall(plane)),
            Self::WallPlane(plane) => (RingClosure::Wall(plane), RingClosure::Planar),
            Self::Radical(plane) | Self::Rulings(plane) | Self::Bisector(plane) => {
                (RingClosure::Wall(plane), RingClosure::Wall(plane))
            }
            Self::EdgePlane {
                on: Operand::A,
                plane,
                ..
            } => (RingClosure::AlongEdge(Operand::A), RingClosure::Wall(plane)),
            Self::EdgePlane {
                on: Operand::B,
                plane,
                ..
            } => (RingClosure::Wall(plane), RingClosure::AlongEdge(Operand::B)),
        }
    }
}

/// The germ face pair's kind arm in [`bool_connect`]'s lane dispatch,
/// the face-pair half of [`GermLane`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KindArm {
    /// [`GermLane::Planar`].
    Planar,
    /// [`GermLane::PlaneWall`].
    PlaneWall,
    /// [`GermLane::WallPlane`].
    WallPlane,
    /// [`GermLane::Radical`].
    Spheres,
    /// [`GermLane::Rulings`]: a cylinder pair, which keeps this arm only
    /// on parallel axes ([`segment_arm`]).
    Cylinders,
    /// [`GermLane::Bisector`]: a cylinder pair whose section is the two
    /// bisector-plane ellipses, a germ inside a face of each operand
    /// ([`segment_arm`]).
    Bisector,
    /// No face-pair arm: the edge-plane lane, where the germ runs along
    /// a conic edge of one solid in a curved face of the other, takes
    /// it, and only here, or it refuses.
    None,
}

/// The kind arm of a germ face pair by its kinds, A's kind first.
fn kind_arm(a: geom::SurfaceKind, b: geom::SurfaceKind) -> KindArm {
    use geom::SurfaceKind::{Cone, Cylinder, Plane, Sphere, Torus};
    match (a, b) {
        (Plane, Plane) => KindArm::Planar,
        (Plane, Sphere | Cylinder | Cone | Torus) => KindArm::PlaneWall,
        (Sphere | Cylinder | Cone | Torus, Plane) => KindArm::WallPlane,
        (Sphere, Sphere) => KindArm::Spheres,
        (Cylinder, Cylinder) => KindArm::Cylinders,
        _ => KindArm::None,
    }
}

/// **A face's surface, with its key in `body`**: the join's walk from a
/// face to its surface, whether the face is one a germ was recorded
/// against or one a segment's halves lie on now. It is the crate's one
/// walk ([`crate::readback::carrier_surface`]), its refusal a desync: a
/// live face's surface key resolves (tier 1), so the one refusal is a
/// face key that no longer resolves.
///
/// That no other walk reads a surface in this file is held by
/// `face_surface_tests::the_join_walks_to_a_faces_surface_in_one_place`,
/// which counts the reads in this file's source.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] where `face` does not resolve.
fn face_surface<T: geom_core::Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<(crate::geometry::SurfaceKey, &geom::Surface<T>), BooleanError> {
    let (key, surface, _sense) =
        crate::readback::carrier_surface(body, face).map_err(|_| BooleanError::JoinDesync {
            what: "a face the join reads no longer resolves",
        })?;
    Ok((key, surface))
}

/// **A segment's germ face pair, as [`bool_connect`] reads it** before
/// any segment's surgery divides a face: each face's surface with its
/// key, A's first, the pair's kind arm ([`segment_arm`]), which a
/// segment along an edge of both solids takes none of, and, for a
/// segment whose two ends are one record, the germ's own loop of the
/// pair's section ([`germ_section_frame`]). A segment inside a face of
/// each operand on a pair the operation's section table carries on a
/// fitted component takes no kind arm: `carried` names that component,
/// the one its germ's branch names, which its chords are arcs of.
struct SegmentPair<T: geom_core::Real> {
    a: (crate::geometry::SurfaceKey, geom::Surface<T>),
    b: (crate::geometry::SurfaceKey, geom::Surface<T>),
    arm: Option<KindArm>,
    carried: Option<usize>,
    site_loop: Option<SectionLoop<T>>,
}

impl<T: Decide> SegmentPair<T> {
    /// The pair `seg`'s germ was recorded against, read in `red`.
    fn read(
        red: &BooleanReduction<T>,
        seg: &SectionSegment<T>,
        band: Band,
    ) -> Result<Self, BooleanError> {
        let germ = &seg.germ;
        let owned = |(key, surface): (_, &geom::Surface<T>)| (key, surface.clone());
        let (a, b) = (
            owned(face_surface(&red.a, germ.a_face)?),
            owned(face_surface(&red.b, germ.b_face)?),
        );
        // A closed-form component (the coaxial pair's circles) is a
        // coincidence the section door decided, which the join does not
        // build until the door records it: only a fitted component is
        // carried.
        let carried = match seg.lane {
            SegmentLane::AlongEdge => None,
            SegmentLane::Section => match germ_section(red, germ)? {
                Some(section) => {
                    let k = branch_component(germ.branch, section)?;
                    section.components()[k].certificate.is_some().then_some(k)
                }
                None => None,
            },
        };
        let arm = match (seg.lane, carried) {
            (SegmentLane::AlongEdge, _) | (SegmentLane::Section, Some(_)) => None,
            (SegmentLane::Section, None) => Some(segment_arm(red, germ, (&a.1, &b.1), band)?),
        };
        let [(entry, _), (cand, _)] = seg.ends;
        let site_loop = if entry == cand && carried.is_none() {
            Some(
                germ_section_frame(red, germ, band)?.ok_or(BooleanError::JoinDesync {
                    what: "a one-site section loop has no conic frame",
                })?,
            )
        } else {
            None
        };
        Ok(Self {
            a,
            b,
            arm,
            carried,
            site_loop,
        })
    }
}

/// The index of the component of `section` a germ's `branch` names: its
/// site's foot's, read where the germ was minted ([`germ_branch`]).
fn branch_component<T: Decide>(
    branch: super::Branch,
    section: &geom_brep::CarriedSection<T>,
) -> Result<usize, BooleanError> {
    match branch {
        super::Branch::Component(k) if k < section.components().len() => Ok(k),
        _ => Err(BooleanError::JoinDesync {
            what: "a germ's branch does not name a component of its section",
        }),
    }
}

/// **The kind arm of a segment's germ face pair** ([`kind_arm`]), whose
/// faces' surfaces are `sa` and `sb`: a cylinder pair keeps
/// [`KindArm::Cylinders`] only where its axes read parallel
/// ([`cylinder_pair_axes`]), at the point and lever the frame dispatch
/// reads that face pair at ([`frame_lever`]). Walls whose axes are not
/// parallel meet in no ruling. A germ inside a face of each operand
/// reads the pair's section there ([`cylinder_pair_frame`]), and takes
/// [`KindArm::Bisector`] where it is the two bisector-plane ellipses;
/// every other such pair, and a germ along an edge, falls through to the
/// edge-plane lane. A reading in the band is refused by
/// [`frame_refusal`].
///
/// Read before any segment's surgery, in the state the segments were
/// matched in: the frame lever reads the faces' boundary vertices,
/// which a chord dividing a face moves, so every segment of one face
/// pair takes the one arm that pair reads. Nothing pins that order: no
/// pose found reads differently after an earlier segment's surgery (the
/// first segment reads the matched state; an in-band or apart reading
/// refuses there, before any surgery; a parallel one stays parallel at a
/// later segment unless a chord's end lands outside both walls' vertex
/// ball), so the argument above is what holds it.
fn segment_arm<T: Decide>(
    red: &BooleanReduction<T>,
    germ: &HalfGerm<T>,
    (sa, sb): (&geom::Surface<T>, &geom::Surface<T>),
    band: Band,
) -> Result<KindArm, BooleanError> {
    let arm = kind_arm(sa.kind(), sb.kind());
    let (
        KindArm::Cylinders,
        geom::Surface::Cylinder {
            origin: o1,
            axis: a1,
            radius: r1,
            ..
        },
        geom::Surface::Cylinder {
            origin: o2,
            axis: a2,
            radius: r2,
            ..
        },
    ) = (arm, sa, sb)
    else {
        return Ok(arm);
    };
    let (at, extent) = frame_lever((sa, &red.a, germ.a_face), (sb, &red.b, germ.b_face), band)?;
    let refusal = |e| frame_refusal(e, (germ.a_face, sa), (germ.b_face, sb));
    match cylinder_pair_axes((*o1, *a1), (*o2, *a2), (*r1, *r2), at, extent.clone(), band) {
        Ok(CylinderAxes::Parallel) => return Ok(KindArm::Cylinders),
        Ok(CylinderAxes::Apart(_)) => {}
        Err(e) => return Err(refusal(e)),
    }
    if germ.a_locus.edge().is_some() || germ.b_locus.edge().is_some() {
        return Ok(KindArm::None);
    }
    match cylinder_pair_frame(sa, sb, at, extent, band) {
        Ok(Some(SectionLoops::Two { .. })) => Ok(KindArm::Bisector),
        Ok(_) | Err(FrameError::NoArm) => Ok(KindArm::None),
        Err(e) => Err(refusal(e)),
    }
}

/// How one solid's ring-lane island is closed for its winding, as
/// [`choose_roles`] reads it from the germ pair, typed by the germ
/// face's kind ([`IslandClosing`] is the closing it hands
/// [`ring_run_ccw`]).
#[derive(Clone, Copy)]
enum RingClosure<T: geom_core::Real> {
    /// A planar face: the island closes on the face's own plane, by the
    /// segment's curve.
    Planar,
    /// A curved face: a cylinder's island closes along this section
    /// plane, on the face's chart; a sphere's, a cone's or a torus's by
    /// the segment's curve.
    Wall((Point3<T>, UnitVec3<T>)),
    /// A cylinder, sphere or torus face, closed by the segment's curve,
    /// a carried chord ([`GermLane::Carried`]).
    Chord,
    /// A segment along an edge of this solid (the named operand), of
    /// both solids or of this one alone, reads no section on this side:
    /// a planar face's island closes on its own plane, and a curved
    /// face has nothing to close along, which is a join arm not yet
    /// built.
    AlongEdge(Operand),
}

/// How [`ring_run_ccw`] closes a ring-lane island's open run, by the
/// face it lies on.
#[derive(Clone, Copy)]
enum IslandClosing<'a, T: geom_core::Real> {
    /// A planar face, with its outward normal: closed by the segment's
    /// curve, the chord the join mints.
    Planar(Vec3<T>, &'a SegmentCurve<T>),
    /// A wall face: closed along the section plane its chords lie in,
    /// on the face's chart.
    Wall((Point3<T>, UnitVec3<T>)),
    /// A sphere, a cone or a torus face: closed by the segment's curve,
    /// an arc of a plane section or a cone's ruling
    /// ([`crate::chord_join::path_island_winding`]).
    Quadric(&'a SegmentCurve<T>),
    /// A cylinder, sphere or torus face closed by a carried chord, whose run may
    /// hold earlier carried chords: closed by the chord's arc with its
    /// evidence of lying on the face
    /// ([`crate::chord_join::path_island_winding_closed_by`]).
    Carried(&'a SegmentCurve<T>),
}

/// Per-solid joining state: the shared chord core plus the F9 side
/// data matching reads.
struct SolidJoin {
    joiner: ChordJoiner,
    sides: Sides,
    /// Aux surfaces minted into THIS body for curved germ pairs (M5
    /// PR 9), keyed by the datum each one IS ([`AuxDatum`]) — one mint
    /// per datum, every chord that rides it shares it (the descriptions
    /// stay key-coherent for D6).
    aux: BTreeMap<AuxDatum, crate::geometry::SurfaceKey>,
    /// `(edge, chord)` for every chord minted on a segment along this
    /// solid's `edge` ([`Self::join`]).
    along: Vec<(EdgeKey, EdgeKey)>,
}

/// What an aux surface in [`SolidJoin::aux`] is a copy of, which is
/// what makes two chords' reads of one entry the same datum.
///
/// The shapes are kept apart because they depend on different things:
/// a partner copy depends on the partner face's surface alone, a
/// radical plane on BOTH spheres, an edge's plane on that edge's curve.
/// Keying the radical plane by the partner face alone hands a second
/// sphere of THIS body the first one's plane — a chord described
/// against a plane it does not lie in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum AuxDatum {
    /// A copy of the OTHER body's germ face's surface (a partner plane
    /// for a wall-side chord, a partner wall for a planar-side one).
    Partner(FaceKey),
    /// The radical plane of a sphere pair or a parallel cylinder pair:
    /// this body's surface and the other body's.
    Radical {
        own: crate::geometry::SurfaceKey,
        partner: crate::geometry::SurfaceKey,
    },
    /// One bisector plane of an equal cylinder pair with intersecting
    /// axes: this body's wall, the other body's, and the branch whose
    /// ellipse lies in it.
    Bisector {
        own: crate::geometry::SurfaceKey,
        partner: crate::geometry::SurfaceKey,
        branch: super::Branch,
    },
    /// The plane of a conic edge of the solid `on`, by the edge's split
    /// root: every piece of one split edge lies on the root's conic, so
    /// they share it.
    EdgePlane { on: Operand, root: EdgeKey },
}

impl SolidJoin {
    /// The plan for joining `halves` in that order, its skip the edge
    /// the segment's locus names ([`ChordJoiner::plan`]).
    fn plan<T: Decide>(
        &self,
        body: &Body<T>,
        halves: (HalfEdgeKey, HalfEdgeKey),
        segment: Option<EdgeKey>,
    ) -> Result<JoinPlan, BooleanError> {
        self.joiner
            .plan(body, halves, SegmentEdge::Locus(segment))
            .map_err(BooleanError::from)
    }

    /// The segment's curve in `lane`, from the site of the plan's first
    /// half ([`ChordJoiner::segment_curve`]). A match whose two chords
    /// are both skipped — a loop holding both halves of the segment's
    /// edge — is refused.
    fn curve<T: Decide>(
        &self,
        body: &mut Body<T>,
        plan: &JoinPlan,
        lane: JoinLane<'_, T>,
        leave: Leave<T>,
    ) -> Result<SegmentCurve<T>, BooleanError> {
        let curve = self.joiner.segment_curve(body, plan, lane, leave)?;
        curve.ok_or(BooleanError::from(SplitJoinError::SectionInvariant {
            face: plan.face(),
            what: "both chords of a matched segment are its own edge (a loop holding both \
                   halves of that edge)",
        }))
    }

    /// The wall-side curve against the germ plane through `origin` with
    /// chart normal `normal`, on the loop `branch` names, through
    /// [`JoinLane::Split`], its aux plane read from and minted into
    /// [`Self::aux`] under `datum`.
    #[allow(clippy::too_many_arguments)] // one internal lane, each argument a named duty
    fn split_curve<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        plan: &JoinPlan,
        (origin, normal): (Point3<T>, UnitVec3<T>),
        datum: AuxDatum,
        branch: super::Branch,
        leave: Leave<T>,
    ) -> Result<SegmentCurve<T>, BooleanError> {
        let mut ctx = crate::chord_join::SectionCtx {
            origin,
            normal,
            plane_key: self.aux.get(&datum).copied(),
            branch: Some(branch),
        };
        let curve = self.curve(body, plan, JoinLane::Split(&mut ctx), leave)?;
        if let Some(k) = ctx.plane_key {
            self.aux.insert(datum, k);
        }
        Ok(curve)
    }

    /// The planar-side curve against the partner `wall`, on the loop
    /// `branch` names, through [`JoinLane::BoolPlanar`], the wall copy
    /// keyed by `partner_face`; `window` is that face's along a cone's
    /// axis.
    #[allow(clippy::too_many_arguments)] // one internal lane, each argument a named duty
    fn bool_planar_curve<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        plan: &JoinPlan,
        (wall, window): (geom::Surface<T>, Option<(T, T)>),
        partner_face: FaceKey,
        branch: super::Branch,
        leave: Leave<T>,
    ) -> Result<SegmentCurve<T>, BooleanError> {
        let datum = AuxDatum::Partner(partner_face);
        let mut partner = self.aux.get(&datum).copied();
        let lane = JoinLane::BoolPlanar {
            wall,
            window,
            partner_key: &mut partner,
            branch,
        };
        let curve = self.curve(body, plan, lane, leave)?;
        if let Some(k) = partner {
            self.aux.insert(datum, k);
        }
        Ok(curve)
    }

    /// The curve of a segment whose chords are arcs of `component`, a
    /// component of the germ pair's carried section, through
    /// [`JoinLane::Carried`]: the partner face's surface `partner` copied
    /// into this body once per `datum` ([`Self::aux`]).
    fn carried_curve<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        plan: &JoinPlan,
        ((component, lever), partner): ((&geom_brep::CarriedComponent<T>, T), geom::Surface<T>),
        datum: AuxDatum,
        leave: Leave<T>,
    ) -> Result<SegmentCurve<T>, BooleanError> {
        let mut ctx = crate::chord_join::CarriedCtx {
            component: component.clone(),
            partner,
            partner_key: self.aux.get(&datum).copied(),
            lever,
        };
        let curve = self.curve(body, plan, JoinLane::Carried(&mut ctx), leave)?;
        if let Some(k) = ctx.partner_key {
            self.aux.insert(datum, k);
        }
        Ok(curve)
    }

    /// Mints the matched segment's chords between `roles` on `curve`
    /// ([`ChordJoiner::join`]): `plan`'s chords where the roles are its
    /// order, else the plan of the roles' own order — the ring lane of a
    /// planar face orders the halves by the curve, after it is computed.
    ///
    /// A segment whose locus on this solid is an edge is that edge
    /// (module docs), so a chord minted on it runs along the edge from
    /// end to end: each is logged in [`Self::along`] as `(edge, chord)`,
    /// the substitution row the carriage reads where the op drops the
    /// edge and keeps the chord. The pairing is the locus's key, never
    /// a position.
    fn join<T: Decide + crate::props::AtRestPolicy>(
        &mut self,
        body: &mut Body<T>,
        roles: (HalfEdgeKey, HalfEdgeKey),
        plan: &JoinPlan,
        curve: &SegmentCurve<T>,
        segment: Option<EdgeKey>,
        tol: Tol,
    ) -> Result<(), BooleanError> {
        let reordered;
        let plan = if roles == plan.halves() {
            plan
        } else {
            reordered = self.plan(body, roles, segment)?;
            &reordered
        };
        let minted = self.joiner.join(body, plan, curve, tol).map_err(of_join)?;
        if let Some(edge) = segment {
            self.along
                .extend(minted.into_iter().map(|chord| (edge, chord)));
        }
        Ok(())
    }
}

/// The Boolean's refusal for a chord its join refused: a chord's end
/// whose vertex reads at the zero band's edge of the plane is an edge of
/// the result on its faces, the Boolean's own decision; every other
/// refusal is the join's.
fn of_join(refusal: SplitJoinError) -> BooleanError {
    match refusal {
        SplitJoinError::SectionEndEscalated {
            reading: crate::chord_join::SectionEndReading::OnPlane,
            diag,
            ..
        } => BooleanError::Escalated {
            decision: BooleanDecision::EdgeOnFaces,
            diag,
        },
        other => other.into(),
    }
}

impl SolidJoin {
    fn new<T: Decide>(red: &BooleanReduction<T>, operand: Operand, band: Band) -> Self {
        Self {
            joiner: ChordJoiner::new(band),
            sides: Sides::new(red, operand),
            aux: BTreeMap::new(),
            along: Vec::new(),
        }
    }
}

/// One solid's F9 side data: the null edges' end vertices by side.
struct Sides {
    /// IN-side end vertices (below ends), from the attributes.
    in_set: SecondaryMap<VertexKey, ()>,
    /// OUT-side end vertices (above ends).
    out_set: SecondaryMap<VertexKey, ()>,
}

impl Sides {
    fn new<T: Decide>(red: &BooleanReduction<T>, operand: Operand) -> Self {
        let mut in_set = SecondaryMap::new();
        let mut out_set = SecondaryMap::new();
        for r in red.null_edges_of(operand) {
            in_set.insert(r.attr.below_end, ());
            out_set.insert(r.attr.above_end, ());
        }
        Self { in_set, out_set }
    }

    /// The up/down sense of a null-edge half — `start ∈ in_set` ⇒ up
    /// (the below-copy audit: side is attribute data, either parity of
    /// copy resolves here). A vertex in both sets names no sense.
    fn is_up<T: Decide>(&self, body: &Body<T>, he: HalfEdgeKey) -> Result<bool, BooleanError> {
        let desync = |what| BooleanError::JoinDesync { what };
        let start = body
            .get_half_edge(he)
            .ok_or(desync("half no longer resolves"))?
            .start;
        self.starts_up(start)
    }

    /// Whether a null half starting at `start` is up: `start` is a below
    /// end and no above end.
    fn starts_up(&self, start: VertexKey) -> Result<bool, BooleanError> {
        let desync = |what| BooleanError::JoinDesync { what };
        match (
            self.in_set.contains_key(start),
            self.out_set.contains_key(start),
        ) {
            (true, false) => Ok(true),
            (false, true) => Ok(false),
            (true, true) => Err(desync("null half starts at a vertex of both side sets")),
            (false, false) => Err(desync("null half starts at a vertex of neither side set")),
        }
    }
}

/// A completed 2-loop null-face pair before role resolution.
#[derive(Clone, Copy, Debug)]
struct UnresolvedPair {
    a_face: FaceKey,
    a_outer: LoopKey,
    a_ring: LoopKey,
    b_face: FaceKey,
    b_outer: LoopKey,
    b_ring: LoopKey,
}

/// One registered pair record: each solid's two germ facings with
/// used-in-a-join flags. Slot `i` of `a` and slot `i` of `b` are the
/// SAME spatial germ (minted from one crossing-record pair with shared
/// face-pair/direction meta) — the F9 correspondence the match walks.
#[derive(Clone, Copy, Debug)]
struct OpenRecord<T: geom_core::Real> {
    a_edge: EdgeKey,
    b_edge: EdgeKey,
    a: [(HalfGerm<T>, bool); 2],
    b: [(HalfGerm<T>, bool); 2],
}

impl<T: geom_core::Real> OpenRecord<T> {
    fn fully_used(&self) -> bool {
        self.a.iter().all(|(_, u)| *u) && self.b.iter().all(|(_, u)| *u)
    }
}

/// One section segment as the join's matching decides it: "which
/// segments exist and what each one is". The join's surgery
/// ([`bool_connect`]) reads the matched segments ([`matched_records`])
/// and the one-site segments ([`join_segments`]).
#[derive(Clone, Copy, Debug)]
pub(super) struct SectionSegment<T: geom_core::Real> {
    /// The two ends, `(pair record, germ slot)`, entry end first. The
    /// record indexes `red.null_pairs`; the slot is one spatial germ in
    /// both operands' germ arrays, consumed in both (per-solid slot
    /// freedom was the R2 desync soup).
    pub ends: [Slot; 2],
    /// The A germ at the entry end. Both ends carry its loci on both
    /// operands ([`partners`]), so its loci are the segment's.
    pub germ: HalfGerm<T>,
    /// How the segment is built.
    pub lane: SegmentLane,
}

/// One germ slot of one pair record: `(record, slot)`.
pub(super) type Slot = (usize, usize);

/// How a [`SectionSegment`] is built in each solid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SegmentLane {
    /// An edge of both solids: each solid's chord copies its own edge
    /// and no section is read (the germ's two faces may share one
    /// carrier).
    AlongEdge,
    /// A section of the germ's face pair, inside a face of at least one
    /// solid.
    Section,
}

impl SegmentLane {
    /// The lane of a segment whose germ runs along an edge of both
    /// solids, or not.
    fn of<T: geom_core::Real>(germ: &HalfGerm<T>) -> Self {
        match (germ.a_locus, germ.b_locus) {
            (super::Locus::OnEdge(_), super::Locus::OnEdge(_)) => Self::AlongEdge,
            _ => Self::Section,
        }
    }
}

/// Every pair record with its germs, none consumed, in `red.null_pairs`
/// order.
fn open_records<T: Decide>(red: &BooleanReduction<T>) -> Result<Vec<OpenRecord<T>>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    // Per-operand germ maps (edge keys are body-lineage-scoped — one
    // map across both bodies would collide).
    let mut a_by_edge: SecondaryMap<EdgeKey, [HalfGerm<T>; 2]> = SecondaryMap::new();
    let mut b_by_edge: SecondaryMap<EdgeKey, [HalfGerm<T>; 2]> = SecondaryMap::new();
    for r in &red.null_edges {
        match r.operand {
            Operand::A => a_by_edge.insert(r.edge, r.germs),
            Operand::B => b_by_edge.insert(r.edge, r.germs),
        };
    }
    red.null_pairs
        .iter()
        .map(|p| {
            let a = *a_by_edge
                .get(p.a_edge)
                .ok_or(desync("pair A edge without a germ record"))?;
            let b = *b_by_edge
                .get(p.b_edge)
                .ok_or(desync("pair B edge without a germ record"))?;
            Ok(OpenRecord {
                a_edge: p.a_edge,
                b_edge: p.b_edge,
                a: [(a[0], false), (a[1], false)],
                b: [(b[0], false), (b[1], false)],
            })
        })
        .collect()
}

/// **Every segment the join builds**: the matched ones
/// ([`matched_records`]), then each one-site loop
/// ([`one_site_segments`]).
pub(super) fn join_segments<T: Decide>(
    red: &BooleanReduction<T>,
    band: Band,
) -> Result<Vec<SectionSegment<T>>, BooleanError> {
    let (mut segments, open) = matched_records(red, band)?;
    segments.extend(one_site_segments(red, &open, band)?);
    Ok(segments)
}

/// **The section segments** (module docs): [`find_match`] to
/// quiescence over the pair records, each match consuming its two germ
/// slots in both solids, with the records as the matching left them.
/// Germ slots no match consumed are the join's loose ends, so every
/// germ is consumed exactly when there are as many segments as records.
/// Reads the records and the annotated clones' site points; the surgery
/// changes neither.
#[allow(clippy::type_complexity)] // (segments, records)
fn matched_records<T: Decide>(
    red: &BooleanReduction<T>,
    band: Band,
) -> Result<(Vec<SectionSegment<T>>, Vec<OpenRecord<T>>), BooleanError> {
    let (sa, sb) = (Sides::new(red, Operand::A), Sides::new(red, Operand::B));
    let mut open = open_records(red)?;
    let mut segments = Vec::new();
    while let Some((entry, cand)) = find_match(&open, red, &sa, &sb, band)? {
        let germ = open[entry.0].a[entry.1].0;
        for (r, slot) in [entry, cand] {
            open[r].a[slot].1 = true;
            open[r].b[slot].1 = true;
        }
        segments.push(SectionSegment {
            ends: [entry, cand],
            germ,
            lane: SegmentLane::of(&germ),
        });
    }
    Ok((segments, open))
}

/// **Whether `rec` is a one-site record**: neither of its germs is
/// used, both name one locus on both operands, and that locus pair's
/// section is curved, so the record's germs, at one site of one loop of
/// it, can only match each other. Its A germ and the loop its branch
/// names, `None` for a pair the section table holds, whose section is
/// read by its components and not by a frame
/// ([`super::section_table`]).
#[allow(clippy::type_complexity)] // (germ, its loop)
fn one_site<T: Decide>(
    red: &BooleanReduction<T>,
    rec: &OpenRecord<T>,
    band: Band,
) -> Result<Option<(HalfGerm<T>, Option<SectionLoop<T>>)>, BooleanError> {
    let [(g0, used0), (g1, used1)] = rec.a;
    if used0 || used1 || g0.a_locus != g1.a_locus || g0.b_locus != g1.b_locus {
        return Ok(None);
    }
    if g0.a_locus.edge().is_none()
        && g0.b_locus.edge().is_none()
        && red
            .sections
            .held((&red.a, g0.a_face), (&red.b, g0.b_face))
            .is_some()
    {
        return Ok(Some((g0, None)));
    }
    let Some(loops) = germ_section_loops((&red.a, &red.b), &g0, band)? else {
        return Ok(None);
    };
    Ok(Some((g0, Some(loops.pick(g0.branch)?))))
}

/// The site of an A germ.
fn germ_site<T: Decide>(
    red: &BooleanReduction<T>,
    germ: &HalfGerm<T>,
) -> Result<Point3<T>, BooleanError> {
    red.a
        .half_edge_start_point(germ.he)
        .ok_or(BooleanError::JoinDesync {
            what: "germ site has no point",
        })
}

/// **The section loops with one site**: a closed section loop whose
/// one site holds both of its germs, one each way round it, on one pair
/// record. The loop is one segment, from that site round the whole loop
/// back to it ([`crate::chord_join`]'s self-loop chord): its two ends are
/// the record's two slots. Two shapes close at one site
/// ([`closes_at_its_site`]):
///
/// - **a transverse section of a one-face closed wall** that crosses the
///   wall's wrap edge (both its halves bound that face) once, piercing
///   the partner's planar face there;
/// - **a closed edge lying in the partner** (a one-segment circle, both
///   of whose ends are its one vertex): the loop is that edge, and the
///   site is its vertex. The partner holds it inside a planar or a
///   spherical face, whose interior the site pierces; inside a planar
///   face whose boundary passes through the site, where the loop only
///   touches that boundary (two rims tangent at their vertices); or
///   along a closed edge of its own through the same site.
///
/// Read from the records [`matched_records`]' quiescence left. Every
/// other [`one_site`] record is left for the
/// [`SplitJoinError::SingleSiteSectionLoop`] refusal: a pierce of a face
/// the joiner cannot side a whole conic on, a site on both operands'
/// wrap edges, a site where a transverse section or a pierce meets an
/// edge of another face, or a closed edge meets one in a curved face,
/// and a closed edge that is not a circle.
///
/// Where a record is taken, three things are kernel bugs, refused as
/// [`BooleanError::JoinDesync`]: another record naming its locus pair
/// on the same loop of its section ([`SectionLoops`]; each loop is one
/// closed curve, which the record's two germs then cover alone, while a
/// section of two loops, the plane×torus pair's, carries the other
/// loop's records); B germs whose loci are not the A germs'; and two
/// germs that agree in sense or turn one way round the loop (insertion
/// mints a null edge's two germs facing apart).
fn one_site_segments<T: Decide>(
    red: &BooleanReduction<T>,
    open: &[OpenRecord<T>],
    band: Band,
) -> Result<Vec<SectionSegment<T>>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let (sa, sb) = (Sides::new(red, Operand::A), Sides::new(red, Operand::B));
    let mut out = Vec::new();
    for (r, rec) in open.iter().enumerate() {
        // A pair the section table holds closes at no one site this join
        // builds: every closing [`closes_at_its_site`] admits has a planar
        // partner or a closed edge. Its record is left to the refusal.
        let Some((g0, Some(frame))) = one_site(red, rec, band)? else {
            continue;
        };
        if !closes_at_its_site(red, &g0, rec.b[0].0.he, band)? {
            continue;
        }
        for (_, other) in open.iter().enumerate().filter(|&(o, _)| o != r) {
            for (g, _) in &other.a {
                if g.a_locus == g0.a_locus && g.b_locus == g0.b_locus && g.branch == g0.branch {
                    return Err(desync(
                        "a one-site record shares its loop with another record",
                    ));
                }
            }
        }
        let g1 = rec.a[1].0;
        if rec
            .b
            .iter()
            .any(|(g, _)| g.a_locus != g0.a_locus || g.b_locus != g0.b_locus)
        {
            return Err(desync("B germ loci differ at a one-site record"));
        }
        if sa.is_up(&red.a, g0.he)? == sa.is_up(&red.a, g1.he)?
            || sb.is_up(&red.b, rec.b[0].0.he)? == sb.is_up(&red.b, rec.b[1].0.he)?
        {
            return Err(desync("a one-site record's two germs agree in sense"));
        }
        let p = germ_site(red, &g0)?;
        let senses = (
            rotational_sense(frame.turn(), p, g0.dir, band)?,
            rotational_sense(frame.turn(), p, g1.dir, band)?,
        );
        if senses.0 == senses.1 {
            return Err(desync(
                "a one-site record's two germs turn one way round its loop",
            ));
        }
        out.push(SectionSegment {
            ends: [(r, 0), (r, 1)],
            germ: g0,
            lane: SegmentLane::of(&g0),
        });
    }
    Ok(out)
}

/// **Whether a one-site record's loop closes at its site**
/// ([`one_site_segments`]), read from what the site is in the germ's cell
/// on each operand ([`site_cell`]): a wrap edge against a pierce of a
/// planar face, or a closed circle against a pierce of a face
/// [`RimSide::of`] sides, against a vertex on the boundary of a planar
/// face it stands on from the other side ([`stands_on`]), or against
/// another closed circle. At a boundary vertex the germ's
/// cell is the face, so the circle leaves the vertex into the face and
/// returns to it; the joiner mints its copy in that face as one chord
/// from the vertex round to itself, dividing the face. A closed edge of
/// any other carrier has no copy, and its loop is refused. A pierce is
/// joined in its face as a ring of its own
/// ([`crate::chord_join::ChordJoiner`]'s lone ring), on the face's
/// [`RimSide`]; the transverse section's conic is the wall's, not a
/// plane section, so only a planar face takes it.
fn closes_at_its_site<T: Decide>(
    red: &BooleanReduction<T>,
    g0: &HalfGerm<T>,
    b_he: HalfEdgeKey,
    band: Band,
) -> Result<bool, BooleanError> {
    let rim =
        |body: &Body<T>, f: FaceKey| Ok::<_, BooleanError>(RimSide::of(face_surface(body, f)?.1));
    Ok(
        match (
            site_cell(&red.a, g0.a_locus, g0.he)?,
            site_cell(&red.b, g0.b_locus, b_he)?,
        ) {
            (SiteCell::Wrap, SiteCell::Pierce(f)) => rim(&red.b, f)? == Some(RimSide::Plane),
            (SiteCell::Pierce(f), SiteCell::Wrap) => rim(&red.a, f)? == Some(RimSide::Plane),
            (SiteCell::Closed(Some(_)), SiteCell::Pierce(f)) => rim(&red.b, f)?.is_some(),
            (SiteCell::Pierce(f), SiteCell::Closed(Some(_))) => rim(&red.a, f)?.is_some(),
            (SiteCell::Closed(Some(_)), SiteCell::Closed(Some(_))) => true,
            (SiteCell::Closed(Some(e)), SiteCell::Other(f)) => {
                rim(&red.b, f)? == Some(RimSide::Plane) && stands_on(red, (Operand::A, e), f, band)?
            }
            (SiteCell::Other(f), SiteCell::Closed(Some(e))) => {
                rim(&red.a, f)? == Some(RimSide::Plane) && stands_on(red, (Operand::B, e), f, band)?
            }
            _ => false,
        },
    )
}

/// **Whether closed edge `e` of operand `x` stands on planar face `f` of
/// the other from the other side**: a planar face of `x` along `e`,
/// which lies in `f`'s plane, faces the other way as the op reads them
/// (`bool_rim_face_opposed`, the normals' cosine levered at that face's
/// reach from `e`; a subtrahend's normals read reversed). A rim
/// standing on the face (a boss on a plate, or over a hole whose rim it
/// touches) is joined at the boundary vertex. A rim lying flush in it,
/// both faces facing one way, is not: joined there, the piece of the
/// face the rim walls off is bounded by the copy's half that runs
/// against the rim's own
/// (`work/cleave/a-flush-closed-rim-at-its-vertex-is-joined-reversed.md`).
fn stands_on<T: Decide>(
    red: &BooleanReduction<T>,
    (x_is, e): (Operand, EdgeKey),
    f: FaceKey,
    band: Band,
) -> Result<bool, BooleanError> {
    let (x, y) = match x_is {
        Operand::A => (&red.a, &red.b),
        Operand::B => (&red.b, &red.a),
    };
    // A ∖ B reads B's complement, whose faces face the other way.
    let flips = red.op == super::BooleanOp::Subtract;
    use super::solid_contain::{PointInSolidError, face_plane};
    let desync = |what| BooleanError::JoinDesync { what };
    let plane = |body: &Body<T>, face: FaceKey| match face_plane(body, face) {
        Ok((_, n)) => Ok(Some(n)),
        Err(PointInSolidError::KindUnsupported { .. }) => Ok(None),
        Err(e) => Err(BooleanError::Containment(e)),
    };
    let Some(n_f) = plane(y, f)? else {
        return Ok(false);
    };
    let edge = x
        .get_edge(e)
        .ok_or(desync("a germ's closed edge no longer resolves"))?;
    let curve = x
        .edge_curve_linked(e, edge)
        .certified()
        .ok_or(desync("a germ's closed edge is scaffolding"))?;
    for h in [edge.he_plus, edge.he_minus] {
        let face = x
            .face_of_half_edge(h)
            .ok_or(desync("a germ's closed edge has a half with no face"))?;
        let Some(n) = plane(x, face)? else {
            continue;
        };
        let arm = crate::wedge_reach(x, [face, face], curve.carrier(), curve.params());
        let cosine = if flips { -n.dot(n_f) } else { n.dot(n_f) };
        return Ok(
            decide("bool_rim_face_opposed", Margin::levered(cosine, arm), band).map_err(
                |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag),
            )? == Sign::Negative,
        );
    }
    Ok(false)
}

/// What a null half's site is in its germ's cell on one operand
/// ([`site_cell`]).
#[derive(Clone, Copy, PartialEq, Eq)]
enum SiteCell {
    /// Inside a face, every real edge there a piece of the face's wrap
    /// edge.
    Wrap,
    /// Inside this face, no real edge reaching it: a pierce of the face's
    /// interior.
    Pierce(FaceKey),
    /// Along an edge: a closed one, both of whose ends are the site
    /// ([`site_cell`]): the edge where it is a circle, the one closed
    /// carrier the joiner copies (`crate::chord_join`'s
    /// `closed_edge_spec`), `None` for any other.
    Closed(Option<EdgeKey>),
    /// Inside this face, a real edge there bounding another face: the
    /// site is a vertex of the face's boundary.
    Other(FaceKey),
}

/// [`SiteCell`] of the site of null half `he`, its germ's cell `locus`.
/// A one-site record's two germs both run along an `OnEdge` locus from
/// the one site, one each way, and an edge leaves each of its ends one
/// way only, so that edge's two ends are both the site: it is closed
/// there.
fn site_cell<T: Decide>(
    body: &Body<T>,
    locus: super::Locus,
    he: HalfEdgeKey,
) -> Result<SiteCell, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let face = match locus {
        super::Locus::InFace(face) => face,
        super::Locus::OnEdge(e) => {
            let edge = body
                .get_edge(e)
                .ok_or(desync("a germ's closed edge no longer resolves"))?;
            let circle = body
                .edge_curve_linked(e, edge)
                .certified()
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }));
            return Ok(SiteCell::Closed(circle.then_some(e)));
        }
    };
    let start = body
        .get_half_edge(he)
        .ok_or(desync("germ half no longer resolves"))?
        .start;
    let end = body
        .half_edge_end(he)
        .ok_or(desync("germ half no longer resolves"))?;
    let site =
        crate::chord_join::null_site(body, &[start, end]).map_err(super::sectors::stale_site)?;
    let mut cell = SiteCell::Pierce(face);
    for v in site {
        for k in body.edges_of_vertex_linked(v) {
            let e = body
                .get_edge(k)
                .ok_or(desync("a site edge no longer resolves"))?;
            if body.edge_curve_linked(k, e).null_scaffold().is_some() {
                continue;
            }
            for h in [e.he_plus, e.he_minus] {
                if body.face_of_half_edge(h) != Some(face) {
                    return Ok(SiteCell::Other(face));
                }
            }
            cell = SiteCell::Wrap;
        }
    }
    Ok(cell)
}

/// [`join_segments`] read as sites: the pair-record count and each
/// segment's two germ sites on the A clone (one point twice for a
/// one-site loop).
#[cfg(any(test, feature = "test-support"))]
pub(super) fn segment_sites(
    red: &BooleanReduction<f64>,
    band: Band,
) -> Result<super::SegmentSites, BooleanError> {
    let open = open_records(red)?;
    let site = |(r, slot): Slot| {
        red.a
            .half_edge_start_point(open[r].a[slot].0.he)
            .ok_or(BooleanError::JoinDesync {
                what: "germ site has no point",
            })
    };
    let segments = join_segments(red, band)?
        .iter()
        .map(|s| Ok([site(s.ends[0])?, site(s.ends[1])?]))
        .collect::<Result<_, BooleanError>>()?;
    Ok((open.len(), segments))
}

/// `bool_connect`'s product: the completed pairs plus the per-operand
/// chord-mef fragment logs (naming emission — `(new face,
/// divided-from face)` at call-time CLONE keys, A rows in the A-clone
/// arena, B rows in the B-clone arena pre-graft).
pub(super) struct Connected {
    pub completed: Vec<CompletedPolygonPair>,
    pub a_fragments: crate::lineage::FaceDivisions,
    pub b_fragments: crate::lineage::FaceDivisions,
    /// Each operand's faces the join killed, `(killed face, face holding
    /// its region)` in that operand's clone keys, A's then B's
    /// (`ChordJoiner::take_absorbed`).
    pub absorbed: [crate::lineage::Substitutions<FaceKey>; 2],
    /// Each operand's chords along its own edges, `(edge, chord)` in
    /// that operand's clone keys, A's then B's: every chord a segment
    /// whose locus on that operand is an edge minted runs along that
    /// edge between its two ends, so it holds the edge's interior
    /// where the op drops the edge ([`SolidJoin::join`]).
    pub along: [Vec<(EdgeKey, EdgeKey)>; 2],
    /// The cuts dividing each dropped coincident face where its fate
    /// changes, made after the join ([`super::fate::cut`]).
    pub fates: super::fate::FateCuts,
}

/// The lockstep joining sweep (module docs). Mutates both annotated
/// clones in `red` in place; returns the completed polygon pairs in
/// completion order, with [`NullFacePair::Boolean`] records set.
pub(super) fn bool_connect<T: Decide + crate::props::AtRestPolicy>(
    red: &mut BooleanReduction<T>,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    band: Band,
    tol: Tol,
) -> Result<Connected, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    crossing_germs_have_their_reading(red, (a_pristine, b_pristine), band)?;
    let segments = join_segments(red, band)?;
    let pairs = segments
        .iter()
        .map(|seg| SegmentPair::read(red, seg, band))
        .collect::<Result<Vec<_>, BooleanError>>()?;
    // The bisector lane's planes, read once per wall pair before any
    // surgery, so every segment on one branch rides one plane.
    let bisectors = bisector_planes(red, &segments, &pairs, band)?;
    let seats = carried_seats(red, &open_records(red)?, &segments, &pairs, band)?;
    let mut sa = SolidJoin::new(red, Operand::A, band);
    let mut sb = SolidJoin::new(red, Operand::B, band);
    let mut completed: Vec<UnresolvedPair> = Vec::new();
    let mut open = open_records(red)?;
    // A one-site record no segment takes is a closed section loop the
    // join does not build. Its section is read here, in the state the
    // segments were matched in, and refused once the segments are
    // joined, before the loose ends are counted.
    let taken: std::collections::BTreeSet<usize> = segments
        .iter()
        .flat_map(|s| s.ends.map(|(r, _)| r))
        .collect();
    let mut single_site = 0;
    for (r, rec) in open.iter().enumerate() {
        if !taken.contains(&r) && one_site(red, rec, band)?.is_some() {
            single_site += 1;
        }
    }

    for (seg, pair) in segments.iter().zip(pairs) {
        let [(entry, entry_slot), (cand, cand_slot)] = seg.ends;
        let (ea, ra) = (open[entry].a[entry_slot].0.he, open[cand].a[cand_slot].0.he);
        let (eb, rb) = (open[entry].b[entry_slot].0.he, open[cand].b[cand_slot].0.he);
        // Still-loose halves (choose_roles' separation constraint —
        // a role order must not wall a pending half off from its match
        // partner). The chosen halves themselves are being consumed.
        let (mut a_loose, mut b_loose) =
            loose_partners(&open, &segments, red, &sa.sides, &sb.sides, band)?;
        a_loose.remove(ea);
        a_loose.remove(ra);
        b_loose.remove(eb);
        b_loose.remove(rb);
        // Curved germ pairs: a pair whose section the operation's
        // section table carries (cylinder × sphere) cuts both sides'
        // chords from one carried component, whatever the kinds. Every
        // other pair's chord lane comes
        // from the germ FACE PAIR — plane×plane takes the straight-chord
        // lane with the partner's plane as its section; plane×cylinder,
        // plane×sphere (M5 S13) and plane×cone mint the C5 section conic
        // on both sides (the wall side with the germ plane as context,
        // the planar side against the partner wall), each taking the arc the
        // matched germs' directions name, so both solids take the SAME
        // geometric arc; a sphere pair rides the wall-side lane on both
        // sides against its radical plane, and so does a parallel
        // cylinder pair, whose chords there are rulings. Any other pair
        // takes the edge-plane lane where its germ runs along a conic
        // edge of one solid in a curved face of the other, and the rest
        // refuse typed citing their C5 routing (per-arm, C12.1).
        let germ = seg.germ;
        let SegmentPair {
            a: (ka, ga),
            b: (kb, gb),
            arm,
            carried,
            site_loop,
        } = pair;
        // The germ faces' SURFACES, deliberately unoriented (S10): what
        // the curved lanes below take from a plane germ is a point and
        // a chart normal — a section datum, an operation input whose
        // normal names a chart, not a material side. The plane as a
        // point set (and hence the section conic and the auxiliary
        // surface minted for it) is identical under
        // a sense flip, so folding the sense in here would rewrite an
        // input that never meant "outward"; the created faces' own
        // orientation comes from the joiner's stored winding.
        // The lanes read the germ normal's components as direction
        // cosines, so its length is decided here, at the read: a plane
        // carrier's normal is unit only by the surfaces' at-rest
        // convention, which no tier certifies. A normal with no decided
        // length is an operand whose plane breaks that convention. The
        // length is a pure number, levered by a LOWER bound on the reach
        // it is consumed over: the ball through the two germ sites this
        // join connects, read from the plane's origin. The lanes consume
        // the normal along the section between those sites, which can
        // reach past the ball, and a shorter arm never decides a length
        // positive that the full reach would not. Read only by the arms
        // that mint a germ normal, before they mutate the body.
        // A one-site loop's two sites are one, and its chord runs round
        // the whole loop: the ball holds, in the other's place, the
        // site's antipode through a conic's centre, a point of the conic
        // too, or a loop's own ball where its centre is no centre of
        // symmetry.
        let antipode = if let Some(lp) = site_loop {
            let p = red
                .a
                .half_edge_start_point(ea)
                .ok_or(desync("germ site has no point"))?;
            Some(
                lp.ball
                    .unwrap_or_else(|| geom_brep::ExtentBall::point(lp.center + (lp.center - p))),
            )
        } else {
            None
        };
        let germ_reach = |body: &Body<T>| -> Result<geom_brep::ExtentBall<T>, BooleanError> {
            let site = |he| {
                body.half_edge_start_point(he)
                    .map(geom_brep::ExtentBall::point)
                    .ok_or(desync("germ site has no point"))
            };
            let other = match antipode {
                Some(ball) => ball,
                None => site(ra)?,
            };
            geom_brep::ExtentBall::enclosing(&[site(ea)?, other])
                .ok_or(desync("a join has no germ sites"))
        };
        let germ_normal = |reach: geom_brep::ExtentBall<T>, origin: Point3<T>, n: Vec3<T>| {
            UnitVec3::levered(n, BOOL_GERM_PLANE_NORMAL, band, reach.lever_from(origin)).map_err(
                |_| desync("a germ plane's normal has no decided length (a broken plane carrier)"),
            )
        };
        // Each solid's adjacency skip asks whether the edge between its
        // two halves is the one the segment's locus on that solid names.
        let (seg_a, seg_b) = (germ.a_locus.edge(), germ.b_locus.edge());
        use geom::Surface as Sf;
        // No wired join arm for this germ pair (a coaxial cylinder pair,
        // which the section door owns, plane×NURBS behind PR 7b): typed, citing
        // the face that holds the segment's entry half now (an earlier
        // segment's `mef` may have divided the germ's recorded face) and
        // that face's kind.
        let no_arm = || {
            let (operand, body, he) = if matches!(ga, Sf::Plane { .. }) {
                (Operand::B, &red.b, eb)
            } else {
                (Operand::A, &red.a, ea)
            };
            let face = match crate::chord_join::he_face(body, he) {
                Ok(face) => face,
                Err(e) => return BooleanError::from(e),
            };
            match face_surface(body, face) {
                Ok((_, s)) => BooleanError::CurvedBooleanUnsupported {
                    operand,
                    face,
                    kind: s.kind(),
                },
                Err(e) => e,
            }
        };
        // A segment along a conic edge of one solid, inside the other's
        // curved face, has a section whatever the pair's kinds: the
        // edge's plane. It is the lane of a pair no other arm takes. A
        // planar face holding the edge lies in that plane, which cuts
        // it in no curve.
        // A one-sided germ only: a germ along an edge of both solids
        // takes no kind lane (`SegmentLane::AlongEdge`, below). A pair
        // with a kind arm keeps it ([`kind_arm`]).
        let edge_lane = |arm: KindArm| -> Result<GermLane<T>, BooleanError> {
            if arm != KindArm::None {
                return Err(desync(
                    "the edge-plane lane asked of a pair with a kind arm",
                ));
            }
            let Some(along) = along_edge_conic((&red.a, &red.b), &germ)? else {
                return Err(no_arm());
            };
            let holder = match along.on {
                Operand::A => &gb,
                Operand::B => &ga,
            };
            let (geom::Curve3::Circle { center, axis, .. }
            | geom::Curve3::Ellipse { center, axis, .. }) = along.carrier
            else {
                return Err(no_arm());
            };
            if matches!(holder, Sf::Plane { .. }) {
                return Err(no_arm());
            }
            Ok(GermLane::EdgePlane {
                on: along.on,
                root: along.root,
                plane: (center, germ_normal(germ_reach(&red.a)?, center, axis)?),
            })
        };
        let lane = if let Some(k) = carried {
            Some(GermLane::Carried(k))
        } else if let Some(arm) = arm {
            let plane_of = |s: &geom::Surface<T>| match *s {
                Sf::Plane { origin, normal, .. } => {
                    Ok((origin, germ_normal(germ_reach(&red.a)?, origin, normal)?))
                }
                _ => Err(desync("a planar kind arm's face is not a plane")),
            };
            Some(match arm {
                KindArm::Planar => GermLane::Planar,
                KindArm::PlaneWall => GermLane::PlaneWall(plane_of(&ga)?),
                KindArm::WallPlane => GermLane::WallPlane(plane_of(&gb)?),
                // **The sphere pair rides its RADICAL PLANE.** Two spheres
                // meet in a circle lying in the one plane both residuals
                // agree on, so on each side the section is that sphere cut
                // by that plane: the wall-side chord lane of the plane×sphere
                // pair above, run on BOTH sides against the same plane. The
                // plane is computed from the pair's own C5 Circle, once per
                // germ, so the two sides' chords are sections of one datum;
                // each body's aux copy of it is keyed by the two spheres it
                // depends on ([`AuxDatum::Radical`]).
                KindArm::Spheres => {
                    let radical = match geom_brep::sphere_sphere_section(&ga, &gb, band) {
                        Ok(geom_brep::SphereSphereSection::Circle(geom::Curve3::Circle {
                            center,
                            axis,
                            ..
                        })) => (center, germ_normal(germ_reach(&red.a)?, center, axis)?),
                        Ok(_) => {
                            return Err(desync(
                                "germ pair's sphere×sphere section is not a circle",
                            ));
                        }
                        Err(geom_brep::SectionError::Escalated(diag)) => {
                            return Err(BooleanError::coincidence(
                                Coincide::Section,
                                DeclarationRead::Moot,
                                diag,
                            ));
                        }
                        Err(_) => return Err(desync("germ pair's section refused at join time")),
                    };
                    GermLane::Radical(radical)
                }
                // **A parallel cylinder pair rides its RADICAL PLANE
                // too** ([`parallel_radical_plane`]): the walls meet in
                // two rulings, both in that plane, and the plane cuts
                // each wall in exactly its two rulings, so each side's
                // chord is the plane × cylinder ruling arm's straight
                // chord on its own wall, read at the germ sites it
                // joins. The kind arm took the pair only with parallel
                // axes; coaxial walls have no such plane and keep the
                // refusal below.
                KindArm::Cylinders => {
                    let reach = geom_brep::Reach::Ball(germ_reach(&red.a)?);
                    match parallel_radical_plane(&ga, &gb, &reach, band)? {
                        Some(radical) => GermLane::Rulings(radical),
                        None => return Err(no_arm()),
                    }
                }
                // **An equal cylinder pair with intersecting axes rides
                // its germ's BISECTOR PLANE**: the walls meet in two
                // ellipses, one in each bisector plane of the axes, and
                // each plane cuts each wall in exactly its ellipse, so
                // each side's chord is the plane × cylinder arm's arc on
                // its own wall, as the sphere pair's is on its radical
                // plane. The germ's branch names the plane.
                KindArm::Bisector => {
                    let lp = bisectors
                        .get(&(ka, kb))
                        .ok_or(desync("a bisector segment's wall pair has no planes"))?
                        .pick(germ.branch)?;
                    GermLane::Bisector((
                        lp.center,
                        germ_normal(germ_reach(&red.a)?, lp.center, lp.axis)?,
                    ))
                }
                arm @ KindArm::None => edge_lane(arm)?,
            })
        } else {
            None
        };
        // Role order per solid, derived independently (module docs,
        // the seam-orientation discipline): cross-solid seam orientation is
        // carried by the sense attributes alone; role order only
        // decides the face partition of a same-loop split, which each
        // solid resolves against its OWN geometry. A cylinder face's
        // ring lane winds its island on the face's chart, closed along
        // this solid's section plane, before the curve; a planar, sphere
        // or cone face's ring lane waits on the segment's curve, below;
        // an along-edge segment reads no section.
        let (a_closure, b_closure) = lane.map_or(
            (
                RingClosure::AlongEdge(Operand::A),
                RingClosure::AlongEdge(Operand::B),
            ),
            GermLane::ring_closures,
        );
        sa.joiner.place_pending(&mut red.a, (ea, ra))?;
        sb.joiner.place_pending(&mut red.b, (eb, rb))?;
        let a_lane = choose_roles(
            (&red.a, Operand::A),
            (ea, ra),
            &a_loose,
            seg_a,
            a_closure,
            band,
        )
        .map_err(|e| pinch_ray_refusal(e, red, Operand::A, (ea, ra)))?;
        let b_lane = choose_roles(
            (&red.b, Operand::B),
            (eb, rb),
            &b_loose,
            seg_b,
            b_closure,
            band,
        )
        .map_err(|e| pinch_ray_refusal(e, red, Operand::B, (eb, rb)))?;
        let (a_halves, b_halves) = (a_lane.curve_order((ea, ra)), b_lane.curve_order((eb, rb)));
        // The section's direction at each site, toward the other: the
        // germ the matched half faces, per operand.
        let germ_at = |on_a: bool| {
            let at = |(r, slot): (usize, usize)| {
                let g = if on_a {
                    open[r].a[slot].0
                } else {
                    open[r].b[slot].0
                };
                (g.he, g.dir)
            };
            Leave {
                at: [at(seg.ends[0]), at(seg.ends[1])],
                datum: Datum::Germ,
            }
        };
        let (leave_a, leave_b) = (germ_at(true), germ_at(false));
        // A chord's datum is looked up by the half it starts at
        // (`Leave::from`), so the two halves each curve is minted
        // between must be the matched germs' own: `choose_roles` orders
        // them and picks no other.
        for (halves, leave) in [(a_halves, &leave_a), (b_halves, &leave_b)] {
            let named = leave.at.map(|(h, _)| h);
            if !(named.contains(&halves.0) && named.contains(&halves.1)) {
                return Err(BooleanError::JoinDesync {
                    what: "a segment's curve halves are not its matched germs' own",
                });
            }
        }
        let (plan_a, plan_b) = (
            sa.plan(&red.a, a_halves, seg_a)?,
            sb.plan(&red.b, b_halves, seg_b)?,
        );
        let (curve_a, curve_b) = match lane {
            None => (
                sa.curve(&mut red.a, &plan_a, JoinLane::AlongEdge, leave_a)?,
                sb.curve(&mut red.b, &plan_b, JoinLane::AlongEdge, leave_b)?,
            ),
            // The chord runs along the two planes' common line: straight
            // on both sides.
            Some(GermLane::Planar) => (
                sa.curve(&mut red.a, &plan_a, JoinLane::Planar, leave_a)?,
                sb.curve(&mut red.b, &plan_b, JoinLane::Planar, leave_b)?,
            ),
            // Both sides' chords are arcs of one carried component, seated
            // at its seat site ([`carried_seats`]).
            Some(GermLane::Carried(k)) => {
                let at = seats
                    .get(&(ka, kb, k))
                    .copied()
                    .ok_or(desync("a carried segment's component has no seat"))?;
                let (component, lever) = carried_component(red, &germ, k, at)?;
                let component = (&component, lever);
                (
                    sa.carried_curve(
                        &mut red.a,
                        &plan_a,
                        (component, gb.clone()),
                        AuxDatum::Partner(germ.b_face),
                        leave_a,
                    )?,
                    sb.carried_curve(
                        &mut red.b,
                        &plan_b,
                        (component, ga.clone()),
                        AuxDatum::Partner(germ.a_face),
                        leave_b,
                    )?,
                )
            }
            Some(GermLane::PlaneWall(plane)) => (
                sa.bool_planar_curve(
                    &mut red.a,
                    &plan_a,
                    (gb.clone(), cone_window(&red.b, germ.b_face, &gb, band)),
                    germ.b_face,
                    germ.branch,
                    leave_a,
                )?,
                sb.split_curve(
                    &mut red.b,
                    &plan_b,
                    plane,
                    AuxDatum::Partner(germ.a_face),
                    germ.branch,
                    leave_b,
                )?,
            ),
            Some(GermLane::WallPlane(plane)) => {
                let curve_a = sa.split_curve(
                    &mut red.a,
                    &plan_a,
                    plane,
                    AuxDatum::Partner(germ.b_face),
                    germ.branch,
                    leave_a,
                )?;
                let window = cone_window(&red.a, germ.a_face, &ga, band);
                let curve_b = sb.bool_planar_curve(
                    &mut red.b,
                    &plan_b,
                    (ga.clone(), window),
                    germ.a_face,
                    germ.branch,
                    leave_b,
                )?;
                (curve_a, curve_b)
            }
            Some(GermLane::Radical(radical)) => {
                let datum = |own, partner| AuxDatum::Radical { own, partner };
                (
                    sa.split_curve(
                        &mut red.a,
                        &plan_a,
                        radical,
                        datum(ka, kb),
                        germ.branch,
                        leave_a,
                    )?,
                    sb.split_curve(
                        &mut red.b,
                        &plan_b,
                        radical,
                        datum(kb, ka),
                        germ.branch,
                        leave_b,
                    )?,
                )
            }
            // Each wall's own section table decides its rulings against
            // the radical plane ([`rulings_are_straight`]). Its refusals
            // come after both split lanes have run, which may have minted
            // an aux plane into either body: a refusal ends the operation
            // and the reduction is dropped, so nothing reads them.
            Some(GermLane::Rulings(radical)) => {
                let datum = |own, partner| AuxDatum::Radical { own, partner };
                let curves = (
                    sa.split_curve(
                        &mut red.a,
                        &plan_a,
                        radical,
                        datum(ka, kb),
                        germ.branch,
                        leave_a,
                    )?,
                    sb.split_curve(
                        &mut red.b,
                        &plan_b,
                        radical,
                        datum(kb, ka),
                        germ.branch,
                        leave_b,
                    )?,
                );
                rulings_are_straight([
                    (&curves.0, Operand::A, germ.a_face),
                    (&curves.1, Operand::B, germ.b_face),
                ])?;
                curves
            }
            Some(GermLane::Bisector(plane)) => {
                let datum = |own, partner| AuxDatum::Bisector {
                    own,
                    partner,
                    branch: germ.branch,
                };
                (
                    sa.split_curve(
                        &mut red.a,
                        &plan_a,
                        plane,
                        datum(ka, kb),
                        germ.branch,
                        leave_a,
                    )?,
                    sb.split_curve(
                        &mut red.b,
                        &plan_b,
                        plane,
                        datum(kb, ka),
                        germ.branch,
                        leave_b,
                    )?,
                )
            }
            Some(GermLane::EdgePlane { on, root, plane }) => {
                let datum = AuxDatum::EdgePlane { on, root };
                match on {
                    Operand::A => (
                        sa.curve(&mut red.a, &plan_a, JoinLane::AlongEdge, leave_a)?,
                        sb.split_curve(&mut red.b, &plan_b, plane, datum, germ.branch, leave_b)?,
                    ),
                    Operand::B => (
                        sa.split_curve(&mut red.a, &plan_a, plane, datum, germ.branch, leave_a)?,
                        sb.curve(&mut red.b, &plan_b, JoinLane::AlongEdge, leave_b)?,
                    ),
                }
            }
        };
        // The ring lane's order, wound with the curve.
        let a_roles = a_lane
            .resolve((&red.a, Operand::A), (ea, ra), &a_loose, &curve_a, band)
            .or_else(|e| glued_slit_roles(e, red, Operand::A, (ea, ra), &a_loose))?;
        let b_roles = b_lane
            .resolve((&red.b, Operand::B), (eb, rb), &b_loose, &curve_b, band)
            .or_else(|e| glued_slit_roles(e, red, Operand::B, (eb, rb), &b_loose))?;
        sa.join(&mut red.a, a_roles, &plan_a, &curve_a, seg_a, tol)?;
        sb.join(&mut red.b, b_roles, &plan_b, &curve_b, seg_b, tol)?;
        for (r, slot) in seg.ends {
            open[r].a[slot].1 = true;
            open[r].b[slot].1 = true;
        }
        // Cut fully-used records, higher index first.
        let mut done: Vec<usize> = [entry, cand]
            .into_iter()
            .filter(|&i| open[i].fully_used())
            .collect();
        done.sort_unstable_by(|x, y| y.cmp(x));
        // A one-site loop's segment ends at one record twice.
        done.dedup();
        for i in done {
            let r = open[i];
            cut_pair(
                red,
                &mut sa,
                &mut sb,
                &mut completed,
                r.a_edge,
                r.b_edge,
                tol,
            )?;
        }
    }

    if single_site != 0 {
        return Err(BooleanError::from(SplitJoinError::SingleSiteSectionLoop {
            count: single_site,
        }));
    }

    // ---- Role resolution, deferred to quiescence: mid-join the
    // region faces are not yet final (an operand face pierced by TWO
    // polygons still spans both seams while the first completes), so
    // probing happens only after every polygon has been cut. ----
    let mut resolved = Vec::with_capacity(completed.len());
    for c in completed {
        let (a_in_loop, a_out_loop) = resolve_roles_geometric(
            red,
            Operand::A,
            b_pristine,
            c.a_face,
            (c.a_outer, c.a_ring),
            band,
            tol,
        )?;
        let (b_in_loop, b_out_loop) = resolve_roles_geometric(
            red,
            Operand::B,
            a_pristine,
            c.b_face,
            (c.b_outer, c.b_ring),
            band,
            tol,
        )?;
        red.a.set_null_face_pair(
            c.a_face,
            NullFacePair::Boolean {
                in_copy: a_in_loop,
                out_copy: a_out_loop,
            },
        )?;
        red.b.set_null_face_pair(
            c.b_face,
            NullFacePair::Boolean {
                in_copy: b_in_loop,
                out_copy: b_out_loop,
            },
        )?;
        resolved.push(CompletedPolygonPair {
            a_face: c.a_face,
            a_in_loop,
            a_out_loop,
            b_face: c.b_face,
            b_in_loop,
            b_out_loop,
        });
    }
    let completed = resolved;

    let leftovers: usize = open
        .iter()
        .map(|r| r.a.iter().filter(|(_, u)| !u).count())
        .sum();
    if leftovers != 0 {
        return Err(BooleanError::from(SplitJoinError::UnpairedLooseEnds {
            count: leftovers,
        }));
    }
    // A pierce ring deferred at a pinch is placed by the join that
    // reaches it; one still pending after every join has run never was.
    sa.joiner.finish(&red.a)?;
    sb.joiner.finish(&red.b)?;
    Ok(Connected {
        completed,
        a_fragments: sa.joiner.take_fragments(),
        b_fragments: sb.joiner.take_fragments(),
        absorbed: [sa.joiner.take_absorbed(), sb.joiner.take_absorbed()],
        along: [sa.along, sb.along],
        fates: super::fate::FateCuts::default(),
    })
}

/// **The bisector lane's section, per wall pair**, keyed by the two
/// walls' surface keys, A's first: each segment of
/// [`KindArm::Bisector`] reads its face pair's two bisector-plane
/// ellipses ([`face_pair_section_loops`]) before any surgery divides a
/// face, the first reading of a wall pair standing for every segment on
/// it, so the chords of one branch lie in one plane.
fn bisector_planes<T: Decide>(
    red: &BooleanReduction<T>,
    segments: &[SectionSegment<T>],
    pairs: &[SegmentPair<T>],
    band: Band,
) -> Result<
    BTreeMap<(crate::geometry::SurfaceKey, crate::geometry::SurfaceKey), SectionLoops<T>>,
    BooleanError,
> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut planes = BTreeMap::new();
    for (seg, pair) in segments.iter().zip(pairs) {
        if pair.arm != Some(KindArm::Bisector) {
            continue;
        }
        let (fa, fb) = (seg.germ.a_face, seg.germ.b_face);
        let key = (pair.a.0, pair.b.0);
        if let std::collections::btree_map::Entry::Vacant(slot) = planes.entry(key) {
            let loops = face_pair_section_loops((&red.a, fa), (&red.b, fb), band)?.ok_or(
                desync("a bisector segment's face pair has no section loops"),
            )?;
            slot.insert(loops);
        }
    }
    Ok(planes)
}

/// **Every germ that rides an ellipse of an equal cylinder pair finds
/// the pair read so on the operands**: the reading its coincidence rows
/// are recorded from (`glue::section_door`) and its nodes are minted from
/// (`node::mint_nodes`), [`crossing_cylinder_reading`] on the unmutated
/// operands' faces. The germ's branch was decided where the germ was
/// minted, on the working bodies; a pair that reading finds crossing
/// and this one does not would build without its rows or its nodes.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] for such a pair; the reading's own
/// refusals.
fn crossing_germs_have_their_reading<T: Decide>(
    red: &BooleanReduction<T>,
    pristine: (&Body<T>, &Body<T>),
    band: Band,
) -> Result<(), BooleanError> {
    let wall = |body: &Body<T>, f: FaceKey| {
        face_surface(body, f).is_ok_and(|(_, s)| super::node::is_wall(s))
    };
    let mut read = std::collections::BTreeSet::new();
    for germ in red.null_edges_of(Operand::A).flat_map(|r| r.germs) {
        let (super::Locus::InFace(fa), super::Locus::InFace(fb)) = (germ.a_locus, germ.b_locus)
        else {
            continue;
        };
        if matches!(
            germ.branch,
            super::Branch::Only | super::Branch::Component(_)
        ) || !wall(&red.a, fa)
            || !wall(&red.b, fb)
            || !read.insert((fa, fb))
        {
            continue;
        }
        if crossing_cylinder_reading((pristine.0, fa), (pristine.1, fb), band)?.is_none() {
            return Err(BooleanError::JoinDesync {
                what: "a germ rides an equal cylinder pair's ellipse where the pair's own \
                       reading finds none",
            });
        }
    }
    Ok(())
}

/// **How far a partner site is along the germ line**, seen from the
/// germ: which half-turn of the section conic it lies in, the chord to
/// it, and on a conic the turn that reaches it. Ordered by
/// [`nearer_along`] among one germ's candidates and by [`nearer`]
/// between germs.
#[derive(Clone, Copy)]
struct Reach<T: geom_core::Real> {
    arm: GermArm,
    chord: T,
    turn: Option<Turn<T>>,
    along: Option<Along<T>>,
}

/// **How far a partner's foot is along the germ's component**, in the
/// direction the germ leaves ([`along_facing`]): the carrier's parameter
/// travelled, the carrier's speed at the germ's foot, and the partner's
/// site.
#[derive(Clone, Copy)]
struct Along<T: geom_core::Real> {
    travel: T,
    speed: T,
    site: Point3<T>,
}

/// A conic germ line as its germ turns along it.
mod turn {
    use super::{BooleanError, Sense, rotational_sense};
    use geom_core::{Band, Decide, Point3, Vec3};

    /// The section's frame; the germ's site and direction, whose
    /// rotational sense says which way is ahead; then a site on the
    /// conic and the line of the conic's tangent there, which
    /// [`super::turned_past`] measures from. The site and its tangent
    /// are another germ's (a partner's, or the germ's own in
    /// [`Turn::read_from_germ`]), and that germ's direction may run
    /// either way along the tangent: [`super::turned_past`] reads the
    /// tangent only through magnitudes, so its orientation never
    /// reaches a verdict.
    ///
    /// [`Turn::facing`] is the one way to make a `Turn`, and it decides
    /// both directions' rotational senses nonzero against the frame it
    /// stores: the germ's is the sign ahead, and the site's is the
    /// divisor [`super::turned_past`] and [`super::off_conic_slack`]
    /// read.
    #[derive(Clone, Copy)]
    pub(super) struct Turn<T: geom_core::Real> {
        center: Point3<T>,
        axis: Vec3<T>,
        sense: Sense,
        germ: (Point3<T>, Vec3<T>),
        site: (Point3<T>, Vec3<T>),
    }

    impl<T: Decide> Turn<T> {
        /// **Whether two germs on one loop of a conic section face each
        /// other**, each a site and its direction, about the loop's
        /// `frame`: they bound one rotational traversal of it exactly
        /// when their [`rotational_sense`]s differ, and the turn from
        /// `germ` toward `site` is then `Some`. A radial direction
        /// refuses there ([`rotational_sense`]).
        pub(super) fn facing(
            frame: (Point3<T>, Vec3<T>),
            germ: (Point3<T>, Vec3<T>),
            site: (Point3<T>, Vec3<T>),
            band: Band,
        ) -> Result<Option<Self>, BooleanError> {
            let sense = rotational_sense(frame, germ.0, germ.1, band)?;
            if rotational_sense(frame, site.0, site.1, band)? == sense {
                return Ok(None);
            }
            Ok(Some(Turn {
                center: frame.0,
                axis: frame.1,
                sense,
                germ,
                site,
            }))
        }
    }

    impl<T: geom_core::Real> Turn<T> {
        /// The same turn read from the germ's own site.
        pub(super) fn read_from_germ(self) -> Self {
            Turn {
                site: self.germ,
                ..self
            }
        }

        /// `(center, axis)`.
        pub(super) fn frame(self) -> (Point3<T>, Vec3<T>) {
            (self.center, self.axis)
        }

        /// The germ's rotational sense.
        pub(super) fn sense(self) -> Sense {
            self.sense
        }

        /// The site measured from.
        pub(super) fn site(self) -> Point3<T> {
            self.site.0
        }

        /// The conic's tangent line at [`Turn::site`].
        pub(super) fn tangent(self) -> Vec3<T> {
            self.site.1
        }
    }
}
use turn::Turn;

/// Which half-turn of its section conic a partner site lies in, seen
/// from the germ and in the germ's own rotational sense. A straight germ
/// line has one, [`GermArm::Ahead`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum GermArm {
    /// Within the half-turn the germ runs into.
    Ahead,
    /// Past it.
    Behind,
}

/// A join coincidence the band cannot decide.
fn join_escalate(diag: geom_core::Indeterminate) -> BooleanError {
    BooleanError::coincidence(Coincide::Join, DeclarationRead::Moot, diag)
}

/// **Is `cand` nearer than `best` along ONE germ's line?** Both are
/// read from the same germ. The half-turn decides where the two differ.
/// Within one half-turn a conic orders them by the turn between them
/// ([`travel`] under `bool_join_arc_travel`, from `best`'s site): the
/// angle travelled about a centred conic's axis is monotone along it,
/// where the chord is not — on an ellipse of aspect √2 or more it peaks
/// inside the half-turn. Two sites in one half-turn are less than a
/// half-turn apart, so the side of `best`'s axis plane `cand` lies on
/// is the order. Near `best`'s site the margin reads the arc between
/// them (between there and the half-turn it keeps only the plane's
/// sign), so a tie is two sites within the band of each other along the
/// conic. A straight line, and two records at one site, take the chord.
fn nearer_along<T: Decide>(
    cand: Reach<T>,
    best: Reach<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    if let (Some(c), Some(b)) = (cand.along, best.along) {
        let one_site = decide("bool_join_chord", Margin::norm3(c.site - b.site), band)
            .map_err(join_escalate)?
            == Sign::Zero;
        if !one_site {
            let ahead = Margin::metered(c.travel - b.travel, geom_core::InfSpeed::new(c.speed));
            match decide("bool_join_foot_travel", ahead, band).map_err(join_escalate)? {
                Sign::Negative => return Ok(true),
                Sign::Positive => return Ok(false),
                Sign::Zero => {}
            }
        }
    }
    if let (true, Some(c), Some(b)) = (cand.arm == best.arm, cand.turn, best.turn) {
        let one_site = decide("bool_join_chord", Margin::norm3(c.site() - b.site()), band)
            .map_err(join_escalate)?
            == Sign::Zero;
        if !one_site {
            match travel("bool_join_arc_travel", b, c.site(), band)? {
                Sign::Negative => return Ok(true),
                Sign::Positive => return Ok(false),
                Sign::Zero => {}
            }
        }
    }
    nearer(cand, best, band)
}

/// **Which of two germs' nearest partners [`first_pair`] commits
/// first.** Each was already chosen along its own germ line
/// ([`nearer_along`]), so this picks an order among pairs rather than a
/// partner: the half-turn, then the chord (`bool_join_nearest`, metres).
fn nearer<T: Decide>(cand: Reach<T>, best: Reach<T>, band: Band) -> Result<bool, BooleanError> {
    Ok(nearest_sign(cand, best, band).map_err(join_escalate)? == Sign::Negative)
}

/// [`nearer`] where the order cannot change the matching
/// ([`first_pair`]): an in-band reading keeps the incumbent, as an
/// exact tie does (scan order, D9). Any other undecided reading — a
/// poisoned margin, an enclosure straddling the band's edge — escalates
/// as [`nearer`]'s does.
fn nearer_or_scan<T: Decide>(
    cand: Reach<T>,
    best: Reach<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    match nearest_sign(cand, best, band) {
        Ok(sign) => Ok(sign == Sign::Negative),
        Err(diag) if in_band(&diag) => Ok(false),
        Err(diag) => Err(join_escalate(diag)),
    }
}

/// Whether an undecided reading lies inside the band: a point margin,
/// or an enclosure wholly inside one sliver band.
fn in_band(diag: &geom_core::Indeterminate) -> bool {
    match diag.margin.kind() {
        geom_core::MarginKind::Value => true,
        // An interval-lane reading; one straddling the band's edge is
        // not in band.
        geom_core::MarginKind::Enclosure => diag.terminal_sliver,
        geom_core::MarginKind::Invalid => false,
    }
}

/// The half-turn, then the chord: [`nearer`]'s one reading.
fn nearest_sign<T: Decide>(
    cand: Reach<T>,
    best: Reach<T>,
    band: Band,
) -> Result<Sign, geom_core::Indeterminate> {
    if cand.arm != best.arm {
        return Ok(if cand.arm == GermArm::Ahead {
            Sign::Negative
        } else {
            Sign::Positive
        });
    }
    decide(
        "bool_join_nearest",
        Margin::of(cand.chord - best.chord),
        band,
    )
}

/// **`axis·((p − c) × v)`**, the one spelling of a conic's turn: for a
/// unit `v` along the conic at `p`, its rotational sense; for any `v`,
/// `v`'s component across the plane through the axis and `p`, times
/// `p`'s radius. Each factor's length is a scale on the whole, so the
/// axis need not be unit for a sign or for a ratio of two of these.
fn turn_of<T: geom_core::Real>(
    (center, axis): (geom_core::Point3<T>, Vec3<T>),
    p: geom_core::Point3<T>,
    v: Vec3<T>,
) -> T {
    axis.dot((p - center).cross(v))
}

/// **How far `p` has turned past `from`'s site, along the conic**: the
/// side of the plane through the conic's axis and that site `p` lies
/// on, positive on the side the germ's sense runs into, read as arc
/// length. With `t̂` the conic's unit tangent at the site, the margin is
/// `turn(p − site) / |turn(t̂)|` ([`turn_of`] at the site): the plane
/// distance over the sine of `ψ`, the angle between radius and tangent.
/// A point `s` along the conic reads `s (1 + ½ s κ cot ψ + O(s²))`, `κ`
/// the curvature there; the plane distance alone reads `s sin ψ`, which
/// falls to `2k/(k² + 1)` of `s` between the vertices of an ellipse of
/// aspect `k`. At the half-turn it is the arc again to first order, by
/// the conic's central symmetry; between, it is neither, and only its
/// sign is read there. `|turn(t̂)|` is the site's rotational sense,
/// decided nonzero when the [`Turn`] was made ([`Turn::facing`]).
/// Reading from the site cancels nothing near it.
///
/// The sites are vertices, and a vertex lies within the band of its
/// edges' carriers but not on them: each edge's endpoints are pinned to
/// its carrier within ε (`geom_brep::certify`), and a split may land on
/// a vertex already held (`Body::split_edge_onto`). A site `δ` off the
/// conic across it moves the margin by `δ cot ψ`, so between two sites
/// the reading is uncertain by [`off_conic_slack`]: [`travel`] decides
/// it against that slack. Metres: a metre vector projected across a
/// metre radius, over a unit tangent projected the same way.
fn turned_past<T: geom_core::Real>(from: Turn<T>, p: geom_core::Point3<T>) -> Margin<T> {
    let frame = from.frame();
    Margin::levered_inv(
        turn_of(frame, from.site(), p - from.site()) * from.sense().unit(),
        turn_of(frame, from.site(), from.tangent()).abs(),
    )
}

/// **What two sites' distance off the conic can move [`turned_past`]
/// by**: `2ε cot ψ` at `from`'s site, each site up to the band's `ε`
/// across the conic. Zero on a circle (`ψ` is a right angle), and
/// `ε (k² − 1)/k` at the flank of an ellipse of aspect `k`, where
/// `cot ψ` is `(k² − 1)/2k`: past the band's escalation threshold once
/// `k` passes about 10. `cot ψ` is
/// `|t̂·(site − c)|·|axis|` over `|turn(t̂)|`, the axis's length
/// cancelling with [`turn_of`]'s.
fn off_conic_slack<T: geom_core::Real>(from: Turn<T>, band: Band) -> Margin<T> {
    let (center, axis) = from.frame();
    let cot = from.tangent().dot(from.site() - center).abs() * axis.norm()
        / turn_of(from.frame(), from.site(), from.tangent()).abs();
    Margin::levered(cot + cot, T::from_f64(band.zero()))
}

/// **[`turned_past`] decided under `name`, clear of the sites' slack**:
/// a definite sign stands only where the margin's size definitely
/// exceeds [`off_conic_slack`] (`bool_join_arc_clear`), and a tie only
/// where the slack itself is within the band (`bool_join_arc_slack`).
/// Anything else is two sites the conic cannot order, and escalates.
fn travel<T: Decide>(
    name: &'static str,
    from: Turn<T>,
    p: geom_core::Point3<T>,
    band: Band,
) -> Result<Sign, BooleanError> {
    let margin = turned_past(from, p);
    let slack = off_conic_slack(from, band);
    let sign = decide(name, margin, band).map_err(join_escalate)?;
    let tie_clear = sign == Sign::Zero
        && decide("bool_join_arc_slack", slack, band).map_err(join_escalate)? == Sign::Zero;
    if !tie_clear {
        decide_positive(
            "bool_join_arc_clear",
            Margin::of(margin.value().abs() - slack.value()),
            band,
        )
        .map_err(join_escalate)?;
    }
    Ok(sign)
}

/// **Which half-turn of the germ's section conic its partner lies in**:
/// [`travel`] read from the germ's own site (`bool_join_arc_ahead`).
///
/// `Zero` reads [`GermArm::Ahead`]: an in-band distance puts the partner
/// at the germ's own azimuth (no travel) or at the half-turn, and both
/// sort ahead of every site genuinely `Behind`; within `Ahead`,
/// [`nearer_along`] orders a half-turn site by its turn like any other.
/// A tie stands only where [`travel`]'s slack is itself within the band,
/// so a partner at the half-turn reads `Ahead` on a circle and on a
/// mild ellipse, and escalates where `cot ψ` at the germ's site makes
/// the slack leave the band (from about `cot ψ = 0.5` on).
///
/// A straight germ line has no turn to read, so every partner is
/// `Ahead` and the order is the chord alone — the planar pairing.
fn germ_arm<T: Decide>(turn: Option<Turn<T>>, band: Band) -> Result<GermArm, BooleanError> {
    let Some(turn) = turn else {
        return Ok(GermArm::Ahead);
    };
    Ok(
        match travel(
            "bool_join_arc_ahead",
            turn.read_from_germ(),
            turn.site(),
            band,
        )? {
            Sign::Negative => GermArm::Behind,
            Sign::Positive | Sign::Zero => GermArm::Ahead,
        },
    )
}

/// What a conic germ with no rotational sense is: a radial germ
/// direction, malformed germ data.
const RADIAL_GERM: &str =
    "a conic germ has no rotational sense (radial germ direction — malformed germ data)";

/// Which way a conic germ turns about its section's axis.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Sense {
    /// `axis·((p − c) × dir)` positive.
    Positive,
    /// Negative.
    Negative,
}

impl Sense {
    /// `+1` or `−1`.
    fn unit<T: geom_core::Real>(self) -> T {
        match self {
            Sense::Positive => T::one(),
            Sense::Negative => T::zero() - T::one(),
        }
    }
}

/// **A conic germ's rotational sense** about its section's axis,
/// [`turn_of`] its unit direction (metres: |p − c| ~ radius), decided
/// under `bool_join_arc_facing`. A zero sense is a radial germ, a loud
/// desync ([`RADIAL_GERM`]); its in-band twin escalates as the kernel's
/// own self-check rather than as a coincidence.
fn rotational_sense<T: Decide>(
    frame: (geom_core::Point3<T>, Vec3<T>),
    p: geom_core::Point3<T>,
    dir: Vec3<T>,
    band: Band,
) -> Result<Sense, BooleanError> {
    let malformed = |diag| BooleanError::Escalated {
        decision: BooleanDecision::SelfCheck(SelfCheck::ArcFacing),
        diag,
    };
    match decide(
        "bool_join_arc_facing",
        Margin::of(turn_of(frame, p, dir)),
        band,
    )
    .map_err(malformed)?
    {
        Sign::Positive => Ok(Sense::Positive),
        Sign::Negative => Ok(Sense::Negative),
        Sign::Zero => Err(BooleanError::JoinDesync { what: RADIAL_GERM }),
    }
}

/// A germ's nearest partner so far, with the scan position it was met
/// at and the match it names.
type Met<T> = (usize, Reach<T>, (Slot, Slot));

/// The nearer of the incumbent `best` and `cand` under `nearer`; the
/// incumbent keeps a tie (deterministic scan order, D9).
fn keep_nearer<T, M>(
    best: Option<(Reach<T>, M)>,
    cand: (Reach<T>, M),
    nearer: impl Fn(Reach<T>, Reach<T>) -> Result<bool, BooleanError>,
) -> Result<Option<(Reach<T>, M)>, BooleanError>
where
    T: geom_core::Real,
{
    Ok(Some(match best {
        Some(b) if !nearer(cand.0, b.0)? => b,
        _ => cand,
    }))
}

/// `scanjoin`, germ form (module docs): among all candidate/entry slot
/// combinations whose A-side germs carry the SAME loci and whose
/// two sites mutually FACE each other along the germ line
/// (`bool_join_facing`, decided — the polygon edge's ends point at one
/// another), with OPPOSED senses in both solids (the sense theorem's
/// neighbor test), take each germ's NEAREST partner along its own line
/// ([`nearer_along`]: non-adjacent same-line sites must not be chorded
/// across an intermediate one), then one of those pairs
/// ([`first_pair`]). The B side consumes the SAME slots — slot `i`
/// of the A and B germ arrays is one spatial germ (registration doc);
/// a B-side locus or sense disagreement at matched slots is a
/// loud desync, never an alternative pairing. Zero-distance
/// combinations (distinct pair records at one coincident site) are
/// skipped — that degeneracy gate is `bool_join_chord` (margin = the
/// chord LENGTH), a separate question from `bool_join_nearest`'s
/// selection (margin = a DIFFERENCE of chord lengths), so the two
/// populations meter separately. Deterministic scan order breaks
/// exact ties (D9).
fn find_match<T: Decide>(
    open: &[OpenRecord<T>],
    red: &BooleanReduction<T>,
    sa: &Sides,
    sb: &Sides,
    band: Band,
) -> Result<Option<(Slot, Slot)>, BooleanError> {
    // Unused germ slots of one record side (half tied to germ slot —
    // the mint facing is honest data for every lane, struts included).
    fn slots<T: geom_core::Real>(side: &[(HalfGerm<T>, bool); 2]) -> Vec<usize> {
        (0..2).filter(|&g| !side[g].1).collect()
    }
    // Each germ's nearest partner along its own line, kept with the scan
    // position it was met at; then the nearest of those, met in scan
    // order, so an exact tie goes to the pair scanned first.
    let mut own: BTreeMap<Slot, Met<T>> = BTreeMap::new();
    let mut at = 0;
    for (cand, rec) in open.iter().enumerate() {
        for (entry, e) in open.iter().enumerate() {
            if entry == cand {
                continue;
            }
            for &cs in &slots(&rec.a) {
                for &es in &slots(&e.a) {
                    let Some(reach) = partners(open, red, sa, sb, (cand, cs), (entry, es), band)?
                    else {
                        continue;
                    };
                    at += 1;
                    keep_own(&mut own, (at, reach, ((entry, es), (cand, cs))), band)?;
                }
            }
        }
    }
    first_pair(own, band)
}

/// Keeps `met` as its germ's nearest partner if it is nearer along the
/// germ's line ([`nearer_along`]) than the one kept; the kept one holds a
/// tie (scan order, D9).
fn keep_own<T: Decide>(
    own: &mut BTreeMap<Slot, Met<T>>,
    met: Met<T>,
    band: Band,
) -> Result<(), BooleanError> {
    let (_, reach, (_, germ)) = met;
    match own.get(&germ) {
        Some(&(_, kept, _)) if !nearer_along(reach, kept, band)? => {}
        _ => {
            own.insert(germ, met);
        }
    }
    Ok(())
}

/// **The pair [`find_match`] commits this round**, from each germ's
/// nearest partner (`own`, keyed by germ, each naming `(partner,
/// germ)`): the nearest of those pairs ([`nearer`]), met in scan order.
///
/// **Where the order cannot change the matching, an in-band order is
/// not a refusal** ([`nearer_or_scan`]). The run then commits exactly
/// this round's pairs, or escalates, where:
///
/// - every germ's nearest partner has it as its own nearest. Committing
///   `(g, h)` then removes no other germ's nearest: a germ whose nearest
///   were `g` would be `g`'s nearest, and that is `h`;
/// - every germ is straight, so its candidates rank by chord alone and
///   each reading is the difference of two chords;
/// - the band's escalation threshold exceeds twice its zero (`K > 2`).
///   A germ's scan keeps its incumbent unless a candidate reads at least
///   `K·zero` nearer, and a tie is within `zero`. In the full scan, every
///   candidate met before the winner `w` lost to an incumbent at least
///   `K·zero` farther than `w`, so it lies at least `(K − 1)·zero`
///   beyond `w`. Every candidate met after `w` lies no nearer than
///   `w − zero`. A scan that has lost candidates other than `w` therefore
///   meets `w` against an incumbent that `K > 2` puts past a tie: `w`
///   takes the incumbency or the reading escalates, and nothing after it
///   reads `K·zero` nearer. Two chained ties span `2·zero`, which a
///   smaller `K` reads as definite.
///
/// So each germ keeps its nearest as candidates go, and each later round
/// holds the same pairs less the ones committed, still mutual. Its scans
/// may escalate under one order and not another; the matching is free
/// of the order, not the run. The order still picks which pair the
/// surgery joins first.
///
/// Elsewhere the order is read, and an in-band reading escalates.
/// Committing a pair can turn a germ whose nearest was in it to its
/// next partner, a pair this round never ranked. Along a conic,
/// [`nearer_along`] ranks by travel from the incumbent's site rather
/// than by one number per candidate.
fn first_pair<T: Decide>(
    own: BTreeMap<Slot, Met<T>>,
    band: Band,
) -> Result<Option<(Slot, Slot)>, BooleanError> {
    let free = band.escalate() > 2.0 * band.zero()
        && own.iter().all(|(germ, &(_, reach, (partner, _)))| {
            reach.turn.is_none()
                && reach.along.is_none()
                && own
                    .get(&partner)
                    .is_some_and(|&(_, _, (back, _))| back == *germ)
        });
    let order = if free { nearer_or_scan } else { nearer };
    let mut met: Vec<_> = own.into_values().collect();
    met.sort_by_key(|&(at, ..)| at);
    let mut best: Option<(Reach<T>, (Slot, Slot))> = None;
    for (_, reach, m) in met {
        best = keep_nearer(best, (reach, m), |c, b| order(c, b, band))?;
    }
    Ok(best.map(|(_, m)| m))
}

/// **Whether two unused germ slots are match partners** — the
/// criterion [`find_match`] pairs on, and the one [`loose_partners`]
/// pairs two records' halves by, so the separation constraint sees the
/// partners the matcher will take: the A germs carry the SAME loci,
/// each `OnEdge` locus at its own site ([`locus_at_site`]); the two
/// halves' senses oppose in A; the sites are distinct
/// (`bool_join_chord`) and FACE each other along the germ line (the
/// locus-aware facing); and the B germs at the same slots mirror them —
/// same loci, opposed senses — or the join has desynced. `Some(reach)`
/// when they are partners — how far the partner is along the germ's
/// line ([`Reach`]). Two slots of one record never are.
fn partners<T: Decide>(
    open: &[OpenRecord<T>],
    red: &BooleanReduction<T>,
    sa: &Sides,
    sb: &Sides,
    (cand, cs): (usize, usize),
    (entry, es): (usize, usize),
    band: Band,
) -> Result<Option<Reach<T>>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    if cand == entry {
        return Ok(None);
    }
    let point_of = |he: HalfEdgeKey| -> Result<geom_core::Point3<T>, BooleanError> {
        red.a
            .half_edge_start_point(he)
            .ok_or(desync("germ site has no point"))
    };
    let (rec, e) = (&open[cand], &open[entry]);
    let (rga, ega) = (rec.a[cs].0, e.a[es].0);
    if ega.a_locus != rga.a_locus || ega.b_locus != rga.b_locus {
        return Ok(None);
    }
    for (germ, b_germ) in [(rga, rec.b[cs].0), (ega, e.b[es].0)] {
        locus_at_site(&red.a, germ.a_locus, germ.he)?;
        locus_at_site(&red.b, germ.b_locus, b_germ.he)?;
    }
    if sa.is_up(&red.a, ega.he)? == sa.is_up(&red.a, rga.he)? {
        return Ok(None);
    }
    // Mutual facing along the germ line (spatially shared between the
    // bodies — decided on the A clone's coincident copies).
    let p_c = point_of(rga.he)?;
    let p_e = point_of(ega.he)?;
    let dist = (p_e - p_c).norm();
    match decide("bool_join_chord", Margin::of(dist), band).map_err(join_escalate)? {
        Sign::Positive => {}
        _ => return Ok(None), // coincident sites: no polygon edge
    }
    // Locus-aware mutual facing: a germ pair the section table holds
    // faces along its component by foot parameter; otherwise straight
    // germ lines take the chord test, and conic germ loci compare
    // rotational senses about the section frame, on one loop of it.
    let faced = match germ_section(red, &rga)? {
        Some(section) => along_facing(
            (&red.sections, &red.a, &red.b),
            section,
            (&rga, p_c),
            (&ega, p_e),
            band,
        )?,
        None => {
            let loops = germ_section_loops((&red.a, &red.b), &rga, band)?;
            facing(loops.as_ref(), &rga, &ega, p_c, p_e, band)?
        }
    };
    if let Faced::Apart = faced {
        return Ok(None);
    }
    // The B side at the SAME slots — mirror checks, not freedom:
    // shared-germ loci and the anti-correlation theorem make
    // disagreement a kernel bug, refused loudly.
    let (rgb, egb) = (rec.b[cs].0, e.b[es].0);
    if rgb.a_locus != rga.a_locus
        || rgb.b_locus != rga.b_locus
        || egb.a_locus != ega.a_locus
        || egb.b_locus != ega.b_locus
    {
        return Err(desync("B germ loci differ at matched slots"));
    }
    if sb.is_up(&red.b, egb.he)? == sb.is_up(&red.b, rgb.he)? {
        return Err(desync("B senses agree at a matched pair"));
    }
    let (turn, along) = match faced {
        Faced::Conic(turn) => (Some(turn), None),
        Faced::Along(along) => (None, Some(along)),
        Faced::Apart | Faced::Straight => (None, None),
    };
    Ok(Some(Reach {
        arm: germ_arm(turn, band)?,
        chord: dist,
        turn,
        along,
    }))
}

/// An `OnEdge` germ's edge runs from the germ's site: one of its ends
/// is a vertex of the site the null half facing the germ belongs to —
/// the copies null edges tie its ends to
/// ([`crate::chord_join::null_site`]); at a site holding several null
/// edges the edge may hang on another copy than this half's. A locus
/// naming an edge anywhere else is a kernel bug, refused before it is
/// matched on.
fn locus_at_site<T: Decide>(
    body: &Body<T>,
    locus: super::Locus,
    he: HalfEdgeKey,
) -> Result<(), BooleanError> {
    let super::Locus::OnEdge(edge) = locus else {
        return Ok(());
    };
    let desync = |what| BooleanError::JoinDesync { what };
    let start = |h: HalfEdgeKey| {
        body.get_half_edge(h)
            .map(|d| d.start)
            .ok_or(desync("germ half no longer resolves"))
    };
    let site = crate::chord_join::null_site(
        body,
        &[
            start(he)?,
            body.half_edge_end(he)
                .ok_or(desync("germ half no longer resolves"))?,
        ],
    )
    .map_err(super::sectors::stale_site)?;
    let (u, v) = body
        .edge_vertices(edge)
        .ok_or(desync("an OnEdge germ's edge no longer resolves"))?;
    if site.contains(&u) || site.contains(&v) {
        Ok(())
    } else {
        Err(desync("an OnEdge germ's edge is not incident to its site"))
    }
}

/// The edge a germ runs along, A's first where it runs along an edge
/// of both solids ([`along_edge_conic`]).
struct AlongEdge<T: geom_core::Real> {
    /// The solid the edge is an edge of.
    on: Operand,
    /// The edge's split root ([`Body::split_root`]).
    root: EdgeKey,
    /// The edge's certified carrier.
    carrier: geom::Curve3<T>,
}

/// **The edge a germ runs along**, read once for the frame and the
/// lane: `None` for a germ inside a face on both solids.
///
/// # Errors
///
/// `JoinDesync` where the edge or its curve no longer resolves, and
/// [`BooleanError::SplitLineage`] where its split lineage never reaches
/// a root.
fn along_edge_conic<T: Decide>(
    (a, b): (&Body<T>, &Body<T>),
    germ: &HalfGerm<T>,
) -> Result<Option<AlongEdge<T>>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let (on, body, edge) = match (germ.a_locus, germ.b_locus) {
        (super::Locus::OnEdge(e), _) => (Operand::A, a, e),
        (_, super::Locus::OnEdge(e)) => (Operand::B, b, e),
        _ => return Ok(None),
    };
    let carrier = body
        .get_edge(edge)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(crate::null::CurveGeom::certified)
        .ok_or(desync("an OnEdge germ's edge carries no certified curve"))?
        .carrier()
        .clone();
    let root = body
        .split_root(edge, |_| false)
        .map_err(BooleanError::SplitLineage)?;
    Ok(Some(AlongEdge { on, root, carrier }))
}

/// The germ pair's section loops: the loops of the section the germ
/// line lies on, each with the centre and axis it turns about, or `None`
/// when that locus is STRAIGHT — a plane×plane pair, and the degenerate
/// plane×cylinder outcomes whose loci ARE lines
/// (ParallelLines/TangentLine). [`germ_section_frame`] picks the germ's
/// own loop.
///
/// A germ along an edge of either solid lies on that edge, and its
/// frame is the edge's own ([`along_edge_conic`]), refused
/// [`BooleanError::EdgeCarrierUnsupported`] where the edge is a spiric
/// or a spline; a germ inside a face on both solids reads its face
/// pair's.
///
/// **`None` is a claim, not a default.** The caller reads it as "take
/// the straight-chord facing test", so a pair whose section arm is not
/// wired must refuse ([`BooleanError::GermFrameUnsupported`]) rather
/// than fall through to it: falling through would mint a wrong chord
/// silently for every pair the dispatch later admits. The face-pair
/// match below is therefore EXHAUSTIVE over kinds by construction.
///
/// Section escalations propagate; non-escalation classification
/// failures at match time are a desync (the germ was minted FROM this
/// pair's crossing).
fn germ_section_loops<T: Decide>(
    (a, b): (&Body<T>, &Body<T>),
    germ: &HalfGerm<T>,
    band: Band,
) -> Result<Option<SectionLoops<T>>, BooleanError> {
    // A germ along an edge lies on that edge, whatever the two faces it
    // was recorded against, which may share one carrier, or meet in no
    // conic. A spiric or spline edge has no frame here: reading the face
    // pair's for it instead would hand a coplanar pair's straight-chord
    // test a curved edge.
    if let Some(along) = along_edge_conic((a, b), germ)? {
        return match along.carrier {
            geom::Curve3::Line { .. } => Ok(None),
            geom::Curve3::Circle { center, axis, .. }
            | geom::Curve3::Ellipse { center, axis, .. } => {
                Ok(Some(SectionLoops::conic((center, axis))))
            }
            geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
                Err(BooleanError::EdgeCarrierUnsupported {
                    operand: along.on,
                    edge: along.root,
                    face: match along.on {
                        Operand::A => germ.a_face,
                        Operand::B => germ.b_face,
                    },
                    face_of: along.on,
                    site: super::EdgeCarrierSite::GermFrame,
                })
            }
        };
    }
    face_pair_section_loops((a, germ.a_face), (b, germ.b_face), band)
}

/// **The section loops of a face pair**, read where the section is
/// consumed ([`frame_reading`]) and levered at the consumed region's
/// measure ([`frame_extent`]): [`germ_section_loops`] for a germ lying
/// inside a face of each operand.
pub(super) fn face_pair_section_loops<T: Decide>(
    (body_a, face_a): (&Body<T>, FaceKey),
    (body_b, face_b): (&Body<T>, FaceKey),
    band: Band,
) -> Result<Option<SectionLoops<T>>, BooleanError> {
    let ((_, sa), (_, sb)) = (face_surface(body_a, face_a)?, face_surface(body_b, face_b)?);
    let (at, extent) = frame_lever((sa, body_a, face_a), (sb, body_b, face_b), band)?;
    pair_section_frame_at(sa, sb, at, extent, band)
        .map_err(|e| frame_refusal(e, (face_a, sa), (face_b, sb)))
}

/// **Where a face pair's frame is read, and what it is levered at**:
/// the reading point ([`frame_reading`]) and the consumed region's
/// measure from it ([`frame_extent`]), for the two faces' surfaces
/// `sa` and `sb`, which the caller resolved from those faces.
fn frame_lever<T: Decide>(
    (sa, body_a, face_a): (&geom::Surface<T>, &Body<T>, FaceKey),
    (sb, body_b, face_b): (&geom::Surface<T>, &Body<T>, FaceKey),
    band: Band,
) -> Result<(geom_core::Point3<T>, FrameExtent<T>), BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let witnesses = |body: &Body<T>, f: FaceKey| {
        super::carrier_pair::face_witnesses(body, f).unwrap_or_else(|| {
            unreachable!(
                "{}, whose surface the caller resolved, does not resolve",
                crate::entity::EntityId::Face(f)
            )
        })
    };
    let (at, span) = frame_reading(sa, sb, witnesses(body_a, face_a), witnesses(body_b, face_b))
        .map_err(desync)?;
    let extent =
        frame_extent((sa, body_a, face_a), (sb, body_b, face_b), at, span, band).map_err(desync)?;
    Ok((at, extent))
}

/// **The germ's own loop** of its pair's section
/// ([`germ_section_loops`]), the one its branch names.
fn germ_section_frame<T: Decide>(
    red: &BooleanReduction<T>,
    germ: &HalfGerm<T>,
    band: Band,
) -> Result<Option<SectionLoop<T>>, BooleanError> {
    let Some(loops) = germ_section_loops((&red.a, &red.b), germ, band)? else {
        return Ok(None);
    };
    Ok(Some(loops.pick(germ.branch)?))
}

/// **Where the germ frame reads its axis rows, and its span**: the
/// centre of the curved face's boundary vertices (the first face's
/// where both are curved), a point among those the section is consumed
/// at, so the gap is read where the section is and not where a
/// carrier's origin is stored; and, for the cylinder pair, the span of
/// both walls' boundary vertices (the diameter of their ball), which a
/// wall's axial extent ends at. [`pair_section_frame_at`] levers the
/// cylinder pair at the longer of the larger radius and this span, and
/// the plane×cylinder pair at its wall's axial extent from the reading
/// point ([`frame_extent`]). No lever is a ball chosen around the faces
/// (`geom_brep::Reach`'s module docs).
///
/// # Errors
///
/// A face pair with no boundary vertices.
#[allow(clippy::type_complexity)] // (reading point, span) — one reading
pub(super) fn frame_reading<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    on_a: Vec<geom_core::Point3<T>>,
    on_b: Vec<geom_core::Point3<T>>,
) -> Result<(geom_core::Point3<T>, Option<T>), &'static str> {
    let ball_of = |points: Vec<geom_core::Point3<T>>| {
        let points: Vec<geom_brep::ExtentBall<T>> = points
            .into_iter()
            .map(geom_brep::ExtentBall::point)
            .collect();
        geom_brep::ExtentBall::enclosing(&points).ok_or("a germ face pair has no boundary vertices")
    };
    let curved = match sa {
        geom::Surface::Plane { .. } => on_b.clone(),
        _ => on_a.clone(),
    };
    let at = ball_of(curved)?.center();
    let span = match (sa, sb) {
        (geom::Surface::Cylinder { .. }, geom::Surface::Cylinder { .. }) => {
            let both = ball_of(on_a.into_iter().chain(on_b).collect())?;
            Some(both.radius() + both.radius())
        }
        _ => None,
    };
    Ok((at, span))
}

/// **What [`pair_section_frame_at`] levers a pair at**: the consumed
/// region's measure, taken from the reading point `at`.
#[derive(Clone, Debug)]
pub(super) enum FrameExtent<T: geom_core::Real> {
    /// Nothing measured: the pair is levered by its radii.
    Radii,
    /// A cylinder pair's walls' span ([`frame_reading`]).
    Span(T),
    /// A plane×cylinder pair's wall face: how far it reaches either way
    /// along its axis from `at`, and its farthest distance from `at`.
    Wall {
        /// How far the face reaches from `at` against the axis.
        below: T,
        /// How far the face reaches from `at` along the axis.
        above: T,
        /// The face's farthest distance from `at`, each edge read over
        /// the span it holds.
        across: T,
        /// The same with each edge levered round its whole carrier, which
        /// the germ also reads
        /// ([`agreed_section`](crate::chord_join::agreed_section)).
        round: T,
    },
    /// A plane×torus pair's torus face: its farthest distance from `at`,
    /// each edge levered round its whole carrier
    /// ([`face_reach_from`](crate::splitting::rules::face_reach_from)).
    Reach(T),
    /// A plane×cone pair's cone face, against the cone's axis
    /// ([`cone_face_reach`](crate::splitting::rules::cone_face_reach)).
    Cone(geom_brep::Reach<T>),
}

/// **The consumed region's measure [`pair_section_frame_at`] levers at**,
/// taken from the reading point `at`. A plane×cylinder pair's section
/// lies on the wall face, which hands the table its axial range from
/// `at` ([`face_axial_range`](crate::splitting::rules::face_axial_range),
/// its curved edges included) and its farthest distance from `at`, read
/// from the rulings' hinge ([`geom_brep::Reach::Face`]): each edge over
/// the span it holds
/// ([`face_reach_from`](crate::splitting::rules::face_reach_from)) and
/// round its whole carrier
/// ([`face_reach_round_from`](crate::splitting::rules::face_reach_round_from)),
/// which [`agreed_section`](crate::chord_join::agreed_section) reads
/// together. A plane×cone pair's section lies on the cone face, which
/// hands the table its range along the cone's axis from `at` and its
/// farthest distance from `at`: the measures the cone's split lane
/// reads the same section at. A plane×torus pair's lies on the torus
/// face, which hands its farthest distance from `at`, the one measure
/// the torus's split lane levers it at. Every other pair takes the
/// walls' `span`.
///
/// # Errors
///
/// A wall whose outer loop is a lone vertex, which has no extent.
pub(super) fn frame_extent<T: Decide>(
    (sa, body_a, face_a): (&geom::Surface<T>, &Body<T>, FaceKey),
    (sb, body_b, face_b): (&geom::Surface<T>, &Body<T>, FaceKey),
    at: geom_core::Point3<T>,
    span: Option<T>,
    band: Band,
) -> Result<FrameExtent<T>, &'static str> {
    let (body, face, axis) = match (sa, sb) {
        (geom::Surface::Plane { .. }, geom::Surface::Cylinder { axis, .. }) => {
            (body_b, face_b, *axis)
        }
        (geom::Surface::Cylinder { axis, .. }, geom::Surface::Plane { .. }) => {
            (body_a, face_a, *axis)
        }
        (geom::Surface::Plane { .. }, cone @ geom::Surface::Cone { .. }) => {
            return cone_reach((body_b, face_b), (body_a, face_a), at, cone, band);
        }
        (cone @ geom::Surface::Cone { .. }, geom::Surface::Plane { .. }) => {
            return cone_reach((body_a, face_a), (body_b, face_b), at, cone, band);
        }
        (geom::Surface::Plane { .. }, geom::Surface::Torus { .. }) => {
            return face_reach(body_b, face_b, at);
        }
        (geom::Surface::Torus { .. }, geom::Surface::Plane { .. }) => {
            return face_reach(body_a, face_a, at);
        }
        _ => return Ok(span.map_or(FrameExtent::Radii, FrameExtent::Span)),
    };
    let lone = |_| LONE_WALL;
    let (below, above) =
        crate::splitting::rules::face_axial_range(body, face, at, axis).map_err(lone)?;
    let across = crate::splitting::rules::face_reach_from(body, face, at).map_err(lone)?;
    let round = crate::splitting::rules::face_reach_round_from(body, face, at).map_err(lone)?;
    Ok(FrameExtent::Wall {
        below,
        above,
        across,
        round,
    })
}

/// [`frame_extent`]'s refusal: a wall with no extent to lever at.
const LONE_WALL: &str = "a germ wall's outer loop is a lone vertex";

/// The window along a cone wall's axis that `face` reaches over
/// ([`cone_axial_window`](crate::splitting::rules::cone_axial_window)):
/// the partner's face, which the planar side's section lies in, or the
/// planar face, which the frame's does. `None` off a cone.
fn cone_window<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    wall: &geom::Surface<T>,
    band: Band,
) -> Option<(T, T)> {
    if !matches!(wall, geom::Surface::Cone { .. }) {
        return None;
    }
    // The one refusal is a face whose outer loop is a lone vertex, which
    // has no extent to narrow by: the reader's own face's window holds
    // the section without it, and that face's own lone loop is refused
    // where its reach is measured.
    crate::splitting::rules::cone_axial_window(body, face, wall, band).ok()
}

/// [`frame_extent`]'s plane×cone arm: the cone face's reach from `at`,
/// its window narrowed to the plane face's, as the planar side's chord
/// reads the same section ([`JoinLane::BoolPlanar`]).
fn cone_reach<T: Decide>(
    (body, face): (&Body<T>, FaceKey),
    (plane_body, plane_face): (&Body<T>, FaceKey),
    at: geom_core::Point3<T>,
    cone: &geom::Surface<T>,
    band: Band,
) -> Result<FrameExtent<T>, &'static str> {
    let within = cone_window(plane_body, plane_face, cone, band);
    crate::splitting::rules::cone_face_reach(body, face, at, (cone, within), band)
        .map(FrameExtent::Cone)
        .map_err(|_| LONE_WALL)
}

/// [`frame_extent`]'s plane×torus arm: the torus face's reach from `at`.
fn face_reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: geom_core::Point3<T>,
) -> Result<FrameExtent<T>, &'static str> {
    crate::splitting::rules::face_reach_from(body, face, at)
        .map(FrameExtent::Reach)
        .map_err(|_| LONE_WALL)
}

/// **The rulings lane's chords are rulings**: each wall's split lane,
/// run against the pair's radical plane, minted the straight chord.
///
/// A wall the plane only touches has a tangent chord, a line on the
/// ruling whatever description the must-carry rule gave it: the walls
/// touch along one ruling, undeclared contact that no arm here takes,
/// refused `CurvedBooleanUnsupported` naming that wall. Any other curve
/// is a conic, a plane parallel to both axes as the kind arm read them
/// ([`segment_arm`]) that a wall's table did not read so, which is a
/// desync.
///
/// No pose is known to reach either refusal through a public door:
/// walls that touch along a ruling meet the coincidence ladder and the
/// crossing layer first, and the kind arm reads the axes parallel before
/// the plane is taken. The rows in `radical_plane_rows` hand it the
/// curves directly.
fn rulings_are_straight<T: geom_core::Real>(
    sides: [(&SegmentCurve<T>, Operand, FaceKey); 2],
) -> Result<(), BooleanError> {
    for (curve, operand, face) in sides {
        match curve.spec().map(|spec| &spec.carrier) {
            None => {}
            Some(geom::Curve3::Line { .. }) => {
                return Err(BooleanError::CurvedBooleanUnsupported {
                    operand,
                    face,
                    kind: geom::SurfaceKind::Cylinder,
                });
            }
            Some(_) => {
                return Err(BooleanError::JoinDesync {
                    what: "a parallel cylinder pair's radical plane cut a wall in a conic",
                });
            }
        }
    }
    Ok(())
}

/// The K funnel name of a parallel cylinder pair's axis offset: the
/// distance between the two axes, which is the length of the radical
/// plane's normal before it is normalized.
const BOOL_JOIN_CC_AXIS_OFFSET: &str = "bool_join_cc_axis_offset";

/// **The radical plane of a parallel cylinder pair**, as a section datum
/// (a point and a chart normal), or `None` for coaxial walls.
///
/// A point's power against a cylinder is its squared distance from the
/// axis less the squared radius; with the axes parallel the two powers
/// differ by a linear function, so the points of equal power form a
/// plane parallel to both axes. Both rulings the walls meet in have
/// power zero against both, so they lie in it. Its normal is the axis
/// offset `w` (from `own`'s axis to `partner`'s, perpendicular to
/// `own`'s), and it stands `x = (d² + r₁² − r₂²) / 2d` along it from
/// `own`'s axis, `d = |w|`.
///
/// `w` is read between the axes' feet at `reach`, where the chords are
/// consumed ([`geom_brep::parallel_axes_at`]). The axes are parallel
/// only within the band, so the offset moves along them by the tilt
/// times the distance from where it is read; read at a stored origin
/// standing far along an axis, it names a plane the walls do not meet
/// in at the reach.
///
/// The offset's length is decided ([`BOOL_JOIN_CC_AXIS_OFFSET`]), metres
/// against the band: a zero offset is a coaxial pair, whose walls meet
/// nowhere or everywhere and name no plane, and is `None`; an offset in
/// the band is the walls' coincidence undecided. Whether the walls cross
/// is not asked here: each wall's own section table answers it against
/// this plane, as two rulings, one, or none.
///
/// The axes are read as unit and parallel, as the kind arm read them
/// before taking the pair ([`segment_arm`]).
#[allow(clippy::type_complexity)] // (plane point, plane normal) — one datum tuple
pub(super) fn parallel_radical_plane<T: Decide>(
    own: &geom::Surface<T>,
    partner: &geom::Surface<T>,
    reach: &geom_brep::Reach<T>,
    band: Band,
) -> Result<Option<(Point3<T>, UnitVec3<T>)>, BooleanError> {
    let (
        geom::Surface::Cylinder {
            origin: o1,
            axis: a1,
            radius: r1,
            ..
        },
        geom::Surface::Cylinder {
            origin: o2,
            axis: a2,
            radius: r2,
            ..
        },
    ) = (own, partner)
    else {
        return Err(BooleanError::JoinDesync {
            what: "a cylinder pair's radical plane asked of a pair that is not two cylinders",
        });
    };
    let geom_brep::ParallelAxes {
        foot1, d_vec: w, ..
    } = geom_brep::parallel_axes_at(reach, (*o1, *a1), (*o2, *a2));
    let normal = match UnitVec3::new(w, BOOL_JOIN_CC_AXIS_OFFSET, band) {
        Ok(n) => n,
        Err(geom_core::UnitVec3Error::Degenerate | geom_core::UnitVec3Error::UnderflowedLength) => {
            return Ok(None);
        }
        Err(geom_core::UnitVec3Error::Escalated(diag)) => {
            return Err(BooleanError::coincidence(
                Coincide::Section,
                DeclarationRead::Moot,
                diag,
            ));
        }
        Err(geom_core::UnitVec3Error::NonFiniteLength) => {
            return Err(BooleanError::JoinDesync {
                what: "a parallel cylinder pair's axis offset has no finite length",
            });
        }
    };
    let d = w.norm();
    let x = (d.powi(2) + r1.powi(2) - r2.powi(2)) / (d + d);
    Ok(Some((foot1 + normal.get() * x, normal)))
}

/// The Boolean's refusal for a germ pair's frame refusal, the pair
/// being the A face `a` and the B face `b` with their surfaces: an
/// escalation refuses as the decision its reading carries
/// ([`FrameError::Escalated`]), which no declaration settles.
pub(super) fn frame_refusal<T: geom_core::Real>(
    e: FrameError,
    a: (FaceKey, &geom::Surface<T>),
    b: (FaceKey, &geom::Surface<T>),
) -> BooleanError {
    match e {
        FrameError::Escalated { decision, diag } => BooleanError::Escalated { decision, diag },
        FrameError::Desync(what) => BooleanError::JoinDesync { what },
        FrameError::NoArm => BooleanError::GermFrameUnsupported {
            a_face: a.0,
            a_kind: a.1.kind(),
            b_face: b.0,
            b_kind: b.1.kind(),
        },
        FrameError::OpenConic { conic } => BooleanError::GermSectionOpenConic {
            a_face: a.0,
            a_kind: a.1.kind(),
            b_face: b.0,
            b_kind: b.1.kind(),
            conic,
        },
        FrameError::DegenerateTorus {
            convention,
            verdict,
        } => {
            let (operand, face) = if matches!(a.1, geom::Surface::Torus { .. }) {
                (Operand::A, a.0)
            } else {
                (Operand::B, b.0)
            };
            BooleanError::DegenerateTorus {
                operand,
                face,
                convention,
                verdict,
            }
        }
        FrameError::Spiric { loops, section } => BooleanError::GermSectionSpiric {
            a_face: a.0,
            a_kind: a.1.kind(),
            b_face: b.0,
            b_kind: b.1.kind(),
            loops,
            section,
        },
    }
}

/// Why [`pair_section_frame_at`] could not name a frame. The keys and
/// bodies live at the call site, so this carries none of them: the
/// dispatch is a statement about the two surfaces, never about the
/// arenas.
pub(super) enum FrameError {
    /// A reading landed in the sliver band, refused as `decision`: a
    /// section predicate as the faces' section undecided
    /// ([`FrameError::section`]), a cylinder pair's axes as
    /// [`BooleanDecision::CylinderAxes`] ([`cylinder_pair_axes`]), an
    /// operand's radius guard as [`BooleanDecision::Radius`]
    /// ([`geom_brep::SectionError::RadiusEscalated`]).
    Escalated {
        /// The decision the reading refuses as.
        decision: BooleanDecision,
        /// The reading's diagnostics.
        diag: geom_core::Indeterminate,
    },
    /// The classification contradicted the germ that was minted from
    /// it — a lockstep failure, not a frontier.
    Desync(&'static str),
    /// No frame: the kind pair has no section arm, or an armed pair's
    /// pose has none (the cylinder × sphere node, a pose not
    /// definitely off the axis, or a cylinder pair whose locus is a
    /// space quartic: skew axes, or intersecting axes at unequal radii).
    NoArm,
    /// The section is an open conic, whose arcs are NURBS carriers the
    /// Boolean has no crossing layer for.
    OpenConic {
        /// The conic.
        conic: geom_brep::OpenConic,
    },
    /// The pair's torus is decided outside its ring convention
    /// (`R > r > 0`): the Boolean's [`BooleanError::DegenerateTorus`],
    /// on the torus face. Its in-band sibling is
    /// [`FrameError::Escalated`] as [`BooleanDecision::Torus`].
    DegenerateTorus {
        /// The half of the convention that refused.
        convention: geom_brep::TorusConvention,
        /// Its decided verdict.
        verdict: geom_brep::recourse::Refused,
    },
    /// A plane cuts a torus in a general spiric quartic, which the
    /// table routes to its unbuilt general rung.
    Spiric {
        /// The loops the section certificate reads.
        loops: super::SpiricLoops,
        /// The table's refusal.
        section: geom_brep::SectionError,
    },
}

impl FrameError {
    /// A section predicate's in-band reading: where the two faces'
    /// surfaces meet, undecided, which no declaration settles.
    fn section(diag: geom_core::Indeterminate) -> Self {
        Self::Escalated {
            decision: BooleanDecision::Coincidence(Coincide::Section, DeclarationRead::Moot),
            diag,
        }
    }
}

/// [`pair_section_frame_at`] for a pair handed without its faces: the
/// cylinder pair levered at `span` where it is given, every pair
/// otherwise at its radii.
#[cfg(test)]
#[allow(clippy::type_complexity)] // (conic center, conic axis) — one frame tuple
pub(super) fn pair_section_frame<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    at: geom_core::Point3<T>,
    span: Option<T>,
    band: Band,
) -> Result<Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>, FrameError> {
    let extent = span.map_or(FrameExtent::Radii, FrameExtent::Span);
    one_loop(pair_section_frame_at(sa, sb, at, extent, band))
}

/// Whether `sa` and `sb`, handed without their faces, are an equal
/// cylinder pair with meeting axes: the dispatch serves the two
/// bisector-plane ellipses, told apart by the bisector split.
#[cfg(test)]
pub(super) fn serves_cylinder_nodes<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    at: geom_core::Point3<T>,
    band: Band,
) -> bool {
    matches!(
        pair_section_frame_at(sa, sb, at, FrameExtent::Radii, band),
        Ok(Some(SectionLoops::Two {
            split: LoopSplit::Bisector { .. },
            ..
        }))
    )
}

/// A one-loop section's frame as its `(centre, axis)`, for the rows of
/// the pairs whose sections are one loop.
#[cfg(test)]
#[allow(clippy::type_complexity, clippy::panic)] // (conic center, conic axis) — one frame tuple
pub(super) fn one_loop<T: geom_core::Real>(
    frame: Result<Option<SectionLoops<T>>, FrameError>,
) -> Result<Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>, FrameError> {
    frame.map(|f| {
        f.map(|loops| match loops {
            SectionLoops::One(lp) => lp.turn(),
            SectionLoops::Two { .. } => panic!("a two-loop section read as one loop"),
        })
    })
}

/// **One loop of a germ pair's section**: the centre and axis it turns
/// about strictly monotonically, which is all the rotational-sense facing
/// test and the turn ranking read of a frame, and where it reaches.
#[derive(Clone, Copy, Debug)]
pub(super) struct SectionLoop<T: geom_core::Real> {
    /// The centre it turns about.
    pub(super) center: geom_core::Point3<T>,
    /// The axis it turns about.
    pub(super) axis: geom_core::Vec3<T>,
    /// `None` where `center` is a centre of symmetry of the loop, so that
    /// a point's antipode through it lies on the loop too; the loop's
    /// ball otherwise.
    pub(super) ball: Option<geom_brep::ExtentBall<T>>,
}

impl<T: geom_core::Real> SectionLoop<T> {
    /// `(center, axis)`, the frame [`rotational_sense`] reads.
    pub(super) fn turn(self) -> (geom_core::Point3<T>, geom_core::Vec3<T>) {
        (self.center, self.axis)
    }
}

/// **The loops of a germ pair's section**: one, or two that a margin at
/// a site of the section tells apart ([`LoopSplit`]).
#[derive(Clone, Copy, Debug)]
pub(super) enum SectionLoops<T: geom_core::Real> {
    /// A section of one loop, or of loops that share one frame.
    One(SectionLoop<T>),
    /// Two loops with frames of their own.
    Two {
        /// The loop on the split's positive side, then its negative one.
        loops: [SectionLoop<T>; 2],
        /// The margin that tells them apart.
        split: LoopSplit<T>,
    },
}

impl<T: Decide> SectionLoops<T> {
    /// A conic's one loop, its centre a centre of symmetry.
    fn conic((center, axis): (geom_core::Point3<T>, geom_core::Vec3<T>)) -> Self {
        Self::One(SectionLoop {
            center,
            axis,
            ball: None,
        })
    }

    /// **The branch the site `p` lies on** ([`super::Branch`]), the
    /// one reading of the split, taken where a germ is minted
    /// ([`germ_branch`]). The split's margin is definite at every point
    /// of a served section off its nodes: on the plane×torus pair's
    /// loops everywhere (`bool_join_section_loop`), so a site whose
    /// margin is `Zero` there is on neither loop, a desync; on the
    /// bisector ellipses everywhere but where they cross
    /// (`bool_germ_bisector_branch`), so a `Zero` there is a germ at a
    /// node, `Ok(None)`. An undecided margin escalates.
    pub(super) fn branch_at(
        &self,
        p: geom_core::Point3<T>,
        band: Band,
    ) -> Result<Option<super::Branch>, BooleanError> {
        let split = match self {
            Self::One(_) => return Ok(Some(super::Branch::Only)),
            Self::Two { split, .. } => split,
        };
        let name = match split {
            LoopSplit::Bisector { .. } => "bool_germ_bisector_branch",
            LoopSplit::Side { .. } | LoopSplit::Radius { .. } => "bool_join_section_loop",
        };
        match decide(name, Margin::of(split.margin(p)), band) {
            Ok(Sign::Positive) => Ok(Some(super::Branch::Positive)),
            Ok(Sign::Negative) => Ok(Some(super::Branch::Negative)),
            Ok(Sign::Zero) => match split {
                LoopSplit::Bisector { .. } => Ok(None),
                LoopSplit::Side { .. } | LoopSplit::Radius { .. } => {
                    Err(BooleanError::JoinDesync {
                        what: "a germ site lies on neither loop of its section (its loop margin is \
                           in the band)",
                    })
                }
            },
            Err(diag) => Err(join_escalate(diag)),
        }
    }

    /// **The loop a germ's branch names**: the one loop of a one-loop
    /// section, the split's positive or negative loop of a two-loop
    /// one. A branch of the other shape is a desync: the germ was
    /// minted against this face pair's section.
    pub(super) fn pick(&self, branch: super::Branch) -> Result<SectionLoop<T>, BooleanError> {
        match (self, branch) {
            (Self::One(lp), super::Branch::Only) => Ok(*lp),
            (Self::Two { loops, .. }, super::Branch::Positive) => Ok(loops[0]),
            (Self::Two { loops, .. }, super::Branch::Negative) => Ok(loops[1]),
            (_, super::Branch::Component(_)) => Err(BooleanError::JoinDesync {
                what: "a germ riding a carried section's component was read against a frame's \
                       loops",
            }),
            _ => Err(BooleanError::JoinDesync {
                what: "a germ's branch does not name a loop of its face pair's section",
            }),
        }
    }
}

/// **The loop a germ rides** ([`super::Branch`]), decided where the germ
/// is minted, at its site `site` (the A copy's point) with its face pair
/// `(face_a, face_b)` and its loci: a germ along an edge rides that
/// edge's own curve ([`germ_section_loops`]). A germ inside a face of
/// each operand on a pair the operation's section table holds rides the
/// component of the pair's carried section its site has its foot on
/// ([`geom_brep::CarriedSection::foot`], [`super::Branch::Component`]),
/// the pair carried here if no germ carried it before. Of the
/// rest, every kind pair but the two whose sections are two loops —
/// plane×torus and the intersecting equal-radius cylinder pair — rides
/// its section's one loop or a straight line, read at the join;
/// otherwise the face pair's section is read
/// ([`face_pair_section_loops`]) and its split decided at the site
/// ([`SectionLoops::branch_at`]).
///
/// # Errors
///
/// The table's refusals of the pair, carried ([`section_unbuilt`]), and
/// the site's foot ([`site_refusal`]); the section's own refusals ([`frame_refusal`]),
/// the split's escalation and desync, and [`BooleanError::NodeUnbuilt`]
/// for a germ on a node of the cylinder pair's section, where the two
/// ellipses cross: an operand's edge through the node.
pub(super) fn germ_branch<T: Decide>(
    sections: &super::section_table::SectionTable<T>,
    (body_a, face_a): (&Body<T>, FaceKey),
    (body_b, face_b): (&Body<T>, FaceKey),
    (locus_a, locus_b): (super::Locus, super::Locus),
    site: Point3<T>,
    band: Band,
) -> Result<super::Branch, BooleanError> {
    use geom::SurfaceKind::{Cylinder, Plane, Torus};
    if locus_a.edge().is_some() || locus_b.edge().is_some() {
        return Ok(super::Branch::Only);
    }
    if let Some(carried) = sections.carried((body_a, face_a), (body_b, face_b)) {
        return carried
            .map_err(|why| faces_section_unbuilt(why, (body_a, face_a), (body_b, face_b)))?
            .foot(site, band)
            .map(|foot| super::Branch::Component(foot.component))
            .map_err(site_refusal);
    }
    let kind = |body: &Body<T>, f: FaceKey| face_surface(body, f).ok().map(|(_, s)| s.kind());
    // A face that does not resolve reads no section here; the join
    // refuses it where it reads the pair.
    match (kind(body_a, face_a), kind(body_b, face_b)) {
        (Some(Plane), Some(Torus))
        | (Some(Torus), Some(Plane))
        | (Some(Cylinder), Some(Cylinder)) => {}
        _ => return Ok(super::Branch::Only),
    }
    let Some(loops) = face_pair_section_loops((body_a, face_a), (body_b, face_b), band)? else {
        return Ok(super::Branch::Only);
    };
    loops
        .branch_at(site, band)?
        .ok_or(BooleanError::NodeUnbuilt {
            a_face: face_a,
            b_face: face_b,
            why: super::NodeWhy::GermOnNode,
        })
}

/// **The section a germ rides, carried**, where the germ lies inside a
/// face of each operand on a pair the operation's section table holds
/// ([`super::section_table`]); `None` otherwise, the pair read by its
/// frame. A germ along an edge rides the edge's own curve.
///
/// # Errors
///
/// [`section_unbuilt`]: the pair's section refused at the glue door, or
/// it did not carry.
fn germ_section<'r, T: Decide>(
    red: &'r BooleanReduction<T>,
    germ: &HalfGerm<T>,
) -> Result<Option<&'r geom_brep::CarriedSection<T>>, BooleanError> {
    if germ.a_locus.edge().is_some() || germ.b_locus.edge().is_some() {
        return Ok(None);
    }
    let faces = ((&red.a, germ.a_face), (&red.b, germ.b_face));
    match red.sections.carried(faces.0, faces.1) {
        None => Ok(None),
        Some(Ok(section)) => Ok(Some(section)),
        Some(Err(why)) => Err(faces_section_unbuilt(why, faces.0, faces.1)),
    }
}

/// **Each carried component's seat**, by the pair's surfaces and the
/// component: the least foot parameter of any site on it, every site
/// being an end of a segment (the matcher consumes every germ). A
/// function of the section and its sites alone, so the component is
/// opened at the same site whatever order the segments are joined in.
///
/// # Errors
///
/// [`section_unbuilt`] and the site's reading ([`site_refusal`]); a
/// desync where a carried segment's germ pair holds no section, or an
/// end of it lies on another component than its germ's branch names.
fn carried_seats<T: Decide>(
    red: &BooleanReduction<T>,
    open: &[OpenRecord<T>],
    segments: &[SectionSegment<T>],
    pairs: &[SegmentPair<T>],
    band: Band,
) -> Result<
    BTreeMap<
        (
            crate::geometry::SurfaceKey,
            crate::geometry::SurfaceKey,
            usize,
        ),
        T,
    >,
    BooleanError,
> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut seats = BTreeMap::new();
    for (seg, pair) in segments.iter().zip(pairs) {
        let Some(k) = pair.carried else {
            continue;
        };
        let section = germ_section(red, &seg.germ)?.ok_or_else(unheld_section)?;
        for (r, slot) in seg.ends {
            let site = red
                .a
                .half_edge_start_point(open[r].a[slot].0.he)
                .ok_or(desync("germ site has no point"))?;
            let foot = section.foot(site, band).map_err(site_refusal)?;
            if foot.component != k {
                return Err(desync(
                    "a carried segment's site lies on another component than its germ's branch \
                     names",
                ));
            }
            seats
                .entry((pair.a.0, pair.b.0, k))
                .and_modify(|t: &mut T| *t = t.min(foot.t))
                .or_insert(foot.t);
        }
    }
    Ok(seats)
}

/// A carried segment whose germ pair the section table holds no section
/// for: the dispatch read one ([`SegmentPair::read`]), so a desync.
fn unheld_section() -> BooleanError {
    BooleanError::JoinDesync {
        what: "a carried segment's germ pair holds no section",
    }
}

/// **Component `k` of `germ`'s carried section, seated** at its seat
/// parameter `at` ([`carried_seats`],
/// [`super::section_table::SectionTable::seated`]), and the section's
/// scale.
///
/// # Errors
///
/// [`section_unbuilt`] for a section that did not carry or a component
/// that did not seat; a desync where the germ's pair holds no section.
fn carried_component<T: Decide>(
    red: &BooleanReduction<T>,
    germ: &HalfGerm<T>,
    k: usize,
    at: T,
) -> Result<(geom_brep::CarriedComponent<T>, T), BooleanError> {
    let section = germ_section(red, germ)?.ok_or_else(unheld_section)?;
    let faces = ((&red.a, germ.a_face), (&red.b, germ.b_face));
    match red.sections.seated(faces.0, faces.1, k, at) {
        None => Err(unheld_section()),
        Some(Ok(component)) => Ok((component.clone(), section.lever())),
        Some(Err(why)) => Err(faces_section_unbuilt(why, faces.0, faces.1)),
    }
}

/// The Boolean's refusal of a germ pair's section, the pair being A's
/// face and B's with their kinds: an escalation refuses as the decision
/// it carries, a desync between the march, the classification and a
/// site as the join's desync, and the rest as
/// [`BooleanError::GermSectionUnbuilt`].
pub(super) fn section_unbuilt(
    why: super::section_table::Uncarried<'_>,
    (a_face, a_kind): (FaceKey, geom::SurfaceKind),
    (b_face, b_kind): (FaceKey, geom::SurfaceKind),
) -> BooleanError {
    use super::section_table::Uncarried;
    use geom_brep::{CarryError, SectionError};
    let unbuilt = |why| BooleanError::GermSectionUnbuilt {
        a_face,
        a_kind,
        b_face,
        b_kind,
        why,
    };
    match why {
        Uncarried::Section(SectionError::Escalated(diag))
        | Uncarried::Carry(CarryError::Escalated(diag)) => section_undecided(*diag),
        Uncarried::Section(SectionError::RadiusEscalated { radius, diag }) => {
            BooleanError::Escalated {
                decision: BooleanDecision::Radius(*radius),
                diag: *diag,
            }
        }
        Uncarried::Section(e) => unbuilt(SectionUnbuilt::Refused(e.clone())),
        Uncarried::Carry(CarryError::Unminted(why)) => unbuilt(SectionUnbuilt::Tangent(why)),
        Uncarried::Carry(CarryError::Winds(e)) => unbuilt(SectionUnbuilt::Winds(*e)),
        Uncarried::Carry(CarryError::PadShort { pad, hull_sup }) => {
            unbuilt(SectionUnbuilt::PadShort {
                pad: *pad,
                hull_sup: *hull_sup,
            })
        }
        Uncarried::Carry(CarryError::TubeUnread) => unbuilt(SectionUnbuilt::TubeUnread),
        Uncarried::Carry(CarryError::NoLane { scalar }) => {
            unbuilt(SectionUnbuilt::NoLane { scalar })
        }
        Uncarried::Carry(CarryError::March(e)) => unbuilt(SectionUnbuilt::March(e.clone())),
        Uncarried::Carry(CarryError::Seat(e)) => unbuilt(SectionUnbuilt::Seat(e.clone())),
        Uncarried::Component => BooleanError::JoinDesync {
            what: "a germ's branch names a component its carried section does not have",
        },
        Uncarried::Carry(CarryError::Desync(what)) => BooleanError::JoinDesync { what },
    }
}

/// [`section_unbuilt`] on the faces `fa` of `a` and `fb` of `b`.
fn faces_section_unbuilt<T: geom_core::Real>(
    why: super::section_table::Uncarried<'_>,
    (a, fa): (&Body<T>, FaceKey),
    (b, fb): (&Body<T>, FaceKey),
) -> BooleanError {
    let kind = |body: &Body<T>, f| face_surface(body, f).map(|(_, s)| s.kind());
    match (kind(a, fa), kind(b, fb)) {
        (Ok(a_kind), Ok(b_kind)) => section_unbuilt(why, (fa, a_kind), (fb, b_kind)),
        (Err(e), _) | (_, Err(e)) => e,
    }
}

/// A site's reading against its section: an escalation as the section
/// undecided, a site on no component a desync.
pub(super) fn site_refusal(e: geom_brep::SiteRefusal) -> BooleanError {
    match e {
        geom_brep::SiteRefusal::Escalated(diag) => section_undecided(diag),
        geom_brep::SiteRefusal::Desync(what) => BooleanError::JoinDesync { what },
    }
}

/// Where two faces' surfaces meet, undecided: no declaration settles it.
fn section_undecided(diag: geom_core::Indeterminate) -> BooleanError {
    BooleanError::Escalated {
        decision: BooleanDecision::Coincidence(Coincide::Section, DeclarationRead::Moot),
        diag,
    }
}

/// **The pair-general section-frame dispatch**, keyed on the germ
/// pair's two surface KINDS and nothing else — no bodies, no keys, so
/// a lane widening the dispatch adds an arm here and every consumer
/// inherits it.
///
/// `Ok(None)` means the locus is STRAIGHT and is only ever returned by
/// an arm that PROVED it: the plane×plane pair (a line by
/// construction), the two degenerate plane×cylinder outcomes whose loci
/// are lines, and a cylinder pair whose AXES are parallel (its
/// intersection is rulings, whatever the radii). A kind pair with no
/// arm is [`FrameError::NoArm`] — never `None`, because the caller
/// reads `None` as "run the straight-chord facing test" and would mint
/// a wrong chord from it.
///
/// The answer is the section's loops, each with its frame
/// ([`SectionLoops`]); a germ takes the loop its branch names, decided
/// where it was minted ([`germ_branch`]). The plane×torus pair
/// ([`pt_germ_frame`]) and the equal cylinder pair with meeting axes
/// ([`cylinder_pair_frame`]) name two loops with frames of their own;
/// every other pair's section is one loop, or loops that share one frame
/// ([`conic_section_frame_at`]).
pub(super) fn pair_section_frame_at<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    at: geom_core::Point3<T>,
    extent: FrameExtent<T>,
    band: Band,
) -> Result<Option<SectionLoops<T>>, FrameError> {
    use geom::Surface as Sf;
    match (sa, sb) {
        (Sf::Plane { .. }, Sf::Torus { .. }) => pt_germ_frame(sa, sb, extent, band),
        (Sf::Torus { .. }, Sf::Plane { .. }) => pt_germ_frame(sb, sa, extent, band),
        (Sf::Cylinder { .. }, Sf::Cylinder { .. }) => cylinder_pair_frame(sa, sb, at, extent, band),
        _ => conic_section_frame_at(sa, sb, at, extent, band)
            .map(|frame| frame.map(SectionLoops::conic)),
    }
}

/// **Cylinder×cylinder.** Two walls with PARALLEL axes meet in rulings —
/// lines — whatever their radii, so `None` here is proven by the axes
/// alone ([`cylinder_pair_axes`], which reads the radii only as a lever)
/// and needs no constructed section: a tangent-ruling pair at rest is
/// exactly this case. The non-parallel half is never straight, and it
/// splits again on COPLANARITY. The signed axis-to-axis gap along
/// `a1×a2` is the section table's own `cc_axes_coplanar` margin
/// (`geom_brep::cylinder_cylinder_section` step 5) and it reads NO
/// radius: a definite gap is SKEW, whose locus is a space quartic —
/// canal territory, the general rung — and keeps `NoArm` verbatim. A
/// Zero gap means the axes meet ([`intersecting_cylinder_axes`]).
///
/// The axis rows are read at the foot of `at` on each axis, levered at
/// the length the walls run together or the larger radius, whichever is
/// longer — two axes a sine θ apart drift θ·span apart over the walls
/// ([`cylinder_pair_axes`]).
///
/// Asked of the table's own row too
/// ([`geom_brep::cylinder_axes_coplanar`], `cc_axes_coplanar`), read
/// between the axes' feet across the reach the parallelism was read
/// across. What the rows pin is its direction:
/// `skew_axes_keep_the_general_rung_at_the_certified_scalar` reds the
/// moment the gap is measured along an axis instead of along `a1×a2`.
fn cylinder_pair_frame<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    at: geom_core::Point3<T>,
    extent: FrameExtent<T>,
    band: Band,
) -> Result<Option<SectionLoops<T>>, FrameError> {
    let (
        &geom::Surface::Cylinder {
            origin: o1,
            axis: a1,
            radius: r1,
            ..
        },
        &geom::Surface::Cylinder {
            origin: o2,
            axis: a2,
            radius: r2,
            ..
        },
    ) = (sa, sb)
    else {
        return Err(FrameError::Desync(
            "the cylinder pair frame was handed another kind pair",
        ));
    };
    let (l1, l2) = ((o1, a1), (o2, a2));
    let reach = match cylinder_pair_axes(l1, l2, (r1, r2), at, extent, band)? {
        CylinderAxes::Parallel => return Ok(None),
        CylinderAxes::Apart(reach) => reach,
    };
    match geom_brep::cylinder_axes_coplanar(&reach, l1, l2, band) {
        Ok(Sign::Zero) => intersecting_cylinder_axes(sa, sb, &reach, band).map(Some),
        Ok(Sign::Positive | Sign::Negative) => Err(FrameError::NoArm),
        Err(diag) => Err(FrameError::section(diag)),
    }
}

/// [`pair_section_frame_at`]'s pairs whose sections share one frame.
/// The plane×cylinder pair reads its axis rows at the foot of `at` on
/// the axis ([`germ_section_loops`]: a point the section is consumed
/// at), levered at `extent`, the consumed region's measure the caller
/// took from `at` ([`frame_extent`]): its wall face's axial extent from
/// `at` and its reach across the wall ([`geom_brep::Reach::Face`]).
/// [`FrameExtent::Radii`], a pair handed without its faces, levers by
/// the radius alone.
#[allow(clippy::type_complexity)] // (conic center, conic axis) — one frame tuple
fn conic_section_frame_at<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    at: geom_core::Point3<T>,
    extent: FrameExtent<T>,
    band: Band,
) -> Result<Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>, FrameError> {
    use geom::Surface as Sf;
    let (plane_s, cyl_s, radius) = match (sa, sb) {
        (Sf::Plane { .. }, Sf::Cylinder { radius, .. }) => (sa, sb, *radius),
        (Sf::Cylinder { radius, .. }, Sf::Plane { .. }) => (sb, sa, *radius),
        // The sphere germ pair (M5 S13): the C5 Circle's frame,
        // through THE table — same escalation plumbing.
        (Sf::Plane { .. }, Sf::Sphere { .. }) | (Sf::Sphere { .. }, Sf::Plane { .. }) => {
            let (plane_s, sph_s) = if matches!(sa, Sf::Plane { .. }) {
                (sa, sb)
            } else {
                (sb, sa)
            };
            return match geom_brep::plane_sphere_section(plane_s, sph_s, band) {
                Ok(geom_brep::PlaneSphereSection::Circle(geom::Curve3::Circle {
                    center,
                    axis,
                    ..
                })) => Ok(Some((center, axis))),
                Ok(geom_brep::PlaneSphereSection::Circle(_)) => Err(FrameError::Desync(
                    "plane×sphere classification carried a non-circle",
                )),
                // A tangent POINT / empty gap under a minted germ is a
                // touching configuration the reduction should not have
                // paired — loud, typed.
                Ok(
                    geom_brep::PlaneSphereSection::TangentPoint(_)
                    | geom_brep::PlaneSphereSection::Empty,
                ) => Err(FrameError::Desync(
                    "germ pair's plane×sphere section is not a locus",
                )),
                Err(geom_brep::SectionError::Escalated(diag)) => Err(FrameError::section(diag)),
                Err(_) => Err(FrameError::Desync(
                    "germ pair's section refused at match time",
                )),
            };
        }
        // The sphere PAIR: the C5 radical-plane Circle's frame, through
        // THE table — same escalation plumbing as the plane×sphere arm
        // above.
        //
        // The frame this dispatch names is the LOCUS's — the centre and
        // axis the rotational-sense facing test turns about — and a
        // sphere pair's locus is that circle whatever either chart's
        // polar axis does. Whether the section is polar for a chart is
        // the ARC-SIDE rule's question, asked per operand in
        // `chord_join::section_case` once the join hands each side the
        // pair's radical plane.
        (Sf::Sphere { .. }, Sf::Sphere { .. }) => {
            return match geom_brep::sphere_sphere_section(sa, sb, band) {
                Ok(geom_brep::SphereSphereSection::Circle(geom::Curve3::Circle {
                    center,
                    axis,
                    ..
                })) => Ok(Some((center, axis))),
                Ok(geom_brep::SphereSphereSection::Circle(_)) => Err(FrameError::Desync(
                    "sphere×sphere classification carried a non-circle",
                )),
                // A tangent POINT / empty gap under a minted germ is a
                // touching configuration the reduction should not have
                // paired — loud, typed.
                Ok(
                    geom_brep::SphereSphereSection::TangentPoint(_)
                    | geom_brep::SphereSphereSection::Empty,
                ) => Err(FrameError::Desync(
                    "germ pair's sphere×sphere section is not a locus",
                )),
                Err(geom_brep::SectionError::Escalated(diag)) => Err(FrameError::section(diag)),
                Err(_) => Err(FrameError::Desync(
                    "germ pair's section refused at match time",
                )),
            };
        }
        (Sf::Plane { .. }, Sf::Cone { .. }) => return pk_germ_frame(sa, sb, extent, band),
        (Sf::Cone { .. }, Sf::Plane { .. }) => return pk_germ_frame(sb, sa, extent, band),
        // **Cylinder×sphere** has no frame: the operation's section
        // table holds the pair, and its germs match by foot parameter on
        // the section's components ([`along_facing`]).
        (Sf::Cylinder { .. }, Sf::Sphere { .. }) | (Sf::Sphere { .. }, Sf::Cylinder { .. }) => {
            return Err(FrameError::Desync(
                "a cylinder × sphere pair is read by its section's components, not by a frame",
            ));
        }
        // The ONE structurally straight pair: a plane×plane section is
        // a line, so "no frame" is a proof here rather than a default.
        (Sf::Plane { .. }, Sf::Plane { .. }) => return Ok(None),
        _ => return Err(FrameError::NoArm),
    };
    let section = match extent {
        FrameExtent::Wall {
            below,
            above,
            across,
            round,
        } => {
            let reach = crate::chord_join::WallReach {
                at,
                below,
                above,
                span: across,
                round,
            };
            crate::chord_join::agreed_section(plane_s, cyl_s, reach, band)
        }
        FrameExtent::Radii | FrameExtent::Span(_) => geom_brep::plane_cylinder_section(
            plane_s,
            cyl_s,
            &geom_brep::Reach::Measured { at, lever: radius },
            band,
        ),
        FrameExtent::Reach(_) | FrameExtent::Cone(_) => {
            return Err(FrameError::Desync(FACE_REACH_ELSEWHERE));
        }
    };
    match section {
        Ok(geom_brep::PlaneCylinderSection::Rim(geom::Curve3::Circle { center, axis, .. }))
        | Ok(geom_brep::PlaneCylinderSection::TiltedEllipse(geom::Curve3::Ellipse {
            center,
            axis,
            ..
        })) => Ok(Some((center, axis))),
        Ok(
            geom_brep::PlaneCylinderSection::ParallelLines { .. }
            | geom_brep::PlaneCylinderSection::TangentLine(_),
        ) => Ok(None),
        Ok(_) => Err(FrameError::Desync(
            "germ pair's section classification is not a locus",
        )),
        Err(e) => Err(section_refusal(e)),
    }
}

/// **How a cylinder pair's axes stand**, as [`cylinder_pair_axes`]
/// reads them.
enum CylinderAxes<T: geom_core::Real> {
    /// Parallel: the walls meet in rulings, or not at all.
    Parallel,
    /// Definitely not parallel, read across this reach, which the frame
    /// dispatch's coplanarity reading and section table re-read.
    Apart(geom_brep::Reach<T>),
}

/// **Whether a cylinder pair's axes are parallel**: the axes `line1`
/// and `line2` of walls of radii `radii`, asked of the section table's
/// own row ([`geom_brep::cylinder_axes_parallel`], `cc_axes_parallel`)
/// at `at`, levered at the larger radius or the walls' span, whichever
/// is longer: the axes drift apart by the sine times the length they run
/// together. A bigger lever only moves a reading toward the definite
/// side, and there the frame dispatch refuses and the join takes no
/// rulings arm, so it never serves a frame or a radical plane the walls'
/// span would not. The frame dispatch and [`segment_arm`] both read it
/// here, from the same point and lever ([`frame_lever`]); on the frame's
/// path the table re-decides this margin from the returned reach.
///
/// # Errors
///
/// [`FrameError::Escalated`] as [`BooleanDecision::CylinderAxes`] where
/// the reading is in the band, and a
/// desync for an extent a cylinder pair is never levered at.
fn cylinder_pair_axes<T: Decide>(
    line1: (geom_core::Point3<T>, geom_core::Vec3<T>),
    line2: (geom_core::Point3<T>, geom_core::Vec3<T>),
    (r1, r2): (T, T),
    at: geom_core::Point3<T>,
    extent: FrameExtent<T>,
    band: Band,
) -> Result<CylinderAxes<T>, FrameError> {
    let lever = match extent {
        FrameExtent::Span(l) => l.max(r1.max(r2)),
        FrameExtent::Radii | FrameExtent::Wall { .. } => r1.max(r2),
        FrameExtent::Reach(_) | FrameExtent::Cone(_) => {
            return Err(FrameError::Desync(FACE_REACH_ELSEWHERE));
        }
    };
    let reach = geom_brep::Reach::Measured { at, lever };
    match geom_brep::cylinder_axes_parallel(&reach, line1, line2, band) {
        Ok(Sign::Zero) => Ok(CylinderAxes::Parallel),
        Ok(Sign::Positive | Sign::Negative) => Ok(CylinderAxes::Apart(reach)),
        Err(diag) => Err(FrameError::Escalated {
            decision: BooleanDecision::CylinderAxes,
            diag,
        }),
    }
}

/// [`FrameExtent::Reach`] or [`FrameExtent::Cone`] handed to a cylinder
/// pair: only a cone or a torus face's frame is levered at them
/// ([`frame_extent`]).
const FACE_REACH_ELSEWHERE: &str = "a curved face's reach levered a pair with no cone or torus";

/// A germ pair's section refusal that is not the table's verdict on the
/// pair's locus: an undecided degeneracy, or a conic carrier whose
/// minor semi-axis the constructor could not decide positive,
/// escalates; anything else is a desync.
fn section_refusal(e: geom_brep::SectionError) -> FrameError {
    match e {
        geom_brep::SectionError::Escalated(diag)
        | geom_brep::SectionError::Carrier(geom::EllipseInvalid::Escalated(diag)) => {
            FrameError::section(diag)
        }
        geom_brep::SectionError::DegenerateTorus(geom_brep::TorusRefusal {
            convention,
            verdict,
        }) => match verdict {
            geom_brep::TorusVerdict::Undecided(diag) => FrameError::Escalated {
                decision: BooleanDecision::Torus(convention),
                diag,
            },
            geom_brep::TorusVerdict::Refused(verdict) => FrameError::DegenerateTorus {
                convention,
                verdict,
            },
        },
        _ => FrameError::Desync("germ pair's section refused at match time"),
    }
}

/// **The plane×cone germ frame**, through THE table's own
/// [`geom_brep::plane_cone_section`]: the ellipse's or the axis-normal
/// circle's centre and axis; a plane through the apex that dips into
/// the cone cuts two generators, proved straight. The apex's touching
/// outcomes under a minted germ are the desync the sphere arm's tangent
/// point is. A parabola or a hyperbola is served by the table in closed
/// form, its arcs NURBS carriers the Boolean cannot yet cross, so it
/// refuses naming the conic ([`FrameError::OpenConic`]).
///
/// Read over the cone face's reach ([`FrameExtent::Cone`], the only
/// extent [`frame_extent`] hands a plane×cone pair); any other extent is
/// a desync.
#[allow(clippy::type_complexity)] // (conic center, conic axis) — one frame tuple
fn pk_germ_frame<T: Decide>(
    plane: &geom::Surface<T>,
    cone: &geom::Surface<T>,
    extent: FrameExtent<T>,
    band: Band,
) -> Result<Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>, FrameError> {
    let FrameExtent::Cone(reach) = extent else {
        return Err(FrameError::Desync(
            "a plane×cone frame was levered at no cone face's reach",
        ));
    };
    match geom_brep::plane_cone_section(plane, cone, &reach, band) {
        Ok(
            geom_brep::PlaneConeSection::TiltedEllipse(geom::Curve3::Ellipse {
                center, axis, ..
            })
            | geom_brep::PlaneConeSection::AxisNormalCircle(geom::Curve3::Circle {
                center,
                axis,
                ..
            }),
        ) => Ok(Some((center, axis))),
        Ok(geom_brep::PlaneConeSection::ApexLinePair { .. }) => Ok(None),
        Ok(
            geom_brep::PlaneConeSection::ApexTangentLine(_)
            | geom_brep::PlaneConeSection::ApexPoint(_),
        ) => Err(FrameError::Desync(
            "germ pair's plane×cone section is not a locus",
        )),
        Ok(geom_brep::PlaneConeSection::OpenConic(section)) => Err(FrameError::OpenConic {
            conic: section.conic(),
        }),
        Ok(_) => Err(FrameError::Desync(
            "plane×cone classification carried a carrier its conic is not",
        )),
        Err(e) => Err(section_refusal(e)),
    }
}

/// **The plane×torus germ frame**, through THE table's own
/// [`geom_brep::plane_torus_section`]. Each served section is two loops,
/// each with a frame it turns about strictly monotonically:
///
/// - **meridian circles** (the plane holds the axis): each circle's own
///   centre and axis, the loop told by the side of the axis's trace;
/// - **spiric ovals** (parallel to the axis, short of the inner
///   equator): the point `k` where the core circle crosses the plane
///   inside the oval, and the plane's normal, the loop told by the side
///   of the axis's trace. In the plane, with `t` running away from the
///   trace along the oval's side and `y` along the axis, `ρ² = d² + t²`
///   (`d` the plane's stand-off) and `k = (t₀, 0)`, `t₀ = √(R² − d²)`.
///   The oval's upper half `y = f(t)` has `f f′ = −(ρ − R)·t/ρ`, so its
///   turn about `k`, times `f`, is `−(t − t₀)(ρ − R)·t/ρ − f²`. `ρ − R`
///   has the sign of `t − t₀`, so both terms are `≤ 0`, and the first is
///   strictly negative at the ends where `f` vanishes: one strict sign
///   along the oval, the lower half by `y ↦ −y`. `k` is no centre of
///   symmetry of the oval, so the loop carries its closed-form ball:
///   `t` between `√((R − r)² − d²)` and `√((R + r)² − d²)`, `|y| ≤ r`;
/// - **concentric circles** (square to the axis): their common centre
///   and axis, the loop told by the distance from the axis against `R`.
///
/// Each loop margin is at least `√((R − r)² − d²)` or `√(r² − h²)` at
/// every point of the section, which the table decided positive.
///
/// A general plane cuts a spiric quartic, which the table routes to its
/// unbuilt general rung; that refuses as [`FrameError::Spiric`], naming
/// the loops by the section certificate's own margins
/// ([`section_cert::torus_plane_loops`](super::section_cert::torus_plane_loops)).
/// A tangent circle or no section under a minted germ is the desync the
/// sphere arm's tangent point is. Where the table routes the pose to its
/// general rung and the certificate's unlevered margins read it touching
/// or empty, the two readings straddle the band about one pose, which
/// escalates ([`PT_TOUCHING_UNDER_A_GERM`]), as does a certificate
/// margin left undecided.
///
/// Levered at the torus face's reach from the reading point
/// ([`FrameExtent::Reach`]); any other extent is a desync.
fn pt_germ_frame<T: Decide>(
    plane: &geom::Surface<T>,
    torus: &geom::Surface<T>,
    extent: FrameExtent<T>,
    band: Band,
) -> Result<Option<SectionLoops<T>>, FrameError> {
    let FrameExtent::Reach(lever) = extent else {
        return Err(FrameError::Desync(
            "a plane×torus frame was levered at no torus face's reach",
        ));
    };
    let (
        &geom::Surface::Plane {
            origin: q,
            normal: n,
            ..
        },
        &geom::Surface::Torus {
            center: c,
            axis: a,
            major_radius: big_r,
            minor_radius: r,
            ..
        },
    ) = (plane, torus)
    else {
        return Err(FrameError::Desync(
            "the plane×torus frame was handed another kind pair",
        ));
    };
    let carried = || FrameError::Desync("plane×torus classification carried another carrier");
    let circle = |curve: geom::Curve3<T>| match curve {
        geom::Curve3::Circle { center, axis, .. } => Ok(SectionLoop {
            center,
            axis,
            ball: None,
        }),
        _ => Err(carried()),
    };
    let section = geom_brep::plane_torus_section(plane, torus, lever, band);
    let split = section
        .as_ref()
        .ok()
        .and_then(|s| LoopSplit::plane_torus(s, (c, a, big_r), n));
    let two = |loops| match split {
        Some(split) => Ok(Some(SectionLoops::Two { loops, split })),
        None => Err(carried()),
    };
    match section {
        Ok(geom_brep::PlaneTorusSection::MeridianCircles { c1, c2 }) => {
            two([circle(c1)?, circle(c2)?])
        }
        Ok(geom_brep::PlaneTorusSection::SpiricOvals { s1, s2 }) => {
            let oval = |curve: geom::Curve3<T>| match curve {
                geom::Curve3::Spiric {
                    center,
                    axis,
                    u_ref,
                    major_radius,
                    minor_radius,
                    offset,
                } => {
                    let side = axis.cross(u_ref);
                    let foot = center + u_ref * offset;
                    let along =
                        |radius: T| ((radius - offset.abs()) * (radius + offset.abs())).sqrt();
                    let (t_in, t_out) = (
                        along(major_radius - minor_radius),
                        along(major_radius + minor_radius),
                    );
                    let halved = |x: T| x / (T::one() + T::one());
                    let (mid, half) = (halved(t_in + t_out), halved(t_out - t_in));
                    Ok(SectionLoop {
                        center: foot + side * along(major_radius),
                        axis: n,
                        ball: Some(geom_brep::ExtentBall::new(
                            foot + side * mid,
                            (half.powi(2) + minor_radius.powi(2)).sqrt(),
                        )),
                    })
                }
                _ => Err(carried()),
            };
            two([oval(s1)?, oval(s2)?])
        }
        Ok(geom_brep::PlaneTorusSection::ConcentricCircles { c1, c2 }) => {
            two([circle(c1)?, circle(c2)?])
        }
        Ok(
            geom_brep::PlaneTorusSection::TangentCircle(_) | geom_brep::PlaneTorusSection::Empty,
        ) => Err(FrameError::Desync(
            "germ pair's plane×torus section is not a locus",
        )),
        Err(e @ geom_brep::SectionError::RoutesToGeneralRung { .. }) => {
            match super::section_cert::torus_plane_loops((c, a, big_r, r), (q, n), band) {
                Ok(Some(loops)) => Err(FrameError::Spiric { loops, section: e }),
                Ok(None) => Err(FrameError::section(crate::invalid_margin::invalid(
                    band,
                    PT_TOUCHING_UNDER_A_GERM,
                ))),
                Err(diag) => Err(FrameError::section(diag)),
            }
        }
        Err(e) => Err(section_refusal(e)),
    }
}

/// [`pt_germ_frame`]'s escalation where the table, levered at the torus
/// face's reach, routes a pose to its general rung and the section
/// certificate's unlevered margins read it touching or empty: two
/// readings of one pose straddling the band
/// ([`crate::invalid_margin::invalid`]).
const PT_TOUCHING_UNDER_A_GERM: &str = "bool_join_torus_section_touching_under_a_germ";

/// **An equal cylinder pair with meeting axes, as its section table
/// classifies it**: the two Zero margins its two bisector-plane
/// ellipses are decided from and the nodes where they cross
/// ([`geom_brep::EqualCylinderSection::TwoEllipses`]), with the two
/// walls' axes, A's first.
pub(super) struct CrossingSection<T: geom_core::Real> {
    pub(super) margins: [geom_core::MarginDiag; 2],
    pub(super) nodes: [Point3<T>; 2],
    pub(super) axes: [Vec3<T>; 2],
    /// Each ellipse's plane, its centre (the axes' meeting point) and
    /// unit normal, the `a1 − a2` plane first.
    pub(super) planes: [(Point3<T>, Vec3<T>); 2],
}

/// **The face pair `(face_a, face_b)` as an equal cylinder pair with
/// meeting axes** ([`CrossingSection`]), read as the join reads it
/// ([`frame_lever`], [`cylinder_pair_axes`]). `Ok(None)` for any other
/// pair or pose: other kinds, parallel axes, unequal radii or skew axes,
/// each decided.
///
/// # Errors
///
/// A reading the pair cannot decide: the parallelism, coplanarity or
/// radius margin in band ([`frame_refusal`]), or the reading's desync.
pub(super) fn crossing_cylinder_reading<T: Decide>(
    (body_a, face_a): (&Body<T>, FaceKey),
    (body_b, face_b): (&Body<T>, FaceKey),
    band: Band,
) -> Result<Option<CrossingSection<T>>, BooleanError> {
    let (Ok((_, sa)), Ok((_, sb))) = (face_surface(body_a, face_a), face_surface(body_b, face_b))
    else {
        return Ok(None);
    };
    let (
        &geom::Surface::Cylinder {
            origin: o1,
            axis: a1,
            radius: r1,
            ..
        },
        &geom::Surface::Cylinder {
            origin: o2,
            axis: a2,
            radius: r2,
            ..
        },
    ) = (sa, sb)
    else {
        return Ok(None);
    };
    let refused = |e| frame_refusal(e, (face_a, sa), (face_b, sb));
    let (at, extent) = frame_lever((sa, body_a, face_a), (sb, body_b, face_b), band)?;
    let reach = match cylinder_pair_axes((o1, a1), (o2, a2), (r1, r2), at, extent, band) {
        Ok(CylinderAxes::Apart(reach)) => reach,
        Ok(CylinderAxes::Parallel) => return Ok(None),
        Err(e) => return Err(refused(e)),
    };
    match geom_brep::cylinder_cylinder_section(sa, sb, &reach.ball(), band) {
        Ok(geom_brep::EqualCylinderSection::TwoEllipses {
            e1,
            e2,
            margins,
            nodes,
        }) => {
            let plane = |e: &geom::Curve3<T>| match *e {
                geom::Curve3::Ellipse { center, axis, .. } => Ok((center, axis)),
                _ => Err(BooleanError::JoinDesync {
                    what: "an equal cylinder pair's section ellipse is no ellipse",
                }),
            };
            Ok(Some(CrossingSection {
                margins,
                nodes,
                axes: [a1, a2],
                planes: [plane(&e1)?, plane(&e2)?],
            }))
        }
        Err(
            e @ (geom_brep::SectionError::Escalated(_)
            | geom_brep::SectionError::Carrier(geom::EllipseInvalid::Escalated(_))),
        ) => Err(refused(section_refusal(e))),
        Err(geom_brep::SectionError::RadiusEscalated { radius, diag }) => {
            Err(refused(FrameError::Escalated {
                decision: BooleanDecision::Radius(radius),
                diag,
            }))
        }
        // Decided: not the crossing family. The parallel-axes loci (the
        // table's own parallelism reading the other way on this margin),
        // unequal radii, skew axes, and a radius or minor semi-axis
        // decided not positive.
        Ok(
            geom_brep::EqualCylinderSection::ParallelLines { .. }
            | geom_brep::EqualCylinderSection::TangentLine(_)
            | geom_brep::EqualCylinderSection::Empty,
        )
        | Err(
            geom_brep::SectionError::RoutesToGeneralRung { .. }
            | geom_brep::SectionError::DegenerateOperand { .. }
            | geom_brep::SectionError::Carrier(geom::EllipseInvalid::MinorNotPositive),
        ) => Ok(None),
        // Refusals two cylinders read apart cannot raise: the kinds
        // were matched above, the coaxial pose answered parallel, no
        // torus question is asked, no extent is handed to this pair, and
        // a bisector's semi-axes are a radius over a half-angle's sine or
        // cosine read off `a1 ∓ a2`, which are finite at any radius in
        // the box.
        Err(
            geom_brep::SectionError::Carrier(geom::EllipseInvalid::SemiAxisNotFinite)
            | geom_brep::SectionError::WrongLane { .. }
            | geom_brep::SectionError::BeyondOperandExtent { .. }
            | geom_brep::SectionError::CoincidentSurfaces
            | geom_brep::SectionError::DegenerateTorus(_)
            | geom_brep::SectionError::Pinched { .. }
            | geom_brep::SectionError::Unordered { .. }
            | geom_brep::SectionError::NoMarch { .. }
            | geom_brep::SectionError::Unaccounted { .. }
            | geom_brep::SectionError::NoRegion { .. }
            | geom_brep::SectionError::Spiric(_),
        ) => Err(BooleanError::JoinDesync {
            what: "two cylinder walls read apart refused as no cylinder pair can",
        }),
    }
}

/// [`crossing_cylinder_reading`], a refused reading taken as no crossing
/// pair: for the coincidence rows (`glue::section_door`), which record no
/// row where a reading refuses, a later decision refusing where the
/// faces meet (the node's mint, [`crossing_germs_have_their_reading`]).
pub(super) fn crossing_cylinder_section<T: Decide>(
    a: (&Body<T>, FaceKey),
    b: (&Body<T>, FaceKey),
    band: Band,
) -> Option<CrossingSection<T>> {
    crossing_cylinder_reading(a, b, band).ok().flatten()
}

/// **The two coincidences an equal cylinder pair with meeting axes is
/// decided from** ([`crossing_cylinder_section`]): the radii equal and
/// the axes meeting, each margin decided Zero by the section table.
pub(super) fn crossing_cylinder_margins<T: Decide>(
    a: (&Body<T>, FaceKey),
    b: (&Body<T>, FaceKey),
    band: Band,
) -> Option<[geom_core::MarginDiag; 2]> {
    crossing_cylinder_section(a, b, band).map(|s| s.margins)
}

/// **The intersecting-axes cylinder pair, routed by the pose's node
/// question** (`cylinder_cylinder_section`, the section value's
/// `CylinderCylinderPose`):
///
/// * the bisector pose (`cc_crossing_nest` Zero, the radii equal and the
///   axes meeting) — the table constructs the two bisector-plane
///   ellipses, two loops each turning about its own plane's normal
///   through the axes' meeting point, a centre of symmetry of both; the
///   bisector split tells them apart ([`LoopSplit::Bisector`]), and they
///   cross at two nodes where it is zero;
/// * every other pose — no conic section, which this frame has no arm
///   for ([`FrameError::NoArm`]); the section value marches it.
fn intersecting_cylinder_axes<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    reach: &geom_brep::Reach<T>,
    band: Band,
) -> Result<SectionLoops<T>, FrameError> {
    // The table reads the ball of the `reach` the parallelism gate above
    // levered from. Its lever there is no shorter, so a tilt the gate
    // read definite reads definite again; the coplanarity row reads at
    // the same feet.
    //
    // Both matches are CLOSED (VERB-SEAT-DESIGN §0, D3): every section
    // outcome and every refusal is named, so a variant added to either
    // enum is a compile-time visit here rather than a silent desync.
    let e = match geom_brep::cylinder_cylinder_section(sa, sb, &reach.ball(), band) {
        Ok(geom_brep::EqualCylinderSection::TwoEllipses { e1, e2, .. }) => {
            let (
                geom::Curve3::Ellipse {
                    center, axis: n1, ..
                },
                geom::Curve3::Ellipse { axis: n2, .. },
            ) = (e1, e2)
            else {
                return Err(FrameError::Desync(
                    "the equal-radius cylinder section carried a non-ellipse",
                ));
            };
            let lp = |axis| SectionLoop {
                center,
                axis,
                ball: None,
            };
            return Ok(SectionLoops::Two {
                loops: [lp(n1), lp(n2)],
                // The planes' own unit normals: `e1`'s along `â₁ − â₂`,
                // `e2`'s along `â₁ + â₂`.
                split: LoopSplit::Bisector {
                    origin: center,
                    sum: n2,
                    diff: n1,
                },
            });
        }
        // The three parallel-axes answers, from a table whose
        // parallelism verdict this dispatch already took the other
        // way on the same margin.
        Ok(
            geom_brep::EqualCylinderSection::ParallelLines { .. }
            | geom_brep::EqualCylinderSection::TangentLine(_)
            | geom_brep::EqualCylinderSection::Empty,
        ) => FrameError::Desync(
            "the equal-radius section of an intersecting-axes cylinder pair classified as \
             a parallel-axes locus",
        ),
        Err(
            e @ (geom_brep::SectionError::Escalated(_)
            | geom_brep::SectionError::Carrier(geom::EllipseInvalid::Escalated(_))),
        ) => section_refusal(e),
        // The table's own routing verdict: NOT a desync, since nothing
        // is contradicted — the pose is not the bisector one (unequal
        // radii on meeting axes, or the table's coplanarity reading the
        // other way on the same margin), and the section value marches
        // it.
        Err(geom_brep::SectionError::RoutesToGeneralRung { .. }) => FrameError::NoArm,
        // Refusals that cannot come out of a cylinder pair this
        // dispatch admitted: the kinds were matched above, the
        // coaxial-equal-radius pose was refused at the parallelism
        // gate, no torus question is asked of two cylinders with
        // meeting axes, the bisector ellipses' semi-axes are decided
        // positive and finite, and no arm this pair reaches states
        // a locus off the extent it was handed (the cylinder pair's
        // ellipses stand on the operands themselves).
        Err(
            geom_brep::SectionError::WrongLane { .. }
            | geom_brep::SectionError::RadiusEscalated { .. }
            | geom_brep::SectionError::DegenerateOperand { .. }
            | geom_brep::SectionError::CoincidentSurfaces
            | geom_brep::SectionError::DegenerateTorus(_)
            | geom_brep::SectionError::BeyondOperandExtent { .. }
            | geom_brep::SectionError::Carrier(
                geom::EllipseInvalid::MinorNotPositive | geom::EllipseInvalid::SemiAxisNotFinite,
            )
            | geom_brep::SectionError::Pinched { .. }
            | geom_brep::SectionError::Unordered { .. }
            | geom_brep::SectionError::NoMarch { .. }
            | geom_brep::SectionError::Unaccounted { .. }
            | geom_brep::SectionError::NoRegion { .. }
            | geom_brep::SectionError::Spiric(_),
        ) => FrameError::Desync(
            "the equal-radius cylinder section refused at the germ pair with a refusal this \
             pair cannot produce",
        ),
    };
    Err(e)
}

/// Whether two germs face each other ([`facing`]).
#[cfg(test)]
pub(super) fn germs_face_each_other<T: Decide>(
    loops: Option<&SectionLoops<T>>,
    g1: &HalfGerm<T>,
    g2: &HalfGerm<T>,
    p1: geom_core::Point3<T>,
    p2: geom_core::Point3<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    Ok(!matches!(
        facing(loops, g1, g2, p1, p2, band)?,
        Faced::Apart
    ))
}

/// How two germs face each other ([`facing`], [`along_facing`]).
enum Faced<T: geom_core::Real> {
    /// They do not.
    Apart,
    /// They do, along a straight germ line.
    Straight,
    /// They do, along one loop of a conic section: the turn from `g1`
    /// toward `g2`.
    Conic(Turn<T>),
    /// They do, along one component of the section the table holds:
    /// how far `g2`'s foot is from `g1`'s.
    Along(Along<T>),
}

/// **Whether two germs face each other along their section's
/// component** ([`super::section_table`]), each a germ and its site:
/// each site's foot on the component it lies on
/// ([`geom_brep::CarriedSection::foot`]), read here at the germ's
/// half-edge's start, which must name the component its germ's branch
/// was minted from (at the mint's own reading of the site,
/// [`germ_branch`]); and the direction each germ leaves along
/// its component, decided on the germ's direction against the carrier's
/// tangent there (`bool_join_foot_heading`, the cosine levered by the
/// section's own scale, [`geom_brep::CarriedSection::lever`]). Each
/// germ's foot and heading are its own, read once for the operation
/// ([`super::section_table::SectionTable::germ_foot`]); both germs are
/// A's halves, the B halves at their slots being mirror checks
/// ([`partners`]). Two germs whose branches name different
/// components (each its site's foot's, [`germ_branch`]), or leaving the
/// same way, never face. A closed component is travelled round; on an
/// open one the partner's foot must lie ahead. `Along` carries the
/// carrier's parameter from `g1`'s foot to `g2`'s, the way `g1` leaves.
///
/// # Errors
///
/// A site's escalation or desync; a germ with no heading along its
/// component, a desync.
fn along_facing<T: Decide>(
    (table, a, b): (&super::section_table::SectionTable<T>, &Body<T>, &Body<T>),
    section: &geom_brep::CarriedSection<T>,
    (g1, p1): (&HalfGerm<T>, Point3<T>),
    (g2, p2): (&HalfGerm<T>, Point3<T>),
    band: Band,
) -> Result<Faced<T>, BooleanError> {
    use super::section_table::{FootRefusal, GermFoot};
    if g1.branch != g2.branch {
        return Ok(Faced::Apart);
    }
    let desync = |what| BooleanError::JoinDesync { what };
    let read = |germ: &HalfGerm<T>, p| -> Result<GermFoot<T>, FootRefusal> {
        let foot = section.foot(p, band).map_err(FootRefusal::Site)?;
        if germ.branch != super::Branch::Component(foot.component) {
            return Err(FootRefusal::Desync(
                "a germ's site has its foot on another component than its branch names",
            ));
        }
        let cos = germ.dir.dot(foot.tangent) / foot.tangent.norm();
        let heading = Margin::levered(cos, section.lever());
        match decide("bool_join_foot_heading", heading, band) {
            Ok(Sign::Positive) => Ok(GermFoot { foot, up: true }),
            Ok(Sign::Negative) => Ok(GermFoot { foot, up: false }),
            Ok(Sign::Zero) => Err(FootRefusal::Desync(
                "a germ leaves its site across its section's component, not along it",
            )),
            Err(diag) => Err(FootRefusal::Heading(diag)),
        }
    };
    let foot = |germ: &HalfGerm<T>, p| {
        table
            .germ_foot(
                (Operand::A, germ.he),
                (a, germ.a_face),
                (b, germ.b_face),
                || read(germ, p),
            )
            .map(|g| (g.foot, g.up))
            .map_err(|e| match e {
                FootRefusal::Site(e) => site_refusal(e),
                FootRefusal::Heading(diag) => join_escalate(diag),
                FootRefusal::Desync(what) => desync(what),
            })
    };
    let ((f1, up1), (f2, up2)) = (foot(g1, p1)?, foot(g2, p2)?);
    if up1 == up2 {
        return Ok(Faced::Apart);
    }
    let component = &section.components()[f1.component];
    let (lo, hi) = component.span;
    let speed = f1.tangent.norm();
    let forward = if up1 { f2.t - f1.t } else { f1.t - f2.t };
    let ahead = Margin::metered(forward, geom_core::InfSpeed::new(speed));
    let travel = match decide("bool_join_foot_ahead", ahead, band).map_err(join_escalate)? {
        Sign::Positive => forward,
        Sign::Negative if component.closed() => forward + (hi - lo),
        Sign::Negative => return Ok(Faced::Apart),
        Sign::Zero => return Err(desync("two distinct sites have one foot on their section")),
    };
    Ok(Faced::Along(Along {
        travel,
        speed,
        site: p2,
    }))
}

/// Mutual germ facing along the germ LOCUS. Straight germ lines take
/// the chord test: both dirs definitely point at each other along the
/// chord (Zero = definite non-facing; in-band escalates in `decide`).
/// On a CONIC germ locus the chord test is structurally degenerate — a
/// semicircle arc leaves BOTH sites exactly perpendicular to the chord
/// — so the arc-aware test asks whether the two germs bound ONE
/// rotational traversal of the section conic: do their
/// [`rotational_sense`]s differ ([`Turn::facing`])? A radial germ is a loud desync there,
/// never a silent non-match. Two germs on different loops of the
/// section never face: each loop is its own traversal.
fn facing<T: Decide>(
    loops: Option<&SectionLoops<T>>,
    g1: &HalfGerm<T>,
    g2: &HalfGerm<T>,
    p1: geom_core::Point3<T>,
    p2: geom_core::Point3<T>,
    band: Band,
) -> Result<Faced<T>, BooleanError> {
    match loops {
        None => {
            let chord = p2 - p1;
            // Facing margins in METRES: unit germ dir · chord = cos ×
            // separation (rim-dimensional audit: the former `/ dist`
            // stripped the metres and compared a bare cosine against
            // the length band — class (c)).
            let f1 = g1.dir.dot(chord);
            let f2 = g2.dir.dot(-chord);
            let faces = decide("bool_join_facing", Margin::of(f1), band).map_err(join_escalate)?
                == Sign::Positive
                && decide("bool_join_facing", Margin::of(f2), band).map_err(join_escalate)?
                    == Sign::Positive;
            Ok(if faces { Faced::Straight } else { Faced::Apart })
        }
        Some(loops) => {
            if g1.branch != g2.branch {
                return Ok(Faced::Apart);
            }
            let f1 = loops.pick(g1.branch)?;
            // One loop, one frame.
            Ok(
                match Turn::facing(f1.turn(), (p1, g1.dir), (p2, g2.dir), band)? {
                    Some(turn) => Faced::Conic(turn),
                    None => Faced::Apart,
                },
            )
        }
    }
}

type LooseMap = SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>;

/// Still-unused null-edge halves, each mapped to its geometric MATCH
/// PARTNER's half in the same solid: the nearest loose germ along its
/// line ([`nearer_along`]) that is its partner by [`partners`] —
/// [`find_match`]'s own criterion,
/// static in the germ geometry, which [`capture_rank`] reads to rank
/// what a chord arc captures. Germ meta is shared between the solids,
/// so the (record, slot) partner relation is computed once (A-clone
/// points — coincident copies) and translated per solid. A one-site
/// loop's two halves, which [`partners`] never pairs (they are one
/// record's), are each other's: its segment ([`one_site_segments`])
/// runs from one round the loop to the other. A loose half with no
/// partner maps to `None` (conservatively separated wherever captured).
fn loose_partners<T: Decide>(
    open: &[OpenRecord<T>],
    segments: &[SectionSegment<T>],
    red: &BooleanReduction<T>,
    sa: &Sides,
    sb: &Sides,
    band: Band,
) -> Result<(LooseMap, LooseMap), BooleanError> {
    let loose: Vec<(usize, usize)> = open
        .iter()
        .enumerate()
        .flat_map(|(i, r)| (0..2).filter(move |&s| !r.a[s].1).map(move |s| (i, s)))
        .collect();
    let mut a_map: LooseMap = SecondaryMap::new();
    let mut b_map: LooseMap = SecondaryMap::new();
    for &(i, s) in &loose {
        let mut best: Option<(Reach<T>, (usize, usize))> = None;
        for &(j, t) in &loose {
            // Across records, the matcher's own criterion ([`partners`]):
            // the separation constraint must see the pairs the matcher
            // will take, or roles get walled off wrongly.
            let Some(reach) = partners(open, red, sa, sb, (i, s), (j, t), band)? else {
                continue;
            };
            best = keep_nearer(best, (reach, (j, t)), |c, b| nearer_along(c, b, band))?;
        }
        let partner = best.map(|(_, jt)| jt);
        a_map.insert(open[i].a[s].0.he, partner.map(|(j, t)| open[j].a[t].0.he));
        b_map.insert(open[i].b[s].0.he, partner.map(|(j, t)| open[j].b[t].0.he));
    }
    for seg in segments {
        let [(r, s), (q, t)] = seg.ends;
        // Defence in depth, unreachable in effect: a one-site record's
        // two slots are consumed together and `bool_connect` cuts it at
        // once, so a consumed pair's halves are keys no live loop holds.
        // The guard keeps the map's domain to unused halves.
        if r != q || open[r].a[s].1 {
            continue;
        }
        for (map, side) in [(&mut a_map, &open[r].a), (&mut b_map, &open[r].b)] {
            map.insert(side[s].0.he, Some(side[t].0.he));
            map.insert(side[t].0.he, Some(side[s].0.he));
        }
    }
    Ok((a_map, b_map))
}

/// Chooses the join role order for one solid (the enforced
/// discipline; module docs for the derivation). The three lanes:
///
/// - **Different loops** (the mekr lane): a pure loop merge — role
///   order is orientation-neutral; keep the given order. Loops of two
///   faces take this lane too, and what runs on the plan's face
///   refuses them ([`JoinPlan::of`]).
/// - **Same loop, the face's OUTER**: the split partitions real
///   boundary between two faces; either partition names the same two
///   directed cycles (role order moves only face identity), so the
///   order is chosen by what the first chord's mef run `[h1 .. h2]`
///   captures of the still-loose halves alone ([`best_arc`]). Both arcs
///   separating a pair is a loud desync.
/// - **Same loop, a RING of its face** (the closed seam-ring lane —
///   pierce-ring scaffolding): the split's remainder stays a ring of
///   the old face and must anti-enclose (a hole boundary), so the mef
///   run — the enclosed patch, the new face's outer, closed by the
///   segment's own curve — must wind CCW around the face's outward
///   normal. Decided intrinsically by [`ring_run_ccw`] once the curve is
///   known ([`RoleLane::resolve`]). A derived order whose run captures
///   a loose half without its partner is a loud desync.
///
/// Cross-solid consistency needs NO coupling of the two solids' role
/// orders: the sense attributes carry the seam orientation (the
/// anti-correlation theorem), and each solid's partition is decided
/// against its own geometry — the zip's antiparallelism assertion is
/// the runtime witness.
fn choose_roles<T: crate::props::AtRestPolicy>(
    (body, operand): (&Body<T>, Operand),
    (ea, ra): (HalfEdgeKey, HalfEdgeKey),
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
    segment: Option<EdgeKey>,
    closure: RingClosure<T>,
    band: Band,
) -> Result<RoleLane<T>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let loop_of = |he: HalfEdgeKey| -> Result<LoopKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or(desync("role half no longer resolves"))?
            .parent_loop)
    };
    let l = loop_of(ea)?;
    if l != loop_of(ra)? {
        return Ok(RoleLane::Decided((ea, ra)));
    }
    let face = body
        .get_loop(l)
        .ok_or(desync("role loop no longer resolves"))?
        .face;
    let outer = body
        .get_face(face)
        .ok_or(desync("role face no longer resolves"))?
        .outer;
    if l == outer {
        // Both arcs dirty is refused loudly, never resolved.
        return best_arc(body, ea, ra, loose)?
            .map(RoleLane::Decided)
            .ok_or(desync("every chord arc separates a loose scaffolding pair"));
    }
    // Ring lane: decided by the run's winding. A planar, sphere, cone or
    // torus face's run is closed by the segment's curve, so it waits on
    // that curve ([`RoleLane::resolve`]), and so does a carried chord's
    // cylinder, sphere or torus face; a cylinder face's on a planar section
    // closes along its section plane on its chart, decided here, before
    // the curve is computed in the order it decides. An along-edge
    // segment reads no section: a curved face there refuses typed,
    // before any chord is computed.
    enum RingFace<T: geom_core::Real> {
        Plane(Vec3<T>),
        Wall((Point3<T>, UnitVec3<T>)),
        Quadric,
        Carried,
    }
    let surface = || face_surface(body, face).map(|(_, s)| s);
    let ring = match closure {
        RingClosure::Wall(section) => match crate::ring_path::Quadric::of(surface()?) {
            Some(crate::ring_path::Quadric::Cylinder { .. }) | None => RingFace::Wall(section),
            Some(_) => RingFace::Quadric,
        },
        RingClosure::Chord => match crate::ring_path::Quadric::of(surface()?) {
            Some(
                crate::ring_path::Quadric::Cylinder { .. }
                | crate::ring_path::Quadric::Sphere { .. }
                | crate::ring_path::Quadric::Torus { .. },
            ) => RingFace::Carried,
            _ => {
                return Err(BooleanError::CurvedBooleanUnsupported {
                    operand,
                    face,
                    kind: surface()?.kind(),
                });
            }
        },
        RingClosure::Planar => RingFace::Plane(
            face_outward_normal(body, face)
                .ok_or(desync(
                    "a planar germ face's ring lane found no planar carrier",
                ))?
                .vec(),
        ),
        RingClosure::AlongEdge(on) => match face_outward_normal(body, face) {
            Some(normal) => RingFace::Plane(normal.vec()),
            None => {
                return Err(BooleanError::CurvedBooleanUnsupported {
                    operand: on,
                    face,
                    kind: surface()?.kind(),
                });
            }
        },
    };
    // Halves either side of the segment's own edge, `x → edge → y`: the
    // joiner mints one chord in either order — the first in `(x, y)`,
    // the second in `(y, x)`, whose first the adjacency skip drops — so
    // the order moves nothing, and the run, the edge closed by its
    // copy, is a sliver no winding orients. That is read off the
    // joiner's own plans, not assumed: the two orders must mint the one
    // same site, or the match is refused, and the run the minted
    // chord's `mef` walls off must rank [`Capture::Clean`].
    for (x, y) in [(ea, ra), (ra, ea)] {
        let sites = |order| {
            JoinPlan::of(body, order, SegmentEdge::Locus(segment), band)
                .map(|plan| plan.sites())
                .map_err(BooleanError::from)
        };
        let across = match (sites((x, y))?, sites((y, x))?) {
            ((Some(a), None), (None, Some(b))) if a == b => true,
            ((Some(_), None), _) | (_, (None, Some(_))) => {
                return Err(desync(
                    "the two role orders of a match across its segment's edge mint different \
                     chords",
                ));
            }
            _ => false,
        };
        if across {
            return match capture_rank(body, (x, y), loose)? {
                Capture::Clean => Ok(RoleLane::Decided((x, y))),
                Capture::RingHeld | Capture::Separates => Err(desync(
                    "a match across its segment's edge separates a loose scaffolding pair",
                )),
            };
        }
    }
    match ring {
        RingFace::Plane(normal) => Ok(RoleLane::Ring { face, normal }),
        RingFace::Quadric => Ok(RoleLane::QuadricRing { face }),
        RingFace::Carried => Ok(RoleLane::CarriedRing { face }),
        RingFace::Wall(section) => {
            let ccw = ring_run_ccw(
                (body, operand),
                face,
                (ea, ra),
                IslandClosing::Wall(section),
                band,
            )?;
            ring_order(body, (ea, ra), loose, ccw).map(RoleLane::Decided)
        }
    }
}

/// What [`choose_roles`] makes of a match on one solid before the
/// segment's curve is computed: the role order, or the ring lane, whose
/// order the run's winding decides.
#[derive(Clone, Copy)]
enum RoleLane<T: geom_core::Real> {
    /// The mekr lane's given order, the outer lane's clean one, or a
    /// wall ring's, wound on its chart before the curve.
    Decided((HalfEdgeKey, HalfEdgeKey)),
    /// A ring of the planar `face`, with its outward `normal`.
    Ring { face: FaceKey, normal: Vec3<T> },
    /// A ring of the sphere, cone or torus `face`.
    QuadricRing { face: FaceKey },
    /// A ring of the cylinder, sphere or torus `face` a carried chord
    /// closes.
    CarriedRing { face: FaceKey },
}

impl<T: crate::props::AtRestPolicy> RoleLane<T> {
    /// The order the segment's curve is computed in: the decided one, or
    /// the match's own on the ring lane, where either order names the
    /// germs' one arc.
    fn curve_order(&self, given: (HalfEdgeKey, HalfEdgeKey)) -> (HalfEdgeKey, HalfEdgeKey) {
        match *self {
            RoleLane::Decided(order) => order,
            RoleLane::Ring { .. } | RoleLane::QuadricRing { .. } | RoleLane::CarriedRing { .. } => {
                given
            }
        }
    }

    /// The role order. On the ring lane it is fully determined by the
    /// face's own orientation — the mef run (the enclosed patch, the new
    /// face's outer) must wind CCW around the face's outward normal so
    /// the remainder ring anti-encloses, closed by the segment's `curve`
    /// ([`ring_run_ccw`]). Exactly one of the two orders satisfies it
    /// (the candidate runs are antiparallel copies). Nothing outside the
    /// run is read: mid-fixpoint, a face hosting several pending polygons
    /// has regions other pending seams still separate from the island's
    /// surround, so an anchor on the face's outer loop could classify the
    /// wrong one.
    fn resolve(
        self,
        (body, operand): (&Body<T>, Operand),
        (ea, ra): (HalfEdgeKey, HalfEdgeKey),
        loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
        curve: &SegmentCurve<T>,
        band: Band,
    ) -> Result<(HalfEdgeKey, HalfEdgeKey), BooleanError> {
        let (face, closing) = match self {
            RoleLane::Ring { face, normal } => (face, IslandClosing::Planar(normal, curve)),
            RoleLane::QuadricRing { face } => (face, IslandClosing::Quadric(curve)),
            RoleLane::CarriedRing { face } => (face, IslandClosing::Carried(curve)),
            RoleLane::Decided(_) => return Ok(self.curve_order((ea, ra))),
        };
        let ccw = ring_run_ccw((body, operand), face, (ea, ra), closing, band)?;
        ring_order(body, (ea, ra), loose, ccw)
    }
}

/// The ring lane's role order from the island's winding: CCW keeps the
/// match's order. A derived order whose run is not [`Capture::Clean`]
/// is a loud desync.
fn ring_order<T: Decide>(
    body: &Body<T>,
    (ea, ra): (HalfEdgeKey, HalfEdgeKey),
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
    ccw: bool,
) -> Result<(HalfEdgeKey, HalfEdgeKey), BooleanError> {
    let (h1, h2) = if ccw { (ea, ra) } else { (ra, ea) };
    match capture_rank(body, (h1, h2), loose)? {
        Capture::Clean => Ok((h1, h2)),
        Capture::RingHeld | Capture::Separates => Err(BooleanError::JoinDesync {
            what: "derived ring role order separates a loose scaffolding pair",
        }),
    }
}

/// Whether the prospective mef run `[h1 .. h2]` — the `next`-order arc
/// from `h1` through `h2`, closed by the segment's `curve` from `end(h2)`
/// back to `start(h1)` (exactly the cycle the joiner's first
/// `mef(Chords { he1: h1, he2: next(h2, tol) })` walls off as the new
/// face, on the curve that `mef` mints) — winds CCW around `face`'s
/// outward normal: the orientation an island's new outer loop must have
/// (the remainder ring anti-encloses iff the run encloses). A conic
/// chord closes it by its own arc: the straight chord across the arc
/// side of a section 2-gon closes the run along the other side's copy,
/// which encloses nothing in either order.
///
/// Reified (issue #93): decided through the `bool_ring_run_winding`
/// predicate by [`Body::planar_run_winding_decided`] — the one home of
/// the sum, its dimension (`2A/P`, audit F4) and its orientation rule,
/// `crate::loop_winding`'s module docs, which the merge's role assigner
/// and `validate`'s tier-3 check 6 read for a stored loop. `normal` is
/// the face's OUTWARD normal, read by [`choose_roles`] through
/// [`face_outward_normal`] with the sense folded in; the run's stored
/// traversal carries the other sign. `Indeterminate` escalates. Zero is
/// a degenerate area-free run and a loud desync (the ring lane only
/// closes full island cycles — slit-growing joins are mekr-lane
/// merges). A spline run edge refuses first
/// ([`BooleanError::EdgeCarrierUnsupported`], naming `operand`'s edge
/// and `face`) on every closing but a carried chord's, whose path reads
/// one where it is certified on the face
/// ([`crate::ring_path::SplineArc`]). A run the sum does not wind otherwise
/// refuses by the reason it names ([`crate::loop_winding::Unwound`]): a
/// spline closing curve on a plane, which no section lane mints there,
/// loudly; a spiric
/// at a scalar that does not certify, typed
/// ([`SplitJoinError::RingIslandUnread`]).
///
/// A wall face ([`IslandClosing::Wall`]) asks the same question on its
/// own chart, [`crate::chord_join::chart_island_winding`], with the run
/// closed along the plane this solid's chords lie in; a sphere, cone or
/// torus face ([`IslandClosing::Quadric`]) asks it without a chart,
/// [`crate::chord_join::path_island_winding`], closed by the curve, and
/// a carried chord's cylinder, sphere or torus face
/// ([`IslandClosing::Carried`]) the same, closed by the chord's arc.
fn ring_run_ccw<T: crate::props::AtRestPolicy>(
    (body, operand): (&Body<T>, Operand),
    face: FaceKey,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
    closing: IslandClosing<'_, T>,
    band: Band,
) -> Result<bool, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    if !matches!(closing, IslandClosing::Carried(_))
        && let Some(edge) = run_spline_edge(body, (h1, h2))?
    {
        return Err(BooleanError::EdgeCarrierUnsupported {
            operand,
            edge,
            face,
            face_of: operand,
            site: super::EdgeCarrierSite::RingRun,
        });
    }
    let (normal, curve) = match closing {
        IslandClosing::Wall(section) => {
            let wound =
                crate::chord_join::chart_island_winding(body, face, (h1, h2), section, band)?;
            return ring_winding_order(wound);
        }
        IslandClosing::Quadric(curve) => {
            let closing = curve.run_closing(h1, face)?;
            let wound = crate::chord_join::path_island_winding(
                body,
                face,
                (h1, h2),
                closing.as_ref(),
                band,
            )?;
            return ring_winding_order(wound);
        }
        IslandClosing::Carried(curve) => {
            let closing = curve.run_closing(h1, face)?;
            let wound = crate::chord_join::path_island_winding_closed_by(
                body,
                face,
                (h1, h2),
                (closing.as_ref(), curve.carried_arc()),
                band,
            )?;
            return ring_winding_order(wound);
        }
        IslandClosing::Planar(normal, curve) => (normal, curve),
    };
    let closing = curve.run_closing(h1, face)?;
    let wound = body
        .planar_run_winding_decided((h1, h2), RunClosing::of(closing.as_ref()), normal, band)
        .map_err(|RunMissesEnd| desync("ring-run arc did not close"))?
        .map_err(|why| {
            BooleanError::from(match why {
                // The run holds none (above), so the spline is the
                // closing curve, and no section lane mints one on a plane.
                Unwound::Spline => SplitJoinError::SectionInvariant {
                    face,
                    what: "the ring lane's closing section curve is a spline",
                },
                Unwound::Uncertified => SplitJoinError::RingIslandUnread {
                    face,
                    kind: geom::SurfaceKind::Plane,
                },
            })
        })?;
    ring_winding_order(wound.map(|decided| decided.sign))
}

/// The split root of the first spline (NURBS) edge on the run `h1 → h2`
/// of a loop: no closing but a carried chord's winds one
/// ([`ring_run_ccw`]). A spiric run edge is read by the
/// windings themselves, on the carriers that bear one.
fn run_spline_edge<T: Decide>(
    body: &Body<T>,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
) -> Result<Option<EdgeKey>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let cycle = body
        .loop_cycle(h1)
        .ok_or(desync("ring-run half no longer resolves"))?;
    let end = cycle
        .iter()
        .position(|&he| he == h2)
        .ok_or(desync("ring-run end is not on its start's loop"))?;
    for &he in &cycle[..=end] {
        let edge = body
            .get_half_edge(he)
            .ok_or(desync("ring-run half no longer resolves"))?
            .edge;
        let curve = body
            .get_edge(edge)
            .and_then(|e| body.get_curve_geom(e.curve))
            .ok_or(desync("a ring-run edge carries no curve"))?;
        // A null edge is scaffolding, wound as its chord.
        let spline = match curve {
            crate::null::CurveGeom::Certified(c) => matches!(c.carrier(), geom::Curve3::Nurbs(_)),
            crate::null::CurveGeom::NullScaffold(_) => false,
        };
        if spline {
            return body
                .split_root(edge, |_| false)
                .map(Some)
                .map_err(BooleanError::SplitLineage);
        }
    }
    Ok(None)
}

/// A degenerate ring-run winding ([`DEGENERATE_RING_WINDING`]) at a run
/// of `side`'s from `h1` through `h2` that passes the point of a ring
/// whose cones touch along a ray in its face
/// ([`super::PierceRingRecord::tied`]): the corner of no angle the tie
/// leaves, the unbuilt shape, refused [`BooleanError::PinchRaysInAFace`]
/// naming that ring's face; one that passes the tip of a slit or ring
/// of either operand's ([`crate::sector_shape::SlitTips`]): `side`'s own, or
/// the other's, paired with the run's vertex there or piercing at its
/// ring vertex, refused
/// [`BooleanError::SlitFaceGlued`]. Every other error, and a degenerate
/// run that passes neither, is returned as it is.
fn pinch_ray_refusal<T: geom_core::Real>(
    e: BooleanError,
    red: &BooleanReduction<T>,
    side: Operand,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
) -> BooleanError {
    if !matches!(e, BooleanError::JoinDesync { what } if what == DEGENERATE_RING_WINDING) {
        return e;
    }
    let body = match side {
        Operand::A => &red.a,
        Operand::B => &red.b,
    };
    let point = |v: VertexKey| body.get_vertex(v).map(|d| d.point);
    // The run's vertices, `h1` through `h2` in `next` order, and `h2`'s
    // end.
    let mut on_run = std::collections::BTreeSet::new();
    let mut run_vertices = Vec::new();
    let mut h = h1;
    for _ in 0..body.half_edges.len() {
        let Some(half) = body.get_half_edge(h) else {
            break;
        };
        on_run.extend(point(half.start));
        run_vertices.push(half.start);
        if h == h2 {
            on_run.extend(body.half_edge_end(h).and_then(point));
            run_vertices.extend(body.half_edge_end(h));
            break;
        }
        h = half.next;
    }
    red.pierce_rings
        .iter()
        .filter(|r| r.tied && r.operand == side)
        .find(|r| point(r.ring_vertex).is_some_and(|k| on_run.contains(&k)))
        .map(|r| BooleanError::PinchRaysInAFace {
            operand: side.other(),
            face: r.face,
        })
        // A run through the tip of a slit, of `side`'s or, at a vertex
        // pair, of the other's, whose carrying face the other solid glues
        // away.
        .or_else(|| {
            slit_tip_on(red, side, run_vertices).map(|(operand, face)| {
                BooleanError::SlitFaceGlued {
                    operand,
                    face,
                    gap: super::SlitFaceGap::RingRun,
                }
            })
        })
        .unwrap_or(e)
}

/// The slit or ring whose tip one of `side`'s `vertices` is
/// ([`crate::sector_shape::SlitTips`]), as its operand and carrying face:
/// `side`'s own at the vertex, else the other's at a vertex the other
/// holds there, a vertex pair's partner or a pierce the vertex's ring
/// serves. A null edge's copy of a vertex reads as the vertex.
fn slit_tip_on<T: geom_core::Real>(
    red: &BooleanReduction<T>,
    side: Operand,
    vertices: impl IntoIterator<Item = VertexKey>,
) -> Option<(Operand, FaceKey)> {
    let copies = super::NullCopies::of_operand(&red.null_edges, side);
    vertices
        .into_iter()
        .flat_map(|v| copies.of(v))
        .find_map(|v| {
            let paired = red
                .contacts
                .vv
                .iter()
                .filter_map(|c| match side {
                    Operand::A => (c.record.a == v).then_some(c.record.b),
                    Operand::B => (c.record.b == v).then_some(c.record.a),
                })
                .chain(
                    red.pierce_rings
                        .iter()
                        .filter(|r| r.operand == side && r.ring_vertex == v)
                        .flat_map(|r| r.pierces.iter().copied()),
                )
                .map(|w| (side.other(), w));
            [(side, v)]
                .into_iter()
                .chain(paired)
                .find_map(|(operand, w)| {
                    red.slit_tips[usize::from(operand == Operand::B)]
                        .face_at(w)
                        .map(|face| (operand, face))
                })
        })
}

/// **The ring lane's order where its run walls off a sliver along a
/// glued slit.** A ring run that encloses no area
/// ([`DEGENERATE_RING_WINDING`]) and passes a slit's tip
/// ([`pinch_ray_refusal`]'s [`BooleanError::SlitFaceGlued`]) runs along
/// the slit's two edges, or their copies on the face the other solid
/// glues to the slit's: either order walls off the one sliver between
/// them, a face of no area, and nothing winds it. The order is the
/// outer lane's, the arc of better [`Capture`] ([`best_arc`]); the
/// sliver's side is read with the section's other loops
/// ([`resolve_roles_geometric`]), which says what the result makes of
/// it. A run both of whose arcs separate a pending pair refuses
/// [`BooleanError::SlitFaceGlued`] ([`super::SlitFaceGap::ArcsSeparate`]);
/// every other refusal is [`pinch_ray_refusal`]'s.
fn glued_slit_roles<T: Decide>(
    e: BooleanError,
    red: &BooleanReduction<T>,
    side: Operand,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
) -> Result<(HalfEdgeKey, HalfEdgeKey), BooleanError> {
    match pinch_ray_refusal(e, red, side, (h1, h2)) {
        BooleanError::SlitFaceGlued { operand, face, .. } => {
            let body = match side {
                Operand::A => &red.a,
                Operand::B => &red.b,
            };
            best_arc(body, h1, h2, loose)?.ok_or(BooleanError::SlitFaceGlued {
                operand,
                face,
                gap: super::SlitFaceGap::ArcsSeparate,
            })
        }
        other => Err(other),
    }
}

/// The ring lane's refusal of a run that encloses no area.
const DEGENERATE_RING_WINDING: &str = "ring-run winding is degenerate (zero enclosed area)";

/// The ring lane's role order from the decided winding: CCW keeps the
/// order. A zero area is a degenerate run, so its in-band twin is the
/// kernel's too.
fn ring_winding_order(wound: Result<Sign, geom_core::Indeterminate>) -> Result<bool, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let sign = wound.map_err(|diag| BooleanError::Escalated {
        decision: BooleanDecision::SelfCheck(SelfCheck::RingWinding),
        diag,
    })?;
    match sign {
        Sign::Positive => Ok(true),
        Sign::Negative => Ok(false),
        Sign::Zero => Err(desync(DEGENERATE_RING_WINDING)),
    }
}

/// What a chord arc's mef run captures of the still-loose halves, worst
/// first: an arc ranks by the worst capture it makes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Capture {
    /// Every loose half on the arc has its partner on the arc too: the
    /// pair still shares a face and joins there.
    Clean,
    /// Some loose half's partner lies off the arc, on another ring of the
    /// face. The mef re-homes that ring by geometry ([`ChordJoiner`]'s
    /// `rehome_rings`) to the side its segment's other end lies on, which
    /// is the arc's side as long as section segments in one face do not
    /// cross.
    RingHeld,
    /// Some loose half has no partner, or its partner lies on the split
    /// loop off the arc or outside the face's rings: the run walls it off
    /// from its partner.
    Separates,
}

/// The rank of the mef run the `next`-order arc `h1 → h2` walls off as
/// the new face ([`Capture`]). Halves on two loops split nothing and rank
/// [`Capture::Clean`].
fn capture_rank<T: Decide>(
    body: &Body<T>,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
) -> Result<Capture, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let loop_of = |he: HalfEdgeKey| -> Result<crate::entity::LoopKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or(desync("role half no longer resolves"))?
            .parent_loop)
    };
    let split = loop_of(h1)?;
    if split != loop_of(h2)? {
        return Ok(Capture::Clean);
    }
    let face = body
        .get_loop(split)
        .ok_or(desync("role loop no longer resolves"))?
        .face;
    let rings = &body
        .get_face(face)
        .ok_or(desync("role face no longer resolves"))?
        .rings;
    let mut inside: Vec<HalfEdgeKey> = Vec::new();
    let mut he = body
        .get_half_edge(h1)
        .ok_or(desync("role arc start no longer resolves"))?
        .next;
    let mut steps = 0usize;
    while he != h2 {
        if loose.contains_key(he) {
            inside.push(he);
        }
        he = body
            .get_half_edge(he)
            .ok_or(desync("role arc left the loop"))?
            .next;
        steps += 1;
        if steps > body.half_edges().count() {
            return Err(desync("role arc did not close"));
        }
    }
    let mut worst = Capture::Clean;
    for &h in &inside {
        let sib = match loose.get(h).copied().flatten() {
            None => return Ok(Capture::Separates),
            Some(sib) if inside.contains(&sib) => continue,
            Some(sib) => sib,
        };
        let held = loop_of(sib)?;
        if held == split || !rings.contains(&held) {
            return Ok(Capture::Separates);
        }
        worst = Capture::RingHeld;
    }
    Ok(worst)
}

/// The outer lane's role order (doc at [`choose_roles`]): the arc of
/// better [`Capture`] rank, `ea → ra` on a tie; `None` when both
/// separate. A [`Capture::Clean`] arc is preferred to a ring-held one,
/// so a ring-held arc is taken only where neither arc is clean: at exact
/// ties the ring-held choice of a split that has a clean one flips
/// tier-3 verdicts, which `the_outer_lane_prefers_a_clean_arc_at_an_exact_tie`
/// (sweep) guards.
fn best_arc<T: Decide>(
    body: &Body<T>,
    ea: HalfEdgeKey,
    ra: HalfEdgeKey,
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
) -> Result<Option<(HalfEdgeKey, HalfEdgeKey)>, BooleanError> {
    let (fwd, back) = (
        capture_rank(body, (ea, ra), loose)?,
        capture_rank(body, (ra, ea), loose)?,
    );
    Ok(match fwd.min(back) {
        Capture::Separates => None,
        best if fwd == best => Some((ea, ra)),
        _ => Some((ra, ea)),
    })
}

/// Cut the corresponding null edges in both solids; completions must
/// coincide (lockstep). Roles stay UNRESOLVED here (resolved at
/// quiescence — see `bool_connect`).
fn cut_pair<T: Decide>(
    red: &mut BooleanReduction<T>,
    sa: &mut SolidJoin,
    sb: &mut SolidJoin,
    completed: &mut Vec<UnresolvedPair>,
    a_edge: EdgeKey,
    b_edge: EdgeKey,
    tol: Tol,
) -> Result<(), BooleanError> {
    let a_out = sa.joiner.cut_core(&mut red.a, a_edge, tol)?;
    let b_out = sb.joiner.cut_core(&mut red.b, b_edge, tol)?;
    match (a_out, b_out) {
        (CutOutcome::Merged, CutOutcome::Merged) => Ok(()),
        (
            CutOutcome::Completed {
                face: a_face,
                ring: a_ring,
            },
            CutOutcome::Completed {
                face: b_face,
                ring: b_ring,
            },
        ) => {
            let desync = |what| BooleanError::JoinDesync { what };
            let a_outer = red
                .a
                .get_face(a_face)
                .ok_or(desync("completed A face no longer resolves"))?
                .outer;
            let b_outer = red
                .b
                .get_face(b_face)
                .ok_or(desync("completed B face no longer resolves"))?
                .outer;
            completed.push(UnresolvedPair {
                a_face,
                a_outer,
                a_ring,
                b_face,
                b_outer,
                b_ring,
            });
            Ok(())
        }
        _ => Err(BooleanError::JoinDesync {
            what: "one solid completed a polygon where the other merged slivers",
        }),
    }
}

/// GEOMETRIC loop-role resolution for a completed section polygon: a
/// loop of the 2-loop null face is the IN copy iff the region faces
/// holding its chords' mates lie inside the OTHER operand, read by the
/// cell-dimension witness ladder ([`complex_side`]) against the
/// pristine other operand. Probing waits for quiescence, so no
/// crossing runs through a region face and the ladder's premise holds.
/// Strut side labels are not consulted. They are derived like every
/// null half's (the module docs' sense theorem; a ring strut is minted
/// by the same rule), and they fix which way each loop's chords run, so
/// reading the roles from geometry alone keeps the zip's
/// [`BooleanError::SeamOrientation`] a check of the labels against an
/// independent reading rather than against themselves.
///
/// What this question adds to the ladder is [`loop_roles`]: the two
/// loops' regions flank the seam, so their sides are opposite.
///
/// **A sliver along the other's slit reads Out**, where neither loop
/// reads a side and no witness met a limit, the one case [`loop_roles`]
/// would refuse [`SplitJoinError::SectionLoopUndecided`]; a limit keeps
/// its refusal. Where the other solid
/// glues away the face a slit or ring of its lies in, `side`'s face
/// flush with it is cut along the slit's two edges, and one loop's
/// regions are the slivers between them, faces of no area
/// ([`sliver_along_a_slit`]). The ladder reads neither loop: the flush
/// region lies on the other boundary, and a sliver has no interior. A
/// slit is a gap of no width in its face, opening on the region between
/// its two walls, which the other solid's material does not fill (tier
/// 3: the material about the line is the two wedges, face | wall a and
/// wall b | face). So the sliver, the face's piece across that gap,
/// lies outside the other solid, and the flush region on the opposite
/// side.
///
/// Where the op keeps the sliver, the output stage pairs the slit's two
/// walls across it into one edge ([`super::slit::merge_slivers`]); where
/// it keeps the flush region instead, the fate stage cuts the dropped
/// one of the two flush faces (`fate::cut`).
///
/// **A loop whose regions no witness reads** reads its regions' germs
/// at its seam instead ([`seam_germ_side`]), where a limit met nothing:
/// a region face narrower than the band holds no point off it.
fn resolve_roles_geometric<T: Decide + crate::props::AtRestPolicy>(
    red: &BooleanReduction<T>,
    side: Operand,
    other_pristine: &Body<T>,
    face: FaceKey,
    (outer, ring): (LoopKey, LoopKey),
    band: Band,
    tol: Tol,
) -> Result<(LoopKey, LoopKey), BooleanError> {
    let body = match side {
        Operand::A => &red.a,
        Operand::B => &red.b,
    };
    let read = |l: LoopKey| -> Result<Reading, BooleanError> {
        complex_side(
            body,
            &region_faces(body, face, l)?,
            (other_pristine, side.other()),
            band,
            tol,
        )
    };
    let (mut o, mut r) = (read(outer)?, read(ring)?);
    // Only where [`loop_roles`] would refuse `SectionLoopUndecided`:
    // a limit either loop's witnesses met still refuses as its own.
    let undecided = |x: &Reading| matches!(x, Reading::Undecided(t) if !t.kept.met_a_limit());
    if undecided(&o) && undecided(&r) {
        if sliver_along_a_slit(red, side, face, ring)? {
            r = Reading::Side(SideCode::Out);
        } else if sliver_along_a_slit(red, side, face, outer)? {
            o = Reading::Side(SideCode::Out);
        }
    }
    if undecided(&o) && undecided(&r) {
        for (l, reading) in [(outer, &mut o), (ring, &mut r)] {
            if let Some(s) = seam_germ_side(body, face, l, other_pristine, band)? {
                *reading = Reading::Side(s);
            }
        }
    }
    loop_roles(face, (outer, o), (ring, r))
}

/// Whether every region face flanking section loop `l` of null face
/// `face`, on `side`, is a sliver ([`super::slit::sliver_halves`]) along
/// one slit or ring of the other operand's: both its ends are tips of
/// that operand's slits in one face ([`slit_tip_on`]), so its one
/// segment is the slit's line.
fn sliver_along_a_slit<T: Decide>(
    red: &BooleanReduction<T>,
    side: Operand,
    face: FaceKey,
    l: LoopKey,
) -> Result<bool, BooleanError> {
    let body = match side {
        Operand::A => &red.a,
        Operand::B => &red.b,
    };
    let mut slit = None;
    for f in region_faces(body, face, l)? {
        let Some(halves) = super::slit::sliver_halves(body, f) else {
            return Ok(false);
        };
        let tips = halves.map(|h| {
            body.get_half_edge(h)
                .and_then(|d| slit_tip_on(red, side, [d.start]))
        });
        match tips {
            [Some(a), Some(b)] if a == b && a.0 == side.other() && slit.is_none_or(|s| s == a) => {
                slit = Some(a);
            }
            _ => return Ok(false),
        }
    }
    Ok(slit.is_some())
}

/// **The side a section loop's regions leave its seam on**, where no
/// witness of theirs decides. A chord lies on the other boundary; where
/// its midpoint lies strictly inside one face `F` of `other`, the
/// material beside it is the half-space behind `F`, and the region face
/// `R` the chord's mate bounds leaves the chord along its inward
/// direction ([`crate::stands::inward`]). `R` crosses no surface of
/// `other` (probing waits for quiescence), so it lies on the side of
/// `F` that direction points to: Out where its dot with `F`'s outward
/// normal is positive, In where negative, read at the chord's length.
/// Zero is a region flush with `F`, and reads nothing; so does a chord
/// on a curve, a midpoint that no one face holds strictly inside, or
/// any margin in band. Every chord that reads is read, so where the
/// chord walk starts cannot matter.
///
/// A planar region face whose rim lies on the other boundary holds no
/// witness off the band where it is narrower than `2Kε`, and its one
/// interior candidate may lie in band where it is wider; its germ is the
/// angle of two carriers that cross. The dihedral's lever door
/// (`geom_brep::classify_dihedral`) does not serve: it reads `|sin θ|`,
/// whether two carriers cross, and not which side of one the other
/// leaves to.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where two chords read the
/// loop's regions on opposite sides: they flank one crossing, so that is
/// a kernel defect; and the walk's and [`holding_face_normal`]'s own.
fn seam_germ_side<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    l: LoopKey,
    other: &Body<T>,
    band: Band,
) -> Result<Option<SideCode>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let point = |v: VertexKey| {
        body.get_vertex(v)
            .and_then(|vd| body.get_point(vd.point).copied())
            .ok_or(desync("section vertex has no point"))
    };
    let mut side = None;
    for (mate, region) in region_halves(body, face, l)? {
        let m = body
            .get_half_edge(mate)
            .ok_or(desync("section mate no longer resolves"))?;
        if !crate::census::edge_is_line(body, m.edge) {
            continue;
        }
        let Ok((_, n_r)) = super::solid_contain::face_plane(body, region) else {
            continue;
        };
        let end = body
            .half_edge_end(mate)
            .ok_or(desync("section mate has no end"))?;
        let (p0, p1) = (point(m.start)?, point(end)?);
        let len = p0.distance(p1);
        let into_region = crate::stands::inward(n_r, (p1 - p0) * (T::from_f64(1.0) / len));
        let Some(n_f) = holding_face_normal(other, p0.lerp(p1, T::from_f64(0.5)), band)? else {
            continue;
        };
        let read = match decide(SEAM_GERM_SIDE, Margin::of(into_region.dot(n_f) * len), band) {
            Ok(Sign::Positive) => SideCode::Out,
            Ok(Sign::Negative) => SideCode::In,
            Ok(Sign::Zero) | Err(_) => continue,
        };
        if side.is_some_and(|s| s != read) {
            return Err(BooleanError::ClassificationInvariant {
                what: "a section loop's chords read its regions on both sides of the other operand",
            });
        }
        side = Some(read);
    }
    Ok(side)
}

/// [`seam_germ_side`]'s sign of a region's germ against a face.
const SEAM_GERM_SIDE: &str = "bool_join_seam_germ_side";
/// [`holding_face_normal`]'s on-plane margin.
const SEAM_GERM_ON_PLANE: &str = "bool_join_seam_germ_on_plane";

/// The outward normal at `q` of the one face of `other` that holds `q`
/// strictly inside ([`super::contain::face_placement`]); `None` where no
/// face does, two do, or a placement or the normal reads nothing there.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] where the holding face no longer
/// resolves, and [`BooleanError::ClassificationInvariant`] where its
/// normal reads `q` off the surface its placement put `q` on.
///
/// # Panics
///
/// On a stale face, as [`super::ops::place_witness`] does.
fn holding_face_normal<T: Decide>(
    other: &Body<T>,
    q: Point3<T>,
    band: Band,
) -> Result<Option<Vec3<T>>, BooleanError> {
    use super::contain::{CurvedPlacement, FaceContainment, PlacementError};
    let mut held = None;
    for (f, _) in other.faces() {
        match super::contain::face_placement(other, f, q, band, SEAM_GERM_ON_PLANE) {
            Ok(CurvedPlacement::Trim(Some(FaceContainment::In))) => {
                if held.replace(f).is_some() {
                    return Ok(None);
                }
            }
            Ok(CurvedPlacement::OffCarrier | CurvedPlacement::Trim(Some(FaceContainment::Out))) => {
            }
            Err(PlacementError::Door(super::ContainError::StaleFace(face))) => {
                super::contain::driver_face_stale(face)
            }
            Ok(CurvedPlacement::Trim(_)) | Err(_) => return Ok(None),
        }
    }
    let Some(held) = held else {
        return Ok(None);
    };
    match crate::face_normal::face_outward_normal_at(other, held, q, band) {
        Ok(Some(n)) => Ok(Some(n.vec())),
        Ok(None) => Err(BooleanError::JoinDesync {
            what: "the face holding a section chord no longer resolves",
        }),
        Err(crate::face_normal::NormalAtError::OffSurface) => {
            Err(BooleanError::ClassificationInvariant {
                what: "a section chord's holding face reads it off the surface it was placed on",
            })
        }
        Err(_) => Ok(None),
    }
}

/// The (IN, OUT) loop order from each loop's ladder reading. Either
/// loop's verdict fixes both roles; a loop whose regions lie ON the
/// other boundary (a declared flush face) reads undecided, and the
/// other loop decides.
///
/// **Agreeing verdicts are a kernel defect**, refused
/// [`SplitJoinError::SectionLoopMixed`]. A section polygon is minted
/// only where the other boundary crosses the face with opposed senses,
/// and each reading comes from a point of an uncut region, so the two
/// sides differ unless the join minted a seam that is not a crossing (a
/// tangential contact), left a crossing uncut, or bound a strut's halves
/// to the wrong germs — each chord then lies on the wrong side of its
/// rim and both loops' regions are pieces of one cap
/// (`join1_delta_probes::overlapping_lens_prisms_declared_union_builds`
/// pins the pose; `insert::strut_faces_first` is the rule it holds).
///
/// **Neither deciding** is the ladder's frontier, refused
/// [`SplitJoinError::SectionLoopUndecided`]: every witness of both
/// loops' regions read the other boundary or too near it, and no chord
/// read a germ ([`seam_germ_side`]). A crossing's two flanks can both do
/// that only where no region face offers an interior witness off it.
/// One such face is curved
/// (`work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior`).
/// Another is planar, with an inward line from each edge's midpoint that
/// meets no line or conic carrier beyond the band (`crate::stands`,
/// rung 3), such as one whose nearest boundary there is a spline or a
/// spiric.
/// No in-band reading is named as the cause: it is about one point. A
/// witness refused near a face the door cannot read is, as it is for a
/// shell ([`super::shell_witness`]).
fn loop_roles(
    face: FaceKey,
    (outer, o): (LoopKey, Reading),
    (ring, r): (LoopKey, Reading),
) -> Result<(LoopKey, LoopKey), BooleanError> {
    let in_first = |s: SideCode, l: LoopKey, m: LoopKey| {
        if s == SideCode::In { (l, m) } else { (m, l) }
    };
    match (o, r) {
        (Reading::Side(o), Reading::Side(r)) if o == r => {
            Err(BooleanError::from(SplitJoinError::SectionLoopMixed {
                face,
            }))
        }
        (Reading::Side(o), _) => Ok(in_first(o, outer, ring)),
        (Reading::Undecided(_), Reading::Side(r)) => Ok(in_first(r, ring, outer)),
        // One ranking over both loops' witnesses ([`crate::ray_walk::Evidence`]):
        // a limit is named, an in-band reading is not (it is about one
        // point), so both of those and an empty one read undecided.
        (Reading::Undecided(o), Reading::Undecided(r)) => Err(match o.kept.then(r.kept).ranked() {
            crate::ray_walk::Ranked::Blocked(e) => BooleanError::Containment(e),
            crate::ray_walk::Ranked::InBand(_) | crate::ray_walk::Ranked::Neither => {
                BooleanError::from(SplitJoinError::SectionLoopUndecided { face })
            }
        }),
    }
}

/// The region faces flanking section loop `l` of null face `face`: the
/// faces holding its half-edges' mates, each once, in loop order.
fn region_faces<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    l: LoopKey,
) -> Result<Vec<FaceKey>, BooleanError> {
    let mut out: Vec<FaceKey> = Vec::new();
    for (_, region) in region_halves(body, face, l)? {
        if !out.contains(&region) {
            out.push(region);
        }
    }
    Ok(out)
}

/// Each half-edge of section loop `l` of null face `face` as its mate
/// and the region face that mate bounds, in loop order; a mate on `face`
/// itself is left out.
fn region_halves<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    l: LoopKey,
) -> Result<Vec<(HalfEdgeKey, FaceKey)>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let LoopBoundary::Cycle { first } = body
        .get_loop(l)
        .ok_or(desync("completed section loop no longer resolves"))?
        .boundary
    else {
        return Err(desync("completed section loop is empty"));
    };
    let mut out = Vec::new();
    for ch in body
        .loop_cycle(first)
        .ok_or(desync("completed section loop not walkable"))?
    {
        let mate = body.mate(ch).ok_or(desync("section half has no mate"))?;
        let region_loop = body
            .get_half_edge(mate)
            .ok_or(desync("section mate no longer resolves"))?
            .parent_loop;
        let region = body
            .get_loop(region_loop)
            .ok_or(desync("region loop no longer resolves"))?
            .face;
        if region != face {
            out.push((mate, region));
        }
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod loop_roles_rows {
    use super::super::shell_witness::Reading;
    use super::super::{BooleanError, SideCode};
    use super::loop_roles;
    use crate::chord_join::SplitJoinError;
    use crate::entity::{FaceKey, LoopKey};
    use crate::stands::Tally;
    use slotmap::SlotMap;

    fn keys() -> (FaceKey, LoopKey, LoopKey) {
        let mut loops: SlotMap<LoopKey, ()> = SlotMap::with_key();
        (FaceKey::default(), loops.insert(()), loops.insert(()))
    }

    fn side(s: SideCode) -> Reading {
        Reading::Side(s)
    }

    fn undecided() -> Reading {
        Reading::Undecided(Tally::default())
    }

    /// Each loop's reading, alone or with its opposite, fixes the same
    /// order; agreeing readings refuse as the kernel's defect; two
    /// undecided loops refuse without naming a witness's cause.
    #[test]
    fn the_two_loops_readings_fix_the_roles_or_refuse() {
        use SideCode::{In, Out};
        let (face, o, r) = keys();
        for (label, a, b, want) in [
            ("outer In", side(In), undecided(), (o, r)),
            ("outer Out", side(Out), undecided(), (r, o)),
            ("ring In", undecided(), side(In), (r, o)),
            ("ring Out", undecided(), side(Out), (o, r)),
            ("opposite", side(In), side(Out), (o, r)),
        ] {
            assert_eq!(loop_roles(face, (o, a), (r, b)).unwrap(), want, "{label}");
        }
        for s in [In, Out] {
            let err = loop_roles(face, (o, side(s)), (r, side(s))).unwrap_err();
            assert!(
                matches!(
                    err,
                    BooleanError::Join(ref j) if matches!(j.error(), SplitJoinError::SectionLoopMixed { face: f } if *f == face)
                ),
                "both {s:?}: {err:?}"
            );
        }
        let err = loop_roles(face, (o, undecided()), (r, undecided())).unwrap_err();
        assert!(
            matches!(
                err,
                BooleanError::Join(ref j) if matches!(j.error(), SplitJoinError::SectionLoopUndecided { face: f } if *f == face)
            ),
            "neither decides: {err:?}"
        );
        // The curved-face frontier reads as what the Boolean cannot yet
        // do, not as a kernel defect.
        let text = err.to_string();
        assert!(
            text.ends_with(geom_core::NOT_YET_ENDING) && !text.contains("kernel"),
            "{text}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod travel_rows {
    //! **One germ's two partners on an ellipse's flank, in walk order.**
    //! The germ leaves `(0, 1)` on the ellipse `(k cos t, sin t)` clockwise
    //! (toward `+x`); its two partners lie on the lower flank at
    //! `t = −π/4` and `Δ` further on. There the chord from the germ
    //! shrinks along the walk, so the chord picks the far one, and the
    //! axis-plane distance reads `Δ sin ψ`: 0.198Δ at `k = 10`, 0.033Δ at
    //! `k = 60`. `Δ` is in bands of the run's ε.

    use super::{BooleanError, GermArm, Reach, Turn, germ_arm, nearer_along};
    use core::f64::consts::FRAC_PI_4;
    use geom_core::{Band, Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The point at `t` and the unit tangent counter-clockwise there,
    /// the way a partner germ faces back toward the germ.
    fn at(k: f64, t: f64) -> (Point3<f64>, Vec3<f64>) {
        let tangent = Vec3::new(-k * t.sin(), t.cos(), 0.0);
        (
            Point3::new(k * t.cos(), t.sin(), 0.0),
            tangent * (1.0 / tangent.norm()),
        )
    }

    /// The partners `[near, far]`, `bands` bands of arc apart.
    fn partners(k: f64, bands: f64) -> [Reach<f64>; 2] {
        let germ = Point3::new(0.0, 1.0, 0.0);
        let near = -FRAC_PI_4;
        let speed = (k * k * 0.5 + 0.5).sqrt();
        let far = near - bands * band().zero() / speed;
        [near, far].map(|t| {
            let (site, tangent) = at(k, t);
            Reach {
                arm: GermArm::Ahead,
                chord: (site - germ).norm(),
                turn: Turn::facing(
                    (Point3::origin(), Vec3::unit_z()),
                    (germ, Vec3::unit_x()),
                    (site, tangent),
                    band(),
                )
                .unwrap(),
                along: None,
            }
        })
    }

    /// The near partner is nearer along the walk read from either side,
    /// where the chord says the far one is: at `k = 10` two dozen bands
    /// apart, which the axis-plane distance leaves in the escalation
    /// band, and at `k = 60` a hundred, which it reads as 3.3 bands.
    #[test]
    fn the_partner_nearer_along_the_walk_is_taken_where_the_chord_says_the_other() {
        for (k, bands) in [(10.0, 24.0), (60.0, 100.0)] {
            let [near, far] = partners(k, bands);
            assert!(far.chord < near.chord, "k = {k}: the chord must disagree");
            assert!(
                matches!(nearer_along(near, far, band()), Ok(true)),
                "k = {k}: near against far: {:?}",
                nearer_along(near, far, band()).err()
            );
            assert!(
                matches!(nearer_along(far, near, band()), Ok(false)),
                "k = {k}: far against near: {:?}",
                nearer_along(far, near, band()).err()
            );
        }
    }

    /// **The sites' slack escalates where it hides the order.** At
    /// `k = 60` the flank's `cot ψ` is 30, so two sites each up to ε off
    /// the conic may move the arc reading by 60 bands: two dozen bands
    /// apart escalate at `bool_join_arc_clear`, read from either side.
    /// On a circle the slack is zero and the same separation orders.
    #[test]
    fn two_sites_within_the_slack_of_a_steep_flank_escalate() {
        let [near, far] = partners(60.0, 24.0);
        for (a, b) in [(near, far), (far, near)] {
            assert!(
                matches!(
                    nearer_along(a, b, band()),
                    Err(BooleanError::Escalated { ref diag, .. })
                        if diag.predicate == Some("bool_join_arc_clear")
                ),
                "{:?}",
                nearer_along(a, b, band())
            );
        }
        let [near, far] = partners(1.0, 24.0);
        assert!(
            matches!(nearer_along(near, far, band()), Ok(true)),
            "circle"
        );
    }
    /// **A partner at the germ's half-turn reads `Ahead` only where the
    /// slack is within the band.** The germ at `t = −π/4` turning
    /// clockwise, its partner at the antipode: the margin is zero, a tie,
    /// which stands on a circle (`cot ψ` 0) and escalates at `k = 10`,
    /// where `cot ψ` is 4.95 there.
    #[test]
    fn a_partner_at_the_half_turn_reads_ahead_only_where_the_slack_is_in_band() {
        let arm = |k: f64| {
            let (germ, dir) = at(k, -FRAC_PI_4);
            let (site, tangent) = at(k, -FRAC_PI_4 + core::f64::consts::PI);
            let turn = Turn::facing(
                (Point3::origin(), Vec3::unit_z()),
                (germ, dir * -1.0),
                (site, tangent),
                band(),
            )
            .unwrap();
            assert!(turn.is_some(), "the germ and its partner face");
            germ_arm(turn, band())
        };
        assert!(matches!(arm(1.0), Ok(GermArm::Ahead)), "circle");
        assert!(
            matches!(
                arm(10.0),
                Err(BooleanError::Escalated { ref diag, .. })
                    if matches!(diag.predicate, Some("bool_join_arc_slack" | "bool_join_arc_clear"))
            ),
            "k = 10: {:?}",
            arm(10.0).err()
        );
    }
}

/// **The join's self-checks refuse as the kernel's own**, at their
/// sites: a mutant routing either to a coincidence reds here.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod self_check_rows {
    use super::super::{BooleanDecision, BooleanError, HalfGerm, SelfCheck};
    use super::{ChordJoiner, JoinLane};
    use geom_core::{Band, KERNEL_DEFECT_ENDING, Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn assert_defect(err: &BooleanError, check: SelfCheck, predicate: &str) {
        assert!(
            matches!(
                err,
                BooleanError::Escalated { decision: BooleanDecision::SelfCheck(c), diag }
                    if *c == check && diag.predicate == Some(predicate)
            ),
            "{check:?}: {err:?}"
        );
        assert!(
            err.to_string().ends_with(KERNEL_DEFECT_ENDING),
            "{check:?}: {err}"
        );
    }

    /// A conic germ whose rotational sense about its section circle lies
    /// in the band: a radial germ direction is malformed germ data, so
    /// its in-band twin is the kernel's too.
    #[test]
    fn an_in_band_arc_facing_is_the_kernels_own_check() {
        let b = band();
        let w = (b.zero() + b.escalate()) / 2.0;
        let germ = |dir| HalfGerm {
            he: crate::entity::HalfEdgeKey::default(),
            a_face: crate::entity::FaceKey::default(),
            b_face: crate::entity::FaceKey::default(),
            a_locus: super::super::Locus::InFace(crate::entity::FaceKey::default()),
            b_locus: super::super::Locus::InFace(crate::entity::FaceKey::default()),
            dir,
            branch: super::super::Branch::Only,
        };
        let (o, z) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let err = super::germs_face_each_other(
            Some(&super::SectionLoops::conic((o, z))),
            &germ(Vec3::new(1.0, w, 0.0)),
            &germ(Vec3::new(0.0, 1.0, 0.0)),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            b,
        )
        .expect_err("an in-band sense escalates");
        assert_defect(&err, SelfCheck::ArcFacing, "bool_join_arc_facing");
    }

    /// A ring run on a prism's top face whose enclosed mean width lies in
    /// the band: the run `(0,0) → (1,h) → (2,0)`, closed by its chord, is
    /// `2A/P ≈ h/2` wide. A zero width is a degenerate run, so its
    /// in-band twin is the kernel's too. (The run is hand-picked on the
    /// face; whether a join reaches one is not asked here.)
    #[test]
    fn an_in_band_ring_run_winding_is_the_kernels_own_check() {
        let tol = Tol::witness();
        let b = band();
        let h = b.zero() + b.escalate();
        let prism = crate::test_support_fixtures::prism_z::<f64>(
            &[(0.0, 0.0), (1.0, h), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
            0.0,
            1.0,
            tol,
        );
        let body = &prism.body;
        let start = |he| body.get_half_edge(he).unwrap().start;
        let face = body.get_face(prism.top_face).unwrap();
        let crate::LoopBoundary::Cycle { first } = body.get_loop(face.outer).unwrap().boundary
        else {
            panic!("the top face's outer loop is a cycle");
        };
        let cycle = body.loop_cycle(first).unwrap();
        let h1 = *cycle
            .iter()
            .find(|&&he| {
                start(he) == prism.top[0]
                    && start(body.get_half_edge(he).unwrap().next) == prism.top[1]
            })
            .expect("the top loop runs (0,0) → (1,h)");
        let h2 = body.get_half_edge(h1).unwrap().next;
        let joiner = ChordJoiner::new(b);
        let plan = joiner
            .plan(body, (h1, h2), crate::chord_join::SegmentEdge::Locus(None))
            .expect("the plan reads the prism's top loop");
        let chord = joiner
            .segment_curve(
                &mut body.clone(),
                &plan,
                JoinLane::Planar,
                crate::chord_join::Leave {
                    at: [
                        (h1, Vec3::new(1.0, 0.0, 0.0)),
                        (h2, Vec3::new(-1.0, 0.0, 0.0)),
                    ],
                    datum: crate::chord_join::Datum::Germ,
                },
            )
            .expect("a planar face's chord is straight")
            .expect("the adjacent chords' between edges are no segment locus");
        let up = super::IslandClosing::Planar(Vec3::new(0.0, 0.0, 1.0), &chord);
        let err = super::ring_run_ccw(
            (body, super::super::Operand::A),
            prism.top_face,
            (h1, h2),
            up,
            b,
        )
        .expect_err("an in-band winding escalates");
        assert_defect(&err, SelfCheck::RingWinding, "bool_ring_run_winding");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod frame_dispatch_tests {
    use geom_core::{Point3, Tol, Vec3};

    use super::{
        FrameError, FrameExtent, frame_refusal, pair_section_frame, pair_section_frame_at,
        section_unbuilt, serves_cylinder_nodes,
    };

    /// The point the rows read the frame's axis rows at.
    fn at() -> Point3<f64> {
        Point3::new(0.0, 0.0, 0.0)
    }

    fn band() -> geom_core::Band {
        geom_core::Band::linear(Tol::witness()).expect("a linear band")
    }

    /// **The germ frame reads at the curved face, not the plane's.** The
    /// plane `z = 0` and a unit cylinder along `x` resting on it, tilted
    /// half the zero band, the wall's boundary vertices about the origin
    /// and the table's centred 1000 m along the axis. Read at the wall
    /// ([`super::frame_reading`]), the gap is the radius: the tangent
    /// ruling's frame (`Ok(None)`), in both face orders. Read at the
    /// table, the gap moved by `1000·θ` and the walls parted (`Empty`,
    /// refused).
    #[test]
    fn the_germ_frame_reads_at_the_wall_not_the_table() {
        let plane = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let tilt = 0.5 * Tol::witness().eps();
        let axis = Vec3::new(1.0, 0.0, tilt).normalize();
        let cyl = geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0),
            axis,
            radius: 1.0,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        let table: Vec<Point3<f64>> = [(995.0, -5.0), (1005.0, -5.0), (1005.0, 5.0), (995.0, 5.0)]
            .iter()
            .map(|&(x, y)| Point3::new(x, y, 0.0))
            .collect();
        let wall: Vec<Point3<f64>> = [-0.5, 0.5]
            .iter()
            .flat_map(|&x| [Point3::new(x, 0.0, 0.0), Point3::new(x, 0.0, 2.0)])
            .collect();
        for (label, a, b, on_a, on_b) in [
            ("plane, wall", &plane, &cyl, table.clone(), wall.clone()),
            ("wall, plane", &cyl, &plane, wall.clone(), table.clone()),
        ] {
            let (at, span) = super::frame_reading(a, b, on_a, on_b).expect("a reading");
            let got = pair_section_frame(a, b, at, span, band());
            assert!(
                matches!(got, Ok(None)),
                "({label}): the tangent ruling's frame, got {:?}",
                got.as_ref().map_err(|e| match e {
                    FrameError::Escalated { diag: d, .. } => d.predicate.unwrap_or("unnamed"),
                    _ => "a refusal",
                })
            );
        }
    }

    /// **The germ frame levers a cylinder pair at its walls' span.** Two
    /// unit cylinders with parallel axes 2 apart along `x`, the second
    /// tilted `0.15·zero`, their walls' boundary vertices 10 m long.
    /// Across the walls' span ([`super::frame_reading`]: the diameter of
    /// their vertices' ball) the tilt reads `1.5·zero`, in the band, and
    /// the frame escalates on the table's `cc_axes_parallel` in both
    /// orders, refused as the axes' own decision. At half the span it
    /// reads `0.75·zero` and at the radius `0.15·zero`, both Zero.
    #[test]
    fn the_germ_frame_levers_a_cylinder_pair_at_its_walls_span() {
        let theta = 0.15 * Tol::witness().eps();
        let c1 = cylinder_at(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), 1.0);
        let c2 = cylinder_at(
            Point3::new(0.0, 2.0, 0.0),
            Vec3::new(theta.cos(), theta.sin(), 0.0),
            1.0,
        );
        let wall = |y: f64| vec![Point3::new(-5.0, y, 0.0), Point3::new(5.0, y, 0.0)];
        for (label, a, b, on_a, on_b) in [
            ("c1, c2", &c1, &c2, wall(1.0), wall(1.0)),
            ("c2, c1", &c2, &c1, wall(1.0), wall(1.0)),
        ] {
            let (at, span) = super::frame_reading(a, b, on_a, on_b).expect("a reading");
            let got = pair_section_frame(a, b, at, span, band());
            assert!(
                matches!(
                    got,
                    Err(FrameError::Escalated {
                        decision: crate::boolean::BooleanDecision::CylinderAxes,
                        diag: ref d,
                    }) if d.predicate == Some("cc_axes_parallel")
                ),
                "({label}): the tilt across the walls is in band, got {:?}",
                got.as_ref().map_err(|e| match e {
                    FrameError::Escalated { diag: d, .. } => {
                        d.predicate.unwrap_or("unnamed")
                    }
                    _ => "a refusal",
                })
            );
            let Err(e) = got else { unreachable!() };
            let face = crate::entity::FaceKey::default();
            let err = frame_refusal(e, (face, a), (face, b));
            assert!(
                matches!(
                    err,
                    crate::boolean::BooleanError::Escalated {
                        decision: crate::boolean::BooleanDecision::CylinderAxes,
                        ..
                    }
                ),
                "({label}): the axes' angle is its own decision, got {err:?}"
            );
        }
    }

    /// **A cylinder pair's axes read by their lever**
    /// ([`super::cylinder_pair_axes`]). Unit walls about the `x` axis and
    /// about an axis through `(0, 2, 0)`: tilted `0.15·zero` off `x`, it
    /// reads parallel at the radius and in the band over a 10 m span;
    /// along `z`, apart at either lever.
    #[test]
    fn cylinder_pair_axes_read_parallel_apart_and_in_band_by_their_lever() {
        use super::{CylinderAxes, FrameExtent, cylinder_pair_axes};
        let theta = 0.15 * Tol::witness().eps();
        let x = (Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let tilted = (
            Point3::new(0.0, 2.0, 0.0),
            Vec3::new(theta.cos(), theta.sin(), 0.0),
        );
        let crossing = (Point3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let read =
            |other, extent| match cylinder_pair_axes(x, other, (1.0, 1.0), at(), extent, band()) {
                Ok(CylinderAxes::Parallel) => "parallel",
                Ok(CylinderAxes::Apart(_)) => "apart",
                Err(FrameError::Escalated {
                    decision: crate::boolean::BooleanDecision::CylinderAxes,
                    diag: d,
                }) if d.predicate == Some("cc_axes_parallel") => "in band",
                Err(_) => "another refusal",
            };
        for (other, extent, want, what) in [
            (
                tilted,
                FrameExtent::Radii,
                "parallel",
                "the tilt at the radius",
            ),
            (
                tilted,
                FrameExtent::Span(10.0),
                "in band",
                "the tilt over 10 m",
            ),
            (
                crossing,
                FrameExtent::Radii,
                "apart",
                "crossing at the radius",
            ),
            (
                crossing,
                FrameExtent::Span(10.0),
                "apart",
                "crossing over 10 m",
            ),
        ] {
            assert_eq!(read(other, extent), want, "{what}");
        }
    }

    /// **The germ frame reads at the curved face, wherever the wall's
    /// origin is stored.** The plane `z = 0` and a unit cylinder along
    /// `x` resting on it, tilted half the zero band, its face's boundary
    /// vertices about the origin and its origin stored 1000 m out along
    /// its axis either way. Read at the face ([`super::frame_reading`]),
    /// the tilt is in the zero band at the radius's lever and the gap is
    /// the radius: the tangent ruling, the straight chord's frame
    /// (`Ok(None)`). Read at the stored origin, the gap moved by
    /// `1000·θ`, five hundred times the band: the walls part on one side
    /// (`Empty`, refused) and cross on the other.
    #[test]
    fn the_germ_frame_reads_at_the_face_wherever_the_origin_is_stored() {
        let plane = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let table: Vec<Point3<f64>> = [(-5.0, -5.0), (5.0, -5.0), (5.0, 5.0), (-5.0, 5.0)]
            .iter()
            .map(|&(x, y)| Point3::new(x, y, 0.0))
            .collect();
        let tilt = 0.5 * Tol::witness().eps();
        let axis = Vec3::new(1.0, 0.0, tilt).normalize();
        let wall: Vec<Point3<f64>> = [-0.5, 0.5]
            .iter()
            .flat_map(|&x| [Point3::new(x, 0.0, 0.0), Point3::new(x, 0.0, 2.0)])
            .collect();
        for along in [1000.0, -1000.0] {
            let cyl = geom::Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, 1.0) + axis * along,
                axis,
                radius: 1.0,
                u_ref: Vec3::new(0.0, 1.0, 0.0),
            };
            for (label, a, b, on_a, on_b) in [
                ("plane, wall", &plane, &cyl, table.clone(), wall.clone()),
                ("wall, plane", &cyl, &plane, wall.clone(), table.clone()),
            ] {
                let (at, span) = super::frame_reading(a, b, on_a, on_b).expect("a reading");
                let got = pair_section_frame(a, b, at, span, band());
                assert!(
                    matches!(got, Ok(None)),
                    "stored {along} m along ({label}): the tangent ruling's frame, got {:?}",
                    got.as_ref().map_err(|e| match e {
                        FrameError::Escalated { diag: d, .. } => d.predicate.unwrap_or("unnamed"),
                        _ => "a refusal",
                    })
                );
            }
        }
    }

    /// A frame dispatch's answer: a conic's centre and axis, or none.
    type Frame = Result<Option<(Point3<f64>, Vec3<f64>)>, FrameError>;

    /// The frame of the plane through `base` of normal `(cos β, 0, sin β)`,
    /// `sin β = k·ε/lever`, and the unit wall about `z` that `face` of
    /// `body` lies on, in both face orders, read and levered as
    /// [`super::germ_section_frame`] reads them: at the wall's boundary
    /// vertices' centre ([`super::frame_reading`]), levered at the wall's
    /// axial extent from there ([`super::frame_extent`]).
    fn wall_frames(
        body: &crate::Body<f64>,
        face: crate::entity::FaceKey,
        base: Point3<f64>,
        k: f64,
        lever: f64,
    ) -> Vec<(&'static str, Frame)> {
        let sin_beta = k * Tol::witness().eps() / lever;
        let normal = Vec3::new((1.0 - sin_beta * sin_beta).sqrt(), 0.0, sin_beta);
        plane_frames(body, face, base, normal)
    }

    /// The frame of the plane through `base` of normal `normal` and the
    /// wall `face` of `body` lies on, in both face orders, read as
    /// [`wall_frames`] reads it.
    fn plane_frames(
        body: &crate::Body<f64>,
        face: crate::entity::FaceKey,
        base: Point3<f64>,
        normal: Vec3<f64>,
    ) -> Vec<(&'static str, Frame)> {
        let plane = geom::Surface::Plane {
            origin: base,
            normal,
            u_ref: Vec3::new(0.0, 1.0, 0.0).cross(normal).normalize(),
        };
        let wall = crate::boolean::join::face_surface(body, face)
            .unwrap()
            .1
            .clone();
        let on = super::super::carrier_pair::face_witnesses(body, face).unwrap();
        [
            ("plane, wall", &plane, &wall),
            ("wall, plane", &wall, &plane),
        ]
        .into_iter()
        .map(|(label, a, b)| {
            let (on_a, on_b) = if matches!(a, geom::Surface::Plane { .. }) {
                (Vec::new(), on.clone())
            } else {
                (on.clone(), Vec::new())
            };
            let (at, span) = super::frame_reading(a, b, on_a, on_b).expect("a reading");
            let extent =
                super::frame_extent((a, body, face), (b, body, face), at, span, band()).unwrap();
            let got = super::one_loop(super::pair_section_frame_at(a, b, at, extent, band()));
            (label, got)
        })
        .collect()
    }

    /// A unit wall about `z` whose face is the ruling `(1, 0, z0)` to
    /// `(1, 0, z1)`.
    fn ruling_wall(z0: f64, z1: f64) -> (crate::Body<f64>, crate::entity::FaceKey) {
        let mut body = crate::Body::<f64>::new();
        let seed = body.mvfs(Point3::new(1.0, 0.0, z0), true).unwrap();
        body.set_face_surface(
            seed.face,
            crate::construction::FaceSurface::New {
                surface: cylinder_at(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 1.0),
                sense: true,
            },
        )
        .unwrap();
        body.mev_line(
            crate::construction::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, z1),
            Tol::witness(),
        )
        .unwrap();
        (body, seed.face)
    }

    /// The escalation's predicate, `"served"` or `"a refusal"`.
    fn verdict(got: &Frame) -> &'static str {
        match got {
            Ok(_) => "served",
            Err(FrameError::Escalated { diag: d, .. }) => d.predicate.unwrap_or("unnamed"),
            Err(_) => "a refusal",
        }
    }

    /// **The germ frame levers a plane×cylinder tilt at the wall's
    /// length, not its radius.** A unit wall about `z` whose face is the
    /// ruling 10 m long through `(1, 0, 0)`, and the plane through that
    /// point tilted so the axis meets it at `sin β = k·ε/5`. The wall
    /// reaches 5 m along the axis either side of its vertices' centre, so
    /// `pc_axis_plane_parallel` reads `k·ε` and escalates at every `k` in
    /// the band. Levered at the radius it read `k·ε/5`, Zero below
    /// `k = 5`, and named the tangent ruling's frame for a section that
    /// leaves the plane across the wall.
    #[test]
    fn a_long_walls_tilt_is_levered_at_its_length_not_its_radius() {
        let (body, face) = ruling_wall(-5.0, 5.0);
        for frac in [0.12, 0.3, 0.45, 0.6, 0.99] {
            let k = frac * Tol::witness().k();
            for (label, got) in wall_frames(&body, face, Point3::new(1.0, 0.0, 0.0), k, 5.0) {
                assert_eq!(
                    verdict(&got),
                    "pc_axis_plane_parallel",
                    "k = {k} ({label}): an in-band tilt over the wall must escalate"
                );
            }
        }
    }

    /// **A wall shorter than its radius is levered at its axial extent,
    /// not the radius.** The ruling `(1, 0, 0)` to `(1, 0, 0.2)`, read at
    /// its centre, reaches 0.1 along the axis; the plane through the
    /// centre tilted so `sin β = k·ε/0.1` reads `k·ε`, in the band, and
    /// escalates. Levered at the radius it read `10·k·ε`, definite from
    /// `k = K/10`, and served a tilted ellipse's frame.
    #[test]
    fn a_short_walls_tilt_is_levered_at_its_axial_extent_not_its_radius() {
        let (body, face) = ruling_wall(0.0, 0.2);
        for frac in [0.12, 0.5, 0.99] {
            let k = frac * Tol::witness().k();
            for (label, got) in wall_frames(&body, face, Point3::new(1.0, 0.0, 0.1), k, 0.1) {
                assert_eq!(
                    verdict(&got),
                    "pc_axis_plane_parallel",
                    "k = {k} ({label}): an in-band tilt over the wall must escalate"
                );
            }
        }
    }

    /// **A rim's bulge levers the germ frame's tilt.** The unit wall
    /// about `z` trimmed at 45°, its rim one closed ellipse on the seam
    /// vertex `(1, 0, 1)` (`oblique_rim_wall`): read at that vertex, the
    /// rim reaches 2 along the axis, so the plane through it tilted to
    /// `sin β = k·ε/2` reads `k·ε` and escalates at every `k` in the band.
    /// The vertex alone levers nothing, and the radius reads `k·ε/2`:
    /// both read Zero at `k = 1.2` and name the tangent ruling's frame.
    #[test]
    fn a_rims_bulge_levers_the_germ_frames_tilt() {
        let (body, face, _) =
            crate::test_support_fixtures::oblique_rim_wall(core::f64::consts::FRAC_PI_4);
        let base = super::super::carrier_pair::face_witnesses(&body, face).unwrap()[0];
        for frac in [0.12, 0.5, 0.99] {
            let k = frac * Tol::witness().k();
            for (label, got) in wall_frames(&body, face, base, k, 2.0) {
                assert_eq!(
                    verdict(&got),
                    "pc_axis_plane_parallel",
                    "k = {k} ({label}): an in-band tilt over the rim must escalate"
                );
            }
        }
    }

    /// A unit wall about `z` whose face is the arc of the rim trimmed at
    /// `φ` over `t ∈ [π/2 − δ, π/2 + δ]` and the ruling from the arc's end
    /// down by `h`, and that boundary sampled.
    fn arc_wall(
        phi: f64,
        delta: f64,
        h: f64,
    ) -> (crate::Body<f64>, crate::entity::FaceKey, Vec<Point3<f64>>) {
        let tol = Tol::witness();
        let (s, c) = phi.sin_cos();
        let carrier = geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(-s, 0.0, c),
            semi_u: 1.0 / c,
            semi_v: 1.0,
            u_ref: Vec3::new(c, 0.0, s),
        };
        let (t0, t1) = (
            core::f64::consts::FRAC_PI_2 - delta,
            core::f64::consts::FRAC_PI_2 + delta,
        );
        let mut body = crate::Body::<f64>::new();
        let seed = body.mvfs(carrier.eval(t0), true).unwrap();
        body.set_face_surface(
            seed.face,
            crate::construction::FaceSurface::New {
                surface: cylinder_at(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 1.0),
                sense: true,
            },
        )
        .unwrap();
        let cyl = body.get_face(seed.face).unwrap().surface;
        let plane = body.add_surface(geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(-s, 0.0, c),
            u_ref: Vec3::new(c, 0.0, s),
        });
        let end = carrier.eval(t1);
        let arc = body
            .mev(
                crate::construction::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                end,
                geom_brep::EdgeCurveSpec {
                    description: geom_brep::EdgeDescriptionSpec::Intersection {
                        pair: geom_brep::SurfacePair::new(cyl, plane),
                        selector: geom_brep::CurveSelector::Witness(
                            carrier.eval(core::f64::consts::FRAC_PI_2),
                        ),
                    },
                    carrier: carrier.clone(),
                    param_start: t0,
                    param_end: t1,
                },
                tol,
            )
            .unwrap();
        let foot = end - Vec3::new(0.0, 0.0, h);
        body.mev_line(
            crate::construction::MevSite::Fan {
                he1: arc.he_minus,
                he2: arc.he_minus,
            },
            foot,
            tol,
        )
        .unwrap();
        let samples = (0..=4000)
            .flat_map(|k| {
                let f = f64::from(k) / 4000.0;
                [carrier.eval(t0 + (t1 - t0) * f), end + (foot - end) * f]
            })
            .collect();
        (body, seed.face, samples)
    }

    /// **A partial rim levers the germ frame at its arc's reach, not the
    /// whole turn.** A unit wall whose face is a short arc of an oblique
    /// rim about its crest-free middle and a ruling (`arc_wall`), read at
    /// its vertices' centre, and the plane through the axis tilted so the
    /// face's farthest sampled axial distance `d` from there levers the
    /// tilt to `k·ε`. At `k < 1` the frame is the rulings' (`Ok(None)`): the
    /// whole turn's reach (2.7 at `φ = 1.2`, where `d` is 0.40) read the same
    /// tilts in the band and escalated. At `k = K/2` the tilt is in the band
    /// at `d` and escalates, which a lever short of the arc reads as Zero.
    #[test]
    fn a_partial_rim_levers_the_frame_at_its_arcs_reach_not_the_whole_turn() {
        let kk = Tol::witness().k();
        for (phi, delta, h) in [(1.2, 0.02, 0.5), (core::f64::consts::FRAC_PI_4, 0.05, 1.2)] {
            let (body, face, samples) = arc_wall(phi, delta, h);
            let on = super::super::carrier_pair::face_witnesses(&body, face).unwrap();
            let n = on.len() as f64;
            let at = on.iter().fold(Point3::new(0.0, 0.0, 0.0), |m, p| {
                m + (*p - Point3::new(0.0, 0.0, 0.0)) / n
            });
            let d = samples
                .iter()
                .fold(0.0_f64, |m, p| m.max((p.z - at.z).abs()));
            for k in [0.5, 0.8, 0.95, 0.5 * kk] {
                let want = if k < 1.0 {
                    "the rulings' frame"
                } else {
                    "pc_axis_plane_parallel"
                };
                for (label, got) in wall_frames(&body, face, Point3::new(0.0, 0.0, 0.0), k, d) {
                    let read = match &got {
                        Ok(None) => "the rulings' frame",
                        _ => verdict(&got),
                    };
                    assert_eq!(
                        read, want,
                        "φ = {phi}, k = {k} ({label}): levered at the arc's reach {d}"
                    );
                }
            }
        }
    }

    /// **The agreement gate never serves on the span alone.** On the
    /// declared-tangency path a plane×cylinder section serves only what
    /// the face's whole-turn reach serves too
    /// ([`crate::chord_join::agreed_section`]). Rim patches of walls of
    /// radius 1e-3, 1 and 1e3 (axes at random, 1e3 off the origin, both
    /// capped at `1e12·ε`; arcs of 1e-6 to 3 rad, heights to ten radii) are
    /// cut by a plane through a corner, tilted off a radial (or off its
    /// normal, through the axis) by a tilt near the band. What chord_join's
    /// [`crate::chord_join::wall_section`] serves at the corner, and the
    /// germ frame at the corners' centre, is what the whole-turn reading
    /// (`face_reach_round_from`, main's measure) serves at the same point,
    /// of the same class. Either lane reading the span alone serves rulings
    /// that reading escalates.
    #[test]
    fn the_agreement_gate_never_serves_on_the_span_alone() {
        use crate::chord_join::{SectionCase, WallSection, wall_section};
        use geom_brep::PlaneCylinderSection as S;
        use test_utils::fuzz;
        let eps = Tol::witness().eps();
        let b = band();
        let mut g = fuzz::start("rim_patch_served_subset");
        let unit = |g: &mut fuzz::Rng| loop {
            let v = Vec3::new(g.range(-1., 1.), g.range(-1., 1.), g.range(-1., 1.));
            if (0.2..1.0).contains(&v.norm()) {
                return v.normalize();
            }
        };
        let log = |g: &mut fuzz::Rng, lo: f64, hi: f64| (lo.ln() + (hi / lo).ln() * g.unit()).exp();
        let class = |got: &Result<S<f64>, geom_brep::SectionError>| match got {
            Ok(S::TiltedEllipse(_) | S::Rim(_)) => "conic",
            Ok(S::ParallelLines { .. }) => "lines",
            Ok(S::TangentLine(_)) => "tangent",
            Ok(S::Empty) => "empty",
            Err(_) => "refused",
        };
        for _ in 0..fuzz::scaled(1000) {
            // Radii and offsets of up to 1e3, short of where ε leaves no room
            // for the fixture to certify a rim on a random axis.
            let room = (1e12 * eps).min(1e3);
            let r = [1e-3_f64, 1.0, 1e3][g.below(3)].min(room);
            let axis = unit(&mut g);
            let seam = {
                let v = unit(&mut g);
                (v - axis * v.dot(axis)).normalize()
            };
            let frame = crate::test_support_fixtures::CylFrame {
                origin: Point3::origin() + unit(&mut g) * room,
                axis,
                radius: r,
                u_ref: seam,
            };
            // Arcs and heights at least `1e4·ε` long, where the band allows.
            let floor = |hi: f64| (1e4 * eps / r).max(1e-6).min(0.1 * hi);
            let du = log(&mut g, floor(3.0), 3.0);
            let h = log(&mut g, floor(10.0), 10.0) * r;
            let mut body = crate::Body::<f64>::new();
            let (face, _) = crate::test_support_fixtures::cyl_wall_sheet_keyed(
                &mut body,
                frame,
                crate::test_support_fixtures::CylKey::OnSeed,
                (0.0, du),
                (0.0, h),
                Tol::witness(),
            );
            let corners: Vec<_> = body.vertex_points().collect();
            let (corner, through) = corners[g.below(corners.len())];
            let radial = {
                let w = through - frame.origin;
                (w - axis * w.dot(axis)).normalize()
            };
            let w = if g.unit() < 0.3 {
                axis.cross(radial)
            } else {
                let v = unit(&mut g);
                (v - axis * v.dot(axis)).normalize()
            };
            let sin_beta = (log(&mut g, 0.05, 3.0 * Tol::witness().k()) * b.zero() / h).min(1.0);
            let normal = w * (1.0 - sin_beta * sin_beta).sqrt() + axis * sin_beta;
            let plane = geom::Surface::Plane {
                origin: through,
                normal,
                u_ref: Vec3::new(0.0, 1.0, 0.0).cross(normal).normalize(),
            };
            let wall = frame.surface::<f64>();
            let main_at = |at: Point3<f64>| {
                let rules = crate::splitting::rules::face_axial_range(&body, face, at, axis);
                let (below, above) = rules.expect("a bounded patch");
                let across = crate::splitting::rules::face_reach_round_from(&body, face, at)
                    .expect("a bounded patch");
                let reach = geom_brep::Reach::Face {
                    at,
                    below,
                    above,
                    across,
                };
                class(&geom_brep::plane_cylinder_section(&plane, &wall, &reach, b))
            };
            // chord_join, read at the corner.
            let unit_normal = geom_core::UnitVec3::new(normal, "rim patch", b).unwrap();
            let head = match wall_section(&body, b, through, unit_normal, face, corner) {
                Ok(Some(WallSection { case, .. })) => match case {
                    SectionCase::Conic(_) => "conic",
                    SectionCase::Loops { .. } => "loops",
                    SectionCase::Straight(_) => "lines",
                    SectionCase::Tangent(_) => "tangent",
                },
                _ => "refused",
            };
            let main = main_at(through);
            assert!(
                head == "refused" || head == main,
                "chord_join: head serves {head} where main reads {main} ({du} rad × {h} on r = {r}, \
                 sin β = {sin_beta}); {}",
                fuzz::replay()
            );
            // The germ frame, read at the corners' centre.
            for (label, got) in plane_frames(&body, face, through, normal) {
                let on = super::super::carrier_pair::face_witnesses(&body, face).unwrap();
                let (at, _) = super::frame_reading(&plane, &wall, Vec::new(), on).unwrap();
                let main = main_at(at);
                let served = match got {
                    Ok(Some(_)) => Some(main == "conic"),
                    Ok(None) => Some(main == "lines" || main == "tangent"),
                    Err(_) => None,
                };
                assert!(
                    served != Some(false),
                    "germ ({label}): head serves a frame main reads as {main} ({du} rad × {h} \
                     on r = {r}, sin β = {sin_beta}); {}",
                    fuzz::replay()
                );
            }
        }
    }

    /// **The germ escalates where the wall's two measures of its reach
    /// across disagree, and never names the whole turn's conic.** The
    /// `1e5·ε × 1e3·ε` patch of a 1 km wall (`chord_join`'s rim-patch row),
    /// cut by the plane through a corner and the axis tilted by
    /// `sin β = k·ε/e`: the patch stands within `k·ε` of the corner's
    /// ruling. Read over its arcs' spans the turn is Zero (the rulings);
    /// round the arcs' whole turn it reads definite (a conic). The germ is
    /// a declared-tangency path, which serves only what both measures
    /// serve, so it escalates on the split.
    #[test]
    fn a_rim_patchs_turn_is_levered_at_its_arcs_in_the_germ_frame() {
        // Pinned to limb 2's kernel limit at ε = 1e-12: the 1 km-radius rim rounds at its radius
        // (`work/offset/the-closed-form-net-builder-rounds-at-a-km-radius-not-at-the-span.md`).
        test_utils::kernel_limit::pinned_row(
            geom_core::Tol::witness().eps(),
            &[1e-12],
            "a_rim_patchs_turn_is_levered_at_its_arcs_in_the_germ_frame",
            || {
                let eps = Tol::witness().eps();
                let (r, e, w) = (1000.0, 1e3 * eps, 1e5 * eps);
                let mut body = crate::Body::<f64>::new();
                let face = crate::test_support_fixtures::cyl_wall_sheet(
                    &mut body,
                    crate::test_support_fixtures::CylFrame::canonical(r),
                    (0.0, w / r),
                    (0.0, e),
                    Tol::witness(),
                );
                for k in [0.5, 0.8, 0.95] {
                    let c = k * Tol::witness().eps() / e;
                    let normal = Vec3::new(0.0, (1.0 - c * c).sqrt(), c);
                    for (label, got) in plane_frames(&body, face, Point3::new(r, 0.0, 0.0), normal)
                    {
                        assert_eq!(
                            verdict(&got),
                            "pc_axis_plane_parallel_straddle",
                            "k = {k} ({label}): the span's rulings against the whole turn's conic"
                        );
                        let Err(FrameError::Escalated { diag, .. }) = got else {
                            unreachable!("named as an escalation")
                        };
                        // The Boolean's ending: the section question's lever alone,
                        // the straddle's, with no unreadable-margin note — the
                        // enclosure is read, where the poison it replaced was not.
                        let said = crate::boolean::BooleanError::coincidence(
                            crate::boolean::Coincide::Section,
                            crate::boolean::DeclarationRead::Moot,
                            diag,
                        )
                        .to_string();
                        assert!(
                        said.ends_with(
                            "Recourse: move the parts so they clearly meet or clearly stand apart there"
                        ) && !said.contains(geom_core::UNREADABLE_MARGIN_NOTE),
                        "k = {k} ({label}): {said}"
                    );
                    }
                }
            },
        );
    }

    /// **A face at one station is cut across the axis in a conic, by the
    /// germ frame too.** A unit wall about `z` whose face is the rim arc
    /// at `z = 0` a quarter turn from `(1, 0, 0)`, read at its vertices'
    /// centre, and the plane `z = 0`: the face reaches nothing along the
    /// axis, and the plane turns about the rulings' hinge by a right
    /// angle, which moves the face by its whole reach across the wall.
    /// The frame is the rim's; a reach across of zero read the tilt as
    /// Zero and named the rulings' straight frame.
    #[test]
    fn a_face_at_one_station_is_cut_across_the_axis_in_the_rims_frame() {
        let mut body = crate::Body::<f64>::new();
        let carrier = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
        body.set_face_surface(
            seed.face,
            crate::construction::FaceSurface::New {
                surface: cylinder_at(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 1.0),
                sense: true,
            },
        )
        .unwrap();
        let cyl = body.get_face(seed.face).unwrap().surface;
        let rim_plane = body.add_surface(geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        });
        let quarter = core::f64::consts::FRAC_PI_2;
        body.mev(
            crate::construction::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            carrier.eval(quarter),
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    pair: geom_brep::SurfacePair::new(cyl, rim_plane),
                    selector: geom_brep::CurveSelector::Witness(carrier.mid_point(0.0, quarter)),
                },
                carrier,
                param_start: 0.0,
                param_end: quarter,
            },
            Tol::witness(),
        )
        .unwrap();
        let base = Point3::new(0.0, 0.0, 0.0);
        for (label, got) in plane_frames(&body, seed.face, base, Vec3::new(0.0, 0.0, 1.0)) {
            assert!(
                matches!(got, Ok(Some(_))),
                "({label}): the rim's frame, got {}",
                verdict(&got)
            );
        }
    }

    /// **A plane×cylinder pair handed without its faces is levered at
    /// its radius, not its box** (row B). The plane `z = 0` and a coin of radius 0.01 on
    /// edge along `x`, its face 2 mm long, resting on the table at the
    /// origin and tilted so that `pc_axis_plane_parallel` levered at the
    /// radius reads `k·ε`: in the band, so the frame escalates at every
    /// `k`, and at `k = 1.2` reads Zero if the lever is cut below the
    /// radius. A lever past the radius (the coin face's box ball, 0.01418)
    /// reads the same tilt as definite from `k = 8`.
    #[test]
    fn a_coin_on_edge_is_levered_at_its_radius() {
        let plane = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let r = 0.01;
        let coin_face = Point3::new(0.001, 0.0, r);
        for k in [1.2, 6.0, 8.0, 9.0, 9.9] {
            let tilt = k * Tol::witness().eps() / r;
            let coin = geom::Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, r),
                axis: Vec3::new(1.0, 0.0, tilt).normalize(),
                radius: r,
                u_ref: Vec3::new(0.0, 1.0, 0.0),
            };
            for (label, a, b) in [
                ("plane, coin", &plane, &coin),
                ("coin, plane", &coin, &plane),
            ] {
                let got = pair_section_frame(a, b, coin_face, None, band());
                assert!(
                    matches!(
                        got,
                        Err(FrameError::Escalated { diag: ref d, .. }) if d.predicate == Some("pc_axis_plane_parallel")
                    ),
                    "k = {k} ({label}): an in-band tilt must escalate, got {:?}",
                    got.as_ref().map_err(|e| match e {
                        FrameError::Escalated { diag: d, .. } => d.predicate.unwrap_or("unnamed"),
                        _ => "a refusal",
                    })
                );
            }
        }
    }

    /// **An operand's radius guard escalates as the radius's own
    /// decision, on a real raise**: the cylinder × sphere pose reads
    /// each radius before any other question, and a cylinder whose
    /// radius lies in the band escalates there. The join's refusal of
    /// the pair's section names whose radius, its lever and the
    /// tolerance the radius gives, and offers no declaration: no face
    /// pair names an operand's own size.
    #[test]
    fn a_radius_guard_escalates_as_the_radius_decision() {
        use crate::boolean::{BooleanDecision, BooleanError, SectionRadius};
        use crate::entity::FaceKey;
        let b = band();
        let mid = (b.zero() + b.escalate()) / 2.0;
        let cyl = geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: mid,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let sph = sphere();
        let Err(e) = geom_brep::pair_section(&cyl, &sph, None, None, b) else {
            panic!("an in-band cylinder radius refuses the section");
        };
        let face = FaceKey::default();
        let err = section_unbuilt(
            crate::boolean::section_table::Uncarried::Section(&e),
            (face, cyl.kind()),
            (face, sph.kind()),
        );
        let BooleanError::Escalated { decision, diag } = err else {
            panic!("the guard escalates: {err:?}");
        };
        assert_eq!(decision, BooleanDecision::Radius(SectionRadius::Cylinder));
        assert_eq!(diag.predicate, Some("cs_cylinder_radius"));
        let text = BooleanError::Escalated { decision, diag }.to_string();
        assert_eq!(test_utils::refusal::recourse_markers(&text), 1, "{text}");
        assert!(
            test_utils::refusal::subjectless_escalations(&text).is_empty()
                && test_utils::refusal::stage_prefixes(&text, &[]).is_empty(),
            "{text}"
        );
        let k = b.escalate() / b.zero();
        assert!(
            text.starts_with("whether a cylinder's radius is positive is undecided: ")
                && text.contains(&format!(
                    "Recourse: make the cylinder's radius clearly larger than the tolerance, \
                     or, if this radius is intended, tighten the tolerance below {:e} m",
                    mid / k
                ))
                && !text.contains("declare"),
            "{text}"
        );
    }

    fn plane() -> geom::Surface<f64> {
        geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn cylinder(axis: Vec3<f64>) -> geom::Surface<f64> {
        geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis,
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn sphere() -> geom::Surface<f64> {
        geom::Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 2.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// A sphere touching [`cylinder`]'s unit wall from outside its
    /// axis at one azimuth (`R = r + d`), the transverse frame's node.
    fn node_sphere() -> geom::Surface<f64> {
        geom::Surface::Sphere {
            center: Point3::new(0.5, 0.0, 0.0),
            radius: 1.5,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// A cylinder about `axis` through `origin`. The chart's `u_ref` is
    /// any unit vector across the axis: this dispatch never reads it.
    fn cylinder_at(origin: Point3<f64>, axis: Vec3<f64>, radius: f64) -> geom::Surface<f64> {
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref: axis.orthonormal_basis().0,
        }
    }

    /// The trap, closed: a germ pair with no section arm must refuse,
    /// never answer `None`. `None` is the straight-chord verdict, and
    /// handing it to a curved pair mints a chord along a locus that is
    /// not a line.
    ///
    /// Both refusal variants are the trap being closed, but they are
    /// NOT interchangeable per row, and the matcher is split so that
    /// the loosening is confined to where it is earned. Only a
    /// CYLINDER PAIR can be served the bisector lane
    /// ([`serves_cylinder_nodes`]) — the split on coplanarity, then on
    /// radius equality, is its only producer — so those rows admit
    /// either answer here and the row below pins which one each meeting
    /// pose gets. Every other kind pair falls through to the dispatch's
    /// final arm, where `NoArm` is the only reachable answer, and a row
    /// that accepted the bisector lane there would green through a
    /// mis-routed dispatch.
    #[test]
    fn a_pair_without_an_arm_refuses_and_never_answers_straight() {
        // `cylinder_pair` marks the rows whose kinds can reach the
        // coplanarity split at all.
        let curved_pairs = [
            // Crossing axes: the locus is a space quartic (or, at equal
            // radii, the bisector ellipses) — never a line.
            (
                cylinder(Vec3::new(0.0, 0.0, 1.0)),
                cylinder(Vec3::new(1.0, 0.0, 0.0)),
                true,
            ),
            (
                cylinder(Vec3::new(0.0, 0.0, 1.0)),
                cylinder(Vec3::new(0.0, 1.0, 1.0).normalize()),
                true,
            ),
            // Skew axes, equal and unequal radii. The displacement is
            // along the common perpendicular `a1 x a2`: displacing
            // along either AXIS leaves the two lines meeting.
            (
                cylinder(Vec3::new(0.0, 0.0, 1.0)),
                cylinder_at(Point3::new(0.0, 0.5, 0.0), Vec3::new(1.0, 0.0, 0.0), 1.0),
                true,
            ),
            (
                cylinder(Vec3::new(0.0, 0.0, 1.0)),
                cylinder_at(Point3::new(0.0, 0.5, 0.0), Vec3::new(1.0, 0.0, 0.0), 0.4),
                true,
            ),
        ];
        for (a, b, cylinder_pair) in curved_pairs {
            if cylinder_pair {
                assert!(
                    serves_cylinder_nodes(&a, &b, at(), band())
                        || matches!(
                            pair_section_frame(&a, &b, at(), None, band()),
                            Err(FrameError::NoArm)
                        ),
                    "a cylinder pair with no arm must refuse rather than default to the \
                     straight chord"
                );
            } else {
                let got = pair_section_frame(&a, &b, at(), None, band());
                assert!(
                    matches!(got, Err(FrameError::NoArm)),
                    "a non-cylinder pair with no arm must refuse with NoArm exactly — \
                     the nodes variant is not reachable for these kinds"
                );
            }
        }
        // Cylinder × sphere has no frame at all: the operation's section
        // table reads it, so the dispatch reaching it is a desync, in
        // both orders and at the figure-eight's node as anywhere.
        let z = Vec3::new(0.0, 0.0, 1.0);
        for (a, b) in [
            (cylinder(z), node_sphere()),
            (node_sphere(), cylinder(z)),
            (cylinder(z), sphere()),
        ] {
            assert!(
                matches!(
                    pair_section_frame(&a, &b, at(), None, band()),
                    Err(FrameError::Desync(_))
                ),
                "a cylinder × sphere pair has no frame"
            );
        }
    }

    /// **The bisector split names each ellipse and vanishes at the
    /// nodes.** At several angles between meeting axes, every sampled
    /// point of the table's `a1 − a2`-plane ellipse reads the positive
    /// branch and picks the loop turning about that plane's normal, every
    /// point of the `a1 + a2`-plane ellipse the negative one, and the two
    /// nodes `p ± r·n̂` read no branch.
    #[test]
    fn the_bisector_split_names_each_ellipse_and_vanishes_at_the_nodes() {
        use super::super::Branch;
        use geom_brep::{EqualCylinderSection, cylinder_cylinder_section};
        let p = Point3::new(0.3, -0.2, 0.7);
        let a1 = Vec3::new(0.0, 0.0, 1.0);
        for deg in [30.0_f64, 60.0, 90.0, 140.0] {
            let t = deg.to_radians();
            let a2 = Vec3::new(t.sin(), 0.0, t.cos());
            let (sa, sb) = (cylinder_at(p, a1, 1.0), cylinder_at(p, a2, 1.0));
            let Ok(Some(loops)) = pair_section_frame_at(&sa, &sb, at(), FrameExtent::Radii, band())
            else {
                panic!("{deg}°: the pair serves its two loops");
            };
            let reach = geom_brep::Reach::Measured {
                at: at(),
                lever: 1.0,
            };
            let Ok(EqualCylinderSection::TwoEllipses { e1, e2, .. }) =
                cylinder_cylinder_section(&sa, &sb, &reach.ball(), band())
            else {
                panic!("{deg}°: the table constructs the ellipses");
            };
            for (ellipse, want) in [(&e1, Branch::Positive), (&e2, Branch::Negative)] {
                let geom::Curve3::Ellipse { axis, .. } = *ellipse else {
                    panic!("{deg}°: an ellipse carrier");
                };
                for k in 0..12 {
                    // Clear of the nodes, which stand at the minor
                    // vertices (parameters ±π/2).
                    let s = f64::from(k) * core::f64::consts::FRAC_PI_6 + 0.2;
                    let q = ellipse.eval(s);
                    let got = loops.branch_at(q, band()).expect("decided");
                    assert_eq!(got, Some(want), "{deg}°, parameter {s}");
                    let lp = loops.pick(want).expect("a loop");
                    assert!(
                        lp.axis.cross(axis).norm() < 1e-12,
                        "{deg}°: the branch picks the loop in its own plane"
                    );
                }
            }
            let n = a1.cross(a2).normalize();
            for node in [p + n, p - n] {
                assert!(
                    matches!(loops.branch_at(node, band()), Ok(None)),
                    "{deg}°: a node reads no branch"
                );
            }
        }
    }

    /// **The non-parallel cylinder pair splits on COPLANARITY, then on
    /// radius equality.** Skew axes keep the general rung's `NoArm` at
    /// any radii; intersecting axes take the nodes at equal radii and
    /// `NoArm` at unequal ones, whose locus is the skew pair's space
    /// quartic class (the table's `cc_radius_equality`).
    #[test]
    fn the_non_parallel_cylinder_pair_splits_on_coplanarity_then_radius() {
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        for (r, equal) in [(1.0_f64, true), (0.4, false)] {
            // Axes meeting at the origin, square and at a slant.
            for partner in [x, Vec3::new(0.0, 1.0, 1.0).normalize()] {
                let (sa, sb) = (
                    cylinder(z),
                    cylinder_at(Point3::new(0.0, 0.0, 0.0), partner, r),
                );
                if equal {
                    assert!(
                        serves_cylinder_nodes(&sa, &sb, at(), band()),
                        "r = {r}, partner {partner:?}: intersecting axes at equal radii serve \
                         the nodes"
                    );
                } else {
                    let got = pair_section_frame(&sa, &sb, at(), None, band());
                    assert!(
                        matches!(got, Err(FrameError::NoArm)),
                        "r = {r}, partner {partner:?}: intersecting axes at unequal radii, got {}",
                        outcome(&got)
                    );
                }
            }
            // The same pair lifted along the common perpendicular
            // `a1 x a2` — the ONE direction that separates the axes. A
            // lift along either axis leaves them meeting, which is what
            // the margin measures and why it is the gap along `a1 x a2`
            // rather than any distance between the origins.
            let skew = pair_section_frame(
                &cylinder(z),
                &cylinder_at(Point3::new(0.0, 0.5, 0.0), x, r),
                at(),
                None,
                band(),
            );
            assert!(
                matches!(skew, Err(FrameError::NoArm)),
                "r = {r}: skew axes keep the general rung"
            );
        }
        // Parallel axes are untouched by the split: still the proven
        // straight locus, at any radii and any gap.
        assert!(
            matches!(
                pair_section_frame(
                    &cylinder(z),
                    &cylinder_at(Point3::new(1.3, 0.0, 0.0), z, 0.4),
                    at(),
                    None,
                    band()
                ),
                Ok(None)
            ),
            "parallel cylinder axes meet in rulings"
        );
    }

    /// **A radius margin in band at meeting axes escalates (D4)**: it
    /// is neither the nodes nor the unequal pair's `NoArm`, each of which
    /// claims a definite class. The partner's radius is `1 + 2·zero` of
    /// the band, past the zero band and short of escalation's edge, so
    /// the pose's inner level `e − |r₁ − r₂|` (`cc_crossing_nest`, `e`
    /// zero at meeting axes) reads it in band; a dispatch that mapped
    /// that escalation to `NoArm` (or to the nodes) reds here.
    #[test]
    fn an_in_band_radius_at_meeting_axes_escalates() {
        let z = Vec3::new(0.0, 0.0, 1.0);
        let r = 1.0 + 2.0 * band().zero();
        for partner in [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 1.0).normalize(),
        ] {
            let got = pair_section_frame(
                &cylinder(z),
                &cylinder_at(Point3::new(0.0, 0.0, 0.0), partner, r),
                at(),
                None,
                band(),
            );
            assert!(
                matches!(&got, Err(FrameError::Escalated { diag, .. })
                    if diag.predicate == Some("cc_crossing_nest")),
                "partner {partner:?}, r = 1 + 2·zero: the radius margin is in band and must \
                 escalate, got {}",
                outcome(&got)
            );
        }
    }

    /// **The nodes are a property of the whole intersecting
    /// equal-radius family, not of the perpendicular pose.** The two
    /// bisector planes' common line runs through the axes' meeting
    /// point along `â₁ × â₂` and is perpendicular to both axes, so it
    /// meets both walls at `p ± r·n̂`, where `n̂ = unit(â₁ × â₂)` — the
    /// unit matters, since `‖â₁ × â₂‖ = sin θ` puts the raw product
    /// strictly inside the walls at every pose but 90°, which is why
    /// the row below normalizes — and those two points
    /// therefore lie on BOTH section ellipses, at every angle between
    /// the axes. This row reads them straight out of the section
    /// table, which is where the claim in
    /// the section table's `nodes` doc is checkable.
    #[test]
    fn the_equal_radius_section_always_crosses_at_two_nodes() {
        use geom_brep::{EqualCylinderSection, cylinder_cylinder_section};
        let p = Point3::new(0.3, -0.2, 0.7);
        let a1 = Vec3::new(0.0, 0.0, 1.0);
        for deg in [20.0_f64, 45.0, 90.0, 130.0] {
            let t = deg.to_radians();
            let a2 = Vec3::new(t.sin(), 0.0, t.cos());
            let r = 0.9;
            let c1 = geom::Surface::Cylinder {
                origin: p,
                axis: a1,
                radius: r,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let c2 = geom::Surface::Cylinder {
                origin: p,
                axis: a2,
                radius: r,
                u_ref: a2.cross(Vec3::new(0.0, 1.0, 0.0)).normalize(),
            };
            let sec =
                cylinder_cylinder_section(&c1, &c2, &geom_brep::ExtentBall::new(at(), 1.0), band())
                    .expect("intersecting equal-radius axes have a section");
            let EqualCylinderSection::TwoEllipses { e1, e2, .. } = sec else {
                panic!("{deg}°: the intersecting equal-radius section is the ellipse pair");
            };
            let n = a1.cross(a2).normalize();
            for sign in [1.0_f64, -1.0] {
                let pinch = p + n * (sign * r);
                for (name, e) in [("e1", &e1), ("e2", &e2)] {
                    let geom::Curve3::Ellipse {
                        center,
                        axis,
                        semi_u,
                        semi_v,
                        u_ref,
                    } = *e
                    else {
                        panic!("{deg}°, {name}: the section carried a non-ellipse");
                    };
                    // The eccentric anomaly of the pinch point, read
                    // off the ellipse's own frame, then evaluated back:
                    // an exact round trip iff the point is ON the
                    // curve, and no projection lane is involved.
                    let v_ref = axis.cross(u_ref);
                    let w = pinch - center;
                    let t = (w.dot(v_ref) / semi_v).atan2(w.dot(u_ref) / semi_u);
                    let d = (e.eval(t) - pinch).norm();
                    assert!(
                        d < 1e-12,
                        "{deg}°, {name}, sign {sign}: the node is {d} off the ellipse"
                    );
                }
            }
        }
    }

    /// The one pair that EARNS the straight answer, and the wired
    /// curved pairs that name a frame — so the row above is a
    /// statement about missing arms, not about the dispatch refusing
    /// everything.
    #[test]
    fn the_wired_pairs_keep_their_verdicts() {
        assert!(
            matches!(
                pair_section_frame(&plane(), &plane(), at(), None, band()),
                Ok(None)
            ),
            "plane×plane is straight by construction"
        );
        // A plane cutting a cylinder square across its axis: the rim
        // circle, whose frame is the section's own centre and axis.
        assert!(
            matches!(
                pair_section_frame(
                    &plane(),
                    &cylinder(Vec3::new(0.0, 0.0, 1.0)),
                    at(),
                    None,
                    band()
                ),
                Ok(Some(_))
            ),
            "plane×cylinder names its conic frame"
        );
        // A plane containing the axis: the section is two rulings, a
        // STRAIGHT locus that the arm proved rather than defaulted to.
        assert!(
            matches!(
                pair_section_frame(
                    &plane(),
                    &cylinder(Vec3::new(1.0, 0.0, 0.0)),
                    at(),
                    None,
                    band()
                ),
                Ok(None)
            ),
            "the parallel-lines outcome is a proven straight locus"
        );
        assert!(
            matches!(
                pair_section_frame(&plane(), &sphere(), at(), None, band()),
                Ok(Some(_))
            ),
            "plane×sphere names its circle frame"
        );
        // Two walls with parallel axes meet in RULINGS, so the straight
        // answer is proven by the axes alone — a tangent-ruling germ
        // pair at rest.
        assert!(
            matches!(
                pair_section_frame(
                    &cylinder(Vec3::new(0.0, 0.0, 1.0)),
                    &cylinder(Vec3::new(0.0, 0.0, 1.0)),
                    at(),
                    None,
                    band()
                ),
                Ok(None)
            ),
            "a parallel-axis cylinder pair is a PROVEN straight locus"
        );
    }

    /// The SPHERE pair names its section circle's frame, and it does so
    /// with the charts left alone: the frame is the LOCUS's, and a
    /// sphere pair's locus is the radical-plane circle whatever either
    /// polar axis does. The three non-loci are `Desync` rather than a
    /// frame — a germ was minted from a pair that has no curve.
    #[test]
    fn the_sphere_pair_names_its_circle_frame_at_any_chart_tilt() {
        let ball = |c: Point3<f64>, r: f64, axis: Vec3<f64>| geom::Surface::Sphere {
            center: c,
            radius: r,
            axis,
            u_ref: axis.cross(Vec3::new(0.37, -0.91, 0.18)).normalize(),
        };
        let z = Vec3::new(0.0, 0.0, 1.0);
        let tilted = Vec3::new(0.3, -0.5, 0.81).normalize();
        // Crossing, and the two charts aim nowhere near the centre
        // line: the frame is named regardless.
        for (a_axis, b_axis) in [(z, z), (z, tilted), (tilted, z)] {
            let got = pair_section_frame(
                &ball(Point3::new(0.0, 0.0, 0.0), 2.0, a_axis),
                &ball(Point3::new(2.5, 0.0, 0.0), 2.0, b_axis),
                at(),
                None,
                band(),
            );
            let Ok(Some((center, axis))) = got else {
                panic!("a crossing sphere pair names its circle frame");
            };
            // a = (d² + r₁² − r₂²)/2d = d/2 at equal radii.
            assert!((center - Point3::new(1.25, 0.0, 0.0)).norm() < 1e-12);
            assert!(axis.dot(Vec3::new(1.0, 0.0, 0.0)) > 0.999_999_999);
        }
        // Separated, tangent, and one sphere given twice: no locus, so
        // no frame — and never the straight-chord `None`.
        for (b, what) in [
            (ball(Point3::new(9.0, 0.0, 0.0), 2.0, z), "separated"),
            (ball(Point3::new(4.0, 0.0, 0.0), 2.0, z), "tangent"),
            (ball(Point3::new(0.0, 0.0, 0.0), 2.0, z), "coincident"),
        ] {
            let got = pair_section_frame(
                &ball(Point3::new(0.0, 0.0, 0.0), 2.0, z),
                &b,
                at(),
                None,
                band(),
            );
            assert!(
                matches!(got, Err(FrameError::Desync(_))),
                "{what}: a non-locus sphere pair is a desync, not a frame and not straight"
            );
        }
    }

    /// What the dispatch hands back: the frame tuple, or its absence.
    type FrameVerdict = Result<Option<(geom_core::Point3<f64>, geom_core::Vec3<f64>)>, FrameError>;

    /// `FrameError` is deliberately not `Debug` (it is an internal
    /// dispatch verdict, not a payload), so the rows below name the
    /// outcome they got through this instead of formatting it.
    fn outcome(got: &FrameVerdict) -> &'static str {
        match got {
            Ok(Some(_)) => "a frame",
            Ok(None) => "the straight-chord verdict",
            Err(FrameError::NoArm) => "NoArm",
            Err(FrameError::Desync(_)) => "a desync",
            Err(FrameError::Escalated { .. }) => "an escalation",
            Err(FrameError::OpenConic { .. }) => "an open conic",
            Err(FrameError::Spiric { .. }) => "a general spiric",
            Err(FrameError::DegenerateTorus { .. }) => "a torus off its ring convention",
        }
    }
}

/// **The cylinder×cylinder coplanarity split at the CERTIFIED scalar**
/// — the two-arm pin for the frame's coplanarity row, the section
/// table's `cc_axes_coplanar` (`geom_brep::cylinder_axes_coplanar`).
///
/// **Why it lives HERE and not in a body-level suite.** The predicate
/// sits in a dispatch that a body-level fixture only reaches after the
/// crossing layer has admitted the pair, and the skew fixture never
/// gets there: skew walls have no declared cover, so the certified lane
/// stops at a crossing-layer door with the germ pair never minted. A
/// suite that drives bodies can therefore pin ONE arm at this scalar,
/// which is not a two-arm pin at all — a mutation that broke the skew
/// half would be invisible to it. Calling
/// [`pair_section_frame`] directly is what makes both arms reachable at
/// `Interval`, and it is the same function the join calls, on the same
/// band.
///
/// Every coordinate here is DYADIC, so each surface's enclosure is a
/// point interval and the margin's width is the arithmetic's alone: the
/// axes are coordinate directions, the radii are 1 and 0.5, and the
/// skew displacement is `0.375 = 3/8`. That is what lets both arms be
/// EQUALITIES rather than "answers or escalates" — a widened enclosure
/// here would be the arithmetic's doing and is exactly what this row is
/// for.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod frame_dispatch_interval_tests {
    use geom_core::{Interval, Point3, Real, Tol, Vec3};

    use super::{FrameError, pair_section_frame, serves_cylinder_nodes};

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    /// The point the rows read the frame's axis rows at.
    fn at() -> geom_core::Point3<Interval> {
        p3(0.0, 0.0, 0.0)
    }

    fn band() -> geom_core::Band {
        geom_core::Band::linear(Tol::witness()).expect("a linear band")
    }

    fn p3(x: f64, y: f64, z: f64) -> Point3<Interval> {
        crate::test_support::identity_map(x, y, z)
    }

    fn v3(x: f64, y: f64, z: f64) -> Vec3<Interval> {
        Vec3::new(x, y, z).map(iv)
    }

    /// A wall with every datum given explicitly — no `normalize`, no
    /// `cross`, so nothing here widens before the dispatch runs. The
    /// caller passes a `u_ref` already across the axis; this dispatch
    /// never reads it, and handing it a dyadic one keeps that true at
    /// the certified scalar as well.
    fn wall(
        origin: Point3<Interval>,
        axis: Vec3<Interval>,
        u_ref: Vec3<Interval>,
        radius: f64,
    ) -> geom::Surface<Interval> {
        geom::Surface::Cylinder {
            origin,
            axis,
            radius: iv(radius),
            u_ref,
        }
    }

    fn about_z(origin: Point3<Interval>, radius: f64) -> geom::Surface<Interval> {
        wall(origin, v3(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0), radius)
    }

    fn about_x(origin: Point3<Interval>, radius: f64) -> geom::Surface<Interval> {
        wall(origin, v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), radius)
    }

    /// **The MEETING arm.** Both axes run through the origin, so the
    /// signed gap along `a₁ × a₂` is the exact zero of a point
    /// interval and the split answers `Zero`: the nodes at equal radii,
    /// the general rung at unequal ones. Read as an equality, not
    /// "nodes or escalates": an escalation here would say the enclosure
    /// decided the lane rather than the geometry.
    #[test]
    fn meeting_axes_take_the_node_door_at_the_certified_scalar() {
        for (r, equal) in [(1.0_f64, true), (0.5, false)] {
            let (sa, sb) = (
                about_z(p3(0.0, 0.0, 0.0), 1.0),
                about_x(p3(0.0, 0.0, 0.0), r),
            );
            assert!(
                if equal {
                    serves_cylinder_nodes(&sa, &sb, at(), band())
                } else {
                    matches!(
                        pair_section_frame(&sa, &sb, at(), None, band()),
                        Err(FrameError::NoArm)
                    )
                },
                "r = {r}: meeting axes at the certified scalar"
            );
        }
    }

    /// **The SKEW arm** — the half a body-level fixture cannot reach at
    /// this scalar. Lift the partner along the common perpendicular
    /// `a₁ × a₂ = ŷ` by the dyadic `0.375`: the gap is exactly `3/8`,
    /// its enclosure is a point, and the split answers a DEFINITE sign,
    /// which keeps the general rung's `NoArm`.
    ///
    /// `0.375` is not an arbitrary number. It is exactly representable,
    /// it is far above every ε row this suite is sampled at, and it is
    /// small enough that the two walls of radius 1 still overlap — a
    /// separation that also cleared the walls would be pinning the
    /// dispatch on a pair no reduction would ever hand it.
    #[test]
    fn skew_axes_keep_the_general_rung_at_the_certified_scalar() {
        for r in [1.0_f64, 0.5] {
            let got = pair_section_frame(
                &about_z(p3(0.0, 0.0, 0.0), 1.0),
                &about_x(p3(0.0, 0.375, 0.0), r),
                at(),
                None,
                band(),
            );
            assert!(
                matches!(got, Err(FrameError::NoArm)),
                "r = {r}: skew axes keep the general rung at the certified scalar"
            );
        }
        // The displacement direction is the ASSERTION, not a detail:
        // sliding along either AXIS leaves the two lines meeting, so
        // those poses must still answer the node door. A gap measured
        // as any distance between the origins would flip these to
        // `NoArm`.
        for along in [p3(0.0, 0.0, 0.75), p3(0.75, 0.0, 0.0)] {
            assert!(
                serves_cylinder_nodes(
                    &about_z(p3(0.0, 0.0, 0.0), 1.0),
                    &about_x(along, 1.0),
                    at(),
                    band()
                ),
                "{along:?}: a lift along an AXIS leaves the axes meeting"
            );
        }
    }

    /// The parallel gate ahead of the split answers `Ok(None)` at the
    /// certified scalar too, so the two arms above are reached rather
    /// than short-circuited — without this row a mutation that made the
    /// parallel test always-`Zero` would green both of them vacuously.
    #[test]
    fn parallel_axes_stay_the_proven_straight_locus_at_the_certified_scalar() {
        assert!(
            matches!(
                pair_section_frame(
                    &about_z(p3(0.0, 0.0, 0.0), 1.0),
                    &about_z(p3(1.25, 0.0, 0.0), 0.5),
                    at(),
                    None,
                    band()
                ),
                Ok(None)
            ),
            "parallel cylinder axes meet in rulings at the certified scalar"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod radical_plane_rows {
    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn wall(origin: Point3<f64>, axis: Vec3<f64>, radius: f64) -> geom::Surface<f64> {
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref: axis.orthonormal_basis().0,
        }
    }

    /// The reach of the single point `p`.
    fn at(p: Point3<f64>) -> geom_brep::Reach<f64> {
        geom_brep::Reach::Ball(geom_brep::ExtentBall::point(p))
    }

    /// **Both rulings lie in the radical plane, and it is parallel to
    /// both axes.** The rulings are found apart from the plane: the
    /// cross-section circles' two meeting points, on the axes' common
    /// perpendicular frame, each checked on both walls. Equal radii,
    /// either one larger, an axis inside the other wall, both operand
    /// orders, axes off every coordinate direction, origins slid along
    /// the axis.
    #[test]
    fn both_rulings_lie_in_the_radical_plane() {
        let axis = Vec3::new(1.0, 2.0, 0.5).normalize();
        let (e1, e2) = axis.orthonormal_basis();
        for (r1, r2, d, slide) in [
            (0.5, 0.2, 0.5, 0.0),
            (0.5, 0.5, 0.7, 0.3),
            (0.5, 0.8, 0.9, -1.0),
            (0.5, 0.2, 0.4, 2.0),
            (0.3, 0.3, 0.1, 0.0),
        ] {
            let tilt = 0.6_f64;
            let toward = e1 * tilt.cos() + e2 * tilt.sin();
            let o1 = Point3::new(0.2, -0.1, 0.3);
            let o2 = o1 + toward * d + axis * slide;
            let (c1, c2) = (wall(o1, axis, r1), wall(o2, -axis, r2));
            // The meeting points of the two circles in the section
            // through o1: x along `toward`, ±h across it.
            let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
            let h = (r1 * r1 - x * x).sqrt();
            let across = axis.cross(toward);
            let rulings = [o1 + toward * x + across * h, o1 + toward * x - across * h];
            for (label, own, partner) in [("ab", &c1, &c2), ("ba", &c2, &c1)] {
                let (p, n) = parallel_radical_plane(own, partner, &at(o1), band())
                    .unwrap()
                    .expect("offset axes name a plane");
                let n = n.get();
                let tag = format!("r {r1}/{r2} d {d} {label}");
                assert!(n.dot(axis).abs() < 1e-12, "{tag}: parallel to the axes");
                for q in rulings {
                    for c in [&c1, &c2] {
                        let geom::Surface::Cylinder {
                            origin,
                            axis: a,
                            radius,
                            ..
                        } = c
                        else {
                            unreachable!()
                        };
                        let off = q - *origin;
                        let r = (off - *a * off.dot(*a)).norm();
                        assert!(
                            (r - radius).abs() < 1e-12,
                            "{tag}: the ruling is on both walls"
                        );
                    }
                    for k in [-3.0, 0.0, 5.0] {
                        let on = q + axis * k;
                        assert!(
                            (on - p).dot(n).abs() < 1e-12,
                            "{tag}: a ruling lies in the plane"
                        );
                    }
                }
            }
        }
    }

    /// **The rulings lane refuses any chord that is not a ruling, naming
    /// the wall it came from.** A tangent chord on either side is the
    /// walls touching, refused naming that side's wall; a conic is a
    /// desync; two straight chords pass.
    #[test]
    fn a_chord_that_is_not_a_ruling_refuses_naming_its_wall() {
        use crate::chord_join::SegmentCurve;
        use slotmap::KeyData;
        let he = |n: u64| HalfEdgeKey::from(KeyData::from_ffi(n));
        let face = |n: u64| FaceKey::from(KeyData::from_ffi(n));
        let (fa, fb) = (face(3), face(7));
        let line = geom::Curve3::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, 1.0),
        };
        let circle = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (s1, s2) = Default::default();
        let witness = Point3::new(0.0, 0.0, 0.5);
        let spec = |description, carrier: &geom::Curve3<f64>| geom_brep::EdgeCurveSpec {
            description,
            carrier: carrier.clone(),
            param_start: 0.0,
            param_end: 1.0,
        };
        let curve = |spec| SegmentCurve::of((he(1), he(2)), spec);
        let straight = curve(None);
        let ask = |a: &SegmentCurve<f64>, b: &SegmentCurve<f64>| {
            rulings_are_straight([(a, Operand::A, fa), (b, Operand::B, fb)])
        };
        assert!(ask(&straight, &straight).is_ok(), "two rulings pass");
        // The tangent chord is the ruling under either description the
        // must-carry rule can give it.
        let intrinsic = curve(Some(spec(
            geom_brep::EdgeDescriptionSpec::TangentIntersection {
                pair: geom_brep::SurfacePair::new(s1, s2),
                witness,
            },
            &line,
        )));
        let conventional = curve(Some(spec(geom_brep::EdgeDescriptionSpec::chart(s2), &line)));
        for (label, got, operand, wall) in [
            ("A touches", ask(&intrinsic, &straight), Operand::A, fa),
            ("B touches", ask(&straight, &intrinsic), Operand::B, fb),
            (
                "A touches, conventional",
                ask(&conventional, &straight),
                Operand::A,
                fa,
            ),
            (
                "B touches, conventional",
                ask(&straight, &conventional),
                Operand::B,
                fb,
            ),
        ] {
            assert!(
                matches!(
                    got,
                    Err(BooleanError::CurvedBooleanUnsupported {
                        operand: o,
                        face: f,
                        kind: geom::SurfaceKind::Cylinder,
                    }) if o == operand && f == wall
                ),
                "{label}: {got:?}"
            );
        }
        let conic = curve(Some(spec(
            geom_brep::EdgeDescriptionSpec::Intersection {
                pair: geom_brep::SurfacePair::new(s1, s2),
                selector: geom_brep::CurveSelector::Witness(witness),
            },
            &circle,
        )));
        assert!(
            matches!(
                ask(&straight, &conic),
                Err(BooleanError::JoinDesync {
                    what: "a parallel cylinder pair's radical plane cut a wall in a conic"
                })
            ),
            "a conic is a desync"
        );
    }

    /// **Coaxial walls name no plane; an offset in the band escalates
    /// under its own name.**
    #[test]
    fn a_coaxial_pair_has_no_plane_and_an_offset_in_band_escalates() {
        let eps = Tol::witness().get().eps;
        let z = Vec3::new(0.0, 0.0, 1.0);
        let on_x = |x: f64| Point3::new(x, 0.0, 0.0);
        let reach = at(Point3::origin());
        let pair = |offset| (wall(on_x(0.0), z, 0.5), wall(on_x(offset), z, 0.3));
        for (label, offset) in [("coaxial", 0.0), ("in the zero band", eps / 4.0)] {
            let (own, partner) = pair(offset);
            let got = parallel_radical_plane(&own, &partner, &reach, band());
            assert!(
                matches!(got, Ok(None)),
                "{label}: {:?}",
                got.map(|p| p.is_some())
            );
        }
        let (own, partner) = pair(4.0 * eps);
        let got = parallel_radical_plane(&own, &partner, &reach, band());
        match got {
            Err(BooleanError::Escalated {
                decision: BooleanDecision::Coincidence(Coincide::Section, DeclarationRead::Moot),
                diag,
            }) => assert_eq!(diag.predicate, Some(BOOL_JOIN_CC_AXIS_OFFSET)),
            other => panic!("in band: {:?}", other.map(|p| p.is_some())),
        }
    }

    /// **The plane is read at the reach, wherever an origin is stored.**
    /// A radius-0.5 wall about `z` and a radius-0.3 one whose axis
    /// passes `(d, 0, 0)` tilted half the zero band toward `y`, read at
    /// the origin; the partner's origin is stored up to 1000 m along its
    /// axis. Offset (`d = 0.6`), the plane holds the walls' two meeting
    /// points in `z = 0` at every stored origin; read at a stored origin
    /// 1000 m up, the offset carried `1000·θ` across it and the plane
    /// turned off those points by some hundreds of `zero`. Coaxial (`d = 0`),
    /// the walls name no plane at every stored origin; read 1000 m up
    /// the offset was `500·zero`, definite, and named one.
    #[test]
    fn the_plane_is_read_at_the_reach_wherever_an_origin_is_stored() {
        let zero = band().zero();
        let theta = 0.5 * zero;
        let tilted = Vec3::new(0.0, theta.sin(), theta.cos());
        let z = Vec3::new(0.0, 0.0, 1.0);
        let reach = at(Point3::origin());
        let (r1, r2, d): (f64, f64, f64) = (0.5, 0.3, 0.6);
        let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
        let h = (r1 * r1 - x * x).sqrt();
        let meets = [Point3::new(x, h, 0.0), Point3::new(x, -h, 0.0)];
        for along in [0.0, 10.0, -10.0, 1000.0, -1000.0] {
            let own = wall(Point3::origin(), z, r1);
            let partner = wall(Point3::new(d, 0.0, 0.0) + tilted * along, tilted, r2);
            for (label, a, b) in [("ab", &own, &partner), ("ba", &partner, &own)] {
                let (p, n) = parallel_radical_plane(a, b, &reach, band())
                    .unwrap()
                    .expect("offset axes name a plane");
                for q in meets {
                    let off = (q - p).dot(n.get()).abs();
                    assert!(
                        off < zero,
                        "stored {along} m along ({label}): the plane holds the walls' meeting \
                         point {q:?} at the reach, {off:e} off"
                    );
                }
            }
            let partner = wall(Point3::origin() + tilted * along, tilted, r2);
            for (label, a, b) in [("ab", &own, &partner), ("ba", &partner, &own)] {
                let got = parallel_radical_plane(a, b, &reach, band());
                assert!(
                    matches!(got, Ok(None)),
                    "stored {along} m along ({label}): coaxial at the reach, no plane: {:?}",
                    got.map(|p| p.is_some())
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **A null half's sense is one side set's.** A start that is only
    /// a below end is up, only an above end down; one in both sets, or
    /// in neither, names no sense and refuses.
    #[test]
    fn a_vertex_in_both_side_sets_names_no_sense() {
        let mut keys = slotmap::SlotMap::<VertexKey, ()>::with_key();
        let [below, above, both, neither] = [(); 4].map(|()| keys.insert(()));
        let mut sides = Sides {
            in_set: SecondaryMap::new(),
            out_set: SecondaryMap::new(),
        };
        for v in [below, both] {
            sides.in_set.insert(v, ());
        }
        for v in [above, both] {
            sides.out_set.insert(v, ());
        }
        assert!(matches!(sides.starts_up(below), Ok(true)), "below end");
        assert!(matches!(sides.starts_up(above), Ok(false)), "above end");
        assert!(
            matches!(sides.starts_up(both), Err(BooleanError::JoinDesync { what }) if what.contains("both")),
            "both ends"
        );
        assert!(
            matches!(sides.starts_up(neither), Err(BooleanError::JoinDesync { what }) if what.contains("neither")),
            "neither end"
        );
    }
}

#[cfg(test)]
mod kind_arm_rows {
    use super::{KindArm, kind_arm};
    use geom::SurfaceKind::{self, Approx, Cone, Cylinder, Nurbs, Plane, Sphere, Torus};

    /// **The edge-plane lane is the fallthrough's alone.** Every face
    /// pair with an arm keeps it, so a germ along an edge in such a pair
    /// rides the pair's own datum; only the pairs with none reach the
    /// edge-plane lane.
    #[test]
    fn the_edge_plane_lane_takes_only_the_pairs_no_arm_takes() {
        let kinds: [SurfaceKind; 7] = [Plane, Cylinder, Cone, Sphere, Torus, Nurbs, Approx];
        let armed = [
            ((Plane, Plane), KindArm::Planar),
            ((Plane, Sphere), KindArm::PlaneWall),
            ((Plane, Cylinder), KindArm::PlaneWall),
            ((Plane, Cone), KindArm::PlaneWall),
            ((Plane, Torus), KindArm::PlaneWall),
            ((Sphere, Plane), KindArm::WallPlane),
            ((Cylinder, Plane), KindArm::WallPlane),
            ((Cone, Plane), KindArm::WallPlane),
            ((Torus, Plane), KindArm::WallPlane),
            ((Sphere, Sphere), KindArm::Spheres),
            ((Cylinder, Cylinder), KindArm::Cylinders),
        ];
        for a in kinds {
            for b in kinds {
                let want = armed
                    .iter()
                    .find(|(pair, _)| *pair == (a, b))
                    .map_or(KindArm::None, |&(_, arm)| arm);
                assert_eq!(kind_arm(a, b), want, "{a:?} × {b:?}");
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod first_pair_rows {
    use super::*;

    /// A straight germ's partner at `chord`, met `at`th.
    fn met(at: usize, germ: Slot, partner: Slot, chord: f64) -> Met<f64> {
        let reach = Reach {
            arm: GermArm::Ahead,
            chord,
            turn: None,
            along: None,
        };
        (at, reach, (partner, germ))
    }

    /// Two mutual pairs whose chords differ by five zeros, the first met
    /// first.
    fn two_pairs(zero: f64) -> Vec<Met<f64>> {
        let (a, b, c, d) = ((0, 0), (1, 0), (2, 0), (3, 0));
        vec![
            met(1, a, b, 1.0 + 5.0 * zero),
            met(2, b, a, 1.0 + 5.0 * zero),
            met(3, c, d, 1.0),
            met(4, d, c, 1.0),
        ]
    }

    fn by_germ(mets: Vec<Met<f64>>) -> BTreeMap<Slot, Met<f64>> {
        mets.into_iter().map(|m| (m.2.1, m)).collect()
    }

    fn escalates_nearest(got: &Result<Option<(Slot, Slot)>, BooleanError>) -> bool {
        matches!(got, Err(BooleanError::Escalated { diag, .. })
            if diag.predicate == Some("bool_join_nearest"))
    }

    /// **An in-band order between two pairs is free exactly where its
    /// premises hold** ([`first_pair`]). Two disjoint mutual straight
    /// pairs whose chords differ by five zeros, at `K = 10`: the pair met
    /// first keeps the round. Each premise broken, the same reading
    /// escalates on `bool_join_nearest`: a fifth germ whose nearest lands
    /// in a pair, a band of `K = 2`, and conic germs (a unit circle's).
    /// A poisoned chord escalates even where the order is free.
    #[test]
    fn an_in_band_order_is_free_only_where_the_matching_cannot_move() {
        let zero = 1e-6;
        let band = Band::new(zero, 10.0 * zero).unwrap();
        let got = first_pair(by_germ(two_pairs(zero)), band);
        assert!(
            matches!(got, Ok(Some((partner, germ))) if partner == (1, 0) && germ == (0, 0)),
            "premises hold: the pair met first keeps the round, got {got:?}"
        );

        let contested = two_pairs(zero)
            .into_iter()
            .chain([met(5, (4, 0), (3, 0), 2.0)])
            .collect();
        let got = first_pair(by_germ(contested), band);
        assert!(escalates_nearest(&got), "a non-mutual nearest: {got:?}");

        let k2 = Band::new(zero, 2.0 * zero).unwrap();
        let tied = vec![
            met(1, (0, 0), (1, 0), 1.0 + 1.5 * zero),
            met(2, (1, 0), (0, 0), 1.0 + 1.5 * zero),
            met(3, (2, 0), (3, 0), 1.0),
            met(4, (3, 0), (2, 0), 1.0),
        ];
        let got = first_pair(by_germ(tied), k2);
        assert!(escalates_nearest(&got), "K = 2: {got:?}");

        let frame = (Point3::origin(), Vec3::unit_z());
        let germ = (Point3::new(1.0, 0.0, 0.0), Vec3::unit_y());
        let turn = |t: f64| {
            Turn::facing(
                frame,
                germ,
                (
                    Point3::new(t.cos(), t.sin(), 0.0),
                    Vec3::new(t.sin(), -t.cos(), 0.0),
                ),
                band,
            )
            .unwrap()
        };
        let conic = two_pairs(zero)
            .into_iter()
            .map(|(at, reach, m)| {
                (
                    at,
                    Reach {
                        turn: turn(1.0),
                        ..reach
                    },
                    m,
                )
            })
            .collect();
        assert!(turn(1.0).is_some(), "the circle's germs turn");
        let got = first_pair(by_germ(conic), band);
        assert!(escalates_nearest(&got), "conic germs: {got:?}");

        let mut poisoned = two_pairs(zero);
        poisoned[2].1.chord = f64::NAN;
        let got = first_pair(by_germ(poisoned), band);
        assert!(
            matches!(&got, Err(BooleanError::Escalated { diag, .. }) if diag.margin.is_invalid()),
            "a poisoned chord escalates: {got:?}"
        );
    }

    /// The rounds [`find_match`] runs, over straight germs at `points`
    /// partnered along `edges` (chord: the distance), each round's pair
    /// chosen by `pick` from the germs' nearest partners ([`keep_own`]);
    /// the matching as sorted pairs.
    fn rounds(
        points: &[(f64, f64)],
        edges: &[(usize, usize)],
        band: Band,
        pick: impl Fn(BTreeMap<Slot, Met<f64>>) -> Result<Option<(Slot, Slot)>, BooleanError>,
    ) -> Result<Vec<(usize, usize)>, BooleanError> {
        let chord = |i: usize, j: usize| {
            let (p, q) = (points[i], points[j]);
            ((p.0 - q.0).powi(2) + (p.1 - q.1).powi(2)).sqrt()
        };
        let mut used = vec![false; points.len()];
        let mut matching = Vec::new();
        loop {
            let mut own = BTreeMap::new();
            let mut at = 0;
            for g in 0..points.len() {
                for &(i, j) in edges {
                    let h = if i == g {
                        j
                    } else if j == g {
                        i
                    } else {
                        continue;
                    };
                    if used[g] || used[h] {
                        continue;
                    }
                    at += 1;
                    keep_own(&mut own, met(at, (g, 0), (h, 0), chord(g, h)), band)?;
                }
            }
            let Some(((p, _), (g, _))) = pick(own)? else {
                matching.sort_unstable();
                return Ok(matching);
            };
            used[p] = true;
            used[g] = true;
            matching.push((p.min(g), p.max(g)));
        }
    }

    /// **At `K = 1.1` the order among mutual straight pairs moves the
    /// matching, so [`first_pair`] reads it** ([`first_pair`]'s third
    /// premise). Eight germs whose first round is all mutual: with the
    /// in-band order free, each round's pairs folded in scan order, and
    /// in scan order started at its third pair, build two different
    /// matchings (a germ's shortened scan crosses two chained ties);
    /// [`first_pair`] escalates. The witness is the PR 4769 review's.
    #[test]
    fn at_k_at_most_2_first_pair_reads_the_order_among_mutual_pairs() {
        let points = [
            (1.6374561303759052, 0.5775992446059082),
            (0.40233212126666984, 0.3418720853489539),
            (2.572078021535734, 2.269339501751353),
            (2.492829820460523, 0.6725138844041666),
            (1.1969478341656838, 1.0524143195635753),
            (0.6507604875594485, 2.121686389408788),
            (0.6512525825421824, 0.9950480970646106),
            (0.40292815436843227, 2.211559158237296),
        ];
        let edges = [
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 7),
            (1, 6),
            (1, 7),
            (2, 3),
            (2, 4),
            (2, 5),
            (2, 6),
            (2, 7),
            (3, 4),
            (3, 5),
            (4, 5),
            (4, 6),
            (4, 7),
            (5, 7),
        ];
        let band = Band::new(1.0, 1.1).unwrap();
        let escalated_mutual = std::cell::Cell::new(None);
        let got = rounds(&points, &edges, band, |own| {
            let mutual = own.iter().all(|(germ, &(_, reach, (partner, _)))| {
                reach.turn.is_none()
                    && own
                        .get(&partner)
                        .is_some_and(|&(_, _, (back, _))| back == *germ)
            });
            let pick = first_pair(own, band);
            if pick.is_err() {
                escalated_mutual.set(Some(mutual));
            }
            pick
        });
        assert!(
            matches!(&got, Err(BooleanError::Escalated { diag, .. })
                if diag.predicate == Some("bool_join_nearest")),
            "first_pair reads the order at K = 1.1: {got:?}"
        );
        assert_eq!(
            escalated_mutual.get(),
            Some(true),
            "the escalating round is all mutual and straight, so the K gate is what reads it"
        );
    }
}

/// **The Boolean's own decision for a join's reading of a section end**
/// ([`of_join`]): a vertex on the plane at the band's edge is an edge of
/// the result on its faces; the plane read square, and a carrier that
/// cannot resolve its end, stay the join's refusal.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod of_join_rows {
    use super::*;
    use crate::chord_join::SectionEndReading;
    use geom_core::{Indeterminate, MarginDiag};

    fn diag() -> Indeterminate {
        Indeterminate {
            margin: MarginDiag::value(1.2e-9),
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("carrier_endpoint_end"),
            terminal_sliver: false,
        }
    }

    #[test]
    fn only_an_end_on_the_plane_is_the_booleans_edge_on_faces() {
        let face = FaceKey::default();
        let end = |reading| SplitJoinError::SectionEndEscalated {
            face,
            reading,
            diag: diag(),
        };
        assert!(matches!(
            of_join(end(SectionEndReading::OnPlane)),
            BooleanError::Escalated {
                decision: BooleanDecision::EdgeOnFaces,
                diag: d,
            } if d == diag()
        ));
        let joins = |refusal: SplitJoinError| {
            let kept = format!("{refusal:?}");
            match of_join(refusal) {
                BooleanError::Join(j) => format!("{:?}", j.error()) == kept,
                _ => false,
            }
        };
        assert!(joins(end(SectionEndReading::SquareToAxis)));
        assert!(joins(SplitJoinError::SectionUnresolved {
            face,
            miss: crate::chord_join::SectionPinMiss::Undecided {
                check: geom_brep::CertCheck::EndpointEnd,
                diag: diag(),
                resolution: MarginDiag::value(2e-9),
            },
        }));
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod face_surface_tests {
    /// **The join walks to a face's surface in one place**
    /// ([`super::face_surface`]): every read of a surface in this file's
    /// code, a face's or a key's, is that function's one.
    #[test]
    fn the_join_walks_to_a_faces_surface_in_one_place() {
        let code = test_utils::source::code_only(include_str!("join.rs"));
        let reads: Vec<usize> = ["get_surface(", "face_surface_linked(", "carrier_surface("]
            .iter()
            .flat_map(|needle| code.match_indices(needle).map(|(at, _)| at))
            .collect();
        let homes: Vec<usize> = code
            .match_indices("fn face_surface<")
            .map(|(at, _)| at)
            .collect();
        assert_eq!(homes.len(), 1, "one home");
        let home = homes[0];
        let open = home + code[home..].find('{').expect("the home's body");
        let body_end = test_utils::source::balanced_end(&code, open).expect("a closed body");
        let elsewhere: Vec<usize> = reads
            .iter()
            .filter(|&&at| !(home..body_end).contains(&at))
            .map(|&at| test_utils::source::line(&code, at))
            .collect();
        assert!(
            reads.len() == 1 && elsewhere.is_empty(),
            "a surface read outside `face_surface`, at lines {elsewhere:?} ({} reads in all)",
            reads.len()
        );
    }
}

#[cfg(test)]
#[path = "join_carrier_site_rows.rs"]
mod join_carrier_site_rows;

#[cfg(test)]
#[path = "join_walk_order_rows.rs"]
mod join_walk_order_rows;
