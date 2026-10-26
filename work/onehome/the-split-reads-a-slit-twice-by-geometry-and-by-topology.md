---
id: the-split-reads-a-slit-twice-by-geometry-and-by-topology
kind: issue
title: The split reads a slit two ways: spur_tips by geometry, section_kind by topology
status: open
opened: 2026-10-19
priority: P1
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
straight-only limit (`work/honejoin/split-section-spur-guard-skips-curved-spurs.md`).

## Owed

One slit reader: the topological one, extended to a tree hanging off an
area polygon's loop, with `strip_spurs` and `drop_slits` reading it.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Mechanical, with one caveat.** Both readers are in `splitting/join.rs`. Keep the topological `section_kind` and drop `spur_tips`. The caveat: if a slit can come from two independently built coincident edges, the geometric read becomes coincidence-door work. DESIGN home: `[coincidence.structural]`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: one slit read by geometry and by topology: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
