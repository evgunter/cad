---
id: delete-the-box-scalar-sweep-leaf-switch
kind: issue
title: Delete the box-scalar sweep-leaf switch: the sweep doors finish their own result at every scalar
status: parked
opened: 2026-11-02
priority: P1
cost: M
blocked_on: [a-door-reading-the-minted-axis-loses-identities-with-quotient-minted-partners, a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided, tier-3-at-every-box-replay-leaf-multiplies-the-decisions, a-boxed-rectangles-certification-runs-out-of-memory-with-the-box-arm-off, the-aimed-faces-advance-is-zero-only-through-its-first-edges-incidence, a-walls-aim-hangs-from-its-moving-edge-so-its-ray-reads-a-box-a-tenth-as-wide, a-moving-face-whose-normal-has-no-zero-component-reads-its-side-only-over-a-narrow-box, the-gathers-finish-re-derives-certificates-its-moved-members-carry, the-two-parameter-plates-volume-refuses-on-a-junction-margin-that-is-not-a-number]
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
  `relog_held` (`crates/topo/src/validate.rs`), and F34 with its audit
  row (`docs/predicate-dimension-audit.md`);
- `topo::Replay`'s second role: it stays a dual's finished body;
- the `B` parameter on `Lofted` and `Revolved` and their `leaf`
  builders (`crates/sweep/src/loft.rs`, `crates/sweep/src/revolve/mod.rs`),
  which become the doors' finish;
- `sweep::test_support::taken`, and `crates/sweep/tests/a_boxed_motion_moves_a_finished_body.rs`'s
  naming of `topo::Replay<Interval>` as the leaf (it does not compile
  with the leaf finished);
- `LeafBody::moved`'s re-finish at f64 goes with the switch. It is
  the switch's measured price (ONEHOME stage 2,
  `crates/editor-core/tests/corpus_finish_count.rs`): the corpus runs
  662 finishes with it, 389 with the move and no f64 leaf re-finish,
  and 505 on main before a move carried its verdict. Since ONEHOME's
  PR 7088 (readers move no body) it runs 466, and since PR 7099 (a gather
  grafts each copy through its seat, the aggregate finishing once) 445,
  and since the n-ary fold and the at-rest intersection take placed
  operands (fork 288 stage 1: each member finished in its own frame,
  then moved by the door) 390;
