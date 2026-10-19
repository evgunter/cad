# SEAMRANK — the plan

Opened 2026-10-19 by EMIT's close (`docs/doc-ledger/emit-leaves-the-tracker.md`).
Every row moved from `work/emit/` by `git mv`, id kept.

## The slate

**33 budget points** of dispatchable work against a ceiling of 33
(unpriced rows charged 2.5).

| pri | item | cost | title |
|---|---|---|---|
| P2 | `a-conventional-vertex-mints-no-edge-derived-name` | H | Naming a conventional vertex refuses typed: its edge-derived name, the ranking skip and the modulo-position order rows are unbuilt |
| P3 | `a-crossing-of-a-seam-edge-with-no-first-side-refuses` | M +design | A crossing of a seam edge whose two sides carry one name has no orientation to read its sense along, and refuses where a lone one used to be |
| P3 | `a-flush-reading-finds-no-point-on-a-nurbs-member-edge` | M | The flush rule's Track reads every point as off a NURBS member edge, because Curve3::param_near has no parameter for a NURBS carrier |
| P3 | `a-unions-flush-reading-cannot-follow-a-nurbs-carrier` | M | A union's flush reading cannot follow a NURBS carrier: such an edge is a line of its own and covers nothing, whatever lies flush with it |
| P3 | `an-end-touch-crossing-has-no-witness-once-edges-join` | M | The end-touch crossing (an edge lying in a face that ends in it) has no test witness since the output stage joins the area-overlap fixture's |
| P3 | `an-in-face-crossing-is-spelled-as-a-seam-of-the-face-it-lies-in` | — +design | A vertex where an edge lying in a face ends in it is published as Seam{merged face, its own seam}, losing which member it marks |
| P3 | `crossing-sense-carries-five-copies-of-the-null-copy-closure` | — | The crossing-sense code holds the null-copy closure in five places and two rules for matching a crossing's sides by image |
| P3 | `crossings-of-a-seam-edge-whose-sides-names-do-not-settle-tie` | M | Crossings of a seam edge whose faces' names do not settle the pair's first side tie, where N2 rules only that an equal pair ties |
| P3 | `meridian-edge-has-no-minting-builder` | E | the minting builders spell a meridian vertex but not a meridian edge, so a seam or cap meridian is hand-spelled |
| P3 | `n2s-crossing-clause-states-only-part-of-the-tie` | E | N2's crossing clause says only an equal pair ties, where rank_crossings also ties an unequal pair whose order it cannot certify |
| P3 | `seam-line-ranks-keep-a-sided-axis-and-a-collapse-correction` | D | Every ranker along a seam line ranks along the minted pair's n_a × n_b, so a commutative consumer needs RankRule to re-orient it; a canonica |
| P3 | `seam-rank-extents-compare-carrier-parameters-as-metres` | M | The crossing ranks compare a curved carrier's parameter extents against a band in metres |
| P3 | `two-seam-runs-whose-seams-differ-in-unopened-faces-can-read-alike-within-the-evaluation` | M | Two seam-run vertices whose seams differ only in faces the detail leaves unopened read alike within the evaluation |
| P3 | `union-in-every-order-helpers-are-copied-six-times` | E | The 'union in every member order' test helper is written six times across editor-core's union suites |


## Order

By band: the P2 row first, then the P3 rows that refuse or tie on a
document over the hygiene rows, which ride with whichever unit next
opens their file. A `+design` row opens with its question, weighed by
a designer pair before a lane (`docs/DESIGN-FORK-PROTOCOL.md`).

## Review posture

A unit that changes what a name says is a full review (falsifier and
style); hygiene that moves no name is a style review.
