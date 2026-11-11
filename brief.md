# Review: PR 6808 (`spiricarm/rotational-lane-torus`), the rotational lane reads the torus chart (stage 3)

You are one of two independent Opus reviewers of evgunter/cad PR 6808. This is a FULL review: falsify the claims below and run the style lane. **The frozen head is `f87c193de862a2129cbab57f852870a5a8222cd4`.** Review exactly that commit and name it in your report. You run in your own cloud container, with the repo checked out at that commit.

## Read first
- `CLAUDE.md`.
- `docs/prompts/reviewer-style-lane.md` and `docs/prompts/implementer-discipline.md`, both IN FULL. Read them in chunks if your tool truncates; the style lane is binding.
- DESIGN [verdicts], [loud-failure], [surfaces] and C9 (nothing transcendental in certification arithmetic).
- The rows `work/spiricarm/the-rotational-lane-reads-the-torus-chart.md`, `a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane.md`, `a-torus-wall-holding-a-spiric-beside-a-spline-chord-has-no-volume-lane.md` and `the-spiric-wall-lane-omits-its-junctions-connectors-from-its-region-term.md`.
- The earlier rotational-lane review rows `docs/dual-review-log/rows/PR-6806.md` and the rows of the lane's stages 1 and 2 (find them with `grep -l rotational docs/dual-review-log/rows/`). Their classes recur: a guard nothing exercises, a witness refused by an earlier gate than the one it names, and a mutant declared equivalent without running every row.
- The PR body, through the GitHub tools. Do NOT read its comments or reviews.

## Isolation (binding)
- Until your report is delivered, read no `spiricarm-rev6808-*` branch but your own, and none of the PR's comments or reviews.
- Disclose any accidental glimpse.
- Post nothing to GitHub.

## Method
- A probe that shows a finding by execution outweighs inspection.
- Use a `CARGO_TARGET_DIR` outside the checkout, and keep runs targeted.
- **The foreground rule has two halves, and both bind.** Never arm background waiters for your own builds. Launch any job longer than about 9 minutes `setsid`-detached with its PID recorded, and poll it in the foreground. Never end your turn with background work running.

## Claims to falsify
1. **The count window holds the true whole-torus count `N`.** `torus_face` reads `X = s_f·ΣA` and holds every whole `N` with `X + N·4π²Rr` meeting `(0, 4π²Rr]`, the area clipped to the window and the flux at both end counts. Show that the true `N` is always among the counts read, for every `X` (straddling 0, straddling a whole torus, wider than a torus, narrow, exactly at an end). Check the window's closed end against the wrap loop (a one-face torus) and its open end against the empty face. Check the PR body's equivalence arguments for the skipped starting ends (`⌊−X.hi/A⌋`, `⌈1 − X.lo/A⌉`) and for reading the window directly at two counts or more. Check that a one-loop face decides nothing, and that the pinched-oval row goes red on the cause it names.
2. **The `v du` lift and the seam term.** Each edge is read on its own lift and carried onto the chain by `−o·2πκ·Δu`, including the branch turns. A loop winding the tube adds `W_v·2πκ·u₀`. Show that the integral is independent of the edge the chain starts at and of the branch each edge is named on (an end named a turn up or down). Check the junction connectors' levels (`Junction::levels`) and the lift's turns at the edge before's end. Check that `require_holes_wound_against` and `require_boundary_windings` refuse every ring layout the reading does not cover.
3. **The iterated `primitive_over` rule** (`taylor`). `y(x) = y(p) + ∫_p^x y′`, with jet `[y(x), y′₀, y′₁/2, y′₂/3, y′₃/4]` and spread `y′([p,q])·(t − x)`. Is it a sound enclosure at every resolution, and does it keep the `h⁵` width? The `y`-spread mutant survives and is argued: check the argument, and try to make it go red. Check that `v(t₀)` is the only transcendental in certification arithmetic (C9), that it enters through `Interval::from_certified`, and that `RationalJets`' weight slope `w′` is right at all three `rational_rim` callers, with no value changed.
4. **The spiric arm's equivalence to the deleted wall.** `EdgeTermQ::Spiric` replaces `spiric.rs`'s `WallChartQ` / `wall_windings` / `require_rings_wound_against` and `quad_lane`'s `spiric_wall`. Is every term the deleted wall read (integrand, `A⃗`, turns, length bound, cost, residual pad, junctions) read by the arm, and every refusal it raised either kept or replaced by a reading that is sound? Check that the ported rows test the same things as before.
5. **The per-span schedule on the torus** (merged from PR 6805). `piece_reach` has a torus arm, and the torus schedule bounds the tube angle's turn per piece as well as the azimuth's. Check that the schedule is sound for every torus edge kind (rim, meridian, projected, spiric), and that `primitive_each` / `rational_primitive_each` read each span at its own pieces.
6. **The chart's levers and pads.** The azimuth lever `R + r·cos v`, the level lever `r`, the flux density `≤ R + r`, the projected speed, and the corner pad from Bol–Fiala at radius `r` on the torus's metric, including where the curvature is negative.
7. **Guards.** The ring-torus guard (`0 < r < R`), the deck guard (`turns_name`), and the image `R` guard. The `lo`-only half of the `R` guard is claimed equivalent: check it.
8. **The oracles.** The torus swap rows' truths (Simpson densities, Villarceau `β/2π`, complements), `donut_slices.rs`'s sliced volume (cut at every corner and crossing, checked against a polar reading), and the re-imported vessel cavity. Are they independent of the lane and exact to better than the pads they are held to?
9. **Rows that moved and residue.** Every re-pinned row in the PR body's table. The implementer reports all 63 slow sweep rows of `sweep`, `step-import` and `topo` passing at this head, at the compiled ε (`--profile ci --ignore-default-filter`); rerun a sample if a finding needs it, not the whole set. Do the three filed rows meet the bar for filing rather than fixing, and do the four closing rows meet their done-whens?

## Deliver
Your FINAL message is your report, ≤150 lines. Also commit it as `review-6808.md` at the repo root on a new branch `spiricarm-rev6808-<your label>/report`, and push it. Your label is in your first message. Push probes to `spiricarm-rev6808-<your label>/probes`.

The report contains:
- a verdict from the ladder: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT;
- findings as MAJOR, MINOR or NOTE, each with `file:function`, what, evidence and confidence;
- a `## Style` section;
- the claims you upheld or falsified;
- your disclosures.
