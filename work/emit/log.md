# EMIT — the log

## 2026-09-20 — opened

Cut out of WIRE, which was carrying 85.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed WIRE's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

8 rows arrived by `git mv` with their ids, bodies and history
unchanged. WIRE keeps its band 3700-3799; band 8300-8399 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## 2026-09-23 — picked up

An orchestrator took the track (status `active`). Ev set the posture in
chat: no A/B protocol, since the account has no Fable usage left, so
implementer and reviewer lanes are Opus and each unit gets a single
reviewer lane (plan, Review posture).

First wave, run in parallel on separate worktrees:

- `loft-anchors-every-section-with-section-zeros-map` — the plan's
  first row. The lane measures before choosing between a per-section
  anchor and a refusal of non-corresponding sections, and stops to
  report if the choice turns out to be a fork about what a loft admits.
- `three-emission-bugs-...` and `names-emit-keeps-an-unguarded-...` —
  the two `E` drive-bys in `names/emit.rs`, one lane, one PR.
- `the-b-side-contact-record-rescue-arm-never-fires` — decide whether
  the arm has a subject; `emit_topo.rs`, disjoint from the other two.

The two vanish classes wait for the second wave, as the plan says, so
they are specified together.

## 2026-09-23 — the two `emit.rs` drive-bys close (PR 3099)

- **`EMISSION_FRAMING`.** All six emission bugs now open with the
  framing, which was reworded to be true of all six: "name emission
  found a kernel bug — an invariant it relies on does not hold". The
  old wording, "inconsistent with the result body", was false for
  `MissingUpstream` and `Duplicate`. The display test now checks that
  each message starts with the framing, not merely that it contains it.
- **The walk guard.** The first version read the source file. Review
  probed it: it failed on 4 of 7 edits that kept behaviour and passed 1
  of 4 edits that really merged two refusals. It also hand-rolled a
  lexer next to `test_utils::source`. It is replaced by behavioural
  rows. topo gains `Body::with_entity_removed_for_tests`, gated on
  `sweep-testing`. The guard drives every hop's refusal of
  `rim_between`, `face_half_edges`, `edge_ends` and `emit_topo`'s
  `chord_faces` through a genuinely dangling body. Folding the rim
  walk onto `face_of_half_edge` turns it red.
