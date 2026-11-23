---
id: a-real-near-reversal-on-a-short-leg-refuses-as-a-cusp
kind: issue
title: JunctionCusp says a junction reverses at any precision at a real 178° turn on a 20ε leg: the reversal test reads the turn's cosine levered by the same short arm
status: open
opened: 2026-11-20
priority: P2
cost: M
refs: [a-real-junction-on-a-short-leg-refuses-as-tangent]
---


(PROFILE lane, from `profile/junction-turn-names-its-factors`, the
sibling sweep of `a-real-junction-on-a-short-leg-refuses-as-tangent`.)

## Finding

`path.rs` `junction_check` sends a junction whose turn
(`seg::junction_turn`, `sin φ · arm`) reads Zero to
`seg::junction_reverses`, which reads `cos φ · arm` on the same arm and
refuses `PathError::JunctionCusp` when that is definitely negative. The
cusp sentence (`PathError::JunctionCusp`'s Display) says the junction
"reverses onto the incoming direction at any precision you could care
about". On a leg of `k·ε` with `k` above the band's `k` (10 at
`Tol::witness()`), a turn a few degrees short of a reversal satisfies
both: `sin φ · arm` inside ε, `cos φ · arm` past `−kε`.

Measured at `Tol::witness()` (ε = 1e-9), after
`Open.at((0,0)).line_to((k·ε, 0))`, departing `.toward(cos φ, sin φ)`:

| k | φ | outcome |
|---|---|---|
| 20 | 178° | `JunctionCusp { margin: 6.98e-10 m, arm: 2e-8 m }`, "reverses onto the incoming direction at any precision you could care about … an undeclared cusp" |
| 20 | 175° | escalates on `path_junction_turn` |
| 50 | 178° | escalates on `path_junction_turn` |

A 2° departure from reversal is a real angle; the refusal asserts a
reversal the input does not have, and the recourse (`.cusp()`) would
author one. The tangent-side twin of this defect was repaired by naming
the factors (`PathError::JunctionTurnWithinTolerance` carries a
`LeveredTurn`, whose `align` is the cosine); the cusp arm still carries
`margin` and `arm` only and says "reverses".

## Shape question

Whether the cusp arm becomes the same turn-within-tolerance shape with a
doubled-back `align` (its recourse adding the leg lever), or keeps its
kind and re-words its sentence, binds `JunctionCusp`'s Python tag
(`junction_cusp`) and the seam's cusp refusal in `seam_arrival_check`,
which reads the same pair of margins. A design call for the owner.
