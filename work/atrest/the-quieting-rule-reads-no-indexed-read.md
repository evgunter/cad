---
id: the-quieting-rule-reads-no-indexed-read
kind: issue
title: The quieting rule refuses every indexed read, so no assertion quiets a row or an overlap between two members of one family
status: open
opened: 2026-11-22
priority: P2
cost: M
---


DESIGN `[assertions]`, **Families** (`docs/DESIGN.md`, the paragraph after `[assertions.quiet]`'s "a new contact or overlap anywhere else is loud"), says an overlap between two members "is quieted by an assertion reading two of them: `c[k]` and `c[(k + 1) mod N]` for neighbours in a ring, or two indices, one `within` the other, for all pairs". No such assertion quiets anything. Both halves of the rule drop an indexed read before matching:

- the row half, `checks::at_rest::reference` (`crates/editor-core/src/checks/at_rest.rs`), filters `!select.body.is_indexed()`;
- the interference half reads its references through the same decode (`gap_assertions` → `reference` → `face_read`).

The filter came with #7017 (`onehome/gather-sample`), before the OFFER unit that made a finding carry its quieting assertion. Since that unit, a row between two members of one family (fins resting each against the next) and an overlap between two members (fins closer than their width) carry no assertion and refuse at no severity (CHECKS DS6). That is safe, but the clause promises a recourse the kernel does not take. Pinned by `a_check_carries_the_assertion_that_quiets_it::a_row_between_two_members_carries_no_assertion` and `intent_s5_c_quietable::an_overlap_a_familys_copy_bounds_carries_the_familys_gap` (its "fins overlapping each other" case).

The fix is a reading of `c[k]` and `c[k + 1]` (or an inner index `within` the outer) as the member each names at the assertion's member, in both halves, and then the mint (`at_rest::quieting`, `at_rest::could_quiet`) offering it. Or, if the clause is what should move, an `[ev]` revision of DESIGN's sentence.
