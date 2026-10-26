---
id: run-loops-of-split-and-pierce-are-twins
kind: issue
title: splitting/insert.rs's run loop and boolean/vtxfac.rs's are near-verbatim twins: one home?
status: open
opened: 2026-10-06
priority: P1
cost: M
design: true
---



Found by PR 4096's review (2026-10-06).

Two places implement the same run scan:

- `crates/topo/src/splitting/insert.rs`: `above_runs` and
  `insert_null_edges`.
- `crates/topo/src/boolean/vtxfac.rs`: `out_runs` and the
  piercing-side runs loop in `classify_vertex_on_face`.

What they share:

- The enumeration: maximal cyclic runs over one class, anchored at the
  first entry of the opposite class.
- The real-member filter.
- `run_site`'s `Fan` and `WholeOrbit` arms.
- The lone-bisector strut at `after`, now with the same invariant and
  the same `unreachable!` check in both.

What differs is the entry type (`SectorEntry` with `SectorEntryKind`,
against vtxfac's `Entry` with `is_edge`), the class
(`PlaneSide::Above` against `SideCode::Out`), the side and attribute
derivation (the split always takes `Above` except at a whole orbit;
vtxfac reads the facing through `strut_faces_first`), and the record
type. Both lanes allow any number of runs; vtxfac also hangs its ring
struts in the runs' angular order (`vtxfac::ring_order`).

Owed: decide whether the enumeration and the site choice (fan, whole
orbit, or strut at `after`, with its check) belong in one helper over
`(he, is_real, in_run)`, which both lanes would call. Each lane would
keep its own side and record. If they share a home, the proof at the
strut site is stated once. If not, say why the two must stay separate.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Design signal.** **Cluster A, the vertex-neighbourhood reading against a surface.** Split and boolean each own run enumeration, the sector walk, the bound side, struts and site choice. The only shared ground is the Euler types and the loose `vertex_section.rs`/`sector_*` modules, and the boolean already reaches into `splitting`. A third copy exists (`boolean/insert.rs`), and it disagrees: it refuses `RunSite::WholeOrbit` where split and `vtxfac` mint it. Unclear: who owns "a vertex neighbourhood's runs against a surface, with the null-edge site and strut". Candidates: beside `RunSite` in `euler.rs`, a topo module such as `vertex_section.rs`, or split's copy made canonical. DESIGN has no canonical-places row for it.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: one run scan implemented twice: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
