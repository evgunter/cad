---
id: a-mate-lift-through-an-indexed-seat-lifts-the-whole-family
kind: issue
title: The mate member walk's lift through an indexed body seat lifts every name of the family, not the indexed member's
status: open
opened: 2026-10-10
---


`names::lift` (`crates/editor-core/src/names/role.rs:1853`) decides whether a
consumer reads its input by the seat's `read` alone (`seat(target.read, …)`,
`role.rs:1886`, and the member/subtract/target arms at `role.rs:1900`–`1929`).
An indexed seat `xs[i]` reads one member of the family `xs`, but the lift
treats it as reading the whole family: a mate declaration reached through
`Union([xs[0], …])` lifts every member's names, and the member the walk
continues at is not narrowed to the index. The evaluation keys names by `xs`
and projects the family at the index (`eval/wire.rs`, `reads_projected`), so
the published names are right; what over-reaches is the mate member walk's
lift, which should keep only the names of instance `i` (`Instance { i, .. }`
heads) when the seat carries an index, evaluating the index slot as the walk
evaluates a `Part(Instance)` pick.

Found building FORK-DM4 unit 1's indexed read (`intent/dm4-names-keyed-by-read`).
