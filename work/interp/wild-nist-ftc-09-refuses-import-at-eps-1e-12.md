---
id: wild-nist-ftc-09-refuses-import-at-eps-1e-12
kind: issue
title: The wild montage's pinned nist_ftc_09 cell refuses STEP import at eps 1e-12 (start-endpoint residual past the band)
status: open
opened: 2026-10-03
priority: P3
cost: M
---


Met by the REACH lane `reach/lily-leaf-1e12` while running
`demos/wild` (`wild-montage <outdir>`) at the three ε rows to check its
mass reading. At 1e-9 and 1e-6 all eight cells import and measure. At
`CAD_TOLERANCE_EPS=1e-12` the run panics in `run_cell` at its import
`unwrap_or_else`, on the sixth cell:

```
nist_ftc_09_asme1_rd: nist/nist_ftc_09_asme1_rd.stp is a PINNED montage cell and must import;
it refused: step import: edge #2928: no intensional description certifies — seam: geometry
attachment gate: the start-endpoint residual at sample 0 definitely exceeds the tolerance band
(the cache does not represent the description, D4 ¶2). There is no way through: this is a
kernel defect or a damaged file; report it; mapped curve: geometry attachment gate: the
start-endpoint residual at sample 0 definitely exceeds the tolerance band (...)
```

The refusal text asks to be reported, so it is. Unmeasured beyond the
payload: whether the import's band at 1e-12 is the file's declared
uncertainty (`eps_in`, `ImportOptions::default()`) or the run's ε, and
so whether this is an honest refusal of a file stated to a looser
uncertainty than the run's band or a band chosen from the wrong ε.
Nothing in CI runs the wild montage at any ε but the default
(`render.yml`'s wild job), so no row is red today.

Reproduce: `cd demos/wild && cargo build --release` then
`CAD_TOLERANCE_EPS=1e-12 <target>/release/wild-montage /tmp/wild`.
