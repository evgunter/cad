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

## 2026-09-24 — the narrowing rule has its doors (PR 3169)

`name_placed_union` now narrows through `defer::narrow_into`. The
"several ⇒ tied" minting decision, spelled at six sites plus a
SectionEdge third form, now goes through one door,
`defer::mint_candidates`. "A lone member keeps the base name" stays
local at each site, because each site's discriminator would answer it
trivially. The change is behaviour-preserving: the corpus name
digests, the 304-cell probe and a reviewer-built ≥2-survivor
placed-union tie are identical to main.

## 2026-09-24 — name-ordered positions have one home (PR 3173)

`names/canonical.rs` is now the one place a path's name-ordered
positions are put in order:
- `Merged` and `BandFace` sets;
- `SideOf` partners (the collapse never sorted these before);
- junction runs;
- a union `Seam`'s sides, and the `OrderAlong` rank value that depends
  on them.

Mint, collapse and every rewrite (`rewrite_path`, `refactor::remap_*`)
go through one core. The rank rule is derived inside it by comparing
the name before and after, for every rank on a seam line, including
one reached through a wrapper.

Review round 1 caught a pair boolean's ranks along an embedded union
seam re-binding silently under a reordering remap. On the reviewer's
probe over every permutation there are now 0 wrong binds and 0
dangling names; main had 990 dangling. Published names that move: 42
`SideOf` partner-order rows, each binding the same geometry.
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

## 2026-09-24 — the stable-name question put to Ev (PR #3193)

Ev's comment on EDIT's #3163 asked for "a more general stable name
system". Three open rows share one cause: a profile locator spells a
canonical (loop, segment) position, and that position is recomputed from
current state. The rows are this program's two `needs_ev` rows and
EDIT's zero-fit row. The `[ev]` PR states the id rule in
`names/README.md` ("N1, the profile pieces").

Evidence behind the PR:
- **Authored positions in names.** `ProfileEdgeRef` and
  `ProfileVertexRef` are the only authored-positional coordinates in
  `RoleSeg`.
  - Nodes and union members are already ids.
  - `Instance { i }` is structural.
  - `Fragment`, `SectionFace { section }` and `HoleRim { hole }` are
    ordered by the kernel, and change only at recorded verdicts (N2, N7).
  - `OrderAlong` carries `of`, so a resized group vanishes
    (`GroupResized`) instead of aliasing.
  - I did not check whether `HoleRim`'s pairing order can reorder at
    an unchanged hole count.
- **Size of the change.** 79 non-test references to the locator types
  across 24 files:
  - `sweep` and `profile` validate;
  - editor-core: names, anchor, program, edit, node, eval;
  - `pncad` and `pncad-py` selectors;
  - viewer marks.
- **What the id rule deletes.** The SetProgram rename machinery in
  `edit.rs`: `LoopProvenance`, `SegmentMap`'s rewrite, `RETIRED_FLOOR`,
  `reanchor_report` and `numbering_move`. That is several hundred lines:
  about 500 around `SegmentMap`, 140 in the value-edit door and 200 of
  provenance types. Part of the `SegmentMap` code stays as the strand
  walk. DM7's
  strand report becomes a difference between two sets of ids.
- **Ids can be diffed across versions.** A pin update can compute its
  strand report from the two versions alone. #3187 showed that
  positional names cannot do that.
- **Ratification checks** (`git log -S`):
  - V3's "index CANONICAL positions" was written by 9ee28b0c7a, which
    built Ev's PR 3102 ruling.
  - DM7's "rewritten in place" arm was written by 7b9423eeca, which
    built Ev's #2904 ruling.
  - DM7's value-edit arm was written by 94f7a773de and revised by e96a305bab (both #3180, agent-landed).
  - N1's "combinatorial identities" dates from the ledger move
    (585b3422ff).
  - The id rule keeps PR 3102's substance: correspondence by canonical
    `k` from the authored start. It changes only what the loft wall's
    name spells.
- **Prior art.** From general knowledge; not checked against source,
  and `references/` is absent in this checkout. Onshape sketch
  entities carry author-level string ids, and extrude faces are queried
  by them. FreeCAD 1.0's element map builds names from Sketcher
  geometry ids. Neither versions a rename ledger for sketch elements.
## 2026-09-24 — rim-piece ranks follow the finished body (PRs 3168 → 3167)

