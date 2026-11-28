# Review: PR 7723 (boxfinish/gather-carries) at frozen head 3a2910cb29

**Verdict: APPROVE-WITH-FIXES.** One MAJOR has to be fixed first: the new public trait method can hand back a finished body for anything (it forges one). The fix is small. The rest of the change does what it claims. Hosted CI on this head is red, but only on two inherited rows, both already filed (see "Gate").

**The base (a correction to the brief).** Running `git merge-base origin/main HEAD` gives 38832098, from 2026-11-24 09:30. That is not this unit's base. The branch merged f21ca495ca at c34d31557b, and that is also the PR's own `base.sha`. f21ca495ca is no longer an ancestor of origin/main, and neither are the merges #7722, #7713, #7651, #7711 and #7720. Main's history split off after 38832098, so those merges look like they have dropped off main. I used f21ca495ca as the base: the diff f21ca495ca..HEAD is exactly the 23 files the brief lists.

## Gate (CI run 45483760346, on this head, checked step by step)
- **Kernel tests: red, on two steps.**
  - The ε 1e-6/1e-12 step fails on `sweep a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta` at 1e-12 (`VolumeUncomputable`, `props_chart_junction_closed`).
    - I re-ran it on the base tree f21ca495ca at `CAD_TOLERANCE_EPS=1e-12` and it is red there too, so it is inherited.
    - It is filed as `work/tesslie/main-red-a-cylinder-wall-through-a-prism-corner-union-is-volume-uncomputable-at-1e-12.md`.
    - The PR body's inherited-red list does not mention it.
  - The slow-set step fails on `topo square_search::the_square_wall_arm_agrees_with_the_traced_section`. That one is inherited and the PR body names it.
- **App tests: green.** That covers every step: the default-ε run, the ε run, the slow-set tests the diff adds (including the new 0.44 s row), and the slow-set name check.
- **Everything else is green:** lint, demos, Python, viewer, sym instrument, corrupt input and mesh.
- **"Gate ok" is red,** because of the kernel job above.

## MAJOR
**M1. `FinishedBody::gathered` (`crates/topo/src/finish.rs`) is a public trait method that returns a finished body for any body it is given, with nothing checked.** DEMONSTRATED, sure.

The trait is public and nothing stops an outside caller from using the method. Three probes on my branch, written as an external crate:
- `review_gather_carries_probes::probe_the_trait_method_finishes_a_body_it_was_not_the_graft_of`:
  - I passed in a 2 m brick together with a 1 cm brick's certificate and graft keys.
  - It came back `Ok` as an `AtRestBody`, and it measures 1e-6 m³.
- `probe_the_trait_method_panics_on_no_part`:
  - Calling it with no parts panics, on the `unreachable!` in `KeptSign::gathered`.
- `probe_the_trait_method_panics_on_keys_of_another_graft`:
  - Calling it with another graft's keys panics, on the `unreachable!` in `KeptSign::rekeyed`.
  - Those are panics on input a caller can write, which `[failure.taxonomy]` rules out.

This breaks `[unrepresentable]` ("a signature is a claim"). It also makes `AtRestBody`'s own doc false (`validate.rs`, the struct doc): "Its one constructor is the certifying scalars' finish … a kept verdict cannot be carried to any body other than the one it was derived on."

The PR body says "an unfinished member cannot be passed as one". That holds for `gather_finished`, but not for the trait method underneath it. The only thing that rules out a non-graft here is the good manners of its one caller.

What the types do rule out, as claimed: `Placed` takes a finished `&F` plus an `Option<RigidMotion<T>>`. `RigidMotion` has private fields, and its third axis is the cross product of the other two (`with_signed_axes` keeps that), so a mirror or a non-rigid map cannot reach the gather.

