---
id: quadrature-budget-conflates-its-lanes-and-budgets
kind: issue
title: PropsError::QuadratureBudget fires from six sites in three lanes under three round budgets and an exact arm — the rounds payload (0 / 1 / N+1) is the only tell
status: open
opened: 2026-09-06
refs: [quad2-rational-max-rounds-dial-decision, budgetexhausted-conflates-three-terminations]
---


## Finding

Filed by the `budgetexhausted-conflates-three-terminations` fix pass
from its style review (the reviewer's M3): the unit's sweep pattern
matched `Budget*` / `*Exhausted` variant NAMES and missed variants
whose name ENDS in `Budget`. This is the hit that is the same shape
the unit fixed — one refusal variant, several terminations, one name —
and it is not fixed in that rider because `crates/geom-brep/src/props/quad.rs`
is the rational quad lane's ground and its round budgets are the
subject of `quad2-rational-max-rounds-dial-decision`.

`PropsError::QuadratureBudget { width_len, target_len, rounds }`
(`crates/geom-brep/src/props/mod.rs:414`) is raised from six sites in
three lanes, under three different round budgets and one no-budget arm:

| site | lane | termination | `rounds` |
|---|---|---|---|
| `quad.rs:594` (`cylinder_cut_face`) | quad, `QUAD_MAX_ROUNDS = 12` | schedule ran out | `QUAD_MAX_ROUNDS + 1` (13) |
| `quad.rs:3260` (`rational_patch_face`) | quad2 rational, `QUAD2_RATIONAL_MAX_ROUNDS = 7` | the last-round bound refused after round 0 | `1` |
| `quad.rs:3275` (`rational_patch_face`) | quad2 rational | schedule ran out | `QUAD2_RATIONAL_MAX_ROUNDS + 1` (8) |
| `quad.rs:3510` (`nurbs_patch_face`) | quad2, exact arm | no composite round can help; refused before any ran | `0` |
| `quad.rs:3589` (`nurbs_patch_face`) | quad2, `QUAD2_MAX_ROUNDS = 6` | the last-round bound refused after round 0 | `1` |
| `quad.rs:3604` (`nurbs_patch_face`) | quad2 | schedule ran out | `QUAD2_MAX_ROUNDS + 1` (7) |

The variant's doc (`props/mod.rs:~430`) spells the convention — "the
schedule's full count when it ran out; `1` when the last-round bound
refused the face after round 0; `0` on the exact arm" — so which
termination fired, and which of three constants is the lever, is
recoverable only by a caller who knows the convention and the lane's
budget constant. Three terminations (ran out / early-refused / exact
arm with no budget) and three levers (`QUAD_MAX_ROUNDS`,
`QUAD2_MAX_ROUNDS`, `QUAD2_RATIONAL_MAX_ROUNDS`) travel under one
name, and the `0/1/N+1` encoding is a payload doing a type's work.

The fix shape is `offset_fit`'s (PR 2008): a face per termination
naming its lever (schedule ran out, carrying the lane's budget; early
refusal, carrying the proven lower bound; the exact arm, carrying no
budget because it has none), a D2 row-1 refinement — the admission
set does not move. It waits on the dial decision because the rational
lane's budget is what that item decides, and a face that names a
constant should name the one that survives it.

## Home

PROPS — `crates/geom-brep/src/props/*` is this program's territory and
the rational quad lane is its own.
