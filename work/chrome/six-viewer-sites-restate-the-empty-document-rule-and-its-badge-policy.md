---
id: six-viewer-sites-restate-the-empty-document-rule-and-its-badge-policy
kind: issue
title: at least six viewer sites outside frame.rs restate the classification and the badge policy that frame::badge_site now decides, and the sweep that closed the originating row found two of them
status: open
opened: 2026-09-21
priority: P3
cost: E
---

## Finding

`viewer-states-the-empty-document-rule-in-four-places-and-the-one-that-
gates-cannot-red` named three restatements and they are closed:
`frame::product_badge`'s doc and its in-module test comment cite
`frame::badge_site`, and `pickindex::scene_for` cites
`ProductErrorKind::means_no_body`. **The class is not closed with
them.** The same crate states the classification, or the badge policy
`badge_site` now decides, in at least six further places.

**This row is the disclosure of a half-fix, and says so.** The first
version of it named two sites; a re-swept pattern (below) found four
more in the same crate, which is itself evidence about the first
sweep rather than about the tree.

| site | what it says, and why it is a hit |
| --- | --- |
| `crates/viewer/src/app.rs`, the badge-column comment (~`:1650`) | Restates the whole policy — *"it declines every state another channel carries: the three per-node arms are the feature tree's, and an empty document is the blank viewport's"* — three sentences after naming `frame::product_badge` as the thing that decides. **The worst of them**: it is a second, uncompiled copy of what `badge_site` answers by `match`, and it is the copy that can drift, because the gate reds on a new `ProductErrorKind` and the comment does not. |
| `crates/viewer/src/app.rs`, the landing arm (~`:1006`) | A **second** `app.rs` site, five hundred lines above the first: *"A naming collision across roots is not a node failure, so no tree badge carries it"* — the `Frame` half of the same partition, argued again. |
| `crates/viewer/src/scene.rs`, `SceneError::NoProduct`'s doc (~`:192`) | *"a failed or poisoned root, or a document denoting no body"* — **the whole partition `badge_site` decides, in prose**, on a variant whose payload is the `ProductError` itself. |
| `crates/viewer/src/scene.rs`, `SceneMesh::nothing`'s doc (~`:417`) | *"an empty recipe, or one holding only datums and profiles"* — `means_no_body`'s worked example, written independently again. Weaker than the others: the sentence is about what the scene value is the picture OF rather than a classification of a refusal. |
| `crates/viewer/src/pane/viewport.rs` (~`:993`) | *"a failed root, which the feature tree badges at the node and `product_badge` therefore declines"* — the `FeatureTree` half re-derived to justify a fixture, in a test comment. |
| `crates/viewer/src/session.rs`, `product_fault`'s doc (~`:798`) | *"the gather-level refusal no per-node badge can carry"* — the `Frame` half stated as if it were a property of the gather rather than a decision of this chrome. |

**Borderline, recorded with its disposition rather than dismissed:**
`crates/viewer/tests/landing_gathers.rs:309`, whose header asserts
*"a document that denotes no body is not a refusal"* — which IS the
classification — and then cites `frame::product_badge` rather than
`means_no_body`. It reads as a test naming its own subject, which is
why it was first called a non-hit; but the sentence is the rule, and
the citation points at a consumer instead of the home. Left for the
taker to judge; not worth a fix on its own.

**Out of this row's reach, and named so it is not re-found:**
`crates/viewer/src/session.rs` (~`:1103`) states the partition again
in the landing's comment, correctly citing `means_no_body` three
lines below. `session.rs` is AUTHOR's (`author/profile-frame`, AUTH-3)
and is also the subject of
`at-rest-badge-reports-an-empty-document-as-a-refusal`, so both
`session.rs` sites wait for that cluster.

**Two more a taker should look at**, not examined closely here:
`crates/viewer/src/tree.rs`'s row-badge docs, which are the other
side of the `FeatureTree` half; and `crates/viewer/README.md`'s
`frame` row, which is the only complete enumeration of the badge
family and the only one `frame_policy.rs`'s repaired guard reads.

## What a taker owes

Cite in each, per `docs/prompts/reviewer-style-lane.md` Q2. `app.rs`
and `viewport.rs` should name `frame::badge_site` and stop
enumerating its arms; `scene.rs` and `session.rs` should name
`ProductErrorKind::means_no_body` for which documents are in that
state and keep only what is about their own subject. No behaviour
changes. Worth doing as one diff: six sites citing one function is
the shape, and six separate diffs would each have to re-argue it.

## The sweeps, and the gap the second one exposed in the first

**First sweep** (the one that closed the originating row):
`grep -rniE "nothing to gather|state,? not a fault|only datums|last
feature|denotes no body|no body-denoting|not malformed"`. It found
two of the six.

**Its stated blind spot was wrong about itself.** It was recorded as
*"a restatement that reuses neither the classification's vocabulary
nor its two worked examples"* — a gap in coverage of OTHER wordings.
The actual miss was worse and nearer: `scene.rs:193` says *"denoting
no body"*, and the pattern carried `denotes no body` and `no
body-denoting` and matched neither. **A morphological inflection of
the classification's own key phrase**, which the pattern was built
around. A grep for a phrase is a grep for one inflection of it unless
it is written around the stem.

**Second sweep**, shaped for that:
`grep -rniE "denot[a-z]*[^.]{0,40}no body|no body[^.]{0,30}denot|
empty document|emptied document|no body-denoting|nothing to
gather|only datums|last feature|state,? not a fault|not malformed"`,
plus a second pass over the policy's own SYMBOLS rather than its
prose — `product_badge|badge_site|means_no_body|RootFailed|
RootPoisoned|per-node (state|badge)` — which is what turned up
`app.rs:1006`, `viewport.rs:993` and `session.rs:798`, none of which
uses the classification's vocabulary at all.

**What the second sweep still cannot match**: a restatement carrying
neither the vocabulary, an inflection of it, nor any of the policy's
symbols — one phrased entirely as "the blank viewport" or in terms of
`SceneMesh::nothing`. It is confined to `crates/viewer/`; prose in
`docs/`, `work/` and the `crates/*/README.md` design pages is not
reached, and `crates/viewer/README.md` is flagged above rather than
swept.

## Filed from outside the fence

Filed under `docs/prompts/implementer-discipline.md` §6 by the lane
for `chrome/empty-document-gate`, whose scope was `frame.rs`,
`pickindex.rs` and `crates/viewer/tests/`. Every site above is
claimed by CHROME jointly with VIEW (and VSEAM, VGEOM, FIT); filed on
CHROME because what the chrome says about a document is CHROME's
charter and the loudest site is a badge-column comment.