## MINOR
**m1. The between-members row `carrying_the_members_verdicts_does_not_certify_them_apart` (editor-core) cannot show an overlap was found.** DEMONSTRATED, sure.
- It asserts `!matches!(census, Ok(empty))`, so any `Err` passes it.
- It runs at plain `Interval`. I measured what the census returns there: `Ok([CensusUndecidable { reason: MayOverlap } …])`, which means "undecided", not "found".
- Under my mutant (the f64 `census_at_rest` returns `[]`) the row stays green.
- My row `review_two_copies_in_one_place_are_found_overlapping_at_f64`, on my branch:
  - On head the census reports `Ok([SameSideInterference{1v1,7v1}])`, and the row is green.
  - Under that mutant it goes red.
- The escalation at `Interval` is the census's own limit, not a hole in this unit. The gap is in the row.

**m2. `KeptSign::gathered` (`props.rs`) is a near-copy of `KeptSign::assembled` in the same impl, and it drops the latter's checks.** BY-INSPECTION, sure.
- `assembled` hard-asserts that every part shares the same band, tol and lane, and that the runs partition the face arena. It then re-orders the runs into arena order.
- `gathered` has one `debug_assert` on the band. It takes tol and lane from the first part without checking them, has no partition check, and assumes that "the parts' order is the aggregate's face order".
- Under mutant M2 below (all but the first part's runs dropped), the gather still returns an `AtRestBody` that silently has no runs for some of its faces. Only the rows that measure volume notice.

**m3. Docs this change made false were not updated.** BY-INSPECTION, sure.
- `editor-core/src/product.rs` module doc, lines 37–47: says "the aggregate is finished at its end … every other fact of it left to the aggregate's finish", and describes a refusal class that no longer happens when every source is finished.
- `eval/framed.rs` module doc:
  - Lines 41–44 say every gather goes through `graft_into`.
  - Lines 69–71 say "its finish asks of the whole".
- `eval/wire.rs:5300-5302` says each body is grafted through `graft_into`.
- **DESIGN `[one-way.places]` (line 328):** "Those doors are the sweep doors … and the `⊔` graft".
  - A `⊔` of finished members now returns a finished body at the box scalars too. It is a leaf only where some member is a leaf.
  - That is a fact this change moved, so the sentence should be re-worded in this PR.
  - In my reading the re-wording does not change what the clause decides: carrying takes the `⊔` to the clause's own stated final state. Confidence: unsure. Whether that needs Ev is the orchestrator's call.

**m4. The PR body's inherited-red list is incomplete.** It is missing the sweep ε-1e-12 row the gate actually fails on (see Gate). Sure.

## NOTE
**n1. Carried versus derived (claim 2).** DEMONSTRATED.
- At f64 they agree to a few ulps, but not bit for bit (for example, carried 2.0000000000000008e-6 against derived 2e-6). I checked this on:
  - the two bricks, one turned;
  - a hollow block (with a cavity) plus a curved drum, placed by a translation and by a turn-plus-translation;
  - a two-solid member gathered three times.
- At `Interval` with a zero-width turn, the carried enclosure is strictly inside the derived one.
- The unit's row allows 1e-15 absolute on 2e-6, which is about 5e-10 relative. That is loose.
- The brief's debug re-derivation would have checked every f64 gather in every suite; the row checks one fixture. So the row is weaker than the check it replaces.
- The author's reason for dropping the check (no generic way to tell the point scalars apart) is right in fact. The only candidate, `!T::TAKE_FINISHES`, is the switch's const and goes away with it.

**n2. Acceptance: what comes after the attributed stop.** DEMONSTRATED, on a local probe tree with PR 7716 merged in.
- The drive certifies the whole box (1,1,0,0).
- `demo-tour certified` then panics at `chaintol.rs:453`, in `certified_pin_boxes`: `pncad::export::body_in_space` of a pin at `Sym<Interval>` refuses with `SliverDihedral … dihedral_sign` (lo −5.25e-5).
- The pin is a sweep leaf. Its move re-finishes the image (`LeafBody::moved_by` calls `T::leaf`).
- So the unit's "the command runs to its end" is still not reached once the P0 is fixed.
- This is outside the unit, but `delete-the-box-scalar-sweep-leaf-switch.md` line 32 only talks about the f64 re-finish. Nothing in that row covers this box-scalar wall.

**n3. `Framed::gathered` reports a refusal under the wrong name.** At a dual it maps `GatherError::Aggregate` to `(0, GraftIn::Moved)`, which reads as "member 0 is past the session box". And `gather_finished` reports an empty member list as `GatherError::Graft { member: 0, JoinDesync("a gather of no body") }`. Its only caller sends empty lists to the graft arm instead, so that path is never taken.

## Per claim
1. **Held.** DEMONSTRATED at f64: for a small brick inside a big one, a brick in a cavity, half-overlapping copies, two copies in one place, and face-touching copies, the old aggregate finish also passes, with the same volume. My reading of validate.rs checks 1–11 agrees: everything is local to an entity or a solid. The census still asks the between-members questions.
2. **Held.** At f64 to ulps; at `Interval`, carried ⊂ derived (n1). At `Sym` there is nothing to compare against, because the derived finish refuses.
3. **Broken (M1).** The types do protect `Placed` and `RigidMotion`.
4. **Held, and the attribution is confirmed.** On main's configuration the unit's lane rows hold. On the probe tree with the const true, I reproduced the stop at `chaintol.rs:401` (0 certified). With PR 7716 merged it passes (1,1,0,0), so the cause is that P0.
5. **Held.** The const is read off the switch's own types. One const for both scalars is right (`Sym<T>` has one generic impl). Second spelling: the editor-core sweeps row asks the same question through `<Interval>::TAKE_FINISHES` directly. That const answers "the taker runs tier 3", which matches "the leaf is unfinished" only by the switch's construction (likely, style).
6. **Held for the counts.** I re-ran `corpus_finish_count` with the pin at 0 on both trees: base 398 and head 375, per document exactly as the PR body says. I did not run the whole-suite slow-set differential (hours of runs), so that part rests on the author's account. I found no f64 case where the old finish refused and the gather now passes.
7. **Held by reading.** The census runs at the `⊔`, the distribute and the product as before, on the aggregate. The between-members row is weak (m1). The Interval escalation is a census limit.
8. **Partly broken.** No plain `assert!` is added and nothing is swallowed. But the two `unreachable!`s can be reached through the public method (M1). `Moved` is reachable. `Graft` and `Aggregate` are not argued reachable (`Aggregate` is tier 2 over a disjoint graft of tier-2 bodies).
9. **Sound for its one caller and in the right home.** The checks are weaker than its twin's (m2).
10. **Mutants: four claims re-run, all confirmed; the fifth (M4) I ran only at f64, where the author's row never reads.**
    - Derive instead of carry: both topo box rows go red.
    - Drop all but the first part's runs: the f64 row and the Sym row go red.
    - A finished member carried only where `!TAKE_FINISHES`: the box-scalar row and the lane row go red.
    - Const set to true: the sweeps row goes red.
    - Census empty at f64: the author's overlap row stays green (it reads `Interval`), and my f64 row goes red.
    - The slow-set entry is listed once and names the test exactly.
11. **Reproduced.**
    - On the probe tree with this head, the 1- and 2-link rows certify at `Interval` and `Sym<Interval>`.
    - With 7716 also merged, all four link counts certify.
    - The trace shows only member finishes (the 6-face bar, the 4-face pins) and no 18-face finish at any scalar.
    - The command still exits, but now at `:453` (n2).
12. **PR 7492.**
    - Text: it wraps `gather_sources` into `gather_into` under `sym::apart`, which conflicts with this PR's rewrite of the same lines.
    - Behaviour: inside that scope the gather now logs only the session-box asks of the moved images and the census, with no tier 3 of the aggregate; neither PR reads what the other writes.
13. **Accurate, spot-checked.**
    - `certify.rs:3550` `locus: None`.
    - `boolean/material.rs:67-71`: graft, then `T::finish`.
    - `step-import/src/lib.rs:1055-1083`: `into_body`, then `graft_disjoint`.
    - None of it was owed by this unit.
14. **m3.** No sentence changes a ratified decision, as far as I can tell (unsure on DESIGN line 328).

## What the end-to-end run showed
- It solves the right problem. On the tour's chain the gather is no longer the wall at plain `Interval` or at `Sym`, and per-member tier 3 runs in each member's own frame.
- What matters in practice:
  1. The forge door (M1).
  2. On the switch-off path, the next wall is the same class of problem (a moved finished body re-derived), on the leaf's move at export (n2).
- Coming at the gather from editor-core is natural: the three callers each collapse to one call.

## Style
Questions I worked through: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 was only partly done: I read `framed.rs`'s header and gather sections, not the file end to end.
- **Q1:** `KeptSign::gathered` against `KeptSign::assembled`, as in m2 (`props.rs` ~2078 and ~2360). sure.
- **Q1:** three nearly parallel refusal enums: `PlacedGraftError{Move,Graft}`, `GatherError{Moved,Graft,Aggregate}` and `GraftIn{Motion,Moved,Graft}`. The translation between them mislabels a refusal (n3). likely.
- **Q2/Q4:** `gather_finished`'s doc proves in prose that "every check of tier 3 is local to one face, edge, vertex, loop or solid". Nothing enforces that. A future tier-3 check that reads two solids would be skipped by every gather without anything going red. This is the premise the whole carry rests on, and it has no guard. likely.
- **Q3:** m1. Also, topo's `a_gather_logs_the_same_verdicts_at_every_lane` compares f64 against a zero-width `Interval` only, not against a boxed turn. likely.
- **Q3:** `at_f64_a_carried_gather_is_what_a_finish_derives` uses a loose absolute tolerance and one fixture (n1). likely.
- **Q5:** the product.rs and framed.rs headers (m3). sure.
- **Q6:** dropping the debug re-derivation is disclosed and argued, and the row is a weaker replacement. "Runs to its end" is not met; that is disclosed and attributed, but nothing schedules the follow-up for the `:453` wall. likely.
- **Q7:**
  - `Held::gathers_finished` reads a crate-global const inside a method generic over `T`. unsure.
  - `Gathered` is a `Deref` enum that every caller then matches, finishing the `Grafted` arm itself, three times over (wire, distribute, product). likely.
  - The empty gather is reported as `BooleanError::JoinDesync`. likely.

## Rubric
- **Idiom and structure: 3.** One door with clean caller collapses, but there is the public forge door and the weakened twin of `assembled`.
- **Test quality: 3.** Every row I mutated went red under its claimed mutant, except that the between-members row passes on an undecidable census (m1) and the f64 row is loose.
- **Doc and comment honesty: 2.** Three module or inline docs and one DESIGN sentence are now false. The body omits the gate's second red. The `AtRestBody` invariant doc is falsified by M1.

## Spec deviations
- **Reported by the author:**
  - option B;
  - no `debug_assert` (a row instead);
  - the command not run to its end (attributed to the P0, and now confirmed);
  - the between-members row's narrower scope.
- **Silent:**
  - the public `FinishedBody::gathered` (M1);
  - DESIGN line 328 not re-worded;
  - the gate red on a row the body does not list.

## What I ran
- **Branch `boxfinish/review-gather-carries-X`, at ba9a7cc287** (pushed; cut from 3a2910cb29):
  - `crates/topo/tests/review_gather_carries_probes.rs`, 8 probes, all green on head;
  - `review_two_copies_in_one_place_are_found_overlapping_at_f64`, appended to `crates/editor-core/tests/a_gather_carries_its_finished_members.rs`.
- **Mutants, local only:** the five listed under claim 10.
- **Base tree f21ca495ca:** `corpus_finish_count` with its per-document print, and the sweep row at ε 1e-12.
- **Probe tree, local only and never pushed:** `origin/boxfinish/probe-switch-off` merged into this head, without and then with PR 7716, plus a local example that prints the chain table's rows. On it I ran the 1-link drive and the full `demo-tour certified`.
- **Read only:** designer reports A and B; PR 7492's `product.rs` diff; the three filed rows; DESIGN `[one-way.places]` line 328.
- **Not run:** the whole-suite slow-set differential.
