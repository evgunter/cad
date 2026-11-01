---
id: a-spelled-union-clones-the-results-per-member
kind: issue
title: wire_combine projects each spelled member by cloning the whole Results map, O(members x nodes)
status: open
opened: 2026-10-10
---


`wire_combine` (`crates/editor-core/src/eval/wire.rs`) resolves each spelled
member through `reads_projected`, which clones the whole `Results` map whenever
it projects (an indexed read, a split port). A `Union([xs[0], …, xs[N−1]])`
therefore clones `Results` N times, O(N · nodes), where the family form
`Union(xs)` reads the same bodies in O(N). Unmeasured; it matters for large
documents with long spelled member lists of indexed reads. A projection that
overlays only the projected entries (or projects into the member list directly,
as the family arm does) closes it.

Raised by review r1 of PR 4527 (style S3).
