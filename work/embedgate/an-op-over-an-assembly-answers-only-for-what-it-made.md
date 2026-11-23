---
id: an-op-over-an-assembly-answers-only-for-what-it-made
kind: issue
title: After an op over an assembly, cells carried unchanged between two pieces keep their meeting unread, the op reads what it made once (a touch recorded, an overlap from added material reported), and nothing but records rides the body (Ev, PR 7068, decision 3)
status: open
opened: 2026-11-18
priority: P1
cost: H
refs: [the-at-rest-gate-certifies-embedding, a-cut-distributed-over-a-seat-re-decides-the-seat-and-reports-its-split-tool, the-editors-distribution-over-a-seat-retires-for-the-kernels-whole-body-boolean, a-cut-of-a-nested-assemble-piece-reads-its-own-records-not-its-census]
---


## Ruling

Ev on PR 7068 (2026-11-18), its third decision as rewritten after Ev's
three questions: "1 is good, the rest of 2 is good, 3 is good"; on the
fillet, "yes, new interference finding at the fillet, no refusal"; on
an overlapping assembly as a cut's tool, not a rule ("the same as
cutting by one of the interfering parts and then the other"). The text
is `docs/DESIGN.md` `[topology.tiers]` (the finished-body paragraph),
`[operations.operators]` and `[topology.records]`. The PR body holds
both designers' final texts with their worked cases.

## The rule

An assembly (`⊔`) keeps its pieces separate: a touch between them is
recorded, an overlap is reported as an interference finding and never
refused. When a later op (a Boolean, a split, a shell, a blend) is
applied to the assembly:

1. **Carried, not read again.** Between two result solids that descend
   from different pieces of one operand, cells the op carried
   unchanged keep the meeting they arrived with. A recorded touch
   keeps its record, carried through the op's lineage. A reported
   overlap stays the `⊔`'s finding, reported once, where it was. No
   gate reads that pair again.
2. **What the op made is read once against the other pieces.**
   - A touch along cells the op minted or moved is the op's own
     record, citing the op. `−`, `∩`, a split and a shell cannot make
     an overlap between pieces, but can leave a bare touch where the
     tool's boundary passes through the curve where two pieces' skins
     cross: two bars crossed and overlapping at the centre, the centre
     cut out, leave four stubs touching along four edges the cut made.
   - A wall cut where the pieces already overlapped (a hole wall, a
     shell's cavity wall) is the old overlap. Nothing new is said. Old
     or new is read off the operand, which the op holds.
   - Where material the op added to one piece overlaps another (a
     fillet filling a concave corner that reaches the neighbouring
     piece), that is a new interference finding at that op, never a
     refusal. `fillet(p ⊔ q)` is `fillet(p) ⊔ q`.
3. **A solid that crosses itself never passes**: a `∪` whose added
   material fuses two overlapping pieces into one solid refuses on the
   per-solid embedding certificate.

Nothing rides the finished body for this but its contact records.

## Where the code stands (read by the designers, not run)

- `wire_subtract` sends a `⊔` operand through
  `editor_core`'s `distribute::assembled`, which sets a fresh
  `product::Census` over the cut pieces: every pair is decided again.
  That is the measured defect of
  `work/tally/a-cut-distributed-over-a-seat-re-decides-the-seat-and-reports-its-split-tool.md`
  (P1: `(bolt ⊔ plate) − pin` raises one finding to six), and
  `work/tally/the-editors-distribution-over-a-seat-retires-for-the-kernels-whole-body-boolean.md`
  retires the distribution for the kernel's whole-body op.
  `work/heldrecords/a-cut-of-a-nested-assemble-piece-reads-its-own-records-not-its-census.md`
  is the same subject. This row's rule is their "done when".
- `wire_shell`, `wire_split` and `wire_blend` take the assembly whole.
- Lineage: the Boolean's `holdings` and `Shelled::lineage` state which
  operand cells a result cell holds. Split merges its operand's solids
  before cutting and states no solid-to-piece descent; blend's
  (`blend/naming.rs`) was not read through.
- `fillet_edges` and `chamfer_edges` are generic over the finished
  body, with no one-solid refusal found.

## Waits on

`embed/records` (HELDRECORDS's first unit): the finished body holds its
records and every door carries them through its lineage. The read in
point 2 is over that lineage.

## Facts the unit settles first

- Whether each door's lineage tells a carried cell from one the door
  minted or moved, and which operand piece a result solid descends
  from.
- Whether the census tells a bare touch from an overlap at a made cell
  on curved pieces (planar it does, `rest::judge`).
- Whether a door can read "did these pieces already meet here" for a
  face that is part old, part new.

## Pins

- An overlapping assembly cut, met, split and shelled finishes with
  one interference finding, at the assembly, and no gate read between
  carried cells of different pieces. Mutant: the gate reads that pair.
- The crossed bars with the centre cut out give four edge records,
  each citing the cut.
- `(bolt ⊔ plate) − pin` reports one finding, not six.
- A `∪` fusing both overlapping pieces refuses with an embedding
  finding.
- A fillet whose added material reaches the neighbouring piece
  finishes, with a new interference finding at the fillet.
