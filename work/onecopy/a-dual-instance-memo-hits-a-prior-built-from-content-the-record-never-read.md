---
id: a-dual-instance-memo-hits-a-prior-built-from-content-the-record-never-read
kind: issue
title: A dual's instance node memo-hits a prior value built from content its record never read
status: closed
opened: 2026-11-16
priority: P3
closed: 2026-11-16
pr: 6790
---


Filed from PR 6790's review (F1). PR 6790 pairs a dual's part run with the record's build of the part by content (`RecordBinding::of_part`, `content_pairing` in `crates/editor-core/src/eval/mod.rs`). It also reuses a prior's part row only where that row read the record's content (`PartStore::enter`, `crates/editor-core/src/eval/parts.rs:868`). A node's own memo still keys an instance by its part's `DocRef`, so it trusts that one pin names one content (A4).

**Measured, with the review's probe (a scratch run, not kept):**
- `D1` is a dual over an honest record `R1` and a resolver answering the part's pin with content `X`. Its instance builds.
- `R2` is a record whose pin-blind resolver answers the same pin with content `Y`.
- `dual(None, R2)` over the `X` resolver refuses the instance as `RecordMispaired(Revision)`.
- `dual(Some(D1), R2)` returns the instance `Ok`: a body from `X`, under a record that read `Y`.
  - The part row is not reused (6790's carry check): `part_evaluations == 1`.
  - The instance node is a memo hit: `reused == 1`, at `mod.rs:5502` (`v.content_key == content_key`).

So the memo is not transparent on a dual run where a resolver breaks A4. The same premise holds at every scalar: an `f64` run over a prior from another resolver's content reuses the instance alike. It is the node memo's own A4 premise, the one `PartFault::ReferenceCycle` rests on.

**Routes:**
1. In a record-bound run, an instantiate node takes no memo hit where its part's row was not reused from the prior. This is plumbing from `PartStore::enter` to the node's memo door.
2. The kernel checks A4 itself at `Entered::enter`, for every scalar. That is the broader route the closed row `a-dual-part-bound-under-a-pin-its-resolver-answers-otherwise-reads-record-refused` named, and it would close this row and make `ReferenceCycle` unreachable alike.

Reachable only through a resolver that answers a pin with other content; no shipped resolver does.

**Resolved (PR 6790), by route 1.** `parts::Reached::carried_from` says whether this run's row for a part is the one the prior carried. `PartStore::enter` takes a reused row from the prior by clone, so that is pointer identity on its `Refusals`. `eval_any` filters the prior's value of an instantiate node through it, families included. A node whose part row was rebuilt re-runs; an honest run's hits are unchanged, at one pointer compare per lookup. Pinned by `asm2a_instantiate::an_unmoved_instance_is_not_a_memo_hit_where_its_part_row_was_rebuilt`: the probe above (`part_evaluations == 1`, `reused == 0`, refused), and a run whose resolver agrees with the record's `Y` while the prior built `X` (rebuilt, and the instance reads `Y`). Both shapes measured red without the filter (`reused == 1`).

**Not closed by it:** an `f64` run over a prior from another resolver's content still reuses the part row by key, and so the instance with it. That is the part store's A4 premise itself, with no record to pair against, so it stays with route 2.
