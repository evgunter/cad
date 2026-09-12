---
id: mate-primitives-is-a-partial-mirror-with-no-growth-alarm
kind: issue
title: MATE_PRIMITIVES is a deliberately partial mirror with nothing to tell it the mirrored enum grew
status: closed
opened: 2026-09-11
closed: 2026-09-12
branch: door/mate-primitives-growth-alarm
---



Filed by the `BooleanOp::ALL` unit, which closed
`hand-maintained-mirrors-of-a-kernel-enum-are-unforced` and is naming
the half of that item its fix does not reach, rather than letting it
die with the closed item (`work/README.md`: a residue disclosed only in
a `## Closed` section is invisible to the re-homing sweep).

## The finding

`crates/viewer/src/forms.rs`'s `MATE_PRIMITIVES` lists three of
`MatePrimitive`'s four variants (`crates/editor-core/src/mate.rs:155`)
**on purpose**: `Clocking` exists so the kernel can refuse it, and a
form offering it would be offering a refusal. Completeness is not what
the list claims, so the fix the closed item took — publish the owner's
`ALL` and map over it — is the WRONG fix here, and the site now says
so.

What the site still has no answer for is the other half. A primitive
the panel SHOULD offer, added to `MatePrimitive` tomorrow, would not
appear in this form and nothing anywhere would say so. A partial mirror
wants to be TOLD its enum grew; it does not want to be regenerated from
it.

## The shape an answer has

Not a projection. Something that reads the two against each other and
reds when the mirrored enum changes — a row over an exhaustive match on
`MatePrimitive` that asserts each variant is either offered here or
named as deliberately absent, so a new variant fails until someone
decides which it is. That is the "a row rather than a mechanism" answer
the closed item priced, taken for the case where the mechanism is wrong
rather than for the case where it is merely expensive.

`crates/viewer/src/frame.rs`'s `SUBJECTS_WITH_AN_EXPIRY_ISSUER` (two of
five `Subject`s) and `forms::DatumKind` (four of five `DatumSpec`
arms) are the same shape one crate closer, and whoever takes this
should say whether one instrument covers all three or whether the
partiality of each is too different to share one.

## Closed (DOOR, unit `mate-primitives-growth-alarm`, 2026-09-12)

### The premise, checked before anything was written

All three halves of it hold.

- **Four variants, three offered, `Clocking` the omission.**
  `MatePrimitive` (`crates/editor-core/src/mate.rs`, `:164`) declares
  `FrameCoincidence`, `Coaxial`, `PlanarRest { offset }` and
  `Clocking`; `MATE_PRIMITIVES` (`crates/viewer/src/forms.rs`) holds
  the first three.
- **`Clocking` exists so the kernel can refuse it.** The refusal is
  `mate::solve`'s coset match (`crates/editor-core/src/mate/solve.rs`,
  `:467`): `MatePrimitive::Clocking` returns
  `MateFault::TableLacks { what: "a standalone clocking with no
  carrying mate" }`. It is exercised typed, at
  `row7a_a_standalone_clocking_refuses_typed`
  (`crates/editor-core/tests/asm_r2a_mate_solve.rs`). Everywhere else
  the variant is represented and never accepted: `authored_lengths`
  gives it `[None]`, `eval`'s content key gives it a tag, the Python
  binding mints and names it. So the justification for the partial
  list stands as the row states it.
- **Nothing alarmed on growth.** `crates/viewer/tests/` names
  `MATE_PRIMITIVES` nowhere. The one gate that reads the site,
  `scripts/gates/viewer-vocab-declared-once.sh`, checks a different
  thing — that a hand-written list is RATIFIED in
  `crates/viewer/README.md`'s roster — and passes unchanged whatever
  `MatePrimitive` does.

### What was built

`partial_mirror!`, in `forms` beside the list it holds, invoked once.
It takes the mirrored enum, the list, and a roster that classifies
every variant as `offered` or as `absent => "reason"`, and expands to
two halves that close on each other:

- a match over the enum with one arm per roster entry — both sections
  — and no wildcard, so a new variant reds `E0004` and is named;
