---
id: hand-listed-partialeq-siblings-outside-the-census-debug-fence
kind: unit
title: six hand-listed PartialEq and Debug walks outside CENSUS-DEBUG's fence, four held by the arrival census's suppression list and two invisible to it
status: spec
opened: 2026-09-15
branch: census/hand-listed-siblings
---


Filed by CENSUS-DEBUG. Its fence was *the types whose `Debug` it
touched*, and the spec directs that the rest be **checked and filed,
not swept**. These are the checked rest.

**Four of the six are suppressed by name in `KNOWN_HAND_LISTED`**
(`crates/test-utils/tests/hand_written_impl_census.rs`), keyed on
`(path, trait, self type)`, with this file as their row. **That entry is
a second finding stacked on the first, never an exemption**: a sibling
row in the same census reds if the walk ever stops finding the
hand-listed impl the entry names, so those four cannot go quiet.

**The other two — `crates/profile/src/lib.rs`'s `SketchPlane` and
`crates/editor-core/src/names/role.rs`'s `NameRef` — have no entry and
can have none** — the census cannot see it at all, for the reason
each one's own line below gives, and an entry naming an impl the walk
never reports hand-listed would red the sibling row every run. Nothing
holds them but this file, which is why they are here. (The suppression list's
fifth member is `clearance.rs`, whose finding is a live one with its own
row under **Owners** below; the two sets of five are not the same
five.)

## The sites, with what each one costs

| site | walk | declared | read by hand | the cost |
| --- | --- | --- | --- | --- |
| `crates/editor-core/src/expr.rs` (`:519`) | `PartialEq for Lit` | `value`, display unit | `value` only | documented and deliberate (D7 — display unit is presentation metadata); the omission is right and the TIE is missing. A `_` binding says the same thing and holds the next field. |
| `crates/editor-core/src/mate/coset.rs` (`:147`) | `PartialEq for Coset` | `subgroup`, `representative` | both | complete today; nothing holds it complete. |
| `crates/editor-core/src/program.rs` (`:1276`) | `PartialEq for ProfileProgram` | `plane`, `loops` | both | complete today; nothing holds it complete. `loop_bit_eq` below it is a second, deeper hand-list. |
| `crates/topo/src/props.rs` (`:491`) | `Debug for SignCertificate` | `body`, `band`, `tol`, `runs`, `refused` | `runs`, plus two method calls | **renders in braced struct shape** — `SignCertificate { volume in […], surface_area …, open_at …, target_refusal … }` — so it makes the same completeness claim `finish()` makes, by hand, and none of the four things it prints is a field. |
| `crates/profile/src/lib.rs` (`:733`) | `SketchPlane::bit_eq`, which `PartialEq` delegates to | placement, origin | twelve coordinates, read through `origin()` and `placement.linear` | **invisible to the census**: a field read through a METHOD is a call, and no text reader can tell a getter from any other call. The delegation is fine; the hand-list one level down is the finding. |
| `crates/editor-core/src/names/role.rs` (`:197`) | `PartialEq for NameRef`, and `Hash`, `Ord` and `Debug` beside it | `Held`'s `name` and `stamp` | `self.0.name` only | **invisible to the census** for the sibling reason one row up: `self.0` is a tuple index the reader answers, and `.name` is a field of the INNER type, whose declaration no text walk reaches. Omitting `stamp` is right and documented — it is a cache, never a decision (D9) — but a third field on `Held` lands outside all four walks with no E0027 anywhere. Found 2026-09-15 by the style review of the census's own blind-spot list. |

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

**Citation corrected 2026-09-16**: this row was filed with `:704`,
which is `SketchPlane::u`. `bit_eq` is at `:733`. Line citations rot on
every edit above them, which is why the census beside this row keys on
`(path, trait, self type)` and says so at `KNOWN_HAND_LISTED`.

## Owners

`clearance.rs` → **shell**, and it has its own file because the two
fields it leaves outside equality are the chart axes its uv pairs are
stated in:
`work/shell/geometrywitness-eq-ignores-the-two-chart-axes-its-uv-fields-are-stated-in.md`.
`mate/coset.rs` → **msolve**; `program.rs` → **edit**;
`profile/src/lib.rs` → **bool**; `names/role.rs` → **edit**; `expr.rs`
and `topo/src/props.rs` are claimed by no open program. Held on this slate as one item because the
class is CENSUS's and the repair is one destructure at each site; a lane
taking any of them may split its site out rather than carry the rest.
