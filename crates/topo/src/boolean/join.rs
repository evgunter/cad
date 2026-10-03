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
//! ([`SegmentEdge`]).
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
//!    itself (`insert::insert_null_pairs`' `record_dir` takes the A
//!    flanker's bound read On there, and `fn germ_dir`'s normal cross
//!    everywhere else), and the loops' direction along it is the region
//!    boundaries' as above, read on that edge.
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
//!    The angular strut spike order (`insert::strut_order`) ranks a
//!    strut's two germs by their angle from the splice corner's
//!    arrival edge, measured inside the sector at any width and read
//!    as distances at the sector's arm; where nothing orders them it
//!    refuses.
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
//! germs, which changes what is nearest for the rest). The criterion
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
use crate::chord_join::{ChordJoiner, CutOutcome, SegmentEdge, SplitJoinError};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::EulerOpError;
use crate::face_normal::face_outward_normal;
use crate::loop_winding::TornLoop;
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
    /// The radical plane of a sphere pair: this body's sphere surface
    /// and the other body's.
    Radical {
        own: crate::geometry::SurfaceKey,
        partner: crate::geometry::SurfaceKey,
    },
}

impl SolidJoin {
    /// The wall-side chord lane against the germ plane through `origin`
    /// with chart normal `normal`: one chord through
    /// [`JoinLane::Split`], its aux plane read from and minted into
    /// [`Self::aux`] under `datum`.
    fn join_split<T: Decide + crate::props::AtRestPolicy>(
        &mut self,
        body: &mut Body<T>,
        (h1, h2): (HalfEdgeKey, HalfEdgeKey),
        (origin, normal): (Point3<T>, UnitVec3<T>),
        datum: AuxDatum,
        segment: SegmentEdge,
        tol: Tol,
    ) -> Result<(), BooleanError> {
        let mut ctx = crate::chord_join::SectionCtx {
            origin,
            normal,
            plane_key: self.aux.get(&datum).copied(),
        };
        self.joiner
            .join(
                body,
                h1,
                h2,
                crate::chord_join::JoinLane::Split(&mut ctx),
                segment,
                tol,
            )
            .map_err(BooleanError::Join)?;
        if let Some(k) = ctx.plane_key {
            self.aux.insert(datum, k);
        }
        Ok(())
    }

