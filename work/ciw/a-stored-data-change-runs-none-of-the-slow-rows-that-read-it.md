---
id: a-stored-data-change-runs-none-of-the-slow-rows-that-read-it
kind: issue
title: A change to what a door stores runs none of the slow rows that read it; three BOXFINISH PRs in a day left or nearly left committed rows red
status: open
opened: 2026-11-15
priority: P1
cost: M
refs: [6959, 6972]
---

Filed by the BOXFINISH orchestrator from two independent reviews of PR 6972
(2026-11-15). The same class as
`a-diff-that-moves-corpus-ids-runs-none-of-the-slow-set-id-pins`, wider:
not only ids.

**The mechanism.** The PR gate's slow-set step runs "the slow-set tests
the diff adds or edits" (`.github/workflows/ci.yml`, header lines 5-9),
and it finds them by test names on the diff's ADDED lines. A change to
what a door STORES (a plane's origin, a stored direction, a chart
image's ends) moves digests, receipts, exported bytes and pins in slow
rows anywhere in the workspace, and names none of them. A re-baseline
in an edited test file does not even run that file's other slow rows.
`docs/prompts/implementer-discipline.md` §2 says the gate runs the slow
tests a diff adds or edits, which an implementer reads as cover it does
not give. A lane's local `--profile ci` run excludes the slow set too.

**Measured.**
- **PR 6972** (the sweep doors keep their frames; every sweep's stored
  planes move). CI run 43669842805 on head `032ac185`: `gate ok` green;
  its slow-set step ran 4 rows (1 kernel, 3 app), at the default ε.
  About 2180 slow rows ran nowhere on that head. Two reviewers then
  found six committed rows red on the head and green on the merge base:
  `step-export` `committed_fixtures_are_byte_golden` and
  `kernel_sidecar_fields_match_live_kernel`; `editor-core`
  `seat8_split_lowering::the_split_documents_evaluate_to_their_committed_digests`,
  `m10_5_r1_probes_interval::e2e_channel_slider_over_an_epsilon_box`
  (at 1e-6 and 1e-12 only) and
  `coincidence_door::the_coaxial_stage6_unions_are_green_at_f64_on_their_coaxial_rows`
  (at 1e-12 only); `viewer`
  `index_memo::a_wide_candidates_interval_reaching_the_aimed_vertex_refuses_with_both`.
  Caught in review, not merged.
- **PR 6959** (a chart image stores its ends), merged 2026-11-14 with a
  green gate. Two slow rows in files it edited are red on main after
  it: `sweep`
  `split_edge_loft_charts::a_split_on_a_spline_chart_carries_the_parents_image`
  and
  `m8_4_intersection_iso::a_boundary_column_intersection_mints_its_iso_image`
  (found by PR 6972's reviewer on its merge base; attribution to PR
  6959 is being measured by BOXFINISH).
- **PR 6958's** fix pass: three of its own slow rows ran at the default
  ε only (`a-slow-rows-eps-pins-run-nowhere-on-the-pr-that-adds-them`).
- **PR 6959, measured** (`boxfinish/chart-ends-slow-rows`): an edit INSIDE
  a slow row's body does not run that row either. The step keys on test
  names on added lines, and a body edit adds none:
  - PR 6959's mechanical edits to the bodies of
    `m8_4_intersection_iso::a_boundary_column_intersection_mints_its_iso_image`
    and of the image `split_edge_loft_charts::a_split_on_a_spline_chart_carries_the_parents_image`
    compares left both red on main at every ε;
  - the gate ran neither.

**What would close it.** One of, or both: the slow-set step selects by
the crates a diff's changed SOURCE reaches (a `src` change to a crate
runs the slow rows of that crate and of its dependents' test files that
hash stored data), at the ε rows too; and the discipline's §2 sentence
says what the gate does. The cost side is
`a-slow-set-edit-to-nextest-toml-runs-the-whole-suite` and
`kernel-shard-long-pole-is-one-test-binary`.

**Until then** an orchestrator whose unit changes stored data asks its
lane for a head-against-merge-base differential of the slow set, as
BOXFINISH now does.
