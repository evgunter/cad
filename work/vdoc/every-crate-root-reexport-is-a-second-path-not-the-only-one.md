---
id: every-crate-root-reexport-is-a-second-path-not-the-only-one
kind: issue
title: The crate root re-export is a SECOND path to an item, never the only one: pub mod camera plus pub use camera::cursor_projection gives two
status: open
opened: 2026-09-06
refs: [2089]
priority: P4
cost: E
---



Found by the style review of #2089.

## The claim

PR #2089, its item's `## Closed` section and `work/view/log.md` all
say the same sentence about the `cursor_projection` move:

> that crate-root re-export moved from the `pub use marks::{…}` list
> to `pub use camera::{…}`, so there stays exactly **ONE** path to the
> function

and set it against `session-shims-and-test-imports`, whose deviation is
"two spellings of every moved path".

## It is two, before and after

`crates/viewer/src/lib.rs:55` is `pub mod camera;` and
`crates/viewer/src/lib.rs:65` is `pub mod marks;`. Both modules are
public, so `crates/viewer/src/camera.rs:908`'s `pub fn
cursor_projection` is reachable as **both**
`viewer::camera::cursor_projection` and — through
`crates/viewer/src/lib.rs:128`'s `pub use camera::{…, cursor_projection}`
— `viewer::cursor_projection`. Before the move the same two paths
existed through `marks`. What the move preserved is the COUNT, not a
uniqueness that was never there.

The module-path spelling is not hypothetical in this crate:
`crates/viewer/tests/datum_draw.rs:23` reaches `viewer::datums::{self,
DatumKind, …}` through exactly that door, and
`session-shims-and-test-imports` exists because 32 test files do it for
`session`.

## Why it is a class and not a slip

Every item in `lib.rs`'s `pub use` blocks has two public paths, because
every module it re-exports from is `pub mod`. The distinction
`session-shims-and-test-imports` actually draws is between two paths
where one is a `pub use` **shim inside a module** (a lie about where
the item lives) and two paths where the second is the crate root (a
convenience). That is a real distinction and worth stating; "exactly
one path" is not it, and a reader who believes it will conclude that a
crate-root re-export removes a spelling rather than adding one.

**Where else to look**: every claim in `work/view/` and in
`crates/viewer/README.md` that a re-export leaves one spelling, and the
`## What it takes` section of `session-shims-and-test-imports`, which
plans a sweep whose end state is described the same way.

## Confidence

`sure` that both paths resolve — it follows from `pub mod camera;`
alone. `likely` that the sentence is worth correcting in all three
places it appears rather than only the PR body, since two of the three
(the item and the log) are the durable ones.

## A second instance, in a comment rather than in a claim about a move (2026-09-15, CHROME's `chrome/citation-repoint`)

Added by CHROME per `docs/prompts/implementer-discipline.md` §6 — a
finding goes onto the slate of the program whose ground it lands on,
and this row is the class. The subject is `crates/viewer/src/app.rs`,
which CHROME has ceded to VIEW, so this is evidence and not a change.

`app.rs`, immediately above `pub use crate::forms::FieldWriting;`:

> The one re-export this module carries: `tests/panel_display.rs`
> reaches the field-writing value by the `viewer::app::FieldWriting`
> path to assert the unit/tick pair on it.

Two things are wrong with it, and they are this row's two halves:

1. **It names a test as the consumer, and there is a PRODUCTION one.**
   `crates/viewer/src/props.rs`'s module docs route a rustdoc
   intra-doc link through `crate::app::FieldWriting` — *"the panel
   divides and multiplies by it exactly as it does for a slot
   ([`shown_in`] / [`authored_in`], through
   [`crate::app::FieldWriting`])"*. That is shipped prose, not a test,
   and it is the second consumer the comment says does not exist.
2. **That link names the wrong home.** `FieldWriting` is defined in
   `crates/viewer/src/forms.rs`. The `app` path resolves, because the
   re-export is a second path exactly as this row says — so the link
   renders fine and is silently wrong about where the type lives. A
   reader following it lands in `app.rs` and finds a `pub use`.

This is the same defect one level out from `cursor_projection`: there
the claim was that a move left *one* path; here the claim is that one
re-export has *one* consumer. Both read a re-export as a redirection
when it is an addition.

**Why it is filed at all**, since a re-export is genuinely not a home:
CHROME's `chrome/citation-repoint` pass closed
`work/chrome/drag-tick-row-cites-app-rs-for-a-finding-that-lives-in-forms-rs`
and judged the re-export wrinkle to leave no open thread. That
judgement was wrong on the second half — the `props.rs` link is live,
shipped, and points at the wrong file — and the review that caught it
was right that it owed a file. The first half stands: none of the six
subjects that report tabled is DEFINED in `app.rs`.

**The sweep, with its receipt.** The shape is: an intra-doc link
spelled `crate::app::…` naming an item `app.rs` only re-exports.
`git grep -n 'crate::app::' -- crates/viewer/src` returns 17 lines, and
`git grep -n '^pub use' -- crates/viewer/src/app.rs` returns **one** —
the `FieldWriting` line above. So the population of this defect in the
crate is bounded by that single re-export, and walking the 17:

- **1 hit, the defect:** `props.rs:40`, `[`crate::app::FieldWriting`]`.
- **6 `use` statements** (`pane/create.rs`, `pane/features.rs`,
  `pane/properties.rs`, `pane/view.rs`, `pane/viewport.rs` ×2) — imports
  of `ViewerBehavior`, `chrome`, the `GLYPH_*` constants, `to_f32`,
  `indeterminate_wording`. Not doc links, and every target is defined
  in `app.rs`.
- **10 doc links to items genuinely defined in `app.rs`**:
  `ViewerApp` and its methods (`sync_scene` ×2,
  `fit_delta_on_scene` ×2, `remember_prefs`), `ViewerBehavior` and its
  methods (`viewport_ui` ×2, `add_profile_ui`), `run`, and
  `indeterminate_wording`. `ViewerBehavior::add_profile_ui` is worth
  naming as a near-miss that is NOT a hit: the method's `impl` is in
  `pane/create.rs`, but the TYPE is in `app.rs`, so the link names the
  right home.

**Blind spots.** This matches the `crate::app::` spelling only — a link
written `[`FieldWriting`](crate::app::FieldWriting)` is caught (that is
`drafts.rs:264`'s shape and it was checked), but a bare `[`FieldWriting`]`
resolved by a local `use` is not, and neither is the `viewer::app::…`
spelling from outside the crate, which is how `tests/panel_display.rs`
reaches it. It is also one crate; the same shape can exist wherever a
module re-exports and a sibling's docs link through it.
