---
id: unresolved-gap-and-read-fault-witness-each-mean-two-things
kind: issue
title: Unresolved::Gap and ReadFault::Witness each name two causes no consumer can tell apart
status: open
opened: 2026-11-03
priority: P3
cost: E
---

Found by PR 6385's re-review.

- `topo/src/bound_side.rs` `Unresolved::Gap` (≈484) now means the
  witness's gap for a witnessed edge (its own point at the arm in the
  gap) and the leading term's for every bound no witness reads (a
  bisector, a face neither a sphere nor a cylinder).
- `ReadFault::Witness` merges "the edge comes back across the face
  within its reach" (a decided contradiction,
  `k_stats::gate_rejection`) with "the witness reading is poisoned or
  straddles the zero band".

No consumer (`sectors::bound_read`, `splitting::entry_side`,
`sectors::read_refusal`) can tell them apart, so a refusal names neither
cause. Owed: split each into its own arm, or a payload that says which.

Moved from TANGRIM to TANG on 2026-11-20, by `git mv`, when TANGRIM merged into TANG: TANGRIM held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one TANG carries.
