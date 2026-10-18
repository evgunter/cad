---
id: a-copy-standing-where-its-body-stands-has-no-d10-spelling
kind: issue
title: A copy standing where its body stands has no D10 spelling: C's world pin is a named interim until the follow-the-body fork
status: open
opened: 2026-10-18
priority: P1
cost: M
---

Owns the retirement of stage 3 C's named interim, the world pin (PR 5267, the orchestrator's ruling). Blocked on the follow-the-body design fork the orchestrator opened with an Opus and a Fable designer: how a copy whose body is built in world coordinates stands where it is under D10, given that a placement is the only reader of a body's own coordinates and a mate reads geometry.

**The interim.** `ConstraintKind::WorldPin(WorldPin::Identity)` (`crates/editor-core/src/mate.rs`), written by `DocEdit::PinWhereItStands`. It is a world pin with no read and no number. The fold places its copy by no map in the world's space (`eval/wire.rs`, `fold_with`). It stands alone in its bundle (`mate::solve::admit_beside_world_pin`, at the edit and load doors). Split carries it; inline refuses it under a moved landing (`InlineError::WorldPinOnAMovedWorld`). The migration (corpus `place_part`), the tour (`demos/tour/src/world.rs`), the demos and the fixtures (`fixture::pin` at the identity, `Recorder::place`, viewer `pinned`, pncad `place`, Python `Doc.place(shapes)` with neither constraints nor pose) place a copy at the identity by it.

**Why a mate would not do (measured on PR 5267).** `pin_at` and `place_at` read the copy's pose off its own geometry against numbers from one evaluation. A copy whose body is built in world coordinates through a variable is therefore pulled back toward the variable's nominal (revalue_corpus: `solve_copy_at_identity` flips on 7 rows), and widened by its width under a box. The tour's plate study certified nothing, and the chain's certifiable fractions fell about 3×. Preferring an anchor that no toleranced variable reaches (`mate::anchor`) left 25 of the tour's 26 copies on a varying anchor, because the tour builds its bodies through toleranced placements.

**Done when** the fork's answer lands: every world pin is restated in the D10 spelling it chooses, `ConstraintKind::WorldPin`, `DocEdit::PinWhereItStands`, `InlineError::WorldPinOnAMovedWorld` and the Python `WorldPin` retire, and spec §4 drops the interim.
