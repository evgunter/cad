---
id: the-post-to-shelf-mate-op-is-spelled-sixteen-times-in-seven-suites
kind: issue
title: The post-top to shelf-bottom SessionOp::AddMate is written out sixteen times across seven viewer suites
status: closed
closed: 2026-09-26
opened: 2026-09-24
priority: P4
cost: E
---


## Finding

- **Where**, measured at `6db5b87f2` plus the insert-door unit's fold:
  the GUI-4 bench's seat mate, written as a literal
  `SessionOp::AddMate { a: common::head(asm::in_part(<post>,
  &bench.post_top)), b: common::head(asm::in_part(bench.shelf_i,
  &bench.shelf_bottom)), class, alignment }`, sixteen times:
  `assembly_display.rs` (4: `add_seat_mate` and three inline),
  `review_gui4_r1.rs` (4), `tree_badges.rs` (3), `msolve4_blame_rows.rs`
  (2: `add_seat`, `add_rest`), `frame_policy.rs`, `instance_authoring.rs`
  and `landing_gathers.rs` (1 each). What varies is the post, the
  `class` (`Rest` or `Tangent`) and the alignment, which already has a
  home (`asm::seat_alignment` / `asm::rest_alignment`); the op around
  it has none. Three of the sixteen are private helpers doing the
  same job under three names (`add_seat_mate`, `add_seat`, `add_rest`).
- **Also seen, not a member**: three sites (`assembly_display.rs`'s
  at-rest and instance-check rows, `landing_gathers.rs`) discard the
  op's outcome entirely. Each checks the mate landed downstream (a
  `mate_nodes(..)[0]` read, a badge the mate must produce), so it is
  not a silent pass; a door would still make it one line.
- **The home** is `common::asm`, beside `seat_alignment`: an op
  builder `seat_op(bench, post, class, alignment) -> SessionOp`, which
  a row then hands to `common::session_insert` or inspects itself. The
  rows that read the outcome's `withdrawn` half keep their inline
  `perform`; they need the op, not the insert.
- **Importance**: low. No oracle rides on the op's spelling.
- **Instrument, and its blind spot**: `git grep -n -A3
  'SessionOp::AddMate {' -- crates/viewer/tests`, filtered to the
  `post_top` / `shelf_bottom` pair. Line-window-shaped (three lines
  after the variant), so a site whose fields are reordered or built
  into a `let` above the op is missed; three `AddMate` sites that take
  a tool's proposal (`review_gui4_r2.rs`, `story_assembly.rs` twice)
  and one on a single node's faces (`gesture_table.rs`) are read and
  are not this construction. A second instrument on the required
  atom, every `&bench.shelf_bottom` under `crates/viewer/tests/`, finds
  the same sixteen plus two that are not members:
  `msolve5_read_below_a_root.rs` builds its mate as a document
  `Node::Mate` through `insert_into`, not as the op, and
  `review_gui4_r2.rs` (`oracle(&bench.shelf, &bench.shelf_bottom)`)
  reads the cap's frame for an expectation and builds no mate. Scope: `crates/viewer/tests/` only,
  because the bench is `common::asm`'s and nothing outside this binary
  can reach it.
- **Raised by**: the S-DUP lane on
  `three-doors-named-insert-mean-two-different-constructions`,
  2026-09-24, while folding `add_seat_mate` onto the session door.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` with no single ground-owner; the class is one construction
spelled sixteen times, which is this program's charter.

## Closed

Folded by the S-DUP `viewer-drain` lane, 2026-09-26, cut from
`0c1932667`.

- **Census re-taken on the atom**, every `in_part(<instance>,
  &bench.shelf_bottom)` under `crates/viewer/tests/` (`git grep -n
  shelf_bottom -- crates/viewer/tests`, read site by site): **17**
  op sites, not 16 — `frame_policy.rs` had two
  (`a_refusal_reached_through_a_mate_names_the_mate_the_tree_blames`
  as well as `a_superseded_free_move_is_news_the_ranking_shows`). The
  two non-members stand as the row read them.
- **The home**: `common::asm::seat_op(bench, post, class, alignment)`,
  and `seat_op_under` for `instance_authoring`'s row, which mates a
  shelf instance it authored itself. Every one of the 17 routes
  through it; `add_seat_mate` and `add_seat` are gone, `tree_badges`'
  three `add_mate` closures are gone, and `msolve4`'s `add_rest`
  stays as the named offender over the door.
- **Commit and pump** has one home too, `common::commit_mate` (the
  insert, the `Node::Mate` kind check, the pump), moved out of
  `mate_tool_flow`. Every site that inserted a seat mate and pumped
  takes it; the two `tree_badges` rows that author two mates into ONE
  evaluation keep `session_insert` and say why in the door's doc.
- **X4 twins folded in the same files**: the middle-of-the-shelf seat
  alignment written out longhand twice (`review_gui4_r1::seat`,
  `instance_authoring::seat_alignment`) and once as a local wrapper
  (`assembly_display::seat_alignment`) is `asm::middle_seat`; the mate
  tool's seat choice written out three times beside `asm::seat`
  (`review_gui4_r1::rest_choice`, `review_gui4_r2::seat`, an inline
  one in `assembly_walk`) routes to `asm::seat`.
- The plants and their per-row red lists are in the PR body.
