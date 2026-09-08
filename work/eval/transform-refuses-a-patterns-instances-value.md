---
id: transform-refuses-a-patterns-instances-value
kind: issue
title: Node::Transform takes one body, so a transform over a pattern refuses WrongOperand although one rigid map of N instances is well-defined
status: spec
opened: 2026-09-05
branch: eval/6-placers-over-instances
---


Found by MSOLVE-1 (PR 1929, deviation 3, confirmed by its correctness
review NOTE-6); filed by the MSOLVE orchestrator. The node vocabulary
is DOCM's ground; the mate consequence is MSOLVE's.

`wire_transform` (`crates/editor-core/src/eval/wire.rs`) takes one
body through `body_operand`; a pattern's value is `Instances`, so
`Transform { input: pattern }` refuses `WrongOperand { expected:
"body", found: "instances" }`. The instantiate door already says one
rigid map carries every solid of a multi-solid value ("a rigid map of a
body is a rigid map of every solid in it"), so the refusal is a fence
of the operand check, not of the math. Consequence for mates: the
member walk admits transform-of-pattern and the solve places it, but
no such document evaluates; MSOLVE-1's row pins both halves. Decide
whether `Transform` (and by the same argument the other single-body
placers) accepts `Instances`, or whether the fence is intended and the
walk should refuse the shape in the mate's own voice.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/eval/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). `eval/wire.rs`'s operand check is EVAL's; whether `Transform` accepts `Instances` is a node-vocabulary decision DOCM shares, so the `[ev]` PR that asks it is announced on DOCM's board.

## Question for Ev (EVAL, shared with DOCM, 2026-09-08)

**Do the single-body placers — `Node::Transform` and `Node::Pattern` —
accept an `Instances` value, or is the fence intended?** Today
`wire_transform` and `wire_pattern` (`crates/editor-core/src/eval/wire.rs`)
take their input through `body_operand`, which admits a `Body` or a
boolean's non-empty result and refuses everything else typed
(`WrongOperand { expected: "body" }`). A pattern's value is
`Instances(Vec<Arc<Body>>)` — N unfused bodies, by D3 — so
`Transform { input: pattern }` and `Pattern { input: pattern }` both
refuse. The mate side already answers the other way: MSOLVE-1 and
MSOLVE-2 made the member walk admit transform-of-pattern and nested
patterns, and the solve places them, so a document the solver reasons
about is one the evaluator refuses (pinned both ways by
`msolve1_transform_aware.rs::a3_pattern_of_transform_seats_and_transform_of_pattern_resolves`
and `::a10_a_nested_pattern_head_is_a_member`). DM3 (`Node::Part`,
DOCM-2) already lets a single instance be selected and then placed, so
this question is only about placing the WHOLE value.

The math does not need the fence: a rigid map of N bodies is N rigid
maps, `transform_rigid` is key-stable, and the transform contributes no
`RolePath` segment (identity-preserving pass-through, D2), so over
`Instances` the `Instance(i)` names would pass through verbatim exactly
as a body's do. And the sentence is an ordinary one: "move this whole
array" is not "pattern the moved master" — a pattern's direction or
axis is read from a datum in document frame, so pattern-of-transform
and transform-of-pattern are different placements whenever the map
rotates.

Three answers, with a recommendation.

**1. The placers are shape-preserving over the value: `Body → Body`,
`Instances → Instances` (RECOMMENDED).** `Transform` over `Instances`
applies its one map to every body and yields `Instances`, names
passing through; `Pattern` over `Instances` yields `Instances` of
N·M bodies with `Instance(i)` wrapping per outer index over the inner
names — the member chain MSOLVE-2 already walks. No node changes its
payload, no schema moves; what changes is `body_operand`'s neighbour
(a `placeable_operand` admitting both kinds), two `wire_*` arms, and
the viewer's seat gate `viewer::combine::denotes_body`, which tracks
the evaluator's door by design and widens for the two placers
(CHROME's file, one arm). The mate walk's admission becomes true of
the document. Against it: `Transform` stops meaning "a body op" and
becomes "a map of a value", which is a vocabulary statement DOCM owns;
and `Boolean` still refuses `Instances`, so the widening is not
uniform across consumers — which is right (a boolean of N bodies is N
booleans or one union, and D3 refuses to guess) but must be said at
`ValuePayload::Instances`'s doc so the asymmetry reads as a decision.

**2. The fence is intended: single-body placers stay single-body, and
the mate walk refuses the shape in its own voice.** Cheapest; the
evaluator is unchanged. MSOLVE-1's row then owes a typed refusal at
the walk (`DanglingHead` or a new arm naming the placer) so the solver
and the evaluator agree, and "move this whole array" is spelled by
moving the pattern's datum and master together — which is a
two-edit sentence for a one-map idea, and is exactly the friction the
demos keep filing. Against it: it ratifies a fence the math does not
need, and the mate side has already built the other answer.

**3. A new multi-body placement node** (`Node::Place` or a
`Transform` payload variant carrying "apply to every instance").
Rejected: it says with a new word what answer 1 says with the value's
own shape, costs the nine-arm node registration `crates/editor-core/README.md`
prices, and leaves `Transform` over `Instances` refusing beside a node
that does the same thing.

**What this asks of you:** answer 1 or 2 (3 is recorded so it is not
re-derived). Under 1, EVAL builds the evaluator side as a small unit
on its own slate, announces the `denotes_body` arm to CHROME, and
MSOLVE's pins flip from "refuses" to "evaluates"; under 2, EVAL files
the walk-side refusal on MSOLVE's board and closes this row as
ratified. Either is one small PR.

## Ruled (Ev, PR 2137, 2026-09-08)

Answer 1: **the placers are shape-preserving over the value** —
`Transform` and `Pattern` accept `Instances` and yield `Instances`; no
payload or schema change; `Boolean` still takes one body, and that
asymmetry is stated at `ValuePayload::Instances`. This row becomes the
unit that executes it: `docs/EVAL-6-SPEC.md`, branch
`eval/6-placers-over-instances`, with a correctness arm. The viewer's
seat gate (`denotes_body`) is CHROME's and is announced to them in the
spec; MSOLVE's two pins flip from "refuses" to "gathers".
