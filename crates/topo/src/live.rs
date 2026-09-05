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
//! which is the validator's business and not liveness.
//!
//! # Guarding
//!
//! That no `Live` exists without a lookup rests on this file's privacy,
//! which **the crate's usual instrument cannot check** — a
//! `compile_fail` doctest cannot name a `pub(crate)` type — so it is
//! checked as source instead, by this module's
//! `every_door_that_hands_out_a_live_looks_up_first`: the field and
//! `Live::new` carry no visibility, every door whose return type hands
//! a `Live` out reaches a lookup before it builds one, the doors and
//! the construction sites are exactly the ones named here, and no other
//! file in `topo/src` builds a `Live` at all.
//!
//! # The other arenas
//!
//! A plan phase that only needs to REFUSE a stale key — no proof to
//! carry into a splice — calls [`require_key`], which lives here for
//! the same reason `Live` does: one statement of what a liveness check
//! is and which [`EntityId`] a failed one names.
use crate::body::Body;
use crate::entity::{EntityId, HalfEdge, HalfEdgeKey};
use crate::euler::EulerOpError;
use geom_core::Real;

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

/// Requires a key to be live in its arena, refusing a stale one as the
/// plan phase's typed error — the obligation [`Body::require_live`]
/// carries for half-edges, for the arenas that have no proof token.
///
/// Only half-edges are spliced through a key the caller holds across a
/// `&mut` call, so only they need a [`Live`] to carry the lookup
/// forward; every other arena's plan phase wants the refusal and
/// nothing else. One body for all of them, so the check and the
/// [`EntityId`] it names cannot drift arena by arena.
pub(crate) fn require_key<K: slotmap::Key, V>(
    arena: &slotmap::SlotMap<K, V>,
    key: K,
    id: fn(K) -> EntityId,
) -> Result<(), EulerOpError> {
    if arena.contains_key(key) {
        Ok(())
    } else {
        Err(EulerOpError::StaleKey { key: id(key) })
    }
}

impl<T: Real> Body<T> {
    /// Requires a half-edge key to be live, refusing a stale one as the
    /// plan phase's typed error.
    ///
    /// This is the plan-phase door: an operator that will splice through
    /// a key it read out of the arena proves it here, **before any
    /// mutation**, so the mutation phase below cannot fail midway
    /// (atomicity).
    pub(crate) fn require_live(&self, he: HalfEdgeKey) -> Result<Live, EulerOpError> {
        Live::of(self, he).ok_or(EulerOpError::StaleKey {
            key: EntityId::HalfEdge(he),
        })
    }

    /// [`Body::resolve_half_edge`] keeping the proof its lookup earns,
    /// for an operator that both reads a half-edge's fields and splices
    /// through the key itself.
    ///
    /// The proof comes out of the same `Some` arm the fields do, so this
    /// door looks up exactly once — an operator that resolved and then
    /// required separately would carry a refusal its own resolve had
    /// already made unreachable.
    pub(crate) fn resolve_half_edge_live(
        &self,
        he: HalfEdgeKey,
    ) -> Result<(Live, HalfEdge), EulerOpError> {
        match self.half_edges.get(he) {
            Some(data) => Ok((Live::new(he), data.clone())),
            None => Err(EulerOpError::StaleKey {
                key: EntityId::HalfEdge(he),
            }),
        }
    }

    /// [`Body::loop_cycle`] with its members proven.
    ///
    /// The walk resolves every member it returns, so this costs one
    /// redundant lookup per member and adds no failure mode: `None` here
    /// means the walk itself failed.
    pub(crate) fn loop_cycle_live(&self, he: HalfEdgeKey) -> Option<Vec<Live>> {
        self.loop_cycle(he)?
            .into_iter()
            .map(|member| Live::of(self, member))
            .collect()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::Live;
    use crate::body::Body;
    use crate::entity::{EdgeKey, HalfEdge, HalfEdgeKey, LoopKey, VertexKey};
    use crate::fixtures::pillow;
    use crate::source_walk::{CodeOnly, crate_sources, src_root};
    use geom_core::Tol;
    use test_utils::source::balanced_end;

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
        assert!(empty.require_live(HalfEdgeKey::default()).is_err());
        let body = pillow(Tol::witness()).body;
        assert!(!body.half_edges.is_empty(), "the arena must be populated");
        assert!(Live::of(&body, HalfEdgeKey::default()).is_none());
        assert!(body.require_live(HalfEdgeKey::default()).is_err());
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
        assert!(body.require_live(dead).is_err());
    }

    /// The spellings that count as *this door resolved the key*: a
    /// read of the arena, or a call to a door this row itself pins to
    /// have performed one.
    ///
    /// **Closed on purpose.** A door whose lookup is spelled some other
    /// way matches nothing here and reds; it does not pass. Treating an
    /// unrecognised spelling as a lookup would pass a door that looks
    /// nothing up, which is the one thing this row exists to catch, so
    /// a new spelling is added here by somebody who has read the door.
    const LOOKUPS: [&str; 6] = [
        "contains_key(", // the arena's membership test
        ".get(",         // the slotmap read whose `Some` arm carries the fields
        "get_half_edge", // the named accessor over that read
        "loop_cycle(",   // the bounded walk, which resolves every member it yields
        "require_key(",  // the shared refusal
        "Live::of(",     // delegation to a door this same row pins
    ];

