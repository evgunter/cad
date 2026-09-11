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
//! a `Live` out looks the key up before it builds one and wraps the key
//! it looked up, that row's own tables enumerate the doors and the
//! construction sites so a new one of either reds, and no other file in
//! `topo/src` builds a `Live` at all.
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
// Test-support code: panicking is a test's failure mechanism (L5).
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::Live;
    use crate::body::Body;
    use crate::entity::{EdgeKey, HalfEdge, HalfEdgeKey, LoopKey, VertexKey};
    use crate::fixtures::pillow;
    use crate::source_walk::{CodeOnly, crate_sources, src_root};
    use geom_core::Tol;
    use test_utils::source::{ItemBody, balanced_end, item_body};

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

    /// The spellings that count as *this door resolved the key*: a read
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
    /// method.** A bare `.get(` is answered by a read of any map at
    /// all, so `self.faces.get(f)` would stand as the lookup for a
    /// half-edge nothing resolved.
    const LOOKUPS: [&str; 4] = [
        "half_edges.contains_key(", // the membership test, and nothing else
        "half_edges.get(",          // the read whose `Some` arm carries the fields
        "loop_cycle(",              // the bounded walk, which resolves every member
        "Live::of(",                // delegation to a door this same row pins
    ];

    /// Every spelling that builds a `Live` from a bare key. `Live` and
    /// `Self` both, because inside `impl Live` the constructor answers
    /// to either name.
    const CONSTRUCTIONS: [&str; 4] = ["Live::new(", "Self::new(", "Live(", "Self("];

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

    /// The text inside the parentheses that the call `needle` opens.
    /// Every needle in [`LOOKUPS`] and [`CONSTRUCTIONS`] ends with its
    /// own `(`, so the carve starts at the last byte of the match.
    fn argument<'a>(body: &'a str, needle: &str) -> Option<&'a str> {
        let at = body.find(needle)?;
        let open = at + needle.len() - 1;
        Some(body[open + 1..balanced_end(body, open)?].trim())
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
    /// did rather than which assertion happened to fire first.
    ///
    /// **What it cannot see**, all of it inherited from reading text:
    /// a lookup reached one hop away through a helper reads as no
    /// lookup (a red, which is the safe direction); an item declared
    /// INSIDE a door's body is not scanned as a door of its own, and
    /// reds through the construction census under its host's name; the
    /// argument check compares SPELLINGS, so a rebinding between the
    /// lookup and the construction defeats it, as does a lookup in a
    /// half-edge arena belonging to some other body; a construction
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

        // 2. Every door looks up before it builds, and wraps what it
        //    looked up. `Self` in a return type names whichever impl the
        //    item sits in, so only inside `impl Live` does it mean a
        //    `Live`; `Live` itself means one anywhere.
        let impl_live = match item_body(src, src.find("impl Live").expect("the `impl Live` block"))
        {
            ItemBody::Body(body) => body,
            other => panic!("the `impl Live` block has no body: {other:?}"),
        };
        let mut doors: Vec<&str> = Vec::new();
        for item in &items {
            let name = item.name;
            let hands_out = mentions(item.returns, "Live")
                || (mentions(item.returns, "Self") && impl_live.contains(&item.span.start));
            if name == "new" || !hands_out {
                continue;
            }
            doors.push(name);
            let first = |needles: &[&'static str]| {
                needles
                    .iter()
                    .filter_map(|n| item.body.find(*n).map(|at| (at, *n)))
                    .min()
            };
            match (first(&LOOKUPS), first(&CONSTRUCTIONS)) {
                (None, _) => violations.push(format!(
                    "`{name}` hands out a `Live` and reaches no lookup this guard knows. \
                     The vocabulary is {LOOKUPS:?} — a door that resolves its key some \
                     other way is a spelling to add there deliberately, never one to pass \
                     unread."
                )),
                (Some((lookup, _)), Some((build, _))) if build < lookup => {
                    violations.push(format!(
                        "`{name}` builds a `Live` at line {} before it looks the key up \
                         at line {}",
                        line_of(src, item.span.start + build),
                        line_of(src, item.span.start + lookup),
                    ));
                }
                (Some((_, looked)), Some((_, built))) => {
                    let (resolved, wrapped) =
                        (argument(item.body, looked), argument(item.body, built));
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
