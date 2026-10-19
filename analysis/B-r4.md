## For Ev

**Recommendation (likely): yes, with one adjustment to how it is described.** Write the arrival-known arc as a verb with no `r` that opens the fillet family's **arrival side**, so the arc's end is bound by the binders a fillet's arrival already uses. Working name `arc_onto()`; the alternative is `half_fillet()`.
- The arc starts exactly at the tip, untrimmed and with no tangency required there.
- It ends at the arrival side's anchor, tangent to the arrival side's direction.

This is better than both earlier designs. There is no direction inside an arc spec. There is no pending state whose `.toward` means something different from what it means after every other fillet verb. There is no fused-row question. And the seam forms come from the fillet family rather than being added.

**The adjustment.** It is a fillet in its *end* only. Its *start* is `arc_to`'s: it consumes a positioned tip with no outgoing direction, exactly where a fillet is ill-typed, because "a bare Point cannot fillet". So it should not carry `fillet` in its name, and it belongs beside `arc_to` in §2's Legs list, with "completes through the arrival builder" stated.

**Which reading of your question.** I take "a fillet, just not necessarily [tangent where it meets the incoming]", that is, tangent at arrival and free at departure.
- The opposite reading is tangent at departure and free at arrival. Without `r` that is `tangent_arc_to(p)`, which already exists. With `r` it is a ray extension, then an arc, then an end at a point by a sharp joint. That is what `.line(len).tangent_arc_to(p)` already spells, and nobody has a case for it.
- A third reading keeps `r` and drops only the departure tangency: an arc of radius `r` from the tip, tangent to the arrival line, then a straight run to the anchor. It is well-defined, but it does not serve the `B`, where the bump must *end* at the foot and its radius is unknown. I would not build it.

