---
id: the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways
kind: issue
title: entry_side and sectors::side_code read one bound's side of a plane two ways, and disagree at every first-order tie
status: open
opened: 2026-10-13
priority: P1
cost: M
design: true
---


Found by the review of PR 4858 (branch `cleave/vertex-against-section`).

## What

Two readers decide which side of a plane one bound of a vertex's
sector leaves on:

- `splitting::entry_side`: the split's neighbourhood walk, which
  `vertex_section` also reads at both doors. A conic reads
  `split_conic_departure` and, on a first-order tie, falls through to
  `geom_brep::conic_bend_side` (second order).
- `boolean::sectors::side_code`: the Boolean's own sector pass. It
  levers the bound at its `Reach` with a sagitta charge, and a tie stays
  On or refuses.

So at every first-order tie they disagree by construction. On the poses
PR 4858 measured, the disagreement changed no outcome. But the Boolean
decides which faces a section enters at a vertex
(`vertex_section`) by one reader, and builds its sectors by the other.

## Owed

One reading of a bound's side, read by both: either `side_code` takes
`entry_side`'s second-order fall-through, or the reverse, decided on
which one certifies what it claims.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Design signal.** **Cluster A, the vertex-neighbourhood reading against a surface.** Split and boolean each own run enumeration, the sector walk, the bound side, struts and site choice. The only shared ground is the Euler types and the loose `vertex_section.rs`/`sector_*` modules, and the boolean already reaches into `splitting`. `neighborhood.rs::entry_side` and `sectors.rs::side_code` disagree at ties. The canonical copy is hosted inside one consumer (`vertex_section.rs` imports `splitting::entry_side`). `[one-way.readings]` names only the crate (topo), not a module.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: two readers of one bound's side: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
