---
id: driver-k-probe-nothing-certified-red
kind: issue
title: m10_3_driver_k_probe_interval reds on its first-ever hosted execution: nothing certified, nothing to sample under k-lint's eps=1e-6
status: open
opened: 2026-08-30
github: 1296
refs: [1191, 1268]
---

## From GitHub issue 1296

Opened 2026-08-30; 0 comments.

Exposed by [PR #1268](https://github.com/evgunter/cad/pull/1268)'s repair of the k-lint probe census (the census had built `--features probe` only, so this `all(probe, interval)` suite was never compiled by the k-lint job — the defect that PR fixes). With the census repaired and the `dev-probe` k-lint row pinned by trailer for verification, the suite executed on hosted CI for the first time and failed:

```
=== k-probe sweep @ eps=1e-6 (E6 driver) ===
thread 'm10_3_driver_k_probe_interval::k_report_driver_dump' panicked at crates/editor-core/tests/m10_3_driver_k_probe_interval.rs:117:5:
nothing certified, nothing to sample
```

(run 33332365082, k-lint job; every other probe suite in the same job green.)

**Why this is design-shaped, not a plain bug:** M10-3's own record pins the macroscopic statement that certification identities widen with the box, so wide-band drives refuse ~everything as Budget today — and the dump's non-emptiness panic looks like deliberate fail-loud (an assertion-free dump never gates). Under the k-lint runner's `eps=1e-6` invocation, the two design choices collide: nothing certifies, so the fail-loud fires. Whether the right resolution is a narrower driver box for the dump, an eps-conditional stated-reason skip, or treating the empty sample as a legitimate red until the widening closes (#1191's class) is M10's call — the suite and the driver are its ratified territory.

**Status quo after #1268 merges:** the census is honest (the suite compiles and lists), and the row reds truthfully whenever a `dev-probe` k-lint draw lands — which is the sampled-matrix behavior main has had all along, now with the real error instead of the census mismatch. Not S-BLEND's to fix; filed with the evidence for M10.

## Home

`work/m10/` — `crates/editor-core/tests/m10*` is an M10 territory glob and the issue says the resolution is M10's call on its own ratified territory.
