---
id: eval-handles-internal-breaks-three-ways-and-run-op-carries-node-facts-as-loose-options
kind: issue
title: eval/: an internal break is handled three ways a few lines apart, and run_op carries node-kind facts as loose Options beside &Node
status: closed
opened: 2026-10-17
priority: P3
cost: M
closed: 2026-10-19
pr: 5406
branch: wire/eval-internal-breaks
---


Found by the style review of PR 5262 (`wire-endings`), items S6 and S9.
Filed, not fixed there.

## Finding

**1. One class, three handlings.** In `crates/editor-core/src/eval/`
an internal break (the evaluator's own state disagreeing with itself)
is met three different ways, often a few lines apart:

- a panic: `unreachable!` (D9's D2 addendum: a bug observed in a
  branch);
- a typed refusal ending in `geom_core::KERNEL_DEFECT_ENDING`
  (`NodeErrorKind::MissingSlot`, `VerbArity`, `ProfileAnchor`,
  `ProfileRecordMissing`, `NamingError::Emission`);
- a silent default that turns the break into an empty value.

`eval/wire.rs` `split_port` holds all of them within eight lines:
`SplitHalf::of_output_body(..).unwrap_or_else(|| unreachable!("a split
defines a port per half"))`, then `let ValuePayload::Split { .. } =
&value.payload else { unreachable!("a split evaluates to its two
sides") }`, then `value.output(port).cloned().unwrap_or_default()`.
The last is already filed as
`wire-rs-defaults-two-missed-records-to-empty`; this row is the
class. Also: `run_op`'s `Node::Assertion` arm
(`node.payload_reads(doc).unwrap_or_default()` falling into
`unreachable!`), and the `Node::World` arm.

Nothing says which handling a given break gets, so each site picks
one. The fix is a stated rule (which breaks are a panic and which are
a typed refusal) and one pass over `eval/` to apply it. Silent
defaults go, whichever way the rule falls.

**2. Node-kind facts ride as loose `Option`s beside `&Node`.**
`wire::run_op` takes `node: &Node<ProfileProgram>` together with
`payload: Option<&Payload<T>>` and `profile_pre: Option<&ProfilePre>`
(both built in `eval::eval_node`). Each is present for exactly some
node kinds: `profile_pre` for `Node::Profile`, `payload` for the
nodes that evaluate one. The types do not say so, so every consumer
re-checks the pairing: `wire_profile` refuses
`ProfileRecordMissing { loop_: None }` when `profile_pre` is `None`,
and the `Node::Assertion` arm reaches `unreachable!` when `payload`
is. An input enum built in `eval_node`, one arm per node kind
carrying what that kind needs, would make the pairing a type fact.
That would delete these checks rather than adding any.

Constraint (why PR 5262 did not do it): `ProfilePre` is computed
before the memo check, because its decisions are pre-key log
entries. So the enum has to be built at the point `eval_node` now
builds `profile_pre`, not inside `run_op`.

## Closed by PR 5406 (WIRE orchestrator, 2026-10-19)

- Finding 2 is done: `wire::OpInput`, one arm per node kind that carries
  facts, built in `eval_node` before the memo check. It carries the
  profile's form, so the op no longer resolves it twice; the assertion
  payload is a typed pair.
- The silent defaults are gone.
- Finding 1, the rule, came back as a design question. It continues as
  `a-kernel-defect-is-a-panic-or-a-typed-refusal-and-nothing-says-which`
  ([ev] PR 5411); the typed-defect sites are untouched until it is
  answered.
