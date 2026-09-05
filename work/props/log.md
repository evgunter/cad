# PROPS log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/props/plan.md`. A/B band 2400–2499
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose PROPS section is the
charter this plan restates. Opens at S-CERT's exit. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `contribution-bounds-via-dual-interval` from `work/m10/`
- `k-stats-escalation-channel-and-redo` from `work/m10/`
- `three-per-node-verdict-shapes` from `work/m10/`
- `certified-lane-non-real-contract-audit` from `work/m10/`
- `m6-sense-gate-recorded-residuals` from `work/issues/`
- `span-carries-its-knot-vector` from `work/issues/`
- `lily-authoring-needs-shadow-vector-algebra` from `work/issues/`
- `interval-orthonormal-basis-sign-hull` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Opened for work early (2026-09-05)

New orchestrator. **The inheritance gate has NOT fired**: S-CERT is live
with CERT-M3 ([#1877](https://github.com/evgunter/cad/pull/1877), CI red
on one eps=1e-6 job, dual review in flight), CERT-N3
([#1879](https://github.com/evgunter/cad/pull/1879), green, in review)
and the ChartRegionLane `[ev]` ruling
([#1878](https://github.com/evgunter/cad/pull/1878), on hold by its own
orchestrator's comment); no exit walk exists; twenty-seven S-CERT issues
still sit in `work/cert/` (one, `nurbs-net-point-map-helper`, is a stale
`review` row — PR 1742 merged — and is S-CERT's to close). A prior
claim that "the work props waits on from cert is done" was wrong on
these facts and is recorded here so it is not re-derived.

Ev's direction (in-chat, 2026-09-05): start the work that does not
overlap those PRs' files, and watch them. The file-disjointness argument
and the resulting lane split are in `plan.md` §Early lanes. PR
subscription from this box failed through both tools; the S-CERT PRs
are watched by scheduled check-ins instead.

**This box.** Single-orchestrator remote container: GitHub through the
MCP tools, no `gh`, no away-channel monitor, lanes are Agent-tool
worktrees under `/home/user/lanes/<lane>` with private
`CARGO_TARGET_DIR`s at `/home/user/<lane>-target`. The orchestrator
branch is the session's designated `claude/props-orchestrator-review-x1voda`
rather than `props/orchestrator` (FILLET's precedent); unit branches
keep the `props/` prefix.

Decisions taken unilaterally at opening:

- **The ninth item folded into the plan**: `sphere-flux-arm-refuses-partial-bands`
  arrived from VERBS' sweep after the opening entry's list of eight;
  it joins the sphere polar lane on the same `fn sphere`.
- **PROPS-1 bundles two respells** (`mirror_across_plane`,
  `reject_from`) because both move `f64` bits and both owe the same
  golden / k-lint / render accounting — one re-baseline pass, not two.
  `lerp` is decided and LEFT (its endpoint asymmetry is documented and
  deliberate; the `Interval` cost gets a sentence at the site). Member
  5 (`rotation_about`'s diagonal floor) is filed as its own item,
  `rotation-about-diagonal-width-floor`, because it respells every
  rotation for a measured sixth of the residue.
- **The Span ruling goes out now** as an `[ev]` PR with recommendation
  A; its sweep waits for CERT-N3's `spline/` edits.
- **Block PROPS-B1 is drawn** and recorded branch-side on
  `props/b1-block` (a block record naming unstarted slots is a
  reviewer-visible leak); pre-draw difficulty for PROPS-1: M. Ordinal
  2400 claims at review dispatch on main.
- **Territory notice** to the S-CERT orchestrator on #1879: the linalg
  lane runs on `geom-core/src/linalg/` files no S-CERT PR touches; the
  request to hand `normalize-overflow-yields-zero-axis` over early.

PROPS-1 dispatched on `props/1-linalg-lost-correlation`; spec
`docs/PROPS-1-SPEC.md` on this branch.
