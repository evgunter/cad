---
id: the-name-gate-is-tested-as-a-table
kind: issue
title: The name gate is tested as a table: every op sequence, holder and chain position, asserting no drop, no false live, and resolution after the landing
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Filed by the DOORS orchestrator on PR 6264's re-verify 5 (`~/.local/share/cad-work/doors-takeover/scratch/reverify-5.md`, "The gate unit, if re-specced"). The gate is `DocSession::name_run` (`crates/viewer/src/session.rs`): the one place a held name is judged. It was reshaped in four fix passes of PR 6264, and each shape had a fresh hole that a single probe found (rv2, rv3, rv4, rv5). Each pass closed only that probe's cell. The repair is the property table in point 6.

## The gate unit's spec

1. **One provenance type. DONE in PR 6264.** `Answered { evaluation, doc, resolver }` (`crates/editor-core/src/take_over.rs`) has private fields and one constructor, `Answered::evaluate`, which evaluates. The viewer's `Seed` (`crates/viewer/src/evalseam.rs`) wraps an `Answered` and has one constructor, `Seed::of`. No value pairs an evaluation with a document it is not a run of.
2. **One question. DONE in PR 6264.** `name_run` answers only with a run of the shown document: the landed run, where its document is the shown one (generation, or value equality at the request), or a takeover door's run whose `Seed::answers` equals the requested document. It never picks a run because it is newer. The resolver clause is NOT done: the seam's assertion (`run_once`) checks a seed's resolver against its request's, but `name_run`'s Landed arm decides by document equality and never compares resolvers.
3. **A verdict carries provenance:** (the pick, the document it was judged in). The chrome may show a verdict judged in the shown document, or the last verdict of that exact pick. PR 6264 carries a followed pick's last verdict over to the followed name where `moved` names it (`DocSession::follow`, with the argument in its doc). Whether that carry is shown "as of an earlier state" is open (below).
4. **Destructive consumers act only on a shown-document verdict:** the tool survival drop, `slot_rows` gating, and any refusal. An abstaining or carried verdict never drops, refuses or offers.
5. **`Moved` is computed from a run that evaluated every body a reader holds,** document or outside. PR 6264's door loops while any reader's body lacked a table: a document site's (`rebinds_of`'s `unchecked`) or a held chain node's. NOT done: a holder whose move is unknown should be told so (pending), not left on its old name; today it keeps its old name and last verdict until the landing judges it.
6. **The test obligation is a table, not instances.**
   - Rows: op sequences over {commit with a takeover (door evaluated / not), commit without one, label, undo, redo, cancel, land}, up to length 3.
   - Columns: holders {selection, form latch, mate pick, mirror plane, blend edges} on {the copy, a copy of the copy, a mirror link whose plane reads the copy, a downstream feature body}.
   - Assert after every step:
     - no drop unless a shown-document run judged the pick stale;
     - no "live" unless a shown-document run, or the same pick's prior verdict, gave it;
     - after the final landing, every holder whose entity has a single successor resolves.
   - The rv2, rv3, rv4 and rv5 probes (`crates/viewer/tests/takeovers.rs`, `crates/editor-core/tests/takeover_door.rs`) are individual cells of that table.

## Open question

**A followed pick reads "pending" instead of carrying its verdict.** PR 6264 carries a followed selection's last verdict onto its new name, which the argument on `DocSession::follow` holds true. The alternative re-verify 5 names: a followed pick starts with no verdict ("pending") until a run of the shown document judges it, so nothing the chrome shows rests on an argument rather than an evaluation. The cost is the CHROME window: slot rows and live standing vanish for a followed selection until the landing. Decide which is wanted, and if pending, what the chrome shows meanwhile.

**Untested cell (re-verify 6, unsure):** a chain link that reads a copy the combine deletes (`Takeover::dropped`). The argument on `DocSession::follow` was not probed there.
