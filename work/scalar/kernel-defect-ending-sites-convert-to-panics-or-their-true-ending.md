---
id: kernel-defect-ending-sites-convert-to-panics-or-their-true-ending
kind: issue
title: Every KERNEL_DEFECT_ENDING site outside editor-core panics or takes its true row's ending, per D9 as rewritten on PR 5411
status: open
opened: 2026-10-20
priority: P2
cost: H
refs: [a-kernel-defect-is-a-panic-or-a-typed-refusal-and-nothing-says-which]
---


Filed by WIRE's orchestrator after Ev ratified PR 5411 (`docs/DESIGN.md`
`[failure]` and `[failure.taxonomy]`, 2026-10-20).
- **The rule.** The kernel panics on kernel bugs and only on kernel bugs,
  and no input should be able to lead to a known panic. A typed refusal
  ending in `geom_core::KERNEL_DEFECT_ENDING` ("only a kernel defect
  reaches this") is therefore one of two things:
  - a bug, so it panics: row 4 `unreachable!` with a why-message, or row 0
    if a type can hold the fact;
  - a misfiled case a correct kernel can reach, so it takes its true
    ending: row 1 with a lever (`KERNEL_LIMIT_RECOURSE`), or row 2
    (`NOT_YET_ENDING`).
- **A load-bearing check.** A shipped re-deriving check runs in release,
  and its failing branch is `unreachable!`.
- **A file is input.** A document check or an import-door check is row 1,
  so `KERNEL_OR_FILE_DEFECT_ENDING` survives only there.
- **A known reach.** Where real input is known to reach a site today, fix
  the bug first (D9: no known panic), then convert it.

Each site's owner audits and converts its own. The constant's doc and,
once no site is left, the constant itself are SCALAR's. WIRE converts
editor-core (`eval/`, `names/` emitters). Sites by owning
program (`work.py territory`, 2026-10-20):

- carve, carverest, strut: `crates/sweep/src/swept.rs`
- chart: `crates/topo/src/chart_region.rs`
- cleave, flush, hone, sect: `crates/topo/src/boolean/join.rs`
- cleave, gauge, hone: `crates/topo/src/boolean/insert.rs`
- cleave, gauge, hone: `crates/topo/src/splitting/containment.rs`
- cleave, gauge, hone: `crates/topo/src/splitting/finish.rs`
- cleave, gauge, hone: `crates/topo/src/splitting/mod.rs`
- cleave, gauge, hone: `crates/topo/src/splitting/section.rs`
- cleave, hone, orbit, reachtail: `crates/topo/src/boolean/ops.rs`
- cleave, hone, tang: `crates/topo/src/chord_join.rs`
- cleave, hone: `crates/topo/src/boolean/mod.rs`
- cleave, hone: `crates/topo/src/boolean/refusal_routes.rs`
- cleave, hone: `crates/topo/src/boolean/vtxfac.rs`
- encl: `crates/geom-brep/src/offset_fit.rs`
- encl: `crates/geom-brep/src/patch_bound.rs`
- lib: `crates/pncad/src/workspace.rs`
- offset, shell: `crates/geom-brep/src/offset.rs`
- paths, round: `crates/profile/src/validate.rs`
- pcert, pctail: `crates/geom-brep/src/pcurve_cache.rs`
- restfront: `crates/topo/src/validate.rs`
- scalar: `crates/geom-core/src/predicate.rs`
- select: `crates/geom-brep/src/certify.rs`
- shelf, shell: `crates/topo/src/replace_face.rs`
- shelf, shell: `crates/topo/src/shell.rs`
- ssiarith, ssiedge, ssimarch: `crates/geom-brep/src/ssi.rs`
- ssiarith, ssiedge, ssimarch: `crates/geom-brep/src/ssi/exhaust.rs`
- ssiarith, ssiedge, ssimarch: `crates/geom-brep/src/ssi/march.rs`
- topo: `crates/topo/src/merge_faces.rs`

Expected split, from FORK-DEFECT's sampling (the designers' reports are
in PR 5411's history): most `topo` / `geom-brep` sites are result checks.
Under the rule these are bugs, so they panic unless a correct kernel can
reach them, in which case they take the limit ending. The audit should
say which, site by site, with the why-message's citation for each panic.
