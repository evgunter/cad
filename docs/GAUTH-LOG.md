# GAUTH — part authoring in the GUI: the log

The plan is `docs/GAUTH-PLAN.md`. The tail of this file is the
program's live status.

## 2026-08-31 — program opened

Scope ruled by Evan in-chat: Phase A and Phase B of the
part-creation survey (run in the same conversation) definitely;
fillet/chamfer authoring and assembly instance authoring wanted.
The survey's findings are restated as the plan's gap section; the
plan's unit specs are the elaboration.

**A/B band 900–999 claimed** in `docs/MODEL-AB-LOG.md` in this
opening commit. Blocks `GAUTH-B<n>`; difficulties pre-logged in
the plan (GAUTH-1 L; GAUTH-2/3/4/5 M) before any draw.

**Unilateral decisions at opening (Evan reviews retroactively):**

1. **New-document identity** (GAUTH-1): id authored at creation —
   `DocumentId::derive(name)` from a name the New form requires.
   Deterministic, matches the corpus spelling, collision refuses
   loudly at workspace resolution. Alternative not taken:
   `random_document_id` (a New door should not mint what a user
   cannot re-derive).
2. **`PlacedUnion` authoring out of scope** (GAUTH-4): rides a
   later unit once Pattern's form settles the vocabulary.
3. **Program shape**: five units, two waves, three concurrent
   lanes (Evan's sizing for this machine).

**Environment adaptations, recorded once (this program runs on a
remote single-orchestrator box, not the mngr fleet):**

- No away-channel and no usage-watch monitors: there is no `gh`
  CLI and no sibling orchestrator here; GitHub goes through the
  session's MCP tools, Evan is present in the driving chat, and
  usage alerts have no agent-dir to resolve against. Questions to
  Evan ride the chat and design-conversation PRs as usual.
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
