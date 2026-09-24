---
id: at-rest-badge-repeats-a-gather-refusal-another-channel-carries
kind: issue
title: the at-rest badge repeats every gather refusal of an assembly-shaped document, including the ones frame::badge_site sends to the feature tree or to product_badge
status: open
opened: 2026-09-24
priority: P3
cost: E
---


## Finding

`crates/viewer/src/session.rs`, `DocSession::land`'s `Err(fault)` arm.
After `chrome/empty-doc-badge`, an assembly-shaped document whose gather
refuses takes `AtRestBadge::Refused { message:
AssemblyError::product_refusal(&fault) }` for every class except the one
`ProductErrorKind::means_no_body` reads as an absence. `app.rs`'s badge
column then draws it through `frame::at_rest_badge`, in
`Tone::Actionable`, beside the two channels `frame::badge_site` already
gives the same refusal:

- a `Frame` class (a naming collision across roots, a graft, a validity
  verdict) is also drawn by `frame::product_badge` in the same column,
  so the reader sees *"product: …"* and *"at rest: assembly: product:
  …"* one badge apart;
- a `FeatureTree` class (`RootFailed`, `RootPoisoned`, `UnknownNode`)
  is badged at the node by the Features pane, and `badge_site`'s doc
  argues that a frame badge for it "would say strictly less, in a
  louder colour" and, for a poisoned root, contradict the pane's
  deliberately quiet row. The at-rest badge is exactly such a frame
  badge.

So the argument `badge_site` makes for `product_badge` holds for the
at-rest badge and is not applied to it. Found by the class sweep for
`at-rest-badge-reports-an-empty-document-as-a-refusal`; not fixed there,
because that row's evidence covers the no-body class only and whether a
gather refusal belongs on the at-rest badge at all is a decision about
what the A5 badge is for.

## What a taker owes

A decision: either the at-rest badge takes no verdict when the gather
refused (the gate never ran, and every class already has a channel or
is an absence), or it is a deliberate second report and its doc says
why. If the first, `AtRestBadge`'s doc and `DocSession::at_rest`'s
`None` gain that case and `crates/viewer/tests/landing_gathers.rs`
gains a row over a gather refusal in an assembly-shaped document; if
the second, the `badge_site` argument needs a sentence on why it stops
at `product_badge`.

## Further evidence (PR 3135's review)

- **The landing now holds its own class policy for the at-rest badge.**
  `DocSession::land`'s `Err(fault)` arm (`session.rs`) badges "every
  class but the one `means_no_body` claims" — a boolean over the class,
  so it is a non-exhaustive sibling of `frame::badge_site`'s `match`. A
  new `ProductErrorKind` reds `badge_site` and silently inherits
  `Refused` here. Either option under "What a taker owes" removes the
  second policy or
  makes it answer to the first.
- **The badge's words claim a gate run that did not happen.** The
  `Refused` built in that arm renders
  `AssemblyError::product_refusal(&fault)` — *"assembly: product: …"* —
  though on a gather refusal the A5 gate never ran: there was no
  product to hand it. That is a further argument for option 1 (no
  at-rest verdict when the gather refused).
