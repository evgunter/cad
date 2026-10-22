# ATREST — plan

## Slate and order

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
   - `an-unmated-row-on-a-part-resolves-parts-at-sym-for-seconds`: the
     cost of replaying a node row on a part. It shares ground with
     COINCIDE.

## The seam with INTENT

INTENT's D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`) puts
the census at each `⊔`'s door. Stage 4 I and 5 C change what that census
records and how its findings are reported. Either may land first:
- **D′3 first:** I and C are written against `⊔`.
- **I or C first:** D′3 moves what they built.

Both sides announce their changes to `assembly.rs` and
`checks/at_rest.rs` on their PRs.

## Review posture

INTENT's, unchanged: the repo-wide tiers (`memories/orchestration-model.md`,
`docs/DUAL-REVIEW-PROTOCOL.md`). Stage 4 I and 5 C are H units, so each
gets a concurrent pair.

## Exit criteria

`intent-stage4-is-built` and `the-at-rest-census-is-a-check` closed, and
the slate empty or re-homed.
