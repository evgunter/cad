---
id: tone-doc-argues-from-a-site-that-now-reads-the-value
kind: issue
title: The actionable-or-not rule is attributed to the pane in four prose homes the move made stale
status: open
opened: 2026-09-19
priority: P4
cost: E
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

**One in `theme.rs` (`:318-322`), NOT stale but a fourth home:**
`Theme::unresolved`'s doc re-derives the classification in prose —
*"(A POISONED badge is not on the list: a row showing someone else's
failure draws quiet, so the colour stays on the row to act on.)"* It is
still TRUE, and that is the point: a palette field has no reason to
know which statuses are actionable, and a restatement that happens to
agree today is how the next divergence starts. It wants a citation of
`RowStatus::tone()` in place of the re-derivation.

**One in `crates/viewer/README.md` (`:833-836`)** — filed separately on
VDOC as `viewer-readme-attributes-the-tone-rule-to-the-pane`, because
that file is VDOC's and a prose defect there is filed, never fixed.

## What is NOT a member

`theme.rs`'s dichromacy carve-out (`:578-581`) — *"every badge that
uses it carries its own words"* — is untouched and stays true;
`app::toned`'s doc now carries that same contract at the one place the
mapping is made. `crates/viewer/tests/tree_badges.rs`'s tone prose was
written by the same lane against the post-move tree and is correct.

Ride the two `frame.rs` members on whichever VNEWS unit next holds that
file; the `theme.rs` member needs whoever holds `theme.rs`.

## Adjudicated 2026-09-20 (`vnews/frame-cluster-order`)

### Both `frame.rs` members are there, still false, and pinned

- The `Tone` header is `crates/viewer/src/frame.rs:1162-1176`; the
  false attribution is `:1166-1171` (*"The Features pane argues it
  explicitly for rows … and until this type existed no value stated
  it"*).
- `product_badge`'s doc carries the second at
  `crates/viewer/src/frame.rs:1764-1768` (*"The Features pane goes
  further and draws a poisoned row deliberately QUIET"*).

Both are false because the rule is now a value and the pane reads it:
`tree::RowStatus::tone()` is `crates/viewer/src/tree.rs:149-154` and
`crates/viewer/src/pane/features.rs:89` draws
`toned(row.status.badge(), &self.theme, row.status.tone())`, with
`:81`'s own comment naming `RowStatus::tone()` as the source.

### The `theme.rs` member is out of fence and stays out

**Its citation moved under this branch and every citation in the file
was repointed with it.** `main` rewrote `crates/viewer/src/theme.rs`
between this branch's base and its merge-in, so the row's four
approximate pointers were re-derived rather than shifted: the
`Theme::unresolved` doc is now `:318-322` (was `~:201-205`), the
dichromacy carve-out `:578-581` (was `~:461-464`), and the
`crates/viewer/README.md` member `:833-836` (was `~:835`). Class-wide
over the file, because `work/view/plan.md`'s rule is that a half-fixed
count or citation is worse than a uniformly stale one.

`crates/viewer/src/theme.rs:318-322` is there and is still TRUE, as the
row says. `theme.rs` is one of the eleven files
`work/view/viewer-src-files-no-successor-claims` reports as claimed by
no re-scope successor, so it cannot ride a VNEWS lane. The `frame.rs`
half does not wait on it.

### Disposition: two of four members ride; the row does NOT close with them

The row's population is four sentences and they have three different
owners, so the rider disposes of half of it and no more:

| member | disposition |
|---|---|
| `crates/viewer/src/frame.rs:1166-1171` | rides group A's carrier |
| `crates/viewer/src/frame.rs:1764-1768` | rides group A's carrier |
| `crates/viewer/src/theme.rs:318-322` | **stays open on this row** — `theme.rs` is claimed by no dispatching program |
| `crates/viewer/README.md:833-836` | already filed on VDOC as `viewer-readme-attributes-the-tone-rule-to-the-pane`; not this row's |

So **this row stays open after group A lands**, carrying the `theme.rs`
member alone, and its `rides_with:` names the carrier for the half that
rides rather than for the row. The `theme.rs` half is not blocked on
anything group A does; it is blocked on the same territory question as
`environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere`
(`work/view/viewer-src-files-no-successor-claims`), and it is the
cheaper half of that question — one citation replacing a
re-derivation, on a sentence that is TRUE today.

No Ev gate on either half. `crates/viewer/GUI-DESIGN.md` G4's tree
clause (`:97-99`) states the POISONED-draws-quiet rule itself and is
untouched by any of the four repairs: what moves is only WHO the doc
says states it, which is a fact about this crate's code and not a
design choice. The general finding lives once, in
`work/vnews/rank-one-discards-the-frames-other-news`'s adjudication
section.

## The two `frame.rs` members landed, 2026-09-24 (`vnews/frame-rs-prose-pass`)

- **`Tone`'s header** no longer says the Features pane argues the rule
  or that no value stated it. It says the feature tree's rows follow
  the same rule, stated by `tree::RowStatus::tone` — a poisoned row is
  `Tone::Advisory` — that both families state a tone as a value, and
  that `app::toned` is the one place a tone becomes a colour.
- **`product_badge`'s doc** keeps its argument and changes its
  attribution: `RowStatus::tone` makes a poisoned row `Advisory`, the
  Features pane draws that value, and a badge shouting about the same
  poisoning would have the chrome say both things at once.

The carrier those two rode on is closed, so `rides_with:` is removed:
**this row stays open with its `theme.rs` member alone**
(`Theme::unresolved`'s doc re-deriving the classification), which the
adjudication above leaves on the territory question `theme.rs` is
waiting on. The `README.md` member was never this row's.
