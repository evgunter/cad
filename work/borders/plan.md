# BORDERS — the plan

Opened 2026-10-19 by EMIT's close (`docs/doc-ledger/emit-leaves-the-tracker.md`).
Every row moved from `work/emit/` by `git mv`, id kept.

## The slate

**34.5 budget points** of dispatchable work against a ceiling of 30
(unpriced rows charged 2.5).

| pri | item | cost | title |
|---|---|---|---|
| P2 | `a-direct-operands-own-split-lineage-climbs-to-a-live-sibling-seam` | M | Borders reads a Direct operand's own split lineage up to a live sibling edge and credits a piece with the sibling's discard seam (filed by FLUSH, 2026-10-26) |
| P2 | `no-reader-reads-a-fallbacks-uncut-faces` | M +design | No naming reader reads BooleanNaming::uncut: a name that depends on a dropped face the fallback could not divide is neither refused nor marked (filed 2026-10-25) |
| P2 | `a-union-over-a-stepped-plate-refuses-in-the-orders-that-fold-the-plate-early` | M | Unions over a stepped plate or a flush engulf refuse 'a piece of a face held as several borders no recorded discard between them' in some me |
| P3 | `a-discard-border-inside-a-merged-face-or-joined-edge-reads-as-no-border` | M +design | A discard's bordered stretch whose end settles inside a merged face or a joined edge is read as no border |
| P3 | `a-face-whose-discarded-fragments-share-a-chord-has-no-fixture` | M | No fixture pins the Borders obstacle join of two discarded fragments of one face across a chord |
| P3 | `a-grafted-b-piece-whose-parent-the-merge-killed-chases-short` | E | A grafted B piece whose parent the boolean's merge killed: chase_b reads no output-stage hops, so its lineage stops at the dead parent |
| P3 | `a-lone-cut-piece-of-an-operand-edge-keeps-its-bare-name` | M | A lone piece of an operand or member edge a cut shortened keeps the edge's bare name |
| P3 | `a-split-join-that-kills-a-split-child-has-no-row` | M | No split row's join kills a split child, so SplitNaming::joined_lineage is never read |
| P3 | `a-union-seam-cut-to-one-piece-keeps-its-bare-name` | M | A union seam a later member cuts to one piece keeps its bare name, so it is renamed when a second piece appears |
| P3 | `are-the-upstream-flip-and-recipe-edit-rungs-reachable-from-emitted-names` | M | Are the Upstream flip and RecipeEdit rungs still reachable from names the emitters mint, now that edge pieces cite their cutters? |
| P3 | `boolean-naming-carries-the-join-rows-twice` | E +design | BooleanNaming carries the output join rows twice, as edge_joins and folded into substitutions, and a hand-built naming can make them disagre |
| P3 | `borders-and-keeps-spell-a-lone-piece-bare` | — +design | A face piece alone on its side is spelled bare, so it gains Borders or Keeps when a sibling appears |
| P3 | `borders-records-a-join-twice-and-reads-lineage-three-ways` | M +design | borders.rs spells a join's seam handover twice (eager seams extension and lazy joined_from chase) and reads split lineage from three sources |
| P3 | `section-face-and-hole-rim-are-ordinals-over-their-group` | — +design | SectionFace { section } and HoleRim { hole } are ordinals over a group, so a new section or hole renumbers the others |
| P4 | `the-cone-split-relation-has-two-homes-and-the-vertex-pass-two-ladders` | M | The cone-split relation lives in two places, name_boolean_vertices dispatches through two ladders, and Obstacles::records is threaded by han |

Parked, and not counted:

| pri | item | waits on |
|---|---|---|
| P2 | `a-whole-ball-cut-by-a-tilted-brick-refuses-an-unclassified-member-edge-side` | see the row's `blocked_on` |


## Order

By band: the P2 rows first, then the P3 rows that refuse or tie on a
document over the hygiene rows, which ride with whichever unit next
opens their file. A `+design` row opens with its question, weighed by
a designer pair before a lane (`docs/DESIGN-FORK-PROTOCOL.md`).

## Review posture

A unit that changes what a name says is a full review (falsifier and
style); hygiene that moves no name is a style review.

## From WIREREST (merged 2026-11-20)

WIRE's second P3 and P4 tail: a split's plane and seam-vertex residue, a union's fold-step and coincidence-row residue, and naming tests that do not hold at every K.

Opened 2026-10-20 by WIRE's cut. Nothing dispatched.

### The slate

**29 budget points** against 30 (re-weighted tiers, 2026-10-28).

| pri | item | cost |
|---|---|---|
| P3 | `a-halfs-plane-has-two-definitions-and-n3-linking-two-homes` | M |
| P3 | `a-plane-piece-the-nesting-rule-cannot-place-reads-as-bordering-the-exterior` | M |
| P3 | `a-split-merge-of-one-operand-faces-pieces-is-a-merged-of-one` | E +design |
| P3 | `a-split-merging-a-two-piece-wall-reads-its-seam-vertex-as-a-pinch-corner` | M |
| P3 | `a-union-folding-a-bridging-prism-last-refuses-its-fold-step` | H |
| P3 | `a-zip-listed-chord-between-merged-faces-along-a-rim-has-no-end-to-end-witness` | M |
| P3 | `three-names-tests-fail-at-ambiguity-k-1-5` | M |
| P3 | `a-unions-member-edge-group-size-reads-the-fold-steps-spelling` | M |
| P3 | `identical-twin-coincidence-rows-arise-on-coincident-lines` | E |
| P4 | `the-judgement-naming-fault-seam-refuses-other-tests-unions` | E |

