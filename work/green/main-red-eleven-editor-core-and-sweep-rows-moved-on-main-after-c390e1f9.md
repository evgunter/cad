---
id: main-red-eleven-editor-core-and-sweep-rows-moved-on-main-after-c390e1f9
kind: issue
title: Main red: eleven editor-core and sweep default-set rows moved on main after c390e1f9 (cut_cylinder's and boss_union's digests, the m10 fence, the at-rest census, the boss-union parting, two sweep band poses)
status: closed
opened: 2026-11-22
priority: P0
cost: E
closed: 2026-11-23
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

## GERM's six (2026-11-23, GERM's `germ/repin-u3b-moves`)

**What moved, measured.** Every corpus document evaluated at `f64` on
`54e6a5c5b8^1` and on `54e6a5c5b8`, every node's outcome and every
body point's bits dumped and diffed: one point moved, in `cut_cylinder`
only. The split vertex on the `x = -0.5` seam went from
`z = 0.6546681248048115` (`0x3fe4f30a9138a665`) to `0.6546681248048116`
(`…a666`), one ulp. It is the same point in the two placed halves
(nodes 46 and 61) and the product (69). Every node's outcome, every
other point and each body's face, edge, vertex and point counts held.
The other seam vertex, `(0.5, 0, 0.3453318751951884)`, did not move.

**Judged sound.** The exact crossing of the split plane with the seam
line, computed in rationals from the plane's `f64` origin and normal,
is `t = 0.65466812480481157…`. The parent's interpolation
`t₀ + (t₁ − t₀)·d₁/(d₁ − d₂)` read `…115`, 0.58 ulp below it, which is
not the nearest double. `cross`'s root reads `…116`, 0.42 ulp above
it, which is the correctly rounded crossing. At `Interval`, 7299
narrowed both seam vertices' `z` enclosures, and each new enclosure lies
inside the old one and holds the exact crossing: `x = 0.5` went from
98 ulp wide to 60, and `x = -0.5` from 71 to 34.

The census golden's line 95 at every ε row is the witness of the same
`VertexVertex` contact (`4v1`/`8v1`), `…115` to `…116`. No finding,
kind or count moved. Main at `7691e4f872` reads the merge's values on
every row: the same digests, and the same census diff after a bless.

**Re-pinned**, each with a one-line reason at the pin:
- `seat8_split_lowering`'s `cut_cylinder`;
- `seat7_sweep_lowering`'s `cut_cylinder`, which is one-digest-two-suites
  with seat8 (`fixture/one-digest-of-one-document-is-pinned-in-two-suites-and-re-blessed-in-one`).
  `boss_union`'s pin in the same table is SECTCONE's and stays red at
  `0x2ea09f26ff2947dd`;
- `intent_s2_c_world`'s `cut_cylinder` row;
- `m10_p_fence`'s `f64` row, together with the `probe` row that pins the
  same number, and its id-masked row;
- `edit_placement_corpus_bits`'s two `cut_cylinder` placements;
- `perf12_census_goldens`' three files, blessed per ε with
  `PERF12_BLESS_CENSUS=1`.

These rows were run at the default ε, 1e-6 and 1e-12, with
`CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=1`. The `probe` row was run at the
default ε only, with `--features probe`. At the default ε, the whole
`editor-core` suite was run with its slow set. Every slow-set pin of
these documents passes: `lib_g16_corpus_name_digests`,
`perf2_name_keying_differential`, `revalue_cut_cylinder`, `m5_pr5_corpus`,
`m5_pr11_corpus_curved` and `m10_di_dual_corpus`. Four GERM-free reds
remain at the default ε. Three are SECTCONE's: seat7's `boss_union`,
the Interval fence and `revalue_boss_union_apart_to_its_parting`. The
fourth, `split_pinch_keeps`' kite moat, is already held by
`main-red-rows-two-lanes-found-red-on-their-merge-base-are-held-by-closed-rows-or-none`.
That row also holds `m10_sym_drive_memo_interval`'s race row.

