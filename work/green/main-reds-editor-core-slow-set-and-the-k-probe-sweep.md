---
id: main-reds-editor-core-slow-set-and-the-k-probe-sweep
kind: issue
title: Main carries 24 editor-core slow-set reds and two reds that abort scripts/k_probe_sweep.sh (sweep ray_wall_margin_twins, editor-core m4_pr8_k_probe content key)
status: closed
opened: 2026-10-21
priority: P1
cost: H
closed: 2026-10-30
---


Found by PR 5552's FULL review on main `d39fbfc` (merge base), and
re-measured by NURBS build A on main `3c342076` (2026-10-21). NURBS
build C sees the same set independently (24, `sym11` among them). None
is PR 5552's: every one fails identically on main. Filed on TCOST, which
owns every file below with TINT; SCALAR co-owns the `m10_*_interval`
files (`work.py territory`). One item for the lot, as ruled; split it if
the causes turn out to differ.

**The editor-core slow set** (`cargo nextest run -p editor-core`, debug,
the slow set included, which `--profile ci` filters out): 3018 run, 24
failed, 3309 s.

| file | failing rows |
|---|---|
| `docm9_range.rs` | `a_budget_too_small_is_indeterminate_and_never_a_bound`, `a_branch_change_is_a_decision_flip_and_names_the_predicate`, `a_decision_flips_within_contains_a_value_that_does_not_build`, `the_first_boundary_stops_the_walk_and_a_seed_on_the_island_certifies_it`, `the_certificate_is_inside_the_locally_valid_range` |
| `m10_bulge_interval.rs` | `m10_bulge_the_d_tabs_literal_split_at_the_nominal`, `m10_bulge_the_d_tabs_parameter_split_at_the_nominal` (the review saw `segment_gap [0, 6]`) |
| `m10_sym_profile_interval.rs` | `the_forms_the_walks_build_are_pinned_per_eps_row`, `the_plains_ledger_lines_are_the_same_under_every_dial_set` |
| `m10_9_pins_interval.rs` | `m10_9_the_pad_at_both_rule_f_dials`, `m10_9_what_the_registry_buys` |
| `m10_6_ci_rows_interval.rs` | `the_margin_thin_accounting_is_goldened_bit_exact` |
| `m10_3_driver_interval.rs` | `a_planted_flip_refuses_flip_crossing_and_names_what_flipped` |
| `m10_3_r2_probes_interval.rs` | `containment_fires_when_the_chamber_is_strictly_inside_the_box` (2095 s) |
| `revalue_corpus.rs` | `revalue_measured_web`, `revalue_loft_prism`, `revalue_declared_tangency`, `revalue_die_chamfer`, `revalue_heat_sink_fins`, `revalue_plate_param_apart_to_its_parting`, `revalue_boss_union_apart_to_its_parting` |
| `sym11_exact_channel_rows.rs` | `sym11_the_exact_channel_never_contradicts_past_the_ceiling` (C: `two_hole_plate` reads `[1253, 991, 1557]` against the stored `[1253, 979, 1557]`) |
| `sym_9_retry_interval.rs` | `sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`, `sym_9_the_drive_writes_the_ladders_receipt` |

Many read stored counts or goldens, so a decision-count re-bless that
skipped the slow set is the first suspect: bisect one row per file
before blessing anything.

**Two rows that abort `scripts/k_probe_sweep.sh`** (run as the script
runs them, `--features probe`, ε `1e-9`):
- `crates/sweep/tests/ray_wall_margin_twins.rs:143`,
  `the_wall_arms_margins_scale_linearly_with_the_model`: "predicates whose
  margins do not scale with the model: [curve_foot_branch: 6 pairs off,
  e.g. 9.990e2]".
- `crates/editor-core/tests/m4_pr8_k_probe.rs:353`,
  `probe_agrees_with_f64_bit_for_bit_over_the_corpus` (ignored, swept):
  "die: node … content key — `ContentBits for Probe` feeds the wrapped
  f64's bits, so the two lanes' memo keys are the same word", the keys
  differing.

