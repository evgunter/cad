---
id: split-sector-coplanar-tilt-beside-side-reads
kind: issue
title: check whether the splitting rules' sector-coplanar tilt and its side reads are decided one at a time
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## What

Investigation, found by PR 4280's sweep and not traced to a served
verdict there. `topo::splitting::rules` decides `split_sector_coplanar`
(`‖n_face × n_SP‖ · extent`, `crates/topo/src/splitting/rules.rs:208`)
beside a vertex position read by earlier side rows.

## The shape of a fix

Trace whether the two each just inside the band reach a served split
verdict; if they do, decide their sum.
