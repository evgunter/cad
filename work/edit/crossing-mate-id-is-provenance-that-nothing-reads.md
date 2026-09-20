---
id: crossing-mate-id-is-provenance-that-nothing-reads
kind: issue
title: An interface crossing's mate id is provenance nothing reads: checked at insertion today, deletable tomorrow
status: spec
branch: edit/crossing-drops-mate-id
opened: 2026-09-19
---


## The finding

`InterfaceCrossing::Mate { mate, class, outer, inner }`
(`crates/editor-core/src/node.rs`) carries the id of the mate that
crossed the cut. Since PR #2872's fix pass it is a PROVENANCE reference
the insert door checks live (`Node::payload_read_sites` lists it, so a
dangling id refuses `ReadSiteMissingNode`), and a later delete of the
mate is not reported. Nothing READS the id: `eval/wire.rs` carries it
into `NodeErrorKind::CrossingUnverified`'s prose (`mate.0` is printed)
and `crates/pncad-py/src/py/refactor.rs` republishes it as a `NodeId`;
no door looks the node up, no re-verification resolves against it, and
the two references that do the work are `outer` and `inner`.

## Why it may matter

A field that is checked but never read is a check with no reader to
protect. The more harmonious shape may be to DELETE the field: the
crossing then carries exactly the two references the seam needs, the
read-site arm and its fixtures go, and the refusal prose names the
crossing by its `outer` instead of by a mate id. What keeps it today is
the wire: `InterfaceCrossing` is persisted (`serde`, `deny_unknown_fields`),
so removing `mate` is a format change — a decision of its own, and the
persist schema is contended ground per `work/paths/program.md`'s
keep-out convention. Put to Ev on `[ev]` #2869's thread (2026-09-19)
as the one item where a nicer shape may exist.

## What closing it would decide

Whether a crossing's provenance is worth a persisted field; if not,
the wire change (a file that carries `mate` refuses or ignores it —
say which), the deletion of the read-site arm and its rows, and the
refusal prose re-worded to name the crossing by `outer`. Ground:
`crates/editor-core/src/{node.rs, edit.rs, eval/wire.rs, refactor.rs}`
(EDIT, FIX), `crates/pncad-py/src/py/refactor.rs` and `pncad.pyi`
(LIB, by announcement).

## Ruled and spec'd (2026-09-20, EDIT orchestrator) — middle tier, wave 15, branch `edit/crossing-drops-mate-id`

**Ruling: the field goes.** A crossing carries exactly the references
the seam needs — `class`, `outer`, `inner` — and no provenance. Ev's
answer on `[ev]` #2869 ("if there isn't a nicer way, sounds good")
left this the one item where a nicer shape exists, and it does: a
checked-but-unread field is a check with no reader to protect, and
the delete of the mate it names is already unreported (DM7's read-site
rule), so nothing observable is lost. No design page names the field
(`ASSEMBLY.md`'s A4 paragraph and AQ8 name the RECORD and the mate
EDGE, not its id) — nothing binding moves.

The spec, as premises (verify each against the tree before building):

1. `InterfaceCrossing::Mate { mate, .. }` (`crates/editor-core/src/node.rs`)
   loses `mate`. The one writer is `refactor::split`'s crossing walk
   (`refactor.rs` ~1595); the readers are `Node::payload_read_sites`'s
   `InstantiatePart` arm (`node.rs` ~3536 — the arm goes, and with it
   the two doc paragraphs above it that argue a provenance check),
   `eval/wire.rs`'s `CrossingUnverified` construction (~439, which
   prints `mate.0`), `doc.rs`'s fixture (~1471), and
   `crates/pncad-py/src/py/refactor.rs`'s republish (~101) with its
   `pncad.pyi` paragraph. `inline`'s dissolve (`refactor.rs` ~1959)
   reads `inner` only and does not move.
2. **The wire.** `InterfaceCrossing` is persisted under
   `deny_unknown_fields`, so a file carrying `mate` refuses at load
   with serde's unknown-field refusal — say so in the PR body, and
   whether that refusal is typed by the load door's census or is
   serde's own message (the EDIT-DECL precedent: an unversioned shape
   with nothing outside the tree persisting it moves with no
   migration; if any corpus or persisted fixture carries a crossing,
   re-author it and list it). The persist schema is PATHS's contended
   ground by keep-out: announce the exact lines.
3. **The refusal names the crossing by `outer`.** `NodeErrorKind::CrossingUnverified`
   carries and prints the remainder-side reference (the one the
   remainder's mate keeps) in place of a node id; its Display row in
   `display_contract.rs` moves with it; the Python tag map does not
   change (grep `crossing_unverified` in `tags.rs` and say).
4. **Rows.** Every row that measured the live mate-id check retires
   BY NAME with what replaces it (the record rows in
   `edit_one_predicate.rs`, `edit_instance_crossing_names.rs`'s
   mate-id rows, `asm_r2b_interface_wire.rs`'s wire rows): the
   replacement pins that a crossing round-trips the wire with three
   fields, that a file carrying a fourth refuses, and that
   `payload_read_sites` on an `InstantiatePart` yields exactly what it
   yields now minus the mate ids (the crossnames unit's exhaustive
   match keeps the arm's absence honest). The census in
   `crates/pncad-py/tests/test_binding_census.py` moves if it lists
   the field.
5. **Territory**: `node.rs`, `edit.rs`, `doc.rs` (EDIT); `eval/wire.rs`,
   `eval/mod.rs` (WIRE, by announcement); `refactor.rs` (FIX, by
   announcement); `persist/*` only if a codec names the field (PATHS,
   contended — announce); `crates/pncad-py` (LIB, by announcement);
   `crates/editor-core/tests/*` (TCOST/TINT). Middle tier: one opus
   style review with a correctness arm, then the fix pass.

