# VGEOM — log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/vgeom/plan.md`. A/B band 5300–5399
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-17) — opened in VIEW's re-scope

Opened by VIEW's orchestrator as one of four successors on VIEW's
ground, per `work/README.md`: *residue is re-homed before the sweep …
to a new program opened for it when the residue coheres into a track of
its own (a dozen items on one territory are a successor's opening
slate, and the closing program opens it)* (Ev, 2026-09-06).

**VIEW is not closed by that act and is not closed by this one.** Its
`Order`'s six units are done, deferred or handed off; its ninety-four
live rows were review accretion on one crate, and four of them cohere
into tracks. VIEW stays open with eight rows, its exit walk is a
separate ratified step, and `docs/DOC-LEDGER.md` records the sweep when
it happens.

21 rows arrived from `work/view/`, each by `git mv` with its body,
its id and its history unchanged — no row's prose was edited on the way
past, and the item schema carries no `program:` field, so a re-home is
the move and nothing else:

- `a-count-slot-launders-a-typed-nan-into-zero`
- `a-fields-text-commits-within-the-renders-own-tolerance`
- `a-flat-rung-pair-is-read-as-flat-below-it`
- `a-nan-edge-distance-wins-its-boundary-rather-than-losing`
- `corner-count-substitutes-u32-max-for-a-length-it-could-not-cast`
- `ctrl-wheel-reaches-no-zoom`
- `cursor-projection-is-f32-in-a-module-whose-matrices-are-f64`
- `finite-bounds-yield-an-infinite-scene-radius`
- `flatten-emits-every-vertex-before-it-judges-any-of-them`
- `id-readback-failure-reads-as-nothing-under-the-cursor`
- `pickindex-merges-parts-on-a-rounded-t-it-never-converts`
- `pickindex-tie-break-rests-on-a-comment`
- `renders-that-multiply-a-finite-guarded-length-spell-the-product-inf`
- `sketch-headings-guard-zero-length-but-not-an-infinite-one`
- `the-budgets-predicted-count-is-not-always-an-over-count`
- `the-fields-door-has-no-width-bound-at-all`
- `the-one-free-transform-is-the-only-total-door-in-camera`
- `the-point3-to-gpu-corner-cast-is-at-three-sites`
- `the-shader-encodes-a-mark-strength-nothing-bounds`
- `viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep`
- `world-per-px-answers-a-scale-for-a-viewport-it-could-not-measure`

The charter that makes these one program, and the sentence that is true
of them and false of the other three tracks' rows, is `plan.md`
§Charter. The band 5300–5399 is claimed in `docs/MODEL-AB-LOG.md` in the
commit that opens this program, per that entry's own rule.

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`
if the posture ever changes; under the inherited posture it records no
row.

## 2026-09-19 — Ev's grid and committed-profile rows landed (PR 2859)

Orchestrated from a session that took Ev's high-priority GUI rows
across the paused viewer programs. Not an A/B-protocol unit, by Ev's
instruction. One unit took both rows, since they share the per-lane
overlay work. The first lane (Fable) died at the account limit before
writing anything; Opus implemented the unit.

The review had a correctness lane and a style lane. It found no MAJOR:
the uniform layout, vertex-word packing, blend state and depth
behaviour all held. It found two MINORs:
- the just-added profile was drawn twice, once as its preview;
- the undrawn-profile badge had no test. The fix pass showed the path
  is reachable, through a denormal bulge.

The fix pass also took most of the style findings:
- the shader's lane-colour switch is generated from `EdgeLane`, not a
  hand list;
- lane codes are the discriminant;
- a profile-against-grid-and-preview separation test. It moved two
  palettes' profile colours.
- an absolute bound on the grid's width;
- a row for a rotated, offset plane;
- four stale docs.

Declined: merging `CommittedProfile`'s loop type with `PreviewLoop`.
`path_authoring.rs` asserts on the vertex index, and PR 2862 is
rewriting `sketch.rs`.

The review ran on reading alone: the machine-wide build slot was held
for an hour. CI is the test record.
