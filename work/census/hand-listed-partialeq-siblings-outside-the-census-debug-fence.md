---
id: hand-listed-partialeq-siblings-outside-the-census-debug-fence
kind: unit
title: six hand-listed PartialEq and Debug walks outside CENSUS-DEBUG's fence, four held by the arrival census's suppression list and two invisible to it
status: closed
opened: 2026-09-15
branch: census/hand-listed-siblings
closed: 2026-09-16
---


Filed by CENSUS-DEBUG. Its fence was *the types whose `Debug` it
touched*, and the spec directs that the rest be **checked and filed,
not swept**. These are the checked rest.

**Four of the six were suppressed by name in `KNOWN_HAND_LISTED`**
(`crates/test-utils/tests/hand_written_impl_census.rs`), keyed on
`(path, trait, self type)`, with this file as their row. **That entry
was a second finding stacked on the first, never an exemption**: a
sibling row in the same census reds if the walk ever stops finding the
hand-listed impl the entry names, so those four could not go quiet.
All four entries were deleted with their repairs (2026-09-16), in the
same diff, as that sibling row requires; `KNOWN_HAND_LISTED` holds one
entry now, `clearance.rs`'s, whose finding is live and has its own row
under **Owners** below.

**The other two — `crates/profile/src/lib.rs`'s `SketchPlane` and
`crates/editor-core/src/names/role.rs`'s `NameRef` — had no entry and
can have none** — the census cannot see either at all, for the reason
each one's own line below gives, and an entry naming an impl the walk
never reports hand-listed would red the sibling row every run. Nothing
held them but this file. **They are repaired and still invisible**: the
census's answer for both did not move when the defect was removed,
which is filed as
`work/tint/census-answers-no-field-read-for-a-walk-that-reads-a-field.md`.

## The sites, with what each one costs

**All six are repaired on `census/hand-listed-siblings`** (2026-09-16),
and each repair's added-field case was executed: a probe field added to
the declaration, the compiler's answer read, the probe removed. The
`cost` column is what the site cost BEFORE; the last column is what the
probe returned after.

| site | walk | declared | read by hand | the cost | added-field probe, after |
| --- | --- | --- | --- | --- | --- |
| `crates/editor-core/src/expr.rs` (`:519`) | `PartialEq for Lit` | `value`, `display_unit` | `value` only | documented and deliberate (D7 — display unit is presentation metadata); the omission is right and the TIE is missing. A `_` binding says the same thing and holds the next field. | E0027 at both patterns. Before: the one E0063 at the sole constructor and **nothing** at the impl — and the `size_of::<Lit>() == 16` assertion above it did not fire either, exactly as its own comment says a `[u8; 6]` in the padding would not. |
| `crates/editor-core/src/mate/coset.rs` (`:147`) | `PartialEq for Coset` | `subgroup`, `representative` | both, and the placement's four vectors through a closure | complete today; nothing holds it complete. | E0027 at both `Self` patterns; and a fourth column added to `Mat3` is an E0027 inside the closure, which before was a silent `x.linear.c2` read. |
| `crates/editor-core/src/program.rs` (`:1276`) | `PartialEq for ProfileProgram` | `plane`, `loops` | both | complete today; nothing holds it complete. | E0027 at both patterns. |
| `crates/topo/src/props.rs` (`:491`) | `Debug for SignCertificate` | `body`, `band`, `tol`, `runs`, `refused` | `runs`, plus two method calls | **rendered in braced struct shape** — so it made the same completeness claim `finish()` makes, by hand, and none of the four things it printed was a field. | E0027 at the new pattern. The braces are gone (see below). |
| `crates/profile/src/lib.rs` (`:733`) | `SketchPlane::bit_eq`, which `PartialEq` delegates to | `placement`, and under it `Affine3`'s two fields, `Mat3`'s three columns, `Vec3`'s three components | twelve coordinates, read through `origin()` and `placement.linear` | **invisible to the census**: a field read through a METHOD is a call, and no text reader can tell a getter from any other call. The delegation is fine; the hand-list one level down is the finding. | E0027 at each of the four levels, probed separately (`SketchPlane`, `Mat3`, `Vec3`). Still invisible to the census — the impl body is still one call — and the delegation site now says so. |
| `crates/editor-core/src/names/role.rs` (`:238`) | `PartialEq for NameRef`, and `Hash`, `Ord`, `Debug` and `Display` beside it | `Held`'s `name` and `stamp` | `self.0.name` only | **invisible to the census** for the sibling reason one row up. Omitting `stamp` is right and documented — it is a cache, never a decision (D9) — but a third field on `Held` landed outside all of them with no E0027 anywhere. | E0027 at all seven patterns across the five walks. Still invisible to the census, and the group now says what holds it. |