Either aborts the sweep, so the corpus K-probe dump cannot be taken
through the script on main; PR 5552's review ran the dump directly.

**Done when** each row is green on main, or re-blessed with its cause
named.

## 2026-10-23 — first nightly, and the merge (INTENT nightly triage)

All 24 slow-set rows are red at every ε row (default, 1e-6, 1e-12) in
the nightly full suite from 2026-10-22 (run 39728496598, main
`0ffab3663d`) onward, and green on 2026-10-21 (run 39549991622,
`8c7db2ae37`). Bisected over the merges between `8c7db2ae37` and the
review's `d39fbfc`, nine rows (the five `docm9_range` rows, both
`m10_bulge` rows, `revalue_loft_prism`, `sym11_…ceiling`) run as the
nightly runs them (`--profile ci --ignore-default-filter`, default ε):
all nine pass at `42af7c0b48` (#5548, which already holds INTENT's
#5506) and eight of them fail at `fa901e4308`, the merge of #5542
(`nurbs/seg-one-line`, `profile::seg`'s `gaps_apart` reading
`segment_gap`); the ninth was still running at 400 s when stopped. The
revalue and `m10_bulge` messages name `segment_gap` themselves. So the
suspected merge is #5542. The rows are still red on 2026-10-23 (run
39906156937, `be9b907694`).

## 2026-10-24 — one more slow-set red on `chart/orchestrator` (CHART implementer)

`intent_s2_c_world::the_migrated_world_is_the_pre_c_product` fails on
`chart/orchestrator` at `1a428c9`: "cut_cylinder: the product moved",
`d2acb2b37c5273db/28` against the recorded `cb8ccadf87044e7f/28`. It
fails the same on `chart/ringed-curved-doors`, whose product body for
`cut_cylinder` is bit-identical to the base's: the same counts, the same
points in the same arena order, the same faces and senses.

## 2026-10-24 — still red (INTENT nightly triage)

Nightly run 40066431545 (main `3dfb96b91c`) still reds 22 of the 24
rows at every ε, with the same panics: `docm9_range` ×5,
`m10_3_driver_interval`, `m10_3_r2_probes_interval`,
`m10_6_ci_rows_interval`, `m10_9_pins_interval` ×2,
`m10_sym_profile_interval` ×2, `revalue_corpus` ×7,
`sym11_exact_channel_rows`, `sym_9_retry_interval` ×2. The two
`m10_bulge_interval` rows pass tonight. Some receipt counts have
drifted since 10-23, though each row fails the same way:
`m10_9_the_pad_at_both_rule_f_dials` read `(1546, 1252, 3048)` on 10-23
and reads `(1546, 1251, 2420)` tonight, against the pin
`(1546, 1260, 3026)`. `sym_9_the_drive_writes_the_ladders_receipt`
went from `[1421, 1378, 34]` to `[1421, 1377, 34]`, against
`[1421, 1386, 34]`. So when a fix lands, re-derive the receipts on the
day. Do not take them from this file.

`k-lint (dev-probe)` still stops on
`ray_wall_margin_twins::the_wall_arms_margins_scale_linearly_with_the_model`:
`predicates whose margins do not scale with the model:
["curve_foot_branch: 6 pairs off, e.g. 9.990e2"]`.

## 2026-10-24 — `every_public_mutation_path_preserves_tier1` red on main (CHART implementer)