Parked, and not counted: `an-order-along-rank-spells-its-groups-size`
(P3 M +design), on WIRE's `a-unions-line-of-several-edges-is-oriented-by-its-least-edge`,
whose build of FORK-ORIENT (nothing is ranked) retires `OrderAlong`.

### Order

- **The split rows come first, as one unit.** These are the first four rows of the slate: the half's plane, the nesting limit, the merge of one, and the two-piece wall. All four were found around PR 5486, and all read `topo::SplitNaming`.
- The D9 rows and the verb-door row moved to WIREREFUSE on 2026-10-28 (`work/doctail/plan.md`).

### Review posture

Inherits WIRE's (WIRE's plan, recoverable by `docs/doc-ledger/wire-leaves-the-tracker.md`): protocol v7, single reviews.

**Class note from WIRE's 2026-10-20 sitting:** a row-4 (`unreachable!`) argument must hold for every K > 1 and every ε that `Tolerance` accepts. It must not rest on two independent tolerance decisions agreeing. Review probes with `CAD_AMBIGUITY_K=1.5` and `CAD_TOLERANCE_EPS=1e-6`.

### WIRE's remainder, moved in 2026-11-03

WIRE closed with eleven P3 rows and nothing above them
(`docs/doc-ledger/wire-leaves-the-tracker.md`); they moved here by
`git mv`, ids and bodies unchanged, with WIRE's `paths`. None counts
towards the load, which only weighs P0, P1 and unbanded rows.

| item | cost |
|---|---|
| `a-boolean-records-a-face-pair-decision-only-where-the-two-faces-meet` | E +design |
| `a-boolean-zero-decided-and-consumed-in-place-reaches-no-row` | M |
| `a-split-records-an-on-vertex-only-where-it-leaves-two-runs` | E +design |
| `a-split-lineage-lists-a-joined-edge-over-chords-and-an-operand-rim-as-minted` | M |
| `a-splits-joined-seam-and-set-piece-keys-have-no-tie-witness` | E |
| `a-joined-set-over-two-tied-operand-edges-loses-its-operand` | M |
| `a-flush-pairs-rim-point-is-spelled-two-ways-by-fold-order` | M |
| `a-unions-tied-fold-rows-at-a-member-corner-bypass-the-corner-rule` | M |
| `union-pairwise-verdicts-that-disagree-through-lineage-refuse-as-a-kernel-defect` | M |
| `an-assembles-contact-carry-raises-emission-for-a-bridge-the-graft-keeps-total` | M |
| `a-crossings-sense-residue-left-in-the-emitters-and-the-remap` | E |

The two `+design` rows are one question: whether a decision a boolean
or split takes and uses in place is a published row. Weigh them
together before building either.

## From SEAMRANK (merged 2026-11-20)

Opened 2026-10-19 by EMIT's close (`docs/doc-ledger/emit-leaves-the-tracker.md`).
Every row moved from `work/emit/` by `git mv`, id kept.

### The slate

**18 budget points** of dispatchable work against a ceiling of 33
(unpriced rows charged 2.5).

| pri | item | cost | title |
|---|---|---|---|
| P2 | `a-conventional-vertex-mints-no-edge-derived-name` | H | Naming a conventional vertex refuses typed: its edge-derived name, the ranking skip and the modulo-position order rows are unbuilt |
| P3 | `an-end-touch-crossing-has-no-witness-once-edges-join` | M | The end-touch crossing (an edge lying in a face that ends in it) has no test witness since the output stage joins the area-overlap fixture's |
| P3 | `an-in-face-crossing-is-spelled-as-a-seam-of-the-face-it-lies-in` | — +design | A vertex where an edge lying in a face ends in it is published as Seam{merged face, its own seam}, losing which member it marks |
| P3 | `crossing-sense-carries-five-copies-of-the-null-copy-closure` | — | The crossing-sense code holds the null-copy closure in five places and two rules for matching a crossing's sides by image |
| P3 | `meridian-edge-has-no-minting-builder` | E | the minting builders spell a meridian vertex but not a meridian edge, so a seam or cap meridian is hand-spelled |
| P3 | `two-seam-runs-whose-seams-differ-in-unopened-faces-can-read-alike-within-the-evaluation` | M | Two seam-run vertices whose seams differ only in faces the detail leaves unopened read alike within the evaluation |
| P3 | `union-in-every-order-helpers-are-copied-six-times` | E | The 'union in every member order' test helper is written six times across editor-core's union suites |
| P4 | `emit-union-track-caller-helpers-panic-inside-closures` | E | emit_union.rs's #[track_caller] lookups panic inside unwrap_or_else closures |

Parked on WIRE's `a-unions-line-of-several-edges-is-oriented-by-its-least-edge`
(the build of FORK-ORIENT, fork-log row 197: nothing is ranked, a seam
is read along each side), and not counted:

| pri | item | cost |
|---|---|---|
| P3 | `a-crossing-of-a-seam-edge-with-no-first-side-refuses` | M +design |
| P3 | `seam-rank-extents-compare-carrier-parameters-as-metres` | M |

### Order

By band: the P2 row first, then the P3 rows that refuse or tie on a
document over the hygiene rows, which ride with whichever unit next
opens their file. A `+design` row opens with its question, weighed by
a designer pair before a lane (`docs/DESIGN-FORK-PROTOCOL.md`).

### Review posture

A unit that changes what a name says is a full review (falsifier and
style); hygiene that moves no name is a style review.
