# ATREST — plan

## Slate and order

Steps 1 and 2 are done (PRs #5605 and #5637). Stage 4 is built.

1. **Wait for INTENT's census unit.**
   `the-census-records-a-planar-area-contact-as-one-face-pair` is in
   flight on INTENT's branch `intent/census-planar-face-pair`. It adds
   the planar and flush face-pair arms and confines face backing.
   `mates-declare-no-contact` (stage 4 I) is parked on it.
2. **Stage 4 I, `mates-declare-no-contact`.**
   `stage4-i-deletes-the-mate-mint-and-must-keep-the-records` is its
   precondition, read against the census unit.
   `the-census-reads-each-mated-patchs-decision-twice` closes with it.
   Spec: `docs/INTENT-STAGE4-SPEC.md` §9 I. Its class list gains the
   planar face pair and the flush pair, after PR #5555.
3. **Stage 5 C, `the-at-rest-census-is-a-check`.**
   `stage5-contact-finding-and-the-unproven-row-report-one-contact-twice`
   is its precondition. Spec: `docs/INTENT-STAGE5-SPEC.md`.
   `intent-stage4-is-built` closes when I lands.
4. **The record rows. Check each against PRs #5494 and #5555 before
   dispatch;** several may already be decided:
   - `a-boolean-reading-two-bodies-of-one-arrival-re-decides-their-contact`:
     PR #5494's door rule (a meeting inside an operand is carried; cells
     of one construction read twice are proven by the first rung) looks
     like its final state.
   - `a-boolean-result-reused-without-its-records-passes-at-rest-on-its-shared-keys`:
     records ride a body's value.
   - `a-boolean-value-glue-into-one-vertex-records-no-coincidence`.
   - `a-census-contact-is-a-row-and-a-record` (a ruling).
   - `an-instance-carries-its-parts-coincidences`.
   - `carried-census-rows-left-unshared`.
   - `a-product-whose-copies-overlap-may-double-count-its-mass`.
   - `declared-only-meetings-clear-at-the-census-gate-unread` (P2; it carries the closed `CONTACT-13`, both moved in from CONTACTHOLD on 2026-10-27). It is a re-read: can an op mint records that hide an overlap at the census's `records_on_their_word` reading? Its top section has the steps.
   - `an-unmated-row-on-a-part-resolves-parts-at-sym-for-seconds`: the
     cost of replaying a node row on a part. It shares ground with
     COINCIDE.

## The seam with INTENT (re-cut 2026-10-26)

No INTENT stage 3 row waits on ATREST. Only the D10 umbrella
`d10-one-way-to-say-intent-is-unbuilt` waits on stage 5 C, at the very
end. So ATREST runs under its own orchestrator, in parallel. What
remains is a file seam, not a dependency:
- **Stage 5 C dispatches after D′3 (#5636) merges.** D′3 moves the census onto each `⊔`'s door (`assembly.rs`, `checks/at_rest.rs`). 5 C makes that census a check resident and retires `assemble`'s gate, so building it before D′3 would rewrite a path D′3 is moving. Write 5 C against the `⊔` door.
- **Everything else on the slate is dispatchable now.** It touches the census's records, findings and topo, not where the census runs:
  - `two-copies-filling-one-volume-certify-at-rest` (P1);
  - `a-census-contact-is-a-row-and-a-record` (P1, a ruling);
  - `stage5-contact-finding-and-the-unproven-row-report-one-contact-twice` (5 C's precondition);
  - the record rows in step 4;
  - `separation-quiets-only-first-rung-rows`;
  - `the-contact-recourse-names-a-class-nobody-can-declare` (needs a ruling: it rewrites a SELECT-DESIGN §3d sentence);
  - `a-declared-contacts-class-admits-continuation`.
- **`an-instance-carries-its-parts-coincidences`** stays parked on `coincidences-are-recorded-at-one-door`.
- Both sides announce changes to `assembly.rs` and `checks/at_rest.rs` on their PRs. D′3's lane is `session_017HwxSpesEsgKsHjvrj6TyM`.

## Review posture

INTENT's, unchanged: the repo-wide tiers (`memories/orchestration-model.md`,
`docs/DUAL-REVIEW-PROTOCOL.md`). Stage 4 I and 5 C are H units, so each
gets a concurrent pair.

## Exit criteria

`intent-stage4-is-built` and `the-at-rest-census-is-a-check` closed, and
the slate empty or re-homed.
