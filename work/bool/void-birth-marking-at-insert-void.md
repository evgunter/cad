---
id: void-birth-marking-at-insert-void
kind: issue
title: voids - structural void-birth marking at insert_void (planned, unscheduled, the eventual outer/void rung)
status: open
opened: 2026-08-25
github: 979
refs: [978, 907]
---

## From GitHub issue 979

Opened 2026-08-25; 0 comments.

Recording Ev's ruling (2026-08-25, the checks-registry conversation): the structural alternative that DISCIPLINES-DESIGN DS6 round 4 (PR #978) recorded as "considered and deferred" is the **eventual plan**, not a rejected option. Planned, unscheduled, not to be implemented yet — this issue is the record.

**The plan.** Every cavity is born through the one ratified void door (`topo::insert_void` — the M2/#907 invariant), so the door can mark cavity shells structurally at birth, making outer-vs-void a structural fact instead of a derived orientation read (`classify_shells`' per-shell signed-volume trilean at `chk_shell_volume_sign`). The single-birthplace invariant is what makes this clean — the ruling's ground.

**What it buys.** The connectedness check's void exclusion becomes combinatorial (restoring LONGTERM-IDEAS I1(0b)'s original "exact, no thresholds" strength); no escalation arm for thin shells; bodies whose shell flux is uncomputable (rational walls) still classify.

**Named obligations, to be designed before implementation** (from the round-4 deferral rationale):

- Maintenance across `revert` (cavity/outer roles under complementation), shell regrouping (a cavity carved open to the outside must lose its mark — the PR #978 review's carve-open probe is the fixture shape), boolean grafts (`graft_solid`, `graft_disjoint_all_keyed`), and instance transplants.
- The derived read does not disappear: a stored mark is a structural claim, and the signed-volume read is its natural verifier — verified never trusted, the C4 shape. Certification should reconcile mark against flux where computable rather than trusting either alone.
- Import (D7): adopted bodies' shells carry no birth mark, so adoption needs its own classification rung (presumably the derived read at ε_in) — the structural rung is sufficient, not necessary, exactly the coincidence-ladder shape.
- D1's "nothing about a body is true that is not derivable from its construction" holds: the mark is derivable from the recipe's void-inserting node; the design question is where it lives so replay reproduces it bit-identically (D9).

Pointers: `docs/DISCIPLINES-DESIGN.md` DS6 (round 4), PR #978, `crates/topo/src/boolean/voids.rs`, `topo::props::classify_shells`, `docs/SMELL-SCAN-2026-08.md:1653` (the related step-export granularity row).

## Home

`crates/topo/src/boolean/voids.rs` is inside S-BOOL's `crates/topo/src/boolean/*` territory, and containment doors are its charter.
