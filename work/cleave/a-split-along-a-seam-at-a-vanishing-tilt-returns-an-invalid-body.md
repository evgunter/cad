---
id: a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body
kind: issue
title: A split through a cylinder's seam vertex at a vanishing tilt returns Ok bodies that fail validate_geometric (VolumeUncomputable)
status: closed
opened: 2026-10-13
priority: P1
cost: M
branch: cleave/seam-split-invalid-ok
closed: 2026-10-14
---


Found by the review of PR 4858
(`the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane`,
branch `cleave/vertex-against-section`). Main and that branch return the
same bodies here: the branch leaves these vertices on the offset's
verdict.

## What

The unit cylinder (`0 ≤ z ≤ 1`, seam vertices at `(±1, 0)`) turned
`φ = 2.2` about its axis. It is split by the plane exactly through its seam vertex
`v = (cos φ, sin φ, 0)`, with normal `n = cos ψ·w + sin ψ·ẑ` (`w` the
wall's normal at `v`), its stored origin moved `L` along the rim's
tangent, in the plane. Measured with
`crates/sweep/examples/vertex_against_section_probe.rs`
(`VS_DOOR=witness VS_PSI=1.1e-7 VS_L=…`):

- ε = 1e-9, `ψ = 1.1e-7`: `split` returns `Ok` at `L ∈ {0.02, 0.1,
  0.18, 0.5}`, and both halves fail `validate_geometric` with
  `VolumeUncomputable { source: Face { … Escalated { …
  props_chart_junction_closed, margin ≈ 2.588e-9 … }, check: Inventory } }`
  on the wall faces. The neighbouring `L` refuse at the join.
- ε = 1e-12, same `ψ`: `Ok` and invalid at `L ∈ {0.1, 0.5}`.

The review measured 3 such bodies at `ψ = 1e-7`, `L ≤ 0.22`. A split
that returns a body its own validation refuses is a wrong answer: the
result gate passes what `validate_geometric` refuses.

## Owed

Find where the split's finish lets these halves through, and make the
split refuse them typed (or build them valid). Pin the family at every
ε, with `validate_geometric` on every body returned.

## Built (branch `cleave/seam-split-invalid-ok`)

**Why the gate passed them.** The split gated each side at tier 2 alone
(`split_direct`'s two `validate_closed` calls, `SplitFinishError::ResultInvalid`);
its own doc said tier 3 was not run. So the gate and `validate_geometric` did
not read one property two ways: the split never asked tier 3. The Boolean's
result gate is the scalar's at-rest gate (`AtRestPolicy::gate_at_rest_kept`).
`split` now gates each side, once it is sorted into its solids, at the same
policy (`AtRestPolicy::gate_at_rest`: `validate_geometric` at a certifying
scalar, nothing at a dual). A refusal is the new
`SplitFinishError::ResultNotAtRest { side, errors }`, carrying the validator's
findings, each with its own ending, as `BooleanError::ResultInvalid` carries
them. The return type stays `Body`; `split-sides-are-not-finished-bodies`
keeps the type.

**Why the halves are invalid.** The section is an ellipse whose principal
vertex is the seam vertex `v` and whose semi-major axis is `1/sin ψ` (about
`9e6` at `ψ = 1.1e-7`). The material is right. The props lane reads the
ellipse's chart image at `v` through its centre, so the wall's junction at `v`
carries the carrier's own rounding amplified by the axis. Measured at
`ε = 1e-9`, `φ = 2.2`, `ψ = 1.1e-7`, `L = 0.1` (instrumented
`interval_loop_turns`): the height channel's gap enclosure is
`[−5.6e-9, 1.9e-9]`, so the gap's upper bound is `5.6e-9` and its lower bound
is `1.0e-16`, against an end slack of `3.7e-16`. The excess `2.59e-9` is in
band, so `props_chart_junction_closed` escalates. At `ψ = 1e-7` the vertex's
trig bracket is refused outright (NaN slack and gap, margin `Invalid`). At
`φ = 0.9` the same family also reaches a frame finding
(`UnrepresentableCurveDatum { Ellipse, URef, .. }`, the frame's own rounding
times the axis; PR 5100's review found it the first finding at `φ = 2.2`
too, on the grazing-plane rows with `L` up to 2.5 at 1e-9) and a spent quadrature budget (`QuadratureBudget`, width
`2.2e-6` against target `1.0e-6` after 13 rounds, 4.4 s in release). The input
is `ψ·1 m = 1e-7` off tangency, outside the band at `ε ≤ 1e-9`. So these are
kernel limitations with no geometry lever, and the refusal offers none: the
findings end in the props table's "no way through yet", the budget's
last-resort loosening, and, for the frame finding, tier 3's defect ending,
which `tier-3s-frame-check-reads-a-section-ellipses-rounding-amplified-by-its-semi-axis`
(PR 4861) files.

**Measured** (`vertex_against_section_probe`, `VS_DOOR=witness`,
`ψ ∈ {1e-7, 1.1e-7}`, `L = 0.02…0.5` step 0.02, 50 poses per ε):

| ε | main | PR 4861 head `ce0dfdf56d` | this branch |
|---|---|---|---|
| 1e-9 | 30 `Ok`, all 30 invalid; 20 `Join` | 30 `Ok`, all invalid; 20 `Join` | 0 `Ok`; 30 `ResultNotAtRest`, 20 `Join` |
| 1e-6 | 50 `Ok`, all valid | 50 `Ok`, all valid | 50 `Ok`, all valid |
| 1e-12 | 21 `Ok`, all invalid; 29 `Join` | 30 `Ok`, all invalid; 20 `Join` | 0 `Ok`; 21 `ResultNotAtRest`, 29 `Join` |

The re-pose does not fix the family: it removes the parameter term, not the
centre's. Ev's PR 4812 end state (the anchored carrier,
`the-section-table-admits-a-curve-on-its-resolution`) is what would build
these halves valid.

**Pinned:** `crates/sweep/tests/split_seam_vanishing_tilt.rs`, 80 poses
(`φ ∈ {2.2, 0.9}`, `ψ ∈ {1e-7, 1.1e-7, 1e-6, 1e-3}`, five `L`, both normals)
at every ε row: every side returned passes `validate_geometric` and the sides
hold `π`; every at-rest refusal carries only the ellipse's findings and no
tighten or move lever; the `ψ = 1e-3` poses answer.

**Fix pass (PR 5100's review).**
- The typing has one home: `topo::boolean::at_rest_refusal` (`boolean/ops.rs`,
  where the Boolean's site census reads it). The Boolean's `gate` and the
  split both route through it. A side refused only on in-band shells is
  `SplitFinishError::ResultEscalated`, `BooleanDecision::ShellRole` rendered
  as the Boolean renders it. Every other refusal is `ResultNotAtRest`, worded
  through `topo::validate::write_result_not_at_rest`, the same function the
  Boolean's `ResultInvalid` uses.
- The pin asserts the exact variant, a nonzero at-rest count at ε ≤ 1e-9
  (26 at 1e-9, 28 at 1e-12, none at 1e-6), each finding's own ending with no
  lever, and the first finding rendered whole.
- `section_ellipse_seam_poses.rs` (PR 4861) is re-baselined. 50 rows that
  returned sides tier 3 refused now refuse at the split's gate, named by the
  first refused side: 18 at 1e-6 (`VolumeUncomputable`) and 32 at 1e-12
  (`URef` frame findings).
- `transform_rigid` does flip tier 3: 40 of 784 turned valid slivers read
  `NegativeVolume` at 1e-9. Filed as
  `a-rigid-turn-reads-a-valid-ellipse-sliver-inside-out` (P2).

