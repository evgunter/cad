# METER log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/meter/plan.md`.

## Opened (2026-09-06)

Opened in the tracker-wide cut of 2026-09-06 (Ev's direction,
in-chat; `docs/WORK-TRACKS-2026-09.md` addendum 2), with GATES, the
two halves of code-quality Track K claimed whole. Eleven rows moved
by `git mv` with a `## Claimed by` record each: eight with `track: K`
and three unlettered rows on K's `tools/*` fence. `S115`(a) (the inert
`agree` column) and `S29` (the sizing policy) stay in
`work/code-quality/` — the first as a member of a roll-up this
program will take as a rider on unit 1 if the roll-up is not split
first, the second as S-MESH's design PR. No branch exists yet; the
first dispatch is unit 1.

## Orchestrator seated; review posture set; slate re-read (2026-09-07)

Ev seated an orchestrator on this program in chat and set its review
posture in the same breath: **no A/B protocol** — most units are easy —
**a style review each, and a full falsification review only for the
units that are genuinely tricky.** `plan.md`'s *Review posture* now
carries that, with the two units that qualify named and the reason each
qualifies stated: `D206` and `k-report-baseline-fold-cert1-roster`, both
because they move committed measurements other lanes read. The A/B band
3200–3299 stays claimed for bookkeeping and no row will be written to
`docs/MODEL-AB-LOG.md` from this program.

The slate was read end to end against the tree before the first
dispatch, and three things came back that the tracker did not know.

**The join item's fix has landed and the item does not know it.**
`tools/tess-lint/src/lib.rs`'s `Kind::Rekeyed` / `Rekey` under
`compare`'s rule-4 precondition closes both branches
`tess-lint-face-ordinal-join` names. `plan.md`'s step 8 still described
that tripwire as unlanded, under the name `Kind::Reordered` it was
proposed with; the sentence is corrected and the item becomes **unit
0**, because reading it as open mis-states what `C15` has left to do.
The unit is not pure bookkeeping: `compare`'s `gated` split routes a
re-key on a scene with no Hessian-sized face to `notes` rather than
`findings`, which is #738's `diefillet` exactly, so the permutation the
item was written around is reported in the lint's quietest voice and
nobody has recorded whether that is right.

**The stale-doc item's own table has gone stale.** Re-derived against
the baseline as committed today (cut `aba2625f8f84`, 2026-09-04, later
than the `a4eb03a` the item read): 1353 rows, 1,552,822 triangles, the
64 NURBS faces at 4.7% carrying 10.6%. The item said 1306 / 1,416,410 /
4.9% / 11.6%. That is the item's own mechanism firing on the item, four
days after it was filed, and it settles both open questions in the
file — the block is stale against re-cuts rather than describing an
older sweep, and the fix is to CITE `tess-lint`'s report header, never
to commit a fourth transcription. Recorded on the item.

**A pin cannot close `k-lint`'s roster in the direction that matters.**
`k-lint-predicate-roster-unpinned` reaches for `D204`'s `include_str!`
remedy, which worked there because `CHART_TAGS` and `Chart::tag` are
the same finite set. `EPS_COUPLED_PREDICATES` is a SUBSET selected from
an open vocabulary by a property that is written down nowhere, so a pin
closes the RENAMED direction and cannot close the ADDED one at all.
Filed as `k-lint-eps-coupled-criterion-unwritten` so the pinning lane
is dispatched knowing it, rather than discovering at review that its
claim is narrower than the item it cites.

Dispatch order from here: unit 0 and unit 1 both sit in
`tools/tess-lint/src/lib.rs` and are sequenced rather than run
together; unit 2 is disjoint (`docs/TESS-BUDGET.md`) and runs beside
whichever of them is live. `D201` goes out as an `[ev]` PR early rather
than at step 8 — it is the slate's one design fork and its long pole,
and the answer wants to be in hand before the cheap end is exhausted.

