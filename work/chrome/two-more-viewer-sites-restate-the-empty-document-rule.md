---
id: two-more-viewer-sites-restate-the-empty-document-rule
kind: issue
title: app.rs and scene.rs restate the empty-document classification the badge_site gate now decides, in two places the row that named three did not reach
status: open
opened: 2026-09-21
priority: P3
cost: E
---

## Finding

`viewer-states-the-empty-document-rule-in-four-places-and-the-one-that-
gates-cannot-red` named three restatements and they are now gone —
`frame::product_badge`'s doc and its in-module test comment cite
`frame::badge_site`, and `pickindex::scene_for` cites
`ProductErrorKind::means_no_body`. A sweep for the shape rather than
the symbol turns up two more sites in the same crate, both outside
that unit's fence.

**`crates/viewer/src/app.rs`, the comment above the `product_badge`
call in the badge column (~`:1650`).** It restates the whole policy —
*"it declines every state another channel carries: the three per-node
arms are the feature tree's, and an empty document is the blank
viewport's"* — three sentences after naming `frame::product_badge` as
the thing that decides. That is now a second, uncompiled copy of what
`frame::badge_site` answers by `match`, and it is the copy that can
drift: the gate reds on a new `ProductErrorKind`, the comment does
not.

**`crates/viewer/src/scene.rs`, `SceneMesh::nothing`'s doc
(~`:417`).** *"a document that denotes no geometry at all — an empty
recipe, or one holding only datums and profiles"* is the worked
example `ProductErrorKind::means_no_body`'s doc carries, written
independently a further time. Weaker than the app.rs site, because
the sentence is a statement about what the scene value is the picture
OF rather than a classification of a gather refusal — but it is the
same worked example, and the originating row's class is *"the same
classification, same reasoning, same worked example, independently
written"*.

## What a taker owes

Cite in both, per `docs/prompts/reviewer-style-lane.md` Q2: `app.rs`
should name `frame::badge_site` and stop enumerating its arms;
`scene.rs` should name `ProductErrorKind::means_no_body` for which
documents are in that state and keep only what is about the extent.
No behaviour changes.

## Sweep that found it, and its blind spot

`grep -rniE "nothing to gather|state,? not a fault|only datums|last
feature|denotes no body|no body-denoting|not malformed"` over
`crates/`, `demos/` and `tools/`. **What it cannot match**: a
restatement that reuses neither the classification's vocabulary nor
its two worked examples — one phrased entirely in terms of
`SceneMesh::nothing` or a blank viewport, say. It also does not
reach the prose in `docs/`, `work/` or the `README.md` design pages,
which were out of the originating unit's subject.

`crates/viewer/tests/landing_gathers.rs`'s A4 header was examined and
is NOT a hit: it states the row's subject and cites
`frame::product_badge` rather than re-deriving the rule.

## Filed from outside the fence

Filed under `docs/prompts/implementer-discipline.md` §6 by the lane
for `chrome/empty-document-gate`, whose scope was `frame.rs`,
`pickindex.rs` and `crates/viewer/tests/`. `app.rs` and `scene.rs` are
claimed by CHROME jointly with VIEW (and VSEAM, VGEOM); filed on
CHROME because what the chrome says about a document is CHROME's
charter and the app.rs site is a badge-column comment.
