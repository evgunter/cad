---
id: a-citation-in-a-line-comment-is-not-checked
kind: issue
title: A bracketed citation in a // comment looks exactly like a rustdoc-gated one and is checked by nothing, and the repaired badge-door scan's population is not the population its README sentence claims
status: open
opened: 2026-09-21
priority: P3
cost: E
---

## Finding 1 — two spellings, one checked, indistinguishable

`scripts/doc-gate.sh` runs rustdoc with
`rustdoc::broken_intra_doc_links` live, so **every `[`name`]` in a
`///` or `//!` doc comment is a checked citation**: rename the target
and the gate reds. The identical characters in an ordinary `//`
comment are inert text that rustdoc never reads, and the rename
leaves a citation pointing at nothing, silently.

Nothing distinguishes the two at the point of reading, and this
repository leans on citation harder than most — the whole argument
for one home per rule is that consumers cite it.

Population, as of this filing:

- `crates/viewer/src/frame.rs` (~`:1071`), in `Withdrawal`'s
  `Display`: `[`LIST_SEPARATOR`]` and `[`frame_status`]` inside a
  `//` comment. Pre-existing — which is what makes this a class and
  not one lane's slip.
- `crates/viewer/src/pickindex.rs` (~`:1081`), the `parts.is_empty()`
  comment: a backticked `ProductErrorKind::means_no_body` in a `//`
  comment. The other shape of the same gap — an honest spelling that
  is still unchecked, so it rots on a rename exactly as quietly.

A third instance was written and repaired on
`chrome/empty-document-gate` before it landed, which is how the class
was noticed.

**What a taker owes.** A decision, not a sweep: either a gate that
resolves bracketed links in `//` comments too (the population is
small and a script can find them), or a stated convention that `//`
comments use plain backticks and `[`…`]` is reserved for doc
comments, applied in one diff. The second is cheap and the first is
what actually holds. Note that the SECOND instance above is not
repaired by the convention — a plain backtick still rots — so the
convention alone only fixes the misleading appearance, not the
staleness.

## Finding 2 — the badge-door guard and its README sentence are not the same population

`crates/viewer/tests/frame_policy.rs`,
`the_readme_counts_its_two_populations_correctly`, sums four return
spellings: `-> Option<Badge>`, `-> Badge`, `-> Vec<Badge>` and
`-> [Badge`. `crates/viewer/README.md` states the rule as **"every
`frame` function returning `Option<Badge>`"**.

Those disagree in both directions, today, with no drift required:

- A door added as `-> Vec<Badge>` **reds the guard** while the
  README's sentence stays true, because the sentence does not range
  over it. Three of the four spellings contribute zero today and
  exist only to catch a door the README would not count.
- A door added as `-> Result<Badge, _>`, `-> Option<&Badge>`,
  `-> Option<(Badge, _)>`, `-> impl Iterator<Item = Badge>`, through
  a type alias, or with a return type rustfmt broke across lines,
  is counted by **neither**, and the completeness argument the README
  rests on (`Badge`'s fields and its three constructors are private
  to `frame`) still says the population is closed. The argument is
  about who can BUILD a badge; the scan is about how a door SPELLS
  returning one, and the gap between those is where a door hides.

The guard was repaired on `chrome/empty-document-gate` for a
different defect — it counted `-> BadgeSite` as a badge door on a
prefix match — and that repair is orthogonal to this: it made the
four spellings match return types honestly, and did not touch which
spellings are in the set or whether the README's sentence ranges over
the same set.

**What a taker owes.** Either widen the README's sentence to the
property the scan actually tests, or narrow the scan to the sentence
and state at the scan why the other three spellings are not sought.
`docs/prompts/reviewer-style-lane.md` Q6 wants the guard and the
claim to be about one population; today they are about two.

## Filed from outside the fence

Filed under `docs/prompts/implementer-discipline.md` §6 by the lane
for `chrome/empty-document-gate`. Finding 1's sites are in that
lane's own files and are left unfixed deliberately: the repair is a
convention decision for the crate, not a line edit. Finding 2 is
about a guard the lane repaired for an unrelated reason, so it is
disclosed here rather than widened into that unit.

