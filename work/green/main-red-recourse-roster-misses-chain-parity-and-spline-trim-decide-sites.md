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
