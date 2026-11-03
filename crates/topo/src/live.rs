//! [`Live`] — a half-edge key a lookup has returned — and the lookups
//! that say where their key came from: the [`KeySource`] doors, and
//! [`FaceView`], a face resolved once by where its key came from, which
//! every walk from a face (its surface, its loops, its boundary members
//! and its edges) reads.
//!
//! [`Body::link_half_edges`] splices by writing `next`/`prev`, and under
//! the D2 addendum a failed lookup there is a kernel bug announced with
//! `unreachable!`. The obligation to know the key resolves therefore
//! belongs to whoever supplies it — a shared helper knows none of its
//! callers. `Live` is that obligation as a type: private field, own
//! module, and **every door that hands one out performs the lookup**.
//!
//! # What a `Live` claims
//!
//! *This key resolved in this body when the token was made* — not that
//! it resolves now, which no plain value can promise across a `&mut`
//! call. Hence the rule, stated here rather than left to be re-derived:
//! **do not remove half-edges between proving a key and splicing one.**
//! Every mutation phase in this crate obeys it by shape — half-edge
//! removal is the last thing an operator does.
//!
//! Breaking it is loud: the arenas are slotmaps, so a removed key never
//! resolves again and a stale token fails at the splice. What no lookup
//! catches is a token proven against one body and spliced into another,
//! where the key may resolve to an unrelated half-edge — live-but-wrong,
//! which is foreignness and not liveness: the hazard [`crate::body`]'s
//! module docs record as unprotected.
//!
//! # Guarding
//!
//! That no `Live` exists without a lookup rests on this file's privacy,
//! which **the crate's usual instrument cannot check** — a
//! `compile_fail` doctest cannot name a `pub(crate)` type — so it is
//! checked as source instead, by this module's
//! `every_door_that_hands_out_a_live_looks_up_first`: the field and
//! `Live::new` carry no visibility, every door whose return type hands
//! a `Live` out looks the key up before it builds one and wraps the key
//! it looked up, that row's own tables enumerate the doors and the
//! construction sites so a new one of either reds, and no other file in
//! `topo/src` builds a `Live` at all.
//!
//! # Where a key came from
//!
//! Every lookup in a plan phase says where its key came from
//! ([`KeySource`]): an **argument** the caller passed ([`Arg`]), whose
//! miss is the caller's typed refusal ([`BadArgument::Stale`]), or a
//! **link** a record of the body holds ([`Link`]), whose miss is a
//! kernel bug — every public door keeps the body tier-1-valid, so a
//! record's link resolves — and panics naming the record (D2 row 4).
//! The source decides the lookup's type: through an argument it answers
//! `Result`, through a link the record itself, so a link's miss has no
//! `Err` to look like a refusal. [`lookup`] is the one place that
//! decision is made; [`require_key`], [`Body::require_live`] and
//! [`Body::resolve_half_edge_live`] are it for their callers' shapes.
//!
//! # Walks from a face
//!
//! A face's record links its surface and its loops, and every walk from
//! it lives here once, on [`FaceView`]: a face resolved once, by where
//! its key came from, and every shape read from the view (its surface,
//! its loops with their roles, its boundary member by member or a loop
//! at a time, its cycles, half-edges and edges). The source is chosen at
//! the door that builds the view: [`Body::proven_face`] for a key the
//! kernel enumerated, minted or resolved, [`Body::linked_face`] for one a
//! record holds, [`Body::passed_face`] for one a caller passed (its miss
//! is the caller's typed refusal), and [`Body::face_views`] for every
//! face. The order of a face's loops, with each one's role, is the
//! record's own ([`crate::entity::Face::loop_roles`],
//! [`crate::entity::Face::loop_keys`]), read raw only by a reader that
//! reports a miss itself, which reads [`Body::unchecked_face_cycles`]
//! for the cycles.
use crate::body::{Body, CYCLES_ARE_CLAIMANTS};
use crate::entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, GeomRef, HalfEdge, HalfEdgeKey, Loop, LoopBoundary,
    LoopKey, LoopRole, VertexKey,
};
use crate::euler::{BadArgument, EulerOpError, RunExtent};
use crate::geometry::PointKey;
use crate::null::CurveGeom;
use geom::Surface;
use geom_core::{Point3, Real};
use slotmap::SecondaryMap;

/// A [`HalfEdgeKey`] a lookup has returned — see the [module
/// docs](self) for the precise claim and its one residue.
///
/// `Copy`, because a proof used twice is the same proof: a splice
/// through a one-half-edge loop links a key to itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Live(HalfEdgeKey);

impl Live {
    /// **The one place a `Live` is built from a bare key.** Private, so
    /// every caller of it is in this file and is a door that has just
    /// completed a successful lookup.
    const fn new(he: HalfEdgeKey) -> Self {
        Self(he)
    }

    /// Proof from a bare key: the lookup, and nothing else. `None` when
    /// the key does not resolve.
    pub(crate) fn of<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> Option<Self> {
        body.half_edges.contains_key(he).then(|| Self::new(he))
    }

    /// The key back out. The proof is one-way by design: this direction
    /// discards it, which is always sound.
    pub(crate) fn key(self) -> HalfEdgeKey {
        self.0
    }
}

/// The premise a link that does not resolve breaks: every link-miss
/// panic names it, and the rows that drive a torn body match on it.
pub(crate) const NAMES_ONLY_LIVE: &str = "every public door keeps the body tier-1-valid, \
     and a tier-1-valid record names only live records";

/// The premise a link that does not resolve breaks on a body
/// mid-operation, where tier 1 is not yet asked: every link-miss panic,
/// [`proven`]-miss panic and broken-walk panic ([`crate::body::Walk::closed`],
/// [`crate::body::Walk::closed_in`])
/// names it beside the at-rest premise, and the reads that panic on a
/// link mid-operation cite it rather than restate it. Every removal of
/// a surface or a curve is orphan-only
/// ([`Body::remove_surface_if_orphaned`],
/// [`Body::remove_curve_if_orphaned`], and `splitting/finish.rs`'
/// sweep, which collects the live ones from faces and curves first).
/// The one removal of topology that is not an Euler operator,
/// [`crate::splitting::finish::carve`]'s, checks its own premise (it
/// drops only records no kept record names), which a read of a carved
/// body cites beside this one.
pub(crate) const OPERATORS_KEEP_LINKS: &str = "mid-operation, every Euler operator leaves each \
     link it writes resolving and each walk it writes closed, and removes a record only once no \
     record names it";

/// Where a key a plan phase resolves came from, and so what its miss
/// is: [`Arg`]'s is the caller's typed refusal, so a lookup through it
/// answers `Result`; [`Link`]'s is a kernel bug that panics, so a
/// lookup through it answers the record itself and has no `Err` to
/// propagate.
pub(crate) trait KeySource: Copy {
    /// What a lookup through this source answers for a found `V`.
    type Answer<V>;

    /// `found`, or this source's answer for `key` failing to resolve.
    #[track_caller]
    fn answer<V>(self, found: Option<V>, key: EntityId) -> Self::Answer<V>;

    /// [`KeySource::answer`] for a geometry key.
    #[track_caller]
    fn answer_geometry<V>(self, found: Option<V>, key: GeomRef) -> Self::Answer<V>;

    /// `f` applied under the answer: the rest of a door whose every
    /// later hop is a link, so cannot refuse.
    fn map<V, W>(answer: Self::Answer<V>, f: impl FnOnce(V) -> W) -> Self::Answer<W>;
}

/// A key the caller passed, as the argument this role names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Arg(pub(crate) &'static str);

impl Arg {
    /// The caller's [`BadArgument::Stale`] for `key`.
    pub(crate) fn miss(self, key: EntityId) -> EulerOpError {
        EulerOpError::Argument(BadArgument::Stale { role: self.0, key })
    }

    /// The caller's [`BadArgument::StaleGeometry`] for `key`.
    pub(crate) fn miss_geometry(self, key: GeomRef) -> EulerOpError {
        EulerOpError::Argument(BadArgument::StaleGeometry { role: self.0, key })
    }
}

impl KeySource for Arg {
    type Answer<V> = Result<V, EulerOpError>;

    fn answer<V>(self, found: Option<V>, key: EntityId) -> Result<V, EulerOpError> {
        found.ok_or_else(|| self.miss(key))
    }

    fn answer_geometry<V>(self, found: Option<V>, key: GeomRef) -> Result<V, EulerOpError> {
        found.ok_or_else(|| self.miss_geometry(key))
    }

    fn map<V, W>(
        answer: Result<V, EulerOpError>,
        f: impl FnOnce(V) -> W,
    ) -> Result<W, EulerOpError> {
        answer.map(f)
    }
}

