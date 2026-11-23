---
id: the-sym-profile-lowering-exhausts-memory-under-the-revalue-instrument
kind: issue
title: Under the guided profile lift, a Sym session's per-node form tables grow without bound and the revalue instrument runs out of memory on five of nine corpus documents
status: closed
opened: 2026-10-17
priority: P1
cost: M
refs: [the-pinned-profile-lift-launders-at-sym]
closed: 2026-10-17
---


Found by the pinned-lift unit (ruling A, branch `inlane/pinned-lift-derived`, f9a5e0b65f). Under ruling A, `symbols` (and a varying box) make an evaluation's profile lift guided, so `crates/editor-core/tests/revalue_corpus.rs` now runs profile generic code at `Sym<f64>` with every variable a symbol.

## Per document

The whole `editor-core` suite ran on that branch, 4 cores and 15 GB, under the drive's default budget (`max_terms` 4096, `max_degree` 128).

| document | outcome |
|---|---|
| `revalue_boss_union` | killed by the kernel (memcg OOM) at 8.6 GB |
| `revalue_cut_cylinder` | killed by the kernel (memcg OOM) at 8.6 GB |
| `revalue_declared_tangency` | killed by the kernel (memcg OOM) at 8.6 GB |
| `revalue_die_chamfer` | killed by the kernel (memcg OOM) at 8.6 GB |
| `revalue_plate_param` | killed by the kernel (memcg OOM) at 8.6 GB |
| `revalue_heat_sink_fins` | still running at 45 min, 5.4 GB |
| `revalue_face_sketch` | passes, 332 s |
| `revalue_loft_prism` | passes, 172 s |
| `revalue_measured_web` | passes, 759 s |

Reproduced on main (82d0f5c2b7) by setting `profile_lift: Guided` in the row's `options`, one row alone, with the address space capped at 7 GB:
- `revalue_plate_param` pinned: 316 MB, 37 s.
- `revalue_plate_param` guided: aborts at the 6.2 GB cap after 803 s.

## Diagnosis, measured

**Not `revalue::record`.** One guided `p0` evaluation with no `record` around it reaches 4.5 GB in 500 s without finishing.

**Leak 1, fixed in this branch: the reduction's sum was outside the size budget.** gdb samples of the running evaluation sit in `geom_core::sym::algebra::poly_subst_square` (`certify_edge_spec` → `check_residual` → `decide_sign` → `discharge_retried` → `rungs` → `reduce`). A counter on its running sum read 62,832 terms against a budget of 4,096, from an input of 1,768 terms. Each product `term · factor` passes `Poly::mul`'s budget, but `Poly::add` takes none, so the sum grew to fifteen times the budget, and the next of `REDUCE_STEPS` substitutions started from it. Now the sum is checked against `max_terms` after each add, and past it the reduction freezes, as `reduce`'s doc already said it would ("past the step or size budget"). Pinned by `geom-core`'s `sym::algebra::tests::a_substitution_whose_sum_outgrows_the_budget_freezes`, red without the check.

That fix is not enough. Guided `plate_param` then aborts at the cap in 219 s instead of 803.

**Leak 2, open: the session's per-node form tables have no total bound.** Counted at 150 s into guided `plate_param` (3.7 GB):
- 22,228 DAG nodes;
- `forms` (plain): 17,491 forms holding 2,451,737 terms;
- `forms_early`: 13,060 forms holding 3,956,494 terms;
- the reduction memo (`Session::reductions`): 500 forms, 519,034 terms. Its count cap (`REDUCTION_FORMS` 50,000) is not what binds.

About 6.4 million terms, at roughly 200 terms a form and up to the 4,096 budget, each term a heap monomial and a rational, account for the gigabytes. `SymBudget` bounds each form; nothing bounds what a session holds in total. Under the pinned lift a profile's values are literals and these forms stay small. Under the guided lift the profile's arcs, frames and carriers are polynomials in every variable, and every certification residual over them is cached per node.

## What is owed: a design question

How a symbolic session bounds what it holds:
- a session-wide term budget past which a node's form freezes (it is decided numerically);
- eviction of per-node forms the walk will not revisit;
- or a smaller per-form budget under the guided lift.

Each changes which decisions discharge symbolically, so it decides the tier's reach, not only its cost. Raised to the INLANE orchestrator before any choice is built. Until it is decided, ruling A cannot land: its revalue rows cannot run.

## Closed

Leak 1 landed in PR 5245. Leak 2 is closed by the INLANE orchestrator's ruling (fork-log row 155: two designers, converged, and the measurements above). The instrument has no reason to run an eager session: it reads no form. Under the pinned lift, on the nine documents, the tier on and the tier off (`SymBudget::none()`) record identical decisions, predicate and sign.

| document | decisions | proven Zeros with the tier on |
|---|---|---|
| plate_param | 15,297 | 7,681 |
| boss_union | 11,274 | 4,529 |
| cut_cylinder | 3,627 | 1,528 |
| declared_tangency | 2,672 | 1,604 |
| die_chamfer | 4,372 | 2,796 |
| heat_sink_fins | 2,945 | 1,812 |
| face_sketch | 1,314 | 864 |
| loft_prism | 764 | 596 |
| measured_web | 1,683 | 981 |

So one function now opens the replay with every variable a symbol: `editor_core::coincide::with_replay_session`. It is deferring, runs at the door's 256-term budget with `DEFAULT_SYM_RETRY`, and takes the guided lift. Three things use it: the coincidence door, its census (whose hand-kept copy is gone), and the revalue instrument. `the_instrument_and_the_door_replay_in_one_session` pins that the instrument's zero box and the door's `symbols` read one decision list and one set of counts.

The guided `plate_param` evaluation alone went from more than 4.9 GB (cancelled at 120 s) to 136 MB in 1.07 s. Every revalue row now runs in 2.0 to 28.5 s, where five were OOM-killed before. The three findings the guided reading turned up are filed and named on their rows.

The "leak 3" reported mid-unit was a misread: the profile's `max_terms_out` is numerator plus denominator, and every op's output passes `within` per polynomial.

What the tier itself should bound is filed under sym, not built: `work/sym/a-sym-session-bounds-what-it-holds-by-cache-and-ceiling-not-reach.md`.
