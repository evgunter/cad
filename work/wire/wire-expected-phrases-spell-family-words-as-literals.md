---
id: wire-expected-phrases-spell-family-words-as-literals
kind: issue
title: wire.rs's operand refusals spell family words as string literals in expected: beside the family consts kind_name and node_value_kind share
status: open
opened: 2026-09-08
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