/// A key a record of the body holds: `holder`'s field `link`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Link {
    /// The record whose field holds the key.
    pub(crate) holder: EntityId,
    /// The field.
    pub(crate) link: &'static str,
}

impl KeySource for Link {
    type Answer<V> = V;

    #[track_caller]
    fn answer<V>(self, found: Option<V>, key: EntityId) -> V {
        let Some(found) = found else {
            dangling_link(self.holder, self.link, key)
        };
        found
    }

    #[track_caller]
    fn answer_geometry<V>(self, found: Option<V>, key: GeomRef) -> V {
        let Some(found) = found else {
            dangling_link(self.holder, self.link, key)
        };
        found
    }

    fn map<V, W>(answer: V, f: impl FnOnce(V) -> W) -> W {
        f(answer)
    }
}

/// A key this call already resolved, minted, or read out of a record
/// it resolved ([`proven`]): a lookup through it answers the record,
/// and its miss is a kernel bug that panics naming the key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Proven;

impl KeySource for Proven {
    type Answer<V> = V;

    #[track_caller]
    fn answer<V>(self, found: Option<V>, key: EntityId) -> V {
        let Some(found) = found else { unproven(key) };
        found
    }

    #[track_caller]
    fn answer_geometry<V>(self, found: Option<V>, key: GeomRef) -> V {
        let Some(found) = found else { unproven(key) };
        found
    }

    fn map<V, W>(answer: V, f: impl FnOnce(V) -> W) -> W {
        f(answer)
    }
}

/// The panic for a [`Proven`] key that does not resolve.
#[track_caller]
fn unproven(key: impl core::fmt::Display) -> ! {
    unreachable!(
        "{key}, which this call minted and kept, resolved, or read out of a record, does not \
         resolve: nothing removes a record during a plan, a call keeps every record it mints \
         and still reads, and {NAMES_ONLY_LIVE}; {OPERATORS_KEEP_LINKS}"
    )
}

/// The panic for a link that does not resolve: `holder`'s field `link`
/// names `key`. Only a kernel bug reaches it ([`NAMES_ONLY_LIVE`]).
#[track_caller]
pub(crate) fn dangling_link(
    holder: impl core::fmt::Display,
    link: &str,
    key: impl core::fmt::Display,
) -> ! {
    unreachable!(
        "{holder}'s {link} names {key}, which does not resolve: {NAMES_ONLY_LIVE}; \
         {OPERATORS_KEEP_LINKS}"
    )
}

/// [`Link`]: `holder`'s field `link`.
pub(crate) const fn link(holder: EntityId, link: &'static str) -> Link {
    Link { holder, link }
}

/// `key`'s record in `arena`, its miss answered as `from` says
/// ([`KeySource`]). The one lookup a plan phase resolves a key through.
#[track_caller]
pub(crate) fn lookup<K: slotmap::Key, V, S: KeySource>(
    arena: &slotmap::SlotMap<K, V>,
    key: K,
    id: fn(K) -> EntityId,
    from: S,
) -> S::Answer<&V> {
    from.answer(arena.get(key), id(key))
}

/// [`lookup`] through a [`Link`]: `holder`'s field `link` names `key`.
#[track_caller]
pub(crate) fn linked<'a, K: slotmap::Key, V>(
    arena: &'a slotmap::SlotMap<K, V>,
    key: K,
    id: fn(K) -> EntityId,
    holder: EntityId,
    link: &'static str,
) -> &'a V {
    lookup(arena, key, id, Link { holder, link })
}

/// `key`'s record in `arena`, for a key this call already resolved,
/// minted, or read out of a record it resolved: nothing removes a record
/// during a plan, a tier-1-valid record names only live records, and
/// mid-operation [`OPERATORS_KEEP_LINKS`], so a miss is a kernel bug and
/// panics. A key read out of a record is better resolved through
/// [`linked`], whose panic names the record that holds it.
#[track_caller]
pub(crate) fn proven<K: slotmap::Key, V, I: core::fmt::Display>(
    arena: &slotmap::SlotMap<K, V>,
    key: K,
    id: fn(K) -> I,
) -> &V {
    let Some(record) = arena.get(key) else {
        unproven(id(key))
    };
    record
}

/// [`proven`], for a record this call writes.
#[track_caller]
pub(crate) fn proven_mut<K: slotmap::Key, V, I: core::fmt::Display>(
    arena: &mut slotmap::SlotMap<K, V>,
    key: K,
    id: fn(K) -> I,
) -> &mut V {
    let Some(record) = arena.get_mut(key) else {
        unproven(id(key))
    };
    record
}

/// Requires a key to be live in its arena, answering a miss as `from`
/// says — [`lookup`] for a plan phase that wants the refusal and not
/// the record. Only half-edges are spliced through a key held across a
/// `&mut` call, so only they carry a [`Live`] forward.
#[track_caller]
pub(crate) fn require_key<K: slotmap::Key, V, S: KeySource>(
    arena: &slotmap::SlotMap<K, V>,
    key: K,
    id: fn(K) -> EntityId,
    from: S,
) -> S::Answer<()> {
    S::map(lookup(arena, key, id, from), |_| ())
}

/// `he`'s place on edge `ek` (its record `data`), for a `he` that names
/// `ek`: on a tier-1-valid body the edge claims it, so a miss panics.
#[track_caller]
fn claimed(he: HalfEdgeKey, ek: EdgeKey, data: &Edge) -> crate::entity::Claim {
    data.claim(he).unwrap_or_else(|| {
        unreachable!(
            "{he:?}'s edge {ek:?} does not claim it in either slot: on a tier-1-valid \
             body an edge claims the two half-edges that name it"
        )
    })
}

/// **An edge of a face's loops** ([`FaceView::edges`]), as one of its
/// half-edges walks it: the half-edge, the edge, its two ends in the
/// half-edge's direction, the half-edge's mate, and the face across it
/// (the face itself across a seam inside it).
#[derive(Clone, Copy)]
pub(crate) struct FaceEdge {
    // Read by the probes' frames, which orient the edge by it.
    #[cfg_attr(not(any(test, feature = "test-support")), allow(dead_code))]
    pub(crate) half: HalfEdgeKey,
    pub(crate) edge: EdgeKey,
    pub(crate) ends: [VertexKey; 2],
    pub(crate) mate: HalfEdgeKey,
    pub(crate) across: FaceKey,
}

/// One member of a face's boundary ([`FaceView::boundary`]).
pub(crate) enum BoundaryMember<'a, T: Real> {
    /// An isolated-vertex loop's `vertex`, at `point`.
    Isolated { vertex: VertexKey, point: Point3<T> },
    /// A loop cycle member `he`, whose record is `half`, and its edge
    /// `ek`, whose record is `edge`.
    Edge {
        he: HalfEdgeKey,
        half: &'a HalfEdge,
        ek: EdgeKey,
        edge: &'a Edge,
    },
}

impl<T: Real> BoundaryMember<'_, T> {
    /// The member's half-edge; none for a lone vertex. **The one
    /// disposition of a lone-vertex loop for a reader of half-edges**: it
    /// holds no half-edge and no edge, so it bounds no area and has
    /// nothing to walk, and every such reader (here and
    /// [`FaceView::cycles`], [`FaceView::edges`])
    /// inherits this skip rather than writing its own.
    pub(crate) fn half_edge(&self) -> Option<HalfEdgeKey> {
        let BoundaryMember::Edge { he, .. } = *self else {
            return None;
        };
        Some(he)
    }

    /// The member's vertex: the lone vertex, or where the half-edge
    /// starts.
    pub(crate) fn vertex(&self) -> VertexKey {
        match *self {
            Self::Isolated { vertex, .. } => vertex,
            Self::Edge { half, .. } => half.start,
        }
    }
}

/// **A face resolved once, by where its key came from**, and every shape
/// of it read from here: its record, its surface, its loops with their
/// roles, its boundary member by member, its cycles, its half-edges and
/// its edges.
///
/// The source decides how the face's own miss reads, once, at the door
/// that builds the view: [`Body::proven_face`] for a key the kernel
/// enumerated, minted or resolved, [`Body::linked_face`] for one a
/// record holds, [`Body::passed_face`] for one a caller passed (whose
/// miss is the caller's to refuse), and [`Body::face_views`] for every
/// face. Every hop past the face is a link its record holds, so every
/// shape below panics on a torn body naming the hop and its holder
/// (D2 row 4), and none of them answers a miss.
///
/// The fields are private to this module, so a view exists only where a
/// door resolved its face: a shape of a face is never read from a bare
/// key. A reader that reports a miss itself (the validators) reads the
/// record's raw keys ([`Face::loop_keys`]) or
/// [`Body::unchecked_face_cycles`] instead.
#[derive(Clone, Copy)]
pub(crate) struct FaceView<'a, T: Real> {
    body: &'a Body<T>,
    key: FaceKey,
    record: &'a Face,
}

