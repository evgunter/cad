# Review: PR 6497 (`spiricarm/spindle-apex`), Surface::Spindle stage 2: the apex as a pole (the football builds)

You are one of two independent Opus reviewers of evgunter/cad PR 6497. It is a FULL review: claims to falsify, plus the style lane. **The frozen head is `c7142ee5772809d2720dfcfbb84d15293cbd6d04`.** Review exactly that commit and name it in your report. You run in your own cloud container, with the repository checked out at that commit.

## Read first
- `CLAUDE.md`.
- `docs/prompts/reviewer-style-lane.md` in full. It is binding: name the questions you exercised, and give every finding a `sure`/`likely`/`unsure` confidence.
- `docs/prompts/implementer-discipline.md`.
- The design, ruled by Ev in PR 6464: `work/spiricarm/horn-and-spindle-tori-have-no-representation.md` `## Ruled` (the horn is H1, the sheet's closed end), `crates/topo/README.md`'s structural conventions (the arc rule), and `docs/DESIGN.md` [surfaces] (including its locus sentence) and [loud-failure].
- The unit: `work/spiricarm/the-spindle-sheets-apex-is-a-pole.md`.
- Stage 1's reviews: `docs/dual-review-log/rows/PR-6473.md` and `PR-6486.md`.
- The PR description, through the GitHub tools (PR 6497's body). Do NOT read its comments or reviews.

## Isolation (binding)
- Until your report is delivered, do not fetch, check out or read any `spiricarm-rev6497-*` branch but your own, or the PR's comments and reviews.
- Disclose any accidental glimpse.
- Post nothing to GitHub.

## Method
- Probes that demonstrate a finding by execution are worth far more than inspection: a red test, a measured wrong value, a surviving mutant.
- Use a `CARGO_TARGET_DIR` outside the checkout, and keep runs targeted.
- **The foreground rule has two halves, and both bind.** Never arm background waiters for your own builds. Launch any job longer than ~9 minutes `setsid`-detached with its PID recorded, and poll it in the foreground. Never end your turn with background work running.

## Claims to falsify
1. **The horn snap.** When the revolve decides the reach `rc − r` to be zero, it stores an exact horn, `R = r`, instead of `R = rc`. The PR's reason: `R = rc` split a near-horn's apexes by about `√(2r·δ)`. Check that:
   - this is the kernel's existing practice at a kind boundary decided zero (the sphere class at `rc ≈ 0` mints `R = 0`), not hidden healing against [loud-failure];
   - the stored datum moves by at most the band;
   - nothing downstream is certified against the user's geometry when it is actually certified against the snapped geometry beyond a stated tolerance.
2. **The apex as a vertex.** At tier 3, an apex at a vertex passes as a pole does. `SheetApexReached` remains for an apex that is neither a vertex nor shown clear. A face whose apex is not a vertex cannot pass. The `edge_join` joinable-vertex changes are right, and nothing else in tier 3 or the join reads an apex as a regular point:
   - `chart_iso_family` reads the edge's own span;
   - `singular_at` names the sheet.
3. **The props Green form at an apex.** A loop passing an apex adds a zero-length rim at `v = ±v*`, oriented by the face's side. The flux reads its side from the sense bit, and the material sign reads both sides. For the football's two half-turn bands the material sign reports no side. Is the volume still certified correctly? Is "no side" a refusal, a silent default, or a correct reading? Try a face whose side the rule gets wrong.
4. **Mesh.**
   - Sheet faces take the pole-column rule.
   - `cert_spindle`, which drops the apex corner's `u` from the bound, is a sound chordal-deviation bound. Fuzz it near the apex on lemon, apple and horn.
   - `Chart::poles` lists the horn's cusp once.
   - The mesh is watertight with a fan at each apex.
5. **The oracles are independent and witness containment**: Pappus for the football, `π∫ρ²dz` for the needle, and the area. On the wrong-apex row, re-storing the faces on the other sheet must be refused at tier 3. Check that each of the six mutants listed in the PR body turns its row red, and look for a seventh that survives.
6. **The sweep (§5).** The PR reports 103 hits on a first pass and 69 on a second, the second aimed at single-variant arms behind `_ =>`, which is how `edge_join::singular_at` hid. Run your own, differently shaped sweep for code that treats an apex, pole or singular point by kind, and report any hit on the football's path or the apple's that mis-handles an apex.
7. **The re-pins.** `revolve_errors`, `one_segment_loop`'s horn row (now `Op`), the near-horn ladder's datum-shift allowance, and the door-roster and roster rows each moved only where the change moved them. Is `one_segment_loop`'s horn `Op` refusal a kernel-defect-shaped refusal of a user-drawable shape? If so, is it filed?
8. **Residue.**
   - The filed part-way row (`work/spiricarm/a-football-revolved-part-way-refuses-at-its-cap-meridians-certificate.md`, P3) is accurate. Is P3 the right band for a user-drawable revolve that refuses with a kernel-defect message?
   - The wide-apple evidence is accurate.
   - No body that builds is mis-measured or mis-meshed silently.

## Deliver
Write your report (≤150 lines) as your FINAL message. Also commit it as `review-6497.md` at the repository root on a new branch `spiricarm-rev6497-<your label>/report`, and push it. Your label is in your first message. Push probe tests to `spiricarm-rev6497-<your label>/probes`.

The report contains:
- The verdict, from the ladder: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT.
- Findings as MAJOR, MINOR or NOTE, each with `file:function`, what is wrong, the evidence, and the confidence.
- A `## Style` section.
- Which claims you upheld or falsified.
- Your disclosures.
