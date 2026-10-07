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
//! # The seam-orientation discipline (PR 5.5 — the derived form of
//! # the book's ssortnulledges / he1↔he2 crossover)
//!
//! Derived from the ratified conventions (outward normals, loops
//! CCW-from-outside: a half-edge with tangent `t` on a face with
//! normal `n` has interior to its LEFT, `n×t` pointing in), each step
//! mirror-checked in the M3-LOG PR 5.5 record:
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
//!    (`insert::strut_order`), read as distances at the sector's arm;
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
//!    [`choose_roles`]' derived rule via [`ring_run_ccw`] (issue #93;
//!    equivalent to PR 5.5's "cycle opposite the residual-material
//!    side" wherever that probe's outer-loop anchor was sound, and
//!    decided intrinsically so multi-polygon faces cannot cross it).
//! 4. **Consistency theorem.** With (2) as data, (3)'s ring rule per
//!    solid, and matching that consumes the SAME germ in both solids
//!    ([`find_match`]'s slot lock), every completed polygon pair has
//!    A's IN loop antiparallel to B's IN loop, and the zip assertion
//!    ([`BooleanError::SeamOrientation`]) is a theorem with a runtime
//!    witness, not a hope.
//!
//! # The fixpoint sweep and lockstep discipline
//!
//! All pair records register up front, and [`section_segments`]
//! matches them: it repeatedly takes the nearest valid match in
//! deterministic scan order until quiescent (each match consumes its
//! germs, so the next round ranks only the germs still free). The criterion
//! reads only record data — loci, senses, site points, section
//! frames — none of which the surgery changes, so the segments are
//! decided before any chord is minted, and the declared-REST zip
//! ([`super::rest`]) reads the same list. The sweep then joins them in
//! that order. Joins, retirements, and completions must occur in
//! BOTH solids together; any divergence is the typed
//! [`BooleanError::JoinDesync`] refusal, never a silent mis-join.
//! There is no geometric sort and no section-area certification here:
//! boolean intersection polygons are in general non-planar (§15.7);
//! degenerate results are netted at the component stage instead.

use geom_core::{Band, Decide, Margin, Point3, Sign, UnitVec3, Vec3};
use slotmap::SecondaryMap;

use super::shell_witness::{Reading, complex_side};
use super::{
    BooleanDecision, BooleanError, BooleanReduction, Coincide, DeclarationRead, HalfGerm, Operand,
    SelfCheck, SideCode,
};
use crate::body::Body;
use crate::chord_join::{
    ChordJoiner, CutOutcome, Datum, JoinLane, JoinPlan, Leave, SegmentCurve, SegmentEdge,
    SplitJoinError,
};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::face_normal::face_outward_normal;
use crate::loop_winding::{RunClosing, RunMissesEnd};
use crate::null::NullFacePair;
use crate::validate::decide;
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

/// The chord lane a germ face pair joins by, with the section datum
/// each side's chords lie in.
#[derive(Clone, Copy)]
enum GermLane<T: geom_core::Real> {
    /// Plane × plane: the chord runs along the two planes' common line.
    Planar,
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
}

impl<T: geom_core::Real> GermLane<T> {
    /// How each side's ring-lane island closes, A's then B's: a planar
    /// germ face on its own plane, a wall along the section its chords
    /// lie in.
    fn ring_closures(self) -> (RingClosure<T>, RingClosure<T>) {
        match self {
            Self::Planar => (RingClosure::Planar, RingClosure::Planar),
            Self::PlaneWall(plane) => (RingClosure::Planar, RingClosure::Wall(plane)),
            Self::WallPlane(plane) => (RingClosure::Wall(plane), RingClosure::Planar),
            Self::Radical(plane) | Self::Rulings(plane) => {
                (RingClosure::Wall(plane), RingClosure::Wall(plane))
            }
        }
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
    /// plane, on the face's chart; a sphere's or a cone's by the
    /// segment's curve.
    Wall((Point3<T>, UnitVec3<T>)),
    /// A segment along an edge of both solids (this solid the named
    /// operand) reads no section: a planar face's island closes on its
    /// own plane, and a curved face has nothing to close along, which
    /// is a join arm not yet built.
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
    /// A sphere or a cone face: closed by the segment's curve, an arc of
    /// a plane section or a cone's ruling
    /// ([`crate::chord_join::path_island_winding`]).
    Quadric(&'a SegmentCurve<T>),
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
    aux: std::collections::BTreeMap<AuxDatum, crate::geometry::SurfaceKey>,
    /// `(edge, chord)` for every chord minted on a segment along this
    /// solid's `edge` ([`Self::join`]).
    along: Vec<(EdgeKey, EdgeKey)>,
}

/// What an aux surface in [`SolidJoin::aux`] is a copy of, which is
/// what makes two chords' reads of one entry the same datum.
///
/// The two shapes are kept apart because they depend on different
/// things: a partner copy depends on the partner face's surface alone,
/// a radical plane on BOTH spheres. Keying the radical plane by the
/// partner face alone hands a second sphere of THIS body the first
/// one's plane — a chord described against a plane it does not lie in.
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
            .map_err(BooleanError::Join)
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
        let curve = self
            .joiner
            .segment_curve(body, plan, lane, leave)
            .map_err(BooleanError::Join)?;
        curve.ok_or(BooleanError::Join(SplitJoinError::SectionInvariant {
            face: plan.face(),
            what: "both chords of a matched segment are its own edge (a loop holding both \
                   halves of that edge)",
        }))
    }

    /// The wall-side curve against the germ plane through `origin` with
    /// chart normal `normal`, through [`JoinLane::Split`], its aux plane
    /// read from and minted into [`Self::aux`] under `datum`.
    fn split_curve<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        plan: &JoinPlan,
        (origin, normal): (Point3<T>, UnitVec3<T>),
        datum: AuxDatum,
        leave: Leave<T>,
    ) -> Result<SegmentCurve<T>, BooleanError> {
        let mut ctx = crate::chord_join::SectionCtx {
            origin,
            normal,
            plane_key: self.aux.get(&datum).copied(),
        };
        let curve = self.curve(body, plan, JoinLane::Split(&mut ctx), leave)?;
        if let Some(k) = ctx.plane_key {
            self.aux.insert(datum, k);
        }
        Ok(curve)
    }

    /// The planar-side curve against the partner `wall`, through
    /// [`JoinLane::BoolPlanar`], the wall copy keyed by `partner_face`.
    fn bool_planar_curve<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        plan: &JoinPlan,
        wall: geom::Surface<T>,
        partner_face: FaceKey,
        leave: Leave<T>,
    ) -> Result<SegmentCurve<T>, BooleanError> {
        let datum = AuxDatum::Partner(partner_face);
        let mut partner = self.aux.get(&datum).copied();
        let lane = JoinLane::BoolPlanar {
            wall,
            partner_key: &mut partner,
        };
        let curve = self.curve(body, plan, lane, leave)?;
        if let Some(k) = partner {
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
        let minted = self
            .joiner
            .join(body, plan, curve, tol)
            .map_err(BooleanError::Join)?;
        if let Some(edge) = segment {
            self.along
                .extend(minted.into_iter().map(|chord| (edge, chord)));
        }
        Ok(())
    }
}

