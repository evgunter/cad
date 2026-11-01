---
id: the-kernel-takes-any-count-has-four-homes-in-the-viewer
kind: issue
title: The circle_split count rule -- kernel takes any count, document admits any -- is restated at four viewer sites, one of them eight lines above its own refutation
status: open
opened: 2026-09-22
priority: P3
cost: D
---



## Finding

One rule about `circle_split` counts — *the cap is the form's, not the
kernel's; the document will hold any count* — is written out at four
sites in the viewer, in four spellings:

- `crates/viewer/src/forms.rs`, `MAX_CIRCLE_SPLIT`'s doc comment:
  *"A product cap and not the kernel's: the kernel takes any count"*
  (`:301`);
- the same doc comment, eight lines down: *"A committed profile can
  hold a larger count (the document admits any)"* (`:307`);
- `crates/viewer/src/widgets.rs`, in the `circle_split` count field:
  *"the field can be handed a committed profile's count, which the
  document admits above the cap"* (`:1152`);
- `crates/viewer/src/pane/profile.rs`, the doc comment of
  `drawing_a_locked_split_circle_above_the_cap_leaves_it_alone`:
  *"A committed `circle_split` above the form's count cap (the
  document admits any count)"* (`:414`).

This is CHROME's established class —
`six-viewer-sites-restate-the-empty-document-rule-and-its-badge-policy`,
`dimension-to-unit-ladders-have-six-homes-in-the-viewer`,
`drag-tick-has-three-homes`,
`positive-finite-predicate-has-six-homes-outside-datums-rs` — and the
usual cost applies: four places to correct when the rule moves, and no
way to tell from one of them whether the others agree.

## Why this instance is worse than the class's average

**The fourth home now sits eight lines above its own refutation.**
`pane/profile.rs`'s test doc comment says *"the document admits any
count"*, and directly under it is the fixture comment landed by
`chrome/split-circle-eps` deriving the interval of radii the document
admits at `n = 1025` — from which it follows that the document admits
any count only at a radius that clears both walls. The sentence is
true of the COUNT in isolation and false as a claim about the figure,
and the file now holds both readings in adjacent paragraphs.

The product consequence is
`work/forms/the-circle-split-cap-offers-counts-the-document-refuses.md`,
which measures the same walls on the surface a person touches. That
row is about the missing radius-aware bound; this one is about the
sentence being in four places, so that when the bound is written the
correction has four sites and no index.

Note that none of the four sentences is wrong as written about counts.
`circle_split_kernel` really does refuse only `n < 2`, and the cap
really is a preview-cost bound scoped to authoring. What they share is
that each is one clause short of the figure, and being four of them
makes adding that clause four edits.

## Home

CHROME. `crates/viewer/src/forms.rs` (two of the four, in one doc
comment), `crates/viewer/src/widgets.rs`,
`crates/viewer/src/pane/profile.rs`. Pre-existing; the fourth home's
awkward position is new with `chrome/split-circle-eps`, the sentence
itself is not.

## Where else to look, and what the sweep found there

Swept, with the result recorded rather than left as a pointer. The
obvious guess is that this is a per-capped-field habit — every
`forms.rs` cap constant explaining itself as a product cap, every
bounded `widgets.rs` field repeating it. **That set is empty.**
`MIN_CIRCLE_SPLIT` and `MAX_CIRCLE_SPLIT` are the only cap constants
in `forms.rs`, `:301` is the only *"product cap and not the kernel's"*
comment in `crates/viewer/src`, and `widgets.rs` has exactly one
`.range(...)` call — this field's. So the class here is four
restatements of ONE rule, not a family of capped fields, and it will
not grow until a second bounded field is added.

One adjacent site the grep for *"admits any"* does not reach:
`forms.rs`'s `MIN_CIRCLE_SPLIT` (`:295`) states the complementary half
— *"the kernel's own floor (`profile::Step::CircleSplit`'s `n`, which
refuses below two at replay)"*. It is a fifth sentence about the same
product-versus-kernel boundary, in the same file, and belongs with the
four when they are unified.

## Found by

The CHROME `chrome/split-circle-eps` fix pass, from a style review that
noticed the test's own doc comment restating the sentence the new
fixture comment refines.
