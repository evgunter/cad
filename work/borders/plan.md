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
