---
id: not-yet-note-separator-and-too-close-family
kind: issue
title: recourse::not_yet joins the unreadable note with ': ' and over-claims the table; the too-close-to-call sentence family parallels Indeterminate::undecided
status: review
branch: encl/not-yet-note-and-undecided-family
pr: 4443
opened: 2026-10-09
priority: P3
---


(ENCL implementer, from the review of PR 4416.)

## What

1. **`geom_brep::recourse::not_yet` uses its own separator and over-claims
   (S4).** `crates/geom-brep/src/recourse.rs` `not_yet` joins the
   unreadable-margin note with `": "` (`format!("{NOT_YET_ENDING}:
   {UNREADABLE_MARGIN_NOTE}")`), but `geom_core::lever_recourse`, the
   one spelling of a lever ending, joins a note with `"; "`. Its doc
   also says that an unreadable margin "adds what it may mean as every
   ending in this table does", but `Unsized::recourse` never adds the
   note on any arm, so that claim is false. Decide whether the not-yet
   ending takes the note the way the lever ending does, and whether
   `Unsized` should note a poisoned margin (D4 ¶1: poison gets the
   unreadable note). Then make the doc state what the table does.
2. **The too-close-to-call family is a near-parallel of
   `Indeterminate::undecided` (S5).**
   - `crates/geom-brep/src/props/mod.rs` `PropsError::Escalated`'s
     Display spells `"{} is too close to call at this tolerance: {}. {}"`
     (subject, `payload()`, ending) by hand.
   - `crates/editor-core/src/mate.rs` `MateError::Indeterminate` spells
     `"{}: a case split could not be decided — {}. Recourse: {}"` with a
     hand-written `Recourse: {}`.
   - The `topo/src/validate.rs` and `topo/src/census.rs` "too close to
     call at this tolerance" sentences are the same family.

   Either give this family one composer beside `undecided`, keeping
   one word for "undecided" and one for "too close to call" if both are
   meant, or fold it into `undecided` if the wording difference is
   accidental. The latter changes text, so the goldens move.
3. **`editor-core/src/sentence.rs` `Recourse<A>` is a second
   "one spelling" of the `Recourse: ` label** (`write!(f, "Recourse: {}",
   self.0)`), parallel to `geom_core::lever_recourse`. Decide which is
   the home.


## Disposition

- Item 1: `not_yet` joins its note through `geom_core::noted`, the one
  `{ending}; {note}` joint `lever_recourse` uses too; `topo::validate`'s
  own copy (`unnamed`, and its `NOT_YET` literal) goes through the same.
  `Unsized` takes no note: its endings already ask for the report the
  note would ("this is a kernel defect … report it", "may indicate a
  kernel bug worth reporting"). The table's docs say so.
- Item 2: no ratification of either word (`git log -S`, `docs/DESIGN.md`,
  crate README design pages), and `certify_undecided` already used "is
  undecided" as a word-budget synonym, so the in-row family folds into
  `undecided`: `PropsError::Escalated` through `Indeterminate::undecided`,
  the payload-less `validate.rs`/`census.rs` clauses through
  `geom_core::undecided!`. The rest of the class is
  `too-close-to-call-remainder`.
- `editor-core/src/mate.rs` `MateError::Indeterminate` is placement
  ground under the D10 hold; it joins this family after the hold lifts
  (listed in `too-close-to-call-remainder`).
- Item 3: `geom_core::Recourse` is the one label; `lever_recourse` and
  `editor_core::sentence` (a re-export) go through it.
