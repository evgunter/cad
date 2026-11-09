---
id: main-red-a-lofted-arc-walls-rim-zeros-count-eighteen
kind: issue
title: Main red: a_lofted_arc_wall_is_a_function_of_the_sections_bulge counts 18 symbolic rim Zeros where it pins 10
status: open
priority: P1
cost: E
opened: 2026-11-04
---


Found by SECTARMS' PR 6479 fix pass, while running the sweep crate's slow set.

`crates/sweep/tests/a_loft_computes_its_walls_in_t.rs:116`
(`a_lofted_arc_wall_is_a_function_of_the_sections_bulge`) asserts
`rim_theorems == vec![true; 10]`, the `pcurve_iso_boundary` Zeros the
loft records over the bulge. It reads 18 of them now, every one
`true`, so the count moved and the proofs did not.

- It is red on main: nightly run 41779026337 (main at `63cbd7c300`), the
  `full suite (eps default, slow set included)` job.
- It was green on the nightly at `2d9dc809d8`.
- Between the two, PCERT merged several changes to
  `crates/geom-brep/src/pcurve_cache.rs`: E2's narrowed iso guard and
  E5's T1, among others. One of these is the likely mover. Unconfirmed.

The row is in the slow set, so a PR's CI does not run it.

What closes it: whoever owns the iso guard re-pins the count with its
reason, or restores the old count if the extra Zeros are a regression.

## Measured (bisected first-parent over main, 2026-11-09, `onecopy/loft-rim-red`)

**Moved by PR 6484** (`pcert/mint-picks-general-lane`, merge `bc927bc4b3`, 2026-11-03).
- Bisect window: `ee5c82a643` (PR 4929, which wrote `vec![true; 10]`, green) to `0d67e1f8` (red).
- Last good: `84b71280f1` (PR 6477). Red on every first-parent commit since.

**No new rim is proven: the same theorems are read twice.** Each Zero's `SymId` was recorded:
- `84b71280f1`: 10 symbolic Zeros over 7 distinct nodes.
- After 6484: the same 10, in the same order, preceded by 8 more reads of 5 of the same nodes.

The 8 come from PR 6484's rewrite of `arc_rim` in `topo::pcurves::nurbs_iso_derive`:
- It now certifies the picked rim (`PcurveCache::certify`) to choose its lane (D3).
- It keeps only the pcurve (`Ok(_) => Ok(rim)`) and drops the certificate.
- The minting walk's `certify_walked` then certifies the same rim again, on the same interval, carrier, chart and band.

So every arc rim on a spline chart is certified twice per mint.

The test runs only in the slow set, and PR 6484 does not spell its name, so its CI never ran it. That is the class in `work/ciw/tree-wide-source-censuses-in-the-slow-set-run-on-no-pr-that-moves-a-site.md`.

`main-red-a-lofted-arc-walls-rim-zeros-count-eighteen-not-ten` is the same red, filed the same day by another lane, and is closed into this row.