`topo` `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
fails on main at `179e41e4`: "the allowlist names `join_edges`, which is
no longer a non-asserting public mutation path". `f3a907e7` moved
`join_edges` and `join_edges_within` into a surgery scope closed by
`sweep_and_close`, and kept their `ALLOWED` entries. The test reads
source, so it fails the same on any branch carrying main.
`the_two_door_tables_cover_the_same_surface` then needs each door in one
table. PR 5664 (`chart/ringed-curved-doors`) ports the fix: both entries
leave `ALLOWED` for `pcurves::staleness_posture::DECLARED`'s `Maintains`
block, beside the other doors that assert through a sweeping scope. It
is a no-op once main carries the same move.

**Also red on main `663dd78fef` (PCERT E1, 2026-10-26):** an ignored golden row, `m10_sym_drive_memo_interval::a_freezing_drive_is_identical_across_thread_counts_too` (`--run-ignored all`), fails at the default ε with `tilted-derived: frozen is the column` (`m10_sym_drive_memo_interval.rs:612`). The failure is identical on PCERT's E1 head (PR #5500). On that run the other ignored Sym goldens red on main were already in the table above or in `work/intent/main-red-the-seat-and-name-table-goldens-moved-at-dprime2.md`: the two `m10_sym_profile`, the two `m10_9_pins`, `sym11`, the two `sym_9_retry`, and seat7 and seat8. The other 40 of the 50 were green.

## 2026-10-26 (SPLICE nightly triage): the ray-wall row's 6 → 62 is #5664

`k-lint (dev-probe)` on nightly run 40274924857 (main `38f1a24ed3`)
died before any probe row ran: `error: failed to download from
https://static.crates.io/crates/wide/1.5.0/download … Could not resolve
host: static.crates.io` while building geom-core for the probe census.
That is a runner DNS failure, not a code red. The 10-25 run (40165703340,
`9f5b44561d`) reached the sweep and stopped on this row with **62**
pairs off: `["curve_foot_branch: 62 pairs off, e.g. 9.990e2"]`. On 10-24
it was 6.

I bisected the row alone over the 35 merges between `3dfb96b91c` and
`9f5b44561d`, running it as the script does (`CAD_TOLERANCE_EPS=1e-9 cargo
test -p sweep --features probe --test all -- ray_wall_margin_twins::the_wall_arms_margins_scale_linearly_with_the_model`):
`fa1d94c7db` reads 6 pairs and `80a6f77af3`, the merge of **#5664**
(`chart/ringed-curved-doors`, cylinder and cone point-in-face doors read
the face's region from its edges), reads 62. Tonight's main `38f1a24ed3`
still reads 62 locally. The row was already red at 6 before #5664, so
#5664 widened an existing red rather than causing it. The 6 pairs are
still unexplained.

## 2026-10-26 — what PR 5742 (the value chart reads a written orientation) moves on these rows

These rows stay red, main's own. Measured on main `6ff6b23` and on PR 5742 at the default ε. PR 5742 adds its own move on top of main's drift, and a re-bless should take both:

- **Each mated placement:**
  - the swing's two `mate_chart_reference_antipode` theorems leave;
  - numerically, two `pose_unit_norm` decisions leave and one `mate_chart_pole` decision joins.
- `m10_9_the_pad_at_both_rule_f_dials`: main `(1546, 1251, 2420)`, PR `(1544, 1250, 2420)`.
- `sym11_exact_channel_rows`:

  | study | main | PR |
  |---|---|---|
  | `two_hole_plate` | `[1253, 989, 297]` | `[1249, 987, 297]` |
  | `r1_annulus` | `[632, 556, 212]` | `[630, 555, 212]` |
  | `r2_filleted_bracket` | `[1630, 1328, 1544]` | `[1626, 1326, 1544]` |
  | `r2_rounded_pad` | `[1570, 1263, 2431]` | `[1568, 1262, 2431]` |
  | `r2_link` | unmoved | unmoved |

- `sym_9_the_drive_writes_the_ladders_receipt`: main `[1421, 1377, 34]`, PR `[1419, 1376, 34]`.
- `m4_pr3_names_ci::die_name_tables_are_golden`: the die's name-table digest moves with the mate's orientation in the minting edits' bytes. Main reads `0x7fb3623b41caf7c9`, the PR `0x07b858a990d27a53`.
- Read the same on both: `m10_sym_profile_interval` ×2, `m10_9_what_the_registry_buys`, `node_standing`, `intent_s2_c_world` and `revalue_corpus` ×7.


## 2026-10-26 — three more slow-set rows red on main `d67ac3cbde` (PR 5742's fix pass)

Measured on main itself at the default ε:
- **`m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval`:** reads `(11122170326591155306, 14158967091460525030)` against its pin `(13242046646792044098, 15401937490762412494)`. The f64 and ids-masked rows hold, so a change on main moved the Interval lane's bits without re-pinning. PR 5742 re-pins this row at its own reading, because the mate's orientation moves the ids the digest feeds; that reading takes in main's move too.
- **`perf12_census_goldens::the_at_rest_census_verdicts_are_goldened_bit_exact`:** drifts from line 13 at every ε. The new lines are `CoincidentWedgesOverlap` findings (`work/restread/main-red-coincident-wedges-overlap-refuses-flush-copies.md`). PR 5742 re-blesses all three ε files at its own reading, for the same reason.
- **`r2_m10_di_probes::assemble_census_door_matches_f64_at_dual64`:** the divergence set gains `heat_sink`, reading `["heat_sink", "kitchen_sink", "cut_cylinder"]` against `["kitchen_sink", "cut_cylinder"]`. It reads the same on PR 5742, which leaves it.

## 2026-10-28 — the nightly `--all-features` job holds `ray_wall_margin_twins` out

`nightly.yml`'s `suite under --all-features (default eps)` job (GREEN,
PR 5962) excludes
`ray_wall_margin_twins::the_wall_arms_margins_scale_linearly_with_the_model`
through its `HELD_OUT_TESTS` nextest filter, because it is this row's
`k-lint (dev-probe)` red. Whoever fixes that test also drops the
`HELD_OUT_TESTS` filter (the env and the `-E "not (…)"` it feeds) from
that job in the same PR, so the job runs it again.

## 2026-10-28 — the `segment_gap` cause fixed; what is still red (GREEN, `green/slow-set-reds`)

Today's failing set comes from nightly run 40637814390 (main `a1cfd1cfdb`), default ε.
In this file: `docm9_range` ×5, `revalue_corpus` ×7, `intent_s2_c_world`,
`m10_3_driver_interval::a_planted_flip…`, `m10_3_r2_probes_interval`
containment, `m10_6_ci_rows_interval` accounting, `m10_9_pins_interval`
×2, `m10_sym_profile_interval` ×2, `sym11`, `sym_9_retry_interval` ×2,
`m4_pr3_names_ci`, `perf12_census_goldens`. The two `m10_bulge` rows were
green on main that night and go red on this branch's fix, which re-pins them.

**The cause (#5542, confirmed).** Two defects in the gap reading #5542
put first in `profile::seg`'s `line_line`:
- a pair that shares a vertex was read, although its gap is zero by
  construction;
- `geom_core::one_line`'s `apart()` read an overlap by its length, so a
  perpendicular span, which projects to a point, sat at the gap's zero
  at any depth inside the other span.

At `Sym<f64>` both are in-band Zero decisions that flip to Negative
under the revalue row's apart move (`revalue_*`: the flips were all
`segment_gap`; the p1 margin scales with the step, so it is geometry,
not noise). At Interval they stop range and drive leaves. Fixed: the
shared-vertex pair is not read, and `apart()` reads the overlap's
depth. Positive readings are bit-identical, and every caller acts only
on Positive.

Green on the fix, measured locally at the default ε:
- `revalue_corpus` ×7, `intent_s2_c_world`, and
  `m10_3_driver_interval::a_planted_flip…`;
- `docm9_range` ×5, after the notch fixture moved (below);
- `profile`, `geom-core`, `topo` and `geom-brep` at `--profile ci`;
- `m10_bulge` ×2, re-pinned: `segment_gap` 0/6 → 0/2;
- the fast-set rows that count those readings, re-pinned in the same PR
  (`kstats_bracket_rows`, `asm2a_instantiate`, `m10_8`, `m4_pr6`, the
  `m10_6` cert keys).

**docm9's notch.** Its seed held two real `segment_gap` flips below the
crossing, at h = (1 − √2)/2 and h = −1/2. The rows' subject is the
crossing, so the fixture moved: nominal −0.125, seed [−0.0625, 0.15].
A new row pins the gap's flip from the old nominal. Filed on CPAIR:
`work/cpair/a-line-pairs-gap-reading-is-a-range-boundary-where-no-contact-changes.md`.

**Still red after the fix (default ε, local):**
- Receipts this fix moves:
  - two_hole_plate's SymCounts go from main `[1305, 987, 297]` to `[1149, 824, 229]`
    (`sym11`, `sym_9` ladder, `m10_9_no_registrant`);
  - the slab's forms go from main Plain/Decision 992 to 976
    (`m10_sym_profile` ×2);
  - m10_6's planted_flip: main certifies 0 of it, the fix certifies 0.5 with
    flip_crossing 0.5.

  These rows need a re-bless at the reading after this fix, each move
  explained. Their stored values also carry drift from other merges
  (5742 and 5728, per the sections above), so they are left for a
  follow-up PR.
- Receipts this fix does not move, read the same on main:
  - `m10_9_the_pad_at_both_rule_f_dials` `(1592, 1250, 2465)`;
  - `sym_9_the_drive_writes_the_ladders_receipt` `[1463, 1380, 34]`;
  - `r1_annulus` `[654, 555, 212]`;
  - `m4_pr3_names_ci` `0x35eb46e3c6c938ab`.

  Not bisected yet.
- `m10_9_what_the_registry_buys`: r2_link's door refuses at
  "another predicate" (limb 2's rung-3 certificate against the
  cylinder) where the pin says `pcurve_envelope`. A behaviour move, not
  a count. Not bisected.
- `m10_3_r2_probes_interval` containment (≈ 2000 s): not run locally.
- Owned by other rows, left to them:
  - `m10_3_driver::the_certification_width…`, `m10_4_r1_probes` and
    `m10_9_no_registrant` (`work/offset/main-red-three-m10-interval-pins-moved-at-5728.md`);
  - `e6_drive_over_a_loft` ×2 (`work/isotail/a-loft-drive-at-interval-refuses-every-leaf-since-the-one-polynomial-weight-test.md`);
  - `perf12` (`work/restread/main-red-perf12-census-golden-kitchen-sink-loses-a-curved-pair-at-5817.md`).
- `k-lint (dev-probe)`: still stops on the ray-wall row, 62 pairs. The
  widening from 6 belongs to `work/charttail/main-red-ringed-curved-doors-reds-four-sweep-rows.md`.
  The base 6 is not investigated here.

**After PR 5952's review** (merged with main, which carries #5935):
- #5935's pin on `m10_3_driver::a_planted_flip…` is dropped; the row
  passes as written.
- `sym11` is re-pinned: the plate's numeric count 832 → 824 and the
  bracket's 1171 → 1161.
- `m10_9_what_the_registry_buys` is re-pinned: the bracket with the door
  shut, (1474, 1228) → (1474, 1218).

All three come from the shared-vertex pairs' gap readings leaving.
Measured at all three ε.

## 2026-10-28 (later) — the rest pinned, not fixed (GREEN, `green/slow-set-reds-pins`)

Under the GREEN orchestrator's rule, the reds left after PR 5952 are
pinned at main's reading (`fa2a942f45`), with comments naming the row
that owns each fix. Rows PR #5935 pins are left to it: `m10_3_driver` ×2,
`m10_4_r1`, `m10_9` ×3 and `sym11`.

**Bisected and explained, so re-blessed:**
- `m4_pr3_names_ci::die_name_tables_are_golden`, both digests. The first
  failing merge is PR 5742 (`19eff38c58`); `8c3f542123` before it passes.
  The mate's orientation is written in the minting edits' bytes, as PR
  5742 measured. Now `0x35eb46e3c6c938ab` and names `0x2bcda7f2f5d4dffb`.

**Bisected to #5542 (`fa901e4308`; `42af7c0b48` passes), pinned:**
- `m10_6` accounting ×3 ε;
- `m10_sym_profile` ×2;
- `sym_9` ×2;
- `m10_3_r2` containment, not re-run at those commits.

All are filed on CPAIR,
`work/cpair/main-red-slow-set-receipts-moved-at-5542.md`, with each row's
values before, at and after #5542, PR 5952's readings, and the
reproduce command.

**Pinned, filed elsewhere:**
- `ray_wall_margin_twins` at 62 pairs:
  `work/issues/ray-wall-twins-curve-foot-branch-margin-scales-with-the-model-squared.md`
  for the six; `work/charttail/…` keeps the widening.
- `m4_pr8_k_probe`: the content and naming keys split on 26 documents,
  pinned as a set:
  `work/tally/probe-lane-content-and-naming-keys-split-from-f64-on-every-corpus-document.md`.

**Still not this row's:** `e6` ×2 (ISOTAIL), `perf12` (RESTREAD), and the
rows #5935 pins. `revalue` ×7, `docm9` ×5 and `intent_s2_c_world` are
PR 5952's.

With `ray_wall_margin_twins` pinned, `nightly.yml`'s `--all-features`
job runs it again: the `HELD_OUT_TESTS` filter is dropped in the same PR,
as the section above asks.

**After PR 5952 landed (2026-10-29):**
- the pins above are re-taken on main with PR 5952, at all three ε;
- `m10_3_r2` containment passes and is un-pinned;
- `m10_6` accounting, `m10_sym_profile` ×2 and the `sym_9` ladder are
  re-pinned at the new readings.

The values are on the CPAIR row.

## 2026-10-29 — one more slow-set red after SELFX (CLEAR's LOOPS implementer)

`m10_5_r2_probes_interval::the_cost_curve_is_flat_where_the_bound_is_broken`
fails at default ε on `clear/loops`, merged up to main `5e934f49f9`
(after SELFX, #5955). The frontier run at `c = 0.5` over the comb's
walls 7 and 9, which are named in both scopes, comes back
`Violated distance=0` on `FaceKey(10v1)/FaceKey(10v1)` with receipt
`candidates=3 violated=1 abandoned=2`, where the row expects
`Refused(Budget(_))` after more than 1 000 cells. LOOPS's differential
ran the pre-LOOPS walk beside the shared one on this row, and the two
agreed, so the red predates LOOPS. The likely cause is SELFX's root arm
"two scopes that meet are at distance 0": a face named in both scopes
meets itself, so every `c > 0` is violated off topology, and the row's
frontier has gone. The last nightly (`a1cfd1cfdb`) predates #5955 and
does not show it. CLEAR owns the cause. **Fixed by PR #5979**, which
re-fixtures the row to two scopes, {wall 7} against {wall 9}. Both
halves hold unchanged: one cell pair at each broken bound, and the
frontier at `c = 0.5` refused on a budget after 65 536 cells.
`the_margin_thin_accounting_is_goldened_bit_exact` (above) is still red
on the same tree.

## Closed (2026-10-30)

Every red this row names is greened on main: re-blessed with its cause,
fixed at its site, or pinned (`test_utils::pinned_red`, exact-value or
census pins, or a nightly-filter exclusion) with a dated line on the
owning program's open row. The last of them landed in #6093. The causes
live on those rows, not here. A pin that goes red later is a new
nightly red, handled from that night's triage.
