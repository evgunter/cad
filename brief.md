# Review: PR 6486 (`spiricarm/spindle-football`), Surface::Spindle stage 1 piece 2: the football with conical ends builds

You are one of two independent Opus reviewers of evgunter/cad PR 6486. It is a FULL review: claims to falsify, plus the style lane. **The frozen head is `49d88cc0f11bc1cce1ef9ef4e8a479263af7fd92`.** Review exactly that commit and name it in your report. You run in your own cloud container, with the repository checked out at that commit.

## Read first
- `CLAUDE.md`.
- `docs/prompts/reviewer-style-lane.md` in full. It is binding: name which of its questions you exercised, and give every finding a `sure`/`likely`/`unsure` confidence.
- `docs/prompts/implementer-discipline.md`, for what the author owed.
- The design Ev ruled on (PR 6464): `work/spiricarm/horn-and-spindle-tori-have-no-representation.md` `## Ruled`, `crates/topo/README.md`'s structural conventions (the arc rule), and `docs/DESIGN.md` [surfaces].
- The unit: `work/spiricarm/the-spindle-sheet-away-from-its-apexes.md`.
- Piece 1 (PR 6473, merged) and its review: `docs/dual-review-log/rows/PR-6473.md`.
- The PR description, through the GitHub tools (PR 6486's body). Do NOT read its comments or reviews.

## Isolation (binding)
- Until your report is delivered, do not fetch, check out or read the other reviewer's branches (anything named `spiricarm-rev6486-*` other than your own), and do not read the PR's comments or reviews.
- Disclose any accidental glimpse in your report.
- Post nothing to GitHub.

## Method
- Probes that demonstrate a finding by execution are worth far more than inspection: a red test, a measured wrong value, a surviving mutant. Write them as ordinary tests.
- Use a `CARGO_TARGET_DIR` outside the checkout, and keep runs targeted.
- **The foreground rule has two halves, and both bind.** Never arm background waiters for your own builds. Launch any job longer than ~9 minutes `setsid`-detached with its PID recorded, and poll it in the foreground. Never end your turn with background work running.

## Claims to falsify
1. **The revolve's classification.** `ArcInterior` (does the arc reach its circle's point of least radius), then `ArcReach` (`rc − r`), together pick sphere, ring torus or sheet exactly per the ratified arc rule. In particular:
   - the in-band near-horn case refuses as a sliver;
   - an arc whose interior crosses the axis refuses (`ArcCrossesAxis`);
   - an arc ending on the axis on a sheet's circle refuses `SheetApex`;
   - nothing that used to build (spheres, ring tori) changes its kind or its result.
   Look for an arc that is misclassified: near tangency, exactly on the axis, a sphere-class arc with rc in band.
2. **`Meters::Spindle` is a sound distance from THIS sheet** (not the other one) for edge certification. That includes a point near the other sheet, near an apex, or near the axis. The PR admits this is written but not fuzzed. Fuzz or probe it, and try to make it certify an edge that is off the sheet.
3. **The six sign fixes** for formulas that assumed R > 0 are correct, and the sweep for that class is complete:
   - pcurves: meridian azimuth, `b*`, spine distance `|R|`;
   - props: meridian fit `|R|`, meridian orientation, `about_core`.
   Grep for the shape, `major_radius`/`R` used as a radius or in a sign-sensitive place, in every file that now lets a sheet through, and report misses.
4. **Tier 3's walk on a sheet.** The new shape `g(v) = r(Rv + r sin v)` reads a sheet face's side correctly. The rows re-pinned from piece 1's `SheetApexReached` (outside, interior, inside an edge, clear) each go red with the walk unwired. An apex-containing face cannot pass.
5. **Props and mesh reuse the torus's closed forms, grid steps and certificate on a sheet**, and are correct there: signed R, lemon vs apple, `|v|` past π/2 on an apple. Check the volume/area closed forms and the mesh certificate on the lemon and the apple independently of the PR's oracle.
6. **The oracle** (`crates/sweep/tests/a_conical_ended_football_revolves.rs`):
   - `∮πρ²dz` / `∮2πρ ds` closed forms are exact and independent of the code under test;
   - the four bands (lemon, apple past π/2, lemon at 0.97·v*, lemon offset by 1e3) assert what they claim;
   - the wrong-sheet row goes red on a sign flip;
   - the mesh-volume check's chordal-deficit bound is right.
7. **Re-pinned rows move only where the change moved them**: `revolve_errors`, `one_segment_loop`, `bool3_torus_doors`, the rosters, `envelope_lemma_fuzz`'s nappe signature, and `door_roster`. No row was loosened to pass.
8. **The edge-certification path's signature change** (`Option<Nappe>` for a `ProjectedChart`, `SpanHull.rho_hi`) preserves every existing caller's behaviour.
9. **Residue.**
   - The closed LATHE row (`work/lathe/revolve-axis-clearance-is-the-torus-convention-under-another-name.md`) really is retired by this change.
   - The filed tour-stop row is real.
   - The wide-apple evidence (tessellation refuses `UnsupportedCurvedDomain`) is accurate, and no sheet that builds through the revolve yet produces a body the mesh or tier 3 mis-handles silently.

## Deliver
Write your report (≤150 lines) as your FINAL message. Also commit it as `review-6486.md` at the repository root on a new branch named `spiricarm-rev6486-<your label>/report`, and push that branch. Your label is in your first message. Push probe tests to `spiricarm-rev6486-<your label>/probes`.

The report contains:
- The verdict, from the ladder: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT.
- Findings as MAJOR, MINOR or NOTE. For each: `file:function`, what is wrong, the evidence, and the confidence.
- A `## Style` section.
- Which claims you upheld or falsified.
- Your disclosures.