impl SolidJoin {
    fn new<T: Decide>(red: &BooleanReduction<T>, operand: Operand, band: Band) -> Self {
        Self {
            joiner: ChordJoiner::new(band),
            sides: Sides::new(red, operand),
            aux: std::collections::BTreeMap::new(),
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
    /// copy resolves here).
    fn is_up<T: Decide>(&self, body: &Body<T>, he: HalfEdgeKey) -> Result<bool, BooleanError> {
        let desync = |what| BooleanError::JoinDesync { what };
        let start = body
            .get_half_edge(he)
            .ok_or(desync("half no longer resolves"))?
            .start;
        if self.in_set.contains_key(start) {
            Ok(true)
        } else if self.out_set.contains_key(start) {
            Ok(false)
        } else {
            Err(desync("null half starts at a vertex of neither side set"))
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

/// One section segment as the join's matching decides it: the one
/// enumeration of "which segments exist and what each one is", read by
/// the join's surgery ([`bool_connect`]) and the declared-REST zip
/// ([`super::rest`]) alike.
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

/// **The section segments** (module docs): [`find_match`] to
/// quiescence over the pair records, each match consuming its two germ
/// slots in both solids. Germ slots no match consumed are the join's
/// loose ends, so every germ is consumed exactly when there are as many
/// segments as records. Reads the records and the annotated clones'
/// site points; the surgery changes neither.
pub(super) fn section_segments<T: Decide>(
    red: &BooleanReduction<T>,
    band: Band,
) -> Result<Vec<SectionSegment<T>>, BooleanError> {
    let (sa, sb) = (Sides::new(red, Operand::A), Sides::new(red, Operand::B));
    let mut open = open_records(red)?;
    let mut segments = Vec::new();
    while let Some((entry, cand)) = find_match(&open, red, &sa, &sb, band)? {
        let germ = open[entry.0].a[entry.1].0;
        for (r, slot) in [entry, cand] {
            open[r].a[slot].1 = true;
            open[r].b[slot].1 = true;
        }
        let lane = match (germ.a_locus, germ.b_locus) {
            (super::Locus::OnEdge(_), super::Locus::OnEdge(_)) => SegmentLane::AlongEdge,
            _ => SegmentLane::Section,
        };
        segments.push(SectionSegment {
            ends: [entry, cand],
            germ,
            lane,
        });
    }
    Ok(segments)
}

/// [`section_segments`] read as sites: the pair-record count and each
/// segment's two germ sites on the A clone.
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
    let segments = section_segments(red, band)?
        .iter()
        .map(|s| Ok([site(s.ends[0])?, site(s.ends[1])?]))
        .collect::<Result<_, BooleanError>>()?;
    Ok((open.len(), segments))
}

/// `bool_connect`'s product: the completed pairs plus the per-operand
/// chord-mef fragment logs (naming emission, M4 PR 3 — `(new face,
/// divided-from face)` at call-time CLONE keys, A rows in the A-clone
/// arena, B rows in the B-clone arena pre-graft).
pub(super) struct Connected {
    pub completed: Vec<CompletedPolygonPair>,
    pub a_fragments: Vec<(FaceKey, FaceKey)>,
    pub b_fragments: Vec<(FaceKey, FaceKey)>,
    /// Each operand's chords along its own edges, `(edge, chord)` in
    /// that operand's clone keys, A's then B's: every chord a segment
    /// whose locus on that operand is an edge minted runs along that
    /// edge between its two ends, so it holds the edge's interior
    /// where the op drops the edge ([`SolidJoin::join`]).
    pub along: [Vec<(EdgeKey, EdgeKey)>; 2],
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
    let segments = section_segments(red, band)?;
    let mut sa = SolidJoin::new(red, Operand::A, band);
    let mut sb = SolidJoin::new(red, Operand::B, band);
    let mut completed: Vec<UnresolvedPair> = Vec::new();
    let mut open = open_records(red)?;

    for seg in &segments {
        let [(entry, entry_slot), (cand, cand_slot)] = seg.ends;
        let (ea, ra) = (open[entry].a[entry_slot].0.he, open[cand].a[cand_slot].0.he);
        let (eb, rb) = (open[entry].b[entry_slot].0.he, open[cand].b[cand_slot].0.he);
        // Still-loose halves (choose_roles' separation constraint —
        // a role order must not wall a pending half off from its match
        // partner). The chosen halves themselves are being consumed.
        let (mut a_loose, mut b_loose) = loose_partners(&open, red, &sa.sides, &sb.sides, band)?;
        a_loose.remove(ea);
        a_loose.remove(ra);
        b_loose.remove(eb);
        b_loose.remove(rb);
        // Curved germ pairs (M5 PR 9): each solid's chord lane comes
        // from the germ FACE PAIR — plane×plane takes the straight-chord
        // lane with the partner's plane as its section; plane×cylinder
        // and plane×sphere (M5 S13) mint the C5 section conic on both
        // sides (the wall side with the germ plane as context, the
        // planar side against the partner wall), each taking the arc the
        // matched germs' directions name, so both solids take the SAME
        // geometric arc; a sphere pair rides the wall-side lane on both
        // sides against its radical plane, and so does a parallel
        // cylinder pair, whose chords there are rulings; any other pair
        // refuses typed citing its C5 routing (per-arm, C12.1).
        let germ = seg.germ;
        let surf_of =
            |body: &Body<T>,
             f: FaceKey|
             -> Result<(crate::geometry::SurfaceKey, geom::Surface<T>), BooleanError> {
                body.get_face(f)
                    .and_then(|fd| Some((fd.surface, body.get_surface(fd.surface)?.clone())))
                    .ok_or(desync("germ face surface no longer resolves"))
            };
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
        let germ_reach = |body: &Body<T>| -> Result<geom_brep::ExtentBall<T>, BooleanError> {
            let site = |he| {
                body.half_edge_start_point(he)
                    .map(geom_brep::ExtentBall::point)
                    .ok_or(desync("germ site has no point"))
            };
            geom_brep::ExtentBall::enclosing(&[site(ea)?, site(ra)?])
                .ok_or(desync("a join has no germ sites"))
        };
        let germ_normal = |reach: geom_brep::ExtentBall<T>, origin: Point3<T>, n: Vec3<T>| {
            UnitVec3::levered(n, BOOL_GERM_PLANE_NORMAL, band, reach.lever_from(origin)).map_err(
                |_| desync("a germ plane's normal has no decided length (a broken plane carrier)"),
            )
        };
        let (ka, ga) = surf_of(&red.a, germ.a_face)?;
        let (kb, gb) = surf_of(&red.b, germ.b_face)?;
        // Each solid's adjacency skip asks whether the edge between its
        // two halves is the one the segment's locus on that solid names.
        let (seg_a, seg_b) = (germ.a_locus.edge(), germ.b_locus.edge());
        use geom::Surface as Sf;
        // No wired join arm for this germ pair (cyl×sphere's rung-3
        // fitted chords, a coaxial cylinder pair, plane×NURBS behind PR
        // 7b): typed, citing the kind whose join arm is missing.
        let no_arm = || {
            let (operand, face, s) = if matches!(ga, Sf::Plane { .. }) {
                (Operand::B, germ.b_face, &gb)
            } else {
                (Operand::A, germ.a_face, &ga)
            };
            BooleanError::CurvedBooleanUnsupported {
                operand,
                face,
                kind: s.kind(),
            }
        };
        // A segment along an edge of both solids is that edge in both:
        // each solid's chord copies its own edge and no section is read
        // (the germ's two faces may share one carrier), so it takes no
        // kind lane.
        let lane = if seg.lane == SegmentLane::AlongEdge {
            None
        } else {
            Some(match (&ga, &gb) {
                (Sf::Plane { .. }, Sf::Plane { .. }) => GermLane::Planar,
                (Sf::Plane { origin, normal, .. }, Sf::Sphere { .. })
                | (Sf::Plane { origin, normal, .. }, Sf::Cylinder { .. }) => GermLane::PlaneWall((
                    *origin,
                    germ_normal(germ_reach(&red.a)?, *origin, *normal)?,
                )),
                (Sf::Sphere { .. }, Sf::Plane { origin, normal, .. })
                | (Sf::Cylinder { .. }, Sf::Plane { origin, normal, .. }) => GermLane::WallPlane((
                    *origin,
                    germ_normal(germ_reach(&red.a)?, *origin, *normal)?,
                )),
                // **The sphere pair rides its RADICAL PLANE.** Two spheres
                // meet in a circle lying in the one plane both residuals
                // agree on, so on each side the section is that sphere cut
                // by that plane: the wall-side chord lane of the plane×sphere
                // pair above, run on BOTH sides against the same plane. The
                // plane is computed from the pair's own C5 Circle, once per
                // germ, so the two sides' chords are sections of one datum;
                // each body's aux copy of it is keyed by the two spheres it
                // depends on ([`AuxDatum::Radical`]).
                (Sf::Sphere { .. }, Sf::Sphere { .. }) => {
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
                // chord on its own wall. The frame dispatch admitted the
                // pair only with parallel axes; coaxial walls have no
                // such plane and keep the refusal below.
                (Sf::Cylinder { .. }, Sf::Cylinder { .. }) => {
                    match parallel_radical_plane(&ga, &gb, band)? {
                        Some(radical) => GermLane::Rulings(radical),
                        None => return Err(no_arm()),
                    }
                }
                _ => return Err(no_arm()),
            })
        };
        // Role order per solid, derived independently (module docs —
        // the PR 5.5 discipline): cross-solid seam orientation is
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
        sa.joiner
            .place_pending(&mut red.a, (ea, ra))
            .map_err(BooleanError::Join)?;
        sb.joiner
            .place_pending(&mut red.b, (eb, rb))
            .map_err(BooleanError::Join)?;
        let a_lane = choose_roles(&red.a, (ea, ra), &a_loose, seg_a, a_closure, band)?;
        let b_lane = choose_roles(&red.b, (eb, rb), &b_loose, seg_b, b_closure, band)?;
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
            Some(GermLane::PlaneWall(plane)) => (
                sa.bool_planar_curve(&mut red.a, &plan_a, gb.clone(), germ.b_face, leave_a)?,
                sb.split_curve(
                    &mut red.b,
                    &plan_b,
                    plane,
                    AuxDatum::Partner(germ.a_face),
                    leave_b,
                )?,
            ),
            Some(GermLane::WallPlane(plane)) => {
                let curve_a = sa.split_curve(
                    &mut red.a,
                    &plan_a,
                    plane,
                    AuxDatum::Partner(germ.b_face),
                    leave_a,
                )?;
                let curve_b =
                    sb.bool_planar_curve(&mut red.b, &plan_b, ga.clone(), germ.a_face, leave_b)?;
                (curve_a, curve_b)
            }
            Some(GermLane::Radical(radical)) => {
                let datum = |own, partner| AuxDatum::Radical { own, partner };
                (
                    sa.split_curve(&mut red.a, &plan_a, radical, datum(ka, kb), leave_a)?,
                    sb.split_curve(&mut red.b, &plan_b, radical, datum(kb, ka), leave_b)?,
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
                    sa.split_curve(&mut red.a, &plan_a, radical, datum(ka, kb), leave_a)?,
                    sb.split_curve(&mut red.b, &plan_b, radical, datum(kb, ka), leave_b)?,
                );
                rulings_are_straight([
                    (&curves.0, Operand::A, germ.a_face),
                    (&curves.1, Operand::B, germ.b_face),
                ])?;
                curves
            }
        };
        // The ring lane's order, wound with the curve.
        let a_roles = a_lane.resolve(&red.a, (ea, ra), &a_loose, &curve_a, band)?;
        let b_roles = b_lane.resolve(&red.b, (eb, rb), &b_loose, &curve_b, band)?;
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

    // ---- A closed section loop with one site: a record whose two
    // germs name one locus on both operands along a conic can only
    // match itself, which the join does not do (a self-matching record
    // would see the adjacency skip fire both ways and retire a real face
    // as the null face). Refused typed before the loose ends are
    // counted. ----
    let mut single_site = 0;
    for r in &open {
        let [(g0, used0), (g1, used1)] = r.a;
        if used0 || used1 || g0.a_locus != g1.a_locus || g0.b_locus != g1.b_locus {
            continue;
        }
        if germ_section_frame(red, &g0, band)?.is_some() {
            single_site += 1;
        }
    }
    if single_site != 0 {
        return Err(BooleanError::Join(SplitJoinError::SingleSiteSectionLoop {
            count: single_site,
        }));
    }

    // ---- Role resolution, deferred to quiescence: mid-join the
    // region faces are not yet final (an operand face pierced by TWO
    // polygons still spans both seams while the first completes), so
    // probing happens only after every polygon has been cut. ----
    let mut resolved = Vec::with_capacity(completed.len());
    for c in completed {
        let (a_in_loop, a_out_loop) =
            resolve_roles_geometric(&red.a, b_pristine, c.a_face, c.a_outer, c.a_ring, band, tol)?;
        let (b_in_loop, b_out_loop) =
            resolve_roles_geometric(&red.b, a_pristine, c.b_face, c.b_outer, c.b_ring, band, tol)?;
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
        return Err(BooleanError::Join(SplitJoinError::UnpairedLooseEnds {
            count: leftovers,
        }));
    }
    // A pierce ring deferred at a pinch is placed by the join that
    // reaches it; one still pending after every join has run never was.
    sa.joiner.finish(&red.a).map_err(BooleanError::Join)?;
    sb.joiner.finish(&red.b).map_err(BooleanError::Join)?;
    Ok(Connected {
        completed,
        a_fragments: sa.joiner.take_fragments(),
        b_fragments: sb.joiner.take_fragments(),
        along: [sa.along, sb.along],
    })
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
}

/// A conic germ line as its germ turns along it: the section's centre
/// and axis, the germ's rotational sense (±1) and the partner site.
#[derive(Clone, Copy)]
struct Turn<T: geom_core::Real> {
    center: geom_core::Point3<T>,
    axis: Vec3<T>,
    sense: T,
    site: geom_core::Point3<T>,
}

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

/// **Is `cand` nearer than `best` along ONE germ's line?** Both are
/// read from the same germ. The half-turn decides where the two differ.
/// Within one half-turn a conic orders them by the turn between them
/// (`bool_join_arc_travel`, [`turned_past`] from `best`'s site): the
/// angle travelled about a centred conic's axis is monotone along it,
/// where the chord is not — on an ellipse of aspect √2 or more it peaks
/// inside the half-turn. Two sites in one half-turn are less than a
/// half-turn apart, so the side of `best`'s axis plane `cand` lies on
/// is the order. A straight line, and two records at one site, take the
/// chord.
fn nearer_along<T: Decide>(
    cand: Reach<T>,
    best: Reach<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    if cand.arm != best.arm {
        return Ok(cand.arm == GermArm::Ahead);
    }
    if let (Some(c), Some(b)) = (cand.turn, best.turn) {
        let escalate =
            |diag| BooleanError::coincidence(Coincide::Join, DeclarationRead::Moot, diag);
        match decide("bool_join_arc_travel", turned_past(b, c.site), band).map_err(escalate)? {
            Sign::Negative => return Ok(true),
            Sign::Positive => return Ok(false),
            Sign::Zero => {}
        }
    }
    nearer(cand, best, band)
}

/// **Which of two germs' nearest partners [`find_match`] joins first.**
/// Each was already chosen along its own germ line ([`nearer_along`]),
/// so this picks an order among pairs rather than a partner: the
/// half-turn, then the chord (`bool_join_nearest`, metres).
fn nearer<T: Decide>(cand: Reach<T>, best: Reach<T>, band: Band) -> Result<bool, BooleanError> {
    if cand.arm != best.arm {
        return Ok(cand.arm == GermArm::Ahead);
    }
    let escalate = |diag| BooleanError::coincidence(Coincide::Join, DeclarationRead::Moot, diag);
    Ok(decide(
        "bool_join_nearest",
        Margin::of(cand.chord - best.chord),
        band,
    )
    .map_err(escalate)?
        == Sign::Negative)
}

/// **How far `p` has turned past `from`'s site**: the signed distance of
/// `p` from the plane through the conic's axis and that site, positive
/// on the side the germ's sense runs into. Metres — a cross product of
/// two metre vectors projected onto the site's radius, the plane's own
/// normal direction. A site on the axis has no such plane: the margin
/// comes back invalid and escalates.
fn turned_past<T: geom_core::Real>(from: Turn<T>, p: geom_core::Point3<T>) -> Margin<T> {
    let u = from.site - from.center;
    let radial = u - from.axis * from.axis.dot(u);
    Margin::levered_inv(
        from.axis.cross(radial).dot(p - from.center) * from.sense,
        radial.norm(),
    )
}

/// **Which half-turn of the germ's section conic its partner lies in**:
/// [`turned_past`] read from the germ's own site (`bool_join_arc_ahead`).
///
/// `Zero` reads [`GermArm::Ahead`]: an in-band distance puts the partner
/// at the germ's own azimuth (no travel) or at the half-turn, and both
/// sort ahead of every site genuinely `Behind`; within `Ahead`,
/// [`nearer_along`] orders a half-turn site by its turn like any other.
///
/// A straight germ line has no turn to read, so every partner is
/// `Ahead` and the order is the chord alone — the planar pairing.
fn germ_arm<T: Decide>(
    turn: Option<Turn<T>>,
    p_c: geom_core::Point3<T>,
    band: Band,
) -> Result<GermArm, BooleanError> {
    let Some(turn) = turn else {
        return Ok(GermArm::Ahead);
    };
    let escalate = |diag| BooleanError::coincidence(Coincide::Join, DeclarationRead::Moot, diag);
    let from_germ = Turn { site: p_c, ..turn };
    Ok(
        match decide(
            "bool_join_arc_ahead",
            turned_past(from_germ, turn.site),
            band,
        )
        .map_err(escalate)?
        {
            Sign::Negative => GermArm::Behind,
            Sign::Positive | Sign::Zero => GermArm::Ahead,
        },
    )
}

/// What a conic germ with no rotational sense is: a radial germ
/// direction, malformed germ data.
const RADIAL_GERM: &str =
    "a conic germ has no rotational sense (radial germ direction — malformed germ data)";

/// **A conic germ's rotational sense** about its section's axis,
/// `axis·((p − c) × dir)` (metres: |p − c| ~ radius, dir unit), decided
/// under `bool_join_arc_facing`. A zero sense is a radial germ
/// ([`RADIAL_GERM`]), so its in-band twin escalates as the kernel's own
/// self-check rather than as a coincidence.
fn rotational_sense<T: Decide>(
    (center, axis): (geom_core::Point3<T>, Vec3<T>),
    p: geom_core::Point3<T>,
    dir: Vec3<T>,
    band: Band,
) -> Result<Sign, BooleanError> {
    let malformed = |diag| BooleanError::Escalated {
        decision: BooleanDecision::SelfCheck(SelfCheck::ArcFacing),
        diag,
    };
    decide(
        "bool_join_arc_facing",
        Margin::of(axis.dot((p - center).cross(dir))),
        band,
    )
    .map_err(malformed)
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
/// across an intermediate one), then the nearest of those pairs
/// ([`nearer`]). The B side consumes the SAME slots — slot `i`
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
    let mut own: std::collections::BTreeMap<Slot, Met<T>> = std::collections::BTreeMap::new();
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
                    let met = (at, reach, ((entry, es), (cand, cs)));
                    match own.get(&(cand, cs)) {
                        Some(&(_, kept, _)) if !nearer_along(reach, kept, band)? => {}
                        _ => {
                            own.insert((cand, cs), met);
                        }
                    }
                }
            }
        }
    }
    let mut met: Vec<_> = own.into_values().collect();
    met.sort_by_key(|&(at, ..)| at);
    let mut best: Option<(Reach<T>, (Slot, Slot))> = None;
    for (_, reach, m) in met {
        best = keep_nearer(best, (reach, m), |c, b| nearer(c, b, band))?;
    }
    Ok(best.map(|(_, m)| m))
}

/// **Whether two unused germ slots are match partners** — the one
/// criterion [`find_match`] pairs on and [`loose_partners`] counts
/// partners with, so the separation constraint sees the partners the
/// matcher will take: the A germs carry the SAME loci, each `OnEdge`
/// locus at its own site ([`locus_at_site`]); the two halves' senses
/// oppose in A; the sites are distinct (`bool_join_chord`) and FACE
/// each other along the germ line (the locus-aware facing); and the B
/// germs at the same slots mirror them — same loci, opposed senses —
/// or the join has desynced. `Some(reach)` when they are partners — how
/// far the partner is along the germ's line ([`Reach`]). Two slots
/// of one record never are.
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
    let escalate = |diag| BooleanError::coincidence(Coincide::Join, DeclarationRead::Moot, diag);
    match decide("bool_join_chord", Margin::of(dist), band).map_err(escalate)? {
        Sign::Positive => {}
        _ => return Ok(None), // coincident sites: no polygon edge
    }
    // Locus-aware mutual facing (fix pass, dev 4): straight germ lines
    // take the M3 chord test bit-identically; conic germ loci compare
    // rotational senses about the section frame.
    let frame = germ_section_frame(red, &rga, band)?;
    if !germs_face_each_other(frame, &rga, &ega, p_c, p_e, band)? {
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
    let turn = match frame {
        None => None,
        Some(frame) => Some(Turn {
            center: frame.0,
            axis: frame.1,
            sense: match rotational_sense(frame, p_c, rga.dir, band)? {
                Sign::Positive => T::one(),
                Sign::Negative => T::zero() - T::one(),
                Sign::Zero => return Err(desync(RADIAL_GERM)),
            },
            site: p_e,
        }),
    };
    Ok(Some(Reach {
        arm: germ_arm(turn, p_c, band)?,
        chord: dist,
        turn,
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

/// The germ pair's section frame: the conic center and axis of the
/// section the germ line lies on, or `None` when that locus is
/// STRAIGHT — a plane×plane pair, and the degenerate plane×cylinder
/// outcomes whose loci ARE lines (ParallelLines/TangentLine).
///
/// **`None` is a claim, not a default.** The caller reads it as "take
/// the straight-chord facing test", so a pair whose section arm is not
/// wired must refuse ([`BooleanError::GermFrameUnsupported`]) rather
/// than fall through to it: falling through would mint a wrong chord
/// silently for every pair the dispatch later admits. The pair match
/// below is therefore EXHAUSTIVE over kinds by construction.
///
/// Section escalations propagate; non-escalation classification
/// failures at match time are a desync (the germ was minted FROM this
/// pair's crossing).
#[allow(clippy::type_complexity)] // (conic center, conic axis) — one frame tuple
fn germ_section_frame<T: Decide>(
    red: &BooleanReduction<T>,
    germ: &HalfGerm<T>,
    band: Band,
) -> Result<Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    // A germ along an edge of both solids lies on that edge: its frame
    // is the edge's own curve (a line is straight, a circle or an
    // ellipse turns about its centre and axis), whatever the two faces
    // the germ was recorded against — which may share one carrier.
    if let (super::Locus::OnEdge(edge), super::Locus::OnEdge(_)) = (germ.a_locus, germ.b_locus) {
        let curve = red
            .a
            .get_edge(edge)
            .and_then(|e| red.a.get_curve_geom(e.curve))
            .and_then(crate::null::CurveGeom::certified)
            .ok_or(desync("an OnEdge germ's edge carries no certified curve"))?;
        return match *curve.carrier() {
            geom::Curve3::Line { .. } => Ok(None),
            geom::Curve3::Circle { center, axis, .. }
            | geom::Curve3::Ellipse { center, axis, .. } => Ok(Some((center, axis))),
            _ => Err(desync(
                "an OnEdge germ's edge is neither a line nor a conic (the operand gates refuse \
                 the kinds)",
            )),
        };
    }
    let surf = |body: &Body<T>, f: FaceKey| -> Result<geom::Surface<T>, BooleanError> {
        body.get_face(f)
            .and_then(|fd| body.get_surface(fd.surface))
            .cloned()
            .ok_or(desync("germ face surface no longer resolves"))
    };
    let sa = surf(&red.a, germ.a_face)?;
    let sb = surf(&red.b, germ.b_face)?;
    // **The lowered parameter-identity channel's first production
    // read.** Radius equality is structural or declared and never
    // inferred from values, and this is the ONE site that can say
    // which: the germ carries both bodies and both face keys, so the
    // per-field side records are reachable here and nowhere below.
    // `Declared` iff the recipe layer evaluated ONE expression into
    // both carriers' radius fields; every other configuration —
    // differing expressions, either side unsourced, imported or
    // hand-built geometry — is `None` and routes the general rung
    // permanently (VERB-SEAT-DESIGN P3).
    let evidence = crate::param_source::field_source_evidence(
        &red.a,
        germ.a_face,
        &red.b,
        germ.b_face,
        crate::param_source::SurfaceField::CylinderRadius,
    );
    // `surf` resolved both faces above, and nothing writes between, so
    // `face_witnesses` reads each.
    let witnesses = |body: &Body<T>, f: FaceKey| {
        super::rest::face_witnesses(body, f).unwrap_or_else(|| {
            unreachable!(
                "{}, which `surf` resolved, does not resolve",
                crate::entity::EntityId::Face(f)
            )
        })
    };
    let (at, span) = frame_reading(
        &sa,
        &sb,
        witnesses(&red.a, germ.a_face),
        witnesses(&red.b, germ.b_face),
    )
    .map_err(desync)?;
    pair_section_frame(&sa, &sb, evidence, at, span, band)
        .map_err(|e| frame_refusal(e, (germ.a_face, &sa), (germ.b_face, &sb)))
}

/// **Where the germ frame reads its axis rows, and its span**: the
/// centre of the curved face's boundary vertices (the first face's
/// where both are curved), a point among those the section is consumed
/// at, so the gap is read where the section is and not where a
/// carrier's origin is stored; and, for the cylinder pair, the span of
/// both walls' boundary vertices (the diameter of their ball), which a
/// wall's axial extent ends at. [`pair_section_frame`] levers the
/// plane×cylinder pair at the radius and the cylinder pair at the longer
/// of the larger radius and this span. No lever is a ball chosen around
/// the faces (`geom_brep::Reach`'s module docs); the radius under-states
/// a long wall's, filed as
/// `germ-frame-levers-a-plane-cylinder-tilt-at-the-radius`.
///
/// # Errors
///
/// A face pair with no boundary vertices.
#[allow(clippy::type_complexity)] // (reading point, span) — one reading
fn frame_reading<T: Decide>(
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

/// **The rulings lane's chords are rulings**: each wall's split lane,
/// run against the pair's radical plane, minted the straight chord.
///
/// A wall the plane only touches has a tangent chord
/// (`TangentIntersection`): the walls touch along one ruling, undeclared
/// contact that no arm here takes, refused `CurvedBooleanUnsupported`
/// naming that wall. Any other curve is a conic, a plane the frame read
/// parallel to both axes and a wall's table did not, which is a desync.
///
/// No pose reaches either refusal through a public door: walls that
/// touch along a ruling, or an axis pair the table reads apart from the
/// frame's verdict, meet the coincidence ladder and the crossing layer
/// first. The rows in `radical_plane_rows` hand it the curves directly.
fn rulings_are_straight<T: geom_core::Real>(
    sides: [(&SegmentCurve<T>, Operand, FaceKey); 2],
) -> Result<(), BooleanError> {
    for (curve, operand, face) in sides {
        match curve.spec().map(|spec| &spec.description) {
            None => {}
            Some(geom_brep::EdgeDescriptionSpec::TangentIntersection { .. }) => {
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
/// The offset's length is decided ([`BOOL_JOIN_CC_AXIS_OFFSET`]), metres
/// against the band: a zero offset is a coaxial pair, whose walls meet
/// nowhere or everywhere and name no plane, and is `None`; an offset in
/// the band is the walls' coincidence undecided. Whether the walls cross
/// is not asked here: each wall's own section table answers it against
/// this plane, as two rulings, one, or none.
///
/// The axes are read as unit and parallel, as the frame dispatch read
/// them before admitting the pair (`pair_section_frame`).
#[allow(clippy::type_complexity)] // (plane point, plane normal) — one datum tuple
pub(super) fn parallel_radical_plane<T: Decide>(
    own: &geom::Surface<T>,
    partner: &geom::Surface<T>,
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
            radius: r2,
            ..
        },
    ) = (own, partner)
    else {
        return Err(BooleanError::JoinDesync {
            what: "a cylinder pair's radical plane asked of a pair that is not two cylinders",
        });
    };
    let delta = *o2 - *o1;
    let w = delta - *a1 * delta.dot(*a1);
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
    Ok(Some((*o1 + normal.get() * x, normal)))
}

/// The Boolean's refusal for a germ pair's frame refusal, the pair
/// being the A face `a` and the B face `b` with their surfaces: a
/// section pose's escalation is a coincidence between the two walls,
/// asked on the pair's parameter-source evidence and no face-pair
/// declaration, and an operand guard's is that radius's own decision.
/// No declaration settles either.
pub(super) fn frame_refusal<T: geom_core::Real>(
    e: FrameError,
    a: (FaceKey, &geom::Surface<T>),
    b: (FaceKey, &geom::Surface<T>),
) -> BooleanError {
    match e {
        FrameError::Escalated(diag) => {
            BooleanError::coincidence(Coincide::Section, DeclarationRead::Moot, diag)
        }
        FrameError::RadiusEscalated { radius, diag } => BooleanError::Escalated {
            decision: BooleanDecision::Radius(radius),
            diag,
        },
        FrameError::Desync(what) => BooleanError::JoinDesync { what },
        FrameError::NoArm => BooleanError::GermFrameUnsupported {
            a_face: a.0,
            a_kind: a.1.kind(),
            b_face: b.0,
            b_kind: b.1.kind(),
        },
        FrameError::IntersectingCylinderAxes { evidence } => BooleanError::GermFrameCylinderPinch {
            a_face: a.0,
            b_face: b.0,
            evidence,
        },
    }
}

/// Why [`pair_section_frame`] could not name a frame. The keys and
/// bodies live at the call site, so this carries none of them: the
/// dispatch is a statement about the kind pair and the DECLARED
/// evidence it was handed, never about the arenas.
pub(super) enum FrameError {
    /// A section predicate landed in the sliver band.
    Escalated(geom_core::Indeterminate),
    /// An operand's radius guard landed in the sliver band
    /// ([`geom_brep::SectionError::RadiusEscalated`]).
    RadiusEscalated {
        /// Whose radius the guard read.
        radius: geom_brep::SectionRadius,
        /// The guard's diagnostics.
        diag: geom_core::Indeterminate,
    },
    /// The classification contradicted the germ that was minted from
    /// it — a lockstep failure, not a frontier.
    Desync(&'static str),
    /// No frame: the kind pair has no section arm, or an armed pair's
    /// pose has none (the cylinder × sphere node, or a pose not
    /// definitely off the axis).
    NoArm,
    /// **A cylinder pair whose axes definitely INTERSECT.** Its locus
    /// is never straight and is never one conic either, so it is
    /// neither `Ok(None)` nor an arm — see
    /// [`BooleanError::GermFrameCylinderPinch`] for the two shapes it
    /// takes and why this dispatch can name neither.
    ///
    /// The evidence it was reached under is carried, because it is what
    /// decides WHICH of those two shapes the locus has: `Declared` and
    /// the section is the verified ellipse pair, `None` and the pair
    /// routes the general rung with the question unanswered.
    IntersectingCylinderAxes {
        /// The radius-equality evidence the germ site read off the
        /// parameter-identity channel.
        evidence: geom_brep::RadiusEvidence,
    },
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
/// The plane×cylinder and cylinder pairs read their axis rows at the
/// foot of `at` on each axis ([`germ_section_frame`]: a point the
/// section is consumed at). The plane×cylinder pair is levered at its
/// radius; the cylinder pair at `span` (the length its walls run
/// together, where the caller knows it) or the larger radius, whichever
/// is longer — two axes a sine θ apart drift θ·span apart over the
/// walls. `None` levers by the radii alone.
#[allow(clippy::type_complexity)] // (conic center, conic axis) — one frame tuple
pub(super) fn pair_section_frame<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    evidence: geom_brep::RadiusEvidence,
    at: geom_core::Point3<T>,
    span: Option<T>,
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
                Err(geom_brep::SectionError::Escalated(diag)) => Err(FrameError::Escalated(diag)),
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
                Err(geom_brep::SectionError::Escalated(diag)) => Err(FrameError::Escalated(diag)),
                Err(_) => Err(FrameError::Desync(
                    "germ pair's section refused at match time",
                )),
            };
        }
        // **Cylinder×sphere** ([`cs_germ_frame`]): the declared-coaxial
        // classification first ([`cs_pair_frame`], unreadable here
        // yet), then the transverse frame for a pose off the axis
        // ([`cs_transverse_frame`]).
        (Sf::Cylinder { .. }, Sf::Sphere { .. }) => return cs_germ_frame(sa, sb, band),
        (Sf::Sphere { .. }, Sf::Cylinder { .. }) => return cs_germ_frame(sb, sa, band),
        // The ONE structurally straight pair: a plane×plane section is
        // a line, so "no frame" is a proof here rather than a default.
        (Sf::Plane { .. }, Sf::Plane { .. }) => return Ok(None),
        // **Cylinder×cylinder.** Two walls with PARALLEL axes meet in
        // rulings — lines — whatever their radii, so `None` here is
        // proven by the axes alone and needs neither radius evidence
        // nor a constructed section: the declared tangent-ruling pair
        // the zip lane rests on is exactly this case. The non-parallel
        // half is never straight, and it splits again on coplanarity
        // (below): skew keeps the general rung's `NoArm`, intersecting
        // axes take their own named door.
        //
        // Asked of the section table's own row
        // ([`geom_brep::cylinder_axes_parallel`], `cc_axes_parallel`),
        // levered at the larger radius or the walls' span, whichever is
        // longer: the axes drift apart by the sine times the length they
        // run together. A bigger lever only moves a reading toward the
        // definite side, and here that side refuses (skew keeps `NoArm`,
        // meeting axes take the pinch door below), so it never serves a
        // frame the walls' span would not. The table re-decides this
        // margin on the intersecting half below from the same reach, so
        // the two are one reading.
        (
            Sf::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
                ..
            },
            Sf::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
                ..
            },
        ) => {
            let lever = span.map_or(r1.max(*r2), |l| l.max(r1.max(*r2)));
            let reach = geom_brep::Reach::Measured { at, lever };
            match geom_brep::cylinder_axes_parallel(&reach, (*o1, *a1), (*o2, *a2), band) {
                Ok(Sign::Zero) => return Ok(None),
                Ok(Sign::Positive | Sign::Negative) => {}
                Err(diag) => return Err(FrameError::Escalated(diag)),
            }
            // **The non-parallel half, split on COPLANARITY.** The
            // signed axis-to-axis gap along `a1×a2` is the section
            // table's own `cc_axes_coplanar` margin
            // (`geom_brep::cylinder_cylinder_section` step 5) and it
            // reads NO radius, so asking it here infers nothing: a
            // definite gap is SKEW, whose locus is a space quartic —
            // canal territory, the general rung — and keeps `NoArm`
            // verbatim. A Zero gap means the axes meet, and that half
            // gets its own named door because its locus is not one
            // conic even when it is a conic pair.
            //
            // Asked of the table's own row too
            // ([`geom_brep::cylinder_axes_coplanar`], `cc_axes_coplanar`),
            // read between the axes' feet. What the rows pin is its
            // direction: `skew_axes_keep_the_general_rung_at_the_certified_scalar`
            // reds the moment the gap is measured along an axis instead
            // of along `a1×a2`.
            return match geom_brep::cylinder_axes_coplanar(&reach, (*o1, *a1), (*o2, *a2), band) {
                Ok(Sign::Zero) => Err(intersecting_cylinder_axes(sa, sb, evidence, &reach, band)),
                Ok(Sign::Positive | Sign::Negative) => Err(FrameError::NoArm),
                Err(diag) => Err(FrameError::Escalated(diag)),
            };
        }
        _ => return Err(FrameError::NoArm),
    };
    let reach = geom_brep::Reach::Measured { at, lever: radius };
    match geom_brep::plane_cylinder_section(plane_s, cyl_s, &reach, band) {
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
        Err(geom_brep::SectionError::Escalated(diag)) => Err(FrameError::Escalated(diag)),
        Err(_) => Err(FrameError::Desync(
            "germ pair's section refused at match time",
        )),
    }
}

/// **The intersecting-axes cylinder pair, routed by the
/// parameter-identity channel.**
///
/// The axes meet, so the locus is neither straight nor one conic and no
/// frame can be named either way — but WHICH shape it has is a
/// radius-equality question, and the channel is the only thing that may
/// answer it (values never may). This is where the answer is used:
///
/// * `Declared` — the recipe layer put ONE expression into both radius
///   fields. The equal-radius closed form is then reachable, and it is
///   REACHED: `cylinder_cylinder_section` verifies the declaration
///   against the geometry (declared ≠ unchecked) and constructs the two
///   bisector-plane ellipses. The refusal that comes back out carries
///   the evidence, so the pinch it names is a proven configuration —
///   four arcs at two valence-4 vertices — rather than one of two
///   possibilities.
/// * `None` — no channel, and none is ever inferred: the pair routes
///   the general rung with the question open. That is the permanent
///   fallback for imported and hand-built geometry, not a gap.
///
/// A declaration the geometry CONTRADICTS is a document-layer bug (D9
/// makes one expression evaluate to one value, so two fields sharing a
/// token cannot hold different radii) and is surfaced as a desync, not
/// laundered into the undeclared arm.
fn intersecting_cylinder_axes<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    evidence: geom_brep::RadiusEvidence,
    reach: &geom_brep::Reach<T>,
    band: Band,
) -> FrameError {
    if evidence == geom_brep::RadiusEvidence::None {
        return FrameError::IntersectingCylinderAxes { evidence };
    }
    // The radii are NEVER compared here — equality is the channel's
    // answer, not this function's. The table reads the same `reach`
    // the parallelism gate above levered from, because it re-decides
    // the two axis margins this dispatch just decided and must reach
    // the same verdicts from the same margins.
    //
    // Both matches are CLOSED (VERB-SEAT-DESIGN §0, D3): every section
    // outcome and every refusal is named, so a variant added to either
    // enum is a compile-time visit here rather than a silent desync.
    match geom_brep::cylinder_cylinder_section(sa, sb, evidence, reach, band) {
        Ok(geom_brep::EqualCylinderSection::TwoEllipses { .. }) => {
            FrameError::IntersectingCylinderAxes { evidence }
        }
        // The three parallel-axes answers, from a table whose
        // parallelism verdict this dispatch already took the other
        // way on the same margin.
        Ok(
            geom_brep::EqualCylinderSection::ParallelLines { .. }
            | geom_brep::EqualCylinderSection::TangentLine(_)
            | geom_brep::EqualCylinderSection::Empty,
        ) => FrameError::Desync(
            "the declared equal-radius section of an intersecting-axes cylinder pair \
             classified as a parallel-axes locus",
        ),
        Err(geom_brep::SectionError::Escalated(diag)) => FrameError::Escalated(diag),
        Err(geom_brep::SectionError::RadiusDeclarationContradicted) => FrameError::Desync(
            "two cylinder radii carrying the SAME lowered parameter source hold \
             different values — one expression evaluated to two numbers",
        ),
        // The table's own routing verdict, honoured the way
        // `cs_pair_frame` honours it: NOT a desync, since nothing is
        // contradicted — the pair marches one rung down. Reachable only
        // if the table's coplanarity verdict differed from this
        // dispatch's on the same margin, and the general rung is the
        // right home for a skew pair either way.
        Err(geom_brep::SectionError::RoutesToGeneralRung { .. }) => FrameError::NoArm,
        // Refusals that cannot come out of a cylinder pair this
        // dispatch admitted: the kinds were matched above, the
        // coaxial-equal-radius pose was refused at the parallelism
        // gate, no coaxiality, torus or conic-carrier question is
        // asked of two cylinders with meeting axes, and no arm this
        // pair reaches states a locus off the extent it was handed
        // (the cylinder pair's ellipses stand on the operands
        // themselves).
        Err(
            geom_brep::SectionError::WrongLane { .. }
            | geom_brep::SectionError::RadiusEscalated { .. }
            | geom_brep::SectionError::CoaxialDeclarationContradicted
            | geom_brep::SectionError::DegenerateOperand { .. }
            | geom_brep::SectionError::CoincidentSurfaces
            | geom_brep::SectionError::DegenerateTorus
            | geom_brep::SectionError::BeyondOperandExtent { .. }
            | geom_brep::SectionError::Carrier(_)
            | geom_brep::SectionError::Spiric(_),
        ) => FrameError::Desync(
            "the declared equal-radius cylinder section refused at the germ pair \
             with a refusal this pair cannot produce",
        ),
    }
}

/// The cylinder×sphere germ frame: the declared-coaxial classification
/// first ([`cs_pair_frame`]), and where it routes to the general rung,
/// the transverse frame ([`cs_transverse_frame`]).
#[allow(clippy::type_complexity)] // (frame centre, frame axis) — one frame tuple
fn cs_germ_frame<T: Decide>(
    cyl: &geom::Surface<T>,
    sph: &geom::Surface<T>,
    band: Band,
) -> Result<Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>, FrameError> {
    match cs_pair_frame(cyl, sph, geom_brep::CoaxialEvidence::None, band) {
        Err(FrameError::NoArm) => cs_transverse_frame(cyl, sph, band),
        other => other,
    }
}

/// **The transverse cylinder×sphere germ frame**: a centre and an axis
/// the section turns about monotonically, once per loop, which is all
/// the rotational-sense facing test and the turn ranking read of a
/// frame.
///
/// In the cylinder's frame put the sphere's centre `s` at offset `d`
/// from the axis along `û` (the axis's unit perpendicular towards `s`),
/// the cylinder's radius `r`, the sphere's `R`. A wall point at azimuth
/// `θ` from `û` lies on the sphere at heights `±h(θ)` over `s`, with
/// `h² = R² − r² − d² + 2rd·cos θ`, so the section is one loop when
/// `R < r + d` and two when `R > r + d`:
///
/// * **One loop** (`bool_germ_frame_cs_reach` Negative): it spans the
///   azimuths where `h² > 0`, up one side and down the other. About the
///   axis `û` through `s` its projection `(r sin θ, ±h)` turns with
///   `(y, z) × (y′, z′) = −r·(rd·sin²θ + h²·cos θ)/h`, and with
///   `c₀ = cos θ₀` the azimuth where `h` vanishes,
///   `rd·sin²θ + h²·cos θ = rd·((cos θ − c₀)² + 1 − c₀²)`, positive
///   for every `|c₀| < 1` — that is, whenever the loop exists and is not
///   a tangency. So `(s, û)` is the frame, and no germ on the loop is
///   radial about it.
/// * **Two loops** (Positive): each is the graph `±h(θ)` over the whole
///   circle, so it turns monotonically about the cylinder's own axis.
///   Both share that axis, the declared-coaxial frame's argument.
/// * **A tangency** (Zero: the walls touch at `θ = π`, the loop's
///   figure-eight node) has no frame and keeps [`FrameError::NoArm`].
///
/// The offset `d` must be definite (`bool_germ_frame_cs_offset`): `û`
/// does not exist on the axis, and a coaxial pose is never read from a
/// measured `d` ([`cs_pair_frame`]), so a Zero or in-band offset keeps
/// `NoArm` verbatim rather than escalating. Radii are read by magnitude:
/// a negative stored radius denotes the same point set.
#[allow(clippy::type_complexity)] // (frame centre, frame axis) — one frame tuple
fn cs_transverse_frame<T: Decide>(
    cyl: &geom::Surface<T>,
    sph: &geom::Surface<T>,
    band: Band,
) -> Result<Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>, FrameError> {
    let (
        &geom::Surface::Cylinder {
            origin,
            axis,
            radius: r,
            ..
        },
        &geom::Surface::Sphere {
            center,
            radius: big_r,
            ..
        },
    ) = (cyl, sph)
    else {
        return Err(FrameError::Desync(
            "the cylinder×sphere frame was handed another kind pair",
        ));
    };
    let foot = origin + axis * (center - origin).dot(axis);
    let off = center - foot;
    let d = off.norm();
    match decide("bool_germ_frame_cs_offset", Margin::of(d), band) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) | Err(_) => return Err(FrameError::NoArm),
    }
    match decide(
        "bool_germ_frame_cs_reach",
        Margin::of(big_r.abs() - r.abs() - d),
        band,
    ) {
        Ok(Sign::Negative) => Ok(Some((center, off * (T::one() / d)))),
        Ok(Sign::Positive) => Ok(Some((foot, axis))),
        // `NoArm`, not the tangency `Desync` the circle arms answer: a
        // germ minted from a crossing elsewhere on the figure-eight is
        // real, and only the frame is missing.
        Ok(Sign::Zero) => Err(FrameError::NoArm),
        Err(diag) => Err(FrameError::Escalated(diag)),
    }
}

