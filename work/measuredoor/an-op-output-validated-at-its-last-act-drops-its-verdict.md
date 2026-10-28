---
id: an-op-output-validated-at-its-last-act-drops-its-verdict
kind: issue
title: topo::shell and the STEP reader validate the body they hand out, then hand it out as a plain Body, so every reader pays the gate again
status: open
opened: 2026-10-23
priority: P3
cost: M
---



Filed by FLUX (unit F2) on the orchestrator's ruling: this is not F2's
work. F2 moves the test sites onto the typed door (Ev's decision on
PR 4822). It does not change what an op returns.

## Measured

Two ops run tier 3 on the body they hand out as their last act. Both
then return the body as a plain `Body<T>`, so the verdict is gone and
whoever reads the output through a typed door pays the gate again.

- **`topo::shell` / `topo::shell_open`, the lead case.**
  `crates/topo/src/shell.rs:2190` runs `validate_geometric(&out_body, tol)`.
  Then `Shelled { body: out_body, naming }` returns it as `Body<T>` (`:913`).
  Every reader gates the same body again:
  - the sweep tests' `finished(...)`;
  - editor-core's `wire_shell` (`crates/editor-core/src/eval/wire.rs:2079`
    wraps `out.body` in `Arc<Body>`);
  - downstream consumers, through `finished_operand` (`:1199`).
- **The STEP reader.** `StepImport::Solid.body` is documented as "tier-valid at rest,
  checked" (`crates/step-import/src/lib.rs:526`). The gates run on the
  shipped body: `gate` (`:935`, per solid, where
  `topo::per_part_gate_owed` asks), then `gate3` (the aggregate). Even so, it
  returns `Body<f64>`, and each typed-door reader of the import validates
  it again.

No other op in `topo`, `sweep` or `geom-brep` validates its output:

- The Boolean and the split already hand out `AtRestBody`
  (`BooleanBody.body`, `crates/topo/src/boolean/ops.rs:191`; the split's
  parts through `gate_at_rest_kept`, `crates/topo/src/splitting/mod.rs:1163`).
- The sweep verbs (extrude, revolve, loft, the sweeps) do not run tier 3.
  Their callers gate (`work/wiretail/a-body-value-consumed-by-k-nodes-is-gated-at-rest-k-times.md`).

### What it would buy

F2's 14-file sweep sample (debug test profile, each file's summed test time at nextest's default parallelism, the faster of two runs each) moved from
11.05 s on main to 13.74 s on F2, which is +24.4%, or +2.70 s. The sweep
suite as a whole moved 1463 s to 1474 s, +0.75%. Of the +2.70 s, 2.46 s
is in the eleven files that build through `topo::shell`:

| file | main | F2 | delta |
|---|---|---|---|
| `verbs_shell` | 3.793 | 4.753 | +0.960 |
| `shell7_r2_probes` | 0.613 | 0.942 | +0.329 |
| `shell7_seam_corner` | 0.984 | 1.274 | +0.290 |
| `shell_curved_mouth` | 1.186 | 1.468 | +0.282 |
| `sf2a_r1` | 0.420 | 0.616 | +0.196 |
| `sf2b_r2_probes` | 0.402 | 0.506 | +0.104 |
| `verbs_tubewall` | 0.270 | 0.365 | +0.095 |
| `shell9_r2_probes` | 0.704 | 0.787 | +0.083 |
| `offd2_r1_probes` | 0.358 | 0.407 | +0.049 |
| `review_blend3_r1_probes` | 1.051 | 1.089 | +0.038 |
| `sf2a_r2_probes` | 0.272 | 0.306 | +0.034 |

The 2.46 s is an upper bound on the saving. Those files also finish
operands and fixtures that are not shell outputs. The three non-shell
files account for the remaining 0.24 s (`fillet_h5_hostless_rim`
+0.186, `ring_r1_probes` +0.046, `review_m3_pr1_sweep` +0.004).

## The change

`Shelled.body: AtRestBody<T>`, kept from the closing gate through
`T::gate_at_rest_kept` (the Boolean's form, `ops.rs:5460`), so that a
non-certifying scalar keeps its nominal verdict. `StepImport::Solid.body`
becomes `AtRestBody<f64>` the same way. Readers that mutate the output
take `into_body()`. The editor's value can then hold the verdict, which
the WIRE row above covers.
