---
id: split-sector-coplanar-tilt-beside-side-reads
kind: issue
title: check whether the splitting rules' sector-coplanar tilt and its side reads are decided one at a time
status: open
opened: 2026-10-07
priority: P2
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

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: a tilt decided beside a position read by earlier rows: margins decided one at a time, or one verdict read off two decisions: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEPAIR (`work/honepair/`) by `git mv`, id and body unchanged: the P2 track opened for one verdict decided on one margin at its own reach: rows that decide two margins one at a time, compose two readings, or lever a reading at the wrong extent. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
