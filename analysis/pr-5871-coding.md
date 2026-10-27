# PR 5871 dual review — blind coding inputs

The A/B labels were assigned by a recorded byte; the mapping is held by the orchestrator, not in this file.

Frozen head: 7c637654edd9e54fc5ff5a69cb6f33cb74353918 (merge-base 745edf4a). Arm: holdout (a concurrent pair), letter M. Both reviewers were dispatched at 18:27Z with one brief. Neither was interrupted, and both report that lane isolation held.

## Review A

REVIEW #5871 @7c637654 (FULL) — APPROVE-WITH-FIXES. MAJOR 0 / MINOR 2 / NOTE 5.

Lane isolation: I read no other review branch, no PR comments and no review scratch. I read the PR body and the diff against merge-base 745edf4a.
Probe branch: the reviewer's probe branch (<sha>), file crates/editor-core/tests/<probe file>.rs. To reuse its fixtures I made five helpers in the PR's own test file pub(crate). All 7 probes pass, and so do the PR's 4 rows and the dprime3 row.

## Claims
1. Exactly once, as the part's: HOLDS (executed).
   - probe_a_leaf_under_two_mids_is_reported_once: host→{mid1,mid2}→leaf gives the leaf's 5 once, through i1 via [mid1's inner].
   - probe_an_instance_under_a_boolean_still_reports: a host Union(instance, far block) gives the part's 5, verbatim.
   - probe_a_partly_proven_part_carries_its_own_verdict: an unproven Rest plus a reunited split in one part; the host's carried rows == the part's own findings.
   - Inside ⊔ vs not: insert_pinned (assemble_world_copies) and pin() are both exercised.
   - Pick: covered by the PR's Gap row.
   - Quieted by the part's own assertion: covered by the PR's row.
2. The part's door alone: HOLDS (read; executed for Gap).
   - DoorJob::run (checks.rs `DoorJob::run`) passes part.doc and part.rows to door_findings. Quiet comes from rows that `quiet_contact` built over the part's own evaluation (eval/parts.rs `evaluate_entered`). No host doc, evaluation or cfg assertion is read.
   - A host variable of the same name cannot reach it: the replay is over part.doc, and symbols are keyed per document. Read, not executed.
3. DoorJob soundness: HOLDS, with one MINOR (below).
   - Cancelable: probe_the_door_job_cancels. A host whose only rows are carried gets pending=[UnprovenCoincidence], a pre-canceled token gives Err, and settling clears pending.
   - One replay per (part, pin): probe_the_door_job_replays_once_per_part. P's rows rest a plate on an instance of Q, so every replay of P resolves Q. A counting resolver shows own job = 1 resolve, and a host with P twice = 1 resolve.
   - Pin: probe_two_pins_are_two_rows_each_its_own_document. One doc id at two pins (different labels) gives 10 rows. Each row.doc content_pin == row.pin, and each is said with its own version's label.
   - Resolve or replay failure: the part doc is held, so the door never re-resolves it. A replay failure folds into Proof::Unproven (loud) in coincide::prove. Read, not executed.
4. Host path unchanged: HOLDS (executed).
   - No instances means `carried` is empty, so the DoorJob is identical.
   - I blessed perf12 census goldens on head and on the merge-base (separate worktree and target) at ε 1e-6, 1e-9 and 1e-12. diff: all three SAME.
   - Unblessed on head, it fails at line 48, ε=1e-9, the inherited kitchen_sink drift.
5. Surface: HOLDS.
   - The viewer check_rows seats at Node(through), and coincidence_cells offers no buttons (frame.rs check_rows).
   - PartialEq ignoring doc is sound, since content_pin covers labels.
   - Holding `doc` in place of `held` loses nothing, because Speaker::of(doc) is a superset. Not executed in Python (no maturin build). I read the projection and the PR's py row.
