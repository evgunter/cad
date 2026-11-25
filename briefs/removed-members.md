# Lane `heldrecords/removed-members` — P1

Row: `work/heldrecords/a-read-record-is-consumed-by-any-removed-remainder-so-a-lineage-gap-drops-it-silently.md` (origin/main). Ruling: Ev on PR 7727 (2026-11-25), the README carry clause now on main (`crates/topo/README.md` tier 3′): every division names the cells the op removed; an unlanded record is read against them (same census confirm as kept pairs) and consumed only where it lands on them, otherwise refuses `Stale`; `Remainder` goes away. Details ruled: **(1a)** blend and shell claim no partial removal (a record landing nowhere on a support face or opened face refuses `Stale`); **(2a)** "moved" consumes only where the whole cell moved (kept or substituted), never one piece of a division. Branch `heldrecords/removed-members` off origin/main; PR base main. Review tier: dual (two Opus) — it changes the carry every op shares.

Build:
1. `Division { pieces, minted, removed }` (crates/topo `lineage.rs`), `Remainder` deleted; `Fate::removes` reads whole removals only.
2. The boolean (`boolean/ops.rs` `operand_fate`, `imaged`, `fate_of`): keep the discarded fragments as removed members in a body the confirm can read (the operand clones hold them; check `census::RecordConfirm::of` accepts a mid-op body, else read removed pairs in the clone and kept ones in the result, mapped by `back`).
3. `records.rs` `landings`: kept pair → carried; only removed members → consumed; neither → `Stale`; the parted case (PR 7489) unchanged; "moved" per (2a).
4. Shell (`shell.rs` `ShellNaming::lineage`, `opened`, dead seams/edges/vertices) and blend (`sweep/src/blend/naming.rs`): no partial removal, per (1a); fix the shell's use of `pieces` as "candidate places".
5. `carried_through`'s docstring ("never dropped because no witness turned up") becomes true; say so.

Measure what moves (the row's own next step): every boolean and surgery test whose records were consumed through a removed remainder — list each as still consumed (touch on a removed member) or now refusing (a gap). Expected: `blend_carries_records::a_record_on_a_support_face_is_consumed` (a record touching nothing) becomes a `Stale` refusal; `one_record_carry::a_lineage_gap_beside_a_removal_refuses_unmapped` is rewritten. If a body we believe valid moves to refusing, stop and report before changing it.

Pins: the PR 7593 pose (a lineage gap beside an unrelated removal) refusing `Stale` on a hand lineage; a real removal consumed (`a_pit_under_a_carried_rest_consumes_it`); (2a) a moved piece of a division not licensing a drop; mutants (consume on any removal, as main; ignore removed members) going red. Note on `work/band/a-blend-refuses-a-touch-in-its-strip-so-no-real-touch-reaches-its-consumption.md`: if it admits a touch, the blend must build its strip as a cell (ruling 1b).

Evidence is local (Ev, 2026-11-23). Standing rules: `briefs/common.md`. Report to `session_018u3gVKp4QiofNJ9Z1mzgD8`.
