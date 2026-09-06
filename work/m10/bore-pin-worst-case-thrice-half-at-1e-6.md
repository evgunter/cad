---
id: bore-pin-worst-case-thrice-half-at-1e-6
kind: issue
title: m10_4 bore-pin worst case is 3x half, not 2x — reds only at eps=1e-6, where the absolute 1e-9 slack stops hiding it
status: closed
opened: 2026-09-03
closed: 2026-09-03
---

## Was

unrowed. Turned up by a Track K PR's CI draw (`interval, eps = 1e-6`),
reproduced on clean `origin/main`, and filed rather than worked because
`crates/editor-core/` is not that PR's ground.

## Finding

`crates/editor-core/tests/m10_4_r2_probes_interval.rs:1247` —
`the_bore_pin_fit_as_a_consumer_reads_it` — **reds on `origin/main` at
`CAD_TOLERANCE_EPS=1e-6` in the `interval` lane**, and passes at the
default ε and at `1e-12`. Reproduced on a clean checkout of
`origin/main` at `6eb7777f7`, three runs:

| ε row | result |
|---|---|
| default | ok |
| `1e-6` | **FAILED** |
| `1e-12` | ok |

The failing line is the worst-case **width** bound, not the bracket
above it:

```rust
let half = eps() / 8.0;                                    // :1216
assert!(wc.lo <= 0.2 - half && wc.hi >= 0.2 + half, …);    // :1246  passes
assert!(wc.hi - wc.lo <= 2.0 * half + 1e-9, "{wc:?}");     // :1247  FAILS
```

Reported value: `WorstCase { lo: 0.1999998124999996, hi: 0.2000001875000002, leaves: 4 }`.

- observed width — `3.750000006e-7`
- `half` at ε=1e-6 — `1.25e-7`
- bound — `2.0 * 1.25e-7 + 1e-9 = 2.51e-7`

**The worst case is three halves wide, not two.** `3.75e-7 / 1.25e-7 = 3.0`
exactly, so this is not noise and not an ε-scale rounding artifact.

**Why it only reds at 1e-6, which is the part worth reading.** `half` is
ε-scaled (`eps()/8`) but the slack term `1e-9` is **absolute**. At the
default row and at `1e-12`, `half` is small enough that `1e-9` dominates
the whole bound and admits any width the study produces; at `1e-6`,
`2.0 * half = 2.5e-7` finally exceeds `1e-9` and the bound starts
actually constraining. So the 3×-versus-2× discrepancy is present at
every ε and is **masked at every ε but this one** by a constant that was
never scaled with the rest of the assertion.

That makes the ε=1e-6 red the honest reading rather than the anomaly,
and it means the two candidate fixes are not equivalent:

- if the worst case *should* be `2 * half`, this is a defect in the
  stackup's worst-case composition that the other ε rows have never
  been able to see;
- if `3 * half` is correct for a `±0.05` uniform study through this
  fit, the assertion has been wrong since it was written and the
  absolute slack is why nobody noticed.

Deciding which is M10's, not a lint's. **Do not widen the `1e-9`.**

## Sweep context

Pattern: an assertion mixing an ε-scaled quantity with an absolute
slack term, so that the bound's tightness varies by ε row. Not swept —
this is one instance, found by a lane draw rather than by a search, and
`crates/editor-core/tests/` is Track W's ground under the code-quality
partition. **What this note cannot claim:** whether the same shape sits
in the other ε-scaled assertions in this file (`:742`, `:811`, `:929`
each define their own `half = eps()/N`) or in its siblings.

Related, and plausibly the same family — the tracker already carries
three `eps=1e-6` reds on main: `e6-driver-k-probe-reds-at-eps-1e-6`,
`driver-k-probe-nothing-certified-red`, `k-probe-sweep-stalls-at-eps-1e-6`.
Whether they share a cause is not established here.

## Reproduction

```
git checkout origin/main
CAD_TOLERANCE_EPS=1e-6 cargo test --features interval -p editor-core \
  --test all m10_4_r2_probes_interval::the_bore_pin_fit_as_a_consumer_reads_it
```