- one `const`-block `assert!` per OFFERED entry, saying the list holds
  that variant at that seat, so classifying a new variant as offered
  indexes a seat the old list does not have and reds `E0080` until the
  list grows. Classifying it absent asserts nothing further, which is
  the point: `Clocking` is not forced into the form.

A trailing `seat == list.len()` closes the third direction — an entry
added to the list that no roster entry classifies.

**Why not the `census!` shape verbatim** (`crates/topo/src/query.rs`,
#2449, read first). The reasoning transfers whole: a runtime row can
only iterate the list, so it can never testify to the list's own
omissions, and only an exhaustive match plus const-block asserts can.
What differs is what the roster carries. `census!` asserts a hand-held
list IS the enum — every roster entry buys a seat, and the tail assert
says the list is no longer than the roster. Here completeness is
exactly what must NOT be forced, so a roster entry buys a seat or buys
nothing, and the tail assert runs the other way: the list may not be
longer than the OFFERED half. `census!`'s two failure directions
become three, and its "the list has drifted" message becomes a
decision the author has to make rather than an edit they have to copy.

**The absent section is not a second hand-written list**, which is the
trap this shape had to clear. It is the other half of the ONE roster
the exhaustive match holds against the enum: a variant can be missing
from both sections (E0004) but cannot be missing from the roster while
present in one section of it. The reason string is required by the
macro's grammar rather than asserted over — an author cannot write an
absence without saying what makes it deliberate, and an `assert!` over
a literal in the same invocation would be a predicate over nothing a
bug could move.

### Red-first evidence

A fifth variant (`Tangent`) was added to `MatePrimitive` and the four
kernel matches it reds were given arms, so the build reached `viewer`.
Then, `cargo check -p viewer --features app`:

1. variant unclassified — `error[E0004]: non-exhaustive patterns:
   `MatePrimitive::Tangent` not covered`, at the roster match;
2. classified `offered`, list untouched — `error[E0080]: index out of
   bounds: the length is 3 but the index is 3`;
3. classified `offered` and the list grown to four — green;
4. list at four but the roster classifying it `absent` —
   `error[E0080]: evaluation panicked: the form offers a variant its
   roster does not classify`;
5. classified `absent`, list back at three — green, with `Clocking`
   and `Tangent` both named absent and neither offered.

The probe was reverted; `git diff` against the committed alarm is
clean of it. (A sixth reading, before the sequence: a roster written in
the wrong ORDER reds `E0080` at the drift message.)

### What the row got wrong, and one thing it did not say

**Nothing in the finding was wrong.** The shape it argued for is the
shape that was built, and its rejection of "publish `ALL` and map over
it" is correct for this site.

What it could not know is that `crates/viewer/src/forms.rs` is behind
`#[cfg(feature = "app")]`, which is NOT a default feature: `cargo
check -p viewer` compiles none of this file, so the first three
readings of this alarm were green over a module that was never
compiled. Any lane verifying viewer work locally needs
`--features app`; CI's viewer rows already carry it.

### The other two sites

The row asked whether one instrument covers `MATE_PRIMITIVES`,
`frame::SUBJECTS_WITH_AN_EXPIRY_ISSUER` and `forms::DatumKind`. The
answer is two-and-a-half, and it is filed on VIEW's slate as
`work/view/two-partial-mirrors-in-the-viewer-have-no-growth-alarm.md`
rather than left here: `SUBJECTS_WITH_AN_EXPIRY_ISSUER` is the same
shape and wants one more macro arm (its list is bare `[Subject; 2]`,
so the seat assert reads `list[seat]` and not `list[seat].0`), while
`DatumKind`'s partiality is between two ENUMS, where "offered" is a
counterpart variant rather than a seat. Neither is taken here — they
are VIEW's ground and this is one row.

The macro therefore stays in `forms`, private and single-caller, and
says at its own site that lifting it beside `vocab::vocabulary!` is
the move at the moment a second partial mirror takes it. Generalising
it now would be guessing at two shapes from one.
