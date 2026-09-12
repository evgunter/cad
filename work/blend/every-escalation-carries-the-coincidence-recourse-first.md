---
id: every-escalation-carries-the-coincidence-recourse-first
kind: issue
title: blend: every Escalated renders the coincidence recourse BEFORE the routed one, and no blend door takes a declaration
status: open
opened: 2026-09-08
---


## Finding

Every `BlendError::Escalated` renders TWO recourses, and the first one
names a lever the blend doors do not have.

`BlendError::Escalated`'s `Display` is
`"escalated at {site:?}: {source} — {recourse}"`
(`crates/sweep/src/blend/mod.rs:1342`, and the gap arm at `:1334`).
`{source}` is `geom_core::Indeterminate`'s own `Display`
(`crates/geom-core/src/predicate.rs:758-786`), which is the margin
payload followed by `COINCIDENCE_RECOURSE`
(`crates/geom-core/src/predicate.rs:690`) — "declare the coincidence,
move the geometry, or lower the tolerance (D4)". Only then does the
blend's routed sentence arrive. So the reader of any in-band blend
refusal is told to DECLARE THE COINCIDENCE first, and then told the
thing that is actually true of their situation.

**No blend door takes a declaration.** `fillet_edges` and
`chamfer_edges` (`crates/sweep/src/blend/build.rs:141`, `:395`) take
`(body, edges, size, tol)` and nothing else; there is no declaration
parameter, no declared-tangency flag on a `BlendRequest`
(`crates/sweep/src/blend/battery.rs`'s `BlendRequest` is
`body`/`edges`/`size`), and no `declare`-shaped door anywhere under
`crates/sweep/src/blend/`. Of the three levers the sentence offers,
one does not exist here and one ("lower the tolerance") is the wrong
direction at the coincidence sites — so the leading recourse is
between misleading and false at every blend escalation, on a body of
prose the user reads BEFORE the sentence that was routed for them.

The routed recourse also cannot repair it: it is APPENDED, so a reader
who follows the first sentence never reaches the second.

## The shape to follow

`profile`'s path door already solved this at its own non-declaring
sites: `PathError::Escalated` composes the message from
`source.payload()` — the payload view WITHOUT the shared coincidence
tail (`crates/geom-core/src/predicate.rs:753`) — and writes its own
recourse (`crates/profile/src/path.rs:1637`, `:1648`, `:1654`), with
the comment at `:1616-1627` stating exactly this reason: "the shared
`COINCIDENCE_RECOURSE` tail on the bare `Indeterminate` Display says
'declare the coincidence', which is meaningless at these sites". Its
junction keys keep the full Display, because there the declaration IS
a door the caller has (`.tangent()`).

Blend has no such door at any site, so the composition should be
`source.payload()` at every arm, with the routed sentence as the only
recourse. That is a change to what every in-band blend refusal reads
like, and several message-pinning suites assert against the rendered
text (`crates/sweep/tests/blend_recourse_followability.rs`,
`crates/sweep/tests/m5_pr12_refusals.rs`), so the re-baseline is part
of the unit.

## Prior art to check first

Whether `COINCIDENCE_RECOURSE`'s three levers should be conditioned at
the geom-core end instead — one door's escalation is another door's —
is the same question `escalation-recourse-dispatch-has-three-homes`
raises from the routing side. The two want deciding together.
