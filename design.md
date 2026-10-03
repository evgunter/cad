# Arc-side rule: how many, which, and who reads what

## For Ev

**Recommendation (likely): no arc-side rule in `chord_spec` at all.** Each lane already decides which arc a chord takes when it pairs the chord's two ends, and the chord should carry that decision forward. Today the code throws that decision away and works the arc out again with one of two other predicates. Instead, the section's **direction at each crossing** (the unit tangent of the section curve at that crossing, pointing along the segment into the face) should be one datum, minted with the crossing. The pairing and `chord_spec` both read it, and `chord_spec` only orients the chord: of the two arcs from p1 to p2 it takes the one whose tangent at p1 agrees with that direction. The S9 window rule and the run-side rule both retire as selectors, and the planar side reads the same datum the wall side does.

**Premise correction.** The brief asks "one rule or two, and which". Read against the tree, it is neither question:

- *Split lane.* `conic_pairs` walks the conic along `h = n_plane × n_out` and enters the face exactly at the down crossings. So for every chord it pairs, it already knows the arc: from the entry, forward along `h`.
- *Boolean, both sides.* Every null-edge half faces a `HalfGerm`. `HalfGerm.dir` is the section's outgoing direction at that site, chosen by `germ_dir` from sector membership in both faces. On a conic locus, the matcher's `germs_face_each_other` reads the rotational sense `axis·((p−c)×dir)` at each site and requires the two to oppose. That sense is "ccw or cw from p1": it is the arc choice itself, made at match time.
- `chord_spec` then derives the arc a second time, in one of two ways: from the divided face's azimuth window, or from the side of the run. On the planar side it derives it a third way, from the mate face's window. Neither derivation can see what the pairing saw.

So the problem is one fact computed in up to four places, and the frontier refusals are the places where a re-derivation is blind:

| Refusal | Where | Why the re-derivation is blind |
|---|---|---|
| `SectionNotPolar` | planar side | it has only a window, and a tilted section has none |
| `NoCertifiedRun` | pierce ring (a cross-loop `mekr` chord) | it co-bounds no run, so the run-side rule has nothing to read |
| `ReflexRunEnd` | run end at a reflex corner (P3 item) | the side of one run edge is not the face's sector |
| up-front `face_azimuth_window` failure | mirrored box | the wall face carries a tilted arc |

The pairing's datum reaches every one of these cases. This confirms the issue's "three readings agree wherever each reads", but draws the opposite conclusion: hand the first reading on, rather than picking a best way to compute it again (sure on the duplication, likely that it clears all four refusals).

**Why the direction is sound where the run-side rule is not (sure).** At a crossing where the face's boundary runs down through the plane, the boundary arrives from the above half-plane and leaves into the below one. Turning counter-clockwise (under `n_out`) from the below direction to the above direction always passes through `h` and never through `−h`, whatever the corner's angle. So `h` lies in the face's sector at every clean crossing, reflex corners included. The run-side rule reads one boundary edge's left instead, which is why it has to refuse at reflex ends. `germ_dir` has the same property by construction, since it selects by membership in the sector.

### The options as final states

**A. One datum, read by every consumer (recommended).** What becomes true:

- The split lane's null-edge records carry the same `dir` that the boolean's germs carry, minted from the crossing's classified sense.
- `conic_pairs` walks by that datum. `chord_spec` takes it in place of a window or run.
- The `JoinLane::BoolPlanar` arm drops `window`.
- What goes: `ChordRun` and the `CoBounded`/`FaceWindow` split, `select_arc`, `select_arc_by_run_side`, `ArcWindowCase`, `ArcSideCase`, `SectionNotPolar`, `SectionConic::azimuth_monotone`, and the arc-side use of `run_azimuth_window` and of the cone apex closure.
- `face_azimuth_window` keeps its other consumers (`solid_contain`, `pcurves`).

What it makes unrepresentable: the two bodies of a boolean choosing different arcs for one spatial section edge. Today each side selects independently and they agree only by hand. Under A both read one germ.

The one decision left is a sign: the direction against `C′(θ₁)`. Its margin is about `|C′|`, so it is far from the band unless the datum is corrupt (zero means a malformed datum, refused loudly).

What it leaves possible: a wrong pairing now ships a wrong arc consistently, with no window left to refuse it. The answer is that the pairing must be right, which is where the correctness already lives. See the boolean matcher note under the orchestrator section; it may need `conic_pairs`' walk order.

It is reversible: the predicates sit in history and A deletes them cleanly.