**Not left to SECTCONE alone: PR 7299 also moved the Interval fence.**
On `54e6a5c5b8^1` it reads `(2a1ab4f085f8c0a6, b6848d57920aac6a)`.
That is already SECTCONE's move off the pin. On `54e6a5c5b8` and on
main it reads `(86435b89a5aa3a4a, 84b17f3f33c05abe)`. The only
difference 7299 makes there is the narrowing above: the two seam
vertices of `cut_cylinder`, in nodes 46, 61 and 69. The row is left
unpinned, as SECTCONE's. Whoever re-takes it re-takes both moves, and
GERM's is judged here.

**Seen again, 2026-11-22 (PR 7219's delta review):** the two `sweep`
rows are red on `3900eb46` and on that PR's head `c65b9292` alike, with
no seed and no ε set, on a binary carrying only a decision logger
(`analysis/oneround/lever-sites-brep/delta-review-report.md` on
`oneround/lever-sites-brep-review`, finding D5). That PR does not move
them: its head merged with main at `5d5cb7d8` gives the two payloads
above. (Carried here from the PR's branch by ONEROUND's orchestrator,
whose copy of this row trailed main's.)

## CIW, 2026-11-23

PR 7501 (`ciw/census-digest-goldens`) re-cuts seat7's `boss_union` on a key-free feed and pins it at its tree's reading, with a comment citing this row: "holds PR 7354's move unjudged; SECTCONE owes the judgement". It also replaces `m10_p_fence`'s Interval golden with in-process claims, and adds `the_interval_corpus_holds_sectcones_move_unjudged`, an id-masked Interval pin citing this row for the same move. Neither pin judges the move. Each goes when SECTCONE has judged it.

## Closed (2026-11-23, GREEN U18)

Every row was re-read on hosted CI at main `a433ff83d6`, default ε, in
probe run 45307943329 (slow-set step). Each is now accounted for:

- **Passing** at main: `seat8_split_lowering`; `seat7_sweep_lowering`
  (its `cut_cylinder` is GERM's judged re-pin, and its `boss_union` is
  CIW's held pin); `intent_s2_c_world`;
  `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
  and `the_interval_corpus_holds_sectcones_move_unjudged` (CIW's held
  pin, which replaced the `Interval` row);
  `edit_placement_corpus_bits`. `m10_p_fence::…_at_interval_has_the_f64_outcomes`
  passed in run 45312452002.
- **Gone from main:** PR 7501 removed
  `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_f64` and
  `perf12_census_goldens.rs`, along with its at-rest golden.
- **Held red, PR 7354's:** the two `sweep` rows. They pass on
  `4a94f7109b^1` (run 45312853278) and fail on `4a94f7109b` (run
  45312883535) with main's text, so they are re-bisected to 7354. A
  refusal changed kind at each band edge, which is a decision only its
  owner can judge, so GREEN did not re-pin them. Each runs under
  `test_utils::pinned_red::row` where it is red:
  `bisector_nodes::the_near_node_poses_at_the_witness_band` at the
  default ε, and `section_end_poses::a_section_end_at_the_band_edge_…`
  at the default ε and 1e-6. Probe run 45322247807 read both pins
  green at all three ε.
- **PR 7354's unjudged moves are SECTCONE's**, in
  `work/sectcone/conic-pairs-on-the-value-moved-two-band-edge-refusals-and-boss-unions-section`.
  That covers the two held reds above, the two CIW `boss_union` pins
  (their citations now point there), and the parting pin's share.
- **`revalue_corpus::revalue_boss_union_apart_to_its_parting`: red on
  main for another reason, and not held.** PR 7354's `(2390, 3856)`
  move went into the pin's later re-cuts (`aca51efe7e`; PR 7396's
  `(2372, 3856)`), and nightly 45117679717 (`dc263f8553`) passed it.
  It now fails earlier, at `drifted: 228` with `carrier_on_surface_{1,2}`
  and `witness_on_surface_{1,2}` laundered. That is
  `main-red-revalue-corpus-carrier-and-witness-on-surface-drift-since-7411`,
  which this lane's CI bisect confirms (evidence on that row). It stays
  red there with its nine siblings.
