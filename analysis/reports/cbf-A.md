## For Ev

**Recommendation (likely): one stored form. Every circle and ellipse carrier is stored anchored, and the centre is never a field.**

- A carrier stores a point of the curve inside the session box (the *anchor*, where the parameter is zero), the frame's directions, the sizes and, for an ellipse, where on the ellipse the anchor sits (its *phase*).
- The centre is one derived reading, good to about u·a (u is f64's unit roundoff, a the larger semi-axis). Export writes it, as you accepted on PR 4812.
- A door that is handed a centre computes the anchor once. From then on the anchor is the curve.
- "Born with a centre" stops being a stored distinction, and the "anchored cache" disappears: with one form there is nothing to cache.

**Premise check: three corrections**

1. **Nothing gives a carrier, so there are no given bits to keep.** (sure on the texts and the tree; likely that this settles it)
   - `[edges]`: the 3-D carrier is "a derived cache", "never a peer of the description".
   - `[import]`, rewritten on PR 7074 (2026-11-17, five weeks after PR 4812): "no value the file printed becomes a cache". Your words there: "nothing imported should be able to reach a state that can't be reached by native things", and "no special caches".
   - The tree does this. `step-import/src/build.rs` builds each edge's carrier by the kernel's own section construction, and the printed curve only picks the component.
   - So PR 4812's "an imported ellipse keeps the file's centre form exactly as read, with the anchored form as an internal cache" describes a state `[import]` now forbids and the tree does not have. An imported conic is a kernel-minted section, which you already decided is anchored.
2. **The clause in question is not your sentence.** "Evaluators read the anchored form either way" was written by the agent on PR 4812, and you answered "sounds good!". Your own words were "import should re-pose in a sense, but ideally it would round trip nicely". The round trip you asked about was the file's, and PR 7074 has since settled it differently.
3. **The "only 5×" is a property of keeping the centre as the truth, not of these conics.** (likely; reasoned from the two evaluation chains, not run)
   - The measured variant re-sums the zero point from the centre on every read and charges that sum's rounding.
   - If the anchor is stored and is the curve, an unmoved conic has the same bits, but the error is u times the distance from the anchor. That is the unbounded gain.

**What the evidence points at**

The built type means two things: an optional anchor beside a frame whose origin is a centre, given on one kind and derived on the other. Two origins are kept in step by hand, on the branch `oneround/anchored-conic`:
- Import's `plus_zero_curve` rebuilds a minted ellipse through the centre door and drops its anchor. (found by reading, not run)
- A re-seam of an anchored ellipse re-anchors through the derived centre, losing u·a.
- Every reader of the frame's origin still gets a centre rounded at the far scale, even on an anchored conic.

**A worked case**

A circle of radius 1500 m through the origin; the point 1 m along the arc.
- From the centre: two terms of size 1500 that cancel, each rounded at about 1e-13 m, fresh on every read.
- From an anchor at the origin: terms of size 3e-4 m and 1 m, rounded at about 1e-16 m.
- Handed only the centre, the door rounds once, at about 1e-13 m. That is one rigid shift of the whole curve, the same for every reader, and certification can see it.

**Each answer as a final state**

*A. One anchored form (recommended).*
- Makes true: any in-box point of any conic reads at about u times its distance from the anchor, within the run's admission floor (the rounding of a short chain on in-box coordinates). (likely; modelled, not measured)
- `[simple.conditioned]` ("then treated as real numbers") then holds for every conic. The seven files the bare-rounding gate lists as `far-centre`, the fitted resolution constant (re-fitted 32 → 40 in this build) and the re-seaming doors lose their reason.
- A far-centred conic in centre form cannot be written down (`[unrepresentable]`).
- It does not depend on PR 7131: a frame's one-ulp width multiplies only short chords.
- The anchor sits anywhere on the arc, not only at a principal vertex. A cylinder's shallow cut needs that: its arc is 0.42 to 0.50 rad from any vertex.
- Leaves possible:
  - one edge using both flanks of a thin ellipse (PR 4812 already gives one carrier per edge);
  - a door handed a far centre with nothing better known. In production that comes from a surface whose own axis or centre is far, the same disease one level up, with no row filed.
- Consequences:
  - Every conic's last bits move.
  - An exported centre is derived, so a centre typed as 0.1 can print as 0.09999999999999998.
  - `step-export`'s "identity, field for field" becomes "the same locus and parameter, centre derived".
- Reversible: yes. The form sits behind the shared `Conic` readings.

*B. Two forms, each read as born (what the lane built).*
- Defensible: on PR 6329 you said another under-the-hood form is "not that bad" (as quoted in the far-centre row).
- Keeps main's bits for every conic made through a centre door.
- Leaves a far-centred centre form representable and silently lossy: every circle mint, the cylinder's cuts, any fixture.
- Keeps two evaluators, the rounding charges and an anchor that a rebuild can drop.
- Its main reason is bit stability, which `memories/output-stability-as-justification.md` says may not justify keeping a second implementation.

*C. The clause as written, centre still the truth.*
- Pays every bit move for a 5× constant.
- Its exact-sum refinement gives nothing at `Interval`, and nothing under PR 7131's reading of a frame.
- Dominated by A.

**Contracts and habits**

- Contracts, which A keeps:
  - same build and inputs give the same bits (`[determinism]`), mesh bytes at one and four threads included;
  - reversal is exact structure (the anchored sum is exactly even and odd in the parameter);
  - new and stronger: the point at parameter zero is the stored anchor bit for bit, and its inverse is exactly zero.
- A contract to re-derive, not re-baseline: "a carried certificate equals a fresh one" (2 rows). `[one-way.places]` carries a certificate only on a bit-identical support. I did not trace why these moved.
- Habits, to regenerate: the 3 bit pins, the 6 mesh digests, the 2 query counts, and the quarter-turn identity (it fails on main too when the centre is zero).
- Not decided by the form: the one-hop STEP fixed point. It needs "evaluate, invert, evaluate" to return the same bits, which no form guarantees. That is import's to provide. (likely)
- The 4 moved verdicts need an oracle before landing. That gates the landing; it does not rank the options.

**Text I would change**

- The GERM row's "Decided (Ev, PR 4812)", first bullet: replace "stored in the form it was born in", the imported-ellipse sentence and "evaluators read the anchored form either way" with the recommendation's first three bullets.
- `geom-core` README, "Float rounding": name the form, so that "re-minted in a well-conditioned form" reads "a conic is stored anchored on the arc it serves".

## For the orchestrator

**Provenance**
- I read PR 4812's comments on GitHub; the quotes are from there. The checkout's shallowness did not matter for that.
- The D7 rewrite is commit `edde229fe`, merged by PR 7074; I read Ev's comments there too.
- Ev's PR 6329 remark is quoted from `work/oneround/a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex.md`. I did not check it on GitHub.

**Errors in the brief and the evidence file**
- "An imported ellipse" is not born with a centre as a carrier. `step-import/src/entities.rs` parses the printed `ELLIPSE` through the centre door, but that value is evidence only; `build.rs` builds the carrier with `topo::section_near`. I did not verify that `section_near` reaches `plane_cone_section`'s anchored arm.
- "Two things Ev decided together pull apart" overstates it. The "keep what was given" half was the agent's elaboration of Ev's round-trip wish, and PR 7074 retired its object.

**Defects found off the question (by reading, nothing run)**
- On `origin/oneround/anchored-conic`, `crates/step-import/src/signed_zero.rs` `plus_zero_curve` destructures an ellipse with `..` and rebuilds it with `Ellipse::new`, so a minted anchored ellipse is demoted to a centre form with a derived centre. It is called on the built carrier (`build.rs`, `plus_zero_curve(rebuilt_in_surface(…))`).
- Same branch, `conic.rs`: `quarter_turned_where` and `seamed_toward` re-anchor from `turned.origin() + …`, the derived centre, so a re-seam of an anchored ellipse loses u·a.
- Rows worth filing:
  - A quadric whose axis, centre or apex is far from its face is the same "stored form that loses digits". I found no row for it.
  - `freecad::cross_dialect_fixed_point` rests on an idempotence that no evaluator guarantees. That is IMPORTGATE's.
  - `tangent_circle` (`conic_quadric/in_band_sweep.rs`) builds far-centred circles through the centre door.

**Not checked**
- That A's evaluation holds the admission floor for a general-phase anchor. It is modelled only.
- Why the two carried-certificate rows moved, the 4 verdicts, and whether the fixed-point row holds under A.
- Whether any production circle mint is handed a far centre with no in-box data. The circle mints are unsurveyed, as the evidence file says.
- No cargo run. No `analysis/design-fork/*` branch opened.

**A form for the general anchor (mine, unmeasured)**
- Keep the principal frame and sizes, and store the anchor with its phase as the pair (cos θ0, sin θ0).
- Evaluate by the chord: `P(θ0+s) = P0 + 2·sin(s/2)·[−a·sin(θ0+s/2)·u + b·cos(θ0+s/2)·v]`. The two components are orthogonal and each is at most the chord.
- A quarter or half turn of the seam is then an exact permutation of the pair, and the anchor point never moves.
- I would not store conjugate semi-diameter vectors: the minor semi-axis read back from their cross product loses about u·a/b.
- A circle needs no phase; its seam turns to the anchor.

**Seams**
- PR 7131: under A a conic frame's origin is the anchor, not the centre, so the roughly 650 non-test reads of `frame.origin()` and `.center()` in `geom`, `geom-brep` and `topo` (surfaces included, by grep) need the conic ones renamed, not silently re-meant. Under A, 7131's decision 1 costs conics only chord-scale width.
- The seven `far-centre` rows of `scripts/gates/bare-rounding-sites.tsv` are the charge sites that A retires.
- `step-export/src/lib.rs`'s "Identity is meant literally" and the test `emitted_conic_carriers_equal_the_kernel_carriers_bitwise` change meaning under A.
- From the branch, A is reached by making the centre doors compute an anchor and dropping the `Option`. The run-4 list of 22 is roughly the expected re-baseline, minus the rows above that need an oracle or a re-derivation.