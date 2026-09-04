---
id: LIB-DIETOOL
kind: unit
title: die_tool re-authoring check — is the Revolve/datum blocker cleared
status: closed
pr: 1632
branch: lib/die-tool-check
opened: 2026-09-03
closed: 2026-09-03
refs: [pncad-py-doc-has-no-node-kind-read-door]
---

Mechanical (no A/B row; brief-as-spec). The die_tool corpus document was
authored around a Revolve/datum limitation; the banked question is whether
the intervening datum/revolve work cleared it. Measure: re-author the
document the natural way through the current public surface; if it works,
land the re-authoring with byte/digest evidence of what changed; if it still
refuses, record the refusal as the living evidence and close the check with
the blocker named.

## Outcome (2026-09-03) — CLEARED

**The blocker, from the record.** `die_pips`' REPORTED deviation (b),
the equator workaround: the revolve name emitter refused an
all-on-axis two-pole meridian ("revolve vertex resolution exceeded
elimination"), so no sphere reached a `Node::Revolve` at all, and the
ball was authored as two quarter arcs meeting at an OFF-AXIS equator
vertex whose bulge came from `tan(π/8)`. `7581fb65d` (2026-08-15)
retired it once the emitter grew its pole export.

**Where it did and did not show.** Not in this corpus document:
`die_tool` (`54f44ac90`) postdates that deletion by one commit and
reuses `die_pips::half_disc_program`, the natural bulge-1 semicircle.
The recipe needed no change and the name-table digest could not move.
It showed in the PYTHON die scene, which still carried both the
workaround and a docstring asserting the retired refusal, and in the
absence the log named: `heat_sink_fins` (Linear, extrude-only) had a
Python twin, `die_tool` (Explicit, Revolve about a `Datum::Axis`) had
none.

**The verdict, by construction.** The natural authoring works.
`crates/pncad-py/tests/test_placed_union.py::TestTheDieTool` says the
document's seven nodes through the bound doors and asserts its
`Doc.save()` text against the registered document's own saved bytes —
`crates/editor-core/tests/corpus/die_tool.pncad`, pinned by
`crates/editor-core/tests/lib_dietool_crossing.rs` — line for line,
identity included, bar the swept `"epsilon"` line.

**Digest evidence.** `lib_g16_corpus_name_digests` green with
`die_tool` at `0x9e24_4be7_b06b_9a40` (unchanged) and `m10_p_fence`
green at all three scalars: the registry is untouched, so neither
gate's roster re-bless procedure applies.
