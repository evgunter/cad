---
id: unowned-viewer-refusals-short-of-the-shape-guard
kind: issue
title: editor-core/geom-brep: refusals the viewer draws on unowned ground that state no recourse or open with a label, by the shape guard's census
status: open
opened: 2026-09-29
priority: P2
cost: M
---


(CHROME `refusal-residue`, from the shape guard's structural prefix
and zero-recourse checks.)

## What

`work.py territory` names no owner for `crates/editor-core/src/checks.rs`,
`crates/editor-core/src/names/` or `crates/geom-brep/src/certify.rs`.
The shape guard (`crates/test-utils/src/refusal.rs`) found three things
on that ground.

1. **Check findings with no recourse.** Eight checks-window rows state
   none: `Check/ChartCoherence(meridian closure)`,
   `Check/ChartCoherence(rim)`, the three
   `Check/ChartCoherenceUnexamined(…)` rows, and
   `Check/SeparationUnavailable/Containment(CorruptFace | NoSuchSolid |
   ZeroVolumeBody)`. They are admitted by exact id in
   `crates/editor-core/tests/refusal_concision_chains.rs`
   `FILED_NO_RECOURSE`, under the comment naming this file.
2. **Name lineage refusals open with a label.** `Naming/SplitLineage`
   renders `… does not hold: split lineage of edge EdgeKey(…) cycles: …`,
   which the guard reads as a label (it has no word only a sentence has).
   The row is kernel-keyed, so the key stands. It is admitted by `FILED`.
   (`Naming/FragmentLineage` was its sibling until
   `fuse/face-lineage-at-append` retired it: face division rows are
   checked where they are added, so no reader meets a cycle.)
3. **A reversed interval ends on a short-edge lever.**
   `param-span-lever-says-vanishingly-short-of-a-reversed-span` closes
   it: `CertCheck::ParamSpan`'s lever (`CertCheck::ending`) now names
   the goal, "move the geometry so this edge runs forward over a clear
   length", which is true of a span at zero and of a reversed one, and
   at a build the decision's note calls either a kernel defect.

## Repair shape

For 1 and 2, rewrite at the source to the standard and drop the entry;
the must-fire check turns a stale entry red.

## Escalations that offer a declaration the door cannot take (CHROME triage)

(From the triage in `work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`.) `work.py territory` names no owner for these two files. Each
renders the whole `Indeterminate`, so it ends in
`COINCIDENCE_RECOURSE` ("declare the coincidence, …"), and each
reaches doors that take no declaration:

- `NewellError::Escalated` (`crates/geom-brep/src/newell.rs`): PR
  4629 closes it. The arm now ends in `NO_DECLARATION_RECOURSE`
  through `Indeterminate::undecided`, since no extrude, revolve or
  loft door that reaches it takes a declaration.
- `PcurveMintError::Escalated` (`crates/topo/src/pcurves.rs` near
  :415, `Display` near :476): "the pcurve at half-edge {half_edge:?}
  escalated: {cause}", with an arena key as well. `PinMiss::Escalated`
  becomes this (near :2450). It is wrapped by the loft, revolve, blend,
  split (`SplitError::Pcurves`), shell, transform and STEP import
  errors, none of which take a declaration, and by the Boolean, which
  does.

The repair is `payload()`, a subject in plain words and a routed
recourse, as in `sweep::blend::BlendError::Escalated`'s `Display`.
