---
id: origin-anchored-fan-oracle-copies
kind: issue
title: Origin-anchored fan/volume oracle copies remain across test and doc templates - post-issue-1362 enumeration
status: open
opened: 2026-08-31
github: 1401
refs: [1362, 1389, sign-asserts-on-structurally-zero-measurements]
track: W
---

## From GitHub issue 1401

Opened 2026-08-31; 0 comments.

(S-MESH orchestrator) Filed from MESH-1's dual review ([#1389](https://github.com/evgunter/cad/pull/1389), issue 1362). The unit fixed the kernel fold and three named template sites, but review enumeration (executed, two independent sweeps) found the copy population is larger, in two spellings:

**`dot(cross)` spelling** (the unit's own pattern should have matched these): `crates/sweep/tests/review_m2_pr5.rs:96`, `:511`, `:1173` — three origin-anchored divergence fans, with `my_signed_volume` at `:86` a fifth private copy of the oracle; `crates/sweep/tests/review_m2_pr4.rs:185` (the fourth copy the PR itself named and routed here).

**Scalar-determinant spelling** (invisible to every lexical `cross` pattern — the blind spot the PR's list omitted): `docs/GUIDE.md:933–946` (EXECUTED by `test_guide.py`, and divergent from its repaired twin `docs/guide/meshing.md` — #1389's fix pass takes this one), `crates/stl/tests/review_m2_pr7.rs:121–129` (doc-comment says "divergence fan about the origin"), `crates/viewer/tests/scene_build.rs:33–49`, `crates/viewer/tests/review_gui0_r2.rs:437–439`.

All are near-origin-harmless today; the hazard is the copy-source shape (issue 1362's argument) plus, per issue 1396, any of them that backs a sign assert on a symmetric fixture. Line numbers are as of #1389's frozen head `f0618c8e`.

The ask: convert each to a local-anchor spelling or record at the site why the naive fold is load-bearing (several are deliberate negative controls of the class — those stay, with their existing prose). Fence note: `sweep`/`stl` tests are T/W-seam ground and `viewer` is unfenced; single-file one-line conversions travel fine with whatever unit next touches each file, and S-MESH will take the mesh-adjacent ones.

## Home

Code quality: a duplicated-spelling enumeration spanning `crates/sweep`, `crates/stl` and `crates/viewer` tests plus `docs/GUIDE.md` — the T/W-seam ground the code-quality register's tracks carry, with no single program's territory covering the set.

## Claimed by S-TCOST (2026-09-04)

Moved from `work/code-quality/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which claimed
**Track W whole** for S-TCOST. Id, `track: W` and body unchanged.

The letter and the program are the same ground: W's fence is
`crates/*/tests/` (all crates) and `crates/test-utils/`, and S-TCOST's
`paths` are `crates/*/tests/*` and `crates/test-utils/*` — an exact
match. `work/code-quality/plan.md` recorded W's claimant as "ground is
`tcost`'s" without the rows ever moving, and no `smell/w-*` lane has
ever run, so the rows were waiting on an owner rather than on a ruling
or a dependency.

**This claim was made on Ev's direction rather than by the S-TCOST
orchestrator**, which is not how `work/README.md` expects a claim to
happen. If S-TCOST would rather not hold a row, moving it back is a
`git mv` and this section is the record of why it arrived.

The two seams `work/code-quality/plan.md` states for W are unchanged
and travel with the rows: `crates/test-utils/src/source.rs` is the
shared home three tracks' rows land in, and the `UNCONVERTED_TODAY`
ceiling is **re-derived from the table at each landing, never lowered
by a row's own member count**; and a W row whose mechanism reaches into
a crate's `src/` is **filed on the owning track** rather than edited
there.
