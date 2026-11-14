---
id: the-filleted-dies-body-moved-at-6533-and-is-pinned-unjudged
kind: issue
title: The filleted die's body moved at PR 6533 and nobody has judged it: seat4's die_fillet digest and step-export's filleted_die.step are pinned at main's reading
status: open
opened: 2026-11-14
priority: P1
cost: E
---


Filed by GREEN's `green/goldens` lane, from nightly 43371638371 (main `e36675c1`). This is the fix row for `work/green/main-red-the-filleted-die-moved-in-seat4-and-the-step-export-fixture.md`, which GREEN closed by pinning.

**What moved.** Two slow-set rows read the filleted die and went red together, at all three ε rows:

- `crates/editor-core/tests/seat4_verb_lowering.rs` `the_blend_documents_evaluate_to_their_committed_digests`: `die_fillet` read `0x7418085874f97095` against the committed `0x9d27e739ef2f43bd`;
- `crates/step-export/tests/export.rs` `committed_fixtures_are_byte_golden`: `filleted_die.step` drifted.

**The merge.** PR 6533 (`onecopy/p0-sheared-fillet`, merge `fe7806519a`, "a blend band's directions are read off its stored geometry, not off its feet's points"), bisected first-parent at ε 1e-9 by `onecopy/main-reds-triage` on 2026-11-09: at `fe7806519a^1` seat4's row passes and `filleted_die.step` regenerates byte-equal; at the merge both read what main reads. PR 6533's description discloses neither.

**What GREEN did.** Both are pinned at what main produces, so the suite is green and each reds on the next move:

- `die_fillet` is pinned at `0x56293fdba9cabaed`. That is the reading above with PR 6856's id move over it (a placement mints its source, so every later id moved; with mint ids read by rank of first appearance the document's digest feed is byte-identical across that merge).
- `filleted_die.step` is regenerated with `cargo run -p step-export --example export_fixtures -- crates/step-export/tests/fixtures`. The bytes that moved are in the PR that filed this row.

`m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked` holds on main, so no vertex point of `die_fillet` moved since that row was last cut (PR 6795, 2026-11-11). It cannot see a surface, a curve or an edge certificate, which is where a band's directions would show.

**Owed.** Say what PR 6533 changed in the filleted die's body and whether the new body is right. PR 6533 changes how a blend band reads its directions, so the body may be meant to move. If it is right, strike the PINNED paragraph in `seat4_verb_lowering.rs` and say so there; if it is wrong, that is a live wrong answer on a normal verb and this row is P0.

Reproduce: `cargo nextest run -p editor-core -E 'test(the_blend_documents_evaluate_to_their_committed_digests)'` and `cargo nextest run -p step-export -E 'test(committed_fixtures_are_byte_golden)'` at `fe7806519a^1` and at `fe7806519a`.

**What the STEP bytes show** (the regenerated `filleted_die.step` against the committed one, entities compared with their `#n` masked): the file keeps its 648 lines and every `VERTEX_POINT`, `ADVANCED_FACE`, `EDGE_CURVE`, `CIRCLE`, plane, cylinder and sphere. What moved is 24 `CARTESIAN_POINT`s, each the origin of one of the 24 `LINE`s (`(0.88, 0.88, 1.0)` became `(0.5, 0.88, 1.0)`, a slide along the line), and 12 `DIRECTION`s by spelling (`(1.0, 0.0, 0.0)` became `(1.0, -0.0, -0.0)`), read by those lines' `VECTOR`s and four `AXIS2_PLACEMENT_3D`s. So no vertex moved and each straight edge has a new origin on what reads as the same line. That is a description of the bytes, not the judgment this row owes.
