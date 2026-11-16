---
id: tests-decided-names-resolve-references-tree-wide
kind: issue
title: The coverage row resolves every reference against names any crate's tests/ decide, so a suite's own name can answer another crate's fixture
status: open
opened: 2026-11-10
priority: P3
cost: E
---


Found by the review of ONECOPY's roster-reach (#6715, R2).

`crates/test-utils/tests/predicate_name_coverage.rs` builds the union a reference must resolve against from every crate's `src` census and the names every crate's `tests/` decide. The tests half is there so a suite's own fixture, a name a test decides and then asserts, resolves. But the union is tree-wide, so a name only some other crate's suite decides resolves too. A fixture in crate A naming a row decided only inside crate B's `tests/` is green, though no production door raises it.

**The narrower reading:** a reference in crate C's `tests/` resolves against the `src` union plus C's own `tests/` names. A reference in any `src` resolves against the `src` union alone.

**Done** when the coverage row keys the tests half per crate, with a plant (a crate-A fixture naming a crate-B suite's own row) red.