6. dprime3 restated row: right contract. NOTE: it asserts count + variant + through only, not verbatim equality with the part's findings (atrest_carried_coincidence.rs covers that).

## Findings
MINOR-1, likely, read (claim 3, "off the evaluation path"):
- eval/parts.rs `evaluate_entered` now runs `checks::door_rows` and `checks::carried_coincidences` inside every part evaluation. That includes `quiet_contact`, which runs `holding_gaps` over the part's assertions for each census row. So it runs on the draw path, and inside every Sym replay that re-enters parts, whether or not a report is ever asked for.
- wire.rs `wire_instantiate_part` also deep-clones each row Vec per instance (`CarriedCoincidenceRows.rows` is a Vec, not an Arc). That runs against the parts.rs comment "every instance of one part shares one row set".
- The proving is off-path; the gathering and cloning are not. Not measured.

MINOR-2, sure, read (Q4: premises the PR didn't update):
- carried.rs `Route` doc: "One type for every carrier of that fact — CarriedInterference and CarriedAtRestFinding" does not list CarriedCoincidence(Rows).
- carried.rs `CarriedFindings` doc: "their at-rest findings, each already tagged with the route", but `coincidences` holds unjudged rows, not findings. "empty on every other op" is also false for placements (pre-existing).
- pncad-py checks.rs `CheckFinding.quiet` docstring cites only `CarriedInterference.of`.

NOTE-1, likely: CarriedCoincidence, CarriedCoincidenceRows and DoorJob derive Debug with an Arc&lt;ProfileDoc&gt;, so any `{found:?}` on a report now dumps whole part documents. Test failure messages will be huge.
NOTE-2, unsure: checks.rs `recourse_words` for CarriedCoincidence passes the host speaker (`self.1`) to the part's finding. It is harmless today (recourse.words takes no speaker) but inconsistent with evidence_story, which uses Speaker::of(doc).
NOTE-3, unsure, design (not re-litigating): DESIGN [assertions.quiet] says a contact is quiet when a Gap is taken between "the finding's two cells … wherever they lie". The host's Gap over picks of the instance's two pieces names those same cells, and the ruling (work item's Re-cut) says the host never quiets them. The ruling may deserve a sentence in [assertions.quiet] or A5 saying a carried row is outside "wherever".
NOTE-4, sure: the quiet sentence "an assertion of the part's says so" could now name it. The doc is in hand (SaidFinding quiet arm, checks.rs).
NOTE-5, sure: the host cfg's UnprovenCoincidence severity (Off) silences carried rows too. That is reasonable, but it is a host setting changing a part's row.

## Style
Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 partially (I read carried.rs end to end; I did not read checks.rs end to end).
- Q1, sure: carried.rs. `impl CarriedRow for CarriedCoincidenceRows` is hand-written beside the `carried_row!` macro, differing only in `rows` for `finding`. Two identical PartialEq-ignoring-doc impls, with a duplicated comment, sit 30 lines apart.
- Q1, sure: two policies for "what a part carries up". In parts.rs, interference and at_rest come from `product.carried_*` (product sources), while door_rows come from `carried_coincidences(&amp;evaluation)` (every evaluated value). The filed P2 is the visible cost. The PR fixes one carrier by choosing the other policy rather than one home. Look for the same split anywhere else `carried` is read.
- Q7, likely: `Pinned&lt;DocRows&gt;` stores rows in a field named `finding` (`pinned.finding.rows`). The generic's vocabulary doesn't fit an unjudged row set.
- Q3, likely: a_carried_contact_is_quiet_by_the_parts_assertion_alone, host half. The host Gap can't quiet anything under the current structure (the host reads no carried quiet), so that half goes red only if someone adds host quieting. That is acceptable as a regression guard, but it is not evidence today.
- Q6, sure: the doc-instead-of-held deviation is an improvement. The P2 follow-up is scheduled as a work/ file.
- Q2: the grep over the diff found no reconciliation prose. The only constants-style twin is the macro copy above.

## Review B

PR #5871 FULL review at head 7c637654: APPROVE-WITH-FIXES. Counts: MAJOR 0, MINOR 2, NOTE 4.

Isolation: I did not read any other review/atrest-carried-* branch, the PR comments, or any review scratch. I read the PR body, its diff, and CI check-run status, which is green on the head.

Probes are on branch the reviewer's probe branch (commit on top of the head), appended to crates/editor-core/tests/atrest_carried_coincidence.rs:
- probe_a_leaf_under_two_mids_is_reported_once: host → mid1/mid2 → leaf, with the host plain and assembled.
- probe_instances_consumed_by_a_boolean_still_carry: a host Union of two instances.
- probe_census_rows_equal_the_parts_verdict_and_none_are_the_hosts: the stacked-copies part, loud and quieted by its own Gap=0, with the host plain and assembled.
- probe_a_carried_only_door_job_cancels: run_checks_at_rest_on, then job.run with a canceled token.
- probe_print_the_sentences: prints the part's sentence beside the host's.
All pass on the head. The PR's 4 rows, the dprime3 row, and coincidence_door, dsc_checks, intent_part_census_seam, intent_dprime3_*, atrest_* and asm_r2b_assembly (112 tests) also pass on the head.

## Claims
1. Exactly once, as the part's: held (executed, sure).
   - Leaf under two mids: the leaf's 5 findings once, routed through the first mid's instance.
   - Boolean over two instances: the 5 carried rows once, plus 26 rows the host's own Union decides. Those are expected; nothing of the part's is doubled.
   - Inside a ⊔ and outside one: identical.
   - Partly quieted: the census probe's carried findings equal the part's own run_checks findings exactly, quiet included, and no finding is the host's own. The PR's test 3 filters to carried rows, so it could not have caught a host-own row; the probe closes that gap.
   - Not executed: a pick feeding a boolean. By reading, carried_coincidences reads every evaluated value (checks.rs carried_coincidences), so a pick cannot drop the rows.
2. The part's door judges, nothing of the host's: held (sure).
   - DoorJob::run calls door_findings over part.doc, and quiet comes from door_rows over the part's evaluation (parts.rs evaluate_entered).
   - Host Gap=0: the PR's row covers it.
   - Same-named host variable: not executed. Symbols are keyed by document (the parts.rs `symbols` comment), and the replay runs over part.doc alone.
3. DoorJob soundness: held (sure).
   - Cancel: executed. The host's own empty row set never replays, so the cancel is observed in the carried replay.
   - Once per distinct (of, pin, rows): CarriedRow for CarriedCoincidenceRows.
   - Pin: the stored Arc&lt;ProfileDoc&gt; is the doc entered for the pin it is carried with (parts.rs evaluate_entered and wire_instantiate_part). I found no pin mismatch.
   - A replay that fails comes back Proof::Unproven with NotReplayed, loud (coincide.rs prove). Not executed with a failing sub-part.
   - NOTE N1 (likely): door_rows plus carried_coincidences now run inside every nested part evaluation (parts.rs evaluate_entered), including the Sym replays the door itself makes. They are cheap reads, not a replay, but they now sit on the evaluation path and no one has measured them.
4. Host path unchanged: held (executed, sure). I blessed the perf12 census goldens with CAD_TOLERANCE_EPS = 1e-6, 1e-9 and 1e-12, on the head and on the merge-base 745edf4a. All three files are byte-identical between the two. Both read kitchen_sink 28 against the committed 29, so that red is inherited. For a document with no instances, `carried` is empty by construction.
5. Surface:
   - PartialEq ignoring doc is sound: route.of plus the content pin fixes the doc, labels included.
   - No `held` rendering is lost: Speaker::of(doc at the pin) speaks the labels.
   - Viewer coincidence_cells returns empty for this evidence (frame.rs coincidence_cells). Python's inner `finding` is spoken over the part's document.
   - MINOR M1 below.
6. dprime3 restatement asserts the right thing: count equals the part's, every finding is CarriedCoincidence through the instance, and the host decides nothing. NOTE N2 (sure): it compares counts only, not the part's findings verbatim. The new file's row 1 covers that.

## Findings
- MINOR M1, checks.rs SaidFinding::recourse_words, CarriedCoincidence arm (claims 2 and 5). Executed via probe_print_the_sentences; sure.
  - The host's sentence ends "Recourse: assert the coincidence at its site". In the host that recourse cannot work, because host assertions are never read; the PR's own test 3 proves exactly that. The author has to edit the part, and the sentence does not say so.
  - CarriedInterference has the same class: "assert a negative Gap between the two faces". That one predates this PR.
- MINOR M2, checks.rs SaidFinding::evidence_story, CarriedCoincidence arm (claim 5). Executed; sure.
  - The arm says only the inner evidence_story, so the part finding's subject is dropped. Part: "Union b9880a…: the end cap…". Host: "…leaves a coincidence of its own unproven: the end cap…", with no Union. For a census row, the part's "copies Place f30d… and Place 0dc1…" is lost too.
  - The quiet suffix says "an assertion of the part's says so", although the part doc is in hand to name "Assertion 4bc0…". The doc was taken in place of `held` precisely so it could speak, and then it is not used to.
- NOTE N3 (sure, inherited class): the host-level CheckFinding.quiet, and Python's CheckFinding.quiet, is a NodeId in the part's id space, sitting on a finding whose doc is the host's. It is documented, and CarriedInterference does the same.
- NOTE N4 (unsure): a loud carried coincidence refuses at Error in the host (never_refuses has no arm for it), even though no host edit can quiet it. That is consistent with CarriedInterference's rule (refuses iff quietable in the part). A5's "never refusing" reads loosely against it; worth one sentence.

## Style
Questions exercised: Q1, Q2, Q4, Q5, Q7, Q8 (carried.rs end to end, and checks.rs around the door), Q3 on the tests.
- Q1, sure: three carriers speak a part's nodes three ways. CarriedInterference uses Speaker::TAG, CarriedAtRestFinding uses held, CarriedCoincidence uses doc. To look further, check Pinned and PartRow callers in parts.rs.
- Q1, sure: carried.rs has two hand-written PartialEq impls with the same comment ("The document is fixed by `route.of` and the pin"). CarriedCoincidenceRows also gets a hand-written CarriedRow beside the carried_row! macro, only because its field is named `rows`.
- Q1, likely: DocRows and CarriedCoincidenceRows are one shape minus route and pin, converted field by field in two places (wire.rs wire_instantiate_part and the parts.rs `below` map).
- Q4/Q5, sure: carried.rs Route's doc still says "One type for every carrier of that fact — CarriedInterference and CarriedAtRestFinding", missing the two new carriers.
- Q4/Q5, sure: PartRow::unspoken's doc says "body names no node… an interference finding", but it now carries DocRows, whose rows name part nodes.
- Q5, likely: the DoorJob doc ("rows… named in the document the report was taken of") no longer holds for `carried`.
- Q4, likely: wire.rs wire_instantiate_part's header comment says "the bookkeeping rows ride unchanged for the same reason" (key-stability). Door rows ride for a different reason: they are in the part's ids and judged over its doc.
- Q3, sure: in atrest_carried_coincidence.rs a_carried_contact_is_quiet_by_the_parts_assertion_alone, both halves filter to carried rows and would pass if the host also re-reported the contact as its own. The probe covers that.
- Q7, unsure: checks.rs carried_coincidences names its accumulator `held`, which collides with the HeldNodes vocabulary in the same module.
- Q7, unsure: an unmeasured eval-path cost (N1) with no guard or schedule (Q6).
- Trivial: the new A5 sentence in ASSEMBLY.md is left as one unwrapped long line.
