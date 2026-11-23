# Delta review: PR 7268, `229a3550..75a008e8f`

This is the reviewer lane's delta read of the fix pass on PR 7268 (ONEROUND). The first review is `review-report.md`, beside this file.

## Verdict: APPROVE-WITH-FIXES

The fix pass does what it says for F1, F3, F4 and F5. Each of my earlier MINORs is answered. The joint room and the spiric are now posed truthfully, and the differential is clean.

The fixes still owed:

- **F2 rests on a false "both certify".** Neither lane certifies the row's circles. Both refuse in band, at every ε.
- **The new spiric row only discriminates at the coarser ε.** It cannot see the span-extent mutant at ε = 1e-12.
- **A few sentences are stale or argued from the wrong mechanism.**

## What I executed, and what I only read

**My base, and why.** I did not diff against the earlier head `229a3550`. The delta merges main twice, and PR 7227 has landed, so an old-head comparison would mix main's moves into the fix pass.

Instead the old reads sit on the new head itself: `75a008e8f` with exactly F1 and F4 put back to their `229a3550` forms.

- F1: `Margin::levered(cos, edge_extent_inf(span))` in the run band.
- F4: `Margin::levered(q, parallel_curvature_arm)` in `band.padded(ε)`.

So every count below is judged against `75a008e8f`, differing only at those two sites. Everything else is identical.

Both trees carry the same `k_stats::classify` site log (`tools/k_stats-site-log.patch`). The fuzz seed is pinned with `CAD_FUZZ_SEED=0x7268`.

**EXECUTED (debug, `ci` profile):**

- geom-brep + topo, both trees, at ε = 1e-9 and 1e-12 (6517 tests each).
- The mutant set below, each over the whole geom-brep + topo set at 1e-9.
- The F2 row instrumented at ε = 1e-9, 1e-6 and 1e-12, logging the five Villarceau gates and the torus incidence read.
- Probe `delta-probe-f2-certify.patch`: it certifies both lanes' images from the F2 row, at three ε.
- Probe `delta-probe-needle-thinness.patch`: needle planes 1e-6 to 1e-14 short of the generator, at ε = 1e-9 and 1e-6.
- The gates on the frozen head with `GATE_BASE=origin/main`:
  - `lever-sources`: OK, 46 sites, 870 fixtures.
  - `lever-doors`: OK, 58 pinned, none above main.
  - `from-f64-allowlist`: OK.
  - `work.py lint`: ok.
- A check for conflict markers in the tree (`git grep` for `<<<<<<<`, `>>>>>>>` and `=======` in `crates scripts work docs`): none.
- `git show --remerge-diff` of the three merges.

**NOT executed:**

- The other five crates for the fix pass. Only the author's run covers them.
- The release slow set.
- The interval job's suites.

**Hosted run 45088520748 on `75a008e8f`** was still `queued` at my last read (run `updated_at` 2026-11-23T01:23Z; I read it again after the mutants finished), and no job had started. So no hosted verification exists yet, and my runs are the verification.

## The differential (fix pass, my base as above)

**geom-brep + topo at 1e-9.**

- Head: 6517 pass. Old reads: one red, the new `a_spiric_on_its_own_torus_is_served_over_any_span`, by design.
- One verdict moves: that row's `pcurve_spiric_chart_axis` reads. Old: Negative 4, in band 1. New: Negative 8, Positive 40.
- Bits move only at the spiric axis (17 test × site pairs).
- **The joint room is bit-identical to the `229a3550` read at every reaching test.** That is 15 test × site pairs.

**geom-brep + topo at 1e-12.**

- One red on both trees: `envelope_lemma_fuzz::projected::a_line_near_a_ruling_pinned_where_the_frame_cost_refuses`, held by `work/onehome/the-carrier-frame-fuzz-bodies-go-red-under-no-mutant.md`.
- No verdict moves. Bits move at the spiric axis only (18 pairs).
- **The new spiric row is green on the old read at 1e-12.** See D-MINOR-2.

**Against the author's account.** The `room_fence` sphere row is not a moving verdict against my base. It moved against the author's base, which carries the original `d − 2ε` read. Against the frozen head's read it was already green, so F4 changes the door and no bits.

## Findings

### D-MINOR-1: F2 rests on a false "both certify"; neither lane's image certifies at the row's circles

DEMONSTRATED, `sure`.

The row is `crates/geom-brep/tests/pcurve_villarceau.rs`, `a_tilt_just_past_the_band_takes_the_general_lane`. The comment at `pcurve_cache.rs:13088-13099` and the fix pass both say no arm mutant went red because both lanes certify. On the row's own circles they do not.

The probe certifies each lane's image with `PcurveCache::certify`, over `(0.3, 4)` and `(0, τ)`:

| Move | Lane | Certification |
|---|---|---|
| `0.99ε` | the closed form | refuses: `MapResidual at sample 0/1 escalated` (margin about 1.02 to 1.18 ε, in band) |
| `1.0015ε` | the general lane | refuses: `EnvelopeTerm(Incidence) at sample 0 escalated` (margin about 2.64 ε, in band) |

