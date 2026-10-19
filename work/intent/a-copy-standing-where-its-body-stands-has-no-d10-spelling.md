---
id: a-copy-standing-where-its-body-stands-has-no-d10-spelling
kind: issue
title: A copy standing where its body stands has no D10 spelling: C's world pin is a named interim until the follow-the-body fork
status: open
opened: 2026-10-18
priority: P1
cost: M
parent: transform-retires-into-a-placement
---

Owns the retirement of stage 3 C's named interim, the world pin (PR 5267, the orchestrator's ruling). The follow-the-body fork (an Opus and a Fable designer) converged: no new relation. Under D10 a body has no location; the interim is tied to B's "all roots share world coordinates" interim, and D retires both. After D a root extrude's start cap reads nothing, and a plane mate on it with three constant zeros is "own coordinates agree" in D10's own vocabulary.

**The interim.** `ConstraintKind::WorldPin(WorldPin::Identity)` (`crates/editor-core/src/mate.rs`), written by `DocEdit::PinWhereItStands`. It is a world pin with no read and no number. The fold places its copy by no map in the world's space (`eval/wire.rs`, `fold_with`). It stands alone in its bundle (`mate::solve::world_pin_stands_alone`, asked of the whole bundle by `AddConstraint`, the insert door, the load door and the fold). Split pins the instance it leaves by it and carries a part's as it is; inline lands it as it is at the identity, and under a moved landing pins its copy at the instance's motion by `pin_at` (`InlineError::WorldPinOnAMovedWorld` where the copy offers no frame). The migration (corpus `place_part`), split, the tour (`demos/tour/src/world.rs`), the demos and the fixtures (`fixture::pin` at the identity, `Recorder::place`, viewer `pinned`, pncad `place`, Python `Doc.place(shapes)` with neither constraints nor pose) place a copy at the identity by it.

**Why a mate would not do (measured on PR 5267).** `pin_at` and `place_at` read the copy's pose off its own geometry against numbers from one evaluation. A copy whose body is built in world coordinates through a variable is therefore pulled back toward the variable's nominal (revalue_corpus: `solve_copy_at_identity` flips on 7 rows), and widened by its width under a box. The tour's plate study certified nothing, and the chain's certifiable fractions fell about 3×. Preferring an anchor that no toleranced variable reaches (`mate::anchor`) left 25 of the tour's 26 copies on a varying anchor, because the tour builds its bodies through toleranced placements.

**Done when** D lands: every world pin is restated as a plane mate on a start cap that reads nothing with three constant zeros (or the pin D's migration writes), `ConstraintKind::WorldPin`, `DocEdit::PinWhereItStands`, `InlineError::WorldPinOnAMovedWorld` and the Python `WorldPin` retire, and spec §4 drops the interim.
