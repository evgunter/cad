---
id: refusals-render-row-dropped-the-clause-that-caught-its-own-defect
kind: issue
title: refusals_render_as_sentences dropped the quote clause that caught the defect it was written for
status: open
opened: 2026-09-06
refs: [2053]
---


Found by the style review of PR 2053 (`view/refusal-all`).

## What moved

`crates/viewer/tests/panel_edits.rs:523`. The row's dump assertion was

    !rendered.contains('{') && !rendered.contains('"')

and is now

    !rendered.contains('{') && !rendered.contains("node:") && !rendered.contains("name:")

## Why the trade is not even

The clause that went is the one the row's own doc comment credited with
the catch it exists for. Before 2053 that comment read: *"The title used
to say 'every refusal' while the body exercised the `Io` arm alone,
which is why it stayed green while `Refusal::Edit` rendered a
`{:?}`-quoted parameter name into the status line."* A `{:?}` over a
`String` or a `ParamName` produces `"width"` — no brace, no `node:`, no
`name:`, and no variant identifier for the `!rendered.contains(arm)`
clause to catch, because the identifier that appears is the payload's,
not the arm's. That regression now passes this row.

The clauses that arrived buy very little. A `Debug` rendering that
contains `node: ` or `name: ` is a named-field struct or struct
variant, so it contains `{` too and the first clause already had it.
They fire only on a hand-written `"node:"` prefix in prose.

Neither is forced by the other: none of the six arms this row walks
renders a quotation mark today, so keeping the quote clause alongside
F6's would have been green. The PR's argument for dropping it — that
`EditError`'s metadata arms quote a user's key deliberately — is about
arms this row does not walk, and would bite only if the roster were
widened to them.

## The shape of a fix

Either restore the quote clause for the six sampled arms (they are all
quote-free and none is a metadata arm), or replace it with something
that actually catches a `{:?}`-rendered payload — the row's own
`assert_ne!(rendered, format!("{refusal:?}"))` is a whole-string
comparison and does not see a `Debug` fragment inside prose.
