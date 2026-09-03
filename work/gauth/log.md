# GAUTH — part authoring in the GUI: the log

The plan is `docs/GAUTH-PLAN.md`. The tail of this file is the
program's live status.

## 2026-08-31 — program opened

Scope ruled by Ev in-chat: Phase A and Phase B of the
part-creation survey (run in the same conversation) definitely;
fillet/chamfer authoring and assembly instance authoring wanted.
The survey's findings are restated as the plan's gap section; the
plan's unit specs are the elaboration.

**A/B band 900–999 claimed** in `docs/MODEL-AB-LOG.md` in this
opening commit. Blocks `GAUTH-B<n>`; difficulties pre-logged in
the plan (GAUTH-1 L; GAUTH-2/3/4/5 M) before any draw.

**Unilateral decisions at opening (Ev reviews retroactively):**

1. **New-document identity** (GAUTH-1): id authored at creation —
   `DocumentId::derive(name)` from a name the New form requires.
   Deterministic, matches the corpus spelling, collision refuses
   loudly at workspace resolution. Alternative not taken:
   `random_document_id` (a New door should not mint what a user
   cannot re-derive).
2. **`PlacedUnion` authoring out of scope** (GAUTH-4): rides a
   later unit once Pattern's form settles the vocabulary.
3. **Program shape**: five units, two waves, three concurrent
   lanes (Ev's sizing for this machine).

**Environment adaptations, recorded once (this program runs on a
remote single-orchestrator box, not the mngr fleet):**

- No away-channel and no usage-watch monitors: there is no `gh`
  CLI and no sibling orchestrator here; GitHub goes through the
  session's MCP tools, Ev is present in the driving chat, and
  usage alerts have no agent-dir to resolve against. Questions to
  Ev ride the chat and design-conversation PRs as usual.
- Liveness and check-ins run off the session's own scheduling
  (subagent completion notifications plus timed self check-ins)
  in place of `hourly-checkin.sh`.
- Disk is the binding resource (~29 G free at opening): lanes are
  created with `new-lane.sh` as usual, heavy builds keep the
  machine-wide slot mutex, and a lane's `target/` is swept as soon
  as its report is in hand.
- Commits authored in this session carry no model-naming trailer
  (the session's own posture and the blinding rule agree here);
  implementer briefs restate the blinding half as always.

## 2026-08-31 — GAUTH-1 delivered (PR #1375, merged)

The five creation doors landed: NewDocument / datum forms / template
profiles / extrude / revolve, each one committed `InsertNode`
through the existing commit door, ring acceptance pinned `bit_eq`
against the committed gallery fixture. Dual review A-W-F/A-W-F,
zero MAJORs; the union fix pass took all 14 items. Notable rulings
en route: `Open` now refuses mid-gesture like `NewDocument`
(deliberate change from GUI-3's silent clear); the bore form guards
bore ≥ radius in chrome while containment stays the one role rule.
Residue owned by issues: #1374 (face-frame placement arm), #1384
(id-reuse aliasing class in tool/selection state — demonstrated),
#1385 (chrome-coverage gap: the CI archive builds without
`--features app`), #1386 (session.rs accretion, cross-unit, all
four wave-1 reviewers independently). Ledger row: ordinal 900,
sample #71.

## 2026-08-31 — GAUTH-3 delivered (PR #1376, merged)

The Add part… door landed: instance authoring against the open
file's own directory, pin minted at commit, typed refusal ladder,
faults badging from the authored path, and a mated two-instance
assembly round-tripping to Certified. Dual review APPROVE +
A-W-F, zero MAJORs; all 11 union items taken, including unifying
store access through `DirResolver::workspace()` and the
filename-sorted chooser. Residue owned by issue #1387 (the memo
key hashes id+pin only, so `Reevaluate` cannot observe any store
change — the honest sentence now sits at `Save`'s docs; plus the
save-as seam rebind and the chooser's missing part-vs-assembly
vocabulary). Ledger row: ordinal 901, sample #72.

