# PROPS sign-hull — `orthonormal_basis` crosses the normal with a decided world axis

**Binding at dispatch** (PROPS program, `work/props/plan.md` §Lanes,
the linalg interval-honesty lane; the item is
`work/props/interval-orthonormal-basis-sign-hull.md` — read it in
full, including §Sized, the §Question for Ev with #1939's four
measurements, and the two ruling sections at its tail; difficulty
logged at spec: **M / NUMERIC**, block PROPS-B2 slot 1, dual review).
Read `docs/prompts/implementer-discipline.md` in full. Branch
`props/sign-hull`, cut from `main`.

## The ruling (Ev, #1944, 2026-09-06): option 1

`Vec3::orthonormal_basis` (`crates/geom-core/src/linalg/vec.rs:~478`)
is Duff's branchless construction, whose seam is the equator
`n.z = 0` — where every vertical wall lives — carried through
`copysign`, whose `Interval` arm must hull `[−1, 1]` at any `n.z`
enclosure touching zero. Every planar wall of every extrusion therefore
stores a sign-hulled `u_ref`, and M10-5's clearance engine re-charts
planar carriers at its own door to get around it. The ruling replaces
the construction: **cross the normal with a world axis chosen by a
total order on the normal's components, and normalise — no sign
transfer anywhere.** Every stored `u_ref` changes. That is the unit's
deliverable, not its cost: a golden that moves because the kernel got
this right re-baselines with its reason in the PR (discipline §3).

### The construction

For unit `n`, let `k` be the index of the component of SMALLEST
magnitude, ties to the HIGHEST index (`z` before `y` before `x`). Then

```text
b1 = normalize(e_k × n)
b2 = n × b1
```

so `(b1, b2, n)` is right-handed (`b1 × b2 = n`). Why this order and
this side of the cross: a horizontal face (`n = ±e_z`) gets `k = y` and
`b1 = ±e_x`; a vertical wall (`n = (c, s, 0)`) gets `k = z`,
`b1 = (−s, c, 0)` — horizontal in the plane — and `b2 = e_z`, up. The
conventional choices a user would make by hand.

Conditioning: `|e_k × n|² = 1 − n_k² ≥ 2/3` for unit `n`, because the
smallest component's square is at most `1/3`. `normalize` is therefore
well-conditioned everywhere on the sphere; there is no equator seam at
all. The construction's discontinuity — one must exist (hairy ball) —
sits on the diagonal set where the two smallest magnitudes tie, and
the frame there changes by a rotation about `n`, never a flip.

### The decision door (the one design point that is the unit's)

The axis choice is a comparison, and generic evaluation code does not
branch on values (DESIGN.md's named-predicate clause; `Real::min`'s
doc). It goes through a **value-level door on `Real`**, spelled ONCE
and implemented on every impl (`f64`, `Interval`, `Dual<T>`, `Sym`,
`k_stats`' wrapper — whatever `grep -rn "fn copysign" crates/geom-core/src`
lists). The reference shape, which the unit may rename or reshape as
long as the properties below hold:

```text
select(d, a, b)  =  a  if d ≤ 0,  else b
```

applied twice: `d1 = |n.z| − |n.y|` selects between the `z`- and
`y`-candidates (ties → `z`), `m = min(|n.z|, |n.y|)`, then
`d2 = m − |n.x|` selects between that winner and the `x`-candidate
(ties → the winner). Applied componentwise to the candidate vectors
(three scalar calls per stage, the same `d`).

Required properties, each with its own test on each impl:

- **`f64`**: a total order; bit-deterministic (D9). The pinned spelling
  is `normalize(e_k × n)` with `k` from the order above, and the unit's
  bitwise test sweeps the sphere plus the axis/equator/diagonal edge
  set — the successor of `orthonormal_basis_matches_the_duff_spelling_bitwise`,
  which is deleted with the spelling it pinned.
