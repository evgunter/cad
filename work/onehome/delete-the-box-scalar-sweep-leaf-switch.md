---
id: delete-the-box-scalar-sweep-leaf-switch
kind: issue
title: Delete the box-scalar sweep-leaf switch: the sweep doors finish their own result at every scalar
status: parked
opened: 2026-11-02
priority: P1
cost: M
blocked_on: [the-side-reader-cannot-read-a-solids-orientation-over-a-box, tier-3-at-every-box-replay-leaf-multiplies-the-decisions, check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends]
refs: [f64-tier-3-reads-volume-uncomputable-in-band-on-hairline-revolves, 6371]
---

The final state Ev ruled on PR 6371 (2026-11-02): "keep aiming for
this final state, but … keep it disabled/selectively disabled for now".

**What goes.**

- `topo::AtRestPolicy::leaf`, the `Leaf` type, `AtRestPolicy::take`,
  `topo::take` and `topo::LeafBody` (`crates/topo/src/props.rs`,
  `crates/topo/src/finish.rs`): `T::Leaf` reads as `T::Finished`,
  `T::leaf` as `T::finish`, and a taker takes the finished body as it is;
- `AtRestBody`'s held finish record, `AtRestBody::holding` and
  `relog_held` (`crates/topo/src/validate.rs`), and F31 with its audit
  row (`docs/predicate-dimension-audit.md`);
- `topo::Replay`'s second role: it stays a dual's finished body;
- the `B` parameter on `Lofted` and `Revolved` and their `leaf`
  builders (`crates/sweep/src/loft.rs`, `crates/sweep/src/revolve/mod.rs`),
  which become the doors' finish;
- `sweep::test_support::taken`;
- the three evaluator doors' `T::leaf` ends, which become `T::finish`:
  the computing frame's twist, the mirror and the `⊔` graft
  (`crates/editor-core/src/eval/wire.rs`);
- `NodeErrorKind::UnfinishedLeaf`'s leaf half: it stays, renamed to what
  it then is, a body read through a placement's map that does not finish
  where an operation takes it (`ReadIn::Mapped`, `ReadIn::taken`), with
  its `input`, class, tags and rendered rows
  (`crates/editor-core/src/eval/mod.rs`, `crates/pncad-py`);
- `Held`'s leaf arm, `Held::finished` and `Held::at_f64`/`arc`
  (`crates/editor-core/src/eval/framed.rs`), and `Framed::own_leaf` and
  `in_frame_leaf`: a value holds a finished body again, and `ReadIn::Held`
  holds that body. `Framed::read_in` and `ReadIn` stay: a body mapped
  into another frame is construction state with or without the switch;
- the `MemberRead::taken_in` and `distribute::Operand::taken`/`taken_in`
  takes of a held body (`crates/editor-core/src/eval/wire.rs`,
  `wire/distribute.rs`): a held body is taken as it is, a mapped one
  still finishes there;
- the connectedness check's take of a held body
  (`crates/editor-core/src/checks.rs`);
- the bindings' `HostBody` held arm, which holds the finished body
  (`crates/pncad-py/src/py/value.rs`);
- `topo::test_support::Finishing for Replay` (`crates/topo/src/lib.rs`)
  and the tour's `Operand` impl for `Replay` and its `Scalar` bound on
  `Leaf` (`demos/tour/src/booleans.rs`, `demos/tour/src/scalar.rs`);
- the drive's "what a certified leaf has not had" paragraph
  (`crates/editor-core/src/drive.rs`) and `docs/DESIGN.md`'s exception;
- the rows `onehome_switch_parity_interval.rs` pins, which hold with
  the finish at every scalar too.

**What waits.** It is parked on the three rows that make the box
scalars' tier 3 read what the box studies build. The `f64` row
(`f64-tier-3-reads-volume-uncomputable-in-band-on-hairline-revolves`) is
not a blocker: `f64` is not behind the switch.

**The test that says it is done.** The rows each blocker names pass with
their main-era assertions after the deletion.
