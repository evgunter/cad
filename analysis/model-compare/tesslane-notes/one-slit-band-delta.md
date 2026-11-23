DELTA REVIEW — PR 7498 @ a5741dec87.

**Verdict: APPROVE-WITH-FIXES.** No blocking finding: the fix pass broke nothing I could find. The fixes are two small doc/row items under "Still open". This is conditional on hosted CI, which **has not run on this head**: "change filter" is queued, "prime the build cache" was skipped, and no test job exists. Someone must confirm the green.

## Item 2 — the cylinder disagreement: I was wrong
The implementer's instrument is right. Mine counted a patch as moved when its head digest was absent from the raw run. In the raw run the slit-band rows panic on their first δ, so their later bodies (the 5-arc plate at δ 3e-3 and 1e-3, the 3-arc plate at 3e-4) were never meshed there. Checked: FaceKey(7v1) at δ 3e-4 with n=59, FaceKey(8v1) at 3e-3 with n=12, and FaceKey(10v1) at 1e-3 with n=20 each appear in the head trace and **0 times** in the raw trace.

Re-taken with a better instrument on the merged head: metric 1 on every chart except the torus, so no refusal changes control flow. Over the whole mesh suite:

| chart | moved / total |
|---|---|
| cylinder | 0 / 7,749 |
| cone | 0 / 4,520 (one "move" was a run-to-run patch-multiset difference of one face, not a moved patch) |
| sphere | 237 / 2,987 |
| spindle | 8 / 25 |

The torus was not toggled by design. MINOR-1's cylinder half is **retracted**. Its spindle half (a suite row moves) stands, and the PR now says so.

## Item 1 — one rule on every chart: right
- **Is any sphere face worse under k = 2 or ½ than in raw UV?** No. `cell_metric` rounds the realized aspect to the nearest power of two in log, so the residual aspect is always in [1/√2, √2]. That is never further from 1 than raw: k = 1 whenever raw is already inside that band.
- **Is containment held in that band?** My earlier synthetic CDT (side chord points against lattice rows, m 1–10 chords against nv 1–8 rows) found no triangle past 1 column or max(1, nv/m) rows anywhere in [1/√2, √2]. The fan only starts below about 0.35.
- **Is the restated lemma true?** Yes. Containment is a property of which lattice cells a triangle's UV box covers, but whether Delaunay achieves it depends on the cell's aspect in the plane the CDT runs in. So "in the plane the CDT runs in" is the correct premise, and "does not depend on the plane" would be false. The curved doc's "under 2√2 on either axis" is the right two-sided form.
- **Correction to a fact in the new doc:** curved.rs says "a curved sphere face with rows reaches aspect 2.4, measured over this crate's suite" and the brief says 0.42–2.4. I measured raw aspects of **0.032 to 2.64** on sphere faces with nv > 1, including nu = 1 wedge slivers (u span 0.02, k = 1/32). So "A sphere's counts keep that premise in raw UV too" is false as written. It is harmless: a nu = 1 face has no interior column for a fan, and 2.64 < 2.83. The sentence is still wrong. MINOR, BY-MEASUREMENT.
- The one-rule ruling stands on `output-stability-as-justification`. A second rule to hold sphere bytes would be exactly the "kept because its output would change" shape that memory forbids.

## My earlier findings
- **MAJOR-1: fixed.** The `cell_metric`/`trimmed_metric` split, a `cdt_metric` home, and the `SPHERE_CELL_ASPECT` cap filed P4. New gap, MINOR, DEMONSTRATED: `the_cdt_cell_keeps_the_containment_lemmas_aspect_on_every_chart` calls `cdt_metric` directly. I made `tessellate_curved`'s call site read `trimmed_metric` instead. The aspect row stayed **green**, and only d9's sphere_wedge byte golden went red. The row pins the helper, not the lane. The stated mutant reddens it only because it mutates the helper's body.
- **MINOR-1: half retracted** (above). The spindle half is fixed in the body.
- **MINOR-2: fixed.** The `work/green` file is on main now and present at this head.
- **MINOR-3: fixed.** `max(1, nv/m)` is read per band.
- **NOTE-1: fixed.** No KernelLimit assertion remains, and ε identity is now two d9 corpus entries. NOTE-2 fixed.
- **Style:** the private FNV is removed and the class is filed P4 (fixed). The sphere-zone row is deleted (fixed). `read_band` literals are named as `CHART_READ` (fixed).
- **The two-arcs row** still passes under raw (k=1); I ran it. A row whose doc honestly says it is a witness on the harmless axis is acceptable only if something else covers that axis. Nothing does, so it is inert weight: a style **likely**, not blocking.
- **New style item:** d9's corpus now imports `crate::the_one_slit_band::filleted_plate`. A golden suite now depends on a helper hosted in a sibling row module, i.e. one consumer hosts the shared core (Q1). **unsure**.

## Re-pins on the merged head (compiled at a5741dec87)
- Whole mesh `all` binary, no filter: **309 pass, 0 fail**. That covers d9 (donut ×2, sphere_wedge, both filleted holes) and route_rows' donut.
- ε 1e-6 and 1e-12: the slit rows and d9 all green.
- Metric = 1 everywhere: the revolved row and the filleted row go red.
- **Viewer `review_pick_r2`: 147,844 at head**, and also 147,844 with the non-torus charts reverted. The sphere moves do not reach the tally, so the implementer's argument holds there.
- `sweep` was **not** re-run by me. That is an open gap for its curved sphere faces, unless CI covers it.
- `work.py lint` ok.

## Still open (fixes)
- The curved.rs sentence on sphere raw aspect, "2.4, keeps the premise in raw UV": false (0.032–2.64).
- The aspect row cannot see the call site; a lane-level relation would.

## What I executed and what I only read
Executed:
- the per-chart differential over the whole mesh suite (local-only env toggle, reverted; the tree is clean);
- the call-site mutant;
- ε 1e-6 and 1e-12 for the slit rows and d9;
- raw-everywhere mutant on the rows;
- viewer tally under two settings;
- lint;
- the re-check of my earlier cylinder evidence.

Read only:
- the d0e59643b8 and 2c8769ed04 diffs;
- the sizing.rs and curved.rs restatements;
- the swept-annulus ruling, which I accept: a NURBS wall in the trimmed lane;
- the check runs: none has run.
