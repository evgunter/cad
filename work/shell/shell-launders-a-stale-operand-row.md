---
id: shell-launders-a-stale-operand-row
kind: issue
title: shell's closing pcurve mint clears the map first, so a stale or missing row on the OPERAND is laundered into a valid result — the verb does not gate its operand's rows, and neither does any of the thirteen producers spelling the same mint
status: open
opened: 2026-09-08
---

Measured by the SHELL-9 R2 review lane, by execution
(`crates/sweep/tests/shell9_r2_probes.rs`,
`r2_the_closing_mint_launders_a_stale_row` and
`r2_the_closing_mint_launders_an_invalid_operand`, kept as pins of the
measured behaviour). A vessel with one face's certified pcurve row
attached to another face's half-edge is tier-3 INVALID with two
`LoopDiscontinuity` findings; `topo::shell` on it returns `Ok` and a
tier-3-valid body. A vessel with one row detached shells to rows
bit-identical to the sound operand's result. At SHELL-9's merge base
both refused `ShellError::NotValid`, because the verb carried the
operand's map through to its own validate; SHELL-9's closing mint
(`crates/topo/src/shell.rs:1686`) runs `mint_pcurves`, whose first act
is `body.pcurves.clear()` (`crates/topo/src/pcurves.rs:1327`), so
every operand row — stale, missing or wrong — stops existing before
the validate reads anything. The base tree's refusal was accidental
(the verb never gated its operand's tier 3), but this tree turns a
defect the tier-3 pcurve pass exists to catch LOUD into a silent
repair.

**Class, not instance.** The same closing mint is spelled by thirteen
producers, none of which gates its operand's rows first:
`crates/topo/src/replace_face.rs:1274`,
`crates/topo/src/merge_faces.rs:1120`, `crates/topo/src/transform.rs:650`,
`crates/topo/src/splitting/mod.rs:650`,
`crates/topo/src/boolean/ops.rs:585`, `crates/topo/src/shell.rs:1686`,
`crates/sweep/src/revolve/mod.rs:749`,
`crates/sweep/src/revolve/tube.rs:510`, `crates/sweep/src/loft.rs:553`,
`crates/sweep/src/blend/surgery.rs:706`,
`crates/step-import/src/assemble.rs:838`. Whether a producer should
refuse an operand whose own rows do not re-certify, or whether
"construction-fresh re-derivation" (`crates/topo/src/pcurves.rs`,
"Persistence and transfer posture") is the ruling and an operand's
rows are never load-bearing, is a posture-table decision for TOPO —
it is not decided here and SHELL-9 adds no operand gate. The SHELL
half is the disclosure: `shell.rs`'s "The closing mint" doc states
the class, and the two R2 rows are what a gate would flip.

**The two simultaneous offset doors have left this class** (SHELL-10,
2026-09-08). `offset_together.rs` and `offset_axial.rs` no longer call
`mint_pcurves`: each closes with `pcurves::mint_pcurves_of` over its
scope's faces, which does not clear the map and re-derives nothing
outside the solids the move set names. So they no longer launder an
out-of-scope operand row in either direction — where the whole-body
pass silently re-derived a half-minted out-of-scope face, the result
now carries the operand's own missing row and `validate_geometric`
reports `Pcurve { MissingCache }` (pinned:
`crates/sweep/tests/shell10_r1_probes.rs`,
`r1_the_door_no_longer_launders_a_half_minted_out_of_scope_face`).
Eleven producers, not thirteen, and the laundering they do is still
this item's.
