---
id: door-table-reason-strings-are-unguarded-prose
kind: issue
title: Read each reason string in review_m1_pr5_internal's ALLOWED table against its door's body, and fix the wrong ones
status: open
opened: 2026-09-19
priority: P4
cost: E
---

`crates/topo/src/review_m1_pr5_internal.rs:237-459` holds `ALLOWED`, a
list of `(door, why tier 1 survives it)` pairs. Two guards read it,
`every_public_mutation_path_preserves_tier1` (`:527`) and
`the_two_door_tables_cover_the_same_surface` (`:655`). Both look only
at the door name (`:552`, `:587`) and never at the reason string. A
reason has been wrong once: `cyl_wall_sheet`'s reason left out
`set_surface_source` and `mint_pcurves`, and PR #2887 fixed it on
2026-09-19. Two of about 25 reasons have been checked against their
doors.

The work is a reading. For each entry, check that every door the
reason names in backticks is one the body calls, and that every
non-asserting door the body calls is named. Correct each reason that
fails, together with the section comment above the test-support
builders. Report the count either way, including zero. Do not add a
test that compares reason strings with door bodies. A check like that
would red whenever someone edits a door body until the prose is
updated. The class has one recorded instance, and that was a stale
reason, not a broken tier-1 guarantee, so the extra fix cycles it would
cause cost more than it would catch.

What would close it: every ALLOWED reason has been read against its door, the wrong ones are corrected, and the count is recorded on the closing PR.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): The `ALLOWED` table (review_m1_pr5_internal.rs:237-459) is read only by name (:552, :587). The one recorded wrong reason (cyl_wall_sheet) was fixed on PR #2887 on 2026-09-19, and nothing has been recorded since. The proposed reason-vs-body test would red on every door-body edit until the prose catches up, so it would cause fix cycles without having caught anything yet. Keep only the reading. See reform/door-table-reason-strings-are-unguarded-prose.md.