/// The cylinder×sphere germ frame, from the DECLARED-coaxial
/// classification (`geom_brep::cylinder_sphere_section`).
///
/// **ONE frame serves BOTH section circles, and that is a proof rather
/// than a convenience.** The declared-coaxial section is two circles of
/// the cylinder's radius, centred at `c ± axis·station` on the
/// cylinder's own axis — the SAME axis for both. The facing test this
/// frame feeds asks only for the rotational sense `axis·((p−c)×dir)`,
/// and sliding `c` along `axis` changes `p−c` by a multiple of `axis`,
/// which the triple product annihilates. So the sense a germ gets is
/// identical whichever of the two stations (or the sphere centre
/// between them) is handed over, and the pair needs neither a second
/// frame nor a pinch door. This is exactly where the cylinder PAIR
/// differs: its crossing ellipses lie in two DIFFERENT bisector
/// planes with two different axes, and no single frame is right for
/// both — hence [`FrameError::IntersectingCylinderAxes`] there and
/// nothing like it here.
///
/// **The declaration cannot be read at this door yet, and that is
/// stated rather than hidden.** This dispatch is keyed on surface
/// KINDS alone — no bodies, no keys — so it has nowhere to consult a
/// coincidence ladder, and coaxiality is declared-only: it is NEVER
/// inferred from a measured axis-to-centre distance, at any tolerance.
/// Every in-tree caller therefore passes
/// `geom_brep::CoaxialEvidence::None`, the section routes to the
/// general rung, and the dispatch reads the transverse frame
/// ([`cs_transverse_frame`]), which keeps [`FrameError::NoArm`]
/// VERBATIM for a pose not definitely off the axis. Coaxiality is placement data (an
/// axis, a centre), so the scalar-field channel read at
/// `germ_section_frame` cannot carry it; its carrier is the axis-shaped
/// identity channel (`docs/AXIS-DECLARATION-DESIGN.md`), unbuilt. A
/// coaxiality declaration enters HERE, and the `Declared` path below is
/// what it reaches.
#[allow(clippy::type_complexity)] // (conic center, conic axis) — one frame tuple
pub(super) fn cs_pair_frame<T: Decide>(
    cyl: &geom::Surface<T>,
    sph: &geom::Surface<T>,
    evidence: geom_brep::CoaxialEvidence,
    band: Band,
) -> Result<Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>, FrameError> {
    match geom_brep::cylinder_sphere_section(cyl, sph, evidence, band) {
        Ok(geom_brep::CylinderSphereSection::TwoCircles { center, axis, .. }) => {
            Ok(Some((center, axis)))
        }
        // A tangent circle / empty gap under a minted germ is a
        // touching (or absent) configuration the reduction should not
        // have paired — loud, typed, exactly as the sphere arms above.
        // The tangency is classification data at BOTH doors: the
        // marcher's own `ssi_cs_tangency` refuses the same pose toward
        // C7, and nothing here re-adjudicates it.
        Ok(
            geom_brep::CylinderSphereSection::TangentCircle { .. }
            | geom_brep::CylinderSphereSection::Empty,
        ) => Err(FrameError::Desync(
            "germ pair's cylinder×sphere section is not a locus",
        )),
        Err(geom_brep::SectionError::Escalated(diag)) => Err(FrameError::Escalated(diag)),
        Err(geom_brep::SectionError::RadiusEscalated { radius, diag }) => {
            Err(FrameError::RadiusEscalated { radius, diag })
        }
        // The undeclared / non-coaxial pose. NOT a desync: nothing is
        // contradicted, the pair simply has no exact arm at this door
        // and marches one rung down.
        Err(geom_brep::SectionError::RoutesToGeneralRung { .. }) => Err(FrameError::NoArm),
        Err(_) => Err(FrameError::Desync(
            "germ pair's section refused at match time",
        )),
    }
}

