# Review of PR 6805 (`spiricarm/cylinder-pair-cost`): single full review

**Head reviewed:** `c2b6282453873e528c6bbe1170793874b59477f9` (frozen). Base for comparisons: merge-base `f0f7a117`.
**Probes:** `spiricarm-rev6805/probes` @ `be71395e` (`swap_rows/framed.rs` and `sphere.rs` `rev6805_*`, and the mutant runner `rev6805-mutants.py`).

## Verdict: NOT-MERGEABLE-AS-IS (1 MAJOR / 3 MINOR / 4 NOTE)

No probe found an unsound enclosure: every reading I took contained its independent truth, at every round, at one piece a span and coarser. The MAJOR is a capability regression. The spread schedule makes the lane **refuse, typed, faces that main measures**, and the PR body says the opposite ("may need a later round … costs work, not soundness"). The fix is probably small, but it has to be made and pinned before merging.

## Findings

**MAJOR-1: one piece over a long or near-pole span refuses at round 0, and the refusal is terminal** (`rotational.rs:span_pieces` with `cylinder_edge_terms` and `sphere_edge_area`; confidence: **sure** on the mechanism and on both fixtures, **unsure** how often production chords hit it).
- `span_pieces` reads one piece a span once a carrier has 16 spans or cut intervals or more, however long a span is or how far it turns. Over a piece that sweeps near the chart's axis, `u′ = (x·y′ − y·x′)/(x² + y²)` reads a refused reciprocal. The round's flux is then non-finite, and `mean_boundary_displacement` (`run.rs`) returns `QuadratureUnsupported` from inside `accept`. That ends the call: it does not refine. Production's `quad_lane::cut_face_rounds` accept takes the same path.
- **Executed, a cylinder case** (`rev6805_a_circle_and_line_of_many_cuts_hold_at_round_zero`): the rectangle `u ∈ [−2.8, 2.8]` whose bottom rim is a `FramedQ::Circle` cut into 17 intervals: 16 short ones, then one of 5.4 rad.
  - Main: round 0 holds at flux width 6.3e-3, round 2 at 4.9e-7, and the face measures to target.
  - Head: rounds 0 and 1 refuse non-finite and round 2 refuses `DegenerateFace`. To target it refuses `QuadratureUnsupported`.
  - Mutant M1 (`span_pieces` returns `pieces`, main's per-span count) turns this probe green, so `span_pieces` is the cause.
- **Executed, a sphere case** (`rev6805_a_16_span_disc_by_the_pole_measures_as_on_main`, red at head): the exact tilted disc circle missing the north pole by 1e-3, its net forced to 16 spans (as a joined chord would be).
  - Main holds both the disc and its complement to target, under both names.
  - Head refuses all four at round 0 (non-finite).
  - In the broader sweep over misses 0.3 down to −1e-3 at 16 and 64 spans, this is the only pair that changed between main and head (logs: 280 refusals and 56 holds at head, 0 escapes on either side).
- **Reach:** the two P1 poses' chords are 625 spans of 1/625 each, so short spans keep them clear. A cylinder × sphere chord, or a sectioned chord whose spans are long relative to its distance from the axis or pole, is not clear. The four `cylinder_sphere_frame` and `verbs_cylsph_opening` rows I ran pass (claim 6). No row covers a many-span chord near a pole.

**MINOR-1: the cost and chart-term halves of the spread are unwitnessed** (`face_cost`, `chart_term_at`; confidence: **sure**, executed). Mutant battery over the 64 `rotational` lib tests, main's rows plus mine:

| mutant | PR's rows red | my probes red |
|---|---|---|
| M1 `span_pieces` = old per-span `pieces` | `a_rounds_pieces_spread…` | (turns the circle probe green) |
| M2 under-read, `(pieces/16).max(1)` | table row; `a_disc_beside_the_pole…` | circle |
| M3 `face_cost` projected arm at the old per-span count | **none** | none |
| M4 `face_cost` projected arm without pieces | **none** | none |
| M5 the last span dropped on nets of 16 or more spans | `a_section_net_of_many_spans…`, `a_circle_grazing_the_pole…` | sections, circle |
| M6 the chart term biased ×1.001 on rationals of 16 or more spans | **none** | `rev6805_many_span_sections…` |
| M7 `framed_spans(Net)` = 1 (edges read the old count) | **none** | (circle only, as at head) |
| M8 cylinder edges read `(pieces/16).max(1)` | none | (circle only) |

- `a_harmonic_faces_work_is_its_edges_pieces` exercises only the harmonic arm. It is the identity of `round_cost`'s formula (`round_cost` is `#[cfg(test)]` and the same expression), so the projected arm the PR restructured has no row.
- `a_section_net_of_many_spans_holds_the_closed_form` runs about `chart.o`. The lever is zero, so the chart term's rationals are multiplied by 0 and `chart_term_at`'s spread is never read. Off the chart (my probe, `about = (1.1, 0.5, −0.7)`, 16 to 64 spans, rounds 0–3) it holds, and catches M6.
- M3, M4, M7 and M8 are sound by construction (they change cost or resolution only). They are listed because claim 2's "face_cost counts the same pieces" has no guard.

**MINOR-2: the reading now depends on how a boundary is cut into edges** (`span_pieces`; confidence: **likely**). A 20-span chord as one edge reads 1 piece a span at round 0, and the same chord as two 10-span edges reads 2. Main read 16 a span either way.
- DESIGN `[tolerance.measure]` says what a measurement spends "depends on what it asks and on the geometry, never on how its surface is cut into faces". That clause is the ruling of #6014 and #6102 (DR-400 moved the rectangle lanes to per-span schedules for exactly this reason).
- The PR does not mention the clause. Whether edge cuts fall under it is Ev's call, not the lane's.

**MINOR-3: the "Unproven" paragraph is false where it is concrete** (PR body; confidence: **sure**). "A sphere chord passing near a pole … may need a later round … That costs work, not soundness." It costs a refusal (MAJOR-1). The rest of the body ("changes resolution, not soundness", "no bound changed") is true of the enclosures and silent about refusals.

**NOTE-1: claim 3 (12 digits, three digits) is a one-off measurement with no guard** (confidence: **sure** that nothing re-takes it; I did not re-measure it). It reads identical *volumes*: the midpoints of enclosures. That is no containment evidence: the sliced-oracle rows and my exact rows are. Only the PR body carries it, and nothing computes with it, so no guard is owed. It should not be cited as evidence of correctness.

**NOTE-2: the SPIRICARM entry row's "waits on" premise has gone stale** (`work/spiricarm/the-rotational-lanes-entry-reads-a-many-span-chord-at-four-pieces-a-span.md`; **sure**). It says it "waits on" PR 6509, which merged before this head, as the PR body itself says. The row should say what is left after `entry_turn`, or name another trigger. It also leaves out that `entry_turn` reads up to `PROJECTED_TURN_PIECES` = 1024 pieces a span adaptively, so a near-pole 625-span chord can cost far more than the "four pieces a span" in the title.

**NOTE-3: homing and bands** (claim 7; **likely**).
- PCERT row: `pcurve_cache.rs` is PCERT's sole path. It is P1, as its parent is, and accurate to the code I read (`INCIDENCE_CUTS`, `twin_span_hulls`, `limb2_offset_sup`).
- ONEHOME row: `ring_path.rs` and `chord_join.rs` are in ONEHOME's paths. ONEHOME's mission is "one implementation per logic", and a speed row sits oddly there, though territory puts it there. P2 is defensible.
- The two SPIRICARM rows are P3. The spiric row is unmeasured cost with no fixture, which reads closer to P4 ("code improvement that is not architectural"). `work.py lint` is clean.

**NOTE-4: the kept probe example is fine** (claim 8; **sure**). `crates/sweep/examples/cylinder_pair_cost_probe.rs` is test-support-only through dev-dependencies (`geom_core::test_support`, `topo::test_support`) and adds nothing to the library. The gate's `clippy --all-targets --all-features` compiles it, and the PR's run is green. It sits beside eleven other probe examples. I did not run it.

## Claims

1. **Upheld for soundness, falsified for "only how many pieces".** Containment held at one piece a span, and coarser:
   - against the 40-digit mpmath truths at 1, 2 and 3 pieces a span on the three hard chords (a cubic past a quarter turn, across the seam, a 1.3-turn spiral);
   - on 16- to 64-span plane sections at rounds 0–3 about an off-chart point, including across the seam and a steep near-full turn;
   - on a 40-cut even circle;
   - on 336 sphere readings near and through the pole.

   But refusals change: MAJOR-1.
2. **Upheld** on consistency. I grepped for the shape (`rational_integrals(`, `rational_vector_area_split(`, `composite_over(`, `rate_bound_over(`, `<< round`, `EDGE_INIT_PIECES`, `PER_SPAN`, `_CUTS`). The other hits are the PR's own disposition list, unchanged: `edge_entry`, `spiric.rs:651`, and the test sites. **Unguarded:** MINOR-1.
3. **Not re-measured.** It is a measurement, unguarded, and not evidence: NOTE-1.
4. **Partly falsified.** The schedule row catches M1 and M2, and the section row catches M5. The cost row and the chart term catch nothing: M3, M4, M6 and M7 survive.
5. **Upheld.** The diff re-pins and loosens no row. `a_swapped_plane_section_holds_the_closed_form` moved into `sections_hold` byte-equivalent.
6. **Run at head, release, all green.** `verbs_cylsph_opening` (all 7 rows, among them the transversal pose at 37 s and the lamina row), and `cylinder_sphere_frame` poses `centre_on_the_wall` (41 s), `in_two_loops` (80 s) and `r_0_1_across_the_rim` (21 s).
7. **Upheld with notes:** NOTE-2 and NOTE-3.
8. **Upheld:** NOTE-4.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q6, Q7. Q5 and Q8 only for `rotational.rs`'s header and the lane loop; I did not read the whole 2820-line file.

- `rotational.rs:span_pieces`, `framed_spans`: the per-span count is a fixed division of the round's total, blind to the span's own extent. The rectangle lanes (DR-400) and `entry_turn` (adaptive) both read geometry where this reads a count. It is a third schedule shape in one lane: per-span fixed (`PROJECTED_ENTRY_PIECES`), adaptive (`entry_turn`), spread (`span_pieces`). **likely**
- `rotational.rs:face_cost`: `pieces` is recomputed from `round` here, in `rotational_cut_face_rounds` (`EDGE_INIT_PIECES << round`, then `*= 2`) and in `round_cost`. Three spellings of one schedule must agree for the work wall to price what runs, and nothing ties them. **likely**
- `rotational.rs:span_pieces` doc: "never fewer than `pieces / EDGE_INIT_PIECES` per span, so every span halves its pieces' width each round". That is true, but it is the only invariant stated, and the one that matters for refusals ("a span is never read in a piece wider than X") is absent. **unsure**
- `rotational.rs:a_harmonic_faces_work_is_its_edges_pieces`: compares `face_cost` with `round_cost`, which is the same formula restated under `#[cfg(test)]`. Q3: it can go red only if the harmonic arm drops its factor. **sure**
- `rotational.rs:a_rounds_pieces_spread_over_a_carriers_spans`: the table's `(0, 16)` row pins a value for zero spans that no caller can reach meaningfully (`spread` multiplies it by 0). The `>= EDGE_INIT_PIECES` loop is implied by the `div_ceil` table. **likely**
- `swap_rows/framed.rs:sections_hold`: `#[allow(clippy::type_complexity)]` on a 5-tuple row type. A named struct would read better. Taste. **unsure**
- `framed.rs:a_section_net_of_many_spans_holds_the_closed_form` doc: "holds the closed form at one piece a span, which is what it reads at round 0". The row runs to target 1e-9, not at round 0, and about `chart.o`, so the chart-term half is unread (MINOR-1). **sure**
- The PR body's §5 sweep table is good practice. Its blind spot is stated, "a rule that loops over spans with its count written inline", but the sweep looked only for cost, not for refusals: the class "a coarser piece can refuse where a finer one read" was not swept, and MAJOR-1 lives there. The same class applies to the filed PCERT suggestion ("whether the incidence read can do the same"). Look there before spreading `INCIDENCE_CUTS`. **likely**

## Disclosures

- **Builds and runs:**
  - geom-brep lib tests in debug, at the head (`/home/user/rev6805-target`) and at the merge-base `f0f7a117` with the same probes applied (`/home/user/rev6805-base-target`).
  - The mutants in a third worktree at the head (`/home/user/rev6805-mut-target`).
  - The sweep rows in release at the head.
  - All on one shared 4-core box. Timings are indicative only.
- **Harness backgrounding:** three foreground `cargo` or wait calls passed the tool's own 120 s or 600 s limit and were backgrounded by the harness. The base probe run and the sweep build were launched `setsid`-detached and polled in the foreground. One wait loop the harness backgrounded finished on its own before I stopped it. No background work remains.
- **What I added to the fixtures:** the probes change two of them. `Arc::net` gains a thread-local minimum segment count, and `sphere_face` maps a failed envelope certification of the fixture to a refusal ("rev6805: …") rather than `unwrap`. Some 64-span and negative-miss pole fixtures fail to certify their own image on both main and head; those are fixture limits, not lane findings.
- **Not done:** I did not re-take the PR's timing tables or claim 3's volumes, and I did not run the P1 rows themselves.
- I posted nothing to GitHub.
