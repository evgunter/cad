# S-MESH log — mesh honesty and budget

Narrative record; the plan is `docs/S-MESH-PLAN.md`, the charter
`docs/WORK-STREAMS-2026-08.md` §S-MESH. Convention as in the other
programs: seam entries at pipeline seams, unit entries at merges, the
tail is the live state.

## Opening state (2026-08-31)

Opened on Evan's direction (in-chat: "pick up S-MESH as the
orchestrator", with S-BOOL taken by the same instruction), by a fresh
orchestrator on a remote container. The plan is a DRAFT design
conversation for its **Rulings sought** section; MESH-1 is
dispatchable pre-ratification as an inherited defect fix whose shape
#303's merged unit established (recorded here as a unilateral
decision).

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `mesh/`** — unit branches
  `mesh/<unit>-<slug>`, orchestrator branch `mesh/orchestrator` (the
  harness-designated session branch `claude/s-mesh-orchestrator-7o6gjc`
  carries the opening PR and is otherwise unused, per the
  S-CERT/S-QA precedent). The remote's dormant `mesh/*` branches are
  pre-program #284-era work, not this program's.
- **A/B ordinal band: S-MESH = 900–999**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in this same commit, per that
  entry's rule (S-QA holds 800–899; 900 was the next free band at
  claim time). Implementer blocks are named `MESH-B1, MESH-B2, …`
  (`MESH-<n>` are unit names). **S-BOOL = 1000–1099 is claimed in the
  same commit** (same orchestrator, `docs/S-BOOL-LOG.md`).
- **This session runs in a remote container** (the S-CERT/S-QA/M10/GUI
  precedent): no persistent `~/.local/share/cad-work`, no script
  monitors (PR watching via MCP subscriptions + scheduled self
  check-ins; away-channel etiquette by hand under the `(S-MESH
  orchestrator)` tag), GitHub through MCP rather than `gh`. Disk ~28 G
  free is the binding constraint: lanes are worktrees sharing one
  object store, own `CARGO_TARGET_DIR` each, ≤ ~2 concurrent lane
  targets shared with S-BOOL, review targets reclaimed at report time.
  The clone arrived shallow; unshallowed with a blob filter at
  opening.

**Sweep at opening** (beyond the charter, what the slate is grounded
in): #320 and #782 are resolved on main and want closing
(orchestrator-direct, after verifying the pins at HEAD — TESS-SPAN
#594 + TESS-SPLIT #951 closed both #320 halves with #950 the
scheduled residual; #782's table re-pinned green and its CI arming
landed, with `docs/VERBS-PLAN.md` already recording "wants closing").
#881 is half-landed at #894 (the named-operations half remains, per
the reopen comment). Inherited from S-CERT by name: #1362 and the
`closing_column` note. `walk.rs` is contended by #1362/#896/#881/#868
— sequenced, never fanned out. S-CERT is live (CERT-6 in flight, then
CERT-8/CERT-10/CERT-M/CERT-N): `props/quad.rs`/`patch_bound.rs`/area
lanes and the tess-budget re-baseline stay its until its slate closes,
so C3/D30/C23 wait on CERT-10 and S26 on CERT-6. Track R table
corrections ride the opening PR (count re-derived after D304's
arrival; C3/D30's discharged #723 gate; D302 deleted with members
relocated — Display landed at `types.rs:271`, consumer half is
#1111's/LIB's, Track U's D47 unblocked for the type).
