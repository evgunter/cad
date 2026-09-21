---
id: tone-doc-argues-from-a-site-that-now-reads-the-value
kind: issue
title: The actionable-or-not rule is attributed to the pane in four prose homes the move made stale
status: open
opened: 2026-09-19
---


Filed by `tone-is-a-value-in-frame-and-a-comment-in-two-panes`'s lane
and widened by its fix pass from one instance to the **population**.
`crates/viewer/src/frame.rs` is serialized across several VNEWS rows
and `crates/viewer/src/theme.rs` is out of fence, so none of these
could be fixed by the lane that made them stale.

## The sweep rule that produces the population

**Every sentence under `crates/viewer` that attributes the
actionable-or-not rule to a SITE rather than to the value that now
states it** (`tree::RowStatus::tone`), or that claims a single home for
the tone-to-chrome mapping. Run as a grep over `Features pane`, `the
one place` / `only place`, `goes further`, `argues`, `picked a colour`
/ `at the call site`, and `act on` / `draws quiet` / `deliberately
QUIET`, each hit read for whether the move made it false — the
prose-census instrument, not a list of noticed sentences.

## The members

**Two in `frame.rs`, both stale, both about the same move:**

- the `Tone` header (~`:1166-1170`) — *"The Features pane argues it
  explicitly for rows — a poisoned row is deliberately QUIET so the eye
  goes to the failed row a reader can do something about — and until
  this type existed no value stated it"*. The pane does not argue it:
  `RowStatus::tone()` states it and the pane reads the value.
- `product_badge`'s doc (~`:1764`) — *"The Features pane goes further
  and draws a poisoned row deliberately QUIET, reserving
  `Tone::Actionable` for the row a reader can act on"*. Same false
  attribution, in the doc that uses the rule to argue why the product
  channel stays silent. **The argument survives; the attribution does
  not** — the reason a badge here would double up is that the tree
  already carries the tone, which is now a value both channels read.

**One in `theme.rs` (~`:201-205`), NOT stale but a fourth home:**
`Theme::unresolved`'s doc re-derives the classification in prose —
*"(A POISONED badge is not on the list: a row showing someone else's
failure draws quiet, so the colour stays on the row to act on.)"* It is
still TRUE, and that is the point: a palette field has no reason to
know which statuses are actionable, and a restatement that happens to
agree today is how the next divergence starts. It wants a citation of
`RowStatus::tone()` in place of the re-derivation.

**One in `crates/viewer/README.md` (~`:835`)** — filed separately on
VDOC as `viewer-readme-attributes-the-tone-rule-to-the-pane`, because
that file is VDOC's and a prose defect there is filed, never fixed.

## What is NOT a member

`theme.rs`'s dichromacy carve-out (~`:461-464`) — *"every badge that
uses it carries its own words"* — is untouched and stays true;
`app::toned`'s doc now carries that same contract at the one place the
mapping is made. `crates/viewer/tests/tree_badges.rs`'s tone prose was
written by the same lane against the post-move tree and is correct.

Ride the two `frame.rs` members on whichever VNEWS unit next holds that
file; the `theme.rs` member needs whoever holds `theme.rs`.
