---
id: operands-convert-to-array-indices-by-hand
kind: issue
title: An operand and its array index convert by hand at about 25 sites in crates/topo/src, with no one conversion
status: open
opened: 2026-11-01
priority: P4
cost: E
---


Found by the review of PR 6210 (`touchrec/kept-side-reads-the-glue`).

## What

Per-operand state in the boolean is held as `[_; 2]` arrays indexed A
then B, and each site converts between an `Operand` and its index on
its own: `usize::from(x == Operand::B)` one way, `[Operand::A,
Operand::B][s]` or `[Operand::A, Operand::B].into_iter().enumerate()`
the other, and a few `match` arms. Nothing on `Operand` names the
conversion, so the A-is-slot-0 convention is restated at every site.

Pass 1, `usize::from(<x> == Operand::B)` and `[Operand::A, Operand::B][`
in `crates/topo/src` (25 hits, line numbers as of PR 6210's branch):

- `boolean/join.rs` `slit_tips` lookup (:5212)
- `boolean/ops.rs`: :807, `boolean_op_recut` `discarded` (:1111),
  null-copy rows (:3508), `Descendants` `absorbed` / `copies` / `along`
  (:3543, :3553, :3670), carried ties (:3854), copy roots (:4039),
  `ledger.incidence` (:4395), `desc.fragments` (:4470), `view` (:5134),
  `at` (:5355), `made` (:7702)
- `boolean/mod.rs`: split side (:784, :796), `divisions` (:817),
  `in_faces` (:822, :827), `vertices` (:847), `tips` (:5985, :6023),
  `copies` (:6635)
- `boolean/insert.rs` `operand: [Operand::A, Operand::B][slot]` (:1288)
- `boolean/recl.rs` `tips` (:801)

Pass 2, aimed at the spellings pass 1 cannot match (`match` arms indexing
`[0]`/`[1]`, `enumerate` over the operand pair):

- `boolean/ops.rs` `fallback_fates` `gone` (`Operand::A =>
  discarded[0]`, :8232)
- `[Operand::A, Operand::B].into_iter().enumerate()`: `boolean/ops.rs`
  :4259, :4562; `boolean/insert.rs` :902; `boolean/ops.rs` `sides`
  (:5133); `boolean/mod.rs` `copies` (:6614)

Not searched: per-operand pairs held as two named fields (`red.a` /
`red.b`) chosen by a `match`, which is the body choice, not an index.

## Owed

One conversion on `Operand` (an index and its inverse, or an
operand-indexed pair type) that every site reads.