The result is the same at ε = 1e-9, 1e-6 and 1e-12.

So the gate is live in the sense the row pins: it chooses the lane. But at the sliver the row builds, its only downstream effect is which in-band refusal names the cause. Both refusals offer the same recourse.

On the LP claim:

- The instrument confirms the row's numbers at all three ε. The incidence read is 0.998ε (0.979ε at 1e-12). The tilt is 1.0015ε, in band. The other gates are about ε/2.
- I did not re-solve the LP. The row writes its move down, and the move does what it says.
- The sliver is about 0.2% wide at 1e-9 and about 2% at 1e-12. That suits a pinned move but leaves no margin.

**"Both certify" was the reason given for the survivors, so the reason is false.** What the survivors show is that, in the suites, the tilt gate never decides a certified outcome.

### D-MINOR-2: the new spiric row cannot see the span-extent mutant at ε = 1e-12; its spans are fixed numbers, not band-relative

DEMONSTRATED, `sure`.

The row is `closed_image_rows::a_spiric_on_its_own_torus_is_served_over_any_span`. With the old read (the arc's extent from below):

- at 1e-9 the row is red, which my mutant `sp_extent` confirms;
- at 1e-12 it is green over all its spans (1e-3 down to 1e-10), because every one of those arcs is past Kε at that band.

The row's doc says "down to a tenth of the band's zero", which is true only at ε = 1e-9. The class owed here is a row that states its spans against the band, so the arm that can be short is killable at every ε the suite runs.

### D-NOTE-1: F1's "the chart holds the oval" is true of every mint path, but its "±1 by construction" leans on incidence, which does not fence a small oval

BY-INSPECTION, `unsure`.

Every path holds:

- `chart_image` (`pcurve_cache.rs:10561`) gives a spiric `Placement::over`, with `edge_only` false and `shown` None.
- `incident_as` then reads `(0, τ)` on a `RevolvedTube::Ring` (`:13697-13700`).
- `edge_only()` is set only by `read_edge_incidence` (`:11084`), which returns `()` and never mints.
- `shown()` is set only on check 4's harmonic re-derivation (`:6696`), which a spiric never reaches.

So the claim about paths is true.

But incidence admits any torus the oval lies within ε of. Consider an oval of minor radius just past `Kε`: the spiric constructor requires `spiric_minor_positive` decided past the band. Such an oval can lie within ε of a large foreign torus whose axis is not parallel. There the cosine is not ±1, and `sign_of` still picks a side.

The image is then refused later: check 1 gates the centre against the tag's torus (`pcurve_spiric_mint_center`). So the outcome is a refusal, not a wrong image. This is the argument for the survivors (`sp_box`, `sp_oldR`), and it holds outside that corner.

### D-NOTE-2: F6's thin-β argument reaches the right conclusion by the wrong mechanism

DEMONSTRATED, `likely`.

The argument says a section with an in-band β "would need its plane within about 1e-17 of a generator's tilt, under an ulp". The probe shows that is not the fence.

`intersect::plane_cone_section` classifies every plane within the band of the generator's tilt as a parabola (`OpenConic`):

- at ε = 1e-9: from 1e-10 short of the generator down to 1e-14;
- at ε = 1e-6: from 1e-8 short down to 1e-14, with 1e-6 short escalating.

So no ellipse is ever minted that thin. At ε = 1e-6, where a plane 1e-10 short of the generator is perfectly representable, that plane still never yields an ellipse.

The ellipses that are minted keep `(1 − |β|)·arm` far past the band. At 1e-9, with the plane 1e-8 short, room is 2.26e-4, the azimuth arm 1.95e4 m/rad, and the old margin 4.4 against an escalation of 1e-8.

The conclusion holds: no genuine row can tell the old gate from the new. The sentence at the row (`closed_image_rows.rs`, the needle doc) should name the classifier's band, not an ulp. The probe's arms (195 to 1.95e6 m/rad) grow as the plane nears the generator, as a needle's major axis does. That is consistent with "arm ≈ 2A", which I did not check further.

### D-NOTE-3: stale sentence in the unit row

BY-INSPECTION, `sure`.

`work/oneround/unsourced-levers-the-stand-ins-and-the-carrier-terms.md`, *Read against the tree*, last bullet, still says: "None has a door that names it; that is a design question on PR 7268". The question was decided on PR 7410, and the ten's row says so.

### D-NOTE-4: the merges

DEMONSTRATED by the gates, `sure`.

There are no conflict markers in the tree. `a4248b8fb` is an ancestor of the head, and after it `git grep` finds none.

`391e9ba34`'s hand-resolved hunks:

- take main's wording at `intersect.rs`'s five apex pins (2532, 2537, 2753, 2756, 2780);
- drop this branch's "No door names it yet" lines there, and keep two citations of the ten's row in main's text;
- set the counts.

The counts match the tree:

- `lever-sources.counts`: `curve_face/pose.rs` 1 (7227's `line_cone_read` conversion arrived).
- `lever-fixtures.counts`: geom-brep 498 and topo 311, against main's 497 and 307. The difference is the needle row's `Reach::fixture` and the bisector row's fixtures.
- `from-f64-sites.tsv`: the `2.0 * band.zero()` entry is gone, against main.

`f2c9cc5f4` and `75a008e8f` show no hand-resolved hunks under `crates/` or `scripts/`.

### D-NOTE-5: F4, F5 and F6 bisector, checked

F4: `decision.rs:1873-1885` is now `Margin::of(d·q)` with `d` from `axis_distance` alone. `parallel_curvature_arm` is gone from the site. That removes my earlier style finding about two spellings of the axis distance. Mutants are in the table below.

F5, each by inspection, `sure`:

- the `sign` pin comment names only the half-turn count;
- `sign_of`'s doc names its four uses;
- the spiric doc no longer names R.

F6, the bisector row:

- 5e-8 reads 5e-10, half the zero;
- 3e-7 reads 3e-9, in band, and the `Coincidence` escalation is now pinned;
- 1e-5 reads an inside;
- none sits on a band edge.

## Mutants (re-planted on the frozen head, the whole geom-brep + topo set at 1e-9, seed 0x7268)

| Site | Mutant | Result |
|---|---|---|
| spiric | span-extent arm (the `229a3550` read) | RED: `a_spiric_on_its_own_torus_is_served_over_any_span` (1e-9 only; see D-MINOR-2) |
| spiric | session box | survives 6517 (equivalent once incidence admits the oval: D-NOTE-1) |
| spiric | old R | survives 6517 (equivalent, as above) |
| spiric (mine) | arm 1e-9 | RED: 16 tests |
| spiric (mine) | sign flipped | RED: 14 tests |
| joint room | old `(d − 2ε)·q`, unpadded | RED: `room_fence` sphere row |
| joint room | pad dropped | RED: both `room_fence` rows |
| joint room | sphere radius for `d` | RED: `room_fence` sphere row and `sphere_twin_rows_interval::near_a_pole_the_windings_refuse_before_the_winding` |
| tilt | `r` | RED: the new F2 row |
| tilt | `R·1e-6` | RED: the new F2 row |
| tilt | `R·1e6` | RED: the F2 row and `a_tube_rate_gates_rejected_zero_is_a_class_reading` |
| tilt (mine) | `2R` | RED: as `R·1e6` |
| tilt (mine) | `R/2` | RED: the F2 row |
| bisector | unit arm | RED: `bisector_rows` |
| bisector | far reach | RED: `bisector_rows` |
| bisector | session box | RED: `bisector_rows` |
| bisector | `BoolSector::arm` | RED: `bisector_rows` |

The implementer's claims hold: the span-extent arm is red, and the session-box arm and old R survive. Their argument is sound outside D-NOTE-1's corner. Every tilt arm mutant, including a factor of 2 either way, is now red on the F2 row. So the gate's arm is pinned, but only through the lane the row asserts (D-MINOR-1).

## Fresh instances of what the fix closes?

- **The spiric row's fixed spans** are the class the review named: a mutant called unobservable that a row kills only at one ε (D-MINOR-2).
- **F2's "both certify"** is a measured reason stated without being checked at the row's own circles (D-MINOR-1).
- **The needle row's tilt, `1e3·band().zero()`,** is a length chosen by rule. It is harmless here because the classifier decides the fence (D-NOTE-2).
- **No new arm chosen without asking what the tilt governs.** F1 and F4 now carry no arm at all.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q6; Q8 not exercised for the delta)

- **Q3.** The F2 row's sliver is 0.2% wide at 1e-9 and rests on nine-digit LP coefficients (`pcurve_villarceau.rs`, `moved`). It is deliberate, but any change to the incidence sampling turns it red with no semantic change. `likely`.
- **Q6.** The tilt comment (`pcurve_cache.rs:13088-13099`) cites the row as where the masking is measured, but the row pins the lane, not certification (D-MINOR-1). `sure`.
- **Q2.** The needle doc's ulp argument postdates the probe that would have refuted its mechanism (D-NOTE-2). `likely`.
- **Q4.** The unit row's last bullet (D-NOTE-3). `sure`.

## Ratings

- **Idiom/structure: 4.** F1 and F4 now say what the numbers are, with no arm. One spelling of the axis distance remains.
- **Test quality: 3.** The bisector row is fixed. The spiric row discriminates only at ε ≥ 1e-9. The F2 row pins a lane whose consequence at its circles is an in-band refusal either way.
- **Doc/comment honesty: 3.** The corrections are true. But "both certify" is false, the needle argument names the wrong fence, and one row sentence is stale.

## Artifacts

All under `analysis/oneround/unsourced-stand-ins/`:

- `delta-probe-f2-certify.patch`
- `delta-probe-needle-thinness.patch`
- `tools/mutants2.py` and `tools/mutrun2.sh`
- `tools/mut2-full.txt` (the mutant results)
