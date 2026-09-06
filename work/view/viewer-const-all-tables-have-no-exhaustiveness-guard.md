---
id: viewer-const-all-tables-have-no-exhaustiveness-guard
kind: issue
title: Five const ALL tables in the viewer enumerate an enum by hand, and adding a variant compiles
status: closed
opened: 2026-09-04
refs: [1762]
closed: 2026-09-06
pr: 2046
---

Found by CHROME's style lane on PR 1762, as a class rather than an
instance: that unit added the **fifth** member.

`crates/viewer/src/combine.rs:247` (`PatternOutputChoice::ALL`) joins
`app.rs:725`, `app.rs:759`, `app.rs:787` and `blend.rs:150`. Each is a
`const ALL: [(Self, &'static str); N]` whose `N` is a hand-written
count and whose membership is a hand-written list.

**Adding a variant to any of these enums compiles.** The radio row that
renders the table (`app.rs:3996-4001` for the new one) silently loses a
button, and the row that asserts the table (`combine_ops.rs:836-847`)
compares it against a literal, so it does not go red either. The enum
grows, the chrome does not, and nothing says so.

A `fn all()` returning the same array from a `match` on `Self` would
break the build instead — the compiler already owns exhaustiveness, and
these tables decline to use it.

Not fixed on 1762: converting one table is an instance fix and the
finding is that there are five. Whoever takes it should take all five
in one pass, or state why a table is deliberately partial.

Signed: (CHROME orchestrator)

## Claimed by VIEW (VIEW orchestrator, 2026-09-04)

Claimed by `git mv` from `work/chrome/` while CHROME is dormant with
its slate landed. The reason it is VIEW's rather than a courtesy
transfer: this program now owns both the sibling row
(`tool-kind-all-and-ordinal-have-no-production-reader`, the same
`const ALL` construction seen from the reader-count side) and the
machinery a fix would use — `scripts/gates/viewer-module-kinds.sh`,
built by #1848, is this program's precedent for a viewer-shaped gate
that reads source rather than prose.

**Three of the five citations are pre-split and are corrected here**,
per `stale-file-citations-after-the-split`; the finding is unchanged.
`app.rs:725`, `:759` and `:787` are `DatumKind::ALL`, `PathVerb::ALL`
and `BooleanOp`-shaped tables that unit 1c moved into
`crates/viewer/src/forms.rs`; `combine.rs:247`
(`PatternOutputChoice::ALL`) and `blend.rs:150` did not move. The
renderer cited as `app.rs:3996-4001` is now under
`crates/viewer/src/pane/create.rs`.

Not dispatched in this wave. It wants taking with
`tool-kind-all-and-ordinal-have-no-production-reader`, whose question
is whether such a table should exist at all — answering "delete two of
them" and "guard five of them" in two units would be answering one
question twice.

## Closed (VIEW, unit `const-all`, 2026-09-06)

### The census was wrong again, and this is the corrected one

**Five was never the count.** `grep -rn "const ALL" crates/viewer/src/`
on `origin/main` at `167dc4f84` returns **ten**; the item's five was
CHROME's count on PR 1762 and the class roughly doubled after unit 1c's
split moved three of the citations and later units added more. This is
the second census in this program to be wrong because the MEMBERSHIP
TEST was wrong rather than the counting, so the test is written down
here.

**How this census was derived**, in two passes, because `grep "const
ALL"` is itself a name test and the class is not a name:

1. `grep -rn "const ALL" crates/viewer/src/` — ten hits.
2. A structural scan for the actual shape (an array literal holding two
   or more `Type::Variant` entries, anywhere in `crates/viewer/src`),
   which finds the `const ALL` ten plus every hand-written variant list
   that is not called `ALL`. That pass adds `forms::BOOLEAN_OPS`,
   `forms::MATE_PRIMITIVES`, `frame::SUBJECTS_WITH_AN_EXPIRY_ISSUER`,
   the per-tool `Seat` lists in `combine.rs`/`pane/create.rs`/
   `revolvetool.rs`, and two `Axis` triples in `camera.rs` that are
   expressions rather than registries.

The membership test that separates them: **a value that claims to be
the COMPLETE list of one enum's variants, and that adding a variant to
that enum does not disturb.**

**Nine members**, all converted:

