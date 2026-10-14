---
id: a-dimensionless-computed-value-reads-with-a-trailing-space
kind: issue
title: a dimensionless computed value reads with a trailing space
status: open
opened: 2026-09-30
priority: P4
cost: E
---


Found by AUTH-7's correctness review (PR 3528). The behaviour is
older than this PR. `props::written_text` formats `"{number} {symbol}"`
(`crates/viewer/src/props.rs`, `written_text`), and the dimensionless
unit's symbol is empty (`quantity::ONE`, `ScalarUnit::of_row("")`).
So `props::computed_text(Dimension::Scalar, 0.25)` reads `"0.25 "`
with a trailing space. It reaches:

* a driven scalar slot's field (`= 0.25 `);
* the refusal's affordance;
* since AUTH-7, a dimensionless measure's tree row.

Visually it is a stray space. In a sentence that quotes the value, it
is a double space. The fix is to leave out the separator when the
symbol is empty, in `written_text`, the one home, with a row pinning
the scalar case.
