---
id: a-failed-boolean-hides-its-operands-and-marks-no-failure-site
kind: issue
title: viewer: a failed boolean draws nothing, so its operands disappear, and the failure site it names is not marked
status: open
opened: 2026-11-22
priority: P1
cost: H
design: true
branch: vgeom/ev-failed-picture
---

**Ev reported this** (in chat, 2026-11-22): "when a boolean fails, it'd be cool if the parts did not disappear but instead highlighted where the problem was".

`Picture::of` / `pieces` (`crates/viewer/src/world.rs:451-478`) draws a boolean as itself, and only `Assemble` expands into operands. A failed boolean therefore draws nothing, and its operands, which it consumes, are not drawn either (`crates/viewer/src/display.rs:304`: "did not evaluate, so there is nothing of it").

Many `topo::BooleanError` variants already carry a site in operand-local keys, for example `CurvedBooleanUnsupported { operand, face: FaceKey, kind }` (`crates/topo/src/boolean/mod.rs:2973-2980`), `DegenerateTorus { operand, face, .. }`, and `PoisonedCarrierDatum { pair: [(Operand, FaceKey); 2], .. }` (`:3479-3485`). But nothing returns the cited entities uniformly (`impl BooleanError` at `:4802`, `kind()` at `:5043`, `:5142`, `:5171`). The evaluator wraps the error bare (`EvalError::Boolean(BooleanError)`, `crates/editor-core/src/eval/mod.rs:1575`), where `Blend` translates its edge to a `StableName` (`eval/mod.rs:1564-1573`). The viewer marks no refusal site for any op.

Three pieces:

1. The viewer draws a failed boolean's evaluated operands, styled as the failed node's inputs, in place of nothing. This needs a ruling on how that reads beside the D-rules on what a node's picture is, hence `design: true`.
2. The kernel adds a `BooleanError` accessor for the faces, edges or points it cites. This is TOPO's ground.
3. The evaluator translates those to stable names as `Blend` does, and the viewer marks them with the held-pick mark lanes (`crates/viewer/src/marks.rs`).

Pieces 2 and 3 need kernel rows filed by whoever specs this one. The words half is `a-failed-boolean-says-so-only-in-its-tree-row`.

## Weighed (2026-11-25)

Two designers agreed on the shape: the hiding rule reads the landed
run (a body is intermediate only while every reader has a value), the
bodies a no-value node reads are drawn as stand-ins with a mark of
their own, and one uniform site (stable names, each with an optional
point) is translated by the evaluator from a kernel accessor and lit in
a site lane of its own. Scope is every body-consuming node, not only
booleans. They split on whether the stand-in rule is stated in GQ2 or
only in the viewer README. Put to Ev on the `vgeom/ev-failed-picture`
PR. Kernel and evaluator rows are filed once Ev rules.
