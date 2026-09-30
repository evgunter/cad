---
id: forms-rs-fieldwriting-doc-says-the-creation-forms-do-not-use-it
kind: issue
title: FieldWriting's doc comment says the creation forms all hand-pick their tick; add_param_ui is a creation form and derives it
status: open
opened: 2026-09-15
priority: P4
cost: E
---


## Finding

`FieldWriting`'s doc comment in `crates/viewer/src/forms.rs` states a
universal about the creation forms that one of them falsifies:

> **The two panel fields this answers for are the SLOT field
> (`ViewerBehavior::slot_value_ui`) and the DOCUMENT PARAMETER's
> (`ViewerBehavior::properties_ui`'s `Selection::Param` arm)** — the
> two a user drags to move the same kind of number. **It is not the
> creation forms' answer: those hold canonical drafts and pick their
> tick from the four constants by hand at each field.**

`ViewerBehavior::add_param_ui` (`crates/viewer/src/pane/properties.rs`)
is a creation form — it MINTS a document parameter — and it does not
pick from the four constants. It derives its tick:
`FieldWriting::of(dimension, None).tick`, naming `FIELD_DRAG_SPEED`
only as the placeholder for "no dimension picked yet", when Create is
refused anyway. Its own comment beside the call says so: *"the tick is
the canonical one for the dimension picked, and the panel's own rule
answers it rather than a constant beside it."*

So two claims in that paragraph are wrong:

1. **"The two panel fields this answers for"** — there are four callers
   of `FieldWriting::of`, all in `pane/properties.rs`
   (`git grep -n 'FieldWriting::of' -- crates/viewer/src`): the two
   named, the free-move probe, and `add_param_ui`.
2. **"It is not the creation forms' answer"** — it is exactly
   `add_param_ui`'s answer, and that site is the shipped precedent for
   the change `work/forms/drag-tick-has-three-homes.md` proposes for
   the rest of them. The comment hides the one piece of evidence that
   the proposed unification already works.

The rest of the paragraph is fine: the hand-picked call sites really do
sit in `widgets`, `pane::create` and `pane::properties`, and it cites
the tracker row correctly.

## Why it matters more than a stale sentence

`drag-tick-has-three-homes` asks whether the creation forms should
derive their tick instead of naming a constant. This comment tells a
reader evaluating that question that no creation form does it today —
which is the strongest possible argument against, and false. A lane
reading only the module docs would re-litigate a decision that has
already been made and shipped at one site.

## This is the THIRD instance of a class nobody has swept

A viewer comment asserting something the tree has falsified:

1. `work/chrome/add-profile-ui-doc-comment-states-a-premise-the-tree-falsified`
   — `add_profile_ui`'s summary says the form authors on world XY and
   that no planarity door exists; both false, and the function's own
   body comment contradicts the first.
2. `addboolean-doc-names-a-vocabulary-that-does-not-exist` — filed by
   the CHROME orchestrator. **Not in the tree at the branch point of
   `chrome/citation-repoint` (`385c01b3`)**; named here from the
   orchestrator's fix-pass adjudication and not independently read, so
   whoever takes this row should confirm its id before citing it.
3. This one.

Three is enough to stop treating them as one-offs. The class sweep that
(2) names is **still unrun**, and this row is evidence that it should
be: all three were found incidentally, by lanes doing something else,
which is the signature of a population nobody has enumerated. The
shape to sweep with is not a grep — it is reading each comment in
`crates/viewer/src/` that makes a claim about what the code elsewhere
does, and checking it. A cheaper starting filter: comments containing
"only", "never", "no ... exists", "is not the", and the doc comments of
every function the module split moved.

## Home

CHROME. One file, `crates/viewer/src/forms.rs` — but the class above is
crate-wide.
