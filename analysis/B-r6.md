## For Ev

**Recommendation (likely): the arc is the arrival itself, not a fillet-shaped arc onto a straight side.** `arrive()`, a verb with no arguments on a point tip, opens an arc-arrival builder.
- **`.at(p)` plus a director (either order):** the arc from the tip to `p`, arriving along that direction. The tip lands as a directed point at `p` with its departure free.
- **`.to(Start)`:** the tangent closer; it arrives along `Start`'s own direction, taken by reference.
- **`.at(Start)` plus a director:** the sharp seam, checked like every seam (a zero turn refuses `SeamTangent`, whose recourse is `.to(Start)`).
- **Where it starts:** from a plain point, or from a leg end, where the derived departure is junction-checked: a tangent departure refuses `JunctionTangent`, naming `.tangent().tangent_arc_to(p)`.
- **Ill-typed:** from a Directed, `Angle` or `Open` tip.
- **Degenerate cases:** mirrors of `tangent_arc_to`'s.
- **No fused, `spec₂` or token cells.** The `Via`/`Center` half of the PR is unaffected.

**The deciding argument.** This verb has no `r`, so the point where the arc meets the line side is the anchor itself, and the line side never has any length. A "line arrival" here is only a direction at `p`, which is exactly what an arc arrival lands with. Treating it as a line side therefore gives its two binders different meanings without giving them different results:
- `.at(p).toward(d)` (the side "runs on") builds the same loop as `.toward(d).to(p).tangent()`: the same vertex at `p`, the same tangent joint, and the same next leg.
- That is two spellings of one loop inside one verb, which the design refuses everywhere else (the entry rule; "one mechanism").

For a real fillet the two are different: `.at` lets the next straight leg extend the side's own segment, so there is no vertex at `p`. Here there is always a vertex at `p`, because the arc ends there. So line grammar's distinction carries no meaning on this verb, and the arc-arrival grammar is the one that fits.

**What the author means.** "It arrives along `d` at `p`" describes the arc's end, not a straight side. In the `B`, the lower bump is followed by the spine, which is a sharp corner, so there is no straight run after `p`. Where one is wanted, `.tangent()` says so, as it does after every leg.

**The seam row.**
- `.at(Start)` adds one row, on this verb's own states. On an arc arrival `.at` ends the arc at its anchor, so anchoring at the entry has a meaning. `Via { q, p: Start }` (the `Via` fillet-arc arrival anchored at `Start`, whose director closes the loop) is the same construction with its anchor carried in the spec.
- It leaves untouched §2a item 4's open far-end `Start` form and the closed `at`/`Start` issue, which concerned line arrivals, where `.at(Start)` means nothing.
- The alternative, `.toward(d).to(Start)`, sits on the state every fillet verb shares, so it would also rule that form for every fillet verb with no case that needs it.

**Your `B`, entered at the foot:**
```text
Open.at(foot).toward(0, 1).line(h).toward(1, 0).tangent_arc_to(mid).arrive().at(Start).toward(-1, 0)
```
Entered elsewhere, the bump ends `….arrive().at(foot).toward(-1, 0).toward(0, 1)…`. A `D` whose cap meets the entry tangentially is `.line_to(tip).arrive().to(Start)`.

**If you don't accept the argument, the split is:**
- (arc) `arrive()`: `.at(p)`+dir ends the arc at `p`, and the sharp seam is `.at(Start)`+dir. One spelling per loop, and it adds the least.
- (line) `arc_onto()`: `.at(p)`+dir runs on along the side, `.toward(d).to(p)` stops at `p`, and the sharp seam is `.toward(d).to(Start)`. It reuses every line-arrival row, gives one shared far-end `Start` row for every fillet verb, and has two spellings of a tangent continuation.

## For the orchestrator

- **Why we crossed twice.** Each of us read the verb's arrival as whichever builder we were not using. The question underneath is whether a zero-length line side is a side at all. The argument above settles it from the emitted structure. I hold the result as likely, not sure, because it rests on the claim below.
- **The claim it rests on (traced from the text, not run):** after an arc, `.at(p).toward(d).line(x)` gives the same loop as `.to(p).tangent().line(x)`. The §4 item 4 same-segment exemption applies only after a straight run, and §2a item 4 states the exact-fit rule ("the side simply IS the arc"). If the implementing unit finds that the two set different joint flags, that difference is the only thing the line grammar would buy, and it should be reported.
- **A's inherited cell disappears under the arc builder.** In A's line-side cells, `.at(p)` alone followed by an arc leg leaves a plain point whose direction the next arc supplies backwards. Under the arc builder, `.at(p)` alone is a pending state awaiting its director, as `RadiusArrivalAt` is. That is one silent cell fewer.
- **Implementation:** three builder states mirroring `RadiusArrival` / `RadiusArrivalAt` / `RadiusArrivalDir`, holding the tip point; `.at` rows that take `Point2` or `Start`; a `CloseTo` row; and `tangent_arc_geom` with the reflected direction, landing through `leg_end_tip`.
- Read-only; nothing built or committed.