impl<'a, T: Real> FaceView<'a, T> {
    /// The face's key.
    pub(crate) fn key(self) -> FaceKey {
        self.key
    }

    /// The face's record.
    pub(crate) fn record(self) -> &'a Face {
        self.record
    }

    /// The surface the face is a region of, a link its record holds.
    #[track_caller]
    pub(crate) fn surface(self) -> &'a Surface<T> {
        let surface = self.record.surface;
        self.body.get_surface(surface).unwrap_or_else(|| {
            dangling_link(
                EntityId::Face(self.key),
                "surface",
                GeomRef::Surface(surface),
            )
        })
    }

    /// The face's loops, each a link its record holds, in
    /// [`Face::loop_roles`]' order, each with its role: the outer loop,
    /// then its rings. A miss names the role's field.
    pub(crate) fn loops(self) -> impl Iterator<Item = (LoopKey, &'a Loop, LoopRole)> + 'a {
        let (body, holder) = (self.body, EntityId::Face(self.key));
        self.record.loop_roles().map(move |(role, lk)| {
            let l = linked(&body.loops, lk, EntityId::Loop, holder, role.field());
            (lk, l, role)
        })
    }

    /// The face's outer loop, a link its record holds: the first of
    /// [`FaceView::loops`], for a reader of it alone.
    #[track_caller]
    pub(crate) fn outer(self) -> (LoopKey, &'a Loop) {
        let lk = self.record.outer;
        let l = linked(
            &self.body.loops,
            lk,
            EntityId::Loop,
            EntityId::Face(self.key),
            LoopRole::Outer.field(),
        );
        (lk, l)
    }

    /// [`FaceView::boundary`] a loop at a time: each loop's key, its
    /// members and its role, so a reader that needs a loop whole has it.
    pub(crate) fn boundary_by_loop(
        self,
    ) -> impl Iterator<Item = (LoopKey, Vec<BoundaryMember<'a, T>>, LoopRole)> + 'a {
        let body = self.body;
        self.loops()
            .map(move |(lk, l, role)| (lk, body.loop_members_linked(lk, l), role))
    }

    /// The face's boundary, member by member, in [`FaceView::loops`]'
    /// order: each isolated-vertex loop's vertex and point, and each
    /// closed loop cycle member with its edge. A loop walk that does not
    /// close on the half-edges claiming the loop panics
    /// ([`Body::loop_walk_of`], [`crate::body::Walk::closed_in`]).
    pub(crate) fn boundary(self) -> impl Iterator<Item = BoundaryMember<'a, T>> + 'a {
        self.boundary_by_loop().flat_map(|(_, members, _)| members)
    }

    /// Every loop with its cycle, a lone-vertex loop's empty one
    /// included: the whole proofs below read every loop's claimants.
    fn every_cycle(self) -> impl Iterator<Item = (LoopKey, Vec<HalfEdgeKey>)> + 'a {
        self.boundary_by_loop().map(|(lk, members, _)| {
            let cycle = members
                .iter()
                .filter_map(BoundaryMember::half_edge)
                .collect();
            (lk, cycle)
        })
    }

    /// **Each cycle loop of the face with its cycle**, in
    /// [`FaceView::loops`]' order, each in `next` order. A lone-vertex
    /// loop has no cycle and is not yielded
    /// ([`BoundaryMember::half_edge`]'s disposition).
    pub(crate) fn cycles(self) -> impl Iterator<Item = (LoopKey, Vec<HalfEdgeKey>)> + 'a {
        self.every_cycle().filter(|(_, cycle)| !cycle.is_empty())
    }

    /// [`FaceView::cycles`]' half-edges in one list: the members of every
    /// cycle bounding the face, outer loop then rings, each in `next`
    /// order.
    pub(crate) fn half_edges(self) -> Vec<HalfEdgeKey> {
        self.cycles().flat_map(|(_, cycle)| cycle).collect()
    }

    /// **Every edge of the face's loops**, rings included, in
    /// [`FaceView::boundary`]'s order ([`FaceEdge`]). A loop with no edge
    /// (a lone ring vertex, whose point the walk resolves) bounds no
    /// area and has none.
    ///
    /// # Panics
    ///
    /// Where a record on the walk from the face does not resolve, a loop
    /// walk does not close on the half-edges that claim the loop, or an
    /// edge does not claim a half-edge that names it (D2 row 4).
    #[track_caller]
    pub(crate) fn edges(self) -> Vec<FaceEdge> {
        let body = self.body;
        self.boundary()
            .filter_map(|member| {
                let BoundaryMember::Edge { he, half, ek, edge } = member else {
                    return None;
                };
                let (mate, across) = body.face_across_linked(he, ek, edge);
                Some(FaceEdge {
                    half: he,
                    edge: ek,
                    ends: [half.start, body.proven_half_edge_end(he)],
                    mate,
                    across,
                })
            })
            .collect()
    }

    /// The keys of [`FaceView::edges`], each once (a seam inside the face
    /// once).
    pub(crate) fn edge_set(self) -> std::collections::BTreeSet<EdgeKey> {
        self.edges().into_iter().map(|e| e.edge).collect()
    }
}

impl<T: geom_core::Decide> FaceView<'_, T> {
    /// **The rows a face re-charted in place drops**: every half-edge of
    /// every loop of the face, outer then rings, each loop proven whole
    /// ([`Body::require_run_of`]'s [`RunExtent::Whole`]: no half-edge but
    /// the cycle's members claims it). [`Body::drop_rows_on_chart_change`]'s
    /// answer for a whole face, taken in a setter's plan phase and handed
    /// to [`Body::drop_rows`] in its mutation phase.
    ///
    /// A surface setter re-charts a face in place: no loop moves, no key
    /// changes, and every row the face stores is suddenly a curve stated
    /// in the chart the face left. The chart decision is the caller's,
    /// taken once ([`Body::same_chart`]), so this reads no surface. Each
    /// loop's walk is [`FaceView::cycles`]', the walk
    /// [`crate::pcurves::validate_pcurves`] reads the same face with, so
    /// the rows the door removes and the rows the validator would have
    /// read are one set by construction; the whole proof makes it every
    /// row the face holds.
    ///
    /// # Panics
    ///
    /// As [`FaceView::boundary`], and where a half-edge claims a loop of
    /// the face its cycle walk does not reach.
    #[track_caller]
    pub(crate) fn whole_half_edges(self) -> Vec<HalfEdgeKey> {
        let mut halves = Vec::new();
        for (r#loop, cycle) in self.every_cycle() {
            self.body
                .require_run_of(cycle.iter().copied(), r#loop, RunExtent::Whole, &[]);
            halves.extend(cycle);
        }
        halves
    }
}

impl<T: Real> Body<T> {
    /// [`FaceView::whole_half_edges`] of each face of `faces`, in order,
    /// under ONE whole-arena pass for all of them: the half-edge arena is
    /// read once for every face's loops, where a face at a time reads it
    /// once per loop. The same proofs, the same panics.
    ///
    /// # Panics
    ///
    /// As [`FaceView::whole_half_edges`], and where a face of `faces`
    /// does not resolve ([`Body::proven_face`]).
    #[track_caller]
    pub(crate) fn faces_whole_half_edges(
        &self,
        faces: impl IntoIterator<Item = FaceKey>,
    ) -> Vec<Vec<HalfEdgeKey>> {
        let mut loops: SecondaryMap<LoopKey, ()> = SecondaryMap::new();
        let mut members: SecondaryMap<HalfEdgeKey, ()> = SecondaryMap::new();
        let mut out = Vec::new();
        for face in faces {
            let mut halves = Vec::new();
            for (r#loop, cycle) in self.proven_face(face).every_cycle() {
                // `require_run_of`'s member proof, without the set it
                // builds per loop.
                for &member in &cycle {
                    assert!(
                        self.claims(member, r#loop),
                        "{member:?}, walked as a member of loop {:?}'s cycle, does not claim \
                         it: {CYCLES_ARE_CLAIMANTS}",
                        r#loop
                    );
                    members.insert(member, ());
                }
                loops.insert(r#loop, ());
                halves.extend(cycle);
            }
            out.push(halves);
        }
        if let Some((stray, data)) = self
            .half_edges
            .iter()
            .find(|&(he, data)| loops.contains_key(data.parent_loop) && !members.contains_key(he))
        {
            unreachable!(
                "{stray:?} claims loop {:?} and its cycle walk does not reach it: \
                 {CYCLES_ARE_CLAIMANTS}",
                data.parent_loop
            );
        }
        out
    }

    /// **`face`, a key the kernel enumerated, minted, resolved or read out
    /// of its own records** ([`proven`]): a miss is a kernel bug, and
    /// panics naming the key and the premise it breaks. A key a record
    /// holds is better resolved through [`Body::linked_face`], whose
    /// panic names the record.
    #[track_caller]
    pub(crate) fn proven_face(&self, face: FaceKey) -> FaceView<'_, T> {
        FaceView {
            body: self,
            key: face,
            record: proven(&self.faces, face, EntityId::Face),
        }
    }

    /// **`face`, a key `holder`'s field `field` holds**: a miss is a
    /// kernel bug ([`NAMES_ONLY_LIVE`]), and panics naming the record and
    /// its field.
    #[track_caller]
    pub(crate) fn linked_face(
        &self,
        face: FaceKey,
        holder: EntityId,
        field: &'static str,
    ) -> FaceView<'_, T> {
        FaceView {
            body: self,
            key: face,
            record: linked(&self.faces, face, EntityId::Face, holder, field),
        }
    }

