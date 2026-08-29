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
- **A/B ordinal band: S-CERT = 700–799**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry, per that entry's rule.
  The opening commit claimed 600–699; S-BLEND opened concurrently,
  drew the same band, and its claim reached main first, so S-CERT
  renumbered to 700–799 under the main-is-authority tiebreak
  before any ordinal was assigned (this is the corrected log the
  banding entry says a collision costs). Implementer blocks are
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

## Seam: first rulings in from the opening conversation (2026-08-29)

Evan, in-chat: **Q3 RULED** — not a design question, orchestrator's
call; CERT-2 and CERT-4's fence seams proceed as planned. Q1/Q2/Q4
got elaboration requests (answered in-chat; outcomes fold into the
plan when ruled). Alongside Q2 Evan stated the general bar — a bit
change ≪ ε is always acceptable when it buys cleaner code — now
recorded in `memories/output-stability-as-justification.md`.
Consequence for CERT-4: the interval-lane-only reformulation stays
the default because #1191's exact-fit rows ride a *structural*
bit-zero (`extent − setback`), which is not ≪-ε drift; if the unit
finds the both-lanes rewrite cleaner it must bring the re-derived
gate design back for a look, not just re-baseline. The PCURVE
orchestrator's PR answers (route 2 unclaimed; vec.rs keep-out
time-boxed to PR 1177; correlated-Interval sites to the 1143 audit)
are folded into the plan at 715a7bb8.

CERT-2 dispatches on Q3's ruling (spec on `cert/orchestrator`;
block CERT-B1 slot 1). CERT-1 lane still running.

## Seam: Q2 and Q4 ruled (2026-08-29)

Evan, in-chat. **Q2 RULED**: the #1006 trio proceeds (shared home,
whole-face-arm collapse — tighter or equal by per-cell-then-union —
magnitude-reading retirement with the re-baseline owned); landed as
CERT-10 in the slate, after CERT-5/CERT-7 which edit two of its
sites. The bit principle sharpened: ≪ ε was *sufficient, not
necessary* — a flipped classification is fine when semantically
correct and the code cleaner (memory updated). Consequence: CERT-4's
f64-bit constraint restates SEMANTICALLY — both-lanes reformulation
permitted if cleaner, provided the exact-fit guarantee survives by a
preserved structural zero or a re-derived gate. **Q4 RULED**:
route 1, knot-aligned composite cells primary for CERT-5 (the
w-uniform-in-v exact arm kept as the strictly-better path where it
applies; route 2 unclaimed, per the PCURVE answer). Open ruling
surface is now **Q1 only** (the #870 gauge/scope choice —
recommendation on record: A′ patch-lanes-only, mean-edge-displacement
gauge, typed refusal).

Lanes: CERT-1 and CERT-2 implementers both still running.

## Seam: Q1 ruled; plan RATIFIED (2026-08-29)

Evan, in-chat: **Q1 RULED** — no always-on area metering (the
ε-validity intent: any realized geometry everywhere within ε of
correct is valid); the check is a hefty `debug_assert` on the A2
gauge. In the same exchange Evan clarified the debug_assert doctrine
— the instrument is right for expensive checks whose failure
PROBABLY indicates a bug, not only for guaranteed ones, and they are
on in release today (`debug-assertions = true`), eventually
debug/CI-only — ratified into DESIGN.md's D2 addendum as the
row-5-boundary note in this branch. CERT-6 re-cut to the ruling
(tripwire + calibration; the opt-in tightness door filed as a
demand-triggered valve, not built). All four rulings are now in;
the plan is marked RATIFIED, with the opening PR held for Evan's
sign-off of the D9 addendum wording it carries.

## CERT-2 merged (2026-08-29) — issue 762 closed; the program's first unit

PR 1221 at f24c5dea, gate green (interval, 1e-12). The unit: issue
762's residue — march.rs's sibling guard (D285's exact signature
red-first), D286 answered with the weight-underflow fixture (better
than the anticipated none-exists verdict; the overflow route proven
closed by the hull-cancellation floor), the poison-arm sentences made
producible, the NaN fold pinned, and three D285-spelling predicate
siblings the impl sweep's fold-shaped pattern missed — found by R2,
fixed in the pass. D285/D286 left the Track Q table in the landing
PR. Issues filed: 1218, 1219 (impl sweep), 1238 (the
finite-but-unusable-speed class, from both reviews' probes). Dual at
ordinal 700, sample #44 (after correcting the #42/#43 collision on
main's ledger): R1 fable A-W-F 1/4/4, R2 opus A-W-F 2/4/4, both
headlines bilateral-at-differing-severity; details in the row.

**Two incidents, recorded:**

1. **Orchestrator error — the fix pass ran cross-slot.** The
   fix-pass dispatch was SendMessage'd to the CERT-1 lane's agent id
   instead of CERT-2's implementer; that lane executed the whole
   union (well), so the fix pass did not inherit CERT-2's arm and
   its covariates are contaminated (excluded from arm comparisons in
   the row). Rule for the successor: verify the agent id against the
   dispatch record before any fix-pass send — the ids are one
   typo apart and nothing else checks them.
2. **Main went red mid-fix-pass** (pncad-py create_exception! merge
   damage from PR 1215 — a fence this program never touches; the PR
   gate builds the merge ref, which is how it bit PR 1221 first).
   Repaired orchestrator-direct at PR 1239 within ~40 minutes, LIB
   flagged on the PR; standing-down comment posted on PR 1221 per
   the babysit rules.

CERT-1's implementation is delivered (PR 1220, green at
default/1e-12); its dual dispatches next at ordinal 701.
