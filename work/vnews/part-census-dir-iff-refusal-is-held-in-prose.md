---
id: part-census-dir-iff-refusal-is-held-in-prose
kind: issue
title: PartCensus::taken is pub and accepts a census with no directory and a listing; the dir-iff-refusal invariant part_listing leans on is prose
status: open
opened: 2026-09-25
priority: P4
cost: E
---

`pane::create::part_listing` (`crates/viewer/src/pane/create.rs:316-324`)
draws no header for a chooser with no directory, because such a chooser
has refused with `Refusal::NoDocumentDirectory`: both halves are minted
from the same resolver in `DocSession::part_census`
(`crates/viewer/src/session.rs:1360`). That pairing is held by that one
caller. `parts::PartCensus::taken` (`crates/viewer/src/parts.rs:151`) is
`pub` and accepts any pair, including `(None, Ok(entries))`, which would
draw a listing with no directory named and nothing saying why.

Found by #3230's final review. The invariant belongs in the type: a
census either has a directory and a scan's answer, or has none and
refuses, e.g. `PartCensus::{Scanned { dir, offered }, NoDirectory}`, so
`part_listing`'s silence is structural rather than argued in a doc.
