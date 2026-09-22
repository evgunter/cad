---
id: leaf-frozen-column-is-schedule-dependent-under-the-drive-memo
kind: issue
title: A leaf receipt's frozen column is schedule-dependent under the drive-scoped plain memo
status: open
opened: 2026-09-14
priority: P0
cost: D
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
