---
id: operands-convert-to-array-indices-by-hand
kind: issue
title: An operand and its array index convert by hand at about 25 sites in crates/topo/src, with no one conversion
status: closed
opened: 2026-11-01
priority: P4
cost: E
closed: 2026-11-23
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

Moved from TOUCHREC to HELDRECORDS on 2026-11-20, by `git mv`, when TOUCHREC merged into HELDRECORDS: TOUCHREC held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one HELDRECORDS carries.

## Closed

2026-11-23, by HELDRECORDS' `nary-lineage` PR (7516), on its review.
`Operand` now names the convention once (`crates/topo/src/boolean/reduce.rs`):
`Operand::BOTH`, the operands A then B in the order every per-operand
pair holds them (so `Operand::BOTH[i]` is the operand at place `i`),
and `Operand::index`, its inverse. Every site reads through them.

Pass 1, the same shapes as above, re-run on the merge with main
(`usize::from(<x> == Operand::B)`, `[Operand::A, Operand::B][…]`,
`[Operand::A, Operand::B].into_iter().enumerate()`,
`if <i> == 0 { Operand::A } else { Operand::B }`, and the bare
`[Operand::A, Operand::B]` pair), each now `.index()` or `Operand::BOTH`:

- `boolean/ops.rs`: 27 lines, including the private `side` helper
  (removed, its five callers read `.index()`), `fallback_fates` `gone`
  (a `match` over `discarded[0]`/`[1]`), and the carried-record lookup in
  the area-record pass, which spelt a `u32` input as an operand by
  `== 0`: an input past 1 now reaches an `unreachable!` instead of
  reading as B.
- `boolean/mod.rs` (12), `boolean/holdings.rs` (5), `boolean/insert.rs`,
  `boolean/join.rs`, `boolean/recl.rs`, `boolean/nary.rs` (2 each),
  `boolean/join_carrier_site_rows.rs`, `boolean/sectors.rs` (1 each).
- Beyond `crates/topo/src`: `crates/editor-core/src/names/emit_topo.rs`
  (5), and the tests `carried_citations_past_index_zero.rs`,
  `crosslap_rest.rs`, `m3_pr4_boolean.rs`.

Pass 2, at pass 1's blind spot (indexing by the *other* operand, and
`match` arms naming a literal index): `boolean/vtxfac.rs`
(`usize::from(operand == Operand::A)`) and `boolean/node.rs` `datum`
(`Operand::A => &walls[1]`) now read `.other().index()`.

Not converted: per-operand state held as two named fields
(`red.a`/`red.b`, `contacts.a_on_b`/`b_on_a`) and chosen by a `match`.
That is a choice of field, not an index, as the row already said. Also
not converted: a `MemberFace`'s first field where it is a member's place
in an n-ary list rather than an operand. In `Holdings` (an operand's
place) it now reads `Operand::B.index()`.

Pass 3, on the delta's review: the `u32` input a row's
`Backing::Carried` cites now has its own pair, `Operand::input` and its
inverse `Operand::of_input`. Every literal reading goes through them:

- `boolean/ops.rs`: the two `u32::from(side == Operand::B)`, the
  carried-record lookup, and the stale vertex-vertex refusal's
  `if input == 0`. An input past 1 now reaches an `unreachable!` where it
  used to read as B.
- `boolean/material.rs`: the `input: 0` / `input: 1` patterns in the
  read-back of the material's records and in the cut's re-citation. An
  input past 1 is unread there, as it was.
- `boolean/records.rs`: a test.
- `crates/editor-core/src/eval/wire.rs`: the fold's `input: 0` arm.
- `boolean/sectors.rs` `pair_sectors`: `operand as usize`, a discriminant
  cast, is now `.index()`.

Sweep: `input: (0|1)`, `input [!=]= (0|1)` and `as usize]` in
`crates/{topo,editor-core,sweep}/src`, with no hits left. Blind spot: a
literal `0`/`1` that stands for an operand under a name other than
`input`.