/// Mutual germ facing along the germ LOCUS (M5 PR 9 fix pass, dev 4).
/// Straight germ lines keep the M3 chord test bit-identically: both
/// dirs definitely point at each other along the chord (Zero =
/// definite non-facing, `continue` semantics; in-band escalates in
/// `decide`). A CONIC germ locus makes the chord test structurally
/// degenerate — a semicircle arc leaves BOTH sites exactly
/// perpendicular to the chord (the two-arc disc, PR 5's canonical
/// authoring, hit exactly this as `UnpairedLooseEnds` "(kernel
/// bug)") — so the arc-aware test asks the honest question instead:
/// do the two germs bound ONE rotational traversal of the section
/// conic, i.e. do their rotational senses `axis·((p−c)×dir)` (metres:
/// |p−c| ~ radius, dir unit) definitely OPPOSE? A Zero sense is a
/// radial germ — malformed germ data, a loud desync, never a silent
/// non-match; its in-band sibling escalates through the funnel
/// (`bool_join_arc_facing`), the two-tolerance pair.
pub(super) fn germs_face_each_other<T: Decide>(
    frame: Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>,
    g1: &HalfGerm<T>,
    g2: &HalfGerm<T>,
    p1: geom_core::Point3<T>,
    p2: geom_core::Point3<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    let escalate = |diag| BooleanError::coincidence(Coincide::Join, DeclarationRead::Moot, diag);
    match frame {
        None => {
            let chord = p2 - p1;
            // Facing margins in METRES: unit germ dir · chord = cos ×
            // separation (rim-dimensional audit: the former `/ dist`
            // stripped the metres and compared a bare cosine against
            // the length band — class (c)).
            let f1 = g1.dir.dot(chord);
            let f2 = g2.dir.dot(-chord);
            Ok(
                decide("bool_join_facing", Margin::of(f1), band).map_err(escalate)?
                    == Sign::Positive
                    && decide("bool_join_facing", Margin::of(f2), band).map_err(escalate)?
                        == Sign::Positive,
            )
        }
        Some(frame) => {
            let d1 = rotational_sense(frame, p1, g1.dir, band)?;
            let d2 = rotational_sense(frame, p2, g2.dir, band)?;
            match (d1, d2) {
                (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Ok(true),
                (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Ok(false),
                (Sign::Zero, _) | (_, Sign::Zero) => {
                    Err(BooleanError::JoinDesync { what: RADIAL_GERM })
                }
            }
        }
    }
}

type LooseMap = SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>;

/// Still-unused null-edge halves, each mapped to its geometric MATCH
/// PARTNER's half in the same solid: the nearest loose germ along its
/// line ([`nearer_along`]) that is its partner by [`partners`] —
/// [`find_match`]'s own criterion,
/// static in the germ geometry, so a captured partner PAIR can still
/// join (same face) while splitting a pair walls one side off. Germ
/// meta is shared between the solids, so the (record, slot) partner
/// relation is computed once (A-clone points — coincident copies) and
/// translated per solid. A loose half with no partner maps to `None`
/// (conservatively separated wherever captured).
fn loose_partners<T: Decide>(
    open: &[OpenRecord<T>],
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
            // The matcher's own criterion ([`partners`]): the separation
            // constraint must count partners with the matcher's eyes or
            // roles get walled off wrongly.
            let Some(reach) = partners(open, red, sa, sb, (i, s), (j, t), band)? else {
                continue;
            };
            best = keep_nearer(best, (reach, (j, t)), |c, b| nearer_along(c, b, band))?;
        }
        let partner = best.map(|(_, jt)| jt);
        a_map.insert(open[i].a[s].0.he, partner.map(|(j, t)| open[j].a[t].0.he));
        b_map.insert(open[i].b[s].0.he, partner.map(|(j, t)| open[j].b[t].0.he));
    }
    Ok((a_map, b_map))
}

