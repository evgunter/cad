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