A union now numbers each member edge's pieces by the cells the
finished body cuts it into (#3168). Before, it ranked them per fold
step, so one rim-piece name denoted different pieces in different
member orders. #3167 (the shared-rim rule) lands with it. Alone,
#3167 would have turned refusals into silent rebinds: on the review
probe, 407 signature mismatches against main's 108. It was merged
only after #3168 had been merged into its branch.

What landed with the review rounds:
- the cell check moved into the `name_frag_` family;
- `of` counts cells, not pieces;
- the clustering is an order-free union–find, and refuses when the
  ambiguity band is narrower than 2 (`NarrowBand`);
- whole-group re-ranking in `cite_member_edges`;
- one same-side-rim rule;
- loud guards for a fold-ranked member-edge piece and for a moved
  vertex that has no single seam.

The rebind row is now able to go red. It pins the 25 cases that
refuse in some orders and publish in others (`KNOWN_MIXED`). The two
causes without an owner are filed P1 as
`union-refuses-in-some-member-orders-and-publishes-in-others`.

Measured on the review probe:
- names absent in one order: main 5200, #3167 alone 12478, both
  7742;
- no case is worse than main;
- the remaining absences all belong to
  `declared-flush-union-edge-and-vertex-names-follow-member-order`
  (P1), which now carries the evidence that the declared-flush body
  itself is order-dependent.

Filed:
- P2 `cite-member-edges-group-rerank-can-reverse-the-folds-rank-direction`
  (review O4b; unreached, untested branch)
## 2026-09-24 — a parent's held names rebind silently across a pin update (PR 3187)

Measured, and the row stays P0. A part inserts a leg before its
wall 1. The part's own door reports the rebind for names the part
holds. A parent that painted `InPart { part wall 1 }` then moves its
pin: `UpdateReference` reports nothing, and the held spelling now
denotes the leg. The new row `asm_parent_held_names` pins that
behaviour.

The fix needs new persisted state. Nothing connects an old pin to a
new one at `UpdateReference`:
- rename rows are not logged;
- the store keeps one snapshot per id;
- two snapshots cannot say whether a leg was inserted or a wall was
  replaced.

The fix would be a per-version rename ledger that `UpdateReference`
carries as data. The PR lists every carrier that holds a name across
a document boundary.

The loud interim (strand every held name at every pin move) is not
landed: it would break every mate on every update. The row is
`needs_ev`, together with
`a-value-edits-last-published-numbering-is-not-recipe-state`, whose
option A is the per-document half of the same ledger. Both go to Ev
as one question.
## 2026-09-24 — the group-size rung reads the emitter's groups (PR 3184)

`GroupResized` used to count a group by how its members' names were
spelled. Tied parents were then summed (4 → 2), and a face a split no
longer divided read 2 → 0. Now each emitter records the groups it
forms, by entity (`names::FragmentGroups`, not persisted), and the
rung only looks the count up. At a union the count is the distinct
published entities a fold step's group descends to, followed by
entity through every later step. Where a group is formed by names (a
seam group a tie formed), the rung declines. No name, stored bit or
`DIAGNOSIS_DIGEST` row moved.

The review took three rounds:
- Round 1 found a partly swallowed union group reporting its full step
  size.
- The fix for that matched rows across fold steps by name, which
  summed tied parents again.
- Round 3 moved the descent to emit time, by entity.

Documented as a known undercount: a piece a later step re-mints as a
`Seam` edge along its own line is not counted as the parent's
descendant.

## 2026-09-25 — a declared flush union names vertices and member edges the same in every order (PR 3198)

Four end passes in `emit_union::name_union` replace fold history with
facts read off the finished body:
- **`Flush`:** a flush stretch is named for the least member edge it
  lies along.
- **`least_vertex`:** a member corner is named for the least member
  vertex, among faces that descend there.
- **`crossing`:** a face crossing a member edge is `Seam{edge, face}`.
- **`retire_into_merges`:** a seam side cites the merge only when the
  merge is the face beside it. It never cites a duplicate or a false
  adjacency.

Absences on the rebind probe fell from 7398 to 816, with vertices and
member-edge pieces at 0.

The review found three regressions against main, all fixed with rows
that go red:
- a two-shell member refused;
- the ZIP document refused `Duplicate`;
- 46 seam sides named a face they do not border.

A new permanent row checks seam adjacency across the corpus. Cost is
about 1.1–1.3× main on a 100-step union chain.

Closed `declared-flush-union-edge-and-vertex-names-follow-member-order`.

Filed:
- P1 `union-face-names-follow-fold-order` (the faces that remain; the
  fork goes to Ev);
- via the PR, zip's leftover-vertex row and EMIT's
  `a-face-cut-and-merged-in-one-step-publishes-a-piece-under-the-name-its-merge-retires`.
## 2026-09-25 — the viewer shows each edit's DM7 rows (PR 3196)

The viewer used to keep only `cluster_rows()` of an applied edit, so no
`Strand`, `StrandedAppearance`, `OrphanedDeclare` or `Rebound` row ever
reached a GUI user.
- **Carried and shown.** `OpOutcome` now carries the rows. The chrome's
  status line shows one notice per row, in the row's own sentence.
- **Netted in one place.** The net over a multi-edit action lives in
  editor-core as `MaintenanceNet`. It takes an edit's rows and its
  after-document together (`&Applied`), and it checks that each row's
  claim still holds:
  - a strand survives only while its carrier still holds the name;
  - an orphan survives only while it is still unconsumed;
  - a rebound folds, and is dropped when a later edit strands or re-lands
    its target.
- **The panic.** `MaintenanceNet` panics if two surviving rebounds share
  a target. It is a bug assertion with a written proof, and it is
  reachable only from test-gated hand rows.

Two review rounds found:
- a rebound whose target a later edit stranded kept a false "still
  denotes" sentence;
- liveness checks tested existence, not the claim;
- the panic's stated reason was false (`Rebind` does merge names, but
  reports no rebound).

