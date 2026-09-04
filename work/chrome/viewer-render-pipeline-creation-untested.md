---
id: viewer-render-pipeline-creation-untested
kind: issue
title: Nothing exercises viewer render-pipeline creation: a startup panic shipped invisibly
status: closed
opened: 2026-09-01
github: 1451
branch: chrome/viewer-app-feature-ci-coverage
pr: 1755
closed: 2026-09-04
---

## From GitHub issue 1451

Opened 2026-09-01; 0 comments.

**The incident.** Since the GAUTH-2 fix pass (e1f549e), `target/debug/viewer` panicked at startup on **every** adapter: `EdgePass::new` requested a `DepthBiasState` on a `LineList` topology, and wgpu 30's spec-level validation refuses that combination in `create_render_pipeline` ("Depth bias is not compatible with non-triangle topology LineList"). The validation is backend-independent — this was not a lavapipe quirk; the shipped app could not open a window anywhere. It was caught only because the story-suites dispatch tried to take PR screenshots under Xvfb/lavapipe, weeks after it landed.

A fix is on branch `claude/subagent-gui-integration-tests-i153yl` (commit 1d5cd01): the bias relocated into `vs_edge` as a relative clip-z shrink, verified live (app starts, edge marks draw without z-fighting).

**The class.** G1 ratifies that only pixel-painting escapes headless tests — but pipeline **creation** is not pixel painting. Device acquisition, shader-module compilation and every `create_render_pipeline` call are pure construction with typed validation, and nothing in CI runs them: the `all` test binary never builds the `app` feature's GPU passes, and the FreeCAD render lanes render the kernel's output, not the viewer. So the whole family of "the pipeline refuses to build" failures — validation-rule changes on a wgpu upgrade, a bad shader edit, a format mismatch — is invisible until a human launches the app.

**Suggested shape.** A CI smoke row that creates a wgpu device on lavapipe (already the gui-shots recipe's adapter; `mesa-vulkan-drivers` is an apt install) and constructs every pass in `viewer::gpu` — no surface, no frame, no pixels asserted. That row goes red at exactly the seam this incident slipped through. The egui-churn watch (GQ6 re-survey §5) would also gain a mechanical tripwire from it.

(story-suites orchestrator)

## Home

`work/issues/` — viewer GPU passes plus a CI smoke row; both owning programs (GUI, S-QA) are closed and no open program's territory covers `crates/viewer`.

## Fixed (CHROME, 2026-09-04)

`gpu::tests::every_pass_builds_on_a_real_device`
(`crates/viewer/src/gpu.rs:1548`) creates a wgpu device on the
software adapter and constructs every pass — no surface, no frame, no
pixels asserted — over both surface formats.

**Home: a unit test inside `gpu.rs`, not a `tests/` row.** `mod gpu`
is private, `ViewportRenderer` is `pub(crate)`, and `IdPass` and
`EdgePass` are bare-private. An integration row could reach them only
by publishing the renderer's internals to serve a test. Nothing was
widened.

**No adapter is a FAILURE, never a skip.** `request_adapter` returning
`Err` panics with a message naming `mesa-vulkan-drivers`. A
skip-if-absent row would print the same thing whether the pipelines
built or the driver was missing, which is the defect this program
exists to close, one layer up. The cost is that
`cargo nextest run -p viewer --features app` is red on a box with no
Vulkan ICD; exactly two invocations run it, and both install one.

**Verified by negative control, both taken and reverted.** Mutating
the edge pass back to `LineList` plus a non-default `DepthBiasState`
turns the row red with the shipped incident's own text — *"Depth bias
is not compatible with non-triangle topology LineList"*. Pointing
`VK_DRIVER_FILES` at a nonexistent ICD turns it red at the adapter,
not green. So the row goes red for the failure it was written for AND
for the absence that would otherwise fake a pass.

The `EdgePass` fix itself had already landed (`gpu.rs:334`,
`EDGE_CLIP_Z_SHRINK` in the vertex shader); this unit is the row.
