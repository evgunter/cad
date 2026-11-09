---
id: question-mark-ladders-the-t1-review-left-unported
kind: issue
title: QML T1's review found sibling ladders spelled by sequential ? and a re-spelled read_piece, on no row
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6637
---


Found by the FULL review of PR 6617 (QML T1, head 11222df9). Line numbers are at that head.

1. **F3. Sequential `?`s outside any loop.** These are the gate's stated blind spot, and they ask the same question as a ported ladder:
   - `replace_face.rs:~3123`, carried_alike's `(transport(a)?, transport(b)?)`. An escalation on a refuses where b's None would answer `Ok(None)`.
   - `offset_nappe.rs:~142`, face_nappe_reading's `lo?`. A lo in band refuses where hi is decided Negative, and min ≤ max forces Mirror.
2. **F4. The same identity read two ways in one file.** `replace_face.rs:3808`, `4431-4437` and `4612-4616` loop `gap_within_eps(..)?` over "every gap within ε" and return `Err(VertexDisagreement)`. The gate reads an Err return as no exit, so these loops are unregistered. Decide which reading is right and make the file read it one way.
   - The same file also spells the sign test two ways: `gap_within_eps` uses `!matches!(s, Sign::Negative)` (`:4029`), and `carried_alike` uses `d.sign == Sign::Negative` (`:3134`).
3. **F6. `Variety::read_piece` re-spells the reading.** `variety.rs:118-122`, from main, is again a hand-written Apart/Zero/Undecided match mapped to `Piece`, beside `IdentityLadder::holds`. Route it through `holds()`.
4. **F7. Name carriers spelled two ways.** Sweep has a const, `LOFT_RIM_PINNED`, with a roster row. The topo ports pass inline literals instead (`"seam_line_collinear"`, `"bool_wrap_rim"`, `"point_in_loop_*"`). Topo has no predicate_census roster, so no census reader sees those names at `read_identity`.

## Resolved (2026-11-09, `onecopy/refusal-standing`)

1. **F3:**
   - `carried_alike`'s two transports climb under `Any`: a side with no image answers "no witness", whatever the other read, and an escalation is kept by `ReplaceFaceError::refusal`.
   - `face_nappe_reading` climbs its two extremes under `Any`: the lowest station decided above the apex answers `Opening`, and the highest decided below answers `Mirror`, each alone, since `station_min ≤ station_max`. Only where neither does are both read through the old table. In `f64` a lo in band beside a decided-negative hi cannot occur (lo ≤ hi), so this port is reached only at interval width.
2. **F4:** the file's sign test is one rung, `gap_past_eps` (a gap decided past ε answers with the gap). It is read by:
   - `gap_within_eps`, a one-rung climb;
   - `carried_alike`'s samples;
   - the corner's pairs, `corner_apart`;
   - `read_ends`' two ends.

   The last two loops are climbs under `Any`, answering `VertexDisagreement` with the gap past ε. `plan_reanchors`' per-endpoint read stays one read through the same rung: its loop is the re-anchor pipeline, with each endpoint's own refusals and a clamp-recovery branch, not a loop over gaps.
3. **F6:** `Variety::read_piece` reads `read_identity(..).holds()`.
4. **F7, narrowed:** the two spellings follow one rule, now stated once on `read_identity`'s doc. A datum's name is a literal at the datum; a crate whose roster runs `predicate_census` (which reads names at `decide*` calls only) declares a `const` carrier. Topo runs no roster, which is filed as `topo-has-no-predicate-name-roster` (P3).

**Reach,** by the same temporary probe as the cone row (8,919 `ci` tests over topo, geom-brep, sweep and step-import): no answer was given past a kept reading at any of these sites.

**Pinned:** `replace_face::corner_rows::a_pair_past_eps_answers_past_a_pair_in_band` reads the production `corner_apart`: an in-band pair, then a pair a unit apart, answers the unit gap. It is red under the old first-refusal loop (checked by restoring it).
