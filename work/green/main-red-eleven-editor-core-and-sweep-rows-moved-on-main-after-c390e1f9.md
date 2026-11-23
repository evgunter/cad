---
id: main-red-eleven-editor-core-and-sweep-rows-moved-on-main-after-c390e1f9
kind: issue
title: "Main red: eleven editor-core and sweep default-set rows moved on main after c390e1f9 (cut_cylinder's and boss_union's digests, the m10 fence, the at-rest census, the boss-union parting, two sweep band poses)"
status: open
opened: 2026-11-22
priority: P0
cost: E
---

Found by BOXFINISH's doors-read lane (PR 7384) on 2026-11-22, re-merging
`origin/main` at `69f31a4e`. Each row below is red on `69f31a4e` itself
(a clean checkout, default filter, default ε,
`CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=1`) with the payload PR 7384's
merged head gives, digest for digest. Each passed on `c390e1f9` in that
lane's slow-set runs at the default ε (and the `editor-core` ones at
1e-6 and 1e-12), so a first-parent merge in `c390e1f9..69f31a4e` (about
forty) moved them. Not bisected. No test file among them is touched in
the range.

`editor-core`:

- `seat7_sweep_lowering::the_sweep_documents_evaluate_to_their_committed_digests`:
  `cut_cylinder: 0xed031d8ebcfb2ed4 (want 0xe5a66d830fff478c)`,
  `boss_union: 0x2ea09f26ff2947dd (want 0x92426dc9578fe111)`.
- `seat8_split_lowering::the_split_documents_evaluate_to_their_committed_digests`:
  `cut_cylinder: 0xed031d8ebcfb2ed4 (want 0xe5a66d830fff478c)`.
- `intent_s2_c_world::the_migrated_world_is_the_pre_c_product`:
  `cut_cylinder: the product moved`, `"cb8ccadf87044e7f/28"` against `"d2acb2b37c5273db/28"`.
- `m10_p_fence::{the_corpus_evaluation_is_bit_identical_at_f64, the_corpus_evaluation_is_bit_identical_at_interval, the_corpus_geometry_is_bit_identical_with_ids_masked}`:
  at `f64`, `(3988826103179017448, 5424432721422482676)` against `(10633404565636908076, 2899951918389042232)`.
- `edit_placement_corpus_bits::every_corpus_placement_places_its_body_by_the_pinned_bits`:
  `a corpus placement's placed body moved`.
- `perf12_census_goldens::the_at_rest_census_verdicts_are_goldened_bit_exact`:
  `drifted from their committed golden at line 95 (eps=1e-9)`.
- `revalue_corpus::revalue_boss_union_apart_to_its_parting`
  (`crates/editor-core/tests/revalue_corpus.rs:520`): `(2390, 3856)`
  against the pinned `(2360, 3857)`; it parts at
  `(2390, 7417, "pcurve_cylinder_chart_incident")`, `moved: 1474`,
  `laundered` empty.

`sweep`:

- `bisector_nodes::the_near_node_poses_at_the_witness_band`:
  `A's cap δ = 1e-8, direct (B, A), ∪: the rim's placement in band, got Err(Euler(RebasedCarrier { … Escalated { check: TransversalityArm, sample: 1, … }`.
- `section_end_poses::a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds`:
  `SquareBox t1e-9 ab U: got Some(Escalated { decision: Coincidence(Section, Moot), … margin … 1.0000000272292198e-9 …`.

The two documents moved, `cut_cylinder` and `boss_union`, are the ones
the first four rows share; one merge moving both is the first guess.
Rows on main that pin these tests hold other moves
(`ciw/a-diff-that-moves-corpus-ids-runs-none-of-the-slow-set-id-pins`,
`fixture/one-digest-of-one-document-is-pinned-in-two-suites-and-re-blessed-in-one`,
`onecopy/boss-unions-body-moved-at-6710-and-is-pinned-unjudged`,
`onecopy/main-red-the-perf12-at-rest-census-golden-moved-with-the-backstop-abort`,
`inlane/under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move`);
none names this range.

**Owed:** bisect the range on the first parent; re-pin with the moving
PR's reason, or fix what it moved.

## Bisected (2026-11-22, BOXFINISH's measurement lane; every step a clean checkout)

Two first-parent merges moved the eleven rows between them; none is
left unexplained. Steps run: `c390e1f9` 11 pass; `2ecec12e84` and
`abe288332a` 11 pass; `4a94f7109b` 5 red; `f7dd37696d` the same 5,
same payloads; `54e6a5c5b8` 11 red; `6e524e45d8`, `5a21dc9b95`,
`69f31a4e` and `0b3433786b` (main at 23:00 UTC) 11 red with one and
the same failure text.

- **PR 7354, `sectcone/conic-pairs-on-the-value` (merge `4a94f7109b`),
  moves five:** `seat7_sweep_lowering`'s `boss_union` digest,
  `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval`,
  `revalue_corpus::revalue_boss_union_apart_to_its_parting`, and
  `sweep`'s `bisector_nodes::the_near_node_poses_at_the_witness_band`
  and `section_end_poses::a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds`.
  Its body re-baselines `mesh` `route_rows` because "each section
  conic's edges are now the carried component's arcs, seated once per
  component ... the same conic, parametrized from another seat", and
  names none of these five; its diff touches none of their files. Read
  from the diff, not shown: `boss_union`'s section circle is the same
  re-seating; the two `sweep` rows are band-edge poses whose decisions
  now escalate (`TransversalityArm` inside a `RebasedCarrier`;
  `Coincidence(Section, Moot)` at margin `1.0000000272e-9`) after the
  PR changed what reads a pair's pose.
- **PR 7299, `germ/crossing-consumers` (merge `54e6a5c5b8`), moves the
  other six** and adds `cut_cylinder` to `seat7`'s payload:
  `seat8_split_lowering`, `intent_s2_c_world`, `m10_p_fence`'s `f64`
  and id-masked rows, `edit_placement_corpus_bits`,
  `perf12_census_goldens`. Its body describes behaviour and refusal
  changes and says nothing of geometry bits moving. Read from the
  diff, not shown: a split vertex once placed by interpolating its
  ends' sides is now placed at `curve_face::cross`'s root, so a
  plane-cut cylinder's vertex bits differ in the last places.

**Owed, by the program that moved each:** SECTCONE re-pins or fixes
its five, GERM its six, each judging the move first (the two `sweep`
rows are decisions that changed, not bits: a refusal's kind at a band
edge). A branch that merged main at or past `54e6a5c5b8` inherits all
eleven with these payloads; any other payload on these rows is the
branch's own.

**Also moved on these rows by an open PR:** BOXFINISH's PR 7370
lowers `revalue_boss_union_apart_to_its_parting`'s `moved` floor for
its own reasons (stated there); whoever re-cuts that row's other pin
should do so on a tree that holds it if it has landed.

**Seen again, 2026-11-22 (PR 7219's delta review):** the two `sweep`
rows are red on `3900eb46` and on that PR's head `c65b9292` alike, with
no seed and no ε set, on a binary carrying only a decision logger
(`analysis/oneround/lever-sites-brep/delta-review-report.md` on
`oneround/lever-sites-brep-review`, finding D5). That PR does not move
them: its head merged with main at `5d5cb7d8` gives the two payloads
above.
