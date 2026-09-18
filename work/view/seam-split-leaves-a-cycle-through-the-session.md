---
id: seam-split-leaves-a-cycle-through-the-session
kind: issue
title: the seams' chain still closes into a ring through session, and the README's new section does not say so
status: closed
opened: 2026-09-06
refs: [2079]
closed: 2026-09-06
pr: 2079
---



Found by the style review of #2079.

## What

`crates/viewer/README.md:620-651` adds *"The seams' modules are a
chain, not a ring"* and draws

    generation  ←  pickindex  ←  evalseam  ←  pick

with the sentence *"So the modules are a chain, each naming only what
is below it"*. That is true of those four modules and of nothing wider.
The crate's module graph still holds a ring that runs straight through
two of them:

- `crates/viewer/src/pick.rs:44` — `use crate::pickindex::{PickIndex, PickIndexError}`;
- `crates/viewer/src/pickindex.rs:64` — `use crate::session::{EdgeSelection, FaceSelection, Hovered, Selection, SessionOp}`;
- `crates/viewer/src/session.rs:78` — `use crate::pick`, for `pick::IndexInputs` at `session.rs:1109`.

`pick → pickindex → session → pick`. It is not new — at the merge base
the same ring was two modules long (`pick.rs:67` named `session`,
`session.rs:77` named `pick`) — and it is held open on purpose, by the
`IndexInputs` hoist the README argues for at its *What a vocabulary
reads, it is handed* section. So this is not a regression and not a
rule violation.

## Why it is a finding anyway

The new section's own opening is *"a cycle here breaks no clause"*,
and its answer is a picture of a chain. A reader who takes the picture
for the crate's shape will believe the viewer's vocabularies are
acyclic, which they are not, and the one surviving ring is of a
different kind from the one that was broken: `evalseam ↔ pick` was one
file holding two layers, `pick ↔ session` is a vocabulary and its
driver trading a minted value. Those are different diagnoses and the
section offers only the first as general.

The durable version of the section would name the ring it does not
break and say why that one is allowed — the argument already exists,
one section further down the same file.

## Confidence

`sure` on the three import lines and on the ring being pre-existing.
`likely` that the section as written misleads.

## Closed

`crates/viewer/README.md`'s section is renamed **The seam modules are a
chain; the crate is not acyclic** and now carries the three import
lines of the surviving ring, the fact that it predates this split (two
modules long at the merge base), and the reason it is held open — the
`IndexInputs` hoist, argued one section further down the same file. It
also separates the two diagnoses the old text conflated: `evalseam ↔
pick` was one file holding two layers, `pick ↔ session` is a vocabulary
and its driver trading a minted value, and nothing in the section
generalises from the first to the second. `pick.rs`'s own header points
at the section rather than restating it.

Whether the surviving ring should be broken at all is not opened here
and is not this item's question — it records that the ring is
deliberate.

**Two of the three line numbers in `## What` above shifted inside the
same fix pass that closed this**, because rewriting the two module
headers moved every import below them: `pick.rs:44` is `:60` and
`pickindex.rs:64` is `:99`; `session.rs:78` is unmoved. The README
carries the current three. The numbers above are left as the reviewer
recorded them — that is what a closed row is for — and the shift is
noted here because this program spent the same day learning that a
resolving citation reads as a checked one
(`stale-file-citations-after-the-split`). This is that item's class
biting inside the PR that corrected it.