## Three of the row's own claims were wrong

Re-derived at `931736fea`; the first is the spec's correction and the
other two are this lane's.

1. **The `:704` citation** was `SketchPlane::u`; `bit_eq` is at `:733`
   (corrected below, 2026-09-16).
2. **`loop_bit_eq` is NOT "a second, deeper hand-list".** The row said
   so of `program.rs`; it is a `match` whose every struct-variant arm
   binds every field and whose fallthrough spells the variants out
   rather than using `_`. Executed: a `probe_zzz` field added to
   `LoopProgram::Circle` is an E0027 at `program.rs:1309` and `:1313`
   (re-derived at this branch's head; the first writing of this line
   carried `origin/main`'s coordinates, 1299 and 1303, into a document
   committed on the branch), inside `loop_bit_eq`, plus five more in
   the same file and **two** in `persist/wire.rs` — an E0027 and an
   E0063, where this line first said one. The compiler holds it and
   always did.
3. **`SketchPlane`'s declared column said "placement, origin".**
   `origin` is a method (`placement.translation`, transcribed), not a
   field; `SketchPlane` declares exactly one field. The hand-list there
   is over the three types UNDER it, which is why the repair binds four
   levels and not one.

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

## `SignCertificate`'s disposition, and the measurement that chose it

The spec offered two shapes and chose neither: stop it looking like a
struct, or give it a tie to the declaration that a new field breaks.
**The measurement was the correspondence between what the render names
and what the type declares.** Of the four things it printed —
`volume in […]`, `surface_area`, `open_at`, `target_refusal` — **zero**
are fields of `SignCertificate`; of its five fields, **zero** were
rendered under their own name. `Type { a: …, b: … }` is what
`derive(Debug)` and `debug_struct(…).finish()` emit and what
`finish_non_exhaustive` exists to qualify, so the braces told a reader
these ARE the fields.

That reading decides it: a new field could not make the render short,
because the render never was the field list — it only looked like one.
There was no omission for a tie to catch, so **the shape is the
defect**, and the shape is what goes. The braces are gone and the
render says it is a reading.

**Both, not one, and not by preference.** The `runs` read was a real
hand-list and the arrival census named it, so the pattern lands too —
that is the bookkeeping the census demands of any repair, and it is
what let the suppression entry go. The measurement chose the render;
the census chose the pattern.

## Closed

Repaired on `census/hand-listed-siblings` (2026-09-16). Six sites, six
repairs, six executed added-field probes (table above). No `..` rest
pattern anywhere in the diff. `KNOWN_HAND_LISTED`'s four entries
deleted in the same diff; mutation-proved both ways — re-adding one
reds `every_known_hand_listed_impl_is_still_found`, reverting one
destructure with no entry reds
`every_hand_written_walk_is_held_to_its_declaration` naming the site.

**Residue, each with its own file** (a residue disclosed in prose dies
with this directory):

- `work/tint/census-answers-no-field-read-for-a-walk-that-reads-a-field.md`
  — the two sites the census cannot see, and the fact that its answer
  for them did not move when the defect did.
- `work/census/componentwise-equality-of-the-linear-types-is-hand-listed.md`
  — the same class one level down, eleven sites in seven crates,
  executed: a fourth `Vec3` component compiles `editor-core` clean.
- `work/tint/the-per-impl-sight-anchor-is-a-suppression-list-that-shrinks.md`
  — **this unit's own cost**, and the silent-omission obligation
  firing. Deleting four `KNOWN_HAND_LISTED` entries took the census's
  only per-impl sight with them: the mutation its residue paragraph
  cited as caught (a classifier reporting only each file's first
  matching impl) reds on `origin/main` and is green here. The
  paragraph is corrected in the same diff.
