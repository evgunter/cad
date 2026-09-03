---
id: viewer-render-pipeline-creation-untested
kind: issue
title: Nothing exercises viewer render-pipeline creation: a startup panic shipped invisibly
status: open
opened: 2026-09-01
github: 1451
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
