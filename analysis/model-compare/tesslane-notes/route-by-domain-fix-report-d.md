Part (d) of 4: what ran, and what is still to run.

**On the frozen head 3494faec95:**
- `scripts/run-gates.sh` (06:53, after the head was committed at 06:47): green, cover-kind-match at 10. doc-gate and `work.py lint` are green.

**On earlier fix-pass heads.** The code delta from these heads to the frozen one is the torus lever's lazy site (218e632dd3), main's merge, and the filing note.
- mesh suite: 495/495 green (05:34, run with the budget feature). The cost run measured 184.4 s on main (483 tests) and 193.8 s on the branch (489 tests).
- Armed sweep: 3762/3764 green, no route-difference panic. The two reds are `mass_props_are_thread_count_invariant`, which is also red on `origin/main` 6396c45978.
- Armed tour: 106/106 green, 0 differences.
- `route_rows` at ε 1e-9, 1e-6 and 1e-12, against constants read on a main worktree: equal.
- Mutants: as tabled in the PR body.
- geom-brep and topo: their last full runs (04:03 and 04:27) predate the `Defect` re-pins and the merge of PR 7051. The reds those runs showed were fixed in later commits (the torus re-pin) or by main (the topo census, PR 7051), but neither suite has been re-run whole since.

**Result to follow:**
1. A whole re-run on the frozen head of geom-brep, topo, mesh and step-import (including `split_iso_side`), so that every suite has a result on the frozen sha.
2. The tess-budget sweep (`--sizing-only`, release) on main and on the frozen head, with wall times and a byte-compare of the CSVs. This replaces the 2026-11-16 cost line in the PR body.

I am pushing nothing.
