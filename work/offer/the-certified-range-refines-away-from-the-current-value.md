---
id: the-certified-range-refines-away-from-the-current-value
kind: unit
title: the certified range halves its whole seed level by level, so it spends its budget away from the value it answers about, and stop discards it
status: closed
opened: 2026-11-22
priority: P1
cost: H
pr: 7369
branch: offer/the-certified-range-refines-toward-the-current-value
closed: 2026-11-23
---

The kernel half of the decided certify design (Ev, #7340). The spec is the `## Design` section of `work/offer/certify-affordance-on-the-bounds-panel.md`, kernel part, together with its `## Decided` section.

## The work, in `editor_core::range`

**A resumable search** shaped like a stepped value. `new` admits the request and evaluates the witness once; `step(&Stop)` advances; `answer()` can be read after any step.
- **Schedule.**
  - Split at the current value first.
  - On each side, replay the interval whole. Where it is undecided, halve it and replay both halves, nearest the current value first.
  - Replay the far half too, so the search looks past undecided ground and can find a proven difference beyond it. A grow-outward-only schedule would make that arm dead, because a certified leaf and a flip leaf are never neighbours (`walk`'s docs).
- **Determinism.** The answer after n steps is a pure function of the inputs and n. No clock is read.
- **Pricing.** Ground the search has not reached is priced as budget (E6).
- **Resolver.** The search takes the session's part resolver. `drive` takes none today: `work/certread/the-box-driver-carries-no-part-resolver`.

**The answer type.**
- A proven interval, possibly empty, derived from the two sides.
- Per side, one of:
  - reached the seed's end;
  - proven different at a point, carrying the driver's evidence, with `kind()` derived from that evidence;
  - not decided past x, carrying its typed reason, with `more_time_helps()` derived from the reason.
- The `within` pair and the straddling span go.
- `LeavesAreNotAPartition` goes, because the search builds its own partition.

**Admission.** One admission door runs no geometry and gives the request-shaped refusals before any drive. It covers a Count field, a structural or defined slot, the seed, and, where the landing knows it, a document that does not build now.

**The seed's extent.** Where a field states its own admissible domain, that domain is the seed. Find what the kernel states, for example a length's lower bound. Otherwise use a fixed span relative to the current value, stated in the answer so the viewer can say "checked to …".

**`certified_range`** becomes a thin wrapper that runs the search to completion. `drive` is unchanged.

**Python** (`pncad-py`) gets the search. `work/pydoor/certified-range-has-no-python-door` is that row; read it and close it here if this unit builds it.

**Rows.**
- On the plate's `hole_r`, the directed search proves a non-empty interval within a step cap that the old drive cannot.
- Stop after n steps returns the proven interval, not a refusal.
- A side whose only boundary lies past undecided ground reports "proven different".
- A part-instancing document is admitted.

**Also.** Re-take the per-replay cost on the corpus documents and record it at the claim site in `range.rs`'s docs. Fix the stale `work/props/` citations in `range.rs` as a drive-by.

Ground: `crates/editor-core/src/range.rs` (DOCTAIL) and `drive.rs` (SCALAR, BOXFINISH). Announce on their logs.
