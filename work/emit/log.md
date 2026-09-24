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

## 2026-09-23 — the global flip lanes are scoped to ancestors

The lane stopped on a definitional fork: what the global lanes may
report as a vanish's cause. The two candidates:

- (a) N1's derivation path, under which the global lanes disappear;
- (b) the transitive ancestors of the minting node, with sentences
  saying "upstream of", not "on the path".

The measurement: the digest corpus and the whole suite are identical
under both. The two differ in a cutter-union scene, where a recorded
flip sits upstream of the cut but not in the vanished name.

The orchestrator chose (b) without asking Ev. It applies the
cause-before-effect principle Ev approved on 3115. An upstream flip is
a candidate cause and outranks the `GroupResized` effect, as long as
its sentence claims only what is known. Under (a), GroupResized's "no
flip was found" would be false. A flip on a non-ancestor node is never
reported, which closes the live wrong answer.
## 2026-09-23 — union seam-chain ranks close (PR 3121)

A union's `Seam` canonicalization swapped the pair into name order
but kept an `OrderAlong` tail ranked along n_a×n_b, A side first. So
reordering members rebound seam-edge names silently: a P0 found by
3114's review. Now a swapped pair reads a seam EDGE's rank as
of−1−rank, which is exact because negating the direction reverses a
certified strict order and keeps ties. A ranked seam vertex keeps its
rank, and a unit row guards that exemption. Its unreachable two-edge
case refuses typed. Pair-boolean names are untouched, and no union in
the tree today collapses a ranked seam edge.

Review found the sibling: a three-member union can rank one seam once
as a two-edge chain along n_a×n_b and once as a descent sub-edge
chain along the edge's own direction. `collapse` flattens both to one
spelling. Filed P0 as `union-seam-edge-ranks-follow-which-step-split-the-seam`.
It goes to the same lane next, aimed at one orientation rule for both
rankers.

## 2026-09-23 — merged-face chords close (PR 3120)

A seam edge whose two crossing faces were both merged faces refused as
`Emission`, a kernel bug, on legal declared unions of blocks. Measured
cause: the edge was never a crossing. It was a piece of one member's
rim edge that a slab had split, lying between two merged faces. The
merged-face read-through needed a partner face that did not exist.

The key now says which side to look at. A certified geometric check,
`chord_on_rim` under predicate `name_chord_on_rim`, decides whether
the chord is that side's rim piece; if it is not, it refuses typed as
a missing rule. Whole-table diffs over 924 cells: 0 fused names moved,
82 of 89 former `Emission` refusals now fuse, and none remain.

Filed from the unit and its reviews:
- P0 `shared-rim-several-is-a-missing-rule-legal-declared-unions-reach`,
  re-banded: the commonest refusal on legal unions.
- P0 `split-of-a-fused-declared-union-refuses-duplicate-vertex-name`.
- P4 `opside-unit-respells-topo-operand`.

## 2026-09-24 — Ev rules the loft unit global

On 3102's thread, Ev chose the global canonical start (V3's lex-min
start retired) and canonical numbering for every verb's published
profile refs. That means migrating the names of clockwise-authored
extrudes and revolves. Re-baselining is not a cost against a correct
change. The loft unit's `needs_ev` is cleared, and the unit is
re-dispatched with the wider scope.

The weekly usage limit stopped all lanes on 2026-09-23 at about 09:30
UTC. They were resumed on 2026-09-24 from their pushed branches and
worktrees.

## 2026-09-24 — a second usage-limit stop

The weekly limit stopped every lane again at about 05:00 UTC. At 12:10
every branch was fully pushed and every worktree was clean, so nothing
was lost except in-flight edits the lanes re-derive. The four lanes
and the review of 3133 were resumed:

- loft correspondence, whose PR is about to open;
- the 3125 fix pass;
- the 3124 fix pass;
- 3133, the split spur, in review.

## 2026-09-24 — Ev on tangent splits

