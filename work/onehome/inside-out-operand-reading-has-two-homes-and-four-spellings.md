---
id: inside-out-operand-reading-has-two-homes-and-four-spellings
kind: issue
title: The inside-out operand reading has two loops, four spelled-out recourse sentences and convention-held unreachable arms
status: open
opened: 2026-10-12
priority: P1
cost: E
---


Left by PR 4725's final re-review (style notes, none gating).

- The recourse "build it with its faces pointing outward, or revert it" is written out four times: `validate.rs` (~L3442, ~L3798, ~L9228) and `solid_contain.rs` (~L645). Give it one home. RESTFRONT's `an-inside-out-findings-end-on-the-kernel-defect-ending` is the neighbouring class.
- `reduce.rs` `gate_sliver_orientation` and `shell_witness::inside_out_operand` both loop over solids reading `SolidFaces::of(..).read_as_given()` through `point_in_solid_faces`: two homes for one reading.
- `census::point_hull` is shared but lives inside the census, one of its consumers.
- The census `unreachable!` on `InsideOut` holds only because no census selection sets `as_given`, and nothing enforces that. The gate's `unreachable!` on `OnBoundary` likewise rests on the probe point's 1 m margin.
- In the gate, `let Ok(sel) = SolidFaces::of(..) else { continue }` can never fail, because the keys come from `body.solids()`. That makes it a silent skip that never runs.
- `inside_out_operand` could return `InsideOutOperand { errors: [] }` if the whole-body read refuses while no single solid does. Not reached.
- The gate's refusal now precedes more specific ones. A valid thin tube cut across used to refuse `CurvedPierceUnsupported` and now refuses `Containment` from the gate. Nothing pins which story wins.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Mechanical, partly stale** (cluster E). `gate_sliver_orientation` is gone and `point_hull` is no longer shared. Still live: the recourse sentence spelled four times (`validate.rs` ×3, `solid_contain.rs`), which one constant fixes. The rest is robustness, not duplication. DESIGN home: `[tolerance.recourse]`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: one reading in two homes and one recourse in four: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
