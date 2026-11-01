---
id: a-seeded-pair-contact-fuzz-run-decides-a-rescued-joint-transversal
kind: issue
title: a seeded run of every_pair_the_shared_end_touch_rescues_escalates_at_its_joint decides a rescued joint Transversal
status: open
opened: 2026-10-23
priority: P2
cost: M
---


Found by the PR gate of PR 5628 (QUAD, measure door unit B), which does
not touch `crates/profile`. The randomly seeded row
`crates/profile/src/seg.rs`' `pair_contact_tests::every_pair_the_shared_end_touch_rescues_escalates_at_its_joint`
failed at the default ε on run 40030217793 (`seg.rs:2962`):

> rescued, and the joint decided Transversal on path_junction_turn:
> prev (216.7155322365412, 234.83046243750164) → (−2.7307022884104457, 91.70366508523337),
> next (−2.7307022884104457, 91.70366508523337) → (−568.4922485129043, −277.29623303663004)
> — reproduce with `CAD_FUZZ_SEED=0x3877462f1a64e462 CAD_FUZZ_EFFORT=1`

Reproduced locally with that seed (it fails the same way). The row's claim is that every pair the shared-end touch rescues
escalates at its joint, and this pair's joint decided `Transversal`
instead. Either the claim is false of this
pair, or the generator drew a pair outside the population the claim is
about. Pin the seed as a deterministic row beside the fix.
