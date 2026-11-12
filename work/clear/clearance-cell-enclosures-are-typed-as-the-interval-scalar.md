---
id: clearance-cell-enclosures-are-typed-as-the-interval-scalar
kind: issue
title: The clearance engine's cell enclosures are typed as the Interval scalar: every input value enters through Interval::from_certified, so a bound on a function over a cell and an uncertain number are one type
status: closed
closed: 2026-11-12
pr: 6821
opened: 2026-11-02
priority: P0
cost: M
---


Raised by Ev on PR 6308 (2026-11-02): "it seems wrong to be putting in
an f64 as an interval? ... isn't it semantically a different type than
the input type? i guess that might be a bit ugly since `Sym` isn't
compatible", then "i'm not sure if `Sym` is completely incompatible or
just difficult/impractical".

`topo::clearance` and `topo::injective` subdivide each face's parameter
domain. For each cell they need an enclosure of the cell's whole image:
a set bound on a function over a domain, which no point evaluation
gives. They build it in `Interval` arithmetic, and every stored value
of the `Body<T>` enters through `Interval::from_certified`
(`clearance.rs` module docs; `window_of`, `boundary_of`, the `lift`
module). So one type, `Interval`, plays two roles. One is the kernel's
uncertain-number scalar: the value of a written quantity across its
family. The other is the engine's range bound. At `f64` a stored value
enters as a degenerate interval, which is exact as a conversion but
reads as treating the float as the value.

**The question, for designers before the at-rest-gate unit is specced**
(that unit is where f64 first enters the engine):
- Whether the engine gets its own enclosure type, built from `Body<T>`
  per scalar, and how each scalar enters it: f64 and Probe as points,
  with the K·ε band `[topology.tiers]` now states; Interval as the
  family's brackets.
- What `Sym` gives. Ev's question is whether it is incompatible, or
  only difficult or impractical. Today it reaches the engine only
  through its interval bounds.

Priority: P0, because `the-at-rest-gate-certifies-embedding` (P0) waits
on it (a blocker inherits the band of what waits on it).

## Dispatch

2026-11-11, CLEAR orchestrator. The fork is with two designers, A and B, under `docs/DESIGN-FORK-PROTOCOL.md` (protocol `2126b860b63`). The arm byte drew two designers, and the draw and labels are on `analysis/design-fork/clear-enclosure`. Both have the same problem statement and no candidate answers. Reconciliation follows `memories/orchestration-model.md`. The outcome goes to Ev as an `[ev]` PR that edits this row and any clause the answer touches, or is decided here if no ratified text moves and the designers converge.

## The answer the designers converged on (fork `clear-enclosure`, 2026-11-11)

Two designers, first reports and two reconciliation rounds (`docs/DESIGN-FORK-LOG.md` row 260). The conflation is not a fault in itself. A lane `Interval` and a cell's bound are the same kind of object: a set of reals containing a function's values over a box. They differ in the box, which belongs to the computation, not to the value. An f64 stored value stands for itself (`[simple.conditioned]`), so `[x, x]` is exact in meaning, and the K·ε band lives in the reading, not the crossing. The faults are three things the one type lets slip:

1. **One crossing, held by the lane.** Every stored value enters through one function on `ClearanceLane`, the value that carries certification rights. No caller spells a lift. `topo::shell::cavity_embedded`'s `lift` parameter goes.
2. **No second reading.** The engine never establishes an equality from a cell bound. "These meet" and "this is zero" come from topology (a shared edge or vertex, a recorded contact), from tier 3's verdict (an edge on its carrier, a chart singularity as a vertex, the material wedge), or, for a separator the engine builds itself, from its construction through the shared edge. Today `self_intersection_on_shared`, `self_intersection_apex` and `self_intersection_cusp` (`crates/topo/src/injective.rs`) re-decide those facts numerically over each cell, against `[one-way.readings]`. Over a wide family they refuse bodies tier 3 accepted. This is also where `Sym`'s identity proofs reach the engine.
3. **A refusal names the width that blocked it.** It names either the family's width, which only a narrower parameter box answers, or the cell's, which refinement or budget answers. A finding's point reading, at two stations, is made at the body's own scalar after the sweep returns, and it is the one logged row. At `Interval` its straddle is the drive's cue.
4. **The engine's entry points own their unlogged scope.** `Sweep::run` and `min_separation` keep cell readings out of the node's verdict log themselves, so the hand-kept scopes F28–F30 (`docs/predicate-dimension-audit.md`) and the shell's missing one (`the-shells-cavity-sweep-logs-its-cell-readings`) go.

**`Sym`** fits the engine, since `Sym<Interval>` is a `Real`, but it decides nothing over a cell: every cell decision is a strict sign it cannot change. It would pay a symbolic expression node per operation per cell. Its benefit arrives through change 2.

**The alternative weighed:** a `CellBound` type wrapping `Interval` with no `Decide`, so that a cell reading in the drive's log fails to compile. It must still be a full `Real` by delegation, because `Vec3<T: Real>` requires it, so it sits close to what PR 2701 retired. It would leave F28's region readings logging, and no production site reads a cell outside the two entry points today.

**For `the-at-rest-gate-certifies-embedding`'s spec:**
- the yes/no verdict is a function bounded on certification rights; only the minimum-clearance measure keeps its `None` at f64, and `Sym<f64>`'s door is to be stated;
- each local reading in `topo::injective` states either that it passes by the band, or which topology or tier-3 fact it reads in place of a cell Zero;
- measure a `Sym<Interval>` leaf on a box wide enough that tier 3 needs its symbolic proof.

## Decided (Ev, PR 6821, 2026-11-12)

Ev approved the converged answer above ("ok! this sounds good!"), after three questions on the PR.
- **`[x, x]` stays a point.** `Interval::from_f64` stores `x` at both ends. Width comes only from outward rounding in arithmetic, or from a cell's extent in the engine.
- **Why `Interval` rather than the generics.** A cell's bound is set-valued at every scalar. Every decision about the body stays at its own `T`.
- **The fix for "no second reading".** The three Zero sites in `injective.rs` read the fact from topology or tier 3 instead of from a cell.
- **Decision 3: the engine's door** (`Sweep::run` and `min_separation` own the unlogged scope), not `CellBound`.
- **`[topology.tiers]`** takes Ev's wording: an exact value `x` enters the question as `[x, x]`, and a stored bracket as itself.

Changes 1–4 are built with `the-at-rest-gate-certifies-embedding`, which this row unblocks, and with `the-shells-cavity-sweep-logs-its-cell-readings`, which the door closes.
