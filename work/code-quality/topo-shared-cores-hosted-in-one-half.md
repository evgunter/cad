---
id: topo-shared-cores-hosted-in-one-half
kind: issue
title: topo — two shared cores still hosted inside one half, finish and conic_plane_crossing_roots
status: open
opened: 2026-08-20
github: 695
refs: [690, S5, S173]
---

## From GitHub issue 695

Opened 2026-08-20; 0 comments.

The remaining residue of SMELL-SCAN **S5** after #690, filed as the schedule that PR's deviations owe. Both reviews of #690 independently returned the same Q6 finding: three of its disclosed deviations defer to a "finish extraction unit" that exists **only as a paragraph in that PR's body** — no issue, no plan row, no spec, no scan row. §C3 names the prose register as the thing that does not execute, and S21's `DocumentId` in the same report is the worked example.

This issue is that register. It carries **two** units, not one — the adversarial review refuted #690's estimate that only `finish` remained.

## Unit 1 — `carve` / `single_solid`

`crates/topo/src/boolean/{finish,ops,rest}.rs` already import `crate::splitting::finish::{carve, single_solid}`. The code is shared; the home is wrong — the same defect #690 fixed for the join core by creating `chord_join.rs`.

Cost as scoped by the implementing lane: two functions, 16 grep hits, ~1 day. Its real difficulty is that `SplitFinishError` is public API re-exported into `pncad`, so the extraction has to answer the public-path question #690 deferred.

**Correction to that scoping, from the style review:** deviation 4 describes `SplitFinishError::Corrupt` as *"payload-free, 2 closure sites in `splitting/finish.rs`"*. The file carries **12 direct constructions** (`:247, :254, :305, :328, :498, :511, :514, :527, :548, :550, :554, :557`) plus 2 closure definitions (`:353, :578`) — **13 sites, not 2**. The credibility of deferring it rested on that number.

## Unit 2 — `conic_plane_crossing_roots`, and it carries a K-name question

`crates/topo/src/boolean/reduce.rs:565` calls `crate::splitting::conic_plane_crossing_roots` (defined `crates/topo/src/splitting/classify.rs:147`) on the boolean's **production** edge×face crossing path — not a test. The surrounding comment says *"the splitting lane's C12.1 machinery reused verbatim"*, and the function's own doc says *"shared with the boolean reduction sweep"*.

It **decides three K predicates**: `split_conic_plane_parallel`, `split_conic_belly_graze`, `split_conic_phase_frame` — all under `split_*` names, decided on the boolean lane.

That is exactly the shape S5 cited as proof the wrong-way dependency is *"bidirectional in fact"* (`bool_between_arc_window` decided at `splitting/join.rs:1611`), and exactly what #690 fixed for the join core. #690's stage table says of the vertex sweep *"different computations … **Nothing shared to extract**"* — refuted; the sharing already exists and only its home is wrong.

Unlike unit 1, this one carries the naming question #661 answered for the sector rungs: three `split_*` K names are recorded from a lane that is not splitting. #652's precedent (pool when the two names are the same computation of the same quantity) and `M3-LOG.md:264`'s counter-precedent (split when they are not) are both on the record; this needs the same evidence-first treatment, and the K-report census note is the home for whatever it decides.

## Also owed here, from #690's deviations

- **Deviation 2** — `chord_join.rs:90-92` still imports three leaves from `splitting/` (`SplitPlane`, `containment`, `rules::face_extent`, the last widened `pub(super)` → `pub(crate)` to permit it). The neutral core still depends on one of its two consumers.
- **Deviation 3** — `SplitJoinError` keeps its split-flavoured name, and more than its name: all 14 `Display` arms open `"split join: …"`, so a boolean refusal renders `boolean op: joining refused: split join: traversal failed at …`. Variant docs explain themselves through `crate::splitting::split` and `crate::splitting::plane_section` only. Renaming reaches `pncad`.
- **Deviation 4** — `SplitFinishError::Corrupt` is the same payload-free defect (c) removed from `SplitJoinError`, one enum over. See the corrected site count above.

## The architectural question underneath, which is not this issue's to settle

This fix class has now produced **three** top-level siblings of `boolean/` and `splitting/` — `sector_shape.rs` (#647), `sector_face.rs` and `chord_join.rs` (#690) — each created because a shared thing had no home belonging to neither half, and each arguing its placement from scratch in its own module doc. The two units above would make it four or five.

At that point *"the crate root is where shared kernel machinery lives"* is a real architecture that ought to be stated **once**, with a rule for what qualifies, rather than three times in three module docs. Related: `sector_shape.rs:399-503` carries a 90-line mechanical anti-re-fork guard that walks all of `topo/src` at runtime and names four shapes it cannot catch; neither `sector_face.rs` nor `chord_join.rs` has an equivalent, and no reason is given.

Also on the record from #690, reported and deliberately not acted on: `DESIGN.md:1275` describes the crate as *"the boolean engine and its splitting/census machinery"* — one engine, splitting subordinate. #690 makes that **less** false and simultaneously shows it cannot be made true as written; the honest sentence names two peer lanes over a shared core. That is a `DESIGN.md` conversation, not a lane's call.

## Home

Code quality: this is the residue of the scan's `S5` finding, carried in the register as `work/code-quality/S5.md`, and one of its two units is already a Track Q row there (`S173`, which cites this number).
