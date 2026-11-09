---
id: question-mark-ladders-the-t1-review-left-unported
kind: issue
title: QML T1's review found sibling ladders spelled by sequential ? and a re-spelled read_piece, on no row
status: open
opened: 2026-11-09
---


Found by the FULL review of PR 6617 (QML T1, head 11222df9). Line numbers are at that head.

1. **F3. Sequential `?`s outside any loop.** These are the gate's stated blind spot, and they ask the same question as a ported ladder:
   - `replace_face.rs:~3123`, carried_alike's `(transport(a)?, transport(b)?)`. An escalation on a refuses where b's None would answer `Ok(None)`.
   - `offset_nappe.rs:~142`, face_nappe_reading's `lo?`. A lo in band refuses where hi is decided Negative, and min ≤ max forces Mirror.
2. **F4. The same identity read two ways in one file.** `replace_face.rs:3808`, `4431-4437` and `4612-4616` loop `gap_within_eps(..)?` over "every gap within ε" and return `Err(VertexDisagreement)`. The gate reads an Err return as no exit, so these loops are unregistered. Decide which reading is right and make the file read it one way.
   - The same file also spells the sign test two ways: `gap_within_eps` uses `!matches!(s, Sign::Negative)` (`:4029`), and `carried_alike` uses `d.sign == Sign::Negative` (`:3134`).
3. **F6. `Variety::read_piece` re-spells the reading.** `variety.rs:118-122`, from main, is again a hand-written Apart/Zero/Undecided match mapped to `Piece`, beside `IdentityLadder::holds`. Route it through `holds()`.
4. **F7. Name carriers spelled two ways.** Sweep has a const, `LOFT_RIM_PINNED`, with a roster row. The topo ports pass inline literals instead (`"seam_line_collinear"`, `"bool_wrap_rim"`, `"point_in_loop_*"`). Topo has no predicate_census roster, so no census reader sees those names at `read_identity`.
