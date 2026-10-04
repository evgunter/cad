---
id: site-census-attributes-a-parents-decisions-to-its-nested-fn
kind: issue
title: every_site_names_the_decision_it_raises reads a nested fn as top level, so a nested fn steals its parent's later decision rows
status: open
opened: 2026-10-04
---

Found by both reviews of PR 4008 (r1 NOTE 6, r2 Q4) and passed on by
the JOIN orchestrator.

## What

`crates/topo/src/boolean/offer_rows.rs`'s
`every_site_names_the_decision_it_raises` attributes each decision
mention to the last line that `top_level_fn` reads as a `fn`.
`top_level_fn` accepts any `fn` indented four spaces or less. A fn
nested inside a top-level fn body (four spaces) therefore counts as top
level, and every decision written after it in the parent is filed under
the nested fn.

Witness: before PR 4008, `boolean/join.rs`' `find_match` held the
nested `fn slots` before its `escalate` closure, and the table carried
`("join.rs", "slots", "Coincide::Join", 1)` for a decision `find_match`
raises. PR 4008 moved that closure into `nearer` and the row went away.
The census followed text position, not the decision, so moving the
decision is what removed the row.

## The shape of a fix

Track brace depth (or the enclosing `fn` item through `syn`), so a
nested fn's span ends with its own body, and decisions after it belong
to the parent again.
