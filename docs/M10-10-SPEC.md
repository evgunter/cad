# M10-10 — the form-level mechanism: the arc's trig atoms made exact, so the plate's four identity residuals go at once

STATUS: BINDING (dispatched 2026-09-06; amended A1 2026-09-07, §2; opened from M10-9's measured
result on Ev's standing ruling — a form-level unit before the exit
walk is re-cut, "1 sounds good"; block M10-B5 slot 1). Unit branch
`m10/m10-10-form-level`. Program plan `work/m10/plan.md`; design
record `docs/ERROR-DESIGN.md` E12 (read in full) with E6 as
substrate; the item `work/m10/M10-10.md`; the finding this unit is
built on, `work/m10/plate-ceiling-is-now-the-scaffold-pushforward.md`
(read in full — its staged-walk table is this unit's baseline and its
acceptance), and the instrument finding
`work/m10/first-refusal-at-twice-the-ceiling-is-an-order-artefact.md`
(read in full — every measurement in this unit is taken the way it
prescribes, never at 2× the ceiling).

## Grounding (substrate facts; verify each at the site)

- **The tier as merged** (M10-9, #2048): `geom_core::sym` — the plain
  quotient form on the hybrid ring bounded at `COEFF_BITS = 256`, the
  EARLY walk alongside it (`SymRules::early`: a second memo per leaf
  in which A0 folds at each `sqrt`/`abs` node, rule C rides, and the
  registered-identity door is consulted), rules A/B per node behind
  `early_ab` (built, measured at 138 s for the plate's nominal replay
  on the BigInt ring, dial-off), `SymCounts { symbolic_zero,
  sign_gated, registered, numeric, frozen, … }`, the eighth K token,
  the over-band-set instrument and the staged-ceiling dial
  (`k_stats::identity_pass_*` behind the test-only
  `identity-pass-testing` feature). Read `sym.rs`'s header in full.
- **What bounds every measured document, read correctly.** At
  ceiling + δ, on five documents at three ε rows, one predicate is
  over the band: `carrier_matches_mapped_source`, the scaffold
  residual `spec.carrier.eval(t_i).distance(mc.eval(s_i))`
  (`crates/geom-brep/src/certify.rs`, the `Resolved::Scaffold` arm;
  `t_i = sample_param(0, θ, i)`, `s_i = i/8`, `CERT_SAMPLES = 9`).
  Passing the plate's identity residuals one at a time (the staged
  walk): mapped-source → `carrier_on_surface_2` → `pcurve_map_residual`
  → `witness_on_surface_2` are worth 1.33× / 1.20× / 1.25× / **1.68e5×**
  — the fourth lands the plate at **0.263 of its real study** against
  `assert_bound [7.29e-9, 2.0e-4]`, a macroscopic REAL margin, and five
  further identity residuals move it by nothing. The four must go at
  once; a per-identity door cannot (M10-9's result).
- **Why the two spellings never meet.** The sketch pushforward
  `SketchSegment::eval` (`crates/geom-brep/src/mapped.rs`, the `Arc`
  arm) is anchored on `a`: `a + (R − I)·v`, `θ = 4·atan(bulge)`,
  `sin(s·θ)`, `cos − 1` spelled `−2·sin²(s·θ/2)` — a deliberate
  enclosure-width trade the module documents. The carrier spells the
  same arc through the sagitta closed forms and its own frame, with
  `param_end = arc_span(bulge) = 4·atan|bulge|`
  (`crates/sweep/src/swept.rs:313`), so at sample `i` both sides carry
  `sin`/`cos` atoms of `(i/2)·atan(bulge)` and the forms meet only
  where the trig collapses (`i = 0`). The cylinder residual
  `(w² − r²)/(2r)` (`crates/geom-brep/src/implicit.rs:88-104`) behind
  `carrier_on_surface_2`, the chart residual behind
  `pcurve_map_residual` (`certify.rs`, the `Resolved::Chart` arm:
  `p.distance(surface.eval(q))`, `q = chart.pcurve.eval(sample)`) and
  the witness residual behind `witness_on_surface_2` carry the same
  atoms one level down.
- **The identities are theorems of the reals with no value to read.**
  For any form `X`: `sin(atan X) = X / sqrt(1 + X²)`, `cos(atan X) =
  1 / sqrt(1 + X²)`; multiples by the Chebyshev recurrences
  (`cos(kφ)`, `sin(kφ)` polynomial in `cos φ`, `sin φ`); halves by the
  positive branch, `cos(φ/2) = sqrt((1 + cos φ)/2)`, `sin(φ/2) =
  sign(φ)·sqrt((1 − cos φ)/2)`, positive-branch because `atan X ∈
  (−π/2, π/2)` puts every half-angle in `(−π/4, π/4)` — a fact about
  `atan`'s RANGE, not about a value. Rule A (`sqrt(X)² = X`) and rule
  B (`sin² + cos² = 1`) then close the ring. Nothing here is clause 3.
- **Rulings that bind here**: a symbolic `Zero` is a theorem; no rule
  in this unit reads a value (rule C stays where it is, dial-off); no
  funnel site is edited; the numeric channel's bits are untouched; the
  frontier (iterated quantities) stays S-CERT's — if any of the four
  residuals turns out to be an iterated quantity (a witness found by
  a march or a polish), it is the frontier, named, not widened into.

## Scope

### 1. The instrument first (binding; the first commit)

- **Retire the 2× read.** M10-8's `m10_8_harness::ceiling` and every
  row that reports "first refusal beyond" from a replay at `2·lo`
  either read the over-band SET at ceiling + δ (M10-9's instrument) or
  are deleted; the helper carries no do-not-build-on warning after
  this because nothing is left to warn about. The finding closes.
- **Per residual, the rendered form under the shipped tier**, on the
  plate at ceiling + δ: for each of the four, the atoms it carries
  (`sin`/`cos` of what argument form; `sqrt` of what; `atan` of what),
  its term count and degree, and whether it is an EXPRESSION identity
  (two spellings of one construction) or an ITERATED quantity
  (S-CERT's). `witness_on_surface_2` in particular: say how the
  witness is built, at the site. This table is the PR body's first
  section and decides §2's reach before anything ships.
- The staged walk (the finding's table) reproduced first-hand on the
  merged head as the baseline, at the default ε.

### 2. Rule D and the early walk made affordable (ship what §1 justifies)

- **D. Trig of `atan`, exact.** In the early walk, at a `Sin`/`Cos`
  node whose argument form is `q · atan(X)` with `q` an exact rational
  of the form the samples produce (`i/2`, `i ∈ 0..=8`, and whatever
  `sample_param` and the carrier's own frame produce — read them),
  rewrite to the closed form in `X` and the atom `sqrt(1 + X²)` by the
  identities above (integer multiples by recurrence, halves by the
  positive branch). Sound unconditionally; the `sqrt` atoms it
  introduces are rule A's shape. Representation is the implementer's;
  the property is what the review falsifies: `sin(k·atan X)`,
  `cos(k·atan X)` and their halves decide `Zero` against their closed
  forms at every width and for negative and straddling `X`, and NOTHING
  is folded at an argument form that is not `q · atan(X)` (an `atan2`,
  an `atan` of a sum plus a constant, `q` not exact).
- **Amendment A1 (orchestrator, 2026-09-07, after the implementer's
  report and before the review freeze).** Rule D as written folds only
  at `sin`/`cos` of `q · atan(X)`, and three of the plate's four
  residuals discharged under it; the fourth, `pcurve_map_residual`,
  carries the cylinder chart's phase `cos(atan2(0, ‖a_r‖))` from
  `stable_azimuth` (`crates/geom-brep/src/pcurve_cache.rs`), with
  `‖a_r‖ = r²/sqrt(r²)` — a form that is NON-NEGATIVE BY ITS OWN
  SYNTAX. `atan2(Z, N) = 0` when `Z` is the zero form and `N` is
  manifestly non-negative (a `sqrt` atom, an `abs` atom, an even power,
  a positive literal, and products, quotients and sums of such) is a
  theorem of the reals at every parameter point where `N > 0`, which is
  everywhere the arc is defined; at `N = 0` the geometry is degenerate
  and clause 1 (the numeric channel's own domain answer) decides first,
  and IEEE's `atan2(0, 0) = 0` agrees with the fold where a value
  exists. No value is read: the positivity is syntactic, exactly as
  rule D's half-angle branch is a fact about `atan`'s range. Rule D
  gains this fold (its own dial or D's — say which), with the argument
  at the impl and pinned: `atan2(0, sqrt(X))`, `atan2(0, X²)`,
  `atan2(0, r²/sqrt(r²))` decide `Zero` at every width and for
  straddling `r`; `atan2(0, X)` for a plain parameter `X` NEVER;
  `atan2(Y, N)` with `Y` not the zero form NEVER; a `Z` that is zero
  only numerically (a coincidence) NEVER. Then the plate re-measured:
  the over-band set at ceiling + δ, the whole-certifying box at three
  ε rows against the staged walk's 0.237/0.263/0.263, the first
  refusal beyond it, and the tour's stop 1. The finding
  `pcurve-chart-phase-is-atan2-of-the-start-radial` is this
  amendment's input; its other two routes (a rule-C sign read at the
  funnel's own certified decision; a structural phase from PCURVE)
  stay filed as the alternatives, not built. The unit's "nothing folds
  at an argument that is not `q · atan(X)`" is amended to "…that is not
  `q · atan(X)` or `atan2(zero-form, manifestly non-negative)`".
- **A/B per node, bounded and memoized.** `early_ab` as built costs
  138 s per nominal plate replay; the reduction per node must be
  memoized by content-hash id across the leaf and bounded by the step
  cap, and its cost re-measured with D on. M10-8's reviewer measured a
  bounded alongside variant at 2.55 s per probe with A0 — that is the
  order to reach.
- **The ring width the four residuals need**, measured with D + A/B
  on: `COEFF_BITS` at 256, 512, 1024 on the plate — a bound change
  ships only if the four discharge at it and the leaf cost stays
  within §4's line. The cost line: a leaf replay of the plate's real
  study ≤ 10× M10-9's (0.16 s → 1.6 s), and the tour's stop 1 within
  the CI job that runs it; past that line the mechanism ships dial-off
  with its numbers and the acceptance is stated as not met.
- **The census**: every row the new rules discharge, the bucket
  "D"/"early A/B" per row; `line_span` and `contact_at_shared_vertex`
  (identity-shaped, M10-9's census) measured too.

### 3. The alternative, measured not built

Retiring the scaffold residual for arc carriers (the finding's second
option) is a PCURVE/D3 question and is NOT this unit's. If §2
discharges `carrier_on_surface_2`, `pcurve_map_residual` and
`witness_on_surface_2` but NOT `carrier_matches_mapped_source`, file
the PCURVE question with the plate's number under the three and the
mapped-source residual's rendered form; do not edit the certifier.

### 4. Honesty instruments

- No new K token: a D/early-walk zero is an unconditional theorem and
  counts in `symbolic_zero`; the driver row must LINT with the count
  moved (read the per-file and TOTAL lines from the hosted log; state
  the `symbolic_zero` delta against M10-9's 48,682 per CSV).
- **The five documents re-measured** at three ε rows, door and rules
  on, as the over-band set at ceiling + δ: the plate (the acceptance),
  R2's bracket, R1's annulus, R2's pad, R2's link — each ceiling with
  its ε-dependence stated and what bounds it now. The E12 claim is a
  ceiling that STOPS scaling with ε; say per document whether it does.
- The plate's real study driven whole: certified leaves, refusals by
  class, the first refusal a REAL margin (`assert_bound` or another
  genuine flip) with its enclosure; the tour's stop 1 becomes the
  certified study and its caption says the numbers.
- M10-9's pins that state the plate at `7.8e2·ε` FLIP by design —
  re-cut as positive pins asserting the mechanism (which rule
  discharged which residual, what bounds now), bisection bracket at
  both ends, ε-relative.
- `work/m10/real-margin-dependency-widening.md` is where the next
  ceiling is expected once the identities are gone; if a document is
  bounded by it, say so with predicate and enclosure and do not widen
  into it.

## Out of scope

The scaffold residual's retirement (PCURVE/D3); implicit and iterated
quantities (S-CERT); any rule that reads a value; edits at any funnel
site; the fillet's registrant (filed); a form-level axiom store (the
mechanism here is algebra, not registration); the GUI.

## Review claims to falsify

1. **Rule D is sound and reads no value**: the closed forms decide
   `Zero` at every width for `k ∈ {1, 2, 3, 4}` and halves; negative
   `X`, straddling `X`, `X` a form with atoms of its own; NEVER at an
   argument that is not `q · atan(X)`; the positive-branch argument for
   halves is a theorem of `atan`'s range and is pinned as such.
2. **§1's table is real**: the four residuals' rendered forms and
   classifications reproduce; `witness_on_surface_2`'s witness is
   built as the table says.
3. **The four discharge**: at ceiling + δ on the plate the over-band
   set holds no identity residual; the per-predicate split at the
   nominal moves each of the four from numeric to theorem.
4. **The plate certifies its real study**: whole-certifying box at
   0.263 of the study (the finding's `2.630e8·ε` at the default ε) at
   ALL THREE ε rows — the ceiling no longer scaling with ε — with the
   first refusal beyond it a REAL margin and its enclosure; or, if not,
   the bounding predicate and enclosure named per row.
5. **Zero impact off**: the new dials off reproduce M10-9's tier
   bit-identically (serialized verdicts and receipts) on every M10
   fixture; every non-`Sym` scalar unchanged.
6. **Cost**: leaf cost per document with the rules on against M10-9's
   numbers; the ring measurement; the tour's stop 1 within its budget;
   the affordability line met or the mechanism dial-off with the
   number.
7. **The instrument**: no row in the tree reads "first refusal" at
   2× the ceiling; the over-band set at ceiling + δ is the only
   spelling; the order-artefact finding closes.
8. **The five documents**: each re-measured, each ceiling's
   ε-dependence stated, what bounds each now named.
9. Every deviation in the PR body with the argument; D-numbering from
   D1.

## Acceptance

Hosted CI green on the full matrix on the final head with the driver K
row LINTED (per-file counts and TOTAL quoted from the log,
`symbolic_zero` moved against M10-9's); §1's table; the re-cut pins
green at three ε rows; the five ceilings with their ε-dependence; the
cost line met or the dial-off stated with the number; every deviation
in the body. The program's exit condition is this unit's acceptance:
**a macroscopic box certifies** — the two-hole plate's real study
returns certified leaves bounded by genuine flips, not by ε. After
merge the orchestrator re-cuts `docs/M10-EXIT-WALK.md` (#1700)
against the measured state.
