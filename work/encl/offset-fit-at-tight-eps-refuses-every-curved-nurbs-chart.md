---
id: offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart
kind: issue
title: the offset fit's certified bound rises from round 6 on, from Bézier-insertion width, so tight-eps fits refuse; and no NURBS-walled body shells, so the body-level cost has no operand
status: parked
opened: 2026-09-05
priority: P0
cost: H
blocked_on: [f64-refinement-inside-an-enclosure-has-five-more-sites]
---

**Owner: the offset fit's — PROPS / S-CERT
(`crates/geom-brep/src/offset_fit.rs`, `crates/geom-brep/src/offset_meters.rs`).**
Filed from SEAT-9 (PR 1995), which made the fit target the run's
ε_precision and so turned two latent facts into consumer-visible ones.

## 1. At a tight ε the fit refuses every genuinely curved chart

Measured on `geom-brep`'s own `bowed()` fixture (a gently bowed
polynomial patch over `[0,1]²`), `fit_offset_at` at `d = ±0.05`:

| target | outcome | rounds | cells | achieved `hull_sup` | wall |
|---|---|---|---|---|---|
| 1e-6 | reaches | 0 | 1 | 4.39e-7 | 12 ms |
| 1e-9 (`DEFAULT_EPS`) | reaches | 3 | 64 | 9.26e-10 | 170 ms |
| 1e-12 | **`BudgetExhausted`** (round budget 6, per-direction cap 48) | — | — | 3.27e-10 | 1.88 s |
| 1e-15 | `BudgetExhausted`, identically | — | — | 3.27e-10 | 1.87 s |

And on `sweep`'s twisted-loft saddle wall, through the kernel door
(`CAD_TOLERANCE_EPS=1e-12 cargo test -p sweep --test all
offd_r1_probes::the_fitted`): `RefinementStalled` on a 20×18 grid after
5 rounds, achieved 2.52e-9.

**The consequence, stated as a kernel property**: on a run at
ε = 1e-12, `topo::replace_face_offset` — and therefore `topo::shell` —
refuses on any face whose chart is a genuinely curved NURBS. The
refusal is typed and loud and is D4's blessed ε-tightening behaviour
rather than a defect; what is unowned is whether the engine's round
budget (6) and per-direction sample cap (48) are the right numbers at
the tight end of the ε range CI gates. No production caller reaches the
NURBS lane today — every in-tree shell fixture offsets analytic
charts — so nothing is broken; the row is here so the budget question
has a home when one does.

`crates/sweep/tests/offd_r1_probes.rs`'s
`the_fitted_obstruction_holds_on_a_curved_fit` pins both arms and
carries `CURVED_FIT_REACH` as the measured boundary.

## 2. The mint's production cost at the default ε is unmeasured on a body

The table above is the FIT's cost in isolation. At ε = 1e-9 the same
fixture mints a fit of 121 control points in 167 ms where 1e-6 gave 16
in 12 ms — an order of magnitude in both, per face — and **no fixture in
the tree shells a NURBS-walled body**, so the cost of a real
`shell` over a spline-walled operand at the default ε has never been
taken. That is a measurement the offset fit's owner is better placed to
design than SEAT was: it wants a body, not a patch, and it wants the
tier-3 re-derivation counted beside the mint (tier 3 re-certifies every
`Approx` face on every validation call, at the same ε).

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/props/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). The body names PROPS as the owner (`offset_fit.rs`, `offset_meters.rs` are its glob).

## Measured on main (2026-09-25)

The lane that took this row measured before touching any number; the
PR carries the traces. What the measurement changed about the row:

**§1's table, re-taken** (`bowed()`, `d = ±0.05`, identical at both
signs; release build, single run each, on a 4-core container shared with
two other lanes):

| target | outcome | rounds | cells | achieved `hull_sup` | wall |
|---|---|---|---|---|---|
| 1e-6 | reaches | 0 | 1 | 4.39e-7 | 4 ms |
| 1e-9 | reaches | 3 | 64 | 9.26e-10 | 45 ms |
| 1e-12 | `BudgetExhausted`, 27×27 | 6 | — | 1.78e-10 | 0.50 s |
| 1e-15 | `BudgetExhausted`, identically | 6 | — | 1.78e-10 | 0.49 s |

**The identical row at 1e-12 and 1e-15 is not evidence of a floor.**
The loop's path does not read the target until it exits, so any two
targets below every round's bound run the same rounds and report the
same last bound. There IS a floor, found by tracing the rounds instead:
the certified bound falls to 6.88e-11 at round 5 and then RISES
(1.78e-10, then 2.49e-9 with the budget raised) while the sampled
residual keeps falling to 6.1e-12. The rise is enclosure width born in
the Bézier decomposition's lerp-form knot insertion
(`crates/geom-core/src/spline/compose.rs`, `insert_once_ring`), which
accumulates at the patch's high-parameter corner. No budget crosses it.
It is removable by the convex form of the same combination, and that
evidence (an A/B on this fixture and on the saddle wall) is filed on
the site's existing row,
`work/props/f64-refinement-inside-an-enclosure-has-five-more-sites.md`,
rather than here.

**With the convex form, the budget question is real and measured:**
1e-12 on `bowed()` needs 9 rounds and 49 samples per direction (shipped
6 and 48), mints in 4.3 s and re-certifies in 1.2 s per validation call
per face (2,116 cells, 2,401 control points). These and the saddle
timings below are release builds, best of 3, on a 4-core container
shared with two other lanes.

**Two claims in §1 no longer hold as written.**

- "Refuses every curved NURBS chart at 1e-12" depends on `d`. The
  saddle wall at the sweep pin's `d = 5e-10` certifies at 1e-12 in one
  round and at 1e-13 in two. `CURVED_FIT_REACH` in
  `crates/sweep/tests/offd_r1_probes.rs` was re-baselined from 1e-11
  to 1e-13 on that measurement; every CI ε row now lands on that
  row's structural arm.
- It is not only the tight end. The saddle wall at a realistic
  `d = ±0.05` refuses at the DEFAULT ε 1e-9 (`BudgetExhausted`,
  4.14e-9). With today's arithmetic the bound bottoms out at 1.15e-9
  and then rises, so no budget reaches 1e-9 there; with the convex
  form it certifies at round 9 on a 49×26 grid (one past the cap),
  minting in 2.1 s and re-certifying in 0.54 s per face.

**§2 has no operand yet.** `topo::shell` refuses every lofted body
before any wall's fit runs: a cap's offset moves its corners, and
re-anchoring the spline wall-to-wall seam that ends there is outside
the face-replacement door's lanes. Filed on SHELL as
`shell-refuses-every-lofted-body-at-a-wall-seam-carrier`; the
public-API body and both of its boundaries are pinned in
`crates/sweep/tests/encl_curved_loft_shell.rs`.