**B. Fold the window rule into the run-side rule (the issue's proposal).** One predicate, chart-free. But:

- It still re-derives what the pairing knew.
- It still cannot serve a cross-loop chord, so the pierce ring keeps refusing.
- It still needs a sector reading at reflex ends (the P3 item).
- On the planar side it would have to read the *other body's* run. The tilted-sphere lane tried a version of this and hit three new refusals.

Better than today, worse than A on every axis above.

**C. Keep both, each where its premise holds.** The window rule buys nothing A lacks. Its one virtue is that it asks about the whole face (the fix for #144's single-point sample). A's direction is not a sample on the run, though. It is the segment's own tangent at its own end, plus the pairing's "next crossing along it", and that is exactly the #144 lesson. C keeps two predicates for one fact plus a by-value window channel. Not recommended.

**Not recommended either: A with the window kept as a cross-check where it applies.** It can only refuse what a correct pairing already guarantees, and it re-imports the azimuth premise. If an independent check that the arc lies in the face is wanted, it belongs in certification, where it would cover every lane, not in selection.

### Ratified text

- The S9 window rule and the run-side rule are agent-written code docs, not DESIGN.md text.
- One DESIGN.md line would change. Gap (d), cyl×sphere germ chords, says what is missing is "the join window itself" (written in #3797, agent text). Under A no window is needed to pick an arc. What is missing there is a C5 section arm plus a frame for the rotational-sense test. The line re-words with the change.

### Worked example

Take the pierce ring on a sphere pair tilted against both charts (the `a_tilted_section_stops_at_the_pierce_ring…` pose). B's boundary pierces A's sphere face at two interior points, so the chord joins a ring to A's outer loop with `mekr`. That chord co-bounds no run, so the run-side rule refuses `NoCertifiedRun`, and a window does not exist because the section is tilted.

Each pierce's germ already has its `dir`: the radical circle's tangent, pointing into B's face, chosen by sector membership. The matcher has already required the two rotational senses to oppose. Under A, the chord is the arc of the radical circle that leaves p1 along `dir`. That is one sign, read once, and the same one on both bodies.

Confidence:

- The recommendation: **likely**.
- The duplication (the pairing knows the arc): **sure**.
- The sector argument: **sure**.
- That the four refusals clear with nothing behind them: **unsure**. Ring closures read the chart too (`RingClosure`, "winds its island on the face's chart"), so a later azimuth-premised step may refuse next. This is unmeasured.

## For the orchestrator

- **Brief error:** `bool_between_arc_window` does not exist in the tree (searched all of `crates/`). The boolean's adjacency skip reads the locus (`SegmentEdge::Is`), not azimuth. The issue's "planar side's adjacency skip is azimuth-premised" seems to describe the lane's reverted attempt, not main. The `split_arc_window` rung does exist.
- **Suspected defect, off-question, unverified; file it under REACH or the boolean owner.** `find_match` and `loose_partners` pick the nearest partner by *chord length*, `(p_e − p_c).norm()` under `bool_join_nearest`, even on a conic locus, where only the facing test is arc-aware.
  - On one face with four crossings on a circle, with alternating senses at 0°, 100°, 200° and 300° ccw, the site at 0° sees two opposite-sense candidates: the one at 100° (chord `1.53r`) and the one at 300° (chord `1.0r`). The 300° site is nearer by chord, but the arc to it passes through the other two crossings, outside the face.
  - This is the defect CLEAVE fixed on the split lane with `conic_pairs`.
  - It matters more under A, because the pairing becomes the only source of the arc. The fix is to order by the walk coordinate (`conic_pairs`' arc-length gap).
  - I did not build a pose that reaches it.
- **Not measured.** I did not prototype A. The cheap way to land it safely is to have `chord_spec` compute both the old selection and the datum's arc and refuse on any disagreement across the full battery, then delete the old selectors. That converts my "likely" into evidence. Candidate rows: the three planar-side poses in `planar-side-of-a-tilted-plane-sphere-cut-has-no-arc-cue`, the two pierce-ring poses in `tilted_sphere_pair.rs`, and a reflex-notch pierce for the P3 item.
- **Split-lane plumbing assumption.** I assumed the sweep's `is_down` and the plane's Above normal are enough to mint a `dir` at each split null edge (entry gives `h`, exit gives `−h`). Faces with two or fewer crossings, which keep the book's rule today, get the datum the same way. Their pairing is forced.
- **Items this would close or reshape:**
  - `arc-side-rule-has-two-predicates`: answered by A.
  - `planar-side-…-no-arc-cue`: answered by A.
  - `run-side-arc-rule-reads-only-the-run-at-each-end`: retired, since the rule goes.
  - DESIGN.md gap (d): re-worded.

