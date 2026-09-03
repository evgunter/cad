---
id: chart-azimuth-and-bbox-anchor-idioms
kind: issue
title: Two three-line idioms with no home - chart-frame azimuth of a direction, and the bbox-centre conditioning anchor
status: open
opened: 2026-08-31
github: 1402
refs: [881, 1389]
---

## From GitHub issue 1402

opened 2026-08-31, 0 comments.

(S-MESH orchestrator) Filed from MESH-1's dual review ([#1389](https://github.com/evgunter/cad/pull/1389)); both reviewers converged on the class independently. At frozen head `f0618c8e`:

**Chart-frame azimuth of a direction** — `w.dot(v_ref).atan2(w.dot(u_ref))` is spelled inline at `mesh/src/walk.rs:216–219` (`Chart::azimuth`, the only named home, one caller, takes a `Point3`), again at `walk.rs:1116–1118` on a `Vec3`, a third time in #1389's test helper `band_mid_az` (`walk.rs:1793–1796`), with a fourth at `step-import/src/chart.rs:213` and a candidate at `sweep/tests/verbs_sphsph_chart.rs:705`. The extractable core ("azimuth of a *direction* in a chart frame") has no signature anywhere, which is why each caller re-spells it.

**Bbox-centre conditioning anchor** — `fold min/max; lo + (hi−lo)·0.5` now has three verbatim Rust spellings (`mesh/src/validate.rs:~142`, `mesh/src/walk.rs:957–964`, `sweep/tests/revolve_common/mod.rs:166–181`) held together only by a comment ("the spelling `validate::signed_volume` uses") — the reconcile-two-spellings comment shape. The Python pair (`test_mesh.py`, `docs/guide/meshing.md`) are deliberate independent oracles and don't count.

Related observation from the same review, recorded with it: `walk.rs` now uses "anchor" in four senses (`Chart::anchor` surface origin, `walk_anchor` traversal index, `unwrap_tie`'s azimuth anchor, `loop_area`'s conditioning anchor) — whoever homes the idioms should pick the word that disambiguates.

S-MESH ground (`crates/mesh`); the `step-import` copy is U's — filed as the seam when reached. Not urgent; a natural rider on MESH-4 (issue 881's named-operations pass, which touches the same files with the same "make the compiler hold it" shape).

## Home

S-MESH: both idioms live in `crates/mesh/*` (S-MESH's territory glob), the issue names S-MESH ground, and it proposes riding MESH-4's named-operations pass over the same files.
