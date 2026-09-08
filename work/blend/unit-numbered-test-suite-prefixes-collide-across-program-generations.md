---
id: unit-numbered-test-suite-prefixes-collide-across-program-generations
kind: issue
title: sweep: unit-numbered test-suite prefixes collide across program generations (blend6_ names two units)
status: open
opened: 2026-09-08
---


## Finding (BLEND-6's dual review, R2, 2026-09-08; class, not instance)

`crates/sweep/tests/` names suites by program unit number, and the
number is not unique across program generations: S-BLEND (band
600–699, closed 2026-08-31) had units BLEND-1…7 and left
`blend6_verb_vocab.rs`, `review_blend6_r1_probes.rs`,
`review_blend6_r2_probes.rs` (its BLEND-6, PR 1328, verb neutrality);
this program's unit 6 landed `blend6_ring_clearance.rs` and its
reviewers wrote `blend6_r1_probes.rs` and
`review_blend6_ring_clearance_r2_probes.rs`. `grep blend6` returns two
units with nothing in common. The same collision is latent at every
`N ≤ 7` of this program: units 1–5's review probe files
(`review_blend1_r1_probes.rs`, `review_blend_e2_r1_probes.rs`,
`review_blend5_r5_probes.rs`, …) sit beside S-BLEND's
`review_blend{1..7}_r{1,2}_probes.rs`, and units 7 and later will
collide the same way as they land.

The unit-5 resolver row
(`review_blend5_r5_probes::every_test_citation_in_the_sweep_docs_resolves_to_a_test_row`)
makes a rename mechanical: every `<module>::<row>` citation in
`crates/sweep/src/**` goes red until it follows.

## Decided for unit 6 (the instance)

Unit 6's fix pass renames its three files by SUBJECT
(`ring_clearance_forms.rs`, `review_ring_clearance_r{1,2}_probes.rs`),
with the mounts and citations following.

## Open (the class)

A naming rule for the program's remaining units (7–13) and whether the
E units' probe files are renamed the same way, or left with a header
line naming the generation. Either way the rule belongs in
`work/blend/program.md` and in `docs/prompts/implementer-discipline.md`'s
suite-naming sentence if it has one. Not a unit; settled by the
orchestrator when unit 7's suite is named.
