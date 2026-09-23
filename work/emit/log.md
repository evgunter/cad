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

## 2026-09-23 — loft anchors goes to Ev

PR 3102, reviewed and fix-passed, CI green. The reviewer's S1 finding
decided the shape: the door's anchoring can only be read off the node
whose table is being read. Before, a caller holding a later section's
own naming reproduced the bug silently. That changes what DM8
(`crates/editor-core/REFERENCES.md`, ratified at `1fd5e16eb`) says
twice. For a later loft section, the answer is in the loft table's
numbering, not in the section program's own. And the clause now binds
where the caller's anchoring comes from. Because the clause is a
ratified one, the PR is retitled `[ev]` and the row carries `needs_ev`.
The other two first-wave PRs do not wait on it.

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

## 2026-09-23 — the two vanish rows are one event

A design lane measured both rows before anything was specified. In
every scene it measured, neither row is a missing predicate flip.
Both are the same event: a fragment group changed size between the
two runs (2 → 1) while every discriminator verdict held. So the
shadow-exec rung can answer neither, whatever witness it is given:

- `name_frag_order_along` ranks siblings against each other. It has
  no cutting partner, so a partner in the `OrderAlong` qualifier would
  have been a name-vocabulary and stored-bits migration that still
  finds no flip.

Chosen: one new rung, `Diagnosis::GroupResized { node, was, now }`,
read off the two name tables. It sits last, before the evidence-free
fallback, so it only converts rows that would otherwise hit that
fallback and can never outrank a recorded flip. It touches no name,
serde or content key. It does add an arm and a rung to N5
(`crates/editor-core/src/names/README.md`, ratified) and re-pins the
diagnosis digest, so the PR goes to Ev as `[ev]`. The rows re-price
from H + H to D + E, with the SideOf row riding with the OrderAlong
one.

The orchestrator's picks on the proposal's open questions:
- the rung goes last;
- the name is `GroupResized`, because a group that grows also vanishes
  every rank;
- a group that vanishes entirely (`now: 0`) is reported;
- the SideOf base offer lands in the same PR;
- the `cutters_gone` enrichment becomes a follow-up row.

Ev can overturn any of these on the PR.
