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

## 2026-09-23 — loft anchors: the measurement, and the fix chosen

The lane stopped before implementing, as briefed. What it measured
(red rows on `emit/loft-anchors`, `tests/edit_step_segments.rs`):

- the kernel lofts a section authored reversed or rotated relative to
  section 0, and the solid is correct. The skin reads each section's
  canonical form, so authoring order is gone before pairing.
- the row's premise that "the refs are per-section already" is false.
  `name_swept_topology` mints ONE `ProfileEdgeRef` per wall, shared by
  every section, so one anchor can be right for one section only.

Both of the row's options were rejected. A refusal would reject lofts
whose solid is right, which changes what a loft admits. Per-section refs
in a name change D5's vocabulary and the stored selection bits. Chosen:
the table stays anchored on section 0, and the one door generalises to
compose `published ∘ own⁻¹`, where the published anchor comes from the
loft's own evaluation rather than from the caller. This is an additive
answer with a dominant argument over the other two, so it did not go to
Ev. The lane files the twisted-loft pairing question (canonical lex-min
start versus the author's intended correspondence) on the sweep
program's slate.
