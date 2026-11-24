---
id: main-red-three-source-censuses-red-since-the-told-quantity-type
kind: issue
title: Main red (slow set): hand_written_impl_census (two rows) and pncad-py's prose_census are red since PR 7159 (the told-quantity type)
status: closed
opened: 2026-11-21
priority: P1
cost: E
refs: [7267]
closed: 2026-11-21
---


Found by ONEROUND's lane (2026-11-21) on main `71068072`, `cargo nextest run -p test-utils -p pncad-py --all-features --ignore-default-filter`. All three are slow-set rows, so PR 7159's gate did not run them.

## What

`test-utils::all hand_written_impl_census::every_hand_written_walk_is_held_to_its_declaration`:

    "crates/geom-core/src/told.rs:97 impl Debug for Read — destructures with a rest arm, which binds nothing and stops nothing",
    "crates/geom-core/src/told.rs:340 impl PartialEq for Quantity — reads self.dimension with no exhaustive pattern",

`test-utils::all hand_written_impl_census::the_walk_still_sees_every_file_holding_one`: "the set of files holding a hand-written `Debug`/`PartialEq` has moved away from IMPL_FILES_TODAY". The walk finds 43 files and the list holds 42; the one the list lacks is `crates/geom-core/src/told.rs`.

`pncad-py prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason`: the census no longer finds four undecided sites its list still names, all in `crates/profile/src/path/program.rs`: `("Refused", "verb"): 1`, `("Takes", "mode"): 2`, `("Takes", "target"): 1`, `("Takes", "verb"): 2`. Nothing is found that the list lacks.

## The merge

`bd8b2c9b`, PR 7159 (`scalar/told-quantity-type`): all three pass on its first parent `4d2d9942` and fail on the merge, and on every main since. The PR added `crates/geom-core/src/told.rs` and rewrote the `Refused` and `Takes` sentences in `profile`'s `path/program.rs`. Not any of ONEROUND's merges.

## Owed (SCALAR, the PR's program)

The two `told.rs` impls destructured exhaustively, as the row's message says, and `told.rs` added to `IMPL_FILES_TODAY`; the four `program.rs` lines out of the prose census's undecided list, if the sites are decided now because the sentences no longer render a payload through `Debug` (not checked here). The same merge's behaviour reds are on `work/scalar/main-red-offer-rows-false-offers-since-the-told-quantity-type.md`.

## 2026-11-21: closed (SCALAR)

`told.rs`'s `impl Debug for Read` binds `Enclosure { lo: _, hi: _ }` and `impl PartialEq for Quantity` destructures `Self { held: _, dimension, canonical_only }` (`canonical_only` came with PR 7229); `told.rs` is on `IMPL_FILES_TODAY`. The four `program.rs` sites are decided: `Refused`'s and `Takes`'s sentences spell `identifier()` and render no payload through `Debug`, so they leave `UNDECIDED` (main's `4985847554` dropped the same four lines; the merge takes it).
