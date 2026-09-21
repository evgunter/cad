---
id: check-10-states-one-outer-and-the-rest-void-but-not-that-a-void-lies-inside-it
kind: issue
title: check 10 refuses two outer shells under one solid but never states that a cavity wall lies INSIDE the outer boundary
status: open
priority: P2
cost: D
parent: ATREST-1
opened: 2026-09-20
---

Disclosed by ATREST-1 at the moment check 10 landed, per its spec's
D-C: SHELL-5's row describes the full check as *"exactly one `Outer`,
every other shell `Void` and inside it"*, and the `and inside it` half
is not made.

`ValidationError::MultipleOuterShells` (`crates/topo/src/validate.rs`,
`shell_roles_of`) refuses a solid whose shells hold two or more
definitely-positive enclosures. It says nothing about WHERE a cavity
wall sits: a shell enclosing negative volume is counted as a cavity of
that solid wherever it is in space, so a solid holding an outer
boundary and a "cavity" that is nowhere near it — or that sits inside
a DIFFERENT solid — passes check 10.

Nesting is a containment claim, and tier 3 has no at-rest containment
walk for it. It is the same family as check 9's deferred nesting half
(`validate-tier3-curved-boundary-containment` and
`check-9-nesting-is-line-bounded-only` carry the face-level version of
the same gap), and the walk that would answer it is
`shell::encloses`-shaped rather than flux-shaped.

The check's own rustdoc cites this file.
