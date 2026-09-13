---
id: calibrator-cpuinfo-parser-selftest-cannot-see-a-broken-parse
kind: issue
title: opt-level-calibrate.py's cpuinfo selftest asserts a subset an empty list satisfies and never drives the parse
status: open
opened: 2026-09-11
refs: [criterion-selftest-fixture-is-one-scalar-in-five-fields, 2330]
---


`scripts/criterion-emit.py:146-155` declares a PARITY OBLIGATION: three
hand-kept copies of the `/proc/cpuinfo` reader exist, in that file, in
`scripts/opt-level-calibrate.py` and in
`crates/editor-core/tests/m4_pr8_latency.rs`, and a change to the field names,
to `HOST_CPU_FLAGS` or to what a null means is owed to all three in the same
diff. The obligation is about the READERS. Their GUARDS were never compared,
and they are not equal.

## Measured, on this tree

* **Rust, `m4_pr8_latency.rs:673-691`** — drives the parser against synthetic
  cpuinfo text and asserts the exact answer: a file with `Features` and no
  `model name` reads as `(None, Some([]))`, and
  `"model name: A\nflags: fpu avx2 sse\nmodel name: B"` reads as
  `(Some("A"), Some(["avx2"]))`. This copy can see a broken parse.
* **Python, `opt-level-calibrate.py:837`** —
  `assert env["cpu_flags"] is None or set(env["cpu_flags"]) <= set(HOST_CPU_FLAGS)`.
  **An empty list satisfies it**, so a parser regressed to `flags = []` at
  `:445` passes. The only other arm, `:841-847`, points `CPUINFO` at a path
  that does not exist, which drives the OSError branch and nothing else. This
  copy cannot see a broken parse.
* **Python, `criterion-emit.py`** — carried the identical pair of holes until
  PR 2330, which added the synthetic-cpuinfo arm the Rust copy has. Measured:
  before that, `flags = [f for f in HOST_CPU_FLAGS if f in present]` → `flags
  = []` left `--selftest` green.

**Both blind copies are the two now sitting in the merge gate** (the Rust one
runs as a test in the workspace suite). One of them has been repaired; the
calibrator's has not, and its history under `docs/perf-data/opt-level/` is
append-only in exactly the way the criterion one is.

## The fix

The arm `criterion-emit.py`'s selftest now carries, transplanted: point
`CPUINFO` at a file whose answer is known and assert model and flags exactly,
alongside the absent-file arm that is already there. Roughly fifteen lines,
no new dependency, and it makes the three copies' GUARDS as comparable as the
obligation already requires their READERS to be.

Worth deciding at the same time whether the obligation's own text should say
so — it currently binds a change to the readers and says nothing about their
tests, which is how two of three drifted to a weaker assertion without any
diff looking wrong.