- **`Interval`**: when the comparison is DECIDED for every point of the
  enclosure — `d.hi() ≤ 0` (includes the point-tie `d = [0, 0]`, where
  the tie-break is the same for every point, because it keys on
  `d == 0` and not on a zero's sign bit) or `d.lo() > 0` — the chosen
  candidate, decoration passed through; when `d` straddles zero with
  positive width, the **hull of both candidates, decoration capped at
  `Def`** — never `Trv`, never empty, never a manufactured non-real:
  the question is real and the honest answer at an undecided tie is
  both frames (DL6, `docs/DUAL-DESIGN.md`). The enclosure property
  that this buys and the test that pins it: for every `f64` normal
  inside an `Interval` normal, the `f64` frame lies inside the
  `Interval` frame. State in the door's doc why `copysign` could not
  serve here: its zero-containing arm hulls at a POINT zero because an
  `f64` zero's sign bit is invisible in an enclosure (#1939's
  measurement 1), whereas this door's tie-break is sign-blind, so the
  point case decides. `Interval::copysign` itself is untouched; its
  hull stays right for every other caller.
- **`Dual<T>`**: value channel is `T`'s door verbatim (the module's
  value-channel bit-identity contract); the tangent follows the chosen
  candidate, ties keeping the tie-break's choice — the `min`/`max` kink
  convention. At an `Interval` value channel that hulls, the tangent
  is hulled the same way (`copysign_deriv`'s straddle arm is the
  precedent for the decoration).
- **`Sym`** and the `k_stats` wrapper: whatever their `copysign` does,
  by the same shape.

The unit does NOT spell the choice with `copysign` on the difference
(`(1 − copysign(1, d))/2` weights): at `f64` that is a total order too,
but at `Interval` it hulls at the point tie — every horizontal face —
which is the defect this unit exists to remove.

### What changes downstream, and what does not

- **The locus never moves.** A plane is its point and its normal;
  `u_ref` only names its points. Every re-blessed golden below is
  accompanied by that receipt: the plane's origin and normal before and
  after are bit-identical.
- **`newell.rs:156`, `step-import/recognize.rs:228`,
  `topo/boolean/boxes.rs:2512,2563,2634,3033`,
  `geom-core/examples/r2_onb_dl6.rs`** — every production caller;
  none re-spells, each is re-read for an assumption about the old
  frame (e.g. `b1 = e_x` at `n = e_z` — true under both constructions
  by the tie order above; `b1` at a vertical wall is now horizontal,
  which it was not before). Say what each caller assumed.
- **M10-5's re-chart retires outright**: `editor_core::clearance::{in_plane_axis, chart_frame}`
  (`crates/editor-core/src/clearance.rs:~2164-2195`) and both call
  sites (`~2088`, `~2821`) go; the engine reads the stored frame, which
  now refines. The `refines` refusal door (`~2121`, `~2207`) STAYS —
  it is an honesty door, not part of the workaround; measure whether
  any M10-5 fixture still reaches it and say. `crates/geom-core/tests/bounds_census.rs`
  and `crates/editor-core/tests/docm1_face_frame.rs` name the retired
  functions; re-point them.
- **`Datum::FaceFrame`** (`crates/editor-core/src/node.rs:~704`): a doc
  line at `spin` saying the reference is the carrier's stored `u_ref`
  as this constructor makes it — axis order and tie-break named — so a
  reader knows what a spin of zero means on a wall and on a cap.
- **#1939's instruments** (`crates/geom-core/tests/onb_signed_zero_evidence.rs`,
  `crates/geom-brep/tests/onb_c_payoff_interval.rs`,
  `crates/{editor-core,step-export}/tests/onb_wall_normal_census.rs`,
  `crates/step-import/tests/onb_wild_normal_census.rs`,
  `crates/geom-core/tests/r1_p2_onb_probes.rs`): the payoff instrument
  becomes this unit's acceptance row (below); the censuses re-run
  counting the TIE class instead of the signed-zero class (how many
  planar faces across the three corpora sit on an exact tie at `f64`
  — decided, expected: every axis-aligned face — and how many straddle
  a tie at the ε-scaled box — hulled, expected: none); the signed-zero
  evidence file is deleted, its question having no referent, and the
  PR says so. `vec.rs`'s constructor doc (`~395-475`) is rewritten in
  the present tense for the new construction — discipline §4, no
  history; the copysign story lives in git.

## Deliverables

- The door on `Real` and every impl, with the property tests above.
- `orthonormal_basis` re-spelled; `vec.rs`'s existing rows
  (`basis_vectors_are_orthonormal_exactly`, the three proptests,
  `orthonormal_basis_poles_and_equator`, `_interval_residuals`,
  `_is_bounded_over_z_enclosures`,
  `_at_a_vertical_plane_is_bounded_and_certified`) re-derived: the
  ones that state properties stay green with the new expected frames;
  the ones that pinned the Duff spelling are replaced by their
  successors (bitwise against the new spelling; EXACT — not merely
  bounded — at a vertical plane and at a cap).
