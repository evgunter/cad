# Reviewer probes for PR 7458 (split-reads), frozen head cc23f94cf

- `crates/editor-core/tests/review_split_reads_probes.rs` (compiles on the merge base
  e1830a1cc too): `base_plate_overlay` (the two-parameter plate at 16 and 64 leaves,
  every leaf's box and class, and for each leaf how many of a 9x9 grid of f64 points
  give a certifying vector other than the witness's), `base_corpus_drives` (every
  corpus document, every continuous variable widened to uniform +-2%, 4 leaves).
- `crates/editor-core/tests/review_split_reads_head_probes.rs`: the read-set
  differential widened (three move sizes, no prior, failures compared by variant,
  panics caught and named) and a two-undecided-node depth probe.
- `overlay.py <dir> 16 64`: base certified regions meeting a non-certified head leaf.
- `base/`, `head/`: the outputs.

Mutants were runtime switches (`REVIEW_MUT=<name>`) in `drive.rs` / `doc.rs`, reverted:
`every_axis` (bisect's read filter true), `own_reads_alone` (free_reads returns before
its fixed point), `reversed` (classify_replay walks the order backwards),
`unread_budget` (`Unsplit::Unread` priced `Budget(Resolution)`), `stop_at_definition`
(free_reads does not follow a non-free definition), `no_upstream_of` (free_reads not
seeded with `Doc::upstream_of`).
