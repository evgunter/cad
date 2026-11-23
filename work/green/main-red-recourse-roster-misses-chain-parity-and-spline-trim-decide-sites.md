---
id: main-red-recourse-roster-misses-chain-parity-and-spline-trim-decide-sites
kind: issue
title: main is red (slow set): recourse_roster does not list the indirect decide sites in chain_parity.rs and spline_trim.rs
status: open
priority: P2
opened: 2026-11-21
---


Found by SECT's implementer while running the slow set locally for PR 7193,
which does not touch these files. It is red at that PR's merge base
`ee30c7ad` and still red on main `44ae29dc`.

`cargo nextest run -p topo --profile ci --ignore-default-filter -E
'test(=recourse_roster::every_decided_name_has_its_door_and_every_site_is_read)'`
fails at `crates/topo/tests/recourse_roster.rs:983` with:

    an indirect site not listed: chain_parity.rs: decide: name
    an indirect site not listed: spline_trim.rs: decide: name

The roster lists `chain_parity.rs: ray_parity_as: name`
(`crates/topo/tests/recourse_roster.rs:798`) and no `spline_trim.rs` line.
Each new site needs a roster line with its disposition, or the decide call
has to move back under the name the roster lists. The gating gap that lets
such a change past its PR is
`green/a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster`.

## Two dead carriers (2026-11-22, seen on `oneround/box-region`'s gate)

The same row also reports `a declared carrier matched nothing in src: Const(NEST)` and `… Const(REACH)`. Main's `0639e45a` removed `const NEST` from topo's source. `REACH` survives only as a test fixture's length (`boolean/torus_frame_rows.rs:48`, `injective.rs:4911`), which is no name. The two `CARRIERS` entries in `crates/topo/tests/recourse_roster.rs` go, or are re-pointed at what replaced them. Seen on PR 7225's hosted slow-set step, which ran the roster because the PR edits the file; reproduced on that branch merged with main `706a250d`, and none of the four lines touches the PR's code.

## Two more unlisted sites (2026-11-23, PR 7225's gate on main `43295779`)

The row also reports two indirect sites in `crates/topo/src/pcurves/collapsed_side_rows.rs`, which PR 7325's merge `ba685969` (`charttail/collapsed-column`) added:
- `joint: DescribedChart::of(surface).unwrap()`;
- `joint_at: DescribedChart::of(surface).unwrap()`.

`spline_trim.rs`'s line is gone on that main; `chain_parity.rs`'s and the two dead carriers (`NEST`, `REACH`) stand.

**More lines on main (2026-11-23, `oneround/lever-sites-topo`).** On `origin/main` at `2130aa45f5`, `recourse_roster::every_decided_name_has_its_door_and_every_site_is_read` prints five more lines beside the two above:

- `a declared carrier matched nothing in src: Const(NEST)`;
- the same for `Const(REACH)` (`crates/topo/tests/recourse_roster.rs:240-241`);
- `an indirect site not listed` for `pcurves/collapsed_side_rows.rs: joint` and `joint_at`, both `DescribedChart::of(surface).unwrap()`;
- the same for `pcurves/decision.rs: decide_joint: chart`.

That lane's branch prints the same lines and nothing else.

**On `origin/main` at `ffb8d91ae5` (2026-11-23).** The two `Const(NEST)`/`Const(REACH)` lines are gone. Two new indirect sites are not listed: `boolean/join.rs: conic_section_frame_at: sa` and `boolean/section_cert.rs: cone_torus: f`. The other five lines stand. `oneround/lever-sites-topo` at its merge with that main prints the same seven lines.
