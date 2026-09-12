---
id: wire-expected-phrases-spell-family-words-as-literals
kind: issue
title: wire.rs's operand refusals spell family words as string literals in expected: beside the family consts kind_name and node_value_kind share
status: closed
opened: 2026-09-08
pr: 2376
branch: wire/family-consts
closed: 2026-09-11
---


(EVAL orchestrator) From EVAL-11's fix pass (PR 2195). That unit gave
the value-family vocabulary one home — `eval::family`'s consts beside
`ValuePayload::kind_name` (`crates/editor-core/src/eval/mod.rs`), used
by `kind_name`, `node_value_kind` and `wire::body_operand`'s `found:`
— and left the rest of `wire.rs`'s operand refusals spelling the same
words as literals in `expected:`: nine bare family words (`"body"`,
`"profile"` ×3, `"instances"`, `"declarations"`, `"measure"`,
`"split"`) and seven composed phrases (`"datum axis"`, `"body or
instances"`, …). The bare ones are the `family` consts by another
route; the composed ones need a small composer or stay prose. Q1
class: one vocabulary, two spellings. EVAL's file; re-homed by the
exit walk if EVAL closes first.

## Re-homed to WIRE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

WIRE is the evaluation seat's successor. `docs/DOC-LEDGER.md` sweep 9
parked three of these rows in `work/issues/` as "the successor's opening
slate" when EVAL closed on 2026-09-08, naming two more already there;
this program is that successor, and it takes the rest of the seat's
ground with them.

Its class at the cut was **E** — nine literals swap to existing consts;
the seven composed phrases are licensed to stay prose. The class is a
dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.

## Closed (2026-09-11) — PR 2376, merged

Seven `expected:` literals in `crates/editor-core/src/eval/wire.rs` now
read `super::family::*`. Refusal text is **byte-identical** at all seven
sites, and the suite that asserts over those strings was run rather than
argued about (1201 passed).

**This item's counts were wrong in both directions** and the lane
corrected the row: seven bare family words, not nine — and the item's own
parenthetical listed eight for a claimed nine, so it was internally
inconsistent before the tree moved; eight composed SITES (six distinct
strings), not seven. The missing "profile" is `"profile node"` at
`:4191`, a composed phrase on the other list.

**No composer**, and the argument is accepted: the composed phrases are
not family words (`"datum frame"` names a variant *within* a family,
whose `found:` answers `"datum"` on purpose, so a const would spell
`"datum"` twice one level up); `concat!` takes literals not consts and
the tree carries no compile-time string concatenation.

**Light style review, and it found the doc comment false.** The unit
wrapped its argument in a twenty-line rewrite of `family`'s module doc
which asserted, among other things, that `expected` has no reader
(`wire.rs:2643` is one) and that the old "three readers" sentence was
stale (`git grep "family::" origin/main` returns exactly three). Reverted
to four lines for four under implementer-discipline §4 — comments state
the invariant, the argument's home is the PR description. The lane's own
verdict: *"the rewrite was redundant on its good half and false on its
new half."*

**One adjudication against the lane's reading of the discipline**, kept
here because it changes how the next case is judged: §2's "a predicate
over things fixed at compile time" does **not** cover a pin on a
committed, user-reachable spelling, even when both sides are consts. The
unit's outcome (no new guard) was right by §2's *other* clause — a
behavioural neighbour subsumes it.

### Residues, each with a file

`interrogate-writes-the-family-vocabulary-a-third-time`,
`wire-refusals-answer-found-with-a-negation-of-expected`,
`composed-expected-phrases-are-hand-copied-across-sites`,
`frame-plane-lane-and-axis-frame-are-one-door`,
`wire-rs-module-header-describes-five-sixths-of-the-file`, and
`work/lib/pncad-py-value-refusals-spell-family-words-as-literals.md`.