    /// **`face`, a key a caller passed**: `None` where it does not
    /// resolve, which is the caller's to refuse in its own terms.
    pub(crate) fn passed_face(&self, face: FaceKey) -> Option<FaceView<'_, T>> {
        let record = self.faces.get(face)?;
        Some(FaceView {
            body: self,
            key: face,
            record,
        })
    }

    /// **Every face of the body**, in arena order.
    pub(crate) fn face_views(&self) -> impl Iterator<Item = FaceView<'_, T>> {
        self.faces.iter().map(move |(key, record)| FaceView {
            body: self,
            key,
            record,
        })
    }

    /// **The explicitly unchecked walk of a face's cycles**, for a reader
    /// that reports a miss itself (the census over a body the validator
    /// has not certified): each of `face`'s loops in
    /// [`Face::loop_keys`]' order, `Ok(cycle)` for a cycle loop the body
    /// can walk, `Err(loop)` for one it cannot (a lost loop, a lone-vertex
    /// loop, an unwalkable cycle). What an unwalkable loop means is the
    /// caller's to say; this walk never decides it. A reader of a body
    /// every door keeps valid reads a [`FaceView`].
    pub(crate) fn unchecked_face_cycles<'a>(
        &'a self,
        face: &'a Face,
    ) -> impl Iterator<Item = Result<Vec<HalfEdgeKey>, LoopKey>> + 'a {
        face.loop_keys().map(move |lk| {
            let Some(loop_) = self.loops.get(lk) else {
                return Err(lk);
            };
            let LoopBoundary::Cycle { first } = loop_.boundary else {
                return Err(lk);
            };
            self.loop_cycle(first).ok_or(lk)
        })
    }

    /// Resolves a vertex's point coordinates (the certification gate's
    /// endpoints): the vertex's miss answered as `from` says
    /// ([`KeySource`]), and its point's, a link the vertex holds, a
    /// panic.
    #[track_caller]
    pub(crate) fn resolve_vertex_point<S: KeySource>(
        &self,
        vertex: VertexKey,
        from: S,
    ) -> S::Answer<Point3<T>> {
        S::map(
            lookup(&self.vertices, vertex, EntityId::Vertex, from),
            |v| {
                *link(EntityId::Vertex(vertex), "point")
                    .answer_geometry(self.points.get(v.point), GeomRef::Point(v.point))
            },
        )
    }

    /// `edge`'s curve-arena entry, a link its record holds.
    #[track_caller]
    pub(crate) fn edge_curve_linked(&self, edge: EdgeKey, data: &Edge) -> &CurveGeom<T> {
        self.get_curve_geom(data.curve).unwrap_or_else(|| {
            dangling_link(EntityId::Edge(edge), "curve", GeomRef::Curve(data.curve))
        })
    }

    /// The mate of `he`, a half of edge `ek` (its record `data`), and the
    /// face across it, the face of the mate's loop: every hop a link.
    #[track_caller]
    fn face_across_linked(
        &self,
        he: HalfEdgeKey,
        ek: EdgeKey,
        data: &Edge,
    ) -> (HalfEdgeKey, FaceKey) {
        let claim = claimed(he, ek, data);
        let mate = linked(
            &self.half_edges,
            claim.mate,
            EntityId::HalfEdge,
            EntityId::Edge(ek),
            claim.mate_field(),
        );
        let across = linked(
            &self.loops,
            mate.parent_loop,
            EntityId::Loop,
            EntityId::HalfEdge(claim.mate),
            "parent_loop",
        )
        .face;
        (claim.mate, across)
    }

    /// Loop `lk`'s members (its record is `l`), as
    /// [`FaceView::boundary`] yields them.
    pub(crate) fn loop_members_linked(&self, lk: LoopKey, l: &Loop) -> Vec<BoundaryMember<'_, T>> {
        match l.boundary {
            LoopBoundary::Empty { vertex } => {
                let point = self.linked_vertex_point(vertex, EntityId::Loop(lk), "boundary");
                vec![BoundaryMember::Isolated { vertex, point }]
            }
            LoopBoundary::Cycle { first } => self
                .loop_walk_of(first, lk)
                .closed_in(lk, first)
                .into_iter()
                .map(|he| {
                    let half = proven(&self.half_edges, he, EntityId::HalfEdge);
                    let ek = half.edge;
                    let edge = linked(
                        &self.edges,
                        ek,
                        EntityId::Edge,
                        EntityId::HalfEdge(he),
                        "edge",
                    );
                    BoundaryMember::Edge { he, half, ek, edge }
                })
                .collect(),
        }
    }

    /// The point of `member`'s vertex ([`BoundaryMember::vertex`]): the
    /// lone vertex's, or the point of the vertex its half-edge starts at,
    /// a link.
    #[track_caller]
    pub(crate) fn member_point(&self, member: &BoundaryMember<'_, T>) -> Point3<T> {
        match *member {
            BoundaryMember::Isolated { point, .. } => point,
            BoundaryMember::Edge { he, half, .. } => {
                self.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start")
            }
        }
    }

    /// [`Body::resolve_vertex_point`] for a vertex `holder`'s field `link`
    /// names.
    #[track_caller]
    pub(crate) fn linked_vertex_point(
        &self,
        vertex: VertexKey,
        holder: EntityId,
        field: &'static str,
    ) -> Point3<T> {
        self.resolve_vertex_point(vertex, link(holder, field))
    }

    /// One clockwise step of `he`'s vertex orbit, `next(mate(he))`
    /// ([`Body::orbit_step`]), for a `he` this call proved: every hop
    /// past it is a link, and a miss panics.
    #[track_caller]
    pub(crate) fn proven_orbit_step(&self, he: HalfEdgeKey) -> HalfEdgeKey {
        let edge = proven(&self.half_edges, he, EntityId::HalfEdge).edge;
        let data = linked(
            &self.edges,
            edge,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        let claim = claimed(he, edge, data);
        linked(
            &self.half_edges,
            claim.mate,
            EntityId::HalfEdge,
            EntityId::Edge(edge),
            claim.mate_field(),
        )
        .next
    }

    /// `he`'s end vertex, `start(next(he))` ([`Body::half_edge_end`]),
    /// for a `he` this call proved: its `next` is a link, and a miss
    /// panics.
    #[track_caller]
    pub(crate) fn proven_half_edge_end(&self, he: HalfEdgeKey) -> VertexKey {
        let next = proven(&self.half_edges, he, EntityId::HalfEdge).next;
        linked(
            &self.half_edges,
            next,
            EntityId::HalfEdge,
            EntityId::HalfEdge(he),
            "next",
        )
        .start
    }

    /// The key of a vertex's point, both resolving, with the misses
    /// answered as [`Body::resolve_vertex_point`] answers them.
    #[track_caller]
    pub(crate) fn resolve_vertex_point_key<S: KeySource>(
        &self,
        vertex: VertexKey,
        from: S,
    ) -> S::Answer<PointKey> {
        S::map(self.resolve_vertex_point(vertex, from), |_| {
            self.vertices[vertex].point
        })
    }

    /// Requires a half-edge key to be live, answering a miss as `from`
    /// says ([`KeySource`]).
    ///
    /// This is the plan-phase door: an operator that will splice through
    /// a key proves it here, **before any mutation**, so the mutation
    /// phase below cannot fail midway (atomicity).
    #[track_caller]
    pub(crate) fn require_live<S: KeySource>(&self, he: HalfEdgeKey, from: S) -> S::Answer<Live> {
        S::map(self.resolve_half_edge_live(he, from), |(live, _)| live)
    }

    /// [`Body::resolve_half_edge`] keeping the proof its lookup earns,
    /// for an operator that both reads a half-edge's fields and splices
    /// through the key itself.
    ///
    /// The proof comes out of the same lookup the fields do, so this
    /// door looks up exactly once.
    #[track_caller]
    pub(crate) fn resolve_half_edge_live<S: KeySource>(
        &self,
        he: HalfEdgeKey,
        from: S,
    ) -> S::Answer<(Live, HalfEdge)> {
        S::map(
            lookup(&self.half_edges, he, EntityId::HalfEdge, from),
            |data| (Live::new(he), data.clone()),
        )
    }

    /// The loop walk from `he`, closed ([`crate::body::Walk::closed`]:
    /// a walk a torn body breaks panics naming the hop), with its
    /// members proven: a closed walk resolved every member it returns.
    #[track_caller]
    pub(crate) fn loop_cycle_live(&self, he: HalfEdgeKey) -> Vec<Live> {
        self.loop_walk(he)
            .closed("loop", he)
            .into_iter()
            .map(|member| {
                Live::of(self, member).unwrap_or_else(|| {
                    unreachable!("the closed loop walk from {he:?} resolved {member:?}")
                })
            })
            .collect()
    }
}