Ev, in chat: a split plane that grazes the target within the sliver
band refuses, and an exact tangency has to be declared. Both are
unlikely to happen by coincidence when the cut elsewhere does not
depend on them. So refusing, as 3133's guard does, is the correct
posture, not a P0. What remains is verb breadth: `split` has no way to
declare an exact tangency, with the boolean's declared contacts as the
precedent. That is filed P1 on REACH's slate from 3133.
## 2026-09-24 — the global flip lanes close (PR 3124)

A vanished name's diagnosis now reads two scopes, through one lane
table (flip, then structural parameter, then recipe edit):

1. its derivation path;
2. its minting node's strict ancestors in either run, each walked
   within its own run's document, minus the path. Answers from this
   scope are reported as `Diagnosis::Upstream`.

A node the name does not depend on is never read. The ladder runs:
path lanes, `qualifier_delta`, `Upstream`, `GroupResized`, fallback.
Review caught a MAJOR: the first ancestor walk mixed the two runs'
edges, so a rewired recipe could reach a node that was an ancestor in
neither run. The fix pass walks each run separately, through the
shared `roots::walk_strict_ancestors`. Two ShadowExec sentences that
claimed per-pair evidence now state their node-level trigger.

## 2026-09-24 — seam-line ranks close (PR 3125)

A union's seam-edge pieces rebound across member orders, because two
rankers oriented one seam line differently: a seam chain minted
already cut, and a descent chain cut later. After three review rounds
there is one home, `names::seam_pair`. It answers which seam line a
rank runs along, through one exhaustive wrapper list, for the emitter
and for the collapse alike. The pair emitter keeps its structural
sides. An equal-named pair (two placements of one prototype) ranks
along its own carrier, and a union cannot produce one because members
are wrapped in `FromMember`.

Measured rebinds, main to head, all 0: 192 in `far`, 352 in `two_b`,
108 in `two`, and 368 in the reviewer's `two_ribs`. Some pair-boolean
names move and are listed in the PR: 12 rows in `cross`, 12 in
`cross_plain`, 4 in the split repro. Filed: P3
`seam-line-sides-is-a-missing-rule`, raised to P0 if a document
reaches it.
## 2026-09-24 — the split spur closes (PR 3133)

A split plane touching the target along an edge while cutting it
elsewhere used to "succeed" with a zero-area spur on the section face.
That left two null-pair copies of one vertex, and the naming layer
reported the result as a truthful `Duplicate`. Review corrected the
mechanism: the direct join refused, and the D7 pinch lane's mirrored
rerun laundered that refusal into a spurred success. The join now
refuses a spur tip as `SplitJoinError::SectionSpur`, with its own
message. Following Ev's ruling in chat, that message says an exact
tangency would need to be declared. Rows now name which predicate
refused, so an area refusal substituting for the spur refusal goes
red.

Filed:
- REACH P1 `split-cannot-declare-an-exact-tangency-with-its-target`;
- REACH P3 `split-section-spur-guard-skips-curved-spurs`;
- ATREST `validate-passes-a-body-with-a-zero-width-slit-face`,
  rewritten and re-banded P3: `split` runs no validation tier on its
  own outputs.

## 2026-09-24 — loft correspondence closes (PR 3147)

Ev's two rulings on 3102 are implemented:

- A profile loop's canonical start is its authored vertex 0, with
  orientation normalized. V3's lex-min start is retired and survives
  only as the containment representative.
- Every verb publishes profile refs in canonical numbering, so the door
  answers any loft section through the section's own anchor.

Consequences:

- The +30° twisted loft builds the author's solid.
- The tour lily's blades lose the twist lex-min gave them: one leaf
  drops from 43,738 to 21,390 triangles.
- 12 of the corpus's 1,612 names migrate, in `plate_param`.
- About 30 goldens and counts are re-baselined. Each is listed in the
  PR with its cause; review verified the four riskiest.
- The fix pass swept stale "program-anchored" and "exact-order band"
  prose.
- New rows that go red under a lex-min start.
- `replay_naming`, so a SetProgram over an old program that replays
  but does not validate keeps its names.
- The now-dead offset knob is removed.