**Unit 1 (`D213` + `D214`) landed the sizing block as one decision.**
The two items pointed at the same seam from opposite sides and the
answer was one edit: `parse` refuses the lane pairing in both
directions (`SIZED_CHART_TAGS`, the consumer's mirror of
`tess_meter::FaceRow::csv_row`'s 2x2), which makes `Row::is_sized` a
function of `chart` on every parsed row, which retires
`IDENTITY_COLUMNS`' block-presence entry rather than re-justifying it —
`chart` is entry zero, so the block entry could never have been the
first disagreement rule 4 reports. `D213`'s mesh chain was re-walked
against the merge base, `per_cell_candidates`'s `?` included, and
holds. The census goes six-of-eight to five-of-seven.

Two things the wave should know. The pairing test derives its
expectation from `SIZED_CHART_TAGS`, so a member gained or lost moves
the expectation with it and passes in silence — the roster needs the
written-out literal pin `CHART_TAGS` already carries, and that is the
only guard on its membership; a cross-root pin against
`Chart::sized_lane` would want `tools/tess-meter/tests/derivations.rs`,
which is the meter's file and its ground, exactly as `CHART_TAGS`'s own
doc says of the ADDED direction. And `docs/TESS-BUDGET.md` enumerates
the identity list twice, both copies still eight entries — filed as
`tess-budget-doc-identity-column-list` rather than edited, because unit
2 holds that file for the wave.

**Unit 1, fix pass.** The review found the unit had disarmed a guard it
did not touch: `an_unknown_chart_tag_is_harness_breakage_not_a_re_key`
asserted only `text.contains("chart")` and `contains("hessian")`, and
the new pairing arm's `(false, true)` message names both — so deleting
`CHART_TAGS`' roster check left all 65 tests green on this branch while
the same deletion reds on `main`. Reproduced in both directions before
the fix. The refusal is now compared WHOLE and on both row shapes (the
unsized one, where the pairing has no opinion, is what the fixture was
missing), the two lane-pairing tests compare whole messages for the
same reason, and the vacuous `if let Err(e) { assert!(!contains(..)) }`
loop is gone — its positive content is
`every_chart_tag_owes_the_sizing_block_or_refuses_it`, which reads both
shapes per tag.

The sweep behind it is mechanical and is the instrument this class
wants: **delete each refusal in `parse`, one at a time, and record
which tests red.** Ten mutations over nine guards. Before the fix, one
guard — the roster — had no red; every other guard reds at least one
row, and the attribution is right in each. After, the roster reds two.
Refusal ORDER is now pinned too (roster before pairing, pairing before
`face`), proved by swapping the two blocks; nothing pinned it before.

`SIZED_CHART_TAGS`' membership is no longer guarded only by a literal
inside the test that reads it: the cross-root pin was WRITTEN rather
than filed —
`tess-meter`'s `the_lints_sized_roster_answers_sized_lane_for_every_tag_this_crate_emits`
reads the declaration out of `tess-lint`'s source and asserts
membership equals `Chart::sized_lane` per tag, with a falsification
guard covering a roster short a sized tag and one carrying an unsized
one. Per-tag biconditional rather than equality, for the same reason
`CHART_TAGS`' pin is containment: the lint parses baselines cut from
older trees, so a retired tag must stay in both rosters.

Also this pass: `CHART_TAGS`' doc said closing its one-way asymmetry
was "`tess-meter`'s ground" when `the_lints_roster_admits_every_tag_this_crate_emits`
had already closed it — a stale sentence that steered the deferral
above. `D214`'s history figures did not reproduce and were re-taken (20
blobs, not 127; no pre-`triangles` header shape exists; the conclusion
is unchanged). A third copy of the identity-column enumeration turned
up in `local-scripts/ci-local.sh`'s hosted-mirror comment and is fixed
here; `C15` and `D201` restated it too and are corrected. `D213`'s
close is rewritten to lead with the argument that carries it — a
property of `parse` and `first_disagreement`, no kernel content — with
the mesh chain demoted to what it is actually for.
