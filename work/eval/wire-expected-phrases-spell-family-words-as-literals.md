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
