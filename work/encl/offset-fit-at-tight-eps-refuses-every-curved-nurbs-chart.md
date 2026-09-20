---
id: offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart
kind: issue
title: at eps 1e-12 the offset fit refuses every curved NURBS chart, and the mint's cost at the default eps is unmeasured on a body
status: open
opened: 2026-09-05
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
