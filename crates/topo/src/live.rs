//! [`Live`] — a half-edge key a lookup has returned.
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
use crate::body::Body;
use crate::entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, GeomRef, HalfEdge, HalfEdgeKey, Loop, LoopKey,
    VertexKey,
};
use crate::euler::{BadArgument, EulerOpError};
use crate::geometry::PointKey;
use crate::null::CurveGeom;
use geom::Surface;
use geom_core::{Point3, Real};

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
/// [`proven`]-miss panic and broken-walk panic ([`crate::body::Walk::closed`])
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
        found.unwrap_or_else(|| dangling_link(self.holder, self.link, key))
    }

    #[track_caller]
    fn answer_geometry<V>(self, found: Option<V>, key: GeomRef) -> V {
        found.unwrap_or_else(|| dangling_link(self.holder, self.link, key))
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
        found.unwrap_or_else(|| unproven(key))
    }

    #[track_caller]
    fn answer_geometry<V>(self, found: Option<V>, key: GeomRef) -> V {
        found.unwrap_or_else(|| unproven(key))
    }

    fn map<V, W>(answer: V, f: impl FnOnce(V) -> W) -> W {
        f(answer)
    }
}

/// The panic for a [`Proven`] key that does not resolve.
#[track_caller]
fn unproven(key: impl core::fmt::Display) -> ! {
    unreachable!(
        "{key}, which this call resolved or read out of a record, does not resolve: \
         nothing removes a record during a plan, and {NAMES_ONLY_LIVE}; {OPERATORS_KEEP_LINKS}"
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
    arena.get(key).unwrap_or_else(|| unproven(id(key)))
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

impl<T: Real> Body<T> {
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

    /// `face`'s chart, a link its record holds.
    #[track_caller]
    pub(crate) fn face_surface_linked(&self, face: FaceKey, data: &Face) -> &Surface<T> {
        self.get_surface(data.surface).unwrap_or_else(|| {
            dangling_link(
                EntityId::Face(face),
                "surface",
                GeomRef::Surface(data.surface),
            )
        })
    }

    /// `edge`'s curve-arena entry, a link its record holds.
    #[track_caller]
    pub(crate) fn edge_curve_linked(&self, edge: EdgeKey, data: &Edge) -> &CurveGeom<T> {
        self.get_curve_geom(data.curve).unwrap_or_else(|| {
            dangling_link(EntityId::Edge(edge), "curve", GeomRef::Curve(data.curve))
        })
    }

    /// `face`'s loops, each a link its record holds: the outer loop, then
    /// its rings. The one order every walk of a face's boundary takes,
    /// and the one spelling of the fields a miss names.
    pub(crate) fn face_loops_linked<'a>(
        &'a self,
        face: FaceKey,
        data: &'a Face,
    ) -> impl Iterator<Item = (LoopKey, &'a Loop)> + 'a {
        core::iter::once((data.outer, "outer"))
            .chain(data.rings.iter().map(|&ring| (ring, "rings")))
            .map(move |(lk, field)| {
                let l = linked(&self.loops, lk, EntityId::Loop, EntityId::Face(face), field);
                (lk, l)
            })
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
        let Some(claim) = linked(
            &self.edges,
            edge,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        )
        .claim(he) else {
            unreachable!(
                "{he:?}'s edge {edge:?} does not claim it in either slot: on a tier-1-valid \
                 body an edge claims the two half-edges that name it"
            )
        };
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
    use super::{Arg, Live};
    use crate::body::Body;
    use crate::entity::{EdgeKey, EntityId, HalfEdge, HalfEdgeKey, LoopKey, VertexKey};
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
}
