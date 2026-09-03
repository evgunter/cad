# CIW — hosted CI, workflows and scripts (plan)

**STATUS: OPEN (2026-09-03).** Opened 2026-09-03 from `docs/WORK-TRACKS-2026-09.md` (CIW section), which is this
program's charter until this plan supersedes it. Live state is
`work/ciw/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`ciw/`** — unit branches
`ciw/<unit>-<slug>`, orchestrator branch `ciw/orchestrator`.
Away-channel tag `(CIW orchestrator)`. A/B ordinal band
**CIW = 1500–1599**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit, per that entry's rule.

## Charter

Hosted CI that reports what it ran and runs what it reports. The
territory is retired code-quality Track J's ground plus the render
lanes and the perf emitters — files no live program owns — and the
class is E throughout: each item's body already states the fix, and
none needs a design decision beyond a one-paragraph call in its PR.

## Review posture

Infra-only units take the S-TCOST posture: one PR per item, a batched
style review against `docs/prompts/reviewer-style-lane.md`, no A/B
row. The band exists for the case a unit moves kernel logic (none is
expected to).

## Unit order

1. `main-latently-red-at-tier-all` — the viewer bin/lib rustdoc
   filename collision reds every TIER=all run; rename the `[[bin]]`
   target (or `doc = false`). First, because every later PR that
   draws the docs tier hits it. Its class half (what a main push
   re-gates) is filed as an `[ev]` ruling, not fixed here.
2. `render-lanes-red-at-missing-merge-ref` — checkout by `github.sha`
   or exit skipped-with-reason when `refs/pull/N/merge` is gone. Its
   duplicate file was closed at opening.
3. `retire-render-automatic-matplotlib-fallback` — Ev's ruling is in
   the body; the step order there is strict (render.sh fails nonzero
   first, the workflow's assert step is dropped after).
4. `hosted-renderer-announces-itself-preview-only`.
5. `nightly-pin-reading-idiom-four-copies` — one `scripts/` reader
   anchored on ci.yml's workflow-level `env:`, refusing on a second
   match.
6. `mirror-parity-never-compares-flags`.
7. `python-suite-zero-test-guard-three-copies` — one shared runner;
   the parity seam moves with it.
8. `committed-conflict-markers-reach-main` — a tree-wide line-anchored
   marker grep; if it lands under `scripts/gates/` it is Track K's
   row and this program files it there.
9. `bounds-tripwire-blind-to-named-alias` — closes code-quality `D102`
   with it (one defect, two files).
10. `cache-rendered-cells-on-input-hash` — the one correctness edge is
    keying on the runner image version.
11. `d107-release-profile-job-lives-in-nightly` — closes `D107`'s CI
    half; the `kemr` visibility half stays Track P's.
12. `rustdoc-gate-disagrees-with-workspace-doc` — establish why the
    hosted gate passes topo, align `doc-gate.sh`'s flags with a
    contributor's `cargo doc -D warnings`; the one-line topo link fix
    is Track Q ground, by note.
13. `doc-gate-two-unread-axes` — a differential run across feature
    halves or accept the hole once; measure a `--release` doc pass
    before adding it.
14. `geom-brep-test-unused-edgedescription-import` — with an
    `--all-features` clippy row so the class stays visible.
15. `the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row` — a
    `--features python` clippy row on the gate and the local mirror;
    LIB clears the one standing lint in `py/value.rs`. Its duplicate
    file in `work/lib/` was closed at opening.
16. `facade-guards-defer-to-rustdoc-json` — an `[ev]` ruling: a
    nightly-toolchain rustdoc-JSON scan of `pncad`'s public API, or
    declare the three text scans permanent and rewrite their docs.
17. `perf-history-cannot-identify-its-host` — host identity in all
    three emitters, same shape; the PERF register's READMEs follow.
18. `sccache-trial-verdict-to-read` — the verdict is written and PR
    1648 is in review; merge and close.

`rustdoc-gate-private-intra-doc-links` stays open on its stated
trigger (a public-only doc set, or Q9) and is not dispatched.

## Fences

- Track K keeps `scripts/gates/*` and `tools/*`; the three
  `clippy-panic-gate-blind-in-macros` / `gated-marker-*` items are K's
  and S-TCOST's and stay in `work/issues/` for them.
- S-TCOST keeps its three scripts and the CI build knobs.
- Absorbing Track K's remaining gate rows when its live lane finishes
  is an option the proposal names and this plan does not take.

## Exit shape

The slate above lands and the two rulings are answered; the walk
convention applies. Residue re-homes before the sweep.
