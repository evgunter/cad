## S54. The "kept in step BY HAND" ladder, which the crate around it has twice repudiated by name

- **Where**: `crates/editor-core/src/eval/wire.rs` (`resolve_selection`,
  `resolve_declarations`); the two sites that cited it as the anti-pattern
  they fixed, `crates/editor-core/src/names/flush.rs:37` and
  `crates/editor-core/src/persist/check.rs:9`; same family at
  `crates/profile/src/path/arc_fillet.rs:21` and
  `crates/pncad-py/src/tests.rs:245`
- **Importance**: medium
- **Confidence**: sure on the structure
- **Raised by**: the detector #641 suggested, run 2026-08-19

**Verdict:** ACCEPTED (Evan, 2026-08-19) — "worth doing. Share it." Executed
by **PR-NUMBER**, below.

**FIXED by PR-NUMBER.** The two doors now walk ONE ladder, a private
`mod ladder` sited between them in `wire.rs`, and the "if you change either
ladder, change both" warning is deleted rather than reworded. The shape that
beat the arity objection is the one the finding's own steelman preferred:
share the RUNGS, not the lookup. `Landing` (`Unique(EntityRef)` / `Tied(u32)`
/ `Absent`) is what a table read produces; `live()` is rung 1 (`NodeGone` with
the deleted-vs-foreign split) and hands back a `Live<'_>` token; `resolve()`
takes that token plus a landing and is rungs 2 and 3 (`Ambiguous` with the tie
witness, `Vanished` with the `NodeChanged` fallback and `last_good: None`).
No closure, no generic over "how to look a name up", one hop from either door.

Each door keeps exactly its own arity, which is what makes the shared version
MORE legible than the duplication rather than less: the fillet door is now
`live` → `landing(target)` → `resolve` → the edge-kind refusal, six lines with
every rung named; the declare door reads its two tables into two landings,
picks a side, and refuses `DeclareBothOperands` itself — the one refusal in
that function that is not N5's.

The rung ORDER, which was the residual hand-coupling a pieces-only extraction
would have left, is enforced by the type system: `Live` is constructible only
by `live()`, so no door can reach rungs 2–3 before the `NodeGone` check that
outranks them (including outranking a door's own refusal — the declare door
asked its side question before rung 1 would have changed one input's answer,
so the order is kept and stated). Nothing is left to keep in step by hand.

Behaviour-identical, arm by arm, including payloads. The pins were checked for
what they actually assert rather than assumed: `m6_5_selection_refusals.rs`
pinned NodeGone/`NodeDeleted`, `Vanished`'s full payload, `Ambiguous` minus
`tie.node`, plus the kind and empty refusals; `m4_pr5_declare.rs` pinned
`Ambiguous`'s payload but `Vanished` by Debug SUBSTRING only, and neither door
pinned the witness site. Both gaps are now closed (declare's `Vanished`
asserts `last_good: None` and `RecipeEdit{NodeChanged}` typed; both doors
assert `tie.node == name.node`). `ForeignNode` stays unpinned at both doors —
the edit door refuses never-existed ids before evaluation, so it is reachable
only across documents; that is a real gap, unchanged by this PR, and it is now
a gap in ONE arm of ONE ladder instead of two. Mutation-checked: flipping
`tie.node` in the shared ladder fails a pin in BOTH suites, which is the
property the duplication did not have.

**The two family members named above were deliberately NOT folded in**, and
neither is this refactor. `arc_fillet.rs:21` restates `fillet_select.rs`'s
ratified justification in prose — the RULE already has one home (that module's
header says so in as many words: "giving the rule one home means the ladder is
stated once … instead of the same paragraph twice"), so what is duplicated is
the paragraph, not the code, and the open question is whether the allowlist
line should cite it instead of restating it. `pncad-py/src/tests.rs:245` is
the family's already-solved instance: the `[lints]` table CANNOT be shared
(the crate cannot inherit `[workspace.lints]` — `unsafe_code = "forbid"`
versus PyO3's generated `unsafe impl`), and the hand-restatement is already
held by a test that breaks the build on drift. Duplication made incapable of
drifting is the outcome, reached mechanically instead of structurally.

**Method note, proposed not adopted.** #641's parent-sense row found its fourth
copy through a comment whose only job was to explain that two spellings were one
rule, which suggests a detector: *a comment that exists to reconcile two
spellings of one rule is evidence the rule needs one home*. Run as
`rg 'BY HAND|kept in (sync|step)|same rule as|mirrors the (implementation|logic|table)'`
over `crates/*/src`, excluding the `bit-identical`/`endpoint-identical`
vocabulary, which is D9's and fenced by [[output-stability-as-justification]].
It found every site above. Adding it to `REVIEW-STYLE-BRIEF.md` §2 would be a
Protocol v5 amendment and so **Evan's to ratify**, not adopted here.
