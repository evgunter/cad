---
id: python-profile-keyword-says-plane-for-a-frame
kind: issue
title: Python's Node.profile keyword says plane= for an operand whose slot, field, label and word are frame
status: open
opened: 2026-10-09
priority: P1
---


Filed at PR 4342 (INTENT stage 2 unit B) by the orchestrator's ruling: one name per thing, not held up B.

Unit B gave the frame operand of a profile and an in-plane axis one name, `frame`: the Rust/wire field (`ProfileProgram.frame`, `Datum::AxisInPlane.frame`), the slot (`OperandSlot::Frame`), its label and the Python `set_param` word. The Python constructor keyword is the one place left saying `plane`: `Node.profile(outline, plane=frame)` (`crates/pncad-py/src/py/`, the `profile` constructor; `pncad.pyi`), and `Node.datum_axis_in_plane(frame, …)`'s parameter name. The keyword should follow the slot's name (`frame=`); every Python caller (tests, `docs/GUIDE.md`, `docs/guide/*.md`, demos) moves with it.
