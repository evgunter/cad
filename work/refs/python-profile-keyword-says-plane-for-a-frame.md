---
id: python-profile-keyword-says-plane-for-a-frame
kind: issue
title: Python's Node.profile keyword says plane= for an operand whose slot, field, label and word are frame
status: closed
opened: 2026-10-09
priority: P1
closed: 2026-11-02
branch: reads/python-profile-keyword-frame
pr: 6317
---


Filed at PR 4342 (INTENT stage 2 unit B) by the orchestrator's ruling: one name per thing, not held up B.

Unit B gave the frame operand of a profile and an in-plane axis one name, `frame`: the Rust/wire field (`ProfileProgram.frame`, `Datum::AxisInPlane.frame`), the slot (`OperandSlot::Frame`), its label and the Python `set_param` word. The Python constructor keyword is the one place left saying `plane`: `Node.profile(outline, plane=frame)` (`crates/pncad-py/src/py/`, the `profile` constructor; `pncad.pyi`), and `Node.datum_axis_in_plane(frame, …)`'s parameter name. The keyword should follow the slot's name (`frame=`); every Python caller (tests, `docs/GUIDE.md`, `docs/guide/*.md`, demos) moves with it.

## Note (INTENT stage 3 A, 2026-10-10)

`Node.datum_axis_in_plane` is gone: INTENT stage 3 A wrote a revolve's axis as `Node.revolve(profile, axis_origin, axis_direction, angle)`, so `Node.profile`'s `plane=` is the one keyword left.

## Closed

Nothing left to rename: INTENT stage 3 D (PR 5506) took the operand off
both profile doors — a profile is 2-D, `Node.polygon` and `Node.profile`
draw on their own xy and take no plane or frame, and a body built from
one stands where its placement puts it. The sweep found one stale
mention of the old keyword, the G3 row of `docs/guide/north-star-audit.md`,
and re-wrote it. Its sibling disagreements between a Python keyword and
its `set_param` word are `python-operand-keywords-and-slot-words-name-one-slot-twice`.
