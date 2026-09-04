---
id: probe-bounds-lacks-driven-slot-guard
kind: issue
title: ProbeBounds on an expression-driven slot lacks the DrivenByExpression guard its sibling doors have
status: closed
opened: 2026-09-01
github: 1458
refs: [1183]
branch: chrome/probe-bounds-lacks-driven-slot-guard
pr: 1746
closed: 2026-09-04
---

## From GitHub issue 1458

Opened 2026-09-01; 0 comments.

Found by the `story_parametric` integration lane. `SetSlot` and `BeginGesture` on an expression-driven slot refuse with the typed `DrivenByExpression` affordance (the ratified micro-decision). `ProbeBounds { target: Slot { … } }` on the same slot does **not**: it runs happily, internally overwriting the expression with sampled literals in scratch, and answers a valid range for a field the user cannot edit numerically.

**Repro (verified against a scratch test):** a drum whose extrude `Distance` is driven by parameter `h` → `ProbeBounds` on that slot → `refusal: None`, `Bounds { origin: 0.02, low: Open { -2047.98 }, high: Open { 2048.02 } }`.

Two defects in one door:
1. **The missing guard** — the probe should refuse a driven slot the way its sibling doors do (or be defined over the driving parameter instead, which is what a user actually wants there).
2. **The seed floor** — for a driven Length slot the probe seeds from 1 written unit = 1 canonical metre, so the search spans ±2 km around a 20 mm part. Even where the probe is legitimate, a metre-scale seed floor against millimetre-scale geometry costs its refinement budget on the wrong decades.

The story works around it by probing the parameter (`BoundsTarget::Param`), which behaves well. Issue 1183 (interval certification of the probe) is adjacent but orthogonal — this is about the door's admission rule, not the search's arithmetic.

(story-suites orchestrator)

## Home

`work/issues/` — `SessionOp::ProbeBounds` lives in `crates/viewer/src/session.rs`, viewer ground with both GUI and GAUTH closed and no open program's territory covering it.

## Fixed (CHROME, 2026-09-04)

Both defects. The probe runs `guard_driven` for `BoundsTarget::Slot`
before anything else and refuses `DrivenByExpression`, naming the
driving parameters the way its sibling doors do
(`crates/viewer/src/session.rs:2266`). The seed floor is gone from the
`Param` arm: `DocParam::Continuous` carries a `display_unit` now, so a
parameter seeds at one WRITTEN unit like a slot does, and the one
spelling of that arithmetic lives in `Session::probe_seed`
(`session.rs:2311`). `Count` steps by 1, having no unit to name. The
seed-floor half for a DRIVEN slot is discharged by the guard rather
than by arithmetic — a refused door has no seed.

The comment that justified the metre seed — *"a parameter stores no
display unit"* — was stale before this unit started; it is deleted
rather than reworded.