    /// Every spelling that builds a `Live` from a bare key. `Live` and
    /// `Self` both, because inside `impl Live` the constructor answers
    /// to either name.
    const CONSTRUCTIONS: [&str; 4] = ["Live::new(", "Self::new(", "Live(", "Self("];

    /// The doors that hand a `Live` out, in source order — the list the
    /// module header states.
    const DOORS: [&str; 4] = [
        "of",
        "require_live",
        "resolve_half_edge_live",
        "loop_cycle_live",
    ];

    /// The items that build one, in source order. `new` is the
    /// constructor itself; the other two are doors that have just
    /// completed a lookup.
    const BUILDERS: [&str; 3] = ["new", "of", "resolve_half_edge_live"];

    /// The 1-based line of byte `at` in `src`.
    fn line_of(src: &str, at: usize) -> usize {
        src[..at].bytes().filter(|c| *c == b'\n').count() + 1
    }

    /// Whether `text` names `name` as a whole token — `Live` in
    /// `-> Option<Live>`, and not the tail of some `NotLive`.
    fn mentions(text: &str, name: &str) -> bool {
        let identish = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
        let b = text.as_bytes();
        text.match_indices(name).any(|(at, _)| {
            (at == 0 || !identish(b[at - 1]))
                && b.get(at + name.len()).is_none_or(|c| !identish(*c))
        })
    }

    /// **The guard the module header names.**
    ///
    /// The compiler already carries half the claim: the field is
    /// private, so the tuple constructor and `Live::new` are unnameable
    /// outside this module and no other file could compile a
    /// construction. The other half — that every door HERE looks the
    /// key up before it builds — is a fact about four function bodies,
    /// and the crate's usual instrument cannot reach it, a
    /// `compile_fail` doctest being unable to name a `pub(crate)` type.
    /// So it is checked as source, over the shared lexer's code-only
    /// view, in which every comment and every literal is spaces and a
    /// match is therefore code.
    ///
    /// Three parts over this file, and the compiler's half restated
    /// over the rest of the crate:
    ///
    /// 1. The declaration stays private in both halves: no visibility
    ///    on the field, none on `Live::new`.
    /// 2. Every item whose return type hands a `Live` out reaches a
    ///    lookup before it builds one. `Live::new` is the exception,
    ///    and part 1 is why it is allowed to be.
    /// 3. The doors and the construction sites are exactly the ones the
    ///    header lists, so a fifth door reds here rather than arriving
    ///    unread.
    /// 4. No other file in `topo/src` builds a `Live` at all — which is
    ///    what makes the four doors the ONLY way to obtain one.
    ///
    /// **What it cannot see**, inherited from a walk that reads text:
    /// a lookup reached one hop away through a helper reads as no
    /// lookup (a red, which is the safe direction); a construction
    /// inside a `macro_rules!` body is text like any other; and `cfg`
    /// is not evaluated.
    #[test]
    fn every_door_that_hands_out_a_live_looks_up_first() {
        let path = src_root().join("live.rs");
        let text = std::fs::read_to_string(&path).expect("this module's own source reads back");
        let code = CodeOnly::of(&text);
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
                "the `Live` field is `{field}`: a public field is a constructor, and every \
                 crate that can name the type could then forge a proof"
            ));
        }
        let new = items
            .iter()
            .find(|item| item.name == "new")
            .expect("`Live::new`, the one place a `Live` is built from a bare key");
        if new.lead.contains("pub") {
            violations.push(format!(
                "`Live::new` is declared `{}`: a caller outside this module could then \
                 build a `Live` from a key nothing resolved",
                new.lead.trim()
            ));
        }

        // 2. Every door looks up before it builds.
        let mut doors: Vec<&str> = Vec::new();
        for item in &items {
            let name = item.name;
            if name == "new" || !(mentions(item.returns, "Live") || mentions(item.returns, "Self"))
            {
                continue;
            }
            doors.push(name);
            let built = CONSTRUCTIONS
                .iter()
                .filter_map(|n| item.body.find(*n))
                .min();
            let looked = LOOKUPS.iter().filter_map(|n| item.body.find(*n)).min();
            match (looked, built) {
                (None, _) => violations.push(format!(
                    "`{name}` hands out a `Live` and reaches no lookup this guard knows. \
                     The vocabulary is {LOOKUPS:?} — a door that resolves its key some \
                     other way is a spelling to add there deliberately, never one to pass \
                     unread."
                )),
                (Some(lookup), Some(build)) if build < lookup => violations.push(format!(
                    "`{name}` builds a `Live` at line {} before it looks the key up at \
                     line {}",
                    line_of(src, item.span.start + build),
                    line_of(src, item.span.start + lookup),
                )),
                _ => {}
            }
        }

        // 3. The doors and the construction sites are the listed ones.
        let mut sites: Vec<(usize, &str)> = Vec::new();
        for needle in CONSTRUCTIONS {
            for (at, _) in src.match_indices(needle) {
                match items.iter().find(|item| item.span.contains(&at)) {
                    Some(item) => sites.push((at, item.name)),
                    // The declaration itself, which part 1 reads.
                    None if src[..at].trim_end().ends_with("struct") => {}
                    None => violations.push(format!(
                        "a `Live` is built at line {} of live.rs, outside every item the \
                         scan read — nothing here can say which door it belongs to",
                        line_of(src, at)
                    )),
                }
            }
        }
        sites.sort_unstable();
        let builders: Vec<&str> = sites.into_iter().map(|(_, name)| name).collect();
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
            for needle in ["Live(", "Live::new("] {
                if other.as_str().contains(needle) {
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
}