#[cfg(test)]
// Test-support code: panicking is a test's failure mechanism (L5).
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Arg, BoundaryMember, Live};
    use crate::body::Body;
    use crate::entity::{
        EdgeKey, EntityId, FaceKey, HalfEdge, HalfEdgeKey, LoopBoundary, LoopKey, LoopRole,
        VertexKey,
    };
    use crate::euler::{BadArgument, EulerOpError};
    use crate::fixtures::pillow;
    use crate::source_walk::{CodeOnly, crate_sources, src_root, tokens};
    use geom_core::Tol;
    use test_utils::source::{ItemBody, balanced_end, item_body};

    /// `require_live`'s answer for `he` as an argument: the caller's
    /// stale key, under the role it was passed as.
    fn refused_stale<T: geom_core::Real>(body: &Body<T>, he: HalfEdgeKey) -> bool {
        body.require_live(he, Arg("he"))
            == Err(EulerOpError::Argument(BadArgument::Stale {
                role: "he",
                key: EntityId::HalfEdge(he),
            }))
    }

    fn scaffold() -> HalfEdge {
        HalfEdge {
            edge: EdgeKey::default(),
            start: VertexKey::default(),
            parent_loop: LoopKey::default(),
            next: HalfEdgeKey::default(),
            prev: HalfEdgeKey::default(),
        }
    }

    /// The null key is what an unspliced half-edge's `next`/`prev` hold
    /// — [`Body::mint_halves`] leaves both provisional — so it is the
    /// key likeliest to reach a door by accident. It resolves in no
    /// body, populated or not.
    #[test]
    fn the_null_key_is_never_live() {
        let empty = Body::<f64>::new();
        assert!(Live::of(&empty, HalfEdgeKey::default()).is_none());
        assert!(refused_stale(&empty, HalfEdgeKey::default()));
        let body = pillow(Tol::witness()).body;
        assert!(!body.half_edges.is_empty(), "the arena must be populated");
        assert!(Live::of(&body, HalfEdgeKey::default()).is_none());
        assert!(refused_stale(&body, HalfEdgeKey::default()));
    }

    /// A key a record holds that does not resolve is a kernel bug, and
    /// the door panics naming the record and its field rather than
    /// refusing typed.
    #[test]
    #[should_panic(expected = "'s next names half-edge")]
    fn a_link_that_misses_panics_naming_its_holder() {
        let body = Body::<f64>::new();
        let holder = EntityId::HalfEdge(HalfEdgeKey::default());
        let _ = body.require_live(HalfEdgeKey::default(), super::link(holder, "next"));
    }

    /// A removed key never becomes live again, however hard the slot it
    /// vacated is reused. This is what makes the module's residue rule
    /// *loud*: a proof that outlived its key fails at the splice rather
    /// than resolving to whatever now occupies the slot.
    #[test]
    fn a_removed_key_stays_dead_across_slot_reuse() {
        let mut body = Body::<f64>::new();
        let dead = body.half_edges.insert(scaffold());
        body.half_edges.remove(dead);
        assert!(Live::of(&body, dead).is_none());
        for _ in 0..200_000 {
            let fresh = body.half_edges.insert(scaffold());
            assert_ne!(fresh, dead, "a reused slot must mint a NEW key");
            assert!(
                Live::of(&body, dead).is_none(),
                "the dead key resolved once its slot was reused"
            );
            body.half_edges.remove(fresh);
        }
        assert!(refused_stale(&body, dead));
    }

    /// The spellings that count as *this door resolved the key*, each
    /// with the position of the key among its call's arguments: a read
    /// of the half-edge arena, or a call to a door this row itself pins
    /// to have performed one.
    ///
    /// **Closed, and it is this file's list rather than a sketch of
    /// one.** Every entry is a spelling a door below actually uses; a
    /// door whose lookup is spelled any other way matches nothing here
    /// and reds. Treating an unrecognised spelling as a lookup would
    /// pass a door that looks nothing up, which is the one thing this
    /// row exists to catch — so a spelling joins the list with the door
    /// that made it necessary, and never ahead of one.
    ///
    /// **Each is anchored on the arena it reads, not on the bare
    /// method.** A bare `.get(` or `lookup(` is answered by a read of
    /// any map at all, so `lookup(&self.faces, f, ..)` would stand as
    /// the lookup for a half-edge nothing resolved.
    const LOOKUPS: [(&str, usize); 5] = [
        ("half_edges.contains_key(", 0), // the membership test, and nothing else
        ("lookup(&self.half_edges,", 1), // the read whose `Ok` carries the fields
        ("resolve_half_edge_live(", 0),  // delegation to a door this same row pins
        ("loop_cycle(", 0),              // the bounded walk, which resolves every member
        ("Live::of(", 1),                // delegation to a door this same row pins
    ];

    /// Every spelling that builds a `Live` from a bare key. `Live` and
    /// `Self` both, because inside `impl Live` the constructor answers
    /// to either name. The `new` paths carry no `(`, so a constructor
    /// named point-free — `.map(Live::new)` — is a construction too.
    const CONSTRUCTIONS: [&str; 4] = ["Live::new", "Self::new", "Live(", "Self("];

    /// The doors that hand a `Live` out, in source order. **This is the
    /// list** — the module header points at this row rather than
    /// restating it, so there is one copy of it to keep true.
    const DOORS: [&str; 4] = [
        "of",
        "require_live",
        "resolve_half_edge_live",
        "loop_cycle_live",
    ];

    /// The items that build one, in source order. `new` is the
    /// constructor itself; the other two are doors that have just
    /// completed a lookup. `require_live` hands one out without
    /// building it: it delegates to `resolve_half_edge_live`.
    const BUILDERS: [&str; 3] = ["new", "of", "resolve_half_edge_live"];

    /// The 1-based line of byte `at` in `src`.
    fn line_of(src: &str, at: usize) -> usize {
        src[..at].bytes().filter(|c| *c == b'\n').count() + 1
    }

    /// Whether `text` names `name` as a whole token.
    fn mentions(text: &str, name: &str) -> bool {
        tokens(text, name).next().is_some()
    }

    /// The `index`th argument of the call `needle` makes at its first
    /// occurrence in `body` — `None` when that occurrence makes no call,
    /// as a constructor named point-free does not. A needle that spells
    /// its own `(` opens the call there; one that does not, at the
    /// first character after it.
    fn argument<'a>(body: &'a str, needle: &str, index: usize) -> Option<&'a str> {
        let at = tokens(body, needle).next()?;
        let open = match needle.find('(') {
            Some(paren) => at + paren,
            None => {
                let end = at + needle.len();
                end + body[end..].find(|c: char| !c.is_whitespace())?
            }
        };
        if body.as_bytes()[open] != b'(' {
            return None;
        }
        let inner = &body[open + 1..balanced_end(body, open)?];
        let mut depth = 0_usize;
        let mut start = 0;
        let mut args = Vec::new();
        for (i, c) in inner.char_indices() {
            match c {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => {
                    args.push(inner[start..i].trim());
                    start = i + 1;
                }
                _ => {}
            }
        }
        args.push(inner[start..].trim());
        args.get(index).copied()
    }

    /// What parts 1–3 of the guard below read off one file's code.
    struct Reading<'a> {
        /// Each names the item it is about, in backticks, first.
        violations: Vec<String>,
        /// The items handing a `Live` out, in source order.
        doors: Vec<&'a str>,
        /// The item holding each construction site, in source order.
        builders: Vec<&'a str>,
    }

    /// Parts 1–3 of the guard below, over any file's code: the live
    /// tree's `live.rs` there, and synthetic text in the rows after it,
    /// which is where the attribution and the blanking have a nested
    /// item to be wrong about.
    fn read_live_guard(code: &CodeOnly) -> Reading<'_> {
        let src = code.as_str();
        let items = code.fns();
        let mut violations: Vec<String> = Vec::new();

        // 1. The declaration, in both halves.
        let decl = src.find("struct Live").expect("the `Live` declaration");
        let open = decl + "struct Live".len();
        let close = balanced_end(src, open).expect("the field list closes");
        let field = src[open + 1..close].trim();
        if field.contains("pub") {
            violations.push(format!(
                "`Live`'s field is `{field}`: a public field is a constructor, and every \
                 crate that can name the type could then forge a proof"
            ));
        }
        let new = items
            .iter()
            .find(|item| item.name == "new")
            .expect("`Live::new`, the one place a `Live` is built from a bare key");
        if new.lead.contains("pub") {
            violations.push(format!(
                "`new` is declared `{}`: a caller outside this module could then build a \
                 `Live` from a key nothing resolved",
                new.lead.trim()
            ));
        }

        // 2. Every door looks up before it builds, and wraps what it
        //    looked up. `Self` in a return type names whichever impl the
        //    item sits in, so only inside `impl Live` does it mean a
        //    `Live`; `Live` itself means one anywhere. Each door is read
        //    by its own body, so what an item nested in it does is that
        //    item's row and not the door's.
        let impl_live = match item_body(src, src.find("impl Live").expect("the `impl Live` block"))
        {
            ItemBody::Body(body) => body,
            other => panic!("the `impl Live` block has no body: {other:?}"),
        };
        let mut doors: Vec<&str> = Vec::new();
        for item in &items {
            let name = item.name;
            let body = item.own_body();
            let hands_out = mentions(item.returns, "Live")
                || (mentions(item.returns, "Self") && impl_live.contains(&item.span.start));
            if name == "new" || !hands_out {
                continue;
            }
            doors.push(name);
            let first = |needles: &mut dyn Iterator<Item = (&'static str, usize)>| {
                needles
                    .filter_map(|(n, key)| tokens(body, n).next().map(|at| (at, n, key)))
                    .min()
            };
            let lookups = first(&mut LOOKUPS.into_iter());
            let builds = first(&mut CONSTRUCTIONS.into_iter().map(|n| (n, 0)));
            match (lookups, builds) {
                (None, _) => violations.push(format!(
                    "`{name}` hands out a `Live` and reaches no lookup this guard knows. \
                     The vocabulary is {LOOKUPS:?} — a door that resolves its key some \
                     other way is a spelling to add there deliberately, never one to pass \
                     unread."
                )),
                (Some((lookup, ..)), Some((build, ..))) if build < lookup => {
                    violations.push(format!(
                        "`{name}` builds a `Live` at line {} before it looks the key up \
                         at line {}",
                        line_of(src, item.span.start + build),
                        line_of(src, item.span.start + lookup),
                    ));
                }
                (Some((_, looked, key)), Some((_, built, _))) => {
                    let (resolved, wrapped) =
                        (argument(body, looked, key), argument(body, built, 0));
                    if resolved != wrapped {
                        violations.push(format!(
                            "`{name}` looks up `{}` and wraps `{}`: the proof it hands \
                             out is about a key it never resolved",
                            resolved.unwrap_or("<unreadable>"),
                            wrapped.unwrap_or("<unreadable>"),
                        ));
                    }
                }
                (Some(_), None) => {}
            }
        }

        // 3. Every construction site, credited to the item it stands in.
        let mut sites: Vec<(usize, &str)> = Vec::new();
        for needle in CONSTRUCTIONS {
            for at in tokens(src, needle) {
                // The innermost item holding the site: the scan yields
                // a host before the items declared inside it.
                match items.iter().rfind(|item| item.span.contains(&at)) {
                    Some(item) => sites.push((at, item.name)),
                    // The declaration itself, which part 1 reads.
                    None if src[..at].trim_end().ends_with("struct") => {}
                    None => violations.push(format!(
                        "`<no item>`: a `Live` is built at line {}, outside every item the \
                         scan read — nothing here can say which door it belongs to",
                        line_of(src, at)
                    )),
                }
            }
        }
        sites.sort_unstable();
        let builders = sites.into_iter().map(|(_, name)| name).collect();
        Reading {
            violations,
            doors,
            builders,
        }
    }

    /// **The guard the module header names.** The claim is stated
    /// there; this is how it is checked, and what a red says.
    ///
    /// The reader is the shared lexer's code-only view of this file, in
    /// which every comment and every literal is spaces — so a match is
    /// code, and the needles above, being literals, cannot answer for
    /// the row that spells them. The items come from `source_walk`'s
    /// item scan; `balanced_end` carves the field list and each call's
    /// arguments; `item_body` carves the `impl Live` block, which is
    /// what separates a `-> Self` that means a `Live` from one that
    /// means a `Body`.
    ///
    /// Every violation is collected before any is reported and each
    /// names the item it is about, so a red says which door and what it
    /// did rather than which assertion happened to fire first. An item
    /// declared inside a door's body is a row of its own, read apart
    /// from its host (`FnItem::own_body`), so a red names the item that
    /// did the thing.
    ///
    /// **What it cannot see**, all of it inherited from reading text:
    /// a lookup reached one hop away through a helper reads as no
    /// lookup (a red, which is the safe direction); the argument check
    /// compares SPELLINGS, so a rebinding between the lookup and the
    /// construction defeats it; a construction whose path no needle
    /// spells defeats every part — the type under another name
    /// (`type L = Live;` or `use … as L`, then `L::new(k)`), the tuple
    /// constructor named point-free (`.map(Self)`, which no spelling
    /// tells from the type), and a path a `macro_rules!` body assembles
    /// from fragments (`$t::new(k)` invoked with `Live`), where only a
    /// construction spelled whole is read; and `cfg` is not evaluated.
    ///
    /// **Which body the lookup read is not this guard's to know, and
    /// nothing catches it.** That `other.half_edges.get(he)` reads the
    /// body the `Live` is spliced into is a data-flow question, which
    /// `source_walk`'s module header rules out; a key proven against
    /// one body and spliced into another is the foreign-key hazard
    /// [`crate::body`]'s module docs record as unprotected, and no
    /// validator row pins that a splice of one is reported.
    #[test]
    fn every_door_that_hands_out_a_live_looks_up_first() {
        let path = src_root().join("live.rs");
        let text = std::fs::read_to_string(&path).expect("this module's own source reads back");
        let code = CodeOnly::of(&text);
        let Reading {
            mut violations,
            doors,
            builders,
        } = read_live_guard(&code);

        // 3. The doors and the construction sites are the listed ones.
        if doors != DOORS {
            violations.push(format!(
                "the items handing out a `Live` are {doors:?}, not {DOORS:?} — each owes a \
                 lookup before it builds, so a new door joins this list once it has one"
            ));
        }
        if builders != BUILDERS {
            violations.push(format!(
                "the items building a `Live` are {builders:?}, not {BUILDERS:?} — \
                 construction is the whole of what this file guards, so a new site joins \
                 that list only behind the lookup that earns it"
            ));
        }

        // 4. The compiler's half, restated over the rest of the crate.
        let mut saw_this_file = false;
        for file in crate_sources() {
            if file == path {
                saw_this_file = true;
                continue;
            }
            let other =
                CodeOnly::of(&std::fs::read_to_string(&file).expect("a readable source file"));
            for needle in ["Live(", "Live::new"] {
                if tokens(other.as_str(), needle).next().is_some() {
                    violations.push(format!(
                        "{} builds a `Live` (`{needle}`): construction lives in live.rs \
                         alone, where every site stands beside the lookup that earns it",
                        file.display()
                    ));
                }
            }
        }
        assert!(
            saw_this_file,
            "the crate walk did not reach live.rs — the guard read nothing"
        );
        assert!(violations.is_empty(), "\n{}", violations.join("\n"));
    }

    /// **The guard's reading, on text that has something to get wrong.**
    /// `live.rs` holds no nested item and no point-free construction, so
    /// the guard's own row is green whichever way it reads them; this
    /// one is not.
    ///
    /// `door` looks its key up and hands a proof out through `of`, but
    /// hosts `forge`, which hosts `deeper`, which builds a `Live` from a
    /// bare key. Read by its whole text, `door` would build before it
    /// looks up; read by its own, it is clean, and the forgery reds
    /// under the name of the item two levels down that commits it — the
    /// innermost item holding the site, not the outermost. `point_free`
    /// looks up and then wraps through `.map(Live::new)`, which a
    /// needle ending in `(` never sees. `resolved` and `wrong_key` look
    /// up through `lookup`, whose key is its second argument: the first
    /// wraps that key and is clean, the second wraps another.
    #[test]
    fn the_guard_credits_each_act_to_the_innermost_item_that_commits_it() {
        let src = "
struct Live(K);
impl Live {
    const fn new(he: K) -> Self { Self(he) }
    fn of(body: &B, he: K) -> Option<Self> { body.half_edges.contains_key(he).then(|| Self::new(he)) }
}
fn door(body: &B, he: K) -> Option<Live> {
    fn forge(k: K) -> Live {
        fn deeper(k: K) -> Live { Live::new(k) }
        deeper(k)
    }
    body.half_edges.contains_key(he).then_some(())?;
    Live::of(body, he)
}
fn point_free(body: &B, he: K) -> Option<Live> {
    body.half_edges.contains_key(he).then_some(he).map(Live::new)
}
fn resolved(&self, he: K, from: F) -> Result<Live, E> {
    lookup(&self.half_edges, he, id, from)?;
    Ok(Live::new(he))
}
fn wrong_key(&self, he: K, other: K, from: F) -> Result<Live, E> {
    lookup(&self.half_edges, he, id, from)?;
    Ok(Live::new(other))
}
";
        let code = CodeOnly::of(src);
        let reading = read_live_guard(&code);
        let named: Vec<&str> = reading
            .violations
            .iter()
            .map(|v| v.split('`').nth(1).unwrap_or(v))
            .collect();
        assert_eq!(
            named,
            vec!["forge", "deeper", "point_free", "wrong_key"],
            "the violations are misattributed, or a nested item's act was read as its \
             host's, or the point-free construction went unseen: {:#?}",
            reading.violations
        );
        assert_eq!(
            reading.doors,
            vec![
                "of",
                "door",
                "forge",
                "deeper",
                "point_free",
                "resolved",
                "wrong_key"
            ],
            "a nested item was not read as a door of its own"
        );
        assert_eq!(
            reading.builders,
            vec!["new", "of", "deeper", "point_free", "resolved", "wrong_key"],
            "a construction site was credited to an item other than the innermost one \
             holding it"
        );
    }

    /// [`crate::fixtures::ops_two_ring_face`]'s face given a third ring,
    /// a triangle planted beside the hole: an outer cycle, then rings
    /// `[hole, tip, pocket]` — a cycle, a lone vertex, a cycle — so the
    /// lone-vertex ring sits between two cycle rings. Returns the body,
    /// the face, its loops in that order and the tip's point.
    fn three_ring_face() -> (Body<f64>, FaceKey, [LoopKey; 4], geom_core::Point3<f64>) {
        let tol = Tol::witness();
        let t = crate::fixtures::ops_two_ring_face(tol);
        let mut body = t.body;
        let LoopBoundary::Cycle { first } = body.get_loop(t.outer).unwrap().boundary else {
            panic!("the ringed face's outer loop is a cycle")
        };
        let z = body
            .get_point(
                body.get_vertex(body.get_half_edge(first).unwrap().start)
                    .unwrap()
                    .point,
            )
            .unwrap()
            .z;
        let rim =
            [(0.85, 0.4), (0.95, 0.5), (0.85, 0.6)].map(|(x, y)| geom_core::Point3::new(x, y, z));
        let pocket = crate::test_support_fixtures::plant_ring_face(&mut body, first, &rim, tol)
            .kill
            .ring;
        let rings = &body.get_face(t.face).unwrap().rings;
        assert_eq!(
            rings[..],
            [t.hole, t.tip, pocket],
            "fixture: the ring order"
        );
        assert_eq!(crate::validate::validate(&body), Ok(()), "fixture: valid");
        let LoopBoundary::Empty { vertex } = body.get_loop(t.tip).unwrap().boundary else {
            panic!("fixture: the tip is a lone-vertex ring")
        };
        let tip = *body
            .get_point(body.get_vertex(vertex).unwrap().point)
            .unwrap();
        (body, t.face, [t.outer, t.hole, t.tip, pocket], tip)
    }

    /// The face boundary iterator yields the outer loop, then the rings
    /// in `face.rings` order; each cycle loop's members are its own
    /// half-edges with their edges, and the lone-vertex ring yields
    /// exactly one `Isolated` member, its vertex at its point.
    #[test]
    fn the_face_boundary_yields_outer_then_rings_and_one_member_per_lone_vertex() {
        let (body, face, order, tip) = three_ring_face();
        let view = body.proven_face(face);
        let by_loop: Vec<_> = view.boundary_by_loop().collect();
        let keys: Vec<LoopKey> = by_loop.iter().map(|(lk, ..)| *lk).collect();
        assert_eq!(keys, order, "outer, then the rings in `face.rings` order");
        let roles: Vec<LoopRole> = by_loop.iter().map(|&(.., role)| role).collect();
        assert_eq!(
            roles,
            [
                LoopRole::Outer,
                LoopRole::Ring(0),
                LoopRole::Ring(1),
                LoopRole::Ring(2)
            ],
            "each loop's role is its place on the face"
        );
        for (lk, members, _) in &by_loop {
            match body.get_loop(*lk).unwrap().boundary {
                LoopBoundary::Empty { vertex } => match members[..] {
                    [BoundaryMember::Isolated { vertex: v, point }] => {
                        assert_eq!(v, vertex, "the lone vertex");
                        assert_eq!(
                            [point.x, point.y, point.z],
                            [tip.x, tip.y, tip.z],
                            "the lone vertex at its point"
                        );
                    }
                    _ => panic!("{lk:?}: a lone-vertex loop yields exactly one `Isolated`"),
                },
                LoopBoundary::Cycle { first } => {
                    let walk = body.loop_walk(first).closed("loop", first);
                    let hes: Vec<HalfEdgeKey> = members
                        .iter()
                        .map(|m| match *m {
                            BoundaryMember::Edge { he, half, ek, .. } => {
                                assert_eq!(half.edge, ek, "{he:?}: its own edge");
                                he
                            }
                            BoundaryMember::Isolated { .. } => {
                                panic!("{lk:?}: a cycle yields no `Isolated`")
                            }
                        })
                        .collect();
                    assert_eq!(hes, walk, "{lk:?}: the members are the loop's walk");
                }
            }
        }
        let flat: Vec<Option<VertexKey>> = view
            .boundary()
            .map(|m| match m {
                BoundaryMember::Isolated { vertex, .. } => Some(vertex),
                BoundaryMember::Edge { .. } => None,
            })
            .collect();
        let per_loop: Vec<Option<VertexKey>> = by_loop
            .iter()
            .flat_map(|(_, ms, _)| ms.iter())
            .map(|m| match m {
                BoundaryMember::Isolated { vertex, .. } => Some(*vertex),
                BoundaryMember::Edge { .. } => None,
            })
            .collect();
        assert_eq!(
            flat, per_loop,
            "the flat view is the by-loop view flattened"
        );
        assert_eq!(
            flat.iter().flatten().count(),
            1,
            "one lone vertex on the face"
        );
    }

    /// A member half-edge whose edge record is gone panics naming the
    /// half-edge's `edge` link and its premises, rather than skipping
    /// the member.
    #[test]
    fn the_face_boundary_panics_on_a_member_whose_edge_is_gone() {
        let (mut body, face, [outer, ..], _) = three_ring_face();
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("fixture: a cycle outer loop")
        };
        let ek = body.get_half_edge(first).unwrap().edge;
        body.edges.remove(ek);
        let named = format!(
            "{}'s edge names {}, which does not resolve",
            EntityId::HalfEdge(first),
            EntityId::Edge(ek)
        );
        crate::review_d18::assert_torn_op_panics(
            "FaceView::boundary, a dead edge",
            &mut body,
            &[&named, super::NAMES_ONLY_LIVE, super::OPERATORS_KEEP_LINKS],
            |b| b.proven_face(face).boundary().count(),
        );
    }

    /// A lone vertex whose point is gone panics naming the vertex's
    /// `point` link and its premises, rather than skipping the loop.
    #[test]
    fn the_face_boundary_panics_on_a_lone_vertex_whose_point_is_gone() {
        let (mut body, face, [_, _, tip, _], _) = three_ring_face();
        let LoopBoundary::Empty { vertex } = body.get_loop(tip).unwrap().boundary else {
            panic!("fixture: a lone-vertex ring")
        };
        let pk = body.get_vertex(vertex).unwrap().point;
        body.points.remove(pk);
        let named = format!("{}'s point names", EntityId::Vertex(vertex));
        crate::review_d18::assert_torn_op_panics(
            "FaceView::boundary, a dead point",
            &mut body,
            &[&named, super::NAMES_ONLY_LIVE, super::OPERATORS_KEEP_LINKS],
            |b| b.proven_face(face).boundary().count(),
        );
    }

    /// **A face's loops are spelled in one order, in one place**
    /// ([`crate::entity::Face::loop_keys`]): no statement in this crate
    /// chains a record's `outer` loop onto its `rings` but a type's own
    /// method over its own fields (`self.outer`). That chain is the shape
    /// every inline walk of a face's loops took; a walk of a body's face
    /// reads a [`FaceView`], and a reader
    /// that reports a miss itself reads `Face::loop_keys`.
    ///
    /// Blind to a walk that reads `outer` and `rings` in separate
    /// statements, which no shape of text tells from a read of the outer
    /// loop alone and then of the rings alone.
    #[test]
    fn a_faces_loops_are_walked_in_one_place() {
        let mut copies = Vec::new();
        for file in crate_sources() {
            let code = CodeOnly::of(&std::fs::read_to_string(&file).expect("a readable source"));
            for statement in code.as_str().split([';', '{', '}']) {
                let flat: String = statement.split_whitespace().collect();
                let holders = |field: &str| -> Vec<String> {
                    flat.match_indices(field)
                        .map(|(at, _)| at)
                        .filter(|&at| {
                            flat[at + field.len()..]
                                .chars()
                                .next()
                                .is_none_or(|c| !(c.is_alphanumeric() || c == '_'))
                        })
                        .map(|at| {
                            let head = &flat[..at];
                            let start = head
                                .rfind(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.'))
                                .map_or(0, |i| i + 1);
                            head[start..].to_owned()
                        })
                        .collect()
                };
                let (outer, rings) = (holders(".outer"), holders(".rings"));
                let chained = flat.contains("chain(")
                    && outer.iter().any(|h| h != "self" && rings.contains(h));
                if chained {
                    copies.push(format!("{}: {}", file.display(), statement.trim()));
                }
            }
        }
        assert!(
            copies.is_empty(),
            "a face's loops walked beside `Face::loop_keys` / `FaceView::loops`: \
             {copies:#?}"
        );
    }

    /// **A face's raw loop keys are read only where a miss is reported**:
    /// [`crate::entity::Face::loop_keys`] and `loop_roles` hand back keys
    /// unresolved, so a walk that resolves each one itself (`get_loop(lk)?`)
    /// would answer a torn link however it pleased. Outside the home they
    /// are read only by the files below, each of which reports or hands
    /// on a miss itself.
    #[test]
    fn loop_keys_is_read_only_where_a_miss_is_reported() {
        const READERS: &[(&str, &str)] = &[
            ("entity.rs", "the order's home"),
            ("live.rs", "the walks' home"),
            ("validate.rs", "the validator reports every miss"),
            ("coherence.rs", "the chart coherence check reports a miss"),
            (
                "census.rs",
                "the tier-3 census hands an unwalkable loop back",
            ),
            (
                "props.rs",
                "each loop goes to a loop door that refuses Corrupt",
            ),
            (
                "euler.rs",
                "find_half_edge is total by its contract; site_face reads keys only",
            ),
            ("fixtures.rs", "the torn-body fixtures report a dead loop"),
            (
                "review_m0_pr7.rs",
                "a census of a body the validator has not certified",
            ),
        ];
        let strays = read_outside(&["loop_keys()", "loop_roles()"], READERS);
        assert!(
            strays.is_empty(),
            "a face's loops resolved outside the home: {strays:#?}"
        );
    }

    /// **The explicitly unchecked walk of a face's cycles is read only
    /// where a miss is reported**: [`Body::unchecked_face_cycles`] answers
    /// a torn loop `Err(loop)` rather than naming the hop, which is right
    /// only for a reader over a body no door has certified.
    #[test]
    fn the_unchecked_cycles_are_read_only_where_a_miss_is_reported() {
        const READERS: &[(&str, &str)] = &[
            ("live.rs", "the walks' home"),
            (
                "census.rs",
                "the tier-3 census hands an unwalkable loop back",
            ),
            (
                "link.rs",
                "the census's touch readers refuse an unwalkable loop as Corrupt",
            ),
        ];
        let strays = read_outside(&["unchecked_face_cycles("], READERS);
        assert!(
            strays.is_empty(),
            "a face's cycles read unchecked outside a reader that reports a miss: {strays:#?}"
        );
    }

    /// The crate sources outside `readers` whose code reads any of
    /// `reads`, each as a method (`.read`).
    fn read_outside(reads: &[&str], readers: &[(&str, &str)]) -> Vec<std::path::PathBuf> {
        crate_sources()
            .into_iter()
            .filter(|f| {
                let p = f.to_string_lossy().replace('\\', "/");
                let p = p.rsplit("/src/").next().unwrap_or(&p).to_owned();
                !readers.iter().any(|(r, _)| p == *r)
                    && reads.iter().any(|read| {
                        CodeOnly::of(&std::fs::read_to_string(f).expect("a readable source"))
                            .as_str()
                            .contains(&format!(".{read}"))
                    })
            })
            .collect()
    }

    /// A declined cube and its first face, to tear.
    fn torn_cube() -> (Body<f64>, FaceKey) {
        let body = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().unwrap().0;
        (body, face)
    }

    /// **A stale face key is `face_surface`'s one `None`**; a live face
    /// whose surface link no longer resolves is a kernel bug, and panics
    /// naming the link.
    #[test]
    fn face_surface_answers_a_stale_key_none() {
        let (mut body, face) = torn_cube();
        assert!(body.face_surface(face).is_some(), "a live face reads");
        body.faces.remove(face);
        assert!(body.face_surface(face).is_none(), "a stale key reads None");
    }

    #[test]
    #[should_panic(expected = "'s surface names")]
    fn face_surface_panics_on_a_dangling_surface_link() {
        let (mut body, face) = torn_cube();
        let surface = body.get_face(face).unwrap().surface;
        body.surfaces.remove(surface);
        let _ = body.face_surface(face);
    }

    /// **`FaceView::edges` panics on a torn loop or a torn mate**, the
    /// links its walk reads.
    #[test]
    #[should_panic(expected = "'s outer names")]
    fn face_edges_panic_on_a_torn_loop() {
        let (mut body, face) = torn_cube();
        let outer = body.get_face(face).unwrap().outer;
        body.loops.remove(outer);
        let _ = body.proven_face(face).edges();
    }

    #[test]
    fn face_edges_panic_on_a_torn_mate() {
        let (mut body, face) = torn_cube();
        let edge = body.proven_face(face).edges()[0];
        let claim = body.get_edge(edge.edge).unwrap().claim(edge.half).unwrap();
        assert_eq!(
            claim.mate, edge.mate,
            "fixture: the walk's mate is the edge's"
        );
        body.half_edges.remove(edge.mate);
        let named = format!(
            "{}'s {} names {}, which does not resolve",
            EntityId::Edge(edge.edge),
            claim.mate_field(),
            EntityId::HalfEdge(edge.mate)
        );
        crate::review_d18::assert_torn_op_panics(
            "FaceView::edges, a dead mate",
            &mut body,
            &[&named, super::NAMES_ONLY_LIVE, super::OPERATORS_KEEP_LINKS],
            |b| b.proven_face(face).edges().len(),
        );
    }

    /// **A stale face key is answered once, by its source**: through
    /// [`Body::proven_face`] it panics naming the key and the premise it
    /// breaks, through [`Body::linked_face`] naming the record that holds
    /// it, and through [`Body::passed_face`] it is `None`, the caller's
    /// to refuse.
    #[test]
    fn a_stale_face_key_is_answered_by_its_source() {
        let (mut body, face) = torn_cube();
        let outer = body.get_face(face).unwrap().outer;
        assert!(body.passed_face(face).is_some(), "a live face resolves");
        body.faces.remove(face);
        assert!(
            body.passed_face(face).is_none(),
            "a passed stale key is None"
        );
        let proven = format!(
            "{}, which this call minted and kept, resolved, or read out of a record, does not \
             resolve",
            EntityId::Face(face)
        );
        crate::review_d18::assert_torn_op_panics(
            "proven_face",
            &mut body,
            &[&proven, super::NAMES_ONLY_LIVE, super::OPERATORS_KEEP_LINKS],
            |b| b.proven_face(face).key(),
        );
        let linked = format!(
            "{}'s face names {}, which does not resolve",
            EntityId::Loop(outer),
            EntityId::Face(face)
        );
        crate::review_d18::assert_torn_op_panics(
            "linked_face",
            &mut body,
            &[&linked, super::NAMES_ONLY_LIVE, super::OPERATORS_KEEP_LINKS],
            |b| b.linked_face(face, EntityId::Loop(outer), "face").key(),
        );
    }
}
