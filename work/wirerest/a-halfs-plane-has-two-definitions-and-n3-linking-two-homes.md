---
id: a-halfs-plane-has-two-definitions-and-n3-linking-two-homes
kind: issue
title: A split half's plane faces are defined twice (holds_section vs plane_pieces), and N3's operand linking lives half in topo and half in the emitter
status: open
opened: 2026-10-20
priority: P3
cost: M
---


Found by the re-review of PR 5486 (2026-10-20), in its Style findings S1–S3 and its minor m2.

- **Two definitions.** `emit_topo.rs` reads `topo::SplitNaming::holds_section` for chords and seams (around lines 754 and 962). Names read `plane_pieces`, which is a strictly larger set: it adds the kept faces of groups linked only through an operand, and the unmerged pieces of linked roots. The two are about ten lines apart in `crates/topo/src/splitting/finish.rs`, and no sentence relates them. Check every `holds_section` caller.
- **N3 linking in two homes.** The plane's group closure lives in topo (`plane_pieces`). The non-plane union-find over roots is a second copy, in `name_split_merges`.
- **An edge between two pieces.** `names/plane_pieces.rs`'s doc says one can exist (left by `merge_skipped`) and cites nothing. `split_chords`/`BETWEEN_SECTIONS` and `names/README.md` ("no edge of a half lies between two section faces") refuse it. So `plane_walls`' tolerance for that edge is either dead code or a refusal one stage later. Settle which one, and delete the loser.
- **Smaller points from the same review:**
  - `record_islands` pairs each ring with its own face's outline, which logs avoidable `outlines_disjoint`/`point_in_loop` readings.
  - `name_split_merges` repeats its record block.
  - `type Named` is a 5-tuple that should be a struct.
  - `NestingContradiction { hole }` now also carries an island.
  - `section(f)` is spelled out three times in `finish.rs`.

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).
