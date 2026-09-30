# CONTACT — the plan

Touches, overlaps and declared contacts: the ordinary solids the boolean lane will not combine.

Opened 2026-09-20 by REACH's priority-seam cut (`work/README.md`,
Track size). Landed: CONTACT-1 through CONTACT-5 (the log has each).
In flight: CONTACT-6, the cut-cavity false `Out`, which turned out to
be `split`'s section-face sense bit (in review).

## The slate

`python3 scripts/work.py status --program contact` is the live table;
this section says only what the table cannot.

**Two tracks, by the file they edit.** Arm 2 of
`sweep_cross_solid_backstop` in `census.rs` is one track: three open
rows edit it, and they run one at a time. The boolean's rest door and
face merge (`boolean/rest.rs`, `merge_faces.rs`) is the other, and it
runs in parallel.

### The census track, in order

1. **`touch-cone-readings-are-levered-directions-not-face-distances`**
   (P1, H), with **`census-touch-cones-are-a-third-vertex-sector-builder`**
   (P1) riding along. This comes first although the next row is a P0.
   The analysis reads unit directions times a lever, and five review
   rounds have each found that lever wrong somewhere (the `sin α` gap is
   open). CONTACT-5 now routes every meeting pair through this analysis,
   so every later census unit stands on it. The fix reads face
   distances, which is a redesign, so a designer pair weighs it first
   (`docs/prompts/designer.md`).
2. **`declared-only-meetings-clear-at-the-census-gate-unread`** (P0,
   H). Read each face-pair-backed event through `TouchSite::verdict`,
   and decide what a curved declared rest owes. The second half is a
   design question. No wrong clear has been built, so it waits for the
   analysis it will extend.
3. **`overlap-lane-boundary-crossing-cuts`** (P0, H), the D3 cut
   schedule. This is the blocker for the `ef_bound_backed` migration,
   not a wrong answer standing today.

### The boolean track

- **`area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step`**
  (P0). The row names three fixes: merge the caps, accept an operand
  pair a prior step admitted, or rename the refusal. That is a design
  fork, so a designer pair weighs it first.

### Smaller rows, taken when a unit opens their file

- `revolved-tube-wall-refuses-bool-wall-trim-period` (P1).
- The P2s: `a-touch-at-a-saddle-corner-refuses-unanalysed`,
  `ray-wall-and-cone-near-root-cancels-over-a-small-lead`,
  `torus-split-lead-escalates-a-legitimately-small-resolvent-root`.
- The P3s, and the unprioritised `cone-chart-trim-…`,
  `sphere-chart-trim-…` and `contact-refusal-prose-outgrows-the-viewer`.

### Ev's channel

`the-dual-review-streams-first-readout-is-owed` (`needs_ev`). The
readout is off-file on `analysis/dual-review/readout-1`; the orchestrator
does not read it.

## Review posture

Named per unit in its item file. The lesson of this track, stated so
that briefs carry it: **every defect the reviews found in CONTACT-1, 4
and 5 was a lever or a bound that is conservative for one verdict and
unsound for another** (a Zero that abandons a ray, against a Zero that
is a verdict). A brief for any unit that levers a `decide` asks, per
verdict, which bound makes that verdict sound.
