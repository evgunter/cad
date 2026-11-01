---
id: the-python-facade-holds-code-that-is-not-a-call-to-the-rust-facade
kind: issue
title: The Python façade holds code that is not a call to the Rust façade: a rest is hand-assembled from a raw mate in Python and the viewer, with no façade gesture
status: open
opened: 2026-10-26
priority: P1
cost: M
refs: [5742]
---


## The rule

Ev, on PR 5742 (2026-10-26): "the python facade should never have code that's not just calling the rust facade, so it should be filed as a thing to fix." `pncad-py` is a projection of `pncad`. Any gesture, convention or decision it makes on its own is a second home: the viewer and a Rust caller cannot reach it, and it drifts.

## The instance that surfaced it

Resting an existing part on a face has no façade gesture, so each caller builds the raw `Mate { on, to }` and decides where the `Flip` goes:
- **Python:** `pncad-py/src/py/mate.rs:60-75` (the `Mate` docstring) teaches the rest as a flip on the target (`PoseDef.flip(PoseDef.plane(...))`), and Python authors write it that way by hand.
- **Viewer:** `viewer/src/matetool.rs:718` builds the same mate itself and flips the target.
- **Sketch-on-face**, the one rest the façade does own (`editor-core/src/edit.rs:6466`, re-exported at `pncad/src/document.rs:105`, called from `pncad-py/src/py/doc.rs:1696`), flips the copy.

- **The viewer's snap** (`viewer/src/session/snap.rs`, `snapped`) restates the value chart to write a fresh mate's values: the least turn onto the mated direction, the nearest orientation, the residual spin about the target's own direction, and which slides the mate leaves. That is gesture logic in the viewer. The façade rest gesture should own it, so the viewer and Python call one door. PR 5742 moved sketch-on-face's own orientation pick into the façade (`Recording::sketch_on_face`, `orient: None`) for the same reason; the snap is the rest of that class.

Before PR 5742's ruling (b), the spin read the flip, so the same contact turned opposite ways depending on which caller spelled it. (b) removes that symptom; the duplication that caused it remains.

## What fixing it is

1. **A façade rest gesture.** "Rest this placement's shape on that face" goes in `pncad` (an editor-core edit beside `sketch_on_face`, re-exported), owning the flip. Python's door calls it, and the viewer's mate tool calls it for a face–face rest. This is placement API, so INTENT builds the gesture, and BIND moves the Python side onto it.
2. **The audit the rule asks for.** Read `crates/pncad-py/src/py/*` for anything beyond marshalling (argument conversion, error projection, tag mapping) and a call into `pncad`: a default chosen, a pose assembled, a convention applied. List each with its `file:line`, and move it into `pncad` or delete it. Conversions that only change representation stay.
3. **A guard, if one is cheap** (Ev, 2026-10-26: the architecture should lead additions to the right place; a gate is the fallback). Either a test that fails when `pncad-py` builds a `Mate`, `PoseDef` or `Node` itself outside a passthrough, or a note in the bindings' README saying where such code belongs. Prefer whichever the audit shows the code actually needs.
