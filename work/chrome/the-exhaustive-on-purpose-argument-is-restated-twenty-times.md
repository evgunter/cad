---
id: the-exhaustive-on-purpose-argument-is-restated-twenty-times
kind: issue
title: The 'a subset pattern answers false for everything it does not name' argument is restated bespoke in ~20 places across crates/viewer/src
status: open
opened: 2026-09-22
priority: P4
cost: E
---


## Finding

*"A subset pattern answers `false` for everything it does not name"*
is a fact about Rust, not about any one policy. `crates/viewer/src`
writes it out bespoke, in its own words, at roughly twenty sites —
each an *exhaustive on purpose* / *no wildcard* comment carrying its
own restatement of the generic half before it gets to the local half
(which is the part worth reading):

`app.rs`, `combine.rs`, `datums.rs`, `forms.rs`, `frame.rs` (×5),
`gpu.rs`, `pickindex.rs`, `session.rs` (×2), `sketch.rs`, `tree.rs`,
`widgets.rs`, `pane/features.rs`, `pane/viewport.rs`,
`session/author.rs`.

**Re-take it rather than trusting that list**, which is the enumerated
subset a reviewer read: `grep -rni exhaustive crates/viewer/src` is 60
lines in **21** files today (the six the list above does not name are
`matetool.rs`, `pickcache.rs`, `seats.rs`, `session/op.rs`,
`session/refuse.rs`, `tools.rs`, `vocab.rs`), and `grep -rni wildcard`
is 11 more. Those are lines, not sites — a multi-line comment counts
several times and a few are unrelated uses of the word — so the number
of RESTATEMENTS is smaller than 60 and larger than the 20 enumerated.
Whoever takes this reads the 71 lines once; the point of the counts is
that nobody has.

`chrome/rowstatus-exhaustive` (PR 3055) writes it a twenty-first time,
at `tree::has_faults`, and that is what makes it worth filing: the
argument is being re-derived by every lane that meets the construct,
which is how a generic fact ends up with twenty slightly different
statements and no home.

## Why it is filed and not fixed

The consolidation is the hard part, not the finding. The local halves
differ and are the valuable ones — *which* states this policy leaves
out, and why — so a merge that keeps them and hoists only the generic
sentence has to decide where the generic sentence lives (a `README`
clause the comments cite? one module doc?) and then touch twenty
comments in fourteen files for no behaviour. That is a sitting of its
own, and it is a taste call that wants deciding once rather than
twenty times.

Related but not the same class:
`six-viewer-sites-restate-the-empty-document-rule-and-its-badge-policy`
is about a DOMAIN rule restated; this is about the language fact the
restatements share.

## Filed from outside the fence

Found by the style review of PR 3055, whose fence was `tree.rs` and
`crates/viewer/tests/`. Every named path is claimed by CHROME jointly
with one or more of AUTHOR, VIEW, VNEWS, VSEAM and VGEOM.

## Evidence 2026-09-22 (`chrome/badge-attribution`, PR 3090)

That lane made `pane/features.rs`'s link decision in `feature_row` an
exhaustive `match` and minted a fresh restatement above it (*"Exhaustive
for the badge match's reason…"*); review caught it and the fix pass cut
it to nothing — the badge match twelve lines up carries the argument
for both. The site is an instance of the construct this row counts,
with no comment of its own, which is the shape this row wants the
others to end in.
