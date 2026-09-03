---
id: m9-3-join-lane-resume-handoff
kind: issue
title: Resume this line of work — M9-3 (join lane), spec drafted, paused before dispatch
status: closed
opened: 2026-08-18
closed: 2026-09-03
github: 607
refs: [594, 600, 606]
---

## From GitHub issue 607

Opened 2026-08-18; 0 comments.

(M9 orchestrator) The M9 thread paused mid-spec after this session hit its model usage wall. Everything needed to pick it up cold is landed in **PR #606** (merged at `aba90421`). This issue is the resume affordance.

## Read first

1. **`docs/M9-LOG.md` tail** — the entry titled "M9-3 PAUSED MID-SPEC (2026-08-18) — resume point". That is the authoritative state; this issue only points at it.
2. **`docs/M9-3-SPEC.md`** — the join lane's work order, **marked DRAFT**. Binding in substance; one ruling deliberately left open (below).
3. `docs/M9-PLAN.md` (ratified), `docs/CONTACT-DESIGN.md` C7+C8, `docs/PCURVE-UNIFY-DESIGN.md` (U2), and `memories/MEMORY.md` + pointers.

`~/.local/share/cad-work/handoff-prompt-m9b.md` carries a SUPERSEDED note at its tail — its switch-time action list is history, all of it done.

## State: nothing is in flight

No implementer or reviewer agents alive. No ordinal claimed on the M9 side. **The block M9-16 draw has NOT been made** — draw it at dispatch (byte mod 4, reject ≥252). Review ordinals moved while M9 was paused (53/54/55 went to ASM and LIB), so claim from the ledger **on main**, never from the log.

## The one thing blocking dispatch

The spec's two-PR split (wall+door, then zip+marks) assumes curved germ / vertex-vertex records reach the rest lane's segment discovery on a two-peg body today. If they don't, PR-B inherits reduction work it isn't scoped for and the split moves.

**The spike that answers it was authored and never run** — the substrate agent died at a model usage limit mid-authorship. Its partial fixture is preserved on branch **`m9/3-spike-wip`** (`2c618982`, `crates/sweep/tests/spike_peg.rs`: plate + radius-0.5 peg sharing the bore carrier, declared Rest, driving the union path so `try_rest_union` is reached). **That branch must never merge** — finish the fixture, run it, read where the pipeline stops, then fix the spec's PR-boundary ruling and drop the DRAFT status.

The full substrate report (more file:line detail than the spec's appendix carries) is still at `~/.local/share/cad-work/m9-3-substrate/report.md` if that lane hasn't been swept; the spec's evidence appendix is self-sufficient if it has.

## What the substrate found that C7's sketch did not

- The wall has a **second, untyped site** — `recl_sectors` (`recl.rs:103-135`) dies `ClassificationInvariant` on curved lumps, and the two-peg path goes through v-v rim sites, not vtxfac pierces.
- **The real front door is `validate_declarations`** (`mod.rs:1503-1530`), not vtxfac — it refuses non-planar declared faces and non-Rest classes before classification runs.
- **The Rest descent is a type-level no-op** (`PlaneRelation` IS `CarrierRelation`), and the second-order trilean for the Tangent case already exists and is already metered — so this unit likely mints **zero** new metered predicates.

## M9-5's dependency: discharged in code, pointer still owed

TESS-SPAN merged (#594), but **ASM-LOG has no at-merge entry for it** — the ASM orchestrator died at a monthly spend cap first (#600). The fresh-state pointer M9-5's baselines were to consume does not exist yet. Either wait for ASM's entry or derive baselines from #594 directly, and say which in M9-5's spec.

## Then, in order

Finish + run the spike → fix the PR-boundary ruling and drop DRAFT → draw block M9-16 → dispatch PR-A → M9-5 spec → the M9 exit walk.

Housekeeping for whoever resumes: this session's four monitors died with it (re-arm from `~/.local/share/cad-work/monitors/` with the env block in the handoff); `m9/0-refusal-migration`, `m9/0b-reflex-arc-pad` and the `m9-3-substrate` lane are all merged-or-salvaged and sweepable.

## Home

`work/issues/`: a spent resume-handoff affordance for the M9 program, which has since closed; the C7 join lane and the declared-Rest ground it points at are now S-MATE's slate (`work/mate/plan.md`), and the historical narrative stays in `docs/M9-LOG.md`. Closed on migration.
