---
id: editor-core-suites-redefine-the-name-table-helpers
kind: issue
title: Twenty-four editor-core suites still redefine the name-table and body readers fixture now has one home for
status: closed
closed: 2026-09-19
opened: 2026-09-17
pr: 2867
branch: edit/suite-helpers-one-home
---


## Finding

`crates/editor-core/tests/fixture/mod.rs` now holds one home for the
readers a name-reading suite works an evaluation with — `tol`, `table`,
`key_of`, `edge_of`, `vertex_of`, `face_of`, `count`, `point`, `ends`,
`face_vertices`, `face_edges` — and for the name-authoring shorthands
`minted`, `fname`, `ename`, `vname`, `rim_edge`, `cap_vertex`. Two
suites read them from there (`edit_ladder_rim.rs`,
`edit_ruled_carve.rs`, PR #2794). **The rest of the tree does not.**

The reviewer of PR #2794 counted the class before the move: `table`
×19, `point` ×9, `key_of` ×5, `count` ×5, `face_of` ×4, `minted` ×3,
and hand-rolled vertex names ×9. Re-measured after it, over
`crates/editor-core/tests/` less `tests/fixture/`, matching both a
`fn <name>(` definition and a `let <name> = |` closure:

- `table` — 12 files (`docm7_union_declare`, `blend5_r1_probes`,
  `m9_d1_r1_probes`, `m4_pr3_names`, `blend5_rim_support`,
  `m4_pr3_names_bool`, `blend5_r2_probes`, `wire_product_gather_tie`,
  `ring_r1_names_probe`, `m9_d1_r2_probes`, `lib_tube_r1_probes`,
  `lib_g14_split_walls`)
- `tol` — 5 files (`edit_step_segments`, `seat7_sweep_lowering`,
  `seat4_verb_lowering`, `seat8_split_lowering`, `docm9_range`)
- `count` — 4 files (`review_m5_pr9_doc_probe`, `blend5_rim_support`,
  `m5_pr10_nodes`, `m10_10_evidence_interval`)
- `point` — 2 (`switch_program_vocabulary`, `wire_rv_bytes`);
  `key_of` — 2 (`switch_program_key`, `msolve1_transform_aware`)
- a vertex name spelled out as a `StableName` literal rather than
  through a shorthand — 8 sites in 6 files (`corpus/kiss_carry`,
  `wire_entity_door`, `docm7_union_declare`, `m9_d1_r1_probes`,
  `m9_d1_r2_probes`, `m4_pr5_declare`)

Twenty-four distinct files in all. The counts differ from the
reviewer's because this pattern matches closures and definitions inside
nested modules as well; both are measurements of the same class.

## Why it matters

A copy is not merely repetition: the copies have already DIVERGED.
`edit_ladder_rim`'s `face_vertices` returned a `HashSet` and
`edit_ruled_carve`'s a `Vec` of the same walk, and every caller of
both wanted membership — so one of the two was doing work no row
asked for, and nothing said which. A reader of either file could not
tell whether the difference meant anything.

## Not swept in PR #2794, deliberately

That unit's fence is the two `edit_*` suites its rows live in. The
twenty-four files above are a different change with a different blast
radius — each copy has to be read for divergence before it is deleted,
the way these two were (byte-identical bodies measured by diff before
the move, both suites' rows green after). This row is that work.

## What a taker owes

The same discipline, file by file: diff the copy against
`fixture`'s door before deleting it, say in the PR which copies had
diverged and how, and leave a suite-local adapter where the local
spelling carries document-specific knowledge (`edit_ladder_rim`'s
`rim_edge` knows the plate's cap end and its `Rim`; it delegates rather
than duplicating). Where a door does not fit a caller, widen the door
or leave the copy and say why — a forced fit is worse than the copy.

## Territory

`crates/editor-core/tests/*` is tcost's and tint's ground; EDIT works
in it under the ladder-rim and ruled-carve units and files here beside
them.

## Ruled and spec'd (2026-09-17, EDIT orchestrator) — middle tier, branch `edit/suite-helpers-one-home`

**Ruling: the twenty-four copies go, file by file, under the row's own
discipline.** For each file: diff the copy against `fixture`'s door
before deleting it; a byte-identical body is deleted for the import;
a diverged body is either (a) a document-specific spelling that stays
as a suite-local adapter DELEGATING to the door (the `rim_edge`
precedent), (b) a door too narrow, widened once in `fixture/mod.rs`
with its current callers re-run, or (c) a genuine second meaning,
left with one sentence saying why — never a forced fit. The PR body
carries a table: file, helper, byte-identical / diverged (how) /
adapter / widened / left, so a reader can audit every deletion. The
eight `StableName` literals that spell a vertex name go through
`vname`/`fname`/`ename`.

**Constraints the tree imposes.** `tests/fixture/` is SYMLINKED into
the viewer's serde-free test binary: nothing added to it may use
`serde`, and `cargo check -p viewer --all-targets` runs before every
push (the banked lesson). A suite whose `all.rs` mount does not reach
`fixture` (say which) keeps its copy and says so. Review probe suites
(`*_r1_probes`, `*_r2_probes`) are adopted review artefacts: their
copies go too — authorship is history, not a reason to keep a
duplicate.

**Rows.** None new; every touched suite's rows green before and after
(the row count per suite in the PR body, unchanged), the `fixture`
helpers' own doc rows if it has them. Mutants: none — the change is a
deletion; the guard is the diff table and the unchanged row counts.

**Territory.** `crates/editor-core/tests/**` only (TCOST/TINT's ground,
worked by EDIT under this filing — announced). Middle tier: one opus
style review with a correctness arm (the diverged-copy table is what
it reads), then the fix pass.


## Built (2026-09-19, PR #2867)

**One home, and the rule that says so.** Every named redefinition of a
`fixture` door is gone from `crates/editor-core/tests/`: 26 bodies were
byte-identical and were deleted for the import (11 × `table`, 5 × `tol`,
`fname`, `vname`, and 8 × `fn name1`, which IS `fixture::minted` under
another name — `fixture/pr4.rs`'s own copy included); 14 had diverged
and were read before they went (`msolve5_read_below_a_root::table_of`,
`edge_key` ×3 in the blend5 suites, `edit_step_segments::point_of` and
`::face_points`, `blend5_r1_probes::arc_height`, `lib_tube_r1_probes`'s
`table` closure, `blend5_rim_support`'s `count` closure, and `fn in_copy`
×5, each of which froze `kind` to `EntityKind::Face` where the door
carries `of.kind` through); 8 local spellings that carry
document-specific knowledge stay as adapters that DELEGATE
(`corpus/kiss_carry::outer_cap_vertex`, `wire_entity_door::end_cap_vertex`,
`m9_d1_r1_probes::outer_pole`, `edit_step_segments::face_vertex_points`,
`blend5_r1_probes::arc_height`, `lib_tube_r1_probes::names`, `shelled`
in both `lib_g17` suites). Two partial applications that carried no
knowledge were inlined at their call sites (`blend5_rim_support`'s
`count`, `m9_d1_r2_probes`'s `pole`).

**The rule now has a home.** `fixture/mod.rs`'s `//!` header says what
the file holds and states the rule this row enforces: import a door,
never copy it; where the door does not fit, widen it here or write an
adapter that delegates — and an adapter never reuses a door's name,
because a door's name in a suite means the door. Four helpers that wore
a door's name with a different signature were renamed to what they are.

**One door widened**: `fixture::pole(node, ProfileVertexRef)`, the
sibling of `cap_vertex` for `RoleSeg::Pole`, which both `m9_d1`
suites' `pole` spellings are. Nothing else was widened; `fixture` is
otherwise unchanged apart from the header and that function, so the
symlinked serde-free viewer test binary gains no dependency
(`cargo check -p viewer --all-targets` green).

Row counts per suite are unchanged against `origin/main`, file by file
(33 suites with rows, listed in the PR body); `cargo nextest run -p
editor-core --test all` is 1472 passed, 5 skipped, the same count as
the merge base.

**Corrections to this row's premises.**

- **The count was both over and under.** `face_of` ×4, `key_of` ×5 and
  `point` ×9 do not exist as redefinitions; `face_of`, `edge_of`,
  `vertex_of`, `ends`, `face_vertices` and `face_edges` have no named
  copies at all. `minted` DOES — seven outside `fixture/` and one
  inside it — but under the name `name1`, which a grep for the door's
  own spelling cannot see. So the class is 24 files by this row's count
  and 31 by the final measurement, and the two sets differ:
  `msolve5_read_below_a_root`, `m5_s1_rest_declare`,
  `edit_step_segments`'s two body readers, the eight `name1` files, the
  five `in_copy` files and the two `shelled` files were not on the list,
  and several listed hits were false.
- **Nine homonyms stay**: `count` as a `Dimension::Count` literal
  (`m5_pr10_nodes`), as a body-entity tally (`review_m5_pr9_doc_probe`)
  and as a substring tally (`m10_10_evidence_interval`); `key_of` as a
  `ContentKey` reader (`switch_program_key`, `seat7_sweep_lowering`,
  `msolve1_transform_aware`); `point` as a `ProgramTarget::Point`
  constructor (`switch_program_vocabulary`, `wire_rv_bytes`); and
  `display_contract`'s functional-update vertex literal, which no
  shorthand expresses.
- **The vertex literals were nine in seven files, not eight in six** —
  the extra is that `display_contract` functional update.
  `docm7_union_declare`'s literal was deferred to
  `edit/sited-declarations` on the ground that the file is rewritten
  wholesale there; #2809 has since landed and the literal survived it
  verbatim, so the deferral was vacuous and the literal takes
  `fixture::cap_vertex` here.
- **The `all.rs` contingency is vacuous.** No suite's mount fails to
  reach `fixture`: `crates/editor-core/Cargo.toml` sets
  `autotests = false` and `tests/all.rs` is the crate's only test
  target, with `mod fixture;` at its root.

**Filed, outside this unit's fence** (both on tcost's slate, both
re-measured on the final head):
`work/tcost/inline-name-table-reads-bypass-the-fixture-door.md` — 127
inline `.name_table` reads in 50 suites, and 11 inline spellings of
`fixture::count` in 6;
`work/tcost/named-copies-of-fixture-doors-under-other-names.md` — 121
single-segment `StableName` literals in 50 files that spell an
authoring door by hand (74 Face, 25 Body, 22 Edge), 22 of them inside a
named helper whose whole body is the literal.

## Closed (2026-09-19, EDIT orchestrator)

Built and merged as PR #2867 (middle tier: one opus style review with
a correctness arm, then the union fix pass). Every NAMED redefinition
of a `fixture` reader is gone from `crates/editor-core/tests/`, each
under the row's own discipline (diff, then delete; a byte-identical
body for the import, a diverged body retired with its reason, a
document-specific spelling kept as an adapter that DELEGATES and
never wears a door's name). Audit on the final head: 26 byte-identical
deletions, 14 diverged and retired, 8 adapters, 2 inlined, one door
widened (`fixture::pole`, the two `m9_d1` suites' near-parallel pair
taken through arm (b)). The row's own counts were a name grep and
over-counted some helpers while missing others; the review found the
class that a hand-written list cannot see — `fn name1` (eight
byte-identical copies of `minted`, one inside the fixture's own tree),
`fn in_copy` (five copies, every one freezing `Face` where the door
carries the master's kind) and `shelled` — and the fix pass took them
all and replaced the list with a shape census (every `-> StableName`
helper outside `fixture/`, grouped by normalised body: none equal to a
door's). `fixture/mod.rs`'s header now states the rule; every
per-suite row count is unchanged against main; one suite sentence of
local knowledge was kept at its import. The main merge predicted as
"one line" was six files with one semantic overlap against the
sited-declarations unit, resolved by the reviewer. Residue re-scoped
onto TCOST's slate: `work/tcost/named-copies-of-fixture-doors-under-other-names`
(121 single-segment `StableName` literals spelling an authoring door
by hand) and `work/tcost/inline-name-table-reads-bypass-the-fixture-door`
(127 inline `.name_table` reads). Territory crossed by announcement:
`crates/editor-core/tests/*` (TCOST/TINT) throughout, the fixture
file included.
