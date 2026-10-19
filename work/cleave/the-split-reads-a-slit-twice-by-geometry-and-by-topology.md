---
id: the-split-reads-a-slit-twice-by-geometry-and-by-topology
kind: issue
title: The split reads a slit two ways: spur_tips by geometry, section_kind by topology
status: open
opened: 2026-10-19
priority: P3
cost: M
---


Found by the review of PR 5350 (`cleave/keel-slit-ring`).

## What

The join reads slits in two places:

- `spur_tips` (`crates/topo/src/splitting/join.rs`) reads them by
  geometry. A `Spur::Slit` is two distinct edges running out and back
  over one line or conic, compared by point distance
  (`split_section_spur`). `Sweep::strip_spurs` pairs it with a strut,
  and `plane_section`'s `drop_slits` takes it out of the outline.
- `Sweep::section_kind` reads them by topology. A loop running along
  each of its edges both ways walks round a tree, and its coincident
  loop is a slit trace, checked by point keys.

`kill_struts` already walks a strut tree from its tips. A one-end slit
is the same structure as a ring with one tip on the outline: the strut
side runs out along a tree and back, and the slit side runs two edges
over each tree edge. One topological reading could serve both, since
copies share point keys, and would retire the geometric pairing and its
straight-only limit (`work/hone/split-section-spur-guard-skips-curved-spurs.md`).

## Owed

One slit reader: the topological one, extended to a tree hanging off an
area polygon's loop, with `strip_spurs` and `drop_slits` reading it.