- the `⊔` graft's `T::leaf` end, which becomes a finish that reads
  what its finished, moved members carry and derives only what the
  merge adds (`crates/editor-core/src/eval/wire.rs`;
  `the-gathers-finish-re-derives-certificates-its-moved-members-carry`:
  a plain `T::finish` there re-certifies every member from values and
  refuses the tour's chain at plain `Interval`); the twist and the mirror move
  what a value holds (`Held::moved`) and end in no gate;
- `NodeErrorKind::UnfinishedLeaf`: with no leaf, every held body is
  finished and a moved one stays finished, so nothing refuses where an
  operation takes it; the arm goes with its `input`, class, tags and
  rendered rows (`crates/editor-core/src/eval/mod.rs`, `crates/pncad-py`);
- `Held`'s leaf arm, `Held::finished` and `Held::at_f64`/`arc`
  (`crates/editor-core/src/eval/framed.rs`), and `Framed::own_leaf` and
  `in_frame_leaf`: a value holds a finished body again, and
  `Framed::read_in` moves it (`FinishedBody::moved`);
- the `MemberRead::taken_in` and `distribute::Operand::taken`/`taken_in`
  takes of a held body (`crates/editor-core/src/eval/wire.rs`,
  `wire/distribute.rs`): a held body is taken as it is;
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
their main-era assertions after the deletion, and the corpus finish
count (`corpus_finish_count`) falls to the switch-free figure, about
389.

## Slate triage (2026-11-08, ONEHOME split)

- Moved to BOXFINISH (`work/boxfinish/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows that let the box scalars finish their sweep leaves at tier 3 and so retire Ev's switch (PR 6371) went to BOXFINISH.
- 2026-11-10 — the twist, the mirror and `ReadIn` left the list: a finished body moved by an isometry stays finished (fork 256, the rigid-motion unit). (ONEHOME rigid-motion lane)
- 2026-11-14 — `blocked_on` gains `the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box`: M1 measured the slab, the derived-frame cube and the tour's stop as that row's, not the side reader's alone. The boxed-motion sweep test joins the list. (BOXFINISH orchestrator)
- 2026-11-14 — one more pin moves with the deletion: `sweep` `a_loft_computes_its_walls_in_t::a_lofted_arc_wall_is_a_function_of_the_sections_bulge` reads 22 symbolic Zeros against its pinned 11 with the switch off, because the finish runs at the door (measured by PR 6959's review, on that PR's head with the probe's switch-off edit). (BOXFINISH orchestrator)
- 2026-11-14 — the quadrature row closed (PR 6959) and leaves `blocked_on`. Two `Interval` loft walls it left are not blockers by measurement, only by inference, so the deletion's own run with the switch off is where they show: `an-iso-arc-rims-end-reads-through-its-map-at-interval` and `a-re-minted-split-rims-row-reads-its-kept-corner-through-its-map`. (BOXFINISH orchestrator)
- 2026-11-15 — the curved twin closed (PR 6979) and leaves `blocked_on`. Its rule is first exercised on a body a sweep door finishes over a box, so the doors' unit carries the row that goes red on a wrong side. (BOXFINISH orchestrator)

## What stands between the tree and the deletion (2026-11-15, M4)

Measured with the box arm off on the frames' head
(`boxfinish/probe-switch-off`, `probe/M4-REPORT.md`):

1. **An arc wall's and a rim circle's `u_ref`:** the tour certifies 0
   (94.9% with the switch on), two m10_3 rows red, 2108 refusing
   finishes.
   `a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided`.
2. **Rays on a slab boxed wider than ± 0.215 of its depth, and the
   loose volume behind them:** two red rows.
   `an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice`.
   (M6, the same day: the slab's volume gives out before its rays do,
   so `check-7s-volume-reading-is-loose-over-a-box` is not in the way
   once the rays read.)
3. **The `Interval` quadrature on an arc-rimmed wall:** one red sweep
   row (`a_seam_in_the_band_takes_its_general_image_at_interval`).
   `an-iso-arc-rims-end-reads-through-its-map-at-interval` (inferred,
   not checked against the row's fixture).
4. **What a finish costs when nothing refuses:**
   `tier-3-at-every-box-replay-leaf-multiplies-the-decisions`.
5. **A line wall that truly turns over the box** (a moving vertex): 4
   refusing finishes, no red row. Not a defect: its normal has the
   width the geometry has, and the drive bisects.

**What the deletion itself re-baselines:** m10_8's decision pins; the
derived-frame row's `frozen == 0` (192 as built); the largest-form pin
(17 terms against 4); the washer's `(501, 461)`; the far-placement
table in `sym11_far_placement_rows.rs` (with the leaf finished at the
door the session box answers first: `OutsideSessionBox` where the
table says `built`, at `Sym<f64>` too); the turned-disc union fixture
(`a_turned_discs_union_refuses_on_the_widened_copy`); and two sweep
files that name `topo::Replay` as the leaf
(`an_aimed_ray_keeps_its_line_over_a_box.rs` among them).

## What stands between the tree and the deletion (2026-11-24, M10)

Measured on main after PR 7394 with the box arm off
(`probe/M10-REPORT.md` on `boxfinish/probe-switch-off` at `7901e1b078`;
PRs 7370, 7480 and 7686 were not on main):

- **Gone since M4 and M9:** the `u_ref` length (kind A) from every
  study and the tour; the turning wall's `side_at_infinity_denom`
  (kind C); every refusing finish in `docm9`.
- **Still there:** every ray set aside on a depth-boxed slab past
  ± 0.215 (kind B; 32 finishes in `m10_3`, 2 in `m10_8`): PR 7480's
  row and what it leaves.
- **New first refusals:** check 7's volume reading on
  `props_meridian_on_surface` (60 finishes; the two-parameter and
  band-parameter plates refuse in every replay), which is
  radial-readers' row (PR 7370), now named in `blocked_on`; one row
  that runs out of memory
  (`a-boxed-rectangles-certification-runs-out-of-memory-with-the-box-arm-off`).
- **Cost:** the tour's first drive is 328 s of wall time, seven times
  M9's leaf time, because the holes now build in every leaf: the cost
  row, re-parked on the store and on the hole finish's ladder climbs.
- **No arc rim reaches the quadrature** in any study, the tour or the
  five crates' suites with the box arm off, so
  `an-iso-arc-rims-end-reads-through-its-map-at-interval` leaves
  `blocked_on` (it was there by inference) and is re-banded P3.
- **Red only with the box arm off, to be re-baselined or explained at
  the deletion (21 rows):** the three `m10_3` rows above, `m10_8`'s
  pin, three `m10_10` pins, three `m10_bulge` rows,
  `m10_9_what_the_registry_buys`, `sym_9`'s ladder receipt, `sym11`'s
  two past-the-ceiling rows, the plains ledger, `decide_3`'s
  no-predicate row, the flagged census, a lofted arc wall's row, the
  two frames rows; and new since M9, `the_plates_largest_form_is_under_the_growth_guard`
  and `sym11`'s far placement at `Sym<f64>`. The report has each by
  name.

- 2026-11-24 — the ray-readings unit closed (PR 7480) and leaves `blocked_on`; the radial readers' row left with PR 7370. In the rays' place stand the three rows PR 7480 measured and left: the aimed face's advance, the wall's aim hanging from its moving edge, and a moving face whose normal has no zero component (a tilted or rotated cap reads its side only over a narrow box). (BOXFINISH orchestrator)
- 2026-11-24 — `blocked_on` gains `a-chains-link-refuses-at-an-edges-stored-curve-with-the-box-arm-off`: M11 attributed the tour command's exit to the box arm (the one-link chain certifies with it on and refuses at an edge's stored curve with it off). (BOXFINISH orchestrator)
- 2026-11-24 — M12 (main with PRs 7370 and 7480, the box arm off): neither PR moved the tour's hole finish or its exit. The chain's link is the plain `Interval` scalar having no tier to answer an identity, a design question on its row; the two-parameter plate now refuses mostly on a junction margin that is not a number (`the-two-parameter-plates-volume-refuses-on-a-junction-margin-that-is-not-a-number`, added to `blocked_on`); and one row is red only with the arm off that was not at M10: PR 7480's `a_ray_whose_own_crossing_is_as_wide_as_the_box_reads_no_pair` (its fixture asks the door for a slab over a wide box, which the door does not finish at the box scalar: the wall's-aim and advance rows). (BOXFINISH orchestrator)
