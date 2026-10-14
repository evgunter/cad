---
id: swept-run-walls-style-residue
kind: issue
title: sweep/names: PR 3736's delta-review style residue — reversal home claimed twice, a stale gate premise, a second wire spelling of a one-piece run, three peel walks, an unguarded sort assumption, an unrowed refusal
status: open
opened: 2026-10-02
priority: P4
cost: E
---


The delta review of PR 3736's fix pass (the orchestrator's read after the
dual review, DR row on that PR) left six small items. None is a wrong
answer today; each is a place the next reader is misled or a guard is
missing.

1. **The reversal map claims one home twice.** `crates/sweep/src/swept.rs:266`
   ("The one home of that involution for a validated loop") and
   `crates/sweep/src/extrude.rs:180` ("The map between the two orders has
   one home") both claim it, and `swept_segments` (`swept.rs:280`) is a
   sibling spelling of the same index map. Pick one home and cite it.
2. **The persist gate's stated premise is stale.**
   `scripts/gates/persist-no-backtracking.sh:65-67` bounds its blind spot
   by "`editor-core` has six hand-written impls, all listed in
   `persist::wire`'s docs"; `PieceRun`'s hand-written `Deserialize`
   (`crates/editor-core/src/names/role.rs:786`) is a seventh and
   `persist/` does not list it. Re-count and list it, or state the
   premise without a number.
3. **A one-element array is a second spelling of a one-piece run.**
   `PieceRunVisitor::visit_seq` (`role.rs:775`) accepts `[locator]` and
   it reads back equal to the bare locator, which is the canonical form
   (`Serialize`, `role.rs:750`), so one value has two accepted wire
   spellings. `band_run_wall_names.rs:250` pins only the empty list's
   refusal. Refuse the one-element list typed (or argue it), with a row.
4. **Three peel walks in `names/merged.rs`.** `constituents_through_wrappers`
   (`:36`, its own match at `:45-46`), `peel` (`:104`) and the side rebuild
   at `:125` each walk the `FromA`/`FromB` wrapper; one `peel` should serve all.
5. **`covers` assumes a sorted set with no guard.** `merged.rs:214-216`
   `binary_search`es `set`; a loaded `Merged` set that is not sorted answers
   false silently. Assert sortedness at the load seam (or `debug_assert!` here).
6. **`RevolveError::PinnedRunStation` has no row.** Minted at
   `crates/sweep/src/revolve/partial.rs:437` (`revolve/mod.rs:606`), reached
   by no test. Add one that pins a station inside a run.
