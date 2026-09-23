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

## 2026-09-23 — the B-side rescue arm closes (PR 3103)

The arm had a real subject, and its own key read was correct. The
subject never reached it because the sibling read `operand_identity`
treated "not a graft destination" as "an A key". In the `(Absent,
Direct)` layout, where the result is B's clone, it looked B-arena keys
up in A's table. That was a live SILENT WRONG NAME on main. Every
union or intersection whose result is B's clone named B's vertices
from A's table where slot keys collided (`big ∩ small` named small's 8
corners `FromA(big …)`), and refused `Emission` where they did not.

Now there is one layout read (`operand_key`) and one side type
(`OpSide<K>`), shared by the face, edge and vertex passes. Before,
there were three hand-written copies and three enums. An
operand-swap symmetry row, over 5 fixtures × {∪, ∩} × both orders,
guards symmetry. Absolute rows (nested corners, split reflex edge,
assembly touch) pin which side a name belongs to. A corrupt-body
`Emission` in `resolve_edge_carrier` is no longer swallowed into a
tie.

Filed:
- `b-arena-edges-skip-the-split-lineage-chase` (P0, measured): an
  order-dependent `SharedRim` refusal on a legal union. It is the next
  P0 on this slate.
- `contact-partner-lookup-takes-the-first-of-several-vv-rows` (P3).

## 2026-09-23 — the B-arena edge chase closes (PR 3114)

`union(tip, bar)` refused `SharedRim` where `union(bar, tip)` named the
edges. The cause, measured: in a B-clone result, `chase_b` stepped
through an always-empty `fwd_edges`. Now the edge-root chase is chosen
by where a side's keys live (`operand_key` returns the side with its
key space): `Direct` chases in the arena for either side, and
`Grafted` goes through `chase_b`. Whole-table diffs changed only the
two cells that had refused.

Review measured the filed seam-chain row and found it a live silent
rename under `Node::Union`: member order rebinds `OrderAlong` edge
names. It is re-banded P0 and dispatched to the same lane on
`emit/seam-chain-ranks`.
## 2026-09-23 — seam-junction closes (PR 3112)

`emit_union::collapse` now reads a seam junction's run of `Seam` lines
as one head. The run is admitted only as the whole path of a
Vertex-kind name, and every other shape still refuses FOREIGN. Each
line collapses to member space through the head-`Seam` rule, and the
run is re-sorted. Two lines that collapse to one refuse loudly. No
stored name bit moves: review diffed whole tables across 308
(document, order) cells, and the only transitions were refusals
becoming names.

Filed from the unit and its review:
- `seam-edge-between-two-merged-faces-refusal-a-legal-declared-union-reaches`
  (P0, re-banded at adjudication): an `Emission` refusal on legal
  unions of ordinary blocks.
- `declared-flush-union-edge-and-vertex-names-follow-member-order`
  (P1): measured with 0 names rebinding, so a reorder makes names
  vanish typed rather than silently rebind.
- `name-ordered-positions-in-a-path-have-no-single-home` (P1, class):
  the sort rule lives in four partial lists. `collapse` does not
  re-sort `SideOf`, contrary to `role.rs`.

## 2026-09-23 — Ev on the loft anchors: a deeper design issue

Ev, on PR 3102: the split between "program numbering" and "published
numbering" for a later loft section points at a deeper design issue.
The orchestrator's reading, posted on the PR: a loft's section
correspondence is the kernel's lex-min canonical guess, not the
author's order. Three open rows share that root:

- this unit;
- carve's twisted-loft P0 (`loft-pairs-sections-by-canonical-start-not-authored-order`,
  which reaches main with 3102);
- carve's first-strip v-parameterization row.

Proposed: make the correspondence authored (step k of every section →
wall k). A mismatched-orientation section is refused typed, option
(a), or normalized with its start kept, option (b); the orchestrator
leans to (a). 3102 is held rather than merged. Its loft machinery
would be deleted by the redesign, and the wrong answer it fixes has no
production caller today. Waiting on Ev's answer.

## 2026-09-23 — Ev ratifies authored loft correspondence

On PR 3102's thread, Ev accepted option (1): orientation is
canonicalized per loop, and each loop's start vertex and the hole
order are as authored. He ruled out an explicit per-section offset as
redundant. The new unit is `loft-section-correspondence-is-authored`
(P0, H). It is given to the lane that did 3102, which has the context.

3102 will close unmerged once the unit's PR opens, and its red rows
carry over. `loft-anchors-every-section-with-section-zeros-map` closes
with the unit. Carve's first-strip row stays open on carve: authored
correspondence does not remove its sensitivity.

## 2026-09-23 — the loft unit stops on the opposite-sense section

The lane measured both scopes before committing.

- **The global canonical-start change.** It sets `start = 0` in
  `profile::validate`. It moves no published name for any verb, and
  body point sets are identical. It does move arena order in 4 corpus
  documents and an extrude volume by 1 ulp, and about 30 goldens and
  verdict counts. It would also retire the ratified V3 clause ("lex-min
  start"). It is outside the agreement, so it was not taken.
- **The loft-only change.** Inside the agreement, but it leaves one
  point open. A section authored in the opposite sense to section 0
  pairs program step n−1−k with section 0's step k. A loft publishes
  one ref per wall, so the ordinary door, asked with that section's
  own naming, would name the reflected wall. The door needs the
  published sense.

Options went to Ev on 3102:
1. one derived orientation bit per section on the loft value;
2. canonical numbering for every verb's refs, a migration for
   clockwise-authored profiles;
3. refuse opposite-sense sections.

The orchestrator recommends option 1. The unit carries `needs_ev`.

## 2026-09-23 — GroupResized lands (PR 3115, Ev approved)

The two vanish rows close together. Their premises were refuted
rather than fixed. `OrderAlong` never needed a partner, because
`name_frag_order_along` ranks siblings against each other. Both rows
are one event: a fragment group that changed size while every verdict
held.

N5 gains `Diagnosis::GroupResized`, placed after every cause-naming
rung and before the evidence-free fallback. Ev accepted the
justification that the ladder orders cause before effect. On whether
it needed his sign-off, Ev said an additive rung that contradicts no
principle he asked for is fine. The Display sentence states only the
two-table fact.

Filed from the review:
- P0 `global-flip-lanes-present-an-unrelated-flip-as-a-vanished-names-cause`,
  measured. This predates the PR.
- P1 `group-size-re-derives-group-membership-from-name-shape`.
- P2 `group-resized-does-not-name-the-cutter-that-stopped-cutting`.
- P4 `name-counts-saturate-silently-at-u32-max`.