    /// The planar-side chord lane against the partner `wall`, whose
    /// face's azimuth window is `window`: one chord through
    /// [`JoinLane::BoolPlanar`], the wall copy keyed by `partner_face`.
    #[allow(clippy::too_many_arguments)]
    fn join_bool_planar<T: Decide + crate::props::AtRestPolicy>(
        &mut self,
        body: &mut Body<T>,
        (h1, h2): (HalfEdgeKey, HalfEdgeKey),
        wall: geom::Surface<T>,
        window: (T, T),
        partner_face: FaceKey,
        segment: SegmentEdge,
        tol: Tol,
    ) -> Result<(), BooleanError> {
        let datum = AuxDatum::Partner(partner_face);
        let mut partner = self.aux.get(&datum).copied();
        self.joiner
            .join(
                body,
                h1,
                h2,
                crate::chord_join::JoinLane::BoolPlanar {
                    wall,
                    window,
                    partner_key: &mut partner,
                },
                segment,
                tol,
            )
            .map_err(BooleanError::Join)?;
        if let Some(k) = partner {
            self.aux.insert(datum, k);
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

/// `bool_connect`'s product: the completed pairs plus the per-operand
/// chord-mef fragment logs (naming emission, M4 PR 3 — `(new face,
/// divided-from face)` at call-time CLONE keys, A rows in the A-clone
/// arena, B rows in the B-clone arena pre-graft).
pub(super) struct Connected {
    pub completed: Vec<CompletedPolygonPair>,
    pub a_fragments: Vec<(FaceKey, FaceKey)>,
    pub b_fragments: Vec<(FaceKey, FaceKey)>,
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
        // Role order per solid, derived independently (module docs —
        // the PR 5.5 discipline): cross-solid seam orientation is
        // carried by the sense attributes alone; role order only
        // decides the face partition of a same-loop split, which each
        // solid resolves against its OWN geometry.
        let (a1, a2) = choose_roles(&red.a, ea, ra, &a_loose, band)?;
        let (b1, b2) = choose_roles(&red.b, eb, rb, &b_loose, band)?;
        // Curved germ pairs (M5 PR 9): each solid's chord lane comes
        // from the germ FACE PAIR — plane×plane takes the straight-chord
        // lane with the partner's plane as its section; plane×cylinder mints
        // the C5 section conic on both sides (the wall side through
        // the S9 window machinery with the germ plane as context, the
        // planar side against the wall face's own window, so both
        // solids select the SAME geometric arc); plane×sphere (M5
        // S13) rides the same two lanes with the exact C5 Circle and
        // the sphere chart's azimuth window (a section tilted against
        // that chart refuses on the planar side); a sphere pair rides the
        // wall-side lane on both sides against its radical plane; any
        // other pair refuses typed citing its C5 routing (per-arm,
        // C12.1).
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
        // point set (and hence the section conic, its azimuth window,
        // and the auxiliary surface minted for it) is identical under
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
        let segment = |locus| {
            SegmentEdge::Is(match locus {
                super::Locus::OnEdge(e) => Some(e),
                super::Locus::InFace(_) => None,
            })
        };
        let (seg_a, seg_b) = (segment(germ.a_locus), segment(germ.b_locus));
        use crate::chord_join::{JoinLane, face_azimuth_window};
        use geom::Surface as Sf;
        if seg.lane == SegmentLane::AlongEdge {
            sa.joiner
                .join(&mut red.a, a1, a2, JoinLane::AlongEdge, seg_a, tol)
                .map_err(BooleanError::Join)?;
            sb.joiner
                .join(&mut red.b, b1, b2, JoinLane::AlongEdge, seg_b, tol)
                .map_err(BooleanError::Join)?;
        } else {
            match (&ga, &gb) {
                (Sf::Plane { .. }, Sf::Plane { .. }) => {
                    // The chord runs along the two planes' common line:
                    // straight on both sides.
                    sa.joiner
                        .join(&mut red.a, a1, a2, JoinLane::Planar, seg_a, tol)
                        .map_err(BooleanError::Join)?;
                    sb.joiner
                        .join(&mut red.b, b1, b2, JoinLane::Planar, seg_b, tol)
                        .map_err(BooleanError::Join)?;
                }
                (Sf::Plane { origin, normal, .. }, Sf::Sphere { .. })
                | (Sf::Plane { origin, normal, .. }, Sf::Cylinder { .. }) => {
                    let reach = germ_reach(&red.a)?;
                    let window = face_azimuth_window(&red.b, &gb, germ.b_face, band)
                        .map_err(BooleanError::Join)?
                        .ok_or(desync("wall germ face has no charted azimuth window"))?;
                    sa.join_bool_planar(
                        &mut red.a,
                        (a1, a2),
                        gb.clone(),
                        window,
                        germ.b_face,
                        seg_a,
                        tol,
                    )?;
                    let plane = (*origin, germ_normal(reach, *origin, *normal)?);
                    sb.join_split(
                        &mut red.b,
                        (b1, b2),
                        plane,
                        AuxDatum::Partner(germ.a_face),
                        seg_b,
                        tol,
                    )?;
                }
                (Sf::Sphere { .. }, Sf::Plane { origin, normal, .. })
                | (Sf::Cylinder { .. }, Sf::Plane { origin, normal, .. }) => {
                    let plane = (*origin, germ_normal(germ_reach(&red.a)?, *origin, *normal)?);
                    sa.join_split(
                        &mut red.a,
                        (a1, a2),
                        plane,
                        AuxDatum::Partner(germ.b_face),
                        seg_a,
                        tol,
                    )?;
                    let window = face_azimuth_window(&red.a, &ga, germ.a_face, band)
                        .map_err(BooleanError::Join)?
                        .ok_or(desync("wall germ face has no charted azimuth window"))?;
                    sb.join_bool_planar(
                        &mut red.b,
                        (b1, b2),
                        ga.clone(),
                        window,
                        germ.a_face,
                        seg_b,
                        tol,
                    )?;
                }
                // **The sphere pair rides its RADICAL PLANE.** Two spheres
                // meet in a circle lying in the one plane both residuals
                // agree on, so on each side the section is that sphere cut
                // by that plane: the wall-side chord lane of the plane×sphere
                // pair above, run on BOTH sides against the same plane. The
                // plane is computed from the pair's own C5 Circle, once per
                // germ, so the two sides' chords are sections of one datum;
                // each body's aux copy of it is keyed by the two spheres it
                // depends on ([`AuxDatum::Radical`]). A radical plane tilted
                // against a chart's polar axis takes the run-side arc rule on
                // that side (`chord_join::select_arc_by_run_side`).
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
                    let datum = |own, partner| AuxDatum::Radical { own, partner };
                    sa.join_split(&mut red.a, (a1, a2), radical, datum(ka, kb), seg_a, tol)?;
                    sb.join_split(&mut red.b, (b1, b2), radical, datum(kb, ka), seg_b, tol)?;
                }
                (a_s, b_s) => {
                    // No wired join arm for this germ pair (cyl×cyl's
                    // equal-radius ellipse pair, cyl×sphere's rung-3
                    // fitted chords, plane×NURBS behind PR 7b): typed,
                    // citing the kind whose join arm is missing.
                    let (operand, face, s) = if matches!(a_s, Sf::Plane { .. }) {
                        (Operand::B, germ.b_face, b_s)
                    } else {
                        (Operand::A, germ.a_face, a_s)
                    };
                    return Err(BooleanError::CurvedBooleanUnsupported {
                        operand,
                        face,
                        kind: s.kind(),
                    });
                }
            }
        }
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
            cut_pair(red, &mut sa, &mut sb, &mut completed, r.a_edge, r.b_edge)?;
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
    Ok(Connected {
        completed,
        a_fragments: sa.joiner.take_fragments(),
        b_fragments: sb.joiner.take_fragments(),
    })
}

/// `scanjoin`, germ form (module docs): among all candidate/entry slot
/// combinations whose A-side germs carry the SAME loci and whose
/// two sites mutually FACE each other along the germ line
/// (`bool_join_facing`, decided — the polygon edge's ends point at one
/// another), with OPPOSED senses in both solids (the sense theorem's
/// neighbor test), pick the NEAREST pair of sites (`bool_join_nearest`,
/// decided — non-adjacent same-line sites must not be chorded across
/// an intermediate one). The B side consumes the SAME slots — slot `i`
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
    let escalate = |diag| BooleanError::coincidence(Coincide::Join, DeclarationRead::Moot, diag);
    let mut best: Option<(T, (Slot, Slot))> = None;
    for (cand, rec) in open.iter().enumerate() {
        for (entry, e) in open.iter().enumerate() {
            if entry == cand {
                continue;
            }
            for &cs in &slots(&rec.a) {
                for &es in &slots(&e.a) {
                    let Some(dist) = partners(open, red, sa, sb, (cand, cs), (entry, es), band)?
                    else {
                        continue;
                    };
                    let m = ((entry, es), (cand, cs));
                    best = match best {
                        None => Some((dist, m)),
                        Some((bd, bm)) => {
                            match decide("bool_join_nearest", Margin::of(dist - bd), band)
                                .map_err(escalate)?
                            {
                                Sign::Negative => Some((dist, m)),
                                _ => Some((bd, bm)),
                            }
                        }
                    };
                }
            }
        }
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
/// or the join has desynced. `Some(chord length)` when they are
/// partners. Two slots of one record never are.
fn partners<T: Decide>(
    open: &[OpenRecord<T>],
    red: &BooleanReduction<T>,
    sa: &Sides,
    sb: &Sides,
    (cand, cs): (usize, usize),
    (entry, es): (usize, usize),
    band: Band,
) -> Result<Option<T>, BooleanError> {
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
    Ok(Some(dist))
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
    );
    let e = body
        .get_edge(edge)
        .ok_or(desync("an OnEdge germ's edge no longer resolves"))?;
    if site.contains(&start(e.he_plus)?) || site.contains(&start(e.he_minus)?) {
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
    pair_section_frame(&sa, &sb, evidence, band)
        .map_err(|e| frame_refusal(e, (germ.a_face, &sa), (germ.b_face, &sb)))
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
    /// The kind pair has no section arm at all.
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
#[allow(clippy::type_complexity)] // (conic center, conic axis) — one frame tuple
pub(super) fn pair_section_frame<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    evidence: geom_brep::RadiusEvidence,
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
        // **Cylinder×sphere.** The DECLARED-coaxial configuration is
        // the one this dispatch can name a frame for, and
        // [`cs_pair_frame`] carries the whole argument — including why
        // ONE frame serves BOTH section circles, and why the
        // declaration cannot be read here yet.
        (Sf::Cylinder { .. }, Sf::Sphere { .. }) => {
            return cs_pair_frame(sa, sb, geom_brep::CoaxialEvidence::None, band);
        }
        (Sf::Sphere { .. }, Sf::Cylinder { .. }) => {
            return cs_pair_frame(sb, sa, geom_brep::CoaxialEvidence::None, band);
        }
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
        // Metered at the larger radius: a bigger lever makes the
        // parallelism margin harder to call Zero, so the error runs
        // toward the refusal, never toward a wrong straight chord.
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
            match decide(
                "bool_germ_frame_axes_parallel",
                Margin::levered(a1.cross(*a2).norm(), r1.max(*r2)),
                band,
            ) {
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
            // **The `/ cross.norm()` is a UNITS correction, and no test
            // can red on it away from the band — said out loud so its
            // absence from the suite reads as a measurement rather than
            // a gap.** `‖a1×a2‖ = sin θ` for unit axes, so dividing is
            // what makes the margin the true axis-to-axis LENGTH the
            // band is denominated in; dropping it scales a definite
            // margin by `sin θ` and can only change a verdict within a
            // factor `sin θ` of the band itself. Reaching that needs an
            // almost-parallel pair, which the gate above answers `Zero`
            // first. Measured: with the division dropped, every row of
            // `frame_dispatch_tests` and its interval twin still
            // greens. What the rows DO pin is the direction — see
            // `skew_axes_keep_the_general_rung_at_the_certified_scalar`,
            // which reds the moment the gap is measured along an axis
            // instead of along `a1×a2`.
            let w0 = *o2 - *o1;
            let cross = a1.cross(*a2);
            return match decide(
                "bool_germ_frame_axes_coplanar",
                Margin::of(w0.dot(cross) / cross.norm()),
                band,
            ) {
                Ok(Sign::Zero) => Err(intersecting_cylinder_axes(sa, sb, evidence, *r1, *r2, band)),
                Ok(Sign::Positive | Sign::Negative) => Err(FrameError::NoArm),
                Err(diag) => Err(FrameError::Escalated(diag)),
            };
        }
        _ => return Err(FrameError::NoArm),
    };
    match geom_brep::plane_cylinder_section(plane_s, cyl_s, radius, band) {
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
    r1: T,
    r2: T,
    band: Band,
) -> FrameError {
    if evidence == geom_brep::RadiusEvidence::None {
        return FrameError::IntersectingCylinderAxes { evidence };
    }
    // `r1`/`r2` are NEVER compared here — equality is the channel's
    // answer, not this function's — they only set the section
    // table's EXTENT: metered at the larger radius, the same lever the
    // parallelism gate above used, because the table re-decides the
    // two axis margins this dispatch just decided and must reach the
    // same verdicts from the same margins.
    //
    // Both matches are CLOSED (VERB-SEAT-DESIGN §0, D3): every section
    // outcome and every refusal is named, so a variant added to either
    // enum is a compile-time visit here rather than a silent desync.
    match geom_brep::cylinder_cylinder_section(sa, sb, evidence, r1.max(r2), band) {
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
            | geom_brep::SectionError::Carrier(_),
        ) => FrameError::Desync(
            "the declared equal-radius cylinder section refused at the germ pair \
             with a refusal this pair cannot produce",
        ),
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
/// general rung, and the pose keeps [`FrameError::NoArm`] VERBATIM —
/// which is the honest answer, because the general rung for this pair
/// IS implemented and marches it. Coaxiality is placement data (an
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
        Some((center, axis)) => {
            let s1 = axis.dot((p1 - center).cross(g1.dir));
            let s2 = axis.dot((p2 - center).cross(g2.dir));
            // A zero sense is malformed germ data, so its in-band twin is
            // the kernel's too.
            let malformed = |diag| BooleanError::Escalated {
                decision: BooleanDecision::SelfCheck(SelfCheck::ArcFacing),
                diag,
            };
            let d1 = decide("bool_join_arc_facing", Margin::of(s1), band).map_err(malformed)?;
            let d2 = decide("bool_join_arc_facing", Margin::of(s2), band).map_err(malformed)?;
            match (d1, d2) {
                (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Ok(true),
                (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Ok(false),
                (Sign::Zero, _) | (_, Sign::Zero) => Err(BooleanError::JoinDesync {
                    what: "a conic germ has no rotational sense (radial germ direction — \
                           malformed germ data)",
                }),
            }
        }
    }
}

type LooseMap = SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>;

/// Still-unused null-edge halves, each mapped to its geometric MATCH
/// PARTNER's half in the same solid: the nearest loose germ that is its
/// partner by [`partners`] — [`find_match`]'s own criterion,
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
    let escalate = |diag| BooleanError::coincidence(Coincide::Join, DeclarationRead::Moot, diag);
    let loose: Vec<(usize, usize)> = open
        .iter()
        .enumerate()
        .flat_map(|(i, r)| (0..2).filter(move |&s| !r.a[s].1).map(move |s| (i, s)))
        .collect();
    let mut a_map: LooseMap = SecondaryMap::new();
    let mut b_map: LooseMap = SecondaryMap::new();
    for &(i, s) in &loose {
        let mut best: Option<(T, (usize, usize))> = None;
        for &(j, t) in &loose {
            // The matcher's own criterion ([`partners`]): the separation
            // constraint must count partners with the matcher's eyes or
            // roles get walled off wrongly.
            let Some(dist) = partners(open, red, sa, sb, (i, s), (j, t), band)? else {
                continue;
            };
            best = match best {
                None => Some((dist, (j, t))),
                Some((bd, bm)) => {
                    match decide("bool_join_nearest", Margin::of(dist - bd), band)
                        .map_err(escalate)?
                    {
                        Sign::Negative => Some((dist, (j, t))),
                        _ => Some((bd, bm)),
                    }
                }
            };
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
///   order is orientation-neutral; keep the given order.
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
///   run — the enclosed patch, the new face's outer — must wind CCW
///   around the face's outward normal. Decided intrinsically by
///   [`ring_run_ccw`] (issue #93; supersedes the PR 5.5
///   residual-material-side probe, whose outer-loop vertex anchor was
///   unsound mid-fixpoint on multi-polygon faces — see the lane
///   comment). A derived order whose run separates a loose pair is a
///   loud desync.
///
/// Cross-solid consistency needs NO coupling of the two solids' role
/// orders: the sense attributes carry the seam orientation (the
/// anti-correlation theorem), and each solid's partition is decided
/// against its own geometry — the zip's antiparallelism assertion is
/// the runtime witness.
fn choose_roles<T: Decide>(
    body: &Body<T>,
    ea: HalfEdgeKey,
    ra: HalfEdgeKey,
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
    band: Band,
) -> Result<(HalfEdgeKey, HalfEdgeKey), BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let loop_of = |he: HalfEdgeKey| -> Result<LoopKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or(desync("role half no longer resolves"))?
            .parent_loop)
    };
    let l = loop_of(ea)?;
    if l != loop_of(ra)? {
        return Ok((ea, ra)); // mekr lane
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
            .ok_or(desync("every chord arc separates a loose scaffolding pair"));
    }
    // Ring lane: intrinsic winding (issue #93). The role order is
    // fully determined by the face's own orientation — the mef run
    // (the enclosed patch, the new face's outer) must wind CCW around
    // the face's outward normal so the remainder ring anti-encloses.
    // Exactly one of the two orders satisfies it (the candidate runs
    // are antiparallel copies). This replaces the PR 5.5
    // residual-material-side probe, which anchored on the face's
    // outer-loop vertices and was UNSOUND mid-fixpoint on faces
    // hosting several pending polygons: the outer anchor classified a
    // region other pending seams still separate from the island's
    // immediate surround (the A×Z counter island — surround IN, outer
    // corners OUT — silently crossed the copies; the zip's
    // antiparallelism witness caught it). The two rules agree wherever
    // the residual anchor was sound (both parities checked in the
    // issue #93 diagnosis), so corpus surgery is unchanged.
    let (h1, h2) = if ring_run_ccw(body, face, ea, ra, band)? {
        (ea, ra)
    } else {
        (ra, ea)
    };
    match clean_dir(body, h1, h2, loose)? {
        Some((c1, _)) if c1 == h1 => Ok((h1, h2)),
        _ => Err(desync(
            "derived ring role order separates a loose scaffolding pair",
        )),
    }
}

/// Whether the prospective mef run `[h1 .. h2]` — the `next`-order arc
/// from `h1` through `h2`, closed by the chord `end(h2) → start(h1)`
/// (exactly the cycle the joiner's first `mef(Chords { he1: h1,
/// he2: next(h2, tol) })` walls off as the new face) — winds CCW around
/// `face`'s outward normal: the orientation an island's new outer loop
/// must have (the remainder ring anti-encloses iff the run encloses).
///
/// Reified (issue #93): decided through the `bool_ring_run_winding`
/// predicate by [`Body::planar_run_winding_decided`] — the one home of
/// the sum, its dimension (`2A/P`, audit F4) and its orientation rule,
/// `crate::loop_winding`'s module docs, which the merge's role assigner
/// and `validate`'s tier-3 check 6 read for a stored loop. The normal
/// handed to it is the face's OUTWARD normal, read through
/// [`face_outward_normal`] with the sense folded in; the run's stored
/// traversal carries the other sign. `Indeterminate` escalates. Zero is
/// a degenerate area-free run and a loud desync (the ring lane only
/// closes full island cycles — slit-growing joins are mekr-lane
/// merges). The ring lane is planar-scoped like
/// [`super::solid_contain::point_in_solid`]'s F5 gate: a non-planar
/// face refuses loudly, and so does a spiric or spline run edge, which
/// the operand gate keeps out.
fn ring_run_ccw<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    h1: HalfEdgeKey,
    h2: HalfEdgeKey,
    band: Band,
) -> Result<bool, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let normal = face_outward_normal(body, face)
        .ok_or(desync("ring-lane face has no planar carrier"))?
        .vec();
    let wound = body
        .planar_run_winding_decided(h1, h2, normal, band)
        .map_err(|torn| match torn {
            TornLoop::Dangling(what) => {
                BooleanError::Join(SplitJoinError::Euler(EulerOpError::from(what)))
            }
            TornLoop::Unclaimed { he, edge } => {
                BooleanError::Join(SplitJoinError::Euler(EulerOpError::UnclaimedHalfEdge {
                    he,
                    edge,
                }))
            }
            TornLoop::Unclosed => desync("ring-run arc did not close"),
        })?
        // The operand gate refuses a spiric or spline carrier and no
        // section lane mints one on a plane, so a run carrying one is
        // the gate's invariant broken: the chord joiner's own reading
        // of the same edge.
        .ok_or(BooleanError::Join(SplitJoinError::SectionInvariant {
            face,
            what: "the ring lane reached a spiric or spline run edge (the operand gates refuse \
                   the kinds)",
        }))?;
    // A zero area is a degenerate run (below), so its in-band twin is
    // the kernel's too.
    let decided = wound.map_err(|diag| BooleanError::Escalated {
        decision: BooleanDecision::SelfCheck(SelfCheck::RingWinding),
        diag,
    })?;
    match decided.sign {
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
) -> Result<(), BooleanError> {
    let a_out = sa
        .joiner
        .cut_core(&mut red.a, a_edge)
        .map_err(BooleanError::Join)?;
    let b_out = sb
        .joiner
        .cut_core(&mut red.b, b_edge)
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
fn resolve_roles_geometric<T: Decide>(
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
/// pins the pose; `insert::strut_facing` is the rule it holds).
///
/// **Neither deciding** is the curved-face frontier, refused
/// [`SplitJoinError::SectionLoopUndecided`]: every witness of both
/// loops' regions read the other boundary or too near it, which a
/// crossing's two flanks cannot both do unless their faces are all
/// curved (`work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior`).
/// No in-band reading is named as the cause: it is about one point.
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
        (Reading::Undecided(_), Reading::Undecided(_)) => {
            Err(BooleanError::Join(SplitJoinError::SectionLoopUndecided {
                face,
            }))
        }
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
    use super::super::shell_witness::{Reading, Tally};
    use super::super::{BooleanError, SideCode};
    use super::loop_roles;
    use crate::chord_join::SplitJoinError;
    use crate::entity::{FaceKey, LoopKey};
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
        let err = super::ring_run_ccw(body, prism.top_face, h1, h2, b)
            .expect_err("an in-band winding escalates");
        assert_defect(&err, SelfCheck::RingWinding, "bool_ring_run_winding");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod frame_dispatch_tests {
    use geom_core::{Point3, Tol, Vec3};

    use super::{FrameError, cs_pair_frame, frame_refusal, pair_section_frame};

    fn band() -> geom_core::Band {
        geom_core::Band::linear(Tol::witness()).expect("a linear band")
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
            let got = pair_section_frame(&a, &b, geom_brep::RadiusEvidence::None, band());
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
            let got = pair_section_frame(&a, &b, evidence, band());
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
            let sec = cylinder_cylinder_section(&c1, &c2, RadiusEvidence::Declared, 1.0, band())
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
                pair_section_frame(&plane(), &plane(), geom_brep::RadiusEvidence::None, band()),
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
                    band()
                ),
                Ok(None)
            ),
            "the parallel-lines outcome is a proven straight locus"
        );
        assert!(
            matches!(
                pair_section_frame(&plane(), &sphere(), geom_brep::RadiusEvidence::None, band()),
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
                    pair_section_frame(&cyl, &sph, geom_brep::RadiusEvidence::None, band()),
                    Err(FrameError::NoArm)
                ),
                "{label}: dispatch, cylinder first"
            );
            assert!(
                matches!(
                    pair_section_frame(&sph, &cyl, geom_brep::RadiusEvidence::None, band()),
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
/// — the two-arm pin for this unit's new decide
/// site, `bool_germ_frame_axes_coplanar`.
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
                    band()
                ),
                Ok(None)
            ),
            "parallel cylinder axes meet in rulings at the certified scalar"
        );
    }
}