`loft-anchors-every-section-with-section-zeros-map` closes with it,
superseded.

Filed:
- P3 `a-lofts-names-follow-only-its-first-sections-reshaping`.
- `a-param-jump-that-swaps-the-outer-loop-renumbers-names-unreported`,
  re-banded P0 at merge because it is a silent rebind.
## 2026-09-24 — SharedRim(Several) is held for its rebind sibling

PR 3167 names a chord over a rim that exists in several collinear
pieces: the chord takes the one piece it lies within. On the probe
corpus, all 62 refusing union cells fuse, and 0 names that fused on
main move. But measured over member orders, 42 order pairs that used
to refuse loudly would now silently bind a rim-piece name to a
different piece. That silent class already exists on main: 18 of 162
pairs that both fuse on main rebind.

The project is fail-loud, so 3167 does not land alone. The P0
`union-rim-piece-ranks-follow-fold-order-so-rim-names-rebind` is
stacked on it and dispatched to the same lane. The two land together
once the probe corpus shows 0 rebinds.

## 2026-09-24 — rim-piece ranks: the piece set is order-shaped too

Review of 3167 and 3168 found that re-ranking a member edge's pieces
over the finished body is not enough. Which member keeps a flush
stretch depends on order, so the SET of pieces under `(m, e)` differs
by order, and ranks within it rebind (`r2ends`: 8 names). On one
fixture, head is worse than main. The corpus could not see this
because it had one flush partner per document.

New rule, sent to the lane: rank each piece by its index in the
geometric subdivision of `e` in the final body, counting every piece.
A piece another member owns in another order then vanishes rather
than rebinds, which DM4 accepts. Nested occurrences in vertex names
are rewritten by the same rule. The pair still lands together only at
0 rebinds on every fixture.
## 2026-09-24 — the narrowing rule has its doors (PR 3169)

`name_placed_union` now narrows through `defer::narrow_into`. The
"several ⇒ tied" minting decision, spelled at six sites plus a
SectionEdge third form, now goes through one door,
`defer::mint_candidates`. "A lone member keeps the base name" stays
local at each site, because each site's discriminator would answer it
trivially. The change is behaviour-preserving: the corpus name
digests, the 304-cell probe and a reviewer-built ≥2-survivor
placed-union tie are identical to main.

## 2026-09-24 — three small rows close (PR 3175)

- Naming counts narrow to u32 through one helper, `names::emit::to_u32`,
  and refuse typed rather than saturating. The resolve rung declines.
- `OpSide<()>` gives way to `topo::Operand` and its existing `other()`.
- The seam-vertex pass reads every contact vv row. Agreeing rows name
  the vertex; distinct names refuse `SeamVertexPartners`. No suite or
  probe reaches two rows, measured over 3,747 vertices.

Filed: `naming-index-casts-saturate-silently-at-u32-max` (P4), widened
at review to the 14 truncating `as u32` casts in `editor-core`.

## 2026-09-24 — value edits report a numbering move (PR 3180)

A value edit (SetParam, SetExpression, SetStructuralParam, SetDocParam,
SetDocParamValue) that moves a profile's canonical numbering used to
rename silently: a hole grown past its outer loop, or a loop whose
sense flipped. It now runs the same carry-and-report door as
SetProgram. DM7 is re-worded to cover edits that move a name's
numbering.

Review found a two-step path through an unreadable state (no replay,
a tie, or zero area). The interim rule makes it loud: an unreadable
side strands every name on that profile. The lossless answer needs
the last published numbering, and that is not recipe state (two
saves are byte-identical while the same name denotes different
walls). The choice between option A (persist it) and option B
(doc-param edits refuse unreadable results) is with Ev in
`a-value-edits-last-published-numbering-is-not-recipe-state`
(`needs_ev`).

Filed:
- P0 `a-child-documents-rebind-leaves-the-parents-held-names-in-the-old-numbering`
- P1 `the-viewer-drops-every-dm7-rename-report`
- P2 `the-value-edit-numbering-check-costs-a-replay-per-swept-profile`
