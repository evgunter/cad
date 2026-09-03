---
id: tier3-approx-regrid-per-face-cost
kind: issue
title: perf - tier 3's per-face Approx re-derivation is a grid per face, measure it on a real shell body
status: open
opened: 2026-08-26
github: 1019
refs: [1012, 1048]
---

## From GitHub issue 1019

opened 2026-08-26, 1 comment.

O5 ratified re-derive-per-face and named the cost up front: "a real cost (a grid per face where edges pay a line schedule) ... if the grid cost bites, that is a perf-lane finding with its own box, not a design change." This is the box.

**What runs.** Tier 3 check 1 calls `PropsQuadLane::recertify_approx` on every `Surface::Approx` face, every validation call. That is `geom_brep::certify_offset`: both door meters (`meter_patch`) plus a two-limb `measure` over the span schedule — a `(u,v)` GRID, against the line schedule an edge carrier pays.

**Measured (debug profile, unoptimized+line-tables, `Tol::witness()` defaults):**

| row | body | wall-clock |
|---|---|---|
| `verbs_offc_consumer::an_approx_faced_body_validates_at_tier_three` | lofted prism, 4 planar `Approx` walls, both signs of `d` ⇒ **8 whole-body tier-3 validations** | **4.73 s serial** (`--test-threads=1`) |
| the two tier-3 rows together (consumer + curved probe) | + twisted loft, 4 saddle `Approx` walls | 11.95 s, default thread count |

≈0.6 s per whole-body validation, ≈0.15 s per face grid. Machine: Intel i7-1065G7 @ 1.30 GHz, 8 threads, 9 GB, WSL2.

**A measurement caveat worth keeping**, because it is how this box nearly got the wrong number: an earlier reading of ~65 s for the same row was taken with the whole suite running in parallel against other agent lanes holding the build slot. The reviewer independently measured ~3.5 s. Contended wall-clock is not a cost figure — the 4.73 s serial number above is.

**What the box is for.** Release-profile numbers, a body with a realistic `Approx` face count (the shell verb's, not four walls), and the obvious lever if it bites: the schedule is fixed by D9, so the saving is in the meters, not in sampling less.

Filed from VERBS-OFF-C (#1012), MINOR-3.

## Comments

**2026-08-27** — orchestrator:

**VERBS-OFF-D PR-2 (#1048) measurement, and a finding about the fixture this issue names.**

The shell verb exists now, so here are its numbers. Same box as the OFF-C readings (i7-1065G7 @ 1.30 GHz, WSL2), `Tol::witness()` defaults, `--test-threads=1`, row `verbs_shell::the_shell_cost_is_measured_not_asserted` (`#[ignore]`d — a timing row is a measurement, not a gate):

| fixture | operand faces → result | debug build | debug tier-3 | release build | release tier-3 |
|---|---|---|---|---|---|
| box | 6 → 12 | 127.2 ms | 10.2 ms | **22.5 ms** | **3.03 ms** |
| vessel (solid of revolution) | 6 → 12 | 97.6 ms | 15.8 ms | **16.4 ms** | **1.65 ms** |
| tube | 4 → 8 | 109.1 ms | 15.1 ms | **18.7 ms** | **0.95 ms** |

Two things follow.

**1. The `O(n²)` whole-body pcurve mint banked at OFF-D PR-1 does not bite at this scale.** `shell` calls `replace_faces_offset` per chart and each of those re-mints the whole body's pcurve map, so an `n`-chart body pays `n` whole-body mints. At 3–6 charts the entire verb is tens of milliseconds in release and the quadratic term is invisible under the per-chart constant. The lever is still there (a composite door that defers the mint to one call at the end; `pcurves::staleness_posture::DECLARED` already has the vocabulary for a door that declares it does not re-mint), but nothing measured here justifies pulling it.

**2. The fixture this issue actually asks for cannot be built yet, and that is the substantive update.** The ask is a body with a realistic `Approx` FACE COUNT — "the shell verb's, not four walls" — so tier 3's per-face re-derivation grid is measured at scale. Shell cannot produce one. OFF-D PR-1 established that a fitted face's boundary cannot be re-described once the face MOVES: a fitted chart covers exactly its own parameter window, so the seam it shares with a neighbouring bounded chart is a row of neither chart afterwards, and the neighbour would have to extend to meet it. `plane × approx` has no C5 route arm either, so no intrinsic re-statement is available. Every body `shell` can build today is therefore analytic and pays no `recertify_approx` at all.

So the grid cost stays measured only on OFF-C's four-wall prism (≈0.15 s per face grid, debug) until the fitted lane closes — which needs every chart bounding an edge to move together, i.e. a body-wide fitted offset rather than a face-wise one. Until then this issue's fixture is blocked rather than pending, and I would rather say so than quietly substitute an analytic body and call it measured.

## Home

PERF's charter is the standing register of unbuilt performance work that ranks open cost centers by payoff, confidence and effort — this issue is exactly the cost box O5's ruling named.
