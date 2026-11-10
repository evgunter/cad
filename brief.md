# Review: PR 6805 (`spiricarm/cylinder-pair-cost`): the rotational lane spreads a round's pieces over a carrier's spans

You are the single FULL reviewer of evgunter/cad PR 6805: falsify its claims, and run the style lane. **The frozen head is `c2b6282453873e528c6bbe1170793874b59477f9`.** Review exactly that commit and name it in your report. You run in your own cloud container, with the repo checked out at that commit.

## Read first
- `CLAUDE.md`.
- `docs/prompts/reviewer-style-lane.md` IN FULL. It is binding, so read it in chunks if your tool truncates.
- `docs/prompts/implementer-discipline.md` IN FULL.
- `work/spiricarm/plan.md`'s lessons. A narrower bound, or a faster path, is not evidence; containment is.
- The rotational lane's review rows: `docs/dual-review-log/rows/PR-6478.md` and `PR-6509.md`. These classes recur: unwitnessed pads, circular expected values, a mutant declared equivalent without running every row of its subject.
- The row `work/spiricarm/a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses.md`.
- The PR body, via the GitHub tools.

Post nothing to GitHub.

## Method
- A probe that demonstrates a finding by execution outweighs inspection.
- Use a `CARGO_TARGET_DIR` outside the checkout, and keep runs targeted.
- **The foreground rule has two halves, and both bind.** Never arm background waiters for your own builds. Launch any job longer than about 9 minutes `setsid`-detached, record its PID, and poll it in the foreground. Never end your turn with background work running.

## Claims to falsify
1. **`span_pieces` changes only how many pieces a round reads, never a bound.** Each span gets `ceil(16/spans) << round` pieces. The Taylor rule must stay sound at any piece count: every enclosure must still contain the truth at round 0 with one piece per span, on a span that is long or strongly curved. Try to break containment with a coarse round 0, for example a chord span whose angle turns a lot, a sphere chord near a pole, or a cylinder chord across the seam.
2. **It is applied consistently** in `cylinder_edge_terms`, `sphere_edge_area` and `chart_term_at`, and `face_cost` counts the same pieces. Check that no other reader of a carrier's pieces was missed: grep for the shape, not the symbol.
3. **"Volumes identical to 12 digits on all six ops; flux widths unchanged to three digits."** Check this is a measurement and not an assertion, and whether anything guards it. Note that narrower or identical is not evidence; containment is.
4. **The new rows can go red.** `a_rounds_pieces_spread_over_a_carriers_spans` and `a_harmonic_faces_work_is_its_edges_pieces` would catch a wrong schedule. `a_section_net_of_many_spans_holds_the_closed_form` (exact rational sections on 20 and 24 spans) checks containment against an independent truth. Run mutants: the old per-span count, a schedule that under-reads, a dropped span.
5. **No row was re-pinned or loosened.**
6. **The sphere sweep rows** (`cylinder_sphere_frame`, `verbs_cylsph_opening`) have not run since the sphere lane merged in, because the gate does not select them. Run at least `verbs_cylsph_opening` and one `cylinder_sphere_frame` pose at this head, and report.
7. **The filed rows are accurate and correctly homed and banded**: the PCERT row (P1), the ONEHOME row (P2) and the two SPIRICARM rows (P3). Check them against `work/README.md` Priority.
8. **The kept probe** `crates/sweep/examples/cylinder_pair_cost_probe.rs` is appropriate as an example, compiles under the gate, and leaks nothing into the library.

## Deliver
Your FINAL message is your report, at most 150 lines. Also commit it as `review-6805.md` at the repo root on a new branch `spiricarm-rev6805/report`, and push it. Push probes to `spiricarm-rev6805/probes`.

The report gives:
- a verdict from the ladder: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT;
- findings as MAJOR, MINOR or NOTE, each with `file:function`, what is wrong, the evidence and your confidence;
- a `## Style` section;
- which claims you upheld and which you falsified;
- your disclosures.
