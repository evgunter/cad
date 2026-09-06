---
id: node-tag-space-census-blind-to-tags-outside-sentinels
kind: issue
title: The content-key tag-space injectivity census reads only between the NODE-TAG-SPACE sentinels, so tags 41–45 written outside them are invisible to it — and node tag 5 already coexists with payload tag 5
status: open
opened: 2026-09-05
refs: [SEAT-7, 1910, 1593]
---


(SEAT orchestrator) Class finding from SEAT-7's dual review (PR 1910),
filed per the durable-home rule; unowned — the content-key machinery in
`crates/editor-core/src/eval/mod.rs` is the memo's, not a verb program's.

**The finding.** `node_tag_space_is_injective` (`eval/mod.rs`, the
census near the `NODE-TAG-SPACE` sentinels) reads tags only BETWEEN its
sentinels, and its own doc says a tag written outside them "does not
exist today" — yet tags 41, 42, 43, 44 and now 45 (SEAT-7's carrier-
radius feed; SEAT-6's v4 bump before it) are all written outside them,
each with a site comment of the form "the next free number in this
key's tag space … append-only". Nothing enforces "next free" or
"append-only"; a collision would be caught by nothing. The comment is
also loose: node tag `5` (Extrude) and payload tag `5` coexist today,
so this is demonstrably not one space, and the sentence claiming it is
should say which spaces exist.

**What a fix needs.** Either the census grows to cover every
`write_tag`/`write_u8`-shaped tag site in the key machinery (a grep-
census over the file with the sentinels retired), or the tag space is
declared once as a closed enum with `ALL` and the sites read from it —
the `ScalarParam::ALL` shape this repo uses for every other census.
Keys are process-internal and never persisted, so the fix is free of
wire consequences; the risk it closes is a silent key collision that
serves a wrong memo entry.
