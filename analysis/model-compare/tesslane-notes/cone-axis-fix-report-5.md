cone-axis-winding — items 1 and 2 pushed as ONE head: 1026f8c3b3b33abe69c395303b45f32a7c916851 (confirmed with ls-remote). The merged-run cure is not built here.

Since 22e7ef6ffd:
- 492b123f91: the default-ε pin `split_rulings::a_draft_cone_ruling_split_within_the_developments_resolution_refuses`; the 1e-12 pin's doc corrected; the class row's doc stating the thin planar neighbour as the class; the thin-arc row's evidence rewritten to the class.
- 1026f8c3b3: `a-ruling-split-within-the-developments-resolution-merges-two-points` at P1. Its cure section is now one line naming `work/tesslane/a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation.md` (not created here; lint does not resolve prose), plus one line on the strip unit. It carries no soundness analysis.
Pre-push on the head, all green: fmt, run-gates, `cargo clippy -p mesh --all-targets --all-features`, mesh whole with the slow set 504/504, and the split-ruling, T1, side-guard, multiply-carried and split-placed rows at ε 1e-6 and 1e-12 (10/10 each).

Reproduced numbers, unchanged from my last note. Wall (1,0),(2+tan α,0),(2,1),(1,1), full turn, seam split, δ 0.01, default ε, gaps on a 1–3 grid. Largest gap that merges / smallest gap that meshes, as absolute lengths on a ruling about 1 long, against 2⁻²⁷·r/sin α at r = 2:
- 0.01°: 3e-5 / 1e-4, bound 8.5e-5 (merges at 0.35 of it);
- 0.1°: 1e-5 / 3e-5, bound 8.5e-6 (1.17);
- 1°: 3e-7 / 1e-6, bound 8.5e-7 (0.35);
- 3°: 1e-7 / 3e-7, bound 2.8e-7 (0.35);
- 26.6°: 1.1e-8 / 3.4e-8, bound 3.3e-8 (0.34), two splits mid-ruling only; the end split at 1.1e-8 meshes.
An end split and two close mid-ruling splits answer alike at every α but 26.6°.

The changed PR-body paragraphs are exactly the ones in my 09:3x message: the two replacement bullets under "**What meshes on the base and refuses on the head**" and the residuals bullet under "## Rows". One correction to the second bullet's last sentence before you paste it: "Filed at P2" becomes "Filed at P1, its cure specified in `a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation`".
