---
id: LIB-SMALL
kind: unit
title: three small closes: refactoring maintenance crosses, Datum.in_plane dimensioned, chamfer and tube guide steps
status: closed
opened: 2026-09-08
branch: lib/small
refs: [python-split-and-inline-outcomes-drop-the-maintenance, datum-in-plane-reads-back-a-length-pair-bare, guide-has-no-chamfer-or-tube-step]
pr: 2240
closed: 2026-09-08
---


Three items, one file each, no shared seam between them.

## Delivered

**1. The refactoring doors carry their maintenance.**
`crates/pncad-py/src/py/refactor.rs`: `SplitOutcome` gains
`remainder_maintenance` and `part_maintenance`, `InlineOutcome` gains
`maintenance`, each taken straight off the kernel outcome; the three
`Doc`-minting getters (`remainder`, `part`, `doc`) hand the record
across with the document instead of `Vec::new()`. **All three sites,
not two** — the third (`InlineOutcome.doc`) is the same defect on the
same ground, not a genuinely empty record: `d::InlineOutcome` has
carried `maintenance` since EVAL-4 and the splice's edits move the
mate graph in every scene that has one.

`Doc.last_maintenance`'s doc (`py/doc.rs`) gains the paragraph that
says a document a refactoring minted reads that refactoring's own
record, and `Doc::accept`'s funnel note gains the sentence that names
the refactoring wrappers as the one family that does not pass through
it — they project a pairing the kernel made below the wrapper rather
than making one here.

The funnel test's two new doors are
`test_the_refactoring_doors_hand_back_the_maintenance_their_edits_performed`
(`tests/test_assembly_author.py`, beside the four-door row, which now
points at it). The scene is the bench stand's whole cluster cut out:
the part re-forms it (`["join", "join"]`), the remainder dissolves it
(`["split", "split"]`), and the inline back reads
`["join", "join", "drop"]`. Non-empty on all three, because an empty
record is indistinguishable from "nothing moved".

The inline half sets the new instance's frame back to the identity
before splicing: the instance inherited the cluster's gauge frame and
the part's roots are plain recipe geometry, so the kernel refuses
(`the frame is not expressible locally`) rather than dropping the
pose. That is the door's own contract, stated at the step.

**2. `Datum.in_plane` reads its origin back dimensioned.** DECIDED IN
FAVOUR OF `Length`, not of the bare shape. The field is now
`Option<((Length, Length), (f64, f64))>` (`py/value.rs`): the first
pair is a position and crosses dimensioned like `Datum.origin` and
like `Node.datum_axis_in_plane` takes it; the second stays bare
because it is a direction, which is `py/place.rs`'s rule. The argument
for the bare shape — that a frame-local coordinate is not a world
length — does not survive being written down: being written in a
frame's coordinates changes the DATUM a position is measured from,
never its dimension, and the write door had already settled it. That
sentence is now AT the field and in the stub, so the convention is
readable without the item.

`pncad.pyi`'s `Datum.in_plane` states the new shape and drops the
pointer at this tracker file. `tests/ty_fixtures/legal.py`'s
`axis_written_in_plane` annotation moves with it. No binding census
row moves: the census maps curated Rust names to Python spellings and
`Datum.in_plane` is not one of them (`UnitVec3 -> Datum.direction` is
the only Datum row and it is about `direction`).

The Python row is `TestDatumReadback` in `tests/test_document.py`:
`0.25 * m` in, a `Length` equal to it out, the direction still bare,
and the sibling `origin` asserted beside it so the class reads as one
convention rather than two.

**3. Two guide steps, before the shell step.** `docs/GUIDE.md` gains
`### Chamfering the same edges: fillet's twin` and `### Tubes: a ring
from its intent, and the same ring with a wall`, placed between the
selection step and `### Hollowing a body`, which is where LIB-G17's
spec said the shell step sits "beside" them. One self-contained
Python block each, of the kind `tests/test_guide.py` executes — 36
executed blocks before, 38 after. The
chamfer block states the twin rule (same opaque, frozen selection)
and the two differences (setback not radius, planar supports only),
meters the derived closed form, shows the chamfer taking more than the
fillet of the same size, and catches `chamfer_selection_empty`. The
tube block spells the five intent parameters, then the hollow tube's
REQUIRED wall with `minor_radius` as the outer radius, an arc window,
and the solid-minus-hollow bore differential that only holds if each
node reached its own kernel door.

## Deviations

- The brief describes `refactor.rs:582` as a third site to judge
  separately from "the two wrappers". It is `InlineOutcome.doc` — the
  inline wrapper's own site — so the three sites are two wrappers and
  the count in the brief is off by the split's two getters. All three
  are fixed on one ground.
- The purity row in the new funnel test asserts the input document's
  reading is UNCHANGED (`["join"]`, the stand's own last insert),
  not empty: a pure door leaves the input's record alone, and the
  stand's last accepted edit had performed a join.