**Its cells.**
- **From:** wherever `arc_to(Bulge { .. })` is admitted. That is a plain point (including the entry `Open.at(p)`, where `Start`'s direction is then derived from the arc, as it is for `Bulge`) and a leg end. It is ill-typed from a Directed tip, where the departure is already bound and the arc would be over-determined.
- **The departure joint at a leg end** is classified like any `arc_to` start tangent:
  - definitely sharp: proceeds;
  - zero turn: refuses `JunctionTangent`, whose recourse names `.tangent().tangent_arc_to(p)`;
  - reversed: refuses `JunctionCusp`.
- **The end: the arrival builder, unchanged.**
  - `.toward(d).to(p)` (direction first; the far-end anchor of §2a item 4): the arc ends at `p` arriving along `d`, and the tip is a directed point with incoming `d`. This is the `B`'s bump, and what the earlier `Arrive` mode did.
  - `.toward(d).at(p)` / `.at(p).toward(d)`: the same arc, but the side carries on through `p`. The tip is Directed along `d`, and the joint at `p` is a constructed tangency, as at a fillet's end.
  - `.angle(θ)` works as `.toward` does throughout. There is no straight run: with no `r`, the tangent point is the anchor itself.
- **Degenerate cases** (mirrors of `tangent_arc_to`'s):
  - `p` at the tip: `DegenerateArcChord`;
  - `d` along the chord: a line, not an arc, refused;
  - `d` against the chord: the full turn, refused.
- **Seam.**
  - `.to(Start)` is the tangent closer: the arc arrives along `Start`'s own direction, which is taken by reference. It is the fillet family's existing seam spelling (`.fillet(r).to(Start)`), so no token is needed, and it is issue 1578's closer.
  - `.toward(d).to(Start)` is a sharp seam with an authored arrival. It is checked against `Start`'s direction, and a zero turn refuses `SeamTangent`, whose recourse is `.to(Start)`. This is the far-end-at-`Start` form §2a item 4 left "well-defined… if a case appears, it is an addition". The `B` is that case, and the form should be admitted for the line arrival of every fillet verb, not only this one.
  - `.at(Start)` stays ill-typed, because `at` takes a position.
  - `Start.arrives_tangent()` is not used by this verb. The arrival side's direction is bound by reference instead.
- **As an arc arrival (`spec₂`):** refused. With no `r` the arc ends at the anchor, so an arc arrival run would have zero length, and the spec would only supply a direction at `p`. "Arrive tangent to the circle about `c`" is a fair future ask, but it is a different datum.
- **Fused, arc-incoming form:** none. There is no incoming side to trim. So the round-7 "sharp→filleted at the same call site" property is untouched: this is not an `arc_to` mode.
- **The `Via`/`Center` token half is unaffected.**
  - `Start.arrives_tangent()` on every closer: the kernel checks it where the leg's data fix its end direction, and supplies it where they leave it free. The `Via` fillet arrival at `Start` is the one place it supplies.
  - It closes issue 1579.
- **What it replaces:** no existing verb. It replaces the proposed `Arriving` mode in both its spellings. `arc_to` keeps its six modes.

**Worked example: your `B`, entered at the spine's foot.**
```text
Open.at(foot).toward(0, 1).line(h)          // spine
    .toward(1, 0).tangent_arc_to(mid)       // upper bump: departure known
    .arc_onto().toward(-1, 0).to(Start)     // lower bump: arrives west at the foot; sharp seam, checked
```
With the entry elsewhere, the lower bump is `.arc_onto().toward(-1, 0).to(foot)`, and the spine follows with `.toward(0, 1)…`. A `D` whose cap meets the entry tangentially is `.line_to(tip).arc_onto().to(Start)`.

**What it leaves possible.** A position-only arrival (`.arc_onto().at(p)` and nothing more) is a plain point awaiting a director, as in any fillet arrival. Following it with another arc, for example `arc_to(Bulge)`, supplies the direction backwards, as `Center@Point` already does. That gives two arcs meeting tangentially at `p`, built by construction. It is coherent and inherited, not designed; say if it should be cut.

**Confidence:**
- Reading and recommendation: likely.
- Cells: likely.
- The token half being unaffected: sure.
- Admitting the far-end `Start` form for every fillet verb: likely.

## For the orchestrator

- **Where it lands in code.**
  - The pending incoming today is `PendingRay` or `PendingArc` (`crates/profile/src/path/verbs.rs`), and the arrival completes through the captured `ArcResolver`. The new verb is a third pending incoming, a bare point (tip position only, no radius), plus a resolver for "the arc from the point, tangent to the arrival carrier at its anchor".
  - The arrival builder and its `At`/`Angle`/`Toward`/`FarEndTo`/`CloseTo` rows are reused. I did not check that the `Open` arrival state's type is generic enough to hold the third incoming without its own tip state; it may need one `DynTip` variant.
- **The far-end `Start` form** (`.toward(d).to(Start)`) is new for every fillet verb. Today `FarEndTo` takes a `Point2`, and `CloseTo` binds both bits. That widening is the one change to ratified text outside this verb: §2a item 4's "Open, deliberately" paragraph, agent text from the G1 round. I did not run provenance on it this round.
- **Corrections to the orchestrator's reading.**
  - It is right on the end. Its "a fillet-family verb" describes the result but misplaces the start state, which the bullet on the adjustment covers.
  - The `.at(p)` vs `.to(p)` distinction matters. Only `.to(p)` gives the `B`'s sharp corner at the foot; `.at(p)` continues tangentially. The PR text should show both.
- **Ratified text to change:**
  - §2 Legs (a new leg beside `arc_to`);
  - §2a item 4 (the far-end `Start` form);
  - §2 Closure (`.to(Start)` named as the tangent closer for this verb);
  - §3's verb table.
  - The `Arriving` mode text drafted in PR #5374 retires. The token half stays as drafted.
- Read-only; nothing built or committed. The worktree `/home/user/design-arrival-b` is removed.
