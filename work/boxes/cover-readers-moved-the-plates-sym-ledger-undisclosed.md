---
id: cover-readers-moved-the-plates-sym-ledger-undisclosed
kind: issue
title: PR 6588's cover readers grew the plate's Plain/Decision walk by 2214 forms (frozen 36 → 62, largest form 28 → 38) with no plate decision moving at these ε; its body said the row passed
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Filed by `onecopy/plate-ledger-pin`, which re-pinned the plate.

Bisected first-parent over main: merge `6bbebd7e31` (PR 6588, `boxes/cover-app-readers`) moved `editor-core` `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned` from `Plain/Decision calls 1203 forms 51423 frozen 36` to `forms 53637 frozen 62`, and the largest form built from 28 to 38 terms. Every other walk line is byte-identical across the merge, and the plate's decisions are too (`Definite` 109 / 715, `Symbolic` 1203, no theorem disputed); the session's frozen forms rise 229 → 255 and its atoms 1338 → 1513.

The op profile (`sym_profile_plate_nominal`, before → after): `Max` +71, `Min` +10, `Sqrt` +90, `Sin` +12, `Cos` +14, `Powi` +244 (powers up to degree 110), `Add` up to 38 terms. That is a reach read off a face cover, which is the PR's subject (mate and trace reaches read topo's face cover). Reverting `geom/src/cover.rs` alone at the merge leaves the ledger moved, so the line site's re-posed arc range is not the cause; the readers are.

The PR's body says the row passes at its final head. Main at its merge fails it, so the move reached main red and undisclosed.

**The growth is corpus-wide, and some of it is past the degree budget** (measured by PR 6663's review):
- Freezes grow on every sym11 document, not on the plate alone: +26 on `two_hole_plate`, +26 on `r1_annulus`, +162 on `r2_filleted_bracket`, +362 on `r2_link`, +198 on `r2_rounded_pad`.
- 4 of the plate's 26 new freezes are `Degree | Add` nodes already past the degree budget (128), not near it.

That none of the plate's decisions moved is a fact about the plate at the ε measured, not a rule: freezing is not monotone in reach, so a form frozen past the budget can flip a verdict on another document or at another ε. The review checked the five sym11 documents across PR 6588 and found no verdict moved on any of them.

What is owed: say whether the plain walk's 26 newly frozen forms and the 2214 forms are the price of the cover reading, or whether the reach can be read without building them (the decisions do not need them, since none moved).

## Two more BOXES moves of the same line (ONECOPY, PR 6663, 2026-11-09)

After PR 6588, the plate's `Plain/Decision` walk moved twice more on 2026-11-09, both cover changes, both undisclosed. Bisected first-parent at ε 1e-9 from `337c944730`, where the ledger read `forms 53631 frozen 62`:

| merge | PR | `Plain/Decision` |
|---|---|---|
| `bc04d87117` | 6639 `boxes/nightly-reds` (the span-reach clamp, `geom/src/cover.rs`) | forms 53631 → 55116, frozen 62 → 66 |
| `296779102b` | 6631 `boxes/cover-carrier-level-piece` (a carrier's surface at a level in its cover) | forms 55116 → 55907 |

Calls (1203) and every other walk line are unchanged, and so is the largest form (38). PR 6639's description says "`m10_sym` plates (38 against 28) and slabs (green) are unchanged": it read the largest-form bound, which held, and not the forms line, which moved. PR 6631's description does not mention the plate. PR 6663 re-pins the plate at main's `forms 55907 frozen 66`.

So the cover's readings have now grown the plate's plain walk three times (6588, 6639, 6631), 51423 → 55907 forms and 36 → 66 frozen. The question this row owes is now about all three: whether that growth is the price of the cover reading, or whether the reach can be read without building forms the decisions never needed.
