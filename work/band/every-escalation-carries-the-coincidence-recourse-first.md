---
id: every-escalation-carries-the-coincidence-recourse-first
kind: issue
title: blend: every Escalated renders the coincidence recourse BEFORE the routed one, and no blend door takes a declaration
status: open
opened: 2026-09-08
priority: P1
cost: E
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

## Another instance, repaired at the site (BLEND-10 fix pass, 2026-09-13)

`PathError::Escalated`'s `Display` (`crates/profile/src/path.rs`) has the
same shape: its fallback arm writes `"path junction classification:
{source}"`, and `{source}` is `Indeterminate`'s own `Display` — the
margin payload followed by `COINCIDENCE_RECOURSE`. Every escalation the
path door relays without a keyed arm therefore tells the reader to
declare a coincidence first.

The instance BLEND-10 met: the fillet door now re-reads the loop it is
about to emit through the verify layer's own segment and joint
predicates, and relays an in-band classification through that arm. The
reader of a refusal about a joint THE DOOR ITSELF MINTED was told to
declare it — a declaration they never wrote and cannot add, since the
door writes the declaration set. Repaired by giving those eight
predicate names their own arm, which names the site ("reading back the
fillet arc this door is about to store") and the levers the stored form
actually has (`FILLET_STORED_FORM_INBAND_RECOURSE`), and which does not
render `{source}` whole. The row is
`fillet_recourse_followability.rs`'s
`the_stored_form_inband_recourse_is_followed_by_dropping_the_fillet`,
which asserts the sentence does NOT contain "declare the coincidence".

What stays open here: the fallback arm itself, and every other
escalation that reaches it.

## A second instance, repaired at the site (BLEND-12, 2026-09-13)

The same arm, the other family. The nine `fillet_*` gates the
construction sugar decides (`crates/profile/src/sugar.rs`) reached the
fallback arm too, so every in-band fillet verdict told its reader to
declare a coincidence at a joint the caller never authored — while the
six `FILLET_*_RECOURSE` sentences written for exactly those gates were
rendered only by a `ProfileError::Escalated { site:
EscalationSite::Fillet, .. }` arm that nothing constructed.

Repaired by giving the nine names their own arm, ahead of BLEND-10's
eight and of the junction keys: it names the site ("resolving the fillet
at this corner") and appends the gate's own sentence, selected by
`validate::fillet_recourse_for` — the crate's one name-to-sentence map —
and does not render `{source}` whole. `EscalationSite::Fillet` was
retired with its arm. The rows are in
`crates/profile/tests/fillet_recourse_followability.rs`, including a
census over the nine names read out of `sugar.rs`'s source.

What stays open here is unchanged: the fallback arm itself, and every
other escalation that reaches it — the junction keys keep the shared
sentence on purpose, because `.tangent()` is a door their caller has.

## The fallback arm's spelling moved (BLEND-15, 2026-09-13)

The arm described above no longer writes `"path junction
classification: {source}"` for every unkeyed name — that label is now a
named arm for the two junction keys, and every other unkeyed name
renders `"escalated: {source} — {geom_core::MissingRecourse}"`.

The class this item is about is unchanged: the fallback still renders
`{source}` whole, so `COINCIDENCE_RECOURSE` still arrives first, ahead
of whatever the site's own levers are. Only the sentence that follows
it has changed, from a false category to a named gap.
