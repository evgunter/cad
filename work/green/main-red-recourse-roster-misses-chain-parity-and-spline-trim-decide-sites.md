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

## Seen by `oneround/unsourced-unions` (PR 7294), 2026-11-21

Confirmed on that branch's final tree (debug build, default ε): `recourse_roster::every_decided_name_has_its_door_and_every_site_is_read` fails with `an indirect site not listed: chain_parity.rs: decide: name` and `… spline_trim.rs: decide: name`. Neither file is in that branch's diff.

## Re-pinned (2026-11-23, `embed/roster-main-drift`)

Reproduced on main `6c5ea3ec`: the row reported the two sites above and
two more, `a declared carrier matched nothing in src: Const(NEST)` and
`Const(REACH)`.

- **The two sites** came with PR 7185 (commit `30439a67`, "a bound is
  one typed term"), which moved each `decide(name, ..)` into a method on
  the value it decides: `Gap::decide` (`crates/topo/src/chain_parity.rs:41`),
  called by `ray_parity_as` as `g.decide(name, arm, band)`, and
  `Floor::decide` (`crates/topo/src/spline_trim.rs:153`), whose one caller
  is the spline region's carrier walk,
  `cell.floor(q).decide(SPLINE_REGION_CARRIER, band)`
  (`crates/topo/src/spline_region.rs:258`). Both are `decide`-spelled
  forwarders, whose callers the census already reads as decide sites, so
  each takes a `FORWARDS` line in `INDIRECT` with its own comment: what
  it decides, whose name it carries, and what a refusal does there
  (chain parity: the piece is halved, and at the depth stop the query
  reads `Side::Near` with the diagnostic; spline floor: the cell is
  walked and the diagnostic kept, and a walk ending unsettled on it
  escalates as `ContainDecision::Carrier`). No code moved.
- **`NEST` and `REACH`** were the local consts of topo's closed-form
  `sphere_plane` and `sphere_sphere` section arms
  (`boolean/section_cert.rs`, `section_sphere_plane_reach`,
  `section_sphere_pair_reach`, `section_sphere_pair_nest`), removed by
  PR 7354 (merge `4a94f710`, "plane × sphere, sphere × sphere and plane ×
  cylinder on the section value"). Those poses are now decided in
  geom-brep, under `ps_center_gap` (`crates/geom-brep/src/intersect.rs:1524`)
  and `ss_carrier_external` / `ss_carrier_internal` (`:1673`), and topo
  reads them off the section value. No topo name is spelled in either
  const any more, so both declarations are removed from `CARRIERS`.

**More lines on main (2026-11-23, `oneround/lever-sites-topo`).** On `origin/main` at `2130aa45f5`, `recourse_roster::every_decided_name_has_its_door_and_every_site_is_read` prints five more lines beside the two above:

- `a declared carrier matched nothing in src: Const(NEST)`;
- the same for `Const(REACH)` (`crates/topo/tests/recourse_roster.rs:240-241`);
- `an indirect site not listed` for `pcurves/collapsed_side_rows.rs: joint` and `joint_at`, both `DescribedChart::of(surface).unwrap()`;
- the same for `pcurves/decision.rs: decide_joint: chart`.

That lane's branch prints the same lines and nothing else.

**On `origin/main` at `ffb8d91ae5` (2026-11-23).** The two `Const(NEST)`/`Const(REACH)` lines are gone. Two new indirect sites are not listed: `boolean/join.rs: conic_section_frame_at: sa` and `boolean/section_cert.rs: cone_torus: f`. The other five lines stand. `oneround/lever-sites-topo` at its merge with that main prints the same seven lines.

- 2026-11-24, seen by sectcone PR 7637 (run 45306553205, head `b0c05e10d7`, merged with main at `cf395d8365`): the panic prints nine lines, none in a file or function that PR touches:
  - `source the reader does not walk: live.rs:2421: neither a #[path = …] mount nor an include! of a data file`;
  - `an indirect site not listed: boolean/join.rs: conic_section_frame_at: sa`;
  - `an indirect site not listed: boolean/section_cert.rs: cone_torus: f`;
  - `an indirect site not listed: census.rs: meeting_point: a.p0 + a.dir.get() * s`;
  - `an indirect site not listed: census.rs: meeting_point: e.p0 + e.dir.get() * ((f.origin - e.p0).dot(f.normal) / along)`;
  - `an indirect site not listed: pcurves/sheet_apex_rows.rs: joint: DescribedChart::of(surface).unwrap()`;
  - `a listed indirect site the census no longer finds: census.rs: meeting_point: a.p0 + a.dir * s`;
  - `a listed indirect site the census no longer finds: census.rs: meeting_point: e.p0 + e.dir * ((f.origin - e.p0).dot(f.normal) / along)`;
  - `a listed indirect site the census no longer finds: ring_path.rs: spline_crossings:`.

  New since this row's last note: the two `census.rs: meeting_point` pairs (`dir` became `dir.get()`), `pcurves/sheet_apex_rows.rs: joint`, and `ring_path.rs: spline_crossings`.
