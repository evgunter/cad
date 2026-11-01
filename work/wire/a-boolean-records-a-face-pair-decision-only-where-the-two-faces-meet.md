---
id: a-boolean-records-a-face-pair-decision-only-where-the-two-faces-meet
kind: issue
title: A boolean records a face-pair Zero decision only where the two faces meet, so a carrier identity the sweep decided between faces that do not meet is no row
status: open
opened: 2026-10-19
priority: P3
cost: E
design: true
---


Filed by WIRE's `wire-citerows` lane (branch
`wire/boolean-rows-every-decision`), from the sweep for the shape of
`a-boolean-records-a-zero-decision-only-where-a-record-cites-it`.

## What

That unit makes every vertex identity a boolean decides with a margin a
`Coincidence` (`boolean/ops.rs` `cite_rows`). The glue door's face-pair
rows keep their own filter: `boolean/glue.rs` `touched` keeps a row
deciding faces `fa`, `fb` one carrier, tangent or coaxial only where the
reduction placed a cell of `fa`'s closure on a cell of `fb`'s, and drops
the rest (`boolean/mod.rs`, the `glue::touched` call). Its stated reason
is frame independence: which pairs a box sweep offers depends on the
frame, and whether two faces meet does not.

## The question

DM4 (`crates/editor-core/REFERENCES.md`, "Contact is judged pairwise",
approved on PR 5329) says a node's rows are its evaluation's decisions,
complete. A face-pair decision between faces that do not meet is a Zero
the boolean took, and it is also one a turned frame might not take.
Whether "every decision" covers it, or a frame-dependent offer is not a
decision of the op, is a reading of D10 `[coincidence.door]`, and the
`touched` filter's doc cites
`intent/a-face-pair-row-only-where-its-decision-shaped-the-result` for
the other half of the rule. Measure first: count dropped rows over the
corpus, and whether any shaped the result.

Signed (WIRE implementer, wire-citerows).

## Slate triage (2026-10-26, CLEAVE)

- Moved to WIRE (`work/wire/`) by `git mv`, id and body unchanged: whose lane filed it: the rows a node publishes and the lineage they are carried by. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
