---
id: the-ladders-follow-ups-from-its-review
kind: issue
title: geom_brep::ladder follow-ups: glue's existence-only read under First, a second non-finite rule in CoincidenceMeasure, a magnitude panic re-spelled, a stale shared_rim doc
status: dispatched
opened: 2026-11-02
priority: P1
cost: M
refs: [6333]
branch: onecopy/ladder-climb
---

From the FULL review and delta review of PR 6333 (`identity-data-reads-its-ladder-from-geom-brep`), recorded as MINORs at merge.

1. **`glue.rs` reads `shared_rim` for existence only, but `shared_rim` is `Answering::First`.** The Boolean's locus caller (`boolean/mod.rs`) uses *which* rim answered, so First is right there: a face pair riding two common circles is outside what the door claims to see. `glue.rs` reads only `.is_none()`, where every answering pair gives the same answer, and PR 4918 R2's supersession argument holds. Since PR 6333, an Interval leaf with an in-band rim pair ahead of a decided one carries a live `rim_circle_*` escalation on an Ok node at the glue door, which is on every curved boolean's path. `drive.rs` `log_read` then bisects it, or refuses it `SliverTerminal`, where main accepted it. Give the existence question its own `Answering::Any` read, or argue that the stricter glue is right, and measure what it changes at the interval drive.
2. **A second non-finite rule outside the ladder.** `carrier_eq::CoincidenceMeasure::decide` classifies ±∞ first, recording a *verdict*, then returns `Unreadable` with a synthetic INVALID margin. `ladder::Rung::decide` records an *escalation* on the poisoned residual. `CoincidenceMeasure::not_zero` mirrors `Rung::kept`. Route it through `Rung::decide`, so the ladder module doc's "the one place a margin is read" holds across the crate boundary.
3. **A magnitude panic re-spelled.** `splitting::order::in_plane_frame` swapped `decide_magnitude_reported` for `Rung::decide` plus a hand-written `unreachable!` for a Negative norm fraction, re-spelling `k_stats`' own panic. `Rung::decide` needs a magnitude form, so a nonnegative reader states that once.
4. **A stale doc.** `rim_wedge::shared_rim`'s "why the comparison is written here rather than consumed … a ladder bent to cover two subjects" predates `same_circle` reading `read_identity`; it is false now.

The probe-gated test that no CI job runs is filed on CIW (`probe-gated-lib-unit-tests-never-run-in-ci`).

## Slate triage (2026-11-03, ONEHOME cut)

- Moved to ONECOPY (`work/onecopy/`) by `git mv`, id and body unchanged. ONEHOME measured 70 budget points against 35 when its orchestrator ran out of usage; the rows it had not started (no lane, no design fork, no in-flight PR riding them) were cut onto ONECOPY, and what is in flight or staged stayed on ONEHOME.
