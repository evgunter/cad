# CIW — hosted CI, workflows and scripts (plan)

**STATUS: OPEN (2026-09-03).** Opened from
`docs/WORK-TRACKS-2026-09.md` (CIW section); this plan supersedes that
charter. Live state is `work/ciw/log.md`'s tail and the item files
beside this plan, never this file.

Branch prefix (the #396 convention): **`ciw/`** — unit branches
`ciw/<slug>`, orchestrator branch `ciw/orchestrator`. Away-channel tag
`(CIW orchestrator)`. A/B ordinal band **CIW = 1500–1599**, claimed in
`docs/MODEL-AB-LOG.md`.

## Charter

Hosted CI that reports what it ran and runs what it reports. The
territory is retired code-quality Track J's ground plus the render
lanes and the perf emitters — files no live program owns.

## Review posture

**No A/B row and no A/B protocol** (Ev, 2026-09-04). Each unit is one
PR, reviewed by a subagent against `docs/prompts/reviewer-style-lane.md`
— a style review, not a dual. A unit that moves logic subtle enough to
be worth a second opinion on correctness gets one extra reviewer for
that, named in its PR with the reason; that is a judgement the
orchestrator makes per unit and not a default. The band above exists
for the case a unit moves kernel logic, and none on this slate does.

## The 2026-09-04 re-read

The slate was audited against the tree on 2026-09-04 rather than
inherited, and six items moved. The finding that moved most of them:
**`evgunter/cad` went public on 2026-09-03**, so standard-runner minutes
are free and the runner is 4 vCPU / 16 GB (was 2 / 7). Every cost
argument in `docs/CI-MINUTES-2026-08.md` — the document opens *"the
Actions allowance was being consumed faster than the work justified"* —
now has a dead premise, and several items were costed against it. That
re-costing is a unit of its own (10 below); until it reports, no figure
from that document may be quoted forward.

## Unit order

1. `render-lanes-red-at-missing-merge-ref` — check out an object that
   outlives the ref (`github.sha`), or end the lane skipped-with-reason
   when `refs/pull/N/merge` is gone; the lane says in its log which it
   did. First because it is the largest single red source in the
   history (103 jobs in 89 runs) and none of them is about the tree.
2. `nightly-pin-reading-idiom-four-copies` — one `scripts/` reader
   anchored on ci.yml's workflow-level `env:`, refusing on a second
   match. Its `MIRROR_EXEMPT` entry is part of the unit. Second because
   the class has now fired on main (`c5263958`).
3. `retire-render-automatic-matplotlib-fallback` — Ev's ruling is in
   the body and the step order there is strict: `render.sh` fails
   nonzero first, the workflow's assert step is dropped after.
4. `hosted-renderer-announces-itself-preview-only` — a second accepted
   sentence meaning "this pass IS the canonical renderer"; Ev has
   sanctioned either spelling of the variable name and it is not worth
   review time.
5. `perf-history-cannot-identify-its-host` — host identity in all three
   emitters, same shape, READMEs after. Ahead of cheaper items because
   the runner class changed on 2026-09-03 and every sample after it is
   unattributable until this lands.
6. `geom-brep-test-unused-edgedescription-import` — CIW takes the
   `--all-features` clippy row; the four trims are VERBS', and go
   first or together (the row's first run is red otherwise).
7. `mirror-parity-never-compares-flags` — an allowlist over the small
   set of semantics-bearing flags, keeping the existing "declare your
   asymmetry in a sentence" shape rather than adding a second
   mechanism.
8. `f3-recosting-on-a-public-repo` — measure, then ask on an `[ev]` PR.
   Gates 9 and 10.
9. `doc-gate-two-unread-axes` — axis (a) differential run or accept the
   hole once; axis (b) needs 8's numbers before a `--release` pass can
   be argued either way. Carries the contributor-facing sentence
   inherited from the closed `rustdoc-gate-disagrees-with-workspace-doc`.
10. `facade-guards-defer-to-rustdoc-json` — an `[ev]` ruling, asked with
    8's number in it. The format-instability half is the real question;
    the cost half is now small.

`nightly-demotions-have-never-run` is not in the order: it is read from
tonight's scheduled nightly (Ev, 2026-09-04 — do not force a dispatch),
and if any of the three demoted rows reds, the repair jumps the queue.

`rustdoc-gate-private-intra-doc-links` stays open on its stated trigger
(a public-only doc set, or Q9) and is not dispatched. Note the trigger
moved closer on 2026-09-03: the repository is public, though nothing
publishes a doc set yet.

`cache-rendered-cells-on-input-hash` is parked on
`work/tcost/rust-cache-never-restores-across-branches` — its design
needs no revision and should be reused as-is when it unparks.

## Closed at the 2026-09-04 re-read, with the reason in each file

- `main-latently-red-at-tier-all` — neither failure is live. The pyo3
  half was fixed at `5859c8c6`; the viewer bin/lib doc collision is a
  **cargo** diagnostic, not a rustdoc one, so `-D warnings` cannot
  reach it and `scripts/doc-gate.sh --pr --scope '--workspace'` is
  green on this tree (run at closing). Its class half became unit 8.
- `rustdoc-gate-disagrees-with-workspace-doc` — answered by
  measurement: the two halves document different feature selections,
  not different verdicts. Residue folded into unit 9.
- `sccache-trial-verdict-to-read` — PR 1648 merged.
- `committed-conflict-markers-reach-main` — Ev, 2026-09-04: a committed
  marker is self-limiting (obvious, repairable later, nothing compounds
  on it), which makes it a poor subject for an absence detector.
- `python-suite-zero-test-guard-three-copies` — Ev, 2026-09-04: never
  observed, and the fix moves a developer tool's contract and a parity
  seam. The guard itself is present and correct at all three sites.

## Re-homed at the same re-read

- `bounds-tripwire-blind-to-named-alias` → `work/code-quality/`. The
  tripwire moved to `scripts/gates/bounds-allowlist.sh` (Track K's, and
  in this program's `keep_out`), and that gate's ratified header now
  argues against the ask as KNOWN GAP 3, with a fixture pinning it.
- `d107-release-profile-job-lives-in-nightly` → `work/code-quality/`.
  The whole fix is an edit to a Track P finding's disposition.
- `rust-cache-never-restores-across-branches` → `work/tcost/` (filed
  new, from PR 1648's finding (d)). Caches are a build knob and this
  program's `keep_out` gives them to S-TCOST.

## Fences

- Track K keeps `scripts/gates/*` and `tools/*`; the
  `clippy-panic-gate-blind-in-macros` / `gated-marker-*` items are K's
  and S-TCOST's and stay in `work/issues/` for them.
- S-TCOST keeps its three scripts and the CI build knobs — profile,
  cache and sharding — measured in-unit or not at all. Unit 8 cites
  S-TCOST's cache measurement; it does not fix it.
- Absorbing Track K's remaining gate rows when its live lane finishes
  is an option the proposal names and this plan does not take.

## Exit shape

The ten units above land and unit 10's ruling is answered; the walk
convention applies. Residue re-homes before the sweep.
