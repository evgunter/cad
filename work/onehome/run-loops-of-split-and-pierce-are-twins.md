---
id: run-loops-of-split-and-pierce-are-twins
kind: issue
title: splitting/insert.rs's run loop and boolean/vtxfac.rs's are near-verbatim twins: one home?
status: open
opened: 2026-10-06
priority: P1
cost: M
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

## Decided (Ev, PR 6359, 2026-11-02)

DESIGN `[one-way.places]`: a vertex's neighbourhood cut by a surface (its sector walk, the cuts and their germs, the germs' order, the null-edge plan and the mint at a vertex) has one home, `topo::star`. A verb supplies its sides and which circle carries its chords. The designers agreed and Ev approved, with this staging:

1. **Stage 1, the gated mint and one site resolution** (this row's unit). One site type: `Fan { he1, he2 }`, a strut whose tip stands for the arc, a strut whose tip stands for the rest (today's `WholeOrbit`), and the lone first strut of a bare ring vertex. `RunSite` becomes private to it, the whole-orbit side flip happens inside the mint, and `mev_null_run` takes only a site the module built. `Body::run_site`, `Body::strut_before_next_real` and `null::ring_tree` move under `topo::star`. The three arc→site scaffolds (split `insert_null_edges`; vtxfac `classify_vertex_on_face`; `boolean/insert.rs` `mint_run`/`run_fan`/`run_degenerates`) and the three tree→sites loops read it, and `mint_run`'s `WholeOrbit` refusal and `run_degenerates` go. Run enumeration (`above_runs`, `out_runs`) moves here over a small view each entry type implements. The vertex–vertex planner's own chord nesting (`b_runs`, `arc_holders`, `held_by`) reads `ring_tree` if it is the same in substance; test the equivalence over the meeting fixtures first, and file it if not.
2. **Stage 2:** one walk and entry type (`Star`), absorbing the split's and the Boolean's walks and their private `sector_face`s, with `sector_face`, `sector_shape` and `bound_side` as its parts.
3. **Stage 3:** the cut (sides → cuts), one germ rule signed by the transition's codes (rides `two-germ-readers-sign-one-line-two-ways`), one germ order (folds `honeone/germ-orders-about-a-normal-have-two-homes`), and `vertex_section::entered_faces` reading the cuts (items 3–4 of `vertex-section-quotient-…`). The offset → side reading stays outside.

The chord choice (corners or runs) is a per-verb parameter. The pierce's runs are sound on the probe's fixture (PR 6360).

**Stage 1 landed (PR 6392, 2026-11-03).** `topo::star` holds:
- `Site`, with its private `RunSite`;
- `TipSide`;
- the gated `mev_null_run`;
- `run_site`, `strut_before_next_real` and `ring_tree`;
- `runs` over the `Member` view;
- `arc_site`, `hang` and `hang_ring`.

`euler::RunSite`, `run_degenerates` and `mint_run`'s `WholeOrbit` refusal are gone. The planner spells every whole-orbit run as its strut at any count of survivors. `run_end`/`run_half` are the one reading of where a run went.

The FULL review found one MAJOR (a whole-orbit run past two survivors handed held runs the tip), which is fixed and pinned (`a_whole_orbit_run_of_four_survivors_is_spelled_as_its_strut`). Filed by the lane:
- `the-vertex-pair-planner-nests-struts-across-pairs-beside-ring-tree` (stage 2);
- `the-pierce-reads-a-struts-facing-the-stars-site-already-names`.

Stages 2 and 3 remain on this row.
