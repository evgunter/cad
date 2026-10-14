---
id: live-tuple-constructor-point-free-is-unseen
kind: issue
title: the Live guard cannot see a point-free tuple construction (.map(Live) / .map(Self)), because Live and Self are also type names
status: open
opened: 2026-09-29
priority: P4
cost: M
design: true
refs: [point-free-surgery-openers-read-as-no-scope]
---


From PR 3425's needle sweep; also listed in `live.rs`'s "what it cannot
see". `CONSTRUCTIONS` and part 4 match `Live(` / `Self(`; a point-free
`.map(Live)` builds a `Live` unseen. Dropping the `(` does not work:
`Live` and `Self` are also type names (`-> Option<Live>`). Deciding it
means telling a value-position path from a type-position one — or
making the tuple constructor private so only `Live::new` can build one,
which would retire the question.
