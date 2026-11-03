# Review: PR 6478 (`spiricarm/rotational-lane-cylinder`), the rotational-chart Green lane, stage 1 (cylinder)

You are one of two independent Opus reviewers of evgunter/cad PR 6478. It is a FULL review: claims to falsify, plus the style lane. **The frozen head is `e12e3c8d0017f9b83be830d4881f580afcb28c9d`.** Review exactly that commit and name it in your report. You run in your own cloud container, with the repository checked out at that commit.

## Read first
- `CLAUDE.md`.
- `docs/prompts/reviewer-style-lane.md` in full. It is binding: name which of its questions you exercised, and give every finding a `sure`/`likely`/`unsure` confidence.
- `docs/prompts/implementer-discipline.md`, for what the author owed.
- The unit's spec: `work/spiricarm/a-rotational-chart-green-lane-reads-projected-chords.md`. Read it at main, `git fetch origin main && git show origin/main:work/spiricarm/a-rotational-chart-green-lane-reads-projected-chords.md`, or in the checkout.
- `work/spiricarm/plan.md`'s lessons, which bind you too. A narrower bound is not evidence: containment is. A re-baselined golden is not a re-pointed row.
- The PR description, through the GitHub tools (PR 6478's body). Do NOT read its comments or reviews; see Isolation.

## Isolation (binding)
- Until your report is delivered, do not fetch, check out or read the other reviewer's branches (anything named `spiricarm-rev6478-*` other than your own), and do not read the PR's comments or reviews.
- Disclose any accidental glimpse in your report.
- Post nothing to GitHub.

## Method
- Probes that demonstrate a finding by execution are worth far more than inspection: a red test, a measured wrong value, a surviving mutant. Write them as ordinary tests.
- Use a `CARGO_TARGET_DIR` outside the checkout, and keep runs targeted (`-p topo`, `-p sweep` with a filter).
- **The foreground rule has two halves, and both bind.** Never arm background waiters for your own builds. Launch any job longer than ~9 minutes `setsid`-detached with its PID recorded, and poll it in the foreground. Never end your turn with background work running.

## Claims to falsify
1. **One lane, no second copy.** `quad/cylinder.rs` moved to `quad/rotational.rs`. The harmonic arithmetic is kept verbatim and is now one arm (`EdgeTermQ::Harmonic`) beside the projected arm. No parallel cylinder lane, edge loop, closure or pad remains anywhere. Grep for the copy (style Q1).
2. **The projected integrand is right and its enclosure is sound.**
   - `u′ = (xy′ − yx′)/(x²+y²)` in homogeneous coordinates on a net (the weight cancels).
   - Trig jets on a `Circle`; the `Line` arm.
   - `v = v_off + v_sign·z`.
   - Pieces cut at the image's sector breaks, so the ρ/pole/tube bounds hold per piece.
   - The Taylor-rule enclosure contains the true integral, including for a chord across the seam, longer than a quarter turn, or winding.
   - Try to break it with exact arithmetic or high precision.
3. **Closure.** `branch_turns` (a projected edge's Δu from the rule must agree with its evaluator ends to a whole number of turns) refuses rather than guesses on a disagreement, and cannot pass a wrong turn count.
4. **`ChartTermQ`'s `o·A⃗`.** The PR says it reads the carriers' vector area (via `rational_vector_area_split`), not the projected region's, "the existing harmonic convention, unpadded". Is that sound for a projected chord whose carrier lies off the cylinder by its fit deviation (1e-12 to 7.8e-11 m, ε-bounded at 1e-9 and 1e-6)? Is the gap actually covered, or is it an uncontained error in a certified bracket?
5. **The oracles witness containment.**
   - The carrier-swap rows (`quad/rotational/swap_rows.rs`) each build a face whose exact answer is known by the harmonic lane, and assert enclosure.
   - A mutant dropping `v_sign` reds the mirrored row; try other mutants (sign of `u′`, a dropped piece, an off-by-one turn).
   - The sliced oracle (`sweep/tests/common/crossed_cylinders.rs`) is exact, and validated against Steinmetz. Is it independent of the code under test?
   - No inclusion–exclusion check anywhere.
6. **`differential::resolution_pad` (`res·area/2`)** is a sound ceiling for rows that accept at round 0, and does not loosen any row that was already tighter.
7. **The SOUND rows are SOUND**: drum × rod (four ops), the fenced unequal and skew unions, the review probe's poses, `parallel_cylinder_join`'s turned pose, and the blend reorder's fillet leg. Each is checked against an oracle, not a re-baselined constant. The tilted-rod ∪, which builds at the right volume but refuses at the census, is pinned exactly, and its filed row (`work/inside/the-census-reads-no-ruling-inside-a-chorded-cylinder-face.md`) is real.
8. **The re-pins move only to the next stop.** The rows are `cylinder_sphere_frame`, `torus_doors_join`, the seam-region door stops, `germ_tilted_rod`, `germ_torus_components` and `germ_radial_hole`. Merging the pins into one word per chart kind (`Door::Sphere`, `SPHERE_FACE`, `TORUS_WALL`) must not hide a body that changed.
9. **The renamed refusal.** "a trimmed face with no volume lane on its chart" is re-pinned at every `starts_with` site; grep for any missed one, demos included. The stale messages and comment are fixed.
10. **The P1 cost row's rewrite** (`work/spiricarm/a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses.md`) states the measured cost honestly.

## Deliver
Write your report (≤150 lines) as your FINAL message. Also commit it as `review-6478.md` at the repository root on a new branch named `spiricarm-rev6478-<your label>/report`, and push that branch. Your label is in your first message. Push probe tests to `spiricarm-rev6478-<your label>/probes` if you write any.

The report contains:
- The verdict, from the ladder: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT.
- Findings as MAJOR, MINOR or NOTE. For each: `file:function`, what is wrong, the evidence (execution or inspection), and the confidence.
- A `## Style` section.
- Which claims you upheld or falsified, and how.
- Your disclosures.