## 2026-08-31 — GAUTH-2 delivered (PR #1381, merged)

Edges are pickable: one new layer-2 naming door (`boundary_names`),
a screen-space pick seeded by the face pick with an occlusion
re-check, selection/hover/highlight riding the shared resolution
path, and the GQ7 pick-priority clause's first concrete instance
(`EDGE_PICK_RADIUS_PX`, one home). The dual found two MAJORs the
fix pass closed red-first: the GPU-vs-ray disagreement diagnostic
fed an edge name (phantom verdict every edge hover), and global
edge priority made whole faces UNREACHABLE for the mate tool —
fixed by `PickKinds` through the one priority door, mate asking
faces-only; issue #1379 carries the corrected measurements and
GQ7's still-open filter vocabulary. Issue #1395 owns the
window-bookkeeping class (seven edge/patch twins) before a third
entity kind mints more. Ledger row: ordinal 902, sample #73.

## 2026-08-31 — GAUTH-4 delivered (PR #1397, merged); block B1 closed

Phase B landed: boolean/split/transform/pattern doors over
role-typed seats, and the modal tools restructured into one
`Tools` value whose one-open rule is structural and whose routing
is exhaustive on `ToolKind` — the shape GAUTH-5's blend tool plugs
into. The fix pass caught a semantic regression the textual main
merge hid (the mate tool's faces-only pick narrowing deleted by
the migration; restored structurally with a row). `denotes_body`
now carries an evaluator-tripwire row with Sweep as the named
exception. Residue owned by issue #1394 (a split's side and a
pattern's instance cannot be named as operands — the authoring
vocabulary's next wall) and the #1386 conversation (which gained
the app.rs and Tools-shape data points). Ledger row: ordinal 903,
sample #74. Block GAUTH-B1 concluded balanced.

## 2026-08-31 — GAUTH-5 delivered (PR #1407, merged); PROGRAM CLOSED

The blend tool landed: canonical-by-construction edge accumulation,
both blend doors through the canonicalizing constructors,
`EdgesOnly` narrowing, the all-edges affordance with its
loading-is-not-a-promise honesty section, and the #217 freeze note
in chrome. The dual found three MAJOR-grade seams, every one
demonstrated by execution, and the fix pass closed each red-first:
the same-frame notice erasure (frame policy now owns notice
ranking), the per-node all-edges load breaking the tool's own
one-target invariant, and strand-blind reconcile. R2's review was
interrupted by a usage limit after verification and delivered from
held state on resume — flagged in the ledger row, pair counts.
Ledger row: ordinal 904, sample #79.

**The program is CLOSED per Ev's in-chat ruling (2026-08-31): no
exit walk.** That ruling is this program's done-state of record in
place of a ratified walk. All five units merged: the GUI authors
parts from nothing (GAUTH-1), picks edges (GAUTH-2), places pinned
instances (GAUTH-3), combines bodies (GAUTH-4), and blends edges
(GAUTH-5). Residue, each with a durable home: #1374 (face-frame
profile placement), #1379 (GQ7's filter vocabulary, two data
points in), #1384 (id-reuse aliasing class in tool state), #1385
(chrome-coverage gap — the CI archive builds without
`--features app`), #1387 (Reevaluate cannot observe store
changes), #1394 (split sides / pattern instances not nameable as
operands), #1395 (pick window-bookkeeping class), #1386 (the
session.rs/app.rs/Tools accretion split conversation, four data
points in). Blocks B1 concluded balanced, B2 closed short (records
in docs/MODEL-AB-LOG.md).

## Tracker migration (2026-09-03)

The plan and this log moved here from `docs/GAUTH-PLAN.md` /
`docs/GAUTH-LOG.md`. The program is closed (Ev's ruling, 2026-08-31);
no live items. The slate view is `work/STATUS.md`.