- **Red-first**: M10-5's 12-gon prism at `Interval` over the ε-scaled
  box (`crates/editor-core/tests/m10_5_clearance_interval.rs`'s
  fixture, through `onb_c_payoff_interval.rs`): all 12 walls' `u_ref`
  exact to within ~1e-15 — including the two whose `n.z` is
  `[−2.2e-16, 2.2e-16]` at `Interval`, which (c′) could not narrow and
  which decide here because `|n.z| ≤ 2.2e-16 < m ≈ 0.26` — and the
  cell's z-enclosure halved per #1939 table 4. Red today (6 walls at
  width 2). Quote the red.
- **Every golden that pins a frame, re-blessed with its reason in the
  PR**: the 18 STEP fixtures under `crates/step-export/tests/fixtures/`
  (every `DIRECTION` record that is a `u_ref`), `crates/editor-core/tests/m4_pr6_golden.rs`,
  the k-lint/K-REPORT baselines if a frame feeds one, and whatever else
  the full workspace suite turns red. Per red row, one of three
  classifications in the PR: (a) a golden pinning the frame —
  re-bless, cite the locus receipt; (b) a test that hand-built its own
  `u_ref` — unaffected, or re-pointed if it asserted the constructor's
  output; (c) a consumer whose BEHAVIOUR changed beyond the frame's
  name — report before touching it; that is a finding, possibly an
  issue file. The hand-built list from `grep -rln "orthonormal_basis\|u_ref" crates/*/tests`
  is ~170 files; only the reds matter, but the classification of each
  red is owed.
- The M10-5 retirement and the `FaceFrame` doc line.
- Sweep obligation (discipline §5): the shape is *a branchless sign
  construction whose `Interval` arm must hull at zero* — every
  `copysign` caller in `crates/*/src` (`grep -rn "\.copysign(" crates/*/src`),
  each classified: sign transfer at a genuinely signed quantity (keep),
  or a branch in disguise that hulls where a decision exists (this
  unit's shape — list, do not fix; file each as an issue if it bites a
  certified lane).

## Seams (file, do not cross without saying so in the PR)

`crates/editor-core/src/clearance.rs` and its `m10_5_*`/`r2_m10_di_*`
tests are M10's; `crates/step-export/tests/fixtures/*.step` and
`crates/step-import/src/recognize.rs` are EXCH's; `node.rs` and
`docm1_face_frame.rs` are DOCM's; `topo/boolean/boxes.rs` is BOOL's.
The orchestrator has posted seam notes; the unit edits these files
for this unit's reason only and lists every such edit in the PR under
a `## Seams crossed` heading.

## Posture

- ε posture: none — no tolerance read moves; say so. The `interval`
  lane is where this unit's acceptance lives: run it locally
  (`--features interval`, the `m10_5_*_interval` and `onb_*_interval`
  rows) and put `CI-Config: lane=interval eps=default` on the commit
  the review is frozen on (the trailer lasts one push).
- Bit identity: **the `f64` frame changes by design** on every normal
  but the ones where the two constructions coincide (name that set —
  expected: none beyond coincidence at a few axis directions). The
  receipt is the re-bless list with the locus invariance row per
  fixture, not stability.
- Review: dual, in the experiment; the reviewers' claims to falsify
  will include the enclosure property at a straddled tie and the
  right-handedness at every axis direction with signed zeros.
- **Landing: the item gets `pr:` and `status: review`. DO NOT MERGE —
  the orchestrator lands after the dual and its fix pass; the fix pass
  closes the item, deletes this spec at merge with its
  `## Per-merge deletion` section in `docs/DOC-LEDGER.md`.** No
  `Co-Authored-By`; push early to `props/sign-hull`.

## Acceptance

The door on every impl with the enclosure property pinned; the 12-gon
prism's 12 walls exact at `Interval`; every moved golden re-blessed
with its locus receipt and classification; M10-5's re-chart gone and
its rows green on the stored frame; the tie census across the three
corpora recorded; the sweep's hit list filed; hosted CI green on the
full matrix including the interval lane.
