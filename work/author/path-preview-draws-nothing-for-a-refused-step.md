---
id: path-preview-draws-nothing-for-a-refused-step
kind: issue
title: viewer: the add-profile path preview draws nothing once any authored step refuses, so the author cannot see what to fix
status: open
opened: 2026-09-18
priority: P0
cost: D
---

Reported by Ev from the viewer (2026-09-18): "arc_fillet_arc and other such paths should try to display something even when invalid, so the user can figure out how to fix them."

**What happens.** `sketch::preview` (`crates/viewer/src/sketch.rs`) draws a chain only when it replays: fully, or, for a chain that has not closed yet (`Transition { verb: None }`), under a provisional `line_to Start`. Every other refusal returns `Err(refusal(..))` for the whole profile, and the viewport shows nothing. A fused step's refusal (`arc_fillet_arc`: a fillet radius that does not fit, an arc spec whose numbers name no arc, a corner that does not resolve) blanks every step before it as well, and those are the steps the author needs to see to judge what the failing numbers should be.

**What is owed.** When a replay refuses at step `k`, draw the longest prefix that does replay, which is `steps[..k]` under the same provisional close the unfinished-chain arm already uses, marked open. Keep the refusal sentence beside it, and mark the drawn tip as the place step `k` failed. For a fused step, the parts that DID resolve (the incoming arc, the arrival carrier) are what would show why the fillet does not fit. Drawing those needs a hook from the driver (the carriers a step bound before it refused), so it is a second, larger step. It is split out as the design item `work/paths/refused-fused-step-reports-no-partial-geometry.md` and waits on that item's ruling. This item's prefix drawing does not. As with the provisional close, nothing about the lattice is re-implemented: the prefix goes through the same `replay`.

