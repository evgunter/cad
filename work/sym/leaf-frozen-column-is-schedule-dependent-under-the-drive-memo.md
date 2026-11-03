---
id: leaf-frozen-column-is-schedule-dependent-under-the-drive-memo
kind: issue
title: A leaf receipt's frozen column is schedule-dependent under the drive-scoped plain memo
status: closed
opened: 2026-09-14
priority: P0
cost: D
closed: 2026-09-22
---



Filed by SYM-7 as it landed the drive-scoped plain memo, as a residue
of the design Ev ratified on `[ev]` #2581 — not a defect in it.

## What

`CertifiedLeaf::decisions` and `RefusedLeaf::decisions` carry
`SymCounts::frozen` for that leaf. With the drive's plain memo on
(`DriveConfig::plain_memo`, `geom_core::sym::DriveMemo`) a leaf that
finds a node's plain form in the memo never freezes it itself, so the
column records **which leaf got to a node first** — and under the
parallel schedule that is a property of the schedule.

Measured on the plate at 48 leaves (`m10_sym_drive_memo_interval`'s
differential row prints it): the leaves' own `frozen` sums to 50,112
with the dial off and 0 with it on, and the drive's own column is
1,044 either way.

## The latent dependence a review found (2026-09-15)

`CertifiedLeaf` and `RefusedLeaf` derive `PartialEq`, and that
derivation covers `decisions` — so it covers the per-leaf `frozen`
column. Rows that compare whole leaf lists across schedules therefore
compare it too, and
`m10_3_r2_probes_interval::my_own_drive_is_bit_identical_across_repeats_and_schedules`
is one of them (both reviewers of SYM-7 found this; the census in that
PR's body under-reported it).

It is green today for a reason that is not a guarantee: the level-0 root
box publishes the whole DAG before any level splits, so every later leaf
finds every form in the memo and freezes NOTHING — both reviewers
measured the leaves' own `frozen` at 0 with the memo on, on both
documents. A document whose DAG grows between levels, or a drive whose
first level is wider than one box, would let two leaves race for a node
and make that column differ between schedules. The row would then red,
and correctly.

So the options below are not only tidying: the first two make that row's
comparison well-founded instead of accidentally satisfied.

The DRIVE's column is sound and that is the one the receipt
serializes: `ParamBoxVerdict::serialize` writes the distinct nodes
frozen over the drive (`DriveMemo::frozen`), a set, so
`m10_3_r1_probes_interval`'s receipt-identity row stays byte-identical
across one sequential and two parallel drives. Nothing in the tree
reads or compares the per-leaf column, and `SymCounts::frozen`'s own
docs now say it is a per-leaf WORK measure rather than a receipt
column. So this is disclosed, not urgent.

## Why it is still a row

A leaf receipt is a receipt. Every other column on it —
`symbolic_zero`, `sign_gated`, `registered`, `numeric` and the two
refusal columns — is a claim about that leaf's own predicates and is
the same under every schedule; this one now is not, and a consumer
that reads a leaf's receipt cannot tell the difference by looking.
The options, none of them taken here because each is a design
decision rather than an implementation one:

- **Drop it from the leaf receipt** and keep `frozen` only on the
  drive's, where it means something schedule-independent.
- **Make it the leaf's NEED rather than its work**: the distinct nodes
  in the drive's frozen set that this leaf's walks reached. That is
  schedule-independent and it is what a reader probably expects, but
  it costs a per-leaf set intersection and a second traversal.
- **Leave it and say so**, which is what SYM-7 did, on the ground that
  nothing reads it.

## Territory

`crates/geom-core/src/sym.rs` (`SymCounts`), `crates/geom-core/src/sym/memo.rs`,
`crates/editor-core/src/drive.rs` (`CertifiedLeaf`, `RefusedLeaf`) — the
last is PROPS' file by territory, so a fix that moves the leaf receipt's
shape is a seam to announce.

## ANSWERED by SYM-13

**The readings above are as filed and two of them have since moved**:
the leaves' own `frozen` is no longer 0 with the dial on (it is 50,112
on the plate at 48 leaves, the same number the dial-off lane reads),
and `SymCounts::frozen` no longer says it is a per-leaf WORK measure.

**The race was built.** Of the two shapes named above, "a drive whose
first level is wider than one box" is unreachable — `drive`'s frontier
starts as exactly one box on every document — and the other one is: a
drive over the M10-3 slab with the symbolic budget cut through the
public `DriveConfig::symbolic` reaches nodes no earlier level
published. On it, before the fix, the leaves' own column read
`0:1613 6:369 7:1061` sequentially and `0:1613 1:369 6:369 7:1061` on
four rayon workers, with every other column, the drive's `frozen` and
the whole serialization identical — so
`m10_3_r2_probes_interval::my_own_drive_is_bit_identical_across_repeats_and_schedules`'s
own comparison was false on it.

**The option taken was the second one: the leaf's NEED.** A leaf's
`frozen` is now the frozen nodes its own reasoning rested on — inside
the closure of the leaf's plain-walk roots: the drive's frozen set,
the ids the leaf's own table does not hold, and the freezes it made
and could not publish. Both
sides are functions of the leaf's box and of the drive, so the column
is the same under every schedule and with the memo dial either way,
but for the one reading named below;
`geom_core::sym::memo`'s header carries the argument, and the gating
row is
`m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials`
over three adversary drives, one of which certifies leaves.

**What it left open.** The unrecorded branch, which is the memo's own
pre-existing disclosure: a leaf that did not RECORD a node can inherit
a recorded ancestor's form instead of freezing. SYM-13's first
reviewer showed by execution that the column could move there **with
every decision column standing still** — so the first draft's sentence
"it is the decisions that move there first" was false — and the unit
answered by counting the leaf's own side from its TABLE
(`Session::foreign`: the ids its DAG names and its table does not
hold) rather than from the freezes its walk happened to make, which a
memo hit can take away.

What remains after that is one reading, filed as
`work/sym/a-taint-induced-freeze-under-a-hit-still-reads-by-order`
(P2): a freeze the TAINT caused — over the budget only because an
unrecorded node stood in for a real form — is in no drive's set and
not in the leaf's table either, so it is counted where the walk made
it. `geom-core`'s
`sym_drive_memo::a_taint_induced_freeze_under_a_hit_is_read_by_order`
pins that reading, and
`no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded` counts
`FreezeCause::Unrecorded` over five drives (both measured documents and
the three adversaries) and pins the branch at zero on all of them: a
drive mints every node inside its own session, so no leaf of one holds
a foreign id at all.

Closed at SYM-13's merge (#3054, 2026-09-22).
