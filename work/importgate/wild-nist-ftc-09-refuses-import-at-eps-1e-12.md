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

- 2026-11-02 — moved from INTERP to IMPORTGATE by `git mv` when INTERP merged into IMPORTGATE, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and INTERP had fallen far under it).
- 2026-11-18 — `importgate/build` (the import builds every cache from the
  description it interprets) moves this file from refusing at 1e-12 to
  refusing at every ε, the default 1e-9 included: its plane × cylinder
  edges read tangent at the file's band (ε_in ≈ 34 µm) and apart at the
  run's, and refuse not-yet-supported naming the snap catalogue's lane (`tests/tier_gate.rs`'s `NIST09_SNAP`,
  `tests/wild.rs`'s refusal row, and `demos/wild`'s refusal pin, the cell
  leaving the montage until it imports). The snap unit, which makes a relation that reads Zero at the file's
  band exact, closes this row.

## After the snap catalogue (`importgate/snap`, 2026-11-19)

The catalogue makes all five of the file's tangent fillets exact
(`snap.rs`'s `nist09s_five_cylinders_snap_and_its_printed_tangent_edges_are_read_off_them`):

- `#4962`, `#4990`, `#5102` each sit between two parallel planes: radius
  set to half the spacing, axis moved to the mid-plane;
- `#5231`, `#5336` each sit between two crossing planes: axis translated
  to where the two offsets meet.

Every move reads Zero at the file's band, and every relation of the
class re-reads exact at the run's. Plane `#4976` is shared by `#4962`
and `#4990` and moved by neither.

The file still refuses, at every ε:

- the five tangent edges its writer printed as crossing lines lie
  `√(2·r·gap)` off the ruling now exact: 115–344 µm against the band
  (33.8 µm, 338 µm). `#4950` (180 µm, in band) is the first, pinned by
  `tier_gate`'s `NIST09_PRINTED_OFF_ITS_TANGENCY`. Filed:
  `a-tangent-edge-printed-as-a-crossing-line-refuses-at-the-build`.
- behind it, cylinder `#5139` stands 56.8 µm off planes `#5125` and
  `#5153`, in the file's band: no relation, and edges `#5118` / `#5132`
  refuse as the file's ambiguity, whatever the snap does.

The `demos/wild` cell stays out.
