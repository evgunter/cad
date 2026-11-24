## For Ev

**Recommendation as it now stands (likely).** The two reports agree on the final state, and the orchestrator's reading of what they agree on is right.
- A validated line side records its director (its unit direction) and its signed length together. Both are minted at one site, so that start + length·director is the end as a formula.
- Every reader reads that pair and never the two points: the sweep doors, the edge's own description, and the verbs' junction checks.
- The reading that decides two planes are one plane no longer multiplies a normal by a decided ±1.

Moved since my first report: I now take the other report's form of that reading (point 1). I now agree that the test-table door goes, provided one call site checks out (point 2).

**1. The sign-free one-plane reading: agreed, with `d₁ − (n₁·n₂)·d₂` and swing `‖n₁ × n₂‖`.** Here `dᵢ = nᵢ·(oᵢ − c)` is each plane's distance from the centre `c` of the region the verdict is used on, and the swing is the angle term. My form, `n₁·(o₂ − o₁) = 0`, makes the same claim but measures it at the wrong place. Three reasons:
- **What each claims.** Put the foot of `c` on each plane, `fᵢ = c − dᵢ·nᵢ`. Then the distance of `f₂` from plane 1 is `n₁·(f₂ − f₁)`, which equals `d₁ − (n₁·n₂)·d₂` up to sign. So the other report's form is my claim, read at the feet of the region's centre rather than at the planes' stored origins.
- **Conditioning in band.** A plane's stored origin can be far from the region (a cap's origin at its frame's origin, for instance). Then my form reads tilt × distance-to-origin, a lever the parallel rung never bounded, and it can say "apart" for planes that meet within the band over the region. The other form differs from today's `d₁ − σ·d₂` only by `(σ − n₁·n₂)·d₂`. That term is second order in a tilt the parallel rung has already bounded at the region's radius. Mine has two claims; one suffices once the planes are parallel. (likely)
- **One form for every reader.** "Replace the decided σ by the product it decided, `n₁·n₂`" applies unchanged to `carrier_eq::reading` (its offset, and the swing `|n₁ − σ·n₂|` → `‖n₁ × n₂‖`). It applies to `certify.rs`'s tangent gap `r₁ − σ·r₂` through the two surface normals at the point. It applies to the line-against-line counterpart (a perpendicular offset compared through `d₁·d₂`). Containment of an origin has no analogue for a gap of radii. (likely for `certify.rs` and the line pair, unsure for `topo::injective::one_plane`, which I did not read)
- **Cost.** The symbolic forms are larger. The other report measured two rungs that were already unproven changing their reason to budget, and no proof lost. I accept that price. I took no measurement of my own.

**2. A direction a test table wrote: agreed that the door goes**, together with the production check that exists only to serve it (the `segment_direction` gate, its resolution gate and its two refusals). Production code kept for a test-only input is the thing `[simple.checks]` and "a mechanism nothing needs" argue against, and my "keep it, length = chord·d" answered the wrong question.
- With the door gone, a loop that gives up its provenance (`into_loop`, `From<ClosedLoop> for ProfileLoop`) gives up its directions too and has them minted. An axis side re-mints the same exact axis.
- The only loss is a verb's exact oblique direction (a leg written with an angle) on a loop that was converted to a table. Nothing I found in production converts a loop and then sweeps it (`into_loop` has one non-test caller, the `From` impl), but I did not trace who calls `From`.
- **The line-directions work does not need this settled.** It can land either way: the pair for a written literal axis is an identity whichever is chosen. This reopens one point of the agreed unit-direction row, and it belongs there.

**3. Points from the other report.**
- *The edge's own description* (`SketchSegment::Line { a, b }`, the edge curve built from its two points): agreed. It is one more re-derivation, and it should read `start + t·director` for `t` in `[0, length]`. It is the likeliest home of the `sym_9` remainder. My report missed it.
- *A run of collinear sides joined into one*: agreed. It takes the first side's director and the sum of the lengths, as an arc run takes its first centre and summed sweep.
- *A verb-authored leg's length*: agreed, it is the authored length. Where the verb builds its end from the pair, the identity is the verb's own definition. The rectangle builds its corners from centre ± half instead; there the authored width and the chord projected on its literal axis are one polynomial, so nothing differs. `emit_straight_leg_at` recovering a length it was handed by `sqrt(norm²)` is a defect of the same class, to go with the work.
- My earlier argument that a single spelling cannot settle (i) stands, and the other report states it more strongly: two quotient walls whose chords differ by a factor that is not constant fail today too. I had not seen that.
- *Not agreed*: the other report's optional E12 sentence. The principle lives in the readers' code and the audit rows, and the doc does not need it.

## For the orchestrator

- I took no measurement this round. Point 1 rests on the identity `n₁·(f₂ − f₁) = −(d₁ − (n₁·n₂)·d₂)` and on the other report's probe.
- **Check before ruling point 2:** grep the callers of `From<ClosedLoop<T>> for ProfileLoop<T>` and `ConstructedLoop::into_loop` in `editor-core`, the bindings and the tour. If a shipped document validates a converted loop and sweeps it with an oblique verb-recorded leg, removing the door loses that leg's exactness. That would then be a measured cost to state, not a reason to keep the door.
- **Still worth a row** (from my first report, not run): a rectangle-verb member beside a table polygon sharing a wall, at `Sym<Interval>` over a box. It should leave the seat unproven on main today, and it is direct evidence for the reader change independent of the door work.
- `a_boss_typed_tangent_to_a_cylinder_glues_one_unproven_tangency` (red in the other report's probe both ways, `curve_pair_pose_parallel` at the band's edge): not mine to trace. Flag it for whoever runs the merged probe, because it looks independent of the reader change.
- `topo::injective::one_plane` is unread by both lanes. Check it for the σ·n shape when building.
