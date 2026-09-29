---
id: the-at-rest-badge-restates-a-failed-root-louder-than-its-row
kind: issue
title: the at-rest badge restates a failed or poisoned root as an Actionable refusal naming no cause, above the tree row that carries it
status: open
opened: 2026-09-25
priority: P3
cost: E
refs: [a-derived-pick-index-failure-outshouts-its-cause, band-refusal-still-badges-every-row]
---


Found by the sweep of `a-derived-pick-index-failure-outshouts-its-cause`
(the class: *a badge or line whose fault follows from a node failure
the feature tree already shows, drawn at least as loud as that row*).

## Finding

`crates/viewer/src/session.rs`, the landing's gather-refusal arm: on an
assembly-shaped document, **every** gather fault becomes
`AtRestBadge::Refused { message: AssemblyError::product_refusal(&fault) }`,
and `frame::at_rest_badge` draws that `Tone::Actionable`. For
`ProductError::RootFailed` / `RootPoisoned` the message is
*"assembly: product: root N failed to evaluate (ask
`Evaluation::node_error` for the typed cause)"* — a restatement of the
failure the tree already badges `Failed` at the node, louder than the
row's own words (which the Features pane draws `Tone::Advisory` under
an `Actionable` tag), and naming the ROOT rather than the row that
carries the cause when the root is poisoned.

`frame::product_badge` declines exactly these classes
(`frame::badge_site` sends them to `BadgeSite::FeatureTree`), and
`frame::index_badge` now draws its own consequence of them as
`Tone::Advisory`, naming `tree::cause_row`. The at-rest badge is the one
sibling left saying it loudly.

**Why it was not taken with the pick-index fix.** The typed fault is
gone by the time the badge is drawn: `AtRestBadge::Refused` carries a
`String`, so `frame.rs` cannot tell a derived refusal from the gate's
own. The fix needs the session to keep the class (or the root) beside
the message, and `session.rs` is AUTHOR's. CHROME's
`band-refusal-still-badges-every-row` (deferred) weighs the same badge
from the other side — option (b) there puts the cause INTO it — so a
taker should read that row first; the two answers are not the same, and
one of them has to be chosen.
