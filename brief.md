# Review: PR 6521 (`spiricarm/spindle-partway`), the football revolved part way builds as its lune

You are the single FULL reviewer of evgunter/cad PR 6521: claims to falsify, plus the style lane. **The frozen head is `c974bfa84d556d9d49c144cba17c0021569605f9`.** Review exactly that commit and name it in your report. You run in your own cloud container, with the repo checked out at that commit.

## Read first
- `CLAUDE.md`.
- `docs/prompts/reviewer-style-lane.md`, IN FULL. Read it in chunks if your tool truncates. It is binding: name the questions you exercised, and give each finding a `sure`/`likely`/`unsure` confidence.
- `docs/prompts/implementer-discipline.md`.
- `docs/DESIGN.md`'s [verdicts], [loud-failure] and [tolerance.*] clauses.
- The row `work/spiricarm/a-football-revolved-part-way-refuses-at-its-cap-meridians-certificate.md`.
- The spindle stages' review rows, `docs/dual-review-log/rows/PR-6473.md`, `PR-6486.md` and `PR-6497.md`. Their classes recur: a guard nothing goes red without, a wrong-X witness refused by an earlier gate.
- The PR body, via the GitHub tools.

Post nothing to GitHub.

## Method
- Probes demonstrated by execution are worth far more than inspection.
- Use a `CARGO_TARGET_DIR` outside the checkout and keep runs targeted.
- **The foreground rule has two halves, and both bind.** Never arm background waiters for your own builds. Launch any job longer than about 9 minutes `setsid`-detached with its PID recorded, and poll it in the foreground. Never end your turn with background work running.

## Claims to falsify
1. **The apex reading** in `projected.rs` (`Meters`' sheet arm) is a sound lower bound on the distance from THIS sheet. On the one stretch of an edge that touches an apex, the bound is the distance from the nearer apex, which is a point of the sheet. The cone's `√|f|` would wrongly accept a curve on the other sheet, which meets this one at the apexes. Fuzz it near both apexes, on lemon, apple and near-horn sheets, with curves on this sheet and on the other one.
2. **The certified-interval fallback (`Meters::certified`).** Where the band cannot decide a sheet stretch's lever, the certificate takes the hull's own reading, read positive off its certified interval (`lower > 0`). Questions:
   - Is that a raw comparison on a control-flow path, against DESIGN [verdicts] "no raw `<` on control-flow paths"?
   - Or is it, as the PR argues, only a choice between two certified upper bounds, with the band decision on the result unchanged?
   - Can the fallback ever certify an edge that the band-decided reading would refuse as off the sheet?
   - Is the cone, torus or any other kind's behaviour changed?
   Give a reasoned verdict. If you judge it a real tension with ratified text, say so and state the problem rather than a fix.
3. **The horn pinch guard** in `sweep::revolve::axis::classify_segment`. A horn's one-segment circle with its vertex at the cusp, revolved part way, refuses `NonManifoldAxisContact`. Check:
   - the guard catches exactly that case and nothing else;
   - it cannot mis-classify a needle, a near-horn, or an ordinary lune;
   - it is consistent with the full revolve's existing refusal.
4. **The lune oracle.** `a_football_revolved_part_way_is_its_lune` checks each lune (lemon, apple, slim lemon, past a half turn, negative angle) against `θ/2π` of the closed-form volume and sheet area, plus two caps. Check:
   - the oracle is independent of the code under test;
   - the caps are computed exactly;
   - every row goes red on a wrong lune, such as the wrong θ sign or the wrong sheet.
5. **The witness row reaches the gate it names.** `limb_2_against_a_sheet_reads_that_sheet_up_to_its_apexes` calls the edge certificate directly, because at the build door the per-sample residual refuses an off-sheet curve first. Check that the off-sheet curves really do refuse at the certificate, as `KernelLimit` against `Spindle`, and that a mutant weakening the side check goes red there.
6. **The mutants.** Rerun the PR's eight. M7 (`> 0` → `>= 0`) is said to be equivalent; is it? Then look for a ninth that survives.
7. **Cost.** A part-way football takes about 0.85 s in debug against 0.03 s for the full revolve, from linear halving at the four apex stretches (about 30–40 rounds). Is it bounded, and does any input make it run long?
8. **Residue.**
   - Is `a-horns-cusp-revolved-part-way-refuses-at-its-cap-meridians-certificate` (P2) accurate, and rightly banded under `work/README.md`'s Priority for a user-drawable revolve? Its `KernelLimit` message may read as a kernel defect.
   - Is `limb-2s-floor-on-a-sheet-reads-the-whole-torus` (P3) accurate?

## Deliver
Your FINAL message is your report, at most 150 lines. Also commit it as `review-6521.md` at the repo root on a new branch `spiricarm-rev6521/report`, and push it. Push probes to `spiricarm-rev6521/probes`.

The report contains:
- a verdict from the ladder: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT;
- findings as MAJOR, MINOR or NOTE, each with `file:function`, what, evidence and confidence;
- a `## Style` section;
- the claims you upheld or falsified;
- your disclosures.
