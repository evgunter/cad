DELTA REVIEW — ADDENDUM to claim 2 (05a7f8dbfd and the slow-set sweep). Same rules as before: alone, nothing posted, no model named.

**The account is right, and so are the re-pins.**
- From my delta report: both viewer rows are green on origin/main cdee746161. On the frozen first-pass head b6993486b8 and on 6f41f1b0ef they are red with identical numbers, so the first pass moved them and no fix-pass item did. My 318-cell mesh differential is byte-identical between the two heads.
- The budget table moves only at `cut_cylinder`, an analytic trimmed wall, consistent with "the clearance itself".
- I did not build `ba68f8ed0a` to reproduce the implementer's tally match there.
- The re-pins at 05a7f8dbfd equal, value for value, what I measured on both heads: (451512, 147856, 16596, 8142), plus the four `cut_cylinder` numbers 4520/7816 and 81208/84504.
- The diff 6f41f1b0ef..05a7f8dbfd touches only those two test files: 5 lines changed, 5 for 5.

**Slow-set sweep at 05a7f8dbfd: no other mesh-derived row is red.** How I chose the rows:
- Every crate that depends on `mesh` (editor-core, pncad, step-import, stl, sweep, topo, tess-meter; viewer through them).
- From `.config/nextest.toml`'s slow-set filter, I took every row of mesh (63), viewer (38), stl (3), pncad (2) and step-import (47).
- For sweep (810 slow rows) and editor-core (456), I took only the rows whose function body mentions tessellate, mesh, scene, fit_delta, Mesh or stl: 26 and 5. topo's and geom's slow rows matched none.

Results:
- **146 rows** across mesh, stl, pncad, step-import, sweep and editor-core with `--ignore-default-filter`: **146 passed**.
- **viewer `--features app`, all 38 slow rows: 38 passed**, the two re-pinned ones included.
- **demos/tour, the nightly step itself** (`cargo nextest run --release`, default profile, so the 9 nightly-only rows included): **106 passed**, 2 `#[ignore]`d.
- **demos/wild**: nightly runs fmt and clippy only, so no test pins a mesh number.

Not run:
- The tour's `certified` cells at ε 1e-12 (`eps_regression` covers the default and 1e-6).
- The die-corpus regeneration diff.
- sweep's and editor-core's slow rows that my grep did not select.
- Anything at ε 1e-6 or 1e-12 beyond those rows' own loops.

**Verdict at 05a7f8dbfd: APPROVE.** The MAJOR is closed by the re-pin. The NOTEs from my delta report stand: the clearance-0 row is a pin under a configuration `tessellate` never passes; the needle miss is filed P3, not cured; the margin is filed P2. Build output deleted; nothing committed.
