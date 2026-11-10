# Review: PR 6806 (`spiricarm/horn-cusp`), a horn's cusp revolved part way builds as its lune

You are one of two independent Opus reviewers of evgunter/cad PR 6806. This is a FULL review: falsify the claims below and run the style lane. **The frozen head is `0c3420d4cb983431637bb3dfbe16586c2a30ee40`.** Review exactly that commit and name it in your report. You run in your own cloud container, with the repo checked out at that commit.

## Read first
- `CLAUDE.md`.
- `docs/prompts/reviewer-style-lane.md` and `docs/prompts/implementer-discipline.md`, both IN FULL. Read them in chunks if your tool truncates; the style lane is binding.
- DESIGN [verdicts], [loud-failure] and [surfaces], the last including its locus sentence.
- The rows `work/spiricarm/a-horns-cusp-revolved-part-way-refuses-at-its-cap-meridians-certificate.md` and `work/spiricarm/a-wide-apples-tube-angle-passes-the-latitude-windows-cut.md`.
- The spindle review rows `docs/dual-review-log/rows/PR-6473.md`, `PR-6486.md` and `PR-6497.md`, and PR 6521's body. Their classes recur: an unguarded guard, a wrong-X witness refused by an earlier gate, and a mutant declared equivalent without running every row.
- The PR body, through the GitHub tools. Do NOT read its comments or reviews.

## Isolation (binding)
- Until your report is delivered, read no `spiricarm-rev6806-*` branch but your own, and none of the PR's comments or reviews.
- Disclose any accidental glimpse.
- Post nothing to GitHub.

## Method
- A probe that shows a finding by execution outweighs inspection.
- Use a `CARGO_TARGET_DIR` outside the checkout, and keep runs targeted.
- **The foreground rule has two halves, and both bind.** Never arm background waiters for your own builds. Launch any job longer than about 9 minutes `setsid`-detached with its PID recorded, and poll it in the foreground. Never end your turn with background work running.

## Claims to falsify
1. **`Meters::at_height` is a sound upper bound on a box's distance from THIS sheet.** It reads a part at its own height. On an apple or a horn, every height with h ≤ |z| < r has a sheet point at ρ_s(z) = R − √(r² − z²), computed as (z² − h²)/(R + √(r² − z²)). The bound is max(ρ_hi − ρ_s(z_near), ρ_s(z_far) − ρ_lo). Fuzz it against the exact distance on horns, apples and near-horns; boxes beside the cusp, straddling h, and near |z| → r; and points on the other sheet.
2. **The three guards (R > 0, |z| > h, |z| < r)** go through the caller's `positive`. The R > 0 guard is claimed equivalent, by the proof in the PR body. Check the proof, and run that mutant over EVERY row of the subject, not only the PR's set.
3. **`Meters::certified` takes the least of `over`, `at_height` and `apex_root`.** Each must be an upper bound, so their minimum is sound. Check that the cone, the torus and the other kinds are unchanged, and that `over` really is byte-identical to main.
4. **`ChartAngle::window` → `Option<Window>`, with `SheetTube` read by `Azimuth::near(0)`.** Does this change every sheet's `v` reading at `Interval`, and is every reader still right, including the latitude-window readers that a sphere uses? At the tie at ±π the reading takes +π. Is anything that reads a horn's cusp `v` at `Interval` mis-handled?
5. **`mesh/src/walk.rs`: a pole at exactly π on a chart whose `v` does not wrap takes the sign of the meridian leaving it.** Is this right for every chart kind that reaches it, sphere included, and not only for the horn?
6. **The oracles.**
   - The lunes (needle, lower needle, four-arc horn) mesh watertight inside their wedge, pass tier 3, and match θ/2π·V and θ/2π·A plus two caps, computed independently.
   - The wide apples now mesh.
   - `limb_2_against_a_horn_reads_the_sheet_up_to_its_cusp` reaches the gate it names. Check that a mutant weakening the height reading goes red there and not only in the fuzz.
7. **The fuzz oracle's change**, to a robust `m − r` and the definitional side condition. Is the new oracle exact, and independent of the code under test?
8. **Residue.** The PR files no rows. Are the out-of-fence finds really already on a slate or unread? Do both rows meet their done-whens?

## Deliver
Your FINAL message is your report, ≤150 lines. Also commit it as `review-6806.md` at the repo root on a new branch `spiricarm-rev6806-<your label>/report`, and push it. Your label is in your first message. Push probes to `spiricarm-rev6806-<your label>/probes`.

The report contains:
- a verdict from the ladder: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT;
- findings as MAJOR, MINOR or NOTE, each with `file:function`, what, evidence and confidence;
- a `## Style` section;
- the claims you upheld or falsified;
- your disclosures.
