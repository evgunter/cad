---
id: a-placed-copy-rests-at-the-identity-only-at-the-nominal
kind: issue
title: A copy the place-part gesture pins at the identity rests there only at the nominal, so the revalue apart move flips its at-rest snap
status: open
opened: 2026-10-18
priority: P3
cost: M
---

Found by PR 5267 (s3 C) when main's revalue contract (every flip is named, #5268's merge) met C's corpus, which places every part through the gesture.

- **Rows:** `revalue_corpus`'s `face_sketch`, `die_chamfer`, `heat_sink_fins`, `loft_prism` and `measured_web` (apart), `declared_tangency` (apart, two of them beside its owed azimuth flips) and `cut_cylinder` (in step).
- **Decision:** `solve_copy_at_identity` (`crates/editor-core/src/mate/solve.rs:433`, the fold's one at-rest verdict, fork C-5). Its margin is 0.0 exactly at p0 and about 1e-6 at p1 (9.4e-7 to 7.8e-6 over the rows), so the sign flips from `Zero` to `Positive`. In `cut_cylinder` the swing the chart takes onto the mated direction flips first (`mate_chart_reference_antipode`, `crates/editor-core/src/mate.rs:842`, 1.25e-16 → 3.2e-7): the copy's face there is exactly antiparallel to its target at the nominal.
- **The builds do not part.** Nothing re-values away after the flips (`after_flip` is empty), and no margin is laundered.
- **Repro:** `REVALUE_DETAIL=1 REVALUE_DOC=face_sketch cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`.

**Mechanism.** `place_at` and `pin_at` (`crates/editor-core/src/mate/pin.rs:178`, `:361`) write the pose a copy is placed at as literal values read off the nominal build — a world plane and three values, or a frame's spins and slides. The mate's other side is read off the copy's own geometry, which the document's variables move. At the nominal the two agree and the copy stands at the identity to rounding, so the snap places it by no map; moved, the copy's geometry leaves the literal pose by the move's own size, and the verdict is `Positive`. The flip is the document's real dependence, not a laundered constant: a part placed by literals stays where the literals say while its shape moves.

What remains to decide: whether a part placed where it was authored (the identity) should be placed by a read of the part's own frame rather than by literals off its geometry, so it rests at every value of its variables; or whether the snap is right to flip and the rows owe it for good.

**Pinned.** Each row owes its flips here through its `flips` entry, so the row reds when they go, and when any other decision flips.
