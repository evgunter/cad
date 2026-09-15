---
id: hand-listed-partialeq-siblings-outside-the-census-debug-fence
kind: issue
title: five hand-listed PartialEq and Debug walks outside CENSUS-DEBUG's fence, each held only by the arrival census's suppression list
status: open
opened: 2026-09-15
---


Filed by CENSUS-DEBUG. Its fence was *the types whose `Debug` it
touched*, and the spec directs that the rest be **checked and filed,
not swept**. These are the checked rest.

Each is suppressed by name in `KNOWN_HAND_LISTED`
(`crates/test-utils/tests/hand_written_impl_census.rs`), keyed on
`(path, trait)`, with this file as its row. **That entry is a second
finding stacked on the first, never an exemption**: a sibling row in the
same census reds if the walk ever stops finding the hand-listed impl the
entry names, so the suppression cannot go quiet.

## The sites, with what each one costs

| site | walk | declared | read by hand | the cost |
| --- | --- | --- | --- | --- |
| `crates/editor-core/src/expr.rs` (`:519`) | `PartialEq for Lit` | `value`, display unit | `value` only | documented and deliberate (D7 — display unit is presentation metadata); the omission is right and the TIE is missing. A `_` binding says the same thing and holds the next field. |
| `crates/editor-core/src/mate/coset.rs` (`:147`) | `PartialEq for Coset` | `subgroup`, `representative` | both | complete today; nothing holds it complete. |
| `crates/editor-core/src/program.rs` (`:1276`) | `PartialEq for ProfileProgram` | `plane`, `loops` | both | complete today; nothing holds it complete. `loop_bit_eq` below it is a second, deeper hand-list. |
| `crates/topo/src/props.rs` (`:491`) | `Debug for SignCertificate` | `body`, `band`, `tol`, `runs`, `refused` | `runs`, plus two method calls | **renders in braced struct shape** — `SignCertificate { volume in […], surface_area …, open_at …, target_refusal … }` — so it makes the same completeness claim `finish()` makes, by hand, and none of the four things it prints is a field. |
| `crates/profile/src/lib.rs` (`:704`) | `SketchPlane::bit_eq`, which `PartialEq` delegates to | placement, origin | twelve coordinates, read through `origin()` and `placement.linear` | **invisible to the census**: a field read through a METHOD is a call, and no text reader can tell a getter from any other call. The delegation is fine; the hand-list one level down is the finding. |

## The premise this corrects

`docs/CENSUS-DEBUG-SPEC.md` ruled `props.rs`'s `SignCertificate` **out**
of the class, on the ground that it *"uses `write!"`* rather than
`debug_struct(…).finish()`. That criterion is about the TERMINATOR. The
class is about the tie to the declaration, and by that question
`SignCertificate` is squarely in: a field added to it is silently absent
from a rendering that looks exhaustive. The spec's instruction not to
fix it was followed; the criterion is corrected here.

`crates/profile/src/lib.rs`'s `SketchPlane` is the second correction,
one level further out: neither the item nor the spec reached a hand-list
hidden behind a delegation, and the census cannot reach it either. It is
listed here because the row is the only thing that can.

## Owners

`clearance.rs` → **shell**, and it is a live wrong answer rather than a
missing tie, so it has its own file:
`work/shell/geometrywitness-eq-ignores-the-two-chart-axes-its-uv-fields-are-stated-in.md`.
`mate/coset.rs` → **msolve**; `program.rs` → **edit**;
`profile/src/lib.rs` → **bool**; `expr.rs` and `topo/src/props.rs` are
claimed by no open program. Held on this slate as one item because the
class is CENSUS's and the repair is one destructure at each site; a lane
taking any of them may split its site out rather than carry the rest.
