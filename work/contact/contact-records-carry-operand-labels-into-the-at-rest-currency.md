---
id: contact-records-carry-operand-labels-into-the-at-rest-currency
kind: issue
title: ContactRecords' operand-labelled lists (a_on_b, b_on_a; VvContact a/b as A-clone/B-clone keys) are reduction vocabulary used as the at-rest declaration type
status: open
opened: 2026-10-02
priority: P3
cost: M
refs: [split-halves-have-no-contact-records-so-no-pseudomanifold-self-check]
---


## What

`ContactRecords` (`crates/topo/src/boolean/mod.rs`) labels its lists by operand. At rest the labels mean nothing: `census`'s `Declared::index` chains `a_on_b` with `b_on_a`, as do `product::carry_contacts` and editor-core's `checks.rs`; step-import and assembly fill one list arbitrarily; and a split result has no A or B at all. One type means two things. Both designers recommend a symmetric at-rest record type (`vv`, `vf`, `curves`, `patches`) in the contact layer, with the A/B lists kept on `BooleanReduction` where `shell_witness` and `coplanar_conic_rows` read them.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.
