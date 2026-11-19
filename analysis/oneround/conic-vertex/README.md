# Far-centre conic probes (ONEROUND, `oneround/conic-vertex`, phase 1)

Evidence for `work/oneround/a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex.md`.
This branch is never merged. Everything here was taken locally on merge base
`b80352cd9`; the numbers are evidence for a decision, not committed measurements.

## Files

- `phase1-report.md` — the report as sent, plus an appendix (the two refusal
  pins, and how the workers' tables were taken).
- `measure-full.patch` — the measurement switch and the lane's own probes.
  Applies to `b80352cd9`.
- `probe-a.patch`, `probe-b.patch`, `probe-c.patch` — the fifteen-site probes
  (splitting/pcurves; census/factored crossing; geom-brep). Each applies to
  `b80352cd9`, alone or on top of `measure-full.patch` (checked together).
- `a_truth12.py`, `a_truth34.py` — exact truth (`Fraction`, 70–80 digit
  `Decimal`) for probe A's dumps.
- `logs/` — `arc_m{0,1,3}.log` (the near-gap meter's readings),
  `res_m{0,1,3}.log` (section-table misses, `MISS` lines), and the failure
  lists of the lib suites run with every conic point in mode 1.

## The switch

`CAD_CONIC_VERTEX` (read once per process, in `geom::curves::conic`):

- `0` (default): main's centre chain, operation for operation.
- `1`: `P = (c + u·a) + u·a·(cos θ − 1) + v·b·sin θ`, with
  `cos θ − 1 = −sin²θ/(1 + cos θ)` where `cos θ > 0`; the vertex summed in
  `f64` and charged in the running bound.
- `2`: as 1, the vertex's rounding charged nothing (same values as 1).
- `3`: as 1, the vertex summed exactly at `f64` (`two_product`, `two_sum`) and
  charged `u·|V|`.

`Conic::point_at`, the point half of `jet_at` and `running_point` switch
together, so the door's walk and its meter read one chain.

## Re-taking a number

```
git checkout b80352cd9 && git apply measure-full.patch probe-a.patch probe-b.patch probe-c.patch
export CARGO_TARGET_DIR=<a directory outside the checkout>
M=0   # or 1, 3
```

| number | command |
|---|---|
| (a) certified counts (`NEARGAP` lines) | `CAD_CONIC_VERTEX=$M cargo nextest run -p topo --lib the_near_gap_families_keep_their_certified_answers --no-capture` |
| (a) meter bound and arc (`ARC` lines) | the same with `CAD_ARC_PRINT=1` |
| (a) the module at each ε | `CAD_TOLERANCE_EPS=1e-12 CAD_CONIC_VERTEX=$M cargo nextest run -p topo --lib conic_quadric --no-fail-fast` |
| (a) the gap pin's roots against the oracle (`GAPROOT`) | `CAD_CONIC_VERTEX=3 cargo nextest run -p topo --lib a_root_whose_slack_reads_in_the_gap_is_refused --no-capture` |
| (b) section-table misses (`FARMISS`) | `CAD_CONIC_VERTEX=$M cargo nextest run -p geom-brep --lib probe_far_centre_section_misses --no-capture` |
| (b) per-point misses of the resolution row (`MISS`) | `CAD_CONIC_VERTEX=$M cargo nextest run -p geom-brep --lib the_resolution_bounds_each_section_tables_carrier --no-capture` |
| (c) apex rung (`APEX`) | `CAD_CONIC_VERTEX=$M cargo nextest run -p topo --lib probe_apex_rung_far_centre --no-capture` |
| (c) splitting / pcurves sites | `CAD_CONIC_VERTEX=$M cargo nextest run -p topo --lib probe_dump --no-capture > dump.txt`, then `python3 a_truth12.py dump.txt` (the `S1`/`S2` lines) and `python3 a_truth34.py dump.txt` (`S3`/`S4`) |
| (c) census chords and depth | `CAD_CONIC_VERTEX=$M cargo nextest run -p topo --lib vertex_probe --no-capture` |
| (c) geom-brep sites | `CAD_CONIC_VERTEX=$M cargo nextest run -p geom-brep --lib conic_vertex_probe --no-capture` |
| what moves with every point in mode 1 | `CAD_CONIC_VERTEX=1 cargo nextest run -p topo --lib --profile ci --no-fail-fast`; the same with `-p geom-brep -p geom` |

The probe tests print and assert nothing (the two repo tests the patch
instruments keep their own assertions). The probe-A command lines are as the
worker reported them; its dumps (about 50 MB) were not kept.
