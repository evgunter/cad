REVIEW — PR 7124 `embed/records` @ 18c956f37a (merge base ad1a6ee129). 
Probe branch: `embed/records-review-XXXXXX`. I saw no other reviewer's work, branch or PR comment.

## Verdict: NOT-MERGEABLE-AS-IS

The type and the door wiring are mostly right, and both of the author's mutants still bite on this head. But the one property the unit exists for fails, and I demonstrated it two ways: a finished body can hold a record its gate did not confirm. (1) The deciding gate throws away the census's own refusals of a record when they are not `Stale`/`Contradicted`. (2) The public `⊔` gate keeps every carried record whatever the census says about it. Both are small fixes, but they change what `Bar::RecordsConfirmed` means, so they should land before this type becomes the floor `embed/gate` builds on.

Gate: run 44953314976 is on head 18c956f37a, `gate ok` success. Step level checked: kernel 12,691/12,691 and app 4,893/4,893, the default and both ε steps all passed. The PR's "17,584" equals 12,691 + 4,893. The "slow-set tests this diff adds or edits" step ran **0 tests** on both shards, so `six_crossing_corners_build_every_op`, `corpus_finish_count` and `product_finishes_once` are not covered by the gate (run by name below).

## MAJOR

**M1. The deciding gate certifies only the records the census refuses as Stale or Contradicted. A record the census says it cannot confirm is held as confirmed.** DEMONSTRATED. Class: code.
`validate.rs:11189` `records_certified` keeps only `StaleContactDeclaration | ContactContradicted`. But the census's record confirm refuses a record outside its lane as `CensusUnsupported` / `CensusEscalated` (`census.rs:11234-11247`, `11418`, `chart_region_finding` `census.rs:13068`), and the filter drops those too, so the record passes.
Probe `review_records_probes::probe_records_handed_to_a_bare_finish`: two cubes kissing at (1,1,1), handed `finish(Built::of(kiss).recording(held + [a's x=1 face, b's x=1 face]))`. Those faces are on one plane, opposed, and meet only at the corner, so their interiors never meet.
- Raw tier 3′ on the same body and records: `Err([CensusUnsupported { subject: FacePair(4v1,12v1), cause: ChartRegion(TouchingBoundary) }])`.
- `finish`: **FINISHED holding 2 records, `Bar::RecordsConfirmed`**.
- The same happens for a's z=1 against b's z=1.
That breaks claim 2 ("certifies every record it is handed") and claim 3 (`Bar::RecordsConfirmed`'s doc: "every record the body holds is confirmed where it lies"). It is also the `topology.tiers` paragraph's "confirms each record it reads" and the PR body's "a finished body holding a record no gate read".
The same filter decides what a door may carry: any door whose carry lands a record outside the census's lanes (curved pairs, records that touch at a boundary) ships it as confirmed. Neither of the author's mutants can see this: mutant A turns certification off, so it cannot tell a narrow filter from a full one.

**M2. `finish_assembled` is public and keeps every carried record, whatever the census reads.** DEMONSTRATED. Class: contract/API, plus test gap.
`Assembled::new` (`finish.rs:95`) and `topo::finish_assembled` (`finish.rs:819`) are `pub`. `validate_assembled` (`validate.rs:10898`) puts `carried` on the body unconditionally: `let mut records = carried;`. The census findings about those records go back in `Censused.findings`, and "nothing it finds refuses".
Probe `probe_a_false_record_through_finish_assembled`: the kiss with a record of a's x=0 face against b's x=2 face (2 apart). The arrivals were all-`None`, all-one, and split by solid:
- every case **FINISHED holding 2 records, `Bar::ArrivalsDecided`**, finding `ContactContradicted{…Rest…margin 2.0}`;
- the image under a translation still holds 2 records;
- only a later door (`union` with a far brick) refuses it.
Consequences:
(a) `Bar::ArrivalsDecided`'s doc "confirmed the records it holds" (`validate.rs:10760`) is false.
(b) The `cfg(test)` fixture `validate::unconfirmed` exists on the premise "No public door can stage one" (`validate.rs:19476`, and the PR body's "no public door can hand a finished operand a false record"). Any crate can stage one through this door. So the blend's own gate can be pinned from `sweep` after all, contrary to the PR body's "No pin" note. The same premise is why `a_carried_record_no_reading_covers` dropped its two split rows.
(c) In production every `⊔` (`wire_assemble`, `distribute::assembled`, product gather, import) hands `carried` re-keyed through a graft. A graft mis-key, which is exactly claim 7's open premise, becomes a reported finding on a body that ships, not a refusal at that door.

## MINOR

**m1. The public `FinishedBody::try_recite` strips a finished body's records when its closure refuses.** DEMONSTRATED (`probe_a_refused_recite_strips_a_finished_body`: records 1 → 0, bar still `RecordsConfirmed`; a re-cite to `Decided(4000000)` is accepted). Class: API.
The trait method is public (`finish.rs:191`). `AtRestBody::try_recite` does `mem::take(&mut self.records).try_recited(f, uncited)?` (`validate.rs:10968`), so an `Err` leaves the body touching with no records and `Bar::RecordsConfirmed`. That is documented ("A refusal leaves it with no record"), but it is a public way to change a finished body's records outside the gate. Any caller can also re-point every backing to a row nothing decided. Claim 1 ("no code outside … can change the records of one") holds for cells only.

**m2. A move does not keep `Bar` on the leaf path.** DEMONSTRATED (`probe_a_move_of_an_assembled_leaf_keeps_its_bar`: a `finish_assembled` body is `ArrivalsDecided`; moved as a `FinishedBody` it stays `ArrivalsDecided`; moved as a `LeafBody` it becomes `RecordsConfirmed`). Class: code/doc.
`LeafBody for AtRestBody::moved_by` (`finish.rs:732`) re-finishes the image through `T::leaf(Built…)`, i.e. `validate` and not `validate_assembled`. A `⊔`'s leaf (`wire_assemble` holds `Held::Leaf`) moved by a placement at f64 comes back as `RecordsConfirmed`. Meanwhile `AtRestBody::bar`'s doc says "an isometry keeps it", and the box scalars' `Replay::moved_by` keeps `assembled`, so the taker calls `finish_assembled` again (`ArrivalsDecided`): the same document reads two bars at two scalars. Nothing anywhere reads `bar()`: no consumer in any crate, no test outside my probes. So nothing reads a `RecordsConfirmed` body as tier 3′ in full (claim 3's second half holds vacuously), and no row would see `Bar` go wrong.

**m3. Claim 10: the gate pays for the whole census to confirm records, then discards half its output.** BY-INSPECTION. Class: code/contract.
`validate` calls `census_and_certify` (the realized global census, unbacked-candidate discovery included, `census.rs:835`) whenever a record exists. It then discards every finding not about a record. That is the six-crossing cost: each Boolean with one carried record pays the full census over every cell pair. It also re-confirms the operand records on cells the op kept bit-identical. `[one-way.places]`' at-rest row says such a certificate "is read wherever that support is bit-identical". So moving the test to the slow set treats the symptom: the cost is in the shape of the call and is not inherent to "confirming records". I did not measure the split between the two halves.

**m4. Claim 5: `_with`-class callers that hand fewer records than the operands hold.** Class: doc/claim, test gap.
- `editor-core/src/checks/at_rest.rs:1351` — `boolean_placed(Intersect, …, &BooleanInputs::default())` intersects two product copies with no records. This is production, the interference check.
- `nary.rs:698-702` — the pairwise judgement, `BooleanInputs::default()` (judgement only).
- The public `nary_boolean`/`nary_placed` take caller `MemberInputs`.
- Every evaluator caller hands full records (`wire.rs:3607`, `4182`), and so does `verbs::run`.
The PR body's sweep names only `boolean_op_with`. Both of its statements are true, of different doors. Mutant C (below) shows how thin the pins are.

**m5. Claim 6/7: the distribute door's records no longer cite what they say.** BY-INSPECTION, likely. Class: code/doc.
`distribute.rs:194`'s doc still says "the records' citations renumbered onto the assembled rows and inputs". The renumbering is gone: it was the PR side's change, and the merge `063f49a3b` kept it.
- Each piece's records now cite `Carried{arrival,k}` with `cited_inputs[arrival] = Read(unit.read)`. That is record k of the read's value, but the body is the piece's Boolean result, whose record k is a different record.
- The census-minted records cite `Decided(0..)` (no `numbering_from`), while the value's `coincidences` holds the pieces' own rows, so `Decided(k)` names a piece's row.
`wire_assemble` avoids this only because its value holds no coincidences. I did not build a document to show a wrong citation read.

**m6. Claim 7: nothing checks that the product's graft and the `⊔`'s graft agree.** BY-INSPECTION. Class: test gap.
`product.rs:1435` `Some(decided) => decided` throws the gather's own aggregate away and keys names, copies and `solid_copies` through the gather's `keys` over the door's body. The two grafts iterate the same bodies in the same order through `Framed::graft_into` into a fresh `Body`, so I found no divergence. Frames differ only if a body is solidless, and the `⊔` refuses that. There is no assert and no row. If they diverged: names on the wrong cells, records attributed to the wrong copy, census rows on the wrong pieces, all silently. A `debug_assert` that the gather's aggregate equals the door's body would cost one comparison.

**m7. Claim 13: `StaleContactDeclaration` now reaches the user from doors, with a recourse they cannot follow.**
"remove that declaration, or move the geometry" (`validate.rs:5243`). In a document no record is a declaration: a stale record at a Boolean, split, shell or blend gate is the kernel's carry writing it. `ContactContradicted` ends with the kernel/file-defect ending; Stale should too. `BlendError::Carry` maps to `Recourse::None` in `recourse_tests` (`blend/mod.rs:2631`), but its text carries a recourse.

**m8. Claim 8: `plate ∖ rod tangent from inside` now refuses `ContactContradicted`.**
That is better than shipping a body without its cusp edges and with a false record, and the row is filed with the owed work. The deletion of `a_solid_missing_its_cusp_edges_leaves_a_solid_in_its_bore_cleared` is disclosed and owed on the row, not done to get green. But its subject (a missing-cusp finding does not stop the census reading the other solid pairs) is now pinned nowhere.
The two dropped `a_carried_record_no_reading_covers` split rows pinned (i) a kept record carried by the split onto the correct side (below, not above), and (ii) a split dividing the curved pair refusing `Stale{record:0}` through the curve × face reading. (i) is covered for planar records by `one_record_carry` and `divided_cell_carry`. (ii), the curved reading at the split, is pinned nowhere now. The stated reason, that no finished operand can hold the record, is false by M2.
`product_finishes_once` 1→0 and `corpus_finish_count` 390→383: the explanation (the gather reads the `⊔` door's finished body) matches the code (`product.rs:1435`). Both pass on the head (run by name).

## NOTE

- n1. Claim 4 holds. Mutant B ("the doors hand `finish` nothing", the author's four sites) gives **24 red** on this head (author: 23, on 18 Nov): editor-core `split_and_shell_carry_contacts`×6 and `blend_carries_contacts`×2 (of the author's ×4), topo `divided_cell_carry`×5, `one_record_carry`×3, `surgery_carries_records`×2, `review_4694`, `review_4730`, `a_vertex_read_by_two_sector_passes`, the two in-module gate pins, sweep `blend_carries_records`. Split's parted-record policy: `carried_across` retires a record whose cells both have an image on the cut and refuses `Parted` otherwise. That is WIRE's interim document policy adopted, as the row's note says; it is a policy the row explicitly left open.
- n2. **Mutant C (mine)**: the plain doors carry A's records and drop B's (`BooleanInputs::of`). Only **2 red**: `one_record_carry::a_boolean_result_reused_as_an_operand_keeps_its_records` and `a_vertex_read_by_two_sector_passes::…shared_ray_builds_at_rest`. No sweep or editor-core row sees a half-carry.
- n3. Mutant A ("the gate takes the record on its word") gives **11 red**: the author's 10 plus the rewritten `a_carried_record_no_reading_covers`. **None is in editor-core**: no document row pins the gate's certification.
- n4. Claim 9 holds. `AtRestBody::moved_by` keeps cells and records unread, under a mirror too; the record kinds are unordered cell pairs and invariant under both. That is the ratified "asks only the session box again". The leaf path re-reads (m2).
- n5. Claim 11: the author's history claim is wrong, but the re-wording is still a description moved by the change. The checkout is shallow; I used the GitHub REST commits API.
  - The sentence went through an `[ev]` PR: 4951 ("[ev] a record is read where it is minted and carried by lineage", merged 2026-10-13, commit `156cf5c0cf`). Ev's version there was "meets tier 3 without the census's search for touches nothing records … the search over them is parked". It moved verbatim to the README in `56851aa0cf`. So "`git log -S` finds no ratification beyond PR 5967" is false.
  - The new text still parks the same unbacked-touch half that 4951 ratified as parked, and moves only what the code moved (records off the body's side, onto the gate). I read it as a description, not a decision changed. Ev may want to see it, because its new clause names a ruling ("not raised by a deciding door's gate").
  - Separately, the re-worded paragraph now contradicts the ratified sentence eight lines above it: "none, for a body that touches nothing, where the census must find no coincidence at all". No gate asks that now.
- n6. Claim 12: `impl Into<Built>` admits `Built` and `Body` (`From<Body>`). `Body`'s sentence, "touches nothing", is the caller's assertion, and the gate never tests it (it raises no unbacked meeting). So `finish(Body::clone(&finished_with_records))` is the silent record drop the PR calls impossible "because a caller forgot the carry". That is exactly what a kernel caller of `merge_coplanar_faces` / `movefac` / `offset_axial` does. No f64↔T conversion enters with the diff.
- n7. Claim 14: the PR body lists the 219 paths' owners, but no owner's row or log is touched. Only HELDRECORDS, ATREST, DOCTAIL and ONECOPY rows are edited.
- n8. Merges: `8c8f92652` resolves only the Spindle `frame` rename in two validate.rs test rows, correctly. In `063f49a3b` I read validate.rs, finish.rs and distribute.rs. The constructors' `where` bounds were re-added for main's `CertifiedPolicy`; `checking()` wraps both `validate_closed` and the certificate (so the `Replay::of_assembled` side carries main's change too); distribute is m5. I did not read the other 21 conflicted files' resolutions line by line.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, and Q8 in part (below).
- `validate.rs:11189` — `records_certified` filters a finished list by variant to decide what a record's confirmation is. Its doc justifies the filter by the cause of a different filter (the unbacked half). The census already knows which findings are about a record; a reader of the filter has to re-derive that. sure
- `finish.rs:62` `Built::recording` replaces, not adds: `Built::of(b).recording(x).recording(y)` silently drops x, in a type whose job is to stop records dropping. likely
- `finish.rs:67`, `From<Body> for Built` — the conversion that is the quiet path round "a door hands finish its records". `[unrepresentable]`: its sentence is not true of every value it admits. likely
- `validate.rs:10752` `Bar` — an enum nobody reads, at a hard-to-change-later type. It names which half was paid, but no consumer branches on it. likely
- `validate.rs:10755`/`10760` — the `Bar` variant docs promise confirmation the code does not give (M1, M2). sure
- `validate.rs:19476` — the fixture's premise ("only this module can reach") is false by M2; the fixture may be unneeded. sure
- `blend/mod.rs:2355` — `BlendError::Carry`'s Display re-spells `CarryRefusal::write_at_a_door` by hand, while shell and split call it (`shell.rs:840`, `splitting/finish.rs:1335`). Editor-core spells the same sentence a third way, with `sentence::Recourse` (`eval/mod.rs`). Three spellings of one refusal text. sure
- `eval/carry.rs:193` — `Carry::contacts` `unreachable!` on a length mismatch: `Citing::Each` is a positional list held in step with the body's records by convention, the duplication this unit set out to remove, as an index. likely
- `eval/carry.rs:278` — `Carry::member` widened from `pub(crate)` to `pub` with no new outside caller that I found. unsure
- `splitting/finish.rs:207` — `SplitContacts` stays a struct with `pub` fields under `pub(in splitting)`; it is now only the return of one private fn. unsure
- `distribute.rs:194` — doc comment stale (m5). sure
- `product.rs:1435` — `decided` arrives as an `Option` that silently discards a whole computed graft. The invariant "a graft is a function of its sources in their order" lives only in a comment (Q2). likely
- `validate.rs:10848`/`10938` — `AtRestBody` impls split across two blocks with different bounds; `try_recite` sits in the looser block, the constructors in the stricter. Probably needed, but it reads as accretion. unsure
- `pncad-py/src/py/value.rs` `HostBody::records` — `Plain` returns no records, so `validate_pseudomanifold` on a host body built from a non-`Held` source runs tier 3′ with none. Per the PR every live caller passes `Held`; nothing makes `Plain` + touching unrepresentable. unsure
- README `crates/topo/README.md:768-791` — one paragraph now states the ratified shape ("passes tier 3′ against them") and, three sentences later, that the gate does not raise what that shape requires (Q4). The doc rotted around a code state meant to be temporary; the flip row schedules the fix. likely
- Class note for the fix pass (sweep obligation): M1's filter shape (a refusal list narrowed by variant to mean "confirmed") may recur. Look at `record_at_rest`'s `recordable` filter and `gate3_findings`.
Q8: I read `finish.rs` end to end except the `take`/`FindingFamily` text bodies, plus the diffs of `validate.rs` (22k lines; I read the gate, both constructors, `census_verdict`, `records_certified`, the fixture and Display, not the whole file), the Boolean `gate`, split `finish.rs`/`mod.rs`, shell's finish, the blend surgery tail, `carry.rs` in full, `product.rs`' changed functions, `wire.rs`' changed nodes, `distribute.rs` `assembled`, and step-import's `gate3`. Not read: props.rs outside the policy impls, census.rs beyond `record_at_rest` and the record-confirm arms, the 190 test files.

## Spec deviations
Reported by the author: `BooleanInputs`/`MemberInputs` not folded; merge-faces/face-offset carries stay caller-side; the gate not flipped (tier 3′ not paid at any door, against the row's "gates each finished body it returns at tier 3′"; orchestrator-ruled, filed with its measurement); mutants and slow set not re-run after the merges. Not reported:
- the gate confirms only Stale/Contradicted (M1);
- `finish_assembled` refuses no record (M2) — the row's "minted only by a gate that ran tier 3′ against those records";
- `Bar` is not preserved on the leaf move (m2);
- the distribute citations (m5);
- the `checks/at_rest` intersection drop (m4);
- the PR body's "no ratification beyond PR 5967" for the README sentence (n5: PR 4951 is an `[ev]` PR that wrote it).

## Rubric
- Idiom and structure: 3. Private fields, two constructors and `Built`/`Assembled` are a clean shape, but `From<Body>`, a public `try_recite` and a public `finish_assembled` reopen what the private fields close.
- Test quality: 2. Both author mutants bite; mutant C bites 2 rows; no editor-core row pins certification; no row pins `Bar`, the graft identity, or a record outside the census's lanes; two pins rest on a premise M2 falsifies.
- Doc and comment honesty: 3. The PR body is unusually candid about what was not re-run. But `Bar`'s docs, the fixture's premise and distribute's doc state what the code does not do.

## What I executed vs read
Executed, all on this head:
- the step-level gate check;
- probes (4 rows in `crates/topo/tests/review_records_probes.rs`);
- mutant A (11 red), mutant B (24 red) and mutant C (2 red), each over the `ci` profile, whole workspace, `--lib --tests`, 17,588 rows;
- the slow-set rows the gate skipped, by name on the clean head: `six_crossing_corners_build_every_op` passes (0.69 s here, serial), `corpus_finish_count` passes at 383, and all 16 `product_finishes_once` rows pass.
Read only: everything else above. History via the GitHub REST commits API (the checkout is shallow). Probe branch `embed/records-review-XXXXXX` holds the probe file, which prints and does not assert; it is evidence, not a row to keep as is.