# S-CERT log — certified-enclosure soundness

Narrative record; the plan is `docs/S-CERT-PLAN.md`, the charter
`docs/WORK-STREAMS-2026-08.md` §S-CERT. Convention as in the other
programs: seam entries at pipeline seams, unit entries at merges, the
tail is the live state.

## Opening state (2026-08-29)

Opened on Evan's direction (in-chat: "can you orchestrate its
program", quoting the charter line naming #723/#893, interval-mode
widening, unmetered enclosures, the offset_fit family, and SMELL
tracks M/N), by a fresh orchestrator on a remote container. The plan
is a DRAFT design conversation for its **Rulings sought** section;
CERT-1 is dispatchable pre-ratification as a charter-named defect
fix (recorded below as a unilateral decision).

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `cert/`** — unit branches
  `cert/<unit>-<slug>`, orchestrator branch `cert/orchestrator`
  (the prefix is the merged cut's own designation; the
  harness-designated session branch `claude/s-cert-orchestration-2eafta`
  carries the opening PR and is otherwise unused).
- **A/B ordinal band: S-CERT = 600–699**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in the same commit that
  opens this program, per that entry's rule. Implementer blocks are
  named `CERT-B1, CERT-B2, …` (`CERT-<n>` are unit names).
- **This session runs in a remote container** (the M10/GUI
  precedent): no persistent `~/.local/share/cad-work`, no script
  monitors (PR watching via MCP subscriptions + scheduled self
  check-ins; away-channel etiquette by hand under the `(S-CERT
  orchestrator)` tag), GitHub through MCP rather than `gh`. Disk
  ~29 G free is the binding constraint: lanes are worktrees sharing
  one object store, own `CARGO_TARGET_DIR` each, ≤ ~2 concurrent
  lane targets, review targets reclaimed at report time. The
  build-slot mutex, per-lane target rule,
  CONFLICTING-means-silent-CI, and push-early rules bind unchanged.
  The clone arrived SHALLOW; unshallowed with a blob filter at
  opening (a successor here should check
  `git rev-parse --is-shallow-repository` before trusting ancestry
  or merges).

**Sweep at opening** (beyond the charter itself, what the slate is
grounded in): #762's headline guard already landed on main at
`91164e3b` (`ssi.rs:991` refuses non-finite; the issue's residue —
`march.rs`'s sibling D285, D286's coverage loss, the NaN-fold and
`exhaust.rs:285` rewording — is CERT-2); PCURVE P-2 (#1177) carries
the #1157 `orthonormal_basis` fix written and measured, so the
keep-out concretizes to `vec.rs`; #723's mechanism confirmed live in
the tree on both sphere arms (the rimless instance measured in the
issue's fourth comment); VERBS-SPHSPH staged behind CERT-1
(VERBS-PLAN item 9); `props/quad.rs` consolidation (C3/C-m, D30)
stays Track R's, gated behind #723; #883 stays parked (reserved lane
H-f); the #723 reproduction artifacts died with their machine and
the fixtures are re-derived from the issue text.

**Unilateral decisions at opening** (per the orchestration memory's
log rule):

1. CERT-1 dispatches pre-ratification. Ground: both issues are
   named in the charter Evan handed over in chat; the fix shapes
   are the issues' own recommendations (#723 option (2); #893's
   three asks); VERBS is staged behind it. The one design-flavored
   part — the rim lever's shape near the poles, S82's reserved
   verdict line — is stated in the spec with a recommendation and
   flagged for Evan at plan ratification; if the ruling goes the
   other way the lever change is local and the failing rows keep.
2. The opening PR rides the harness session branch rather than
   `cert/orchestrator`, to respect the harness branch designation
   for this session's own pushes; unit lanes use `cert/` per the
   cut. If Evan prefers the orchestrator branch spelling, it is a
   rename at the next seam.