/// Chooses the join role order for one solid (PR 5.5 — the enforced
/// discipline; module docs for the derivation). The three lanes:
///
/// - **Different loops** (the mekr lane): a pure loop merge — role
///   order is orientation-neutral; keep the given order. Loops of two
///   faces take this lane too, and what runs on the plan's face
///   refuses them ([`JoinPlan::of`]).
/// - **Same loop, the face's OUTER**: the split partitions real
///   boundary between two faces; either partition names the same two
///   directed cycles (role order moves only face identity), so the
///   order is chosen by the clean-arc constraint alone: the first
///   chord's mef run `[h1 .. h2]` must not SEPARATE a still-loose
///   scaffolding pair (walling a pending site off from its partner).
///   Both arcs dirty is a loud desync.
/// - **Same loop, a RING of its face** (the closed seam-ring lane —
///   pierce-ring scaffolding): the split's remainder stays a ring of
///   the old face and must anti-enclose (a hole boundary), so the mef
///   run — the enclosed patch, the new face's outer, closed by the
///   segment's own curve — must wind CCW around the face's outward
///   normal. Decided intrinsically by [`ring_run_ccw`] once the curve is
///   known ([`RoleLane::resolve`]). A derived order whose run separates
///   a loose pair is a loud desync.
///
/// Cross-solid consistency needs NO coupling of the two solids' role
/// orders: the sense attributes carry the seam orientation (the
/// anti-correlation theorem), and each solid's partition is decided
/// against its own geometry — the zip's antiparallelism assertion is
/// the runtime witness.
fn choose_roles<T: Decide>(
    body: &Body<T>,
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
        return clean_dir(body, ea, ra, loose)?
            .map(RoleLane::Decided)
            .ok_or(desync("every chord arc separates a loose scaffolding pair"));
    }
    // Ring lane: decided by the run's winding. A planar, sphere or cone
    // face's run is closed by the segment's curve, so it waits on that
    // curve ([`RoleLane::resolve`]); a cylinder face's closes along its section
    // plane on its chart, decided here, before the curve is computed in
    // the order it decides. An along-edge segment reads no section: a
    // curved face there refuses typed, before any chord is computed.
    enum RingFace<T: geom_core::Real> {
        Plane(Vec3<T>),
        Wall((Point3<T>, UnitVec3<T>)),
        Quadric,
    }
    let on_quadric = body
        .get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .is_some_and(|s| matches!(s, geom::Surface::Sphere { .. } | geom::Surface::Cone { .. }));
    let ring = match closure {
        RingClosure::Wall(_) if on_quadric => RingFace::Quadric,
        RingClosure::Wall(section) => RingFace::Wall(section),
        RingClosure::Planar => RingFace::Plane(
            face_outward_normal(body, face)
                .ok_or(desync(
                    "a planar germ face's ring lane found no planar carrier",
                ))?
                .vec(),
        ),
        RingClosure::AlongEdge(operand) => match face_outward_normal(body, face) {
            Some(normal) => RingFace::Plane(normal.vec()),
            None => {
                let kind = body
                    .get_face(face)
                    .and_then(|f| body.get_surface(f.surface))
                    .map(geom::Surface::kind)
                    .ok_or(desync("ring-lane face no longer resolves"))?;
                return Err(BooleanError::CurvedBooleanUnsupported {
                    operand,
                    face,
                    kind,
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
    // same site, or the match is refused, and the order the minted
    // chord's `mef` runs on keeps the loose-pair separation constraint
    // every same-loop order does.
    for (x, y) in [(ea, ra), (ra, ea)] {
        let sites = |order| {
            JoinPlan::of(body, order, SegmentEdge::Locus(segment), band)
                .map(|plan| plan.sites())
                .map_err(BooleanError::Join)
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
            return match clean_dir(body, x, y, loose)? {
                Some((c1, _)) if c1 == x => Ok(RoleLane::Decided((x, y))),
                _ => Err(desync(
                    "a match across its segment's edge separates a loose scaffolding pair",
                )),
            };
        }
    }
    match ring {
        RingFace::Plane(normal) => Ok(RoleLane::Ring { face, normal }),
        RingFace::Quadric => Ok(RoleLane::QuadricRing { face }),
        RingFace::Wall(section) => {
            let ccw = ring_run_ccw(body, face, (ea, ra), IslandClosing::Wall(section), band)?;
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
    /// A ring of the sphere or cone `face`.
    QuadricRing { face: FaceKey },
}

impl<T: Decide> RoleLane<T> {
    /// The order the segment's curve is computed in: the decided one, or
    /// the match's own on the ring lane, where either order names the
    /// germs' one arc.
    fn curve_order(&self, given: (HalfEdgeKey, HalfEdgeKey)) -> (HalfEdgeKey, HalfEdgeKey) {
        match *self {
            RoleLane::Decided(order) => order,
            RoleLane::Ring { .. } | RoleLane::QuadricRing { .. } => given,
        }
    }

    /// The role order. On the ring lane (issue #93) it is fully
    /// determined by the face's own orientation — the mef run (the
    /// enclosed patch, the new face's outer) must wind CCW around the
    /// face's outward normal so the remainder ring anti-encloses, closed
    /// by the segment's `curve` ([`ring_run_ccw`]). Exactly one of the
    /// two orders satisfies it (the candidate runs are antiparallel
    /// copies). This replaces the PR 5.5 residual-material-side probe,
    /// which anchored on the face's outer-loop vertices and was UNSOUND
    /// mid-fixpoint on faces hosting several pending polygons: the outer
    /// anchor classified a region other pending seams still separate
    /// from the island's immediate surround (the A×Z counter island —
    /// surround IN, outer corners OUT — silently crossed the copies; the
    /// zip's antiparallelism witness caught it).
    fn resolve(
        self,
        body: &Body<T>,
        (ea, ra): (HalfEdgeKey, HalfEdgeKey),
        loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
        curve: &SegmentCurve<T>,
        band: Band,
    ) -> Result<(HalfEdgeKey, HalfEdgeKey), BooleanError> {
        let (face, closing) = match self {
            RoleLane::Ring { face, normal } => (face, IslandClosing::Planar(normal, curve)),
            RoleLane::QuadricRing { face } => (face, IslandClosing::Quadric(curve)),
            RoleLane::Decided(_) => return Ok(self.curve_order((ea, ra))),
        };
        let ccw = ring_run_ccw(body, face, (ea, ra), closing, band)?;
        ring_order(body, (ea, ra), loose, ccw)
    }
}

/// The ring lane's role order from the island's winding: CCW keeps the
/// match's order. A derived order whose run separates a loose pair is a
/// loud desync.
fn ring_order<T: Decide>(
    body: &Body<T>,
    (ea, ra): (HalfEdgeKey, HalfEdgeKey),
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
    ccw: bool,
) -> Result<(HalfEdgeKey, HalfEdgeKey), BooleanError> {
    let (h1, h2) = if ccw { (ea, ra) } else { (ra, ea) };
    match clean_dir(body, h1, h2, loose)? {
        Some((c1, _)) if c1 == h1 => Ok((h1, h2)),
        _ => Err(BooleanError::JoinDesync {
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
/// merges). A spiric or spline run edge, which the operand gate keeps
/// out, refuses loudly.
///
/// A wall face ([`IslandClosing::Wall`]) asks the same question on its
/// own chart, [`crate::chord_join::chart_island_winding`], with the run
/// closed along the plane this solid's chords lie in; a sphere or cone
/// face ([`IslandClosing::Quadric`]) asks it without a chart,
/// [`crate::chord_join::path_island_winding`], closed by the curve.
fn ring_run_ccw<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
    closing: IslandClosing<'_, T>,
    band: Band,
) -> Result<bool, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let (normal, curve) = match closing {
        IslandClosing::Wall(section) => {
            let wound =
                crate::chord_join::chart_island_winding(body, face, (h1, h2), section, band)
                    .map_err(BooleanError::Join)?;
            return ring_winding_order(wound);
        }
        IslandClosing::Quadric(curve) => {
            let closing = curve.run_closing(h1, face).map_err(BooleanError::Join)?;
            let wound = crate::chord_join::path_island_winding(
                body,
                face,
                (h1, h2),
                closing.as_ref(),
                band,
            )
            .map_err(BooleanError::Join)?;
            return ring_winding_order(wound);
        }
        IslandClosing::Planar(normal, curve) => (normal, curve),
    };
    let closing = curve.run_closing(h1, face).map_err(BooleanError::Join)?;
    let wound = body
        .planar_run_winding_decided((h1, h2), RunClosing::of(closing.as_ref()), normal, band)
        .map_err(|RunMissesEnd| desync("ring-run arc did not close"))?
        // The operand gate refuses a spiric or spline carrier and no
        // section lane mints one on a plane, so a run carrying one is
        // the gate's invariant broken: the chord joiner's own reading
        // of the same edge.
        .ok_or(BooleanError::Join(SplitJoinError::SectionInvariant {
            face,
            what: "the ring lane reached a spiric or spline run edge (the operand gates refuse \
                   the kinds)",
        }))?;
    ring_winding_order(wound.map(|decided| decided.sign))
}

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
        Sign::Zero => Err(desync(
            "ring-run winding is degenerate (zero enclosed area)",
        )),
    }
}

/// The clean chord-arc role order, if any (doc at [`choose_roles`]):
/// `Some((h1, h2))` such that the `next`-order arc `h1 → h2` avoids
/// every `unused` half; different loops trivially clean.
fn clean_dir<T: Decide>(
    body: &Body<T>,
    ea: HalfEdgeKey,
    ra: HalfEdgeKey,
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
) -> Result<Option<(HalfEdgeKey, HalfEdgeKey)>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let loop_of = |he: HalfEdgeKey| -> Result<crate::entity::LoopKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or(desync("role half no longer resolves"))?
            .parent_loop)
    };
    if loop_of(ea)? != loop_of(ra)? {
        return Ok(Some((ea, ra)));
    }
    // A direction is BAD iff its arc SEPARATES a loose half from its
    // match partner (captures exactly one of a partner pair, or a
    // half with no computable partner — the capture would wall it off
    // on the new face where its partner cannot reach it). Capturing a
    // complete partner pair together is harmless: they still share a
    // face and join there.
    let separates = |from: HalfEdgeKey, to: HalfEdgeKey| -> Result<bool, BooleanError> {
        let mut inside: Vec<HalfEdgeKey> = Vec::new();
        let mut he = body
            .get_half_edge(from)
            .ok_or(desync("role arc start no longer resolves"))?
            .next;
        let mut steps = 0usize;
        while he != to {
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
        for &h in &inside {
            match loose.get(h).copied().flatten() {
                // Last loose half of its record: its partner is at
                // another site — separated.
                None => return Ok(true),
                Some(sib) => {
                    if !inside.contains(&sib) {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    };
    if !separates(ea, ra)? {
        Ok(Some((ea, ra)))
    } else if !separates(ra, ea)? {
        Ok(Some((ra, ea)))
    } else {
        Ok(None)
    }
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
    let a_out = sa
        .joiner
        .cut_core(&mut red.a, a_edge, tol)
        .map_err(BooleanError::Join)?;
    let b_out = sb
        .joiner
        .cut_core(&mut red.b, b_edge, tol)
        .map_err(BooleanError::Join)?;
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
/// Strut side labels are never consulted: pierce-ring struts carry
/// provisional labels, and a single-face seam ring has no in-solid
/// label to anchor on.
///
/// What this question adds to the ladder is [`loop_roles`]: the two
/// loops' regions flank the seam, so their sides are opposite.
fn resolve_roles_geometric<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    other_pristine: &Body<T>,
    face: FaceKey,
    outer: LoopKey,
    ring: LoopKey,
    band: Band,
    tol: Tol,
) -> Result<(LoopKey, LoopKey), BooleanError> {
    let side = |l: LoopKey| -> Result<Reading, BooleanError> {
        complex_side(
            body,
            &region_faces(body, face, l)?,
            other_pristine,
            band,
            tol,
        )
    };
    loop_roles(face, (outer, side(outer)?), (ring, side(ring)?))
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
/// **Neither deciding** is the curved-face frontier, refused
/// [`SplitJoinError::SectionLoopUndecided`]: every witness of both
/// loops' regions read the other boundary or too near it, which a
/// crossing's two flanks cannot both do unless their faces are all
/// curved (`work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior`).
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
            Err(BooleanError::Join(SplitJoinError::SectionLoopMixed {
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
                BooleanError::Join(SplitJoinError::SectionLoopUndecided { face })
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
    let desync = |what| BooleanError::JoinDesync { what };
    let LoopBoundary::Cycle { first } = body
        .get_loop(l)
        .ok_or(desync("completed section loop no longer resolves"))?
        .boundary
    else {
        return Err(desync("completed section loop is empty"));
    };
    let mut out: Vec<FaceKey> = Vec::new();
    for ch in body
        .loop_cycle(first)
        .ok_or(desync("completed section loop not walkable"))?
    {
        let mate = body.mate(ch).ok_or(desync("section half has no mate"))?;
        let region_loop = body
            .get_half_edge(mate)
            .ok_or(desync("section mate no longer resolves"))?
            .parent_loop;
        let region_face = body
            .get_loop(region_loop)
            .ok_or(desync("region loop no longer resolves"))?
            .face;
        if region_face != face && !out.contains(&region_face) {
            out.push(region_face);
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
                    BooleanError::Join(SplitJoinError::SectionLoopMixed { face: f }) if f == face
                ),
                "both {s:?}: {err:?}"
            );
        }
        let err = loop_roles(face, (o, undecided()), (r, undecided())).unwrap_err();
        assert!(
            matches!(
                err,
                BooleanError::Join(SplitJoinError::SectionLoopUndecided { face: f }) if f == face
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
        };
        let (o, z) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let err = super::germs_face_each_other(
            Some((o, z)),
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
        let err = super::ring_run_ccw(body, prism.top_face, (h1, h2), up, b)
            .expect_err("an in-band winding escalates");
        assert_defect(&err, SelfCheck::RingWinding, "bool_ring_run_winding");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod frame_dispatch_tests {
    use geom_core::{Point3, Tol, Vec3};

    use super::{FrameError, cs_pair_frame, frame_refusal, pair_section_frame};

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
            let got = pair_section_frame(a, b, geom_brep::RadiusEvidence::None, at, span, band());
            assert!(
                matches!(got, Ok(None)),
                "({label}): the tangent ruling's frame, got {:?}",
                got.as_ref().map_err(|e| match e {
                    FrameError::Escalated(d) => d.predicate.unwrap_or("unnamed"),
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
    /// orders. At half the span it reads `0.75·zero` and at the radius
    /// `0.15·zero`, both Zero.
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
            let got = pair_section_frame(a, b, geom_brep::RadiusEvidence::None, at, span, band());
            assert!(
                matches!(
                    got,
                    Err(FrameError::Escalated(ref d)) if d.predicate == Some("cc_axes_parallel")
                ),
                "({label}): the tilt across the walls is in band, got {:?}",
                got.as_ref().map_err(|e| match e {
                    FrameError::Escalated(d) => d.predicate.unwrap_or("unnamed"),
                    _ => "a refusal",
                })
            );
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
                let got =
                    pair_section_frame(a, b, geom_brep::RadiusEvidence::None, at, span, band());
                assert!(
                    matches!(got, Ok(None)),
                    "stored {along} m along ({label}): the tangent ruling's frame, got {:?}",
                    got.as_ref().map_err(|e| match e {
                        FrameError::Escalated(d) => d.predicate.unwrap_or("unnamed"),
                        _ => "a refusal",
                    })
                );
            }
        }
    }

    /// **A coin on edge on a table is levered at its radius, not its
    /// box** (row B). The plane `z = 0` and a coin of radius 0.01 on
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
                let got = pair_section_frame(
                    a,
                    b,
                    geom_brep::RadiusEvidence::None,
                    coin_face,
                    None,
                    band(),
                );
                assert!(
                    matches!(
                        got,
                        Err(FrameError::Escalated(ref d)) if d.predicate == Some("pc_axis_plane_parallel")
                    ),
                    "k = {k} ({label}): an in-band tilt must escalate, got {:?}",
                    got.as_ref().map_err(|e| match e {
                        FrameError::Escalated(d) => d.predicate.unwrap_or("unnamed"),
                        _ => "a refusal",
                    })
                );
            }
        }
    }

    /// **An operand's radius guard escalates as the radius's own
    /// decision, on a real raise**: the declared-coaxial cylinder ×
    /// sphere arm reads each radius before any pose, and a cylinder
    /// whose radius lies in the band escalates there. The germ frame's
    /// refusal names whose radius, its lever and the tolerance the
    /// radius gives, and offers no declaration: no face pair names an
    /// operand's own size. The raise enters at `cs_pair_frame` with
    /// `CoaxialEvidence::Declared`, which no public door passes today,
    /// so this row pins the routing a caller of that evidence will
    /// reach, not a path a Boolean reaches now.
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
        let Err(e) = cs_pair_frame(&cyl, &sph, geom_brep::CoaxialEvidence::Declared, b) else {
            panic!("an in-band cylinder radius refuses the frame");
        };
        let face = FaceKey::default();
        let err = frame_refusal(e, (face, &cyl), (face, &sph));
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

    /// A cylinder about `axis` through `origin`. The chart's `u_ref` is
    /// any unit vector across the axis — this dispatch never reads it,
    /// and picking the less-aligned coordinate direction keeps it from
    /// being poison on an axis-aligned pose.
    fn cylinder_at(origin: Point3<f64>, axis: Vec3<f64>, radius: f64) -> geom::Surface<f64> {
        let seed = if axis.x.abs() < 0.5 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref: axis.cross(seed).normalize(),
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
    /// CYLINDER PAIR can reach `IntersectingCylinderAxes` — the split
    /// on coplanarity is the only producer of that variant — so those
    /// rows admit either refusal here and the row below pins which one
    /// each gets. Every other kind pair falls through to the
    /// dispatch's final arm, where `NoArm` is the only reachable
    /// answer, and a row that accepted the pinch variant there would
    /// green through a mis-routed dispatch.
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
            // Cylinder × sphere, both orders. This pose is EXACTLY
            // coaxial (`sphere()` sits on `cylinder()`'s axis) and its
            // walls cross, so it is the pair's declared-arm pose — and
            // it still refuses, because the declaration channel does
            // not exist and coaxiality is never inferred from the
            // measured distance. `NoArm` is the routing, not a
            // frontier: the general rung marches this pair. The pinch
            // variant stays unreachable — pinned exactly.
            (cylinder(Vec3::new(0.0, 0.0, 1.0)), sphere(), false),
            (sphere(), cylinder(Vec3::new(0.0, 0.0, 1.0)), false),
        ];
        for (a, b, cylinder_pair) in curved_pairs {
            let got =
                pair_section_frame(&a, &b, geom_brep::RadiusEvidence::None, at(), None, band());
            if cylinder_pair {
                assert!(
                    matches!(
                        got,
                        Err(FrameError::NoArm | FrameError::IntersectingCylinderAxes { .. })
                    ),
                    "a cylinder pair with no arm must refuse rather than default to the \
                     straight chord"
                );
            } else {
                assert!(
                    matches!(got, Err(FrameError::NoArm)),
                    "a non-cylinder pair with no arm must refuse with NoArm exactly — \
                     the pinch variant is not reachable for these kinds"
                );
            }
        }
    }

    /// **The non-parallel cylinder pair splits on COPLANARITY, and on
    /// nothing else.** Intersecting axes take the pinch door; skew
    /// axes keep the general rung's `NoArm` verbatim. The split reads
    /// no radius, so both rows run at equal AND unequal radii and get
    /// the same answer — which is the never-infer rule made visible:
    /// this dispatch cannot tell the two apart and does not try.
    #[test]
    fn the_non_parallel_cylinder_pair_splits_on_coplanarity_alone() {
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        for r in [1.0_f64, 0.4] {
            // Axes meeting at the origin.
            let meeting = pair_section_frame(
                &cylinder(z),
                &cylinder_at(Point3::new(0.0, 0.0, 0.0), x, r),
                geom_brep::RadiusEvidence::None,
                at(),
                None,
                band(),
            );
            assert!(
                matches!(meeting, Err(FrameError::IntersectingCylinderAxes { .. })),
                "r = {r}: intersecting axes take the pinch door"
            );
            // The same pair lifted along the common perpendicular
            // `a1 x a2` — the ONE direction that separates the axes. A
            // lift along either axis leaves them meeting, which is what
            // the margin measures and why it is the gap along `a1 x a2`
            // rather than any distance between the origins.
            let skew = pair_section_frame(
                &cylinder(z),
                &cylinder_at(Point3::new(0.0, 0.5, 0.0), x, r),
                geom_brep::RadiusEvidence::None,
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
                    geom_brep::RadiusEvidence::None,
                    at(),
                    None,
                    band()
                ),
                Ok(None)
            ),
            "parallel cylinder axes meet in rulings"
        );
    }

    /// **The intersecting-axes cylinder pair routes on the
    /// parameter-identity channel, and on nothing else.**
    ///
    /// The frontier is the same either way — no conic frame exists for
    /// a pair of crossing walls — but WHICH shape the locus has is a
    /// radius-equality question, and the refusal now carries the
    /// channel's answer instead of leaving it open. `Declared` reaches
    /// the equal-radius closed form (`cylinder_cylinder_section`), which
    /// verifies the declaration against the geometry and classifies the
    /// ellipse pair; `None` reaches nothing and routes the general rung
    /// with the question unanswered, which is the permanent fallback
    /// for imported and hand-built geometry.
    ///
    /// The radii are IDENTICAL in both rows: the only difference is the
    /// evidence, which is exactly the claim.
    #[test]
    fn the_intersecting_axes_pair_routes_on_declared_evidence() {
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let pair = || (cylinder(z), cylinder_at(Point3::new(0.0, 0.0, 0.0), x, 1.0));
        for evidence in [
            geom_brep::RadiusEvidence::None,
            geom_brep::RadiusEvidence::Declared,
        ] {
            let (a, b) = pair();
            let got = pair_section_frame(&a, &b, evidence, at(), None, band());
            match got {
                Err(FrameError::IntersectingCylinderAxes { evidence: carried }) => assert_eq!(
                    carried, evidence,
                    "the refusal must carry the evidence it was reached under"
                ),
                ref other => panic!(
                    "{evidence:?}: expected the pinch door, got {}",
                    outcome(other)
                ),
            }
        }
    }

    /// **A declared equality the geometry contradicts is a desync, not a
    /// quiet fall back to the undeclared arm.**
    ///
    /// Two carriers whose radius fields carry the same lowered source
    /// cannot hold different values — one expression evaluates to one
    /// number (D9) — so this configuration is a document-layer bug, and
    /// the dispatch says so rather than silently routing the general
    /// rung as if nothing had been declared. It is also the row that
    /// proves the closed form is genuinely REACHED on the declared
    /// side: only a call into the section table can notice this.
    #[test]
    fn a_declared_equality_the_geometry_contradicts_is_a_desync() {
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let got = pair_section_frame(
            &cylinder(z),
            &cylinder_at(Point3::new(0.0, 0.0, 0.0), x, 0.4),
            geom_brep::RadiusEvidence::Declared,
            at(),
            None,
            band(),
        );
        assert!(
            matches!(got, Err(FrameError::Desync(_))),
            "a contradicted declaration must be loud, got {}",
            outcome(&got)
        );
    }

    /// **The pinch is a property of the whole intersecting
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
    /// [`BooleanError::GermFrameCylinderPinch`]'s doc is checkable.
    #[test]
    fn the_equal_radius_section_always_crosses_at_two_pinch_points() {
        use geom_brep::{EqualCylinderSection, RadiusEvidence, cylinder_cylinder_section};
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
            let sec = cylinder_cylinder_section(
                &c1,
                &c2,
                RadiusEvidence::Declared,
                &geom_brep::Reach::Measured {
                    at: at(),
                    lever: 1.0,
                },
                band(),
            )
            .expect("intersecting equal-radius axes have a section");
            let EqualCylinderSection::TwoEllipses { e1, e2 } = sec else {
                panic!("{deg}°: the intersecting equal-radius section is the ellipse pair");
            };
            let n = a1.cross(a2).normalize();
            for sign in [1.0_f64, -1.0] {
                let pinch = p + n * (sign * r);
                for (name, e) in [("e1", &e1), ("e2", &e2)] {
                    let geom::Curve3::Ellipse {
                        center,
                        axis,
                        major,
                        minor,
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
                    let t = (w.dot(v_ref) / minor).atan2(w.dot(u_ref) / major);
                    let d = (e.eval(t) - pinch).norm();
                    assert!(
                        d < 1e-12,
                        "{deg}°, {name}, sign {sign}: the pinch point is {d} off the ellipse"
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
                pair_section_frame(
                    &plane(),
                    &plane(),
                    geom_brep::RadiusEvidence::None,
                    at(),
                    None,
                    band()
                ),
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
                    geom_brep::RadiusEvidence::None,
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
                    geom_brep::RadiusEvidence::None,
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
                pair_section_frame(
                    &plane(),
                    &sphere(),
                    geom_brep::RadiusEvidence::None,
                    at(),
                    None,
                    band()
                ),
                Ok(Some(_))
            ),
            "plane×sphere names its circle frame"
        );
        // Two walls with parallel axes meet in RULINGS, so the straight
        // answer is proven by the axes alone — the declared
        // tangent-ruling germ pair the zip lane rests on.
        assert!(
            matches!(
                pair_section_frame(
                    &cylinder(Vec3::new(0.0, 0.0, 1.0)),
                    &cylinder(Vec3::new(0.0, 0.0, 1.0)),
                    geom_brep::RadiusEvidence::None,
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
                geom_brep::RadiusEvidence::None,
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
                geom_brep::RadiusEvidence::None,
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

    // -----------------------------------------------------------------
    // The cylinder×sphere arm
    // -----------------------------------------------------------------

    /// The same rigid map the geom-brep section rows use, off every
    /// axis plane, applied to BOTH operands: the configuration is
    /// unchanged and only its pose is.
    fn twin(s: &geom::Surface<f64>) -> geom::Surface<f64> {
        let m = geom_core::Affine3::rotation_about_axis(
            Point3::new(0.3, -0.2, 0.7),
            Vec3::new(1.0, 2.0, 3.0).normalize(),
            0.7,
        ) * geom_core::Affine3::translation(Vec3::new(0.11, 0.23, -0.37));
        match *s {
            geom::Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => geom::Surface::Cylinder {
                origin: m.transform_point(origin),
                axis: m.transform_vec(axis),
                radius,
                u_ref: m.transform_vec(u_ref),
            },
            geom::Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => geom::Surface::Sphere {
                center: m.transform_point(center),
                radius,
                axis: m.transform_vec(axis),
                u_ref: m.transform_vec(u_ref),
            },
            _ => panic!("the coaxial fixture is a cylinder and a sphere"),
        }
    }

    fn ball(center: Point3<f64>, radius: f64) -> geom::Surface<f64> {
        geom::Surface::Sphere {
            center,
            radius,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
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
            Err(FrameError::Escalated(_)) => "an escalation",
            Err(FrameError::RadiusEscalated { .. }) => "a radius escalation",
            Err(FrameError::IntersectingCylinderAxes { .. }) => "the cylinder pinch",
        }
    }

    /// **ONE frame serves BOTH section circles**, measured rather than
    /// argued: the rotational sense the facing test reads,
    /// `axis·((p−c)×dir)`, is identical whether it is taken about the
    /// frame this dispatch returns or about either circle's OWN centre.
    /// That invariance is what makes a single frame correct here and is
    /// exactly what a crossing cylinder pair lacks — its two ellipses
    /// have two different axes, which is why the pinch door exists
    /// there and nothing like it is needed here.
    #[test]
    fn one_frame_serves_both_coaxial_cylinder_sphere_circles() {
        for (label, cyl, sph) in [
            (
                "direct",
                cylinder(Vec3::new(0.0, 0.0, 1.0)),
                ball(Point3::new(0.0, 0.0, 0.0), 2.0),
            ),
            (
                "re-posed twin",
                twin(&cylinder(Vec3::new(0.0, 0.0, 1.0))),
                twin(&ball(Point3::new(0.0, 0.0, 0.0), 2.0)),
            ),
        ] {
            let got = cs_pair_frame(&cyl, &sph, geom_brep::CoaxialEvidence::Declared, band());
            let Ok(Some((c, axis))) = got else {
                panic!(
                    "{label}: the declared coaxial pose must name a frame, got {}",
                    outcome(&got)
                );
            };
            let station = 3.0_f64.sqrt(); // sqrt((2-1)(2+1))
            // A frame-independent radial direction across the axis.
            let u = if axis.cross(Vec3::new(1.0, 0.0, 0.0)).norm() > 0.5 {
                axis.cross(Vec3::new(1.0, 0.0, 0.0)).normalize()
            } else {
                axis.cross(Vec3::new(0.0, 1.0, 0.0)).normalize()
            };
            let v = axis.cross(u);
            for st in [station, -station] {
                let own = c + axis * st;
                for i in 0..8 {
                    let t = f64::from(i) / 8.0 * core::f64::consts::TAU;
                    let p = own + u * t.cos() + v * t.sin();
                    // The circle's own unit tangent at p.
                    let dir = u * -t.sin() + v * t.cos();
                    let shared = axis.dot((p - c).cross(dir));
                    let per_circle = axis.dot((p - own).cross(dir));
                    assert!(
                        (shared - per_circle).abs() < 1e-12,
                        "{label}: station {st}, t {t}: the sense moved with the centre \
                         ({shared} vs {per_circle})"
                    );
                    // And it is a real, definite sense — not zero on
                    // both sides, which would make the row vacuous.
                    assert!(
                        shared.abs() > 0.5,
                        "{label}: sense {shared} is not definite"
                    );
                }
            }
        }
    }

    /// **The declaration is the whole gate.** The SAME exactly-coaxial
    /// pose answers with a frame under `Declared` and refuses `NoArm`
    /// under `None` — the never-infer rule, at this door. `NoArm` here
    /// is a routing (the general rung marches the pair), never a
    /// desync, and the dispatch's own arm passes `None` today because
    /// it is keyed on KINDS and has nowhere to read a ladder.
    #[test]
    fn the_coaxial_frame_needs_the_declaration_and_never_infers_it() {
        for (label, cyl, sph) in [
            (
                "direct",
                cylinder(Vec3::new(0.0, 0.0, 1.0)),
                ball(Point3::new(0.0, 0.0, 0.0), 2.0),
            ),
            (
                "re-posed twin",
                twin(&cylinder(Vec3::new(0.0, 0.0, 1.0))),
                twin(&ball(Point3::new(0.0, 0.0, 0.0), 2.0)),
            ),
        ] {
            assert!(
                matches!(
                    cs_pair_frame(&cyl, &sph, geom_brep::CoaxialEvidence::Declared, band()),
                    Ok(Some(_))
                ),
                "{label}: declared"
            );
            assert!(
                matches!(
                    cs_pair_frame(&cyl, &sph, geom_brep::CoaxialEvidence::None, band()),
                    Err(FrameError::NoArm)
                ),
                "{label}: undeclared"
            );
            // And that is what the kind dispatch itself does today, in
            // both operand orders.
            assert!(
                matches!(
                    pair_section_frame(
                        &cyl,
                        &sph,
                        geom_brep::RadiusEvidence::None,
                        at(),
                        None,
                        band()
                    ),
                    Err(FrameError::NoArm)
                ),
                "{label}: dispatch, cylinder first"
            );
            assert!(
                matches!(
                    pair_section_frame(
                        &sph,
                        &cyl,
                        geom_brep::RadiusEvidence::None,
                        at(),
                        None,
                        band()
                    ),
                    Err(FrameError::NoArm)
                ),
                "{label}: dispatch, sphere first"
            );
        }
    }

    /// A germ was minted from a crossing, so a section that is a
    /// TANGENCY, an empty gap, or a contradicted declaration is a
    /// lockstep failure — loud, typed, and never `None`. Each is a
    /// distinct cause and each gets the desync door, exactly as the
    /// two sphere arms above treat their non-locus outcomes.
    #[test]
    fn a_cylinder_sphere_pose_that_is_not_a_locus_is_a_desync() {
        let z = Vec3::new(0.0, 0.0, 1.0);
        for (row, sph) in [
            // R = r: the tangent circle, classification data.
            ("tangent", ball(Point3::new(0.0, 0.0, 0.0), 1.0)),
            // R < r: the sphere never reaches the wall.
            ("empty", ball(Point3::new(0.0, 0.0, 0.0), 0.5)),
            // Declared coaxial, definitely off the axis: the
            // declaration is verified and contradicted.
            ("off-axis", ball(Point3::new(0.4, 0.0, 0.0), 2.0)),
        ] {
            for (label, cyl, s) in [
                ("direct", cylinder(z), sph.clone()),
                ("re-posed twin", twin(&cylinder(z)), twin(&sph)),
            ] {
                let got = cs_pair_frame(&cyl, &s, geom_brep::CoaxialEvidence::Declared, band());
                assert!(
                    matches!(got, Err(FrameError::Desync(_))),
                    "{row} / {label}: expected a desync, got {}",
                    outcome(&got)
                );
            }
        }
    }

    /// An ill-conditioned declared pair escalates through this door
    /// rather than picking a branch — the same plumbing the sphere
    /// arms use, on this arm.
    #[test]
    fn an_ill_conditioned_declared_coaxial_pair_escalates() {
        let eps = Tol::witness().get().eps;
        let z = Vec3::new(0.0, 0.0, 1.0);
        for (label, cyl, sph) in [
            (
                "direct",
                cylinder(z),
                ball(Point3::new(0.0, 0.0, 0.0), 1.0 + 3.0 * eps),
            ),
            (
                "re-posed twin",
                twin(&cylinder(z)),
                twin(&ball(Point3::new(0.0, 0.0, 0.0), 1.0 + 3.0 * eps)),
            ),
        ] {
            let got = cs_pair_frame(&cyl, &sph, geom_brep::CoaxialEvidence::Declared, band());
            // The PREDICATE is pinned, not merely the variant: the
            // section arm this door forwards from has four numeric
            // rows, every one of which escalates through
            // `FrameError::Escalated`, so the variant alone would not
            // say which one this fixture is ill-conditioned at.
            let Err(FrameError::Escalated(diag)) = got else {
                panic!("{label}: expected an escalation, got {}", outcome(&got));
            };
            assert_eq!(diag.predicate, Some("cs_wall_reach"), "{label}");
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

    use super::{FrameError, pair_section_frame};

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
    /// interval and the split answers `Zero` — the pinch door. Read as
    /// an equality, not "pinch or escalates": an escalation here would
    /// say the enclosure decided the lane rather than the geometry.
    ///
    /// Run at equal AND unequal radii, because the margin reads no
    /// radius and the certified lane must not start inferring one.
    #[test]
    fn meeting_axes_take_the_pinch_door_at_the_certified_scalar() {
        for r in [1.0_f64, 0.5] {
            let got = pair_section_frame(
                &about_z(p3(0.0, 0.0, 0.0), 1.0),
                &about_x(p3(0.0, 0.0, 0.0), r),
                geom_brep::RadiusEvidence::None,
                at(),
                None,
                band(),
            );
            assert!(
                matches!(got, Err(FrameError::IntersectingCylinderAxes { .. })),
                "r = {r}: meeting axes take the pinch door at the certified scalar"
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
                geom_brep::RadiusEvidence::None,
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
        // those poses must still answer the pinch door. A gap measured
        // as any distance between the origins would flip these to
        // `NoArm`.
        for along in [p3(0.0, 0.0, 0.75), p3(0.75, 0.0, 0.0)] {
            let got = pair_section_frame(
                &about_z(p3(0.0, 0.0, 0.0), 1.0),
                &about_x(along, 1.0),
                geom_brep::RadiusEvidence::None,
                at(),
                None,
                band(),
            );
            assert!(
                matches!(got, Err(FrameError::IntersectingCylinderAxes { .. })),
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
                    geom_brep::RadiusEvidence::None,
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

/// **The transverse cylinder × sphere frame** ([`cs_transverse_frame`]),
/// held to what the facing test and the turn ranking read of a frame:
/// along each loop of the analytic section the rotational sense
/// `axis·((p − c) × t)` keeps one definite sign, and the loop turns once
/// about the axis.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod transverse_cs_frame_rows {
    use core::f64::consts::{PI, TAU};

    use geom_core::{Affine3, Point3, Tol, Vec3};

    use super::{FrameError, pair_section_frame};

    /// The point the rows read the frame's axis rows at.
    fn at() -> Point3<f64> {
        Point3::new(0.0, 0.0, 0.0)
    }

    fn band() -> geom_core::Band {
        geom_core::Band::linear(Tol::witness()).expect("a linear band")
    }

    /// A cylinder of radius `r` about the axis through `foot` along `a`
    /// (unit), and a sphere of radius `big` centred `d` off that axis
    /// along `e1` (unit, perpendicular to `a`) from `foot`.
    #[derive(Clone, Copy)]
    struct Pose {
        foot: Point3<f64>,
        a: Vec3<f64>,
        e1: Vec3<f64>,
        r: f64,
        d: f64,
        big: f64,
    }

    impl Pose {
        fn at(r: f64, d: f64, big: f64) -> Self {
            Self {
                foot: Point3::new(0.0, 0.0, 0.0),
                a: Vec3::new(0.0, 0.0, 1.0),
                e1: Vec3::new(1.0, 0.0, 0.0),
                r,
                d,
                big,
            }
        }

        /// The same pose moved by `m`, the cylinder's stored origin slid
        /// off the foot along the axis.
        fn moved(self, m: &Affine3<f64>) -> Self {
            Self {
                foot: m.transform_point(self.foot),
                a: m.transform_vec(self.a),
                e1: m.transform_vec(self.e1),
                ..self
            }
        }

        fn center(self) -> Point3<f64> {
            self.foot + self.e1 * self.d
        }

        fn surfaces(self) -> (geom::Surface<f64>, geom::Surface<f64>) {
            (
                geom::Surface::Cylinder {
                    origin: self.foot + self.a * 0.37,
                    axis: self.a,
                    radius: self.r,
                    u_ref: self.e1,
                },
                geom::Surface::Sphere {
                    center: self.center(),
                    radius: self.big,
                    axis: self.a,
                    u_ref: self.e1,
                },
            )
        }

        /// The section's loops, each a closed run of `(point, tangent)`
        /// samples in its traversal order, read off the cylinder's chart:
        /// a wall point at azimuth `θ` from `e1` lies on the sphere at
        /// heights `±h(θ)`, `h² = R² − r² − d² + 2rd·cos θ`.
        fn loops(self) -> Vec<Vec<(Point3<f64>, Vec3<f64>)>> {
            let e2 = self.a.cross(self.e1);
            let (r, d) = (self.r, self.d);
            let h2 = |t: f64| self.big * self.big - r * r - d * d + 2.0 * r * d * t.cos();
            let at = |t: f64, up: f64| {
                let h = h2(t).sqrt();
                let p = self.foot + (self.e1 * t.cos() + e2 * t.sin()) * r + self.a * (up * h);
                let dh = -r * d * t.sin() / h;
                let tan = (e2 * t.cos() - self.e1 * t.sin()) * r + self.a * (up * dh);
                (p, tan * up)
            };
            let n = 400;
            if h2(PI) > 0.0 {
                // Two loops, each over the whole circle.
                [1.0, -1.0]
                    .into_iter()
                    .map(|up| {
                        (0..n)
                            .map(|k| at(TAU * f64::from(k) / f64::from(n), up))
                            .collect()
                    })
                    .collect()
            } else {
                // One loop over `|θ| < θ₀`, up one side and down the other,
                // its samples kept off the azimuths where `h` vanishes.
                let theta0 = ((r * r + d * d - self.big * self.big) / (2.0 * r * d)).acos();
                let span = |k: i32| -theta0 + 2.0 * theta0 * (f64::from(k) + 0.5) / f64::from(n);
                let up = (0..n).map(|k| at(span(k), 1.0));
                let down = (0..n).rev().map(|k| at(span(k), -1.0));
                vec![up.chain(down).collect()]
            }
        }
    }

    fn poses() -> Vec<(&'static str, Pose)> {
        let twin = Affine3::rotation_about_axis(
            Point3::new(0.3, -0.2, 0.7),
            Vec3::new(1.0, 2.0, 3.0).normalize(),
            0.7,
        ) * Affine3::translation(Vec3::new(0.11, 0.23, -0.37));
        let base = [
            ("one loop, centre on the wall", Pose::at(0.5, 0.5, 0.3)),
            ("one loop, centre inside", Pose::at(0.5, 0.35, 0.3)),
            ("one loop, centre outside", Pose::at(0.5, 0.7, 0.3)),
            ("one loop, near-coaxial wide", Pose::at(0.5, 0.1, 0.55)),
            ("one loop, nearly a figure-eight", Pose::at(0.5, 0.3, 0.79)),
            ("one loop, far off", Pose::at(0.5, 1.2, 0.9)),
            ("two loops", Pose::at(0.5, 0.1, 0.7)),
            ("two loops, wide", Pose::at(0.5, 0.2, 0.8)),
            ("two loops, nearly a figure-eight", Pose::at(0.5, 0.3, 0.81)),
            ("two loops, big ball", Pose::at(0.5, 0.05, 2.0)),
        ];
        base.iter()
            .copied()
            .chain(base.iter().map(|&(l, p)| (l, p.moved(&twin))))
            .collect()
    }

    /// **Every loop turns once about the frame, its sense never radial.**
    /// The frame is read through the kind dispatch in both operand
    /// orders; along each analytic loop the sense keeps one sign with a
    /// definite size, and the projected radius sweeps exactly one turn.
    #[test]
    fn every_section_loop_turns_once_about_the_transverse_frame() {
        for (label, pose) in poses() {
            let (cyl, sph) = pose.surfaces();
            for (order, got) in [
                (
                    "cylinder first",
                    pair_section_frame(
                        &cyl,
                        &sph,
                        geom_brep::RadiusEvidence::None,
                        at(),
                        None,
                        band(),
                    ),
                ),
                (
                    "sphere first",
                    pair_section_frame(
                        &sph,
                        &cyl,
                        geom_brep::RadiusEvidence::None,
                        at(),
                        None,
                        band(),
                    ),
                ),
            ] {
                let Ok(Some((c, axis))) = got else {
                    panic!("{label}, {order}: a frame");
                };
                for (i, run) in pose.loops().iter().enumerate() {
                    let senses: Vec<f64> = run
                        .iter()
                        .map(|&(p, t)| axis.dot((p - c).cross(t)))
                        .collect();
                    let sign = senses[0].signum();
                    assert!(
                        senses.iter().all(|s| s * sign > 1e-6),
                        "{label}, {order}, loop {i}: the sense changes or vanishes along the \
                         loop (min {:e}, max {:e})",
                        senses.iter().copied().fold(f64::INFINITY, f64::min),
                        senses.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                    );
                    let radial = |p: Point3<f64>| {
                        let u = p - c;
                        u - axis * axis.dot(u)
                    };
                    let mut turn = 0.0;
                    for k in 0..run.len() {
                        let (u, v) = (radial(run[k].0), radial(run[(k + 1) % run.len()].0));
                        turn += axis.dot(u.cross(v)).atan2(u.dot(v));
                    }
                    assert!(
                        (turn.abs() - TAU).abs() < 1e-6,
                        "{label}, {order}, loop {i}: the loop turns {turn} about the frame"
                    );
                }
            }
        }
    }

    /// **Which frame**: a one-loop section turns about the axis through
    /// the sphere's centre towards it from the cylinder's axis, and a
    /// two-loop section about the cylinder's own axis.
    #[test]
    fn the_loop_count_picks_the_frame() {
        for (label, pose) in poses() {
            let (cyl, sph) = pose.surfaces();
            let Ok(Some((c, axis))) = pair_section_frame(
                &cyl,
                &sph,
                geom_brep::RadiusEvidence::None,
                at(),
                None,
                band(),
            ) else {
                panic!("{label}: a frame");
            };
            let two = pose.big > pose.r + pose.d;
            let (want_axis, on) = if two {
                (pose.a, pose.foot)
            } else {
                (pose.e1, pose.center())
            };
            assert!(
                axis.cross(want_axis).norm() < 1e-12 && (axis.norm() - 1.0).abs() < 1e-12,
                "{label}: axis {axis:?}"
            );
            let off = c - on;
            assert!(
                off.cross(want_axis).norm() < 1e-12,
                "{label}: centre {c:?} is off the frame's axis line"
            );
        }
    }

    /// **The poses with no frame keep `NoArm`; an undecided reach
    /// escalates.** The walls touching at `θ = π` (`R = r + d`, the
    /// figure-eight's node) and a coaxial pose, decided or in the band,
    /// keep the dispatch's `NoArm`; a reach margin in the band escalates
    /// under `bool_germ_frame_cs_reach`.
    #[test]
    fn a_tangency_or_a_coaxial_pose_keeps_no_arm() {
        let eps = Tol::witness().get().eps;
        for (label, pose, want) in [
            ("node", Pose::at(0.5, 0.25, 0.75), "NoArm"),
            ("coaxial", Pose::at(0.5, 0.0, 0.75), "NoArm"),
            ("offset in band", Pose::at(0.5, 4.0 * eps, 0.75), "NoArm"),
            (
                "reach in band",
                Pose::at(0.5, 0.25, 0.75 + 4.0 * eps),
                "bool_germ_frame_cs_reach",
            ),
        ] {
            let (cyl, sph) = pose.surfaces();
            let got = pair_section_frame(
                &cyl,
                &sph,
                geom_brep::RadiusEvidence::None,
                at(),
                None,
                band(),
            );
            let read = match got {
                Err(FrameError::NoArm) => "NoArm",
                Err(FrameError::Escalated(diag)) => diag.predicate.unwrap_or("unnamed"),
                _ => "another answer",
            };
            assert_eq!(read, want, "{label}");
        }
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
                let (p, n) = parallel_radical_plane(own, partner, band())
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
        let spec = |tangent: bool| {
            let (s1, s2) = Default::default();
            let witness = Point3::new(0.0, 0.0, 0.5);
            geom_brep::EdgeCurveSpec {
                description: if tangent {
                    geom_brep::EdgeDescriptionSpec::TangentIntersection { s1, s2, witness }
                } else {
                    geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, witness }
                },
                carrier: line.clone(),
                param_start: 0.0,
                param_end: 1.0,
            }
        };
        let curve = |spec| SegmentCurve::of((he(1), he(2)), spec);
        let straight = curve(None);
        let ask = |a: &SegmentCurve<f64>, b: &SegmentCurve<f64>| {
            rulings_are_straight([(a, Operand::A, fa), (b, Operand::B, fb)])
        };
        assert!(ask(&straight, &straight).is_ok(), "two rulings pass");
        let touching = curve(Some(spec(true)));
        for (label, got, operand, wall) in [
            ("A touches", ask(&touching, &straight), Operand::A, fa),
            ("B touches", ask(&straight, &touching), Operand::B, fb),
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
        let conic = curve(Some(spec(false)));
        assert!(
            matches!(ask(&straight, &conic), Err(BooleanError::JoinDesync { .. })),
            "a conic is a desync"
        );
    }

    /// **Coaxial walls name no plane; an offset in the band escalates
    /// under its own name.**
    #[test]
    fn a_coaxial_pair_has_no_plane_and_an_offset_in_band_escalates() {
        let eps = Tol::witness().get().eps;
        let z = Vec3::new(0.0, 0.0, 1.0);
        let at = |x: f64| Point3::new(x, 0.0, 0.0);
        for (label, offset) in [("coaxial", 0.0), ("in the zero band", eps / 4.0)] {
            let got =
                parallel_radical_plane(&wall(at(0.0), z, 0.5), &wall(at(offset), z, 0.3), band());
            assert!(
                matches!(got, Ok(None)),
                "{label}: {:?}",
                got.map(|p| p.is_some())
            );
        }
        let got =
            parallel_radical_plane(&wall(at(0.0), z, 0.5), &wall(at(4.0 * eps), z, 0.3), band());
        match got {
            Err(BooleanError::Escalated {
                decision: BooleanDecision::Coincidence(Coincide::Section, DeclarationRead::Moot),
                diag,
            }) => assert_eq!(diag.predicate, Some(BOOL_JOIN_CC_AXIS_OFFSET)),
            other => panic!("in band: {:?}", other.map(|p| p.is_some())),
        }
    }
}