The lane pushed one empty commit to restart CI after a runner shutdown.
That is against the session rules; it stays in the history, and it was
not repeated.

Filed elsewhere:
- work/vseam: redo re-lands an edit's maintenance unreported (P3);
- work/chrome: cluster acts are not shown on the line (P4);
- work/chrome: a long cascade crowds the status line (P3);
- work/lib: Python has no `MaintenanceNet` door (P3).
## 2026-09-25 — Ev: profile pieces are named by minted step ids (PR 3193)

Ev ruled "yes this makes sense!" on #3193: a profile piece is named
`{ step, role }`, where the step id is minted when the step is
authored. The rule is N1's paragraph "the profile pieces". Three open
problems share one cause, a name spelled by a position that is
recomputed from current state:
- a parent's held names across a pin update (P0);
- a value edit through an unreadable state (P1);
- EDIT's zero-fit renumbering (#3163).

Under the rule nothing renumbers, so none of them arises.

- **Filed:** P0 unit `profile-pieces-are-named-by-minted-step-ids`,
  cost H, which builds the rule.
- **Parked on that unit:**
  - `a-child-documents-rebind-leaves-the-parents-held-names-in-the-old-numbering`;
  - `a-value-edits-last-published-numbering-is-not-recipe-state`;
  - `the-value-edit-numbering-check-costs-a-replay-per-swept-profile`.
- **Not EMIT's to close:** EDIT's row, and EDIT's #3158 retirement
  question. Both are moot under the rule, and EDIT's orchestrator closes
  them.

## 2026-09-25 — Ev: union contact is pairwise, before the fold (PR 3200)

The first recommendation was that a flush contact covered by a third
member is not a contact. Ev rejected it: "a whole set can get out of
having any declared contacts just by having none of the contacts be
blamed on a single pair". The ruled rule:
- every touching member pair is judged as its own two-member union,
  before the fold;
- an undeclared contact refuses in every order, and that includes a
  covered one;
- a declared contact is satisfied wherever the fold meets it.

DM4 is re-worded, and its footer records the ruling. Filed the P1 unit
`union-contact-is-judged-pairwise-before-the-fold` (cost D). Measured
across the fixtures: 5 of 153 member pairs touch undeclared, and the
only new refusals are `row` and `rowids`.

## 2026-09-25 — Ev: step roles (PR 3202)

Ev answered the step-id build's three open questions:
- **Roles.** Roles are the path-language side of the name. Ev noted that
  the path algebra and its lowering are "two ways of describing the
  same thing"; user-facing text keeps the language the path was
  written in.
- **Circles.** A circle is `Piece(0)`/`Piece(1)` for now, and the P0
  step-id build ships without waiting.
- **Loft seams.** One vertex locator per section.
- **Q4 withdrawn.** Ev was right: a fillet never has an authored corner,
  so no authored point leaves the path.

Ev raised the deeper point: vertex + bulge cannot express a full turn,
so a circle is split in two and the lowering diverges from the
authored path. EMIT filed it on PATHS's slate as
`lower-profiles-to-carrier-and-interval-not-vertex-and-bulge` (P1, H),
with a recommendation that a dedicated PATHS orchestrator take it.
`needs_ev` is cleared on `profile-pieces-are-named-by-minted-step-ids`.

## 2026-09-25 — GroupResized names the seams that changed (PR 3205)

`GroupResized` now carries `cutters: GroupCutters`, with these arms:
- `Read { gone, new }`: the seams on the group's parent that only one run
  spells, read from both tables;
- `NotSeamBounded`, `TiedParents`, `NoSeamOnRecord` and `SeamUnread`:
  each says why it cannot read, and none claims without evidence.

The review found several misreports:
- a partial read (deep `[Seam, Frag, Frag]` rows) claimed "same cutters"
  or a false `gone`;
- a cutter that the fold re-ranked read as both gone and new;
- the docs said "stopped cutting" where the evidence is a seam spelled in
  one run;
- two walls rendered identically.

All are fixed:
- one fragment-tail helper, plus the `SeamUnread` arm;
- union cutters compared with their fold tail stripped;
- relabelled docs;
- role words in Display.

`DIAGNOSIS_DIGEST` moved once: `flip-vanish` now names B's cap vertex as
gone.

Known limit: a cutter vertex fused onto the parent edge reads as gone.
It is documented and not detected, because telling it apart needs the
body. The change is additive to N5 and was not taken to Ev, per his
ruling on #3115.

## Announced seam from PATHS (2026-09-25)

Ev ruled on #3218 that a profile lowers to verbatim vertices +
`Line | Arc { centre, radius, Δθ }`, so a circle becomes one segment.
PATHS's `circle-lowers-to-one-segment` (unit 4 of 6, parked behind three
refactor units) will re-spell a circle's step-id pieces from
`Piece(0)`/`Piece(1)` to one `Carrier`, which is the second names break
agreed on #3202. Nothing is needed from EMIT now. PATHS will announce
again before unit 4 dispatches.

Signed (PATHS orchestrator).