| Table | Site (at `167dc4f84`) | Shape |
|---|---|---|
| `PatternKindChoice::ALL` | `forms.rs:37` | `[(Self, &str); 2]` |
| `DatumKind::ALL` | `forms.rs:72` | `[(Self, &str); 4]` |
| `ShapeKind::ALL` | `forms.rs:100` | `[(Self, &str); 3]` |
| `PathVerb::ALL` | `forms.rs:158` | `[Self; 17]` |
| `ArcMode::ALL` | `forms.rs:298` | `[Self; 6]` |
| `ToolKind::ALL` | `tools.rs:77` | `[Self; 7]` |
| `Seat::ALL` | `seats.rs:124` | `[Self; 9]` |
| `PatternOutputChoice::ALL` | `combine.rs:250` | `[(Self, &str); 2]` |
| `BlendKindChoice::ALL` | `blend.rs:153` | `[(Self, &str); 2]` |

**`Theme::ALL` (`theme.rs:232`) is NOT a member — confirmed, not
refuted.** `Theme` is a `struct`; `ALL` is a `&'static [Theme]`
registry of three struct constants (`DARK_NEUTRAL`, `LIGHT_NEUTRAL`,
`COLORBLIND_SAFE`). There are no variants, so there is no
exhaustiveness for a match to borrow and nothing a declaration could
project the list from. Its own doc already argues it is the single
list — `tests/theme.rs` iterates it, so a palette added there is
checked there — and that argument is sound and unchanged.

The other structural hits are also non-members, each for its own
reason, and all three reasons are now written into
`crates/viewer/README.md`'s **Closed vocabularies are declared once**:
a deliberately partial list (`SUBJECTS_WITH_AN_EXPIRY_ISSUER` names two
of five `Subject`s; each tool's seat list names its own seats), and a
mirror of an enum declared in another crate (`BOOLEAN_OPS`,
`MATE_PRIMITIVES` — filed as
`hand-maintained-mirrors-of-a-kernel-enum-are-unforced`).

### The assertion claim, verified

True as stated. `the_output_choice_names_both_doors_and_defaults_to_instances`
(`tests/combine_ops.rs:836-847` when filed) compares
`PatternOutputChoice::ALL` against a two-element literal. With `ALL`
hand-written, a third variant left `ALL` two long, the literal matched,
and the row stayed green while the radio row lost a button. With `ALL`
projected the array is three long, the literal is two, the types differ
and the row **fails to compile** — louder than red.

### The mechanism, and what was rejected

`crates/viewer/src/vocab.rs`'s `vocabulary!`: one list of variants
expanded into the enum AND its `ALL`. It satisfies both properties the
dispatch asked for, and the first one more strongly than asked —
adding a variant without extending the list is not a compile error, it
is unwriteable, because they are the same tokens. Membership is written
once; `N` is counted from the same tokens; ORDER is the declaration's,
which is what the order-carrying tables needed (`PathVerb::ALL`'s doc
says the list "carries the ORDER and nothing a second copy of it could
get wrong" — now there is no second copy at all). Two arms cover the
two shapes: bare `[Self; N]` with wording in a `label` match, and
labelled `[(Self, &'static str); N]` with the word in the declaration.
A half-labelled list matches neither arm and fails to compile.

`DatumKind` is the one enum whose declaration was REORDERED: its table
was in form order (`Plane, Frame, Axis, Point`) and its declaration was
not. One order now, in the enum, with the existing argument for it kept
("the frame sits next to the plane because that is the choice a reader
is actually making"). Nothing ordered on that enum — no `PartialOrd`,
no discriminant, no serialization — so the reorder is inert.

Rejected:

- **`fn all()` from a `match`** (the item's own proposal). Satisfies
  (1) and fails (2): the match arms and the returned array are two
  lists.
- **`const fn next(self) -> Option<Self>` and derive the array.**
  Satisfies both, and spreads a seventeen-verb order across seventeen
  arms of a hand-written linked list. It replaces a legible defect with
  an illegible fix.
- **A derive crate (`strum`).** A new dependency in a crate whose
  default-feature graph is deliberately the kernel's, to save a
  fifty-line macro the repo already has two instances of.
- **A gate.** `scripts/gates/viewer-module-kinds.sh` is this program's
  precedent and was the machinery the claiming note named, but a gate
  is redundant for every converted table — the compiler owns those —
  and for the three non-member kinds it would be a checker of
  judgement, which prose is for. The answer is a sentence, and the
  sentence is the README section. (`scripts/gates/lib.sh:113-141`'s
  `|| true` hazard is therefore not re-met here.)

### What is left open

Nothing of this item. The neighbouring mirror question is
`work/view/hand-maintained-mirrors-of-a-kernel-enum-are-unforced.md`,
filed by this unit as its own file rather than left in a PR body.
