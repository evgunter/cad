# MATE-2 R2 review probe artifacts (PR #1417, frozen head c27ecb5a)

- `mate2_r2_probes.rs` (in crates/sweep/tests/, registered in all.rs):
  adversarial rows — partial/diagonal declaration covers, rotated
  seams, offset engagement, proud-one-end, full-period pins, and the
  additivity ULP measurement. All green on the head at default ε,
  1e-6, and 1e-12.
- `main_side_instrumentation.patch`: the trace patch applied to the
  MERGE BASE d4908e32 (main state) in a throwaway worktree — env-gated
  eprintln at the covered computation, inside vertex_on_curved_face,
  and at the circle rung's refusing return (main reduce.rs:1179).
- `main_mechanism_trace.txt`: the re-trace of claim 1 on main —
  covered=true, OnEdge(4v1) + Out at edge 4v3 x face 3v1, frontier at
  the circle rung's endpoint loop.
- `main_red_first.txt`: the unit's three rows run on main — partial
  and seated-collar red with the PR's exact quotes, full engagement
  green (deviation 1 verified).
- `lily_main_out.txt` / `lily_head_out.txt`: the full lily wall-probe
  output on both trees — byte-identical (diffed clean).
- `lily_head_wall12_trace.txt`: the traced wall-12 refusal on the
  head — covered=true, verdict=None (not Out) at both endpoints of
  edge 4v1 vs the corm's full-period face 3v1 (claim 5's mechanism).

The head-tree trace instrumentation in reduce.rs and the `lily-walls`
dispatch in demos/tour/src/main.rs are review scaffolding on this
branch only — not for merge.
