---
id: a-placement-pins-by-written-values-without-a-mate
kind: issue
title: A placement pins by written values without a mate: stage 3 D′, the placement pivot
status: parked
opened: 2026-10-18
priority: P0
cost: H
refs: [transform-retires-into-a-placement]
design: true
blocked_on: [transform-retires-into-a-placement]
---

**Spec:** `docs/INTENT-STAGE3-SPEC.md` §5′, restated against D's head (`7bcc7f6b4c`) by symbol; where this body and §5′ differ, §5′ governs. It lists the seven interims D's head carries and what D′ makes of each, the order of work, the absorbed rows (each now `parent:` this item), and five open design questions (Q1 how the recipe names a placement's source and target space; Q2 how a value names its freedom beside a mate; Q3 what the viewer shows besides the world; Q4 what a placement helper takes from its caller; Q5 what inline makes of a moved landing), which go to designer pairs before dispatch. Parked on D (`transform-retires-into-a-placement`). E and G wait on this unit; so do `a-placement-carries-its-sources-at-rest-result` and `each-space-runs-its-census-between-arrivals` (spec §12, boundaries).

Stage 3 D′, which follows D (`transform-retires-into-a-placement`); opened by the item `a-placement-relates-two-frames` (PR 5312). D10 `[placement]` was restated by PR 5312 (fork-log row 162; Ev approved). The clauses this builds:
- `[placement.spaces]`: "Only a placement relates two spaces". A space is a construction's coordinates or the world, and spaces form a DAG.
- `[placement.placement]`: "a bundle that leaves one unset is refused at the door, so every placement is pinned and an unwritten freedom cannot be represented". Also: "what reads it downstream reads the space's bodies, never a placement".
- `[placement.mates]`: "a placement whose source touches nothing has none, and its values state the whole motion".
- `[placement.values]`: where no direction is fixed, "the rotation is a written chain of at most three spins, each about the target space's x, y or z … no two in a row about one axis; turns do not commute, so the author writes the order". The slides run along the target's (swung) axes. "a placement whose values are all zero and which has no mate is the identity".
- `[placement.world]`: "The product is the world's carried bodies". An instance places another document's world or any space of it.
- DI5: free-move repositions one shown space's picture and may write display metadata.

D lands with `mate::stand` as the interim: a Plane mate from a face no variable reaches (an Axis mate for a plane-less root) plus written zeros. That is a legal bundle under #5312. D′ is the pivot:

- **The unmated chart.** `Value` without a mate (`Value.mate` optional), with `Coord` gaining slides along the target space's axes and the author-ordered spin chain. Where a mate fixes a direction, C-1's swung chart stands. This runs through `mate/value.rs`, `mate/solve.rs`, the edit doors, persist, the wire bytes and the content key. About 300 explicit `Value`/`AddValue` sites and 303 `Coord::Slide/Spin` sites at D's head.
- **F counts spins by axis**: a repeat about one axis in a row, or a fourth spin, is overconstrained.
- **Every freedom written.** An under-pinned bundle refuses at the door, so `Space::Loose`, `Evaluation::loose_copies`/`loose_roots`, the façade's loose-copy export refusal and the viewer's free-move over loose spaces retire. Free-move goes over shown spaces (DI5).
- **`stand` becomes an unmated placement into the world.** Its values are the datum chain's slides and spins, verbatim where a step is one slide or spin about a target axis and expressions otherwise, or six zeros. Retired with it:
  - `mate::anchor::Anchors` and the variable-free-face search;
  - the Axis-mate fallback for revolves and rings (row 161 Q1 is superseded; Q2, the loft stations, stands);
  - the in-plane/out-of-plane split;
  - the instance-anchor heuristic.
  `place_at`, `pin_at` and `frame_at` are re-spelled to match.
- **Inline at a moved landing** writes the landing's motion into the values, so `InlineError::RepinRefused` and `ValueOnAMovedWorld` retire.
- **Spec** `docs/INTENT-STAGE3-SPEC.md` §1, §4 and §5 restated to match. §1 settles how a space names its bodies to its readers when one source is placed into it twice; that fork is the orchestrator's designer pair, and its ruling comes before D′ starts.
- **Re-bless**: every placement's bits and bytes move (the corpus digests, goldens, the `.pncad`/`.cad` snapshots and the tour stops).

Supersedes `a-stand-on-an-instance-reads-its-part-by-the-names-shape`, `a-value-charted-on-the-target-reads-frame-carries-the-datum-chain` and `an-inline-at-a-moved-landing-reading-a-variable-refuses`. Not started until D has merged; the orchestrator dispatches it.

Found while D's rows were restated (2026-10-18):
- **A copy at the identity places again.** The pin helpers tell a placed copy by its motion (`mate::pin::copy_of`: `value.motion.is_some()`), and a placement at the bit-exact identity has none. So a copy of an instance or body that is already pinned at the identity is accepted as a new copy in the same space, while one pinned elsewhere refuses `AlreadyPlaced`. Under #5312 placing world content into the world is a space reaching itself. D′ makes a placement's source a space and refuses that structurally. The D rows that rely on the loophole (`place_moved`, `slid_with(top, top, …)`) are in `msolve1_transform_aware`, `msolve13_read_at_operand` and `mate1_member_vocab`, and they are restated with it.
- **Deleted in D, pending §1's naming ruling**, because which copy a mate's side mints on is the twice-placed source's question:
  - `msolve1_transform_aware::a3b_a_placed_copy_of_a_pattern_seats_and_gates`: the at-rest gate reported `StaleContactDeclaration` (via `Refuted`) for copies 1 and 2 of a placed 3-copy pattern;
  - `msolve13_read_at_operand::a1b_a_placement_above_the_operand_mints_nothing`: a mate read at an unplaced pattern copy minted on a placement of that copy that stands elsewhere.
- **Absorb, if the chart is rewritten:** `sym-solve-atan2-has-no-branch-rule-about-pi` (Sym's angle readback) and `a-slide-value-doubles-its-translations-width` (the slide's width). D's box-lane solve fixes (`Chart::turned`, `mate::solve::tightened`, Sym's at-rest snap) are the base the new chart builds on.

