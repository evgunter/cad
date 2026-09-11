---
id: product-gather-refuses-a-split-root-whose-tie-spans-both-halves
kind: issue
title: The product gather refuses DuplicateName for a split root whose tied name has one candidate in each half
status: open
opened: 2026-09-06
---



Found by MSOLVE-5's correctness arm (PR 2090, probe P10), outside the
unit's fence; reproduces on main (`product.rs` untouched by the unit).

Document: a 4×4×4 block minus a U-shaped cutter, leaving two cap
fragments under ONE name (a genuine N2 tie, the shape
`m4_pr3_names_bool::symmetric_u_cutter_fragments_tie_and_naming_stays_total`
pins), then split by the plane `y = 2`, which separates the two
fragments without cutting either. The split's own table stays `Tied`
with one candidate per half, as `names/table.rs:186-189` documents.
The split is the only root, so the gather carries each half as its
own source — and `carry_names` (`crates/editor-core/src/product.rs:786-816`)
inserts the name once per source body: the second half hits
`DuplicateName` and the gather refuses `ProductError::Naming`
("root 8's face name (minted by node 6) collides in the product's
name table"). A document that evaluates and names totally does not
gather.

What is owed: a tie spanning the output bodies of one root needs
`insert_tied` (or a merge) on the aggregate table, or the row dropped
under a stated rule — either way a decision about the product's
table, not the split's. The probe is saved with the correctness arm's
file (`/home/user/msolve-1-scratch/correct5/msolve5_probes.rs`,
`probe10_split_root_separating_a_tie_gathers`) and is easy to rebuild
from the description above.