**§1, answered by Ev on PR 5315 (a space names its bodies)**, which D′ builds:
- A body placed into a space is a **pick**: a `Body` variable of the space defined as one copy, or `Bodies` for a pattern.
- Its `of` is the path through the source's placements to the first variable it reaches, and that is its one spelling.
- Readers hold the pick; nothing else reads a placement's copies. An unpicked copy is still a body of the space.
- Names pass through from the source and are told apart by the pick. A duplicate pick is two variables.
- `RoleSeg::Placed`, `RoleSeg::InPart`, `CopyKey` and `StableName::copy_of` retire. The minting and reading sites as of D's head are listed in the orchestrator readout: `names/role.rs:1579`, `:2287`, `:2302`; `names/emit.rs:919-1000`; `eval/wire.rs:2575`, `:3546-3754`; `operand.rs:502`; `eval/parts.rs:852`; `eval/mod.rs:5585`, `:6558`; `coincide.rs:588`; `refactor.rs:2834`, `:3424`, `:4420-4470`; `edit.rs` `stranded_by_repoint`'s copy reach. The rows deleted pending this ruling, `msolve1::a3b` and `msolve13::a1b`, come back restated over picks.

D′'s carry shape must not preclude two units of their own, from PR 4666 (which awaits Ev's last look): each space runs its own census between arrivals, and a placement carries its source's at-rest decisions.

**What D now builds of this (the orchestrator's rulings 1 and 2, 2026-10-20).** D brings the mateless bundle's data model forward, so D′ starts from it rather than building it:
- `Value.mate` is optional; `Coord` has `Slide(TargetAxis)` along, and `SpinAbout(TargetAxis)` about, the target space's x, y and z (`SlideAxis` is renamed `TargetAxis`). A bundle with no mate is evaluated in the written order: the spin chain about the copy's own origin, then the slides (`mate::value::unmated_motion`).
- The door rule for a bundle with no mate (`mate::solve::unmated`, `unpinned`): every freedom written (`MateFault::Unpinned` otherwise), spins a chain of at most three with no two in a row about one axis, and a repeat refused as overconstrained. `DocEdit::AddValues` writes such a bundle whole, and `DocEdit::RemoveConstraints` removes one. Persist, the wire bytes and the content key (v16) carry it; Python (`MateValue(None, ..)`, `Coord.spin_about`, `Doc.add_values`) and the façade (`Placed::write`, `Placed::identity`) expose it.
- `mate::pin::stand` writes the identity bundle (no mate, the constant `0` on every freedom) for every copy standing where its body stands, so `stand`'s Axis-mate fallback and its variable-free-face search are gone, and the viewer's "place where authored" gesture writes `stand` (`a-stand-on-a-revolve-shares-its-written-axis-numbers` is closed with it).
- FORK-REST: only the identity bundle as written places by no map, in every lane; every other placement runs its map. The snap, `marked` and the `SolveScalar` rest hooks are gone.

**Left for D′** (unchanged by D): mixed bundles' mateless values (a lone point mate's spin chain about the mated point), point-mate chains, picks, the loose-space retirement (`Space::Loose`, `loose_copies`, the viewer's free-move over loose spaces), re-spelling `place_at`, `pin_at` and `frame_at` (they, and `place_against`, still read `mate::anchor::Anchors`, which therefore stays), inline at a moved landing, the instance-anchor heuristic, and the spec restatement. `a-turn-by-the-nearest-quarter-turn-rounds` (this item's child) carries two Interval symptoms measured on D's head, one of them a red on the nightly's 1e-12 row of `m10_p_fence`'s Interval fence.

**Left for D′ by D's exact-turn lane (2026-10-20).** A turn written as a fraction of a turn is now exact in every lane (`geom_core::Angle`, `a-turn-by-the-nearest-quarter-turn-rounds`, closed), and the corpus and tour writers that mean an axis-aligned frame state it (`corner_frame`, `moved_frame`, a bundle with no mate). The helpers still read turns off geometry by `atan2` and write them as typed numbers, so a placement they write is exact only where those numbers happen to be:
- `place_at`: the plane path's `Spin` value (`Chart::spin_at`), and the axis path's world read (`axis_in_world` → `moved_by`'s turns) and its `Spin`;
- `place_against`: the plane path's `Spin`, the tilted path's frame (`frame_at_in`), and the axis path's target frame and `Spin` (`axis_placed`);
- `frame_at`/`frame_at_in`: the picked frame moved by `moved_pose`'s turns, so even a frame meant on the body's own axes is a picked frame undone by rounded turns;
- `pin_at`: `world_at` → `moved_by`'s turns.
Callers left on them where the frame is meant exactly: `die_tool`'s master (`place_against`) and its `PlacedUnion` rule frames (`Frame::rotate_then_translate` from radians), with its byte-identical Python twin (`crates/pncad-py/tests/test_placed_union.py`); the kitchen sink's copy turned by `π/3` (a sixth of a turn, `Frame::rotate_then_translate`); the tour's `world::plane_on`/`axis_on` (frame reads), `assembly.rs`'s `lying` (`-π/2`) and every `world::against`/`in_world` call with a turned frame; `test_support::plane_on_frame` for a plane no face of its body is parallel to. A façade door that takes a frame by written turns (as `moved_frame` does for a pose) is what these want.

