//! **The symbolic identity tier** (ERROR-DESIGN E12): a lane scalar that
//! carries, beside its numeric value, a handle into an expression DAG
//! over the document's continuous-parameter symbols — so a margin whose
//! expression is IDENTICALLY ZERO in the parameters decides
//! [`Sign::Zero`] before any enclosure is consulted, for every parameter
//! value, at any box width.
//!
//! # The defect this closes
//!
//! The kernel's certification population is full of checked IDENTITIES:
//! an edge's endpoint lies ON its carrier, a side plane is cosurface
//! with its neighbour. Their margin is exactly zero in real arithmetic
//! for every parameter value, and their INTERVAL enclosure over a box of
//! width `w` is `[0, c·w]` with `c ≈ 2–4`, because the two sides of the
//! identity reach the funnel as two separately evaluated intervals and
//! interval arithmetic cannot see that the occurrences of the parameter
//! are one number. A leaf goes definite only once its own width is a
//! fraction of ε, so a macroscopic tolerance box refuses all of its mass.
//! No re-association at the decide site can recover it — the dependence
//! was lost upstream — which is why the tier tracks it from the
//! parameter down.
//!
//! # What a symbolic `Zero` claims, and why it is a THEOREM
//!
//! [`Decide::sign_within`] on a [`Sym<T>`] answers `Ok(Sign::Zero)`
//! without consulting the enclosure exactly when two things hold:
//!
//! 1. the value channel **certifies** ([`CertifiedEnclosure`]) — the
//!    computation was defined on the whole input box; and
//! 2. the node's POLYNOMIAL NORMAL FORM over the parameter symbols is
//!    the zero polynomial.
//!
//! The normal form is computed with EXACT rational coefficients and
//! **nothing in it ever reads a value**. Every node denotes a real-valued
//! function of its indeterminates (the parameter symbols, π, and the
//! opaque atoms below), and every scalar operation's value channel
//! encloses that same real. So a form that is zero as a polynomial is
//! zero under every real assignment of its indeterminates — in
//! particular under the actual one, at every parameter point of the box.
//! That is the whole soundness argument, and it is pinned by the
//! `sym_theorem` rows of `geom-core`'s suite.
//!
//! Clause 1 is not decoration. Without it a domain violation
//! (`sqrt(-1) - sqrt(-1)`) would decide `Zero` on an expression that has
//! no real value at all; the gate is the same door
//! [`crate::Interval::sign_within`] refuses at, for the same reason.
//!
//! # The DAG, and what is opaque in it
//!
//! Nodes are `Param(symbol)`, `Lit(f64 bits)`, `Pi`, `Add`/`Sub`/`Mul`/
//! `Neg`/`Powi`, `Div` as `Mul(a, Inv(b))`, and OPAQUE atoms for every
//! other [`Real`] operation (`sqrt`, `abs`, the trigonometric family,
//! `floor`, `min`, `max`, `copysign`, and the span-hull seam), keyed by
//! the normal form of their arguments. Two atoms whose arguments have
//! the same normal form are the SAME indeterminate — which is what lets
//! `sqrt(x² + y²) − sqrt(y² + x²)` cancel — and an atom applied to a
//! zero form folds to the value the function takes at zero where that
//! value is rational (`sqrt 0 = 0`, `cos 0 = 1`, `acos 0 = π/2`).
//!
//! The periodic reducers are deliberately NOT overridden: [`Real`]'s
//! defaulted bodies are a fixed composition of `÷`, [`Real::floor`], `·`
//! and `−`, so they decompose into the DAG on their own and only the
//! `floor` stays opaque. Overriding them would be strictly less
//! cancellation for no gain.
//!
//! **What the plain form is, and what it does not reach**, is [`form`]'s
//! own docs, beside the quotient they are about.
//!
//! # The arc family, and what reaches it (M10-8)
//!
//! **The family, named.** A swept arc's carrier is `Circle { center:
//! c, radius: r, u_ref: (q − c).normalize() }` (`sweep::swept`), so its
//! identities carry `sqrt` atoms: `u_ref·u_ref = (v·v)/sqrt(v·v)²`, the
//! rim's endpoint pinning `‖c + (q − c)·r/‖q − c‖ − q‖ = 0` iff
//! `‖q − c‖ = r`, and every dimension a document authors arriving as an
//! `f64` literal with a 53-bit mantissa. Through M10-7 the tier reached
//! none of it: the forms did not freeze on their SIZE (the budget
//! is never reached), they froze on their COEFFICIENTS — `sqrt(1)^58`
//! and `sqrt` of exact-square dyadic constants stood opaque in every
//! rim form, and the products of three 53-bit mantissas overflowed the
//! `i128` the coefficients were kept in.
//!
//! **The constant fold, by measurement** (M10-8; the first rule of the
//! shipped set, [`SymRules::shipped`]): **A0** — `sqrt(c)` and `abs(c)`
//! of a constant form fold to the exact rational — in a second walk
//! ALONGSIDE the plain form (the plain form is asked first and stays
//! M10-7's, so nothing it proves is lost), over an arbitrary-precision
//! coefficient ring bounded at [`rational::COEFF_BITS`]. That alone moves R2's
//! filleted bracket's whole-certifying box from `3.7e1 · ε` to
//! `3.9e2 · ε` (10.4×) and R1's annulus from `2.0e1 · ε` to `7.8e2 · ε`
//! (39×), at about 1.8× the cost per leaf of M10-7's tier where the
//! plain form does not answer (plate 0.35 → 0.65 s, bracket 1.47 →
//! 2.7 s); M10-4's stepped shaft certifies its real ±0.1 study whole.
//! Per predicate at the nominal, A0 turns `carrier_on_surface_1` on
//! the plate from 108/72 (theorem/numeric) to 180/0, and on the bracket
//! from 0/243 to 108/135. Run REPLACING the plain form instead it is
//! cheaper and loses: two bracket rows at the nominal
//! (`carrier_endpoint_start` 44 → 42, `carrier_matches_mapped_source`
//! 234 → 225) and M10-6's min-clearance boxes, to coefficient freezes at
//! the bound — which is why it ships alongside.
//!
//! **Rules A and B** — `sqrt(X)² = X` and `sin² + cos² = 1`
//! (`algebra`) — add no discharge over the top residual on any
//! measured document, and PER NODE in the early walk they are what
//! closes the ring behind rule D (the M10-10 section below). **Rule C** —
//! `sqrt(X) = R` where `X = R²` as forms and `R`'s sign is certified
//! over the box, clause 3, the one fold that reads a value (`signed`)
//! — is sound, unit-pinned, folds on no document at the shipped bound,
//! moves no ceiling at any bound, and stays dial-off.
//!
//! # The registered-identity door (M10-9)
//!
//! **What the tier cannot prove, a constructor can state.** E12 keeps
//! a recourse in reserve for exactly the rim residual above —
//! discharge by PROVENANCE — and [`Sym::register_equal`] is it: a
//! session-level record that two DAG nodes denote one function of the
//! parameters, made by the site that GUARANTEES it
//! (`sweep::swept::register_rim_identity`, whose doc comment carries
//! the proof), verified in the lane scalar at the moment it is made,
//! refused typed when it lies, and consulted by a THIRD normal-form
//! walk beside the plain one and the early one.
//!
//! A registered `Zero` is not a theorem and is never counted as one.
//! The tier's own zeros rest on exact rational arithmetic and nothing
//! else; this one rests additionally on the registrant's argument, so
//! it has its own column ([`SymCounts::registered`]), its own K token
//! (`registered`) and its own line in the driver's receipt. The
//! attribution is NECESSITY: a decision counts there only when the
//! plain form and the early form have both declined and the walk with
//! the registry answers, so `symbolic_zero` is M10-8's on every
//! document, to the decision.
//!
//! **The unit of scope is the CONSTRUCTOR, not the identity.** The
//! swept arc carrier's builder registers EVERY same-object identity it
//! guarantees whose consumer node it can build identically, and it
//! guarantees two: the RIM identity `‖q_from − c‖ = r` and the SPAN
//! identity `carrier.eval(param_end) = q_to`, registered componentwise
//! (`sweep::swept::register_rim_identity` and `register_span_identity`,
//! each with its proof). The span registrant is E12's reserve's own
//! example — "a typed 'built as `carrier.eval(t0)`' token" — and it is
//! same-object because `Curve3::eval`'s `Circle` arm DELEGATES to a
//! `Real`-bounded door (`Curve3::circle_at`) the constructor calls too,
//! so the node it states the identity about is the node the certifier
//! asks about.
//!
//! The revolve's latitude carriers (`sweep::revolve::surfaces` and
//! `::full`) mint the same circle under the same guarantee and state
//! the RIM identity too — the same rule applied to the second
//! constructor, not to the second identity. Rim only: neither builder
//! is handed the far endpoint, so the span identity has nothing to be
//! stated about (`work/blend/revolve-carriers-state-only-the-rim`).
//!
//! **Where the door may be called is an ALLOWLIST**, not a
//! convention: `scripts/gates/register-equal-allowlist.sh` names the
//! ratified constructor sites, because the method hands every generic
//! `T: Real` body a value COMPARISON — the capability evaluation-code
//! discipline exists to keep out of that position, and one that adds no
//! bound for `no-extra-real-bounds` to see.
//!
//! **What they reach, measured** (M10-9, at the shipped set; the
//! per-predicate tables are `editor-core/tests/m10_9_evidence_interval`
//! and the pins `m10_9_pins_interval`). At each document's nominal both
//! endpoint pinnings go from numeric to registered outright —
//! `carrier_endpoint_start` 16/16 on the plate, 16/16 on R1's annulus,
//! 20 of 22 on R2's bracket, 24 of 32 on R2's pad; `carrier_endpoint_end`
//! 16/16, 16/16, 20/20 and 24 of 28 — plus part of
//! `carrier_matches_mapped_source` (8, 8, 8 and 12). They reach
//! `carrier_on_surface_*` not at all: those rest on `u_ref·u_ref = 1`,
//! which needs the SQUARED identity `v·v = r²` rather than either of
//! these, and the three are different nodes.
//!
//! **What the door alone does NOT move** (M10-9's finding): no
//! ceiling on any of the five measured documents changes by a digit
//! with the door alone — read as the over-band SET at the refusing end
//! of the bisection, exactly one predicate is over the band on all five
//! documents, at all three ε rows, door open and door shut:
//! `carrier_matches_mapped_source`, the carrier against the
//! `MappedCurve` pushforward at the certifier's own samples, an
//! identity between two INDEPENDENTLY BUILT objects that meet only
//! where their trig collapses
//! (`work/sym/plate-ceiling-is-now-the-scaffold-pushforward`). The
//! door reaches its `i = 0` sample and no other — until the trig is
//! written in closed form, which is the section below: with rule D on,
//! the door is what closes that residual at EVERY sample, and the two
//! move the plate together where neither does alone
//! (`m10_10_pins_interval`).
//!
//! **The bound is a SET at ceiling + δ, never one drive's first
//! refusal.** A drive stops at the first predicate that refuses, and at
//! a scale past the ceiling several are over the band at once, so which
//! name comes back is evaluation ORDER — validation before
//! certification. Read at twice the plate's ceiling, the reported
//! refusal walks with each registrant. The instrument is one
//! home — `editor-core/tests/m10_8_harness`'s `over_band_set` and
//! `bound`, the bracket and the set at its refusing end — and nothing
//! in the tree spells a bound any other way.
//!
//! # The form-level algebra (M10-10)
//!
//! **Rule D — trig of `atan`, exact** ([`SymRules::trig_of_atan`],
//! `trig`): in the early walk a `sin`/`cos` node whose argument form is
//! `q · atan(X)`, `q = k/2ᵐ`, rewrites to its closed form in `X` and the
//! atom `sqrt(1 + X²)` — `cos φ = 1/S`, `sin φ = X/S`, halves on the
//! positive branch (a theorem of `atan`'s RANGE: `φ/2ʲ ∈ (−π/4, π/4)`
//! has a positive cosine, so `cos(θ/2) = +sqrt((1 + cos θ)/2)` and
//! `sin(θ/2) = sin θ/(2·cos(θ/2))`, no sign read), multiples by angle
//! addition; and, under the same dial (amendment A1), `atan2(Z, N)` of
//! the zero form over a form non-negative BY SYNTAX (`sqrt`/`abs`
//! atoms, even powers, positive coefficients, perfect squares, and
//! their products, quotients and sums — `manifest::nonneg`) is
//! the zero form, and `sin`/`cos` at an exact half-multiple of π is
//! its constant. Nothing folds at any other argument shape. The two
//! spellings of an arc — the pushforward's `sin(s·θ)`, `−2·sin²(s·θ/2)`
//! at `θ = 4·atan b` and the carrier's `cos t`, `sin t` at `t =
//! (i/8)·4·atan|b|` — are then rational functions of the same atoms,
//! and **rules A/B per node** ([`SymRules::early_ab`]) close the ring:
//! the substitution is linear (`algebra::poly_subst_square` accumulates
//! one numerator over one common denominator), bounded by
//! [`EARLY_STEPS`] and [`EARLY_AB_TERMS`], and skipped on a form past
//! the size cap. With them, the early walk also takes the **zero
//! normalization** `0/d + x = x`, `0/d · x = 0` (`combine`): the
//! quotient form cancels no common factor, so a zero numerator dragged
//! its denominator into every sum — an arc's `n̂ · apothem` at bulge one
//! is `0/‖chord‖`, its centre became `mid·‖chord‖/‖chord‖`, and rule A
//! expanded the `‖chord‖²` that rode along into polynomials of rising
//! degree with 53-bit coefficients that froze at every ring width.
//! All three are behind dials; [`SymRules::without_the_algebra`] is
//! M10-9's tier bit for bit, and `m10_9_pins_interval` holds M10-9's
//! rows under it.
//!
//! **What it reaches on the plate, at the nominal** (theorem / gated /
//! registered / numeric, `m10_10_pins_interval`): `carrier_on_surface_2`
//! 108/0/0/72 → 180/0/0/0 and `witness_on_surface_2` 12/0/0/8 →
//! 20/0/0/0 as THEOREMS; `carrier_matches_mapped_source` 180/0/8/64 →
//! 180/0/72/0, every sample through the DOOR — rule D makes the trig
//! meet, and the rim identity `‖q − c‖ = r` the registrant states is
//! what closes it, so the count is `registered`, honestly. The fourth
//! residual the staged walk names, `pcurve_map_residual`, 0/0/0/36 →
//! 0/0/36/0 through the door as well: it carries the chart's phase
//! `atan2(0, r²/sqrt(r²))` from the cylinder chart derivation
//! (`pcurve_cache::stable_azimuth`, whose `u_ref` on the extrude's
//! wall is the start's own radial), which the A1 fold takes as the
//! zero form — `atan2` of the zero form over a form non-negative by
//! syntax, positive wherever the arc exists, and the `r² = 0` box is
//! one clause 1 refuses first — and on the definitely-negative frame
//! the azimuth's `+ π` leaves `cos π = −1`; the rim identity closes
//! what is left. Nothing read a sign: the fold is a fact about
//! `atan2`'s value on a syntactic class, the same posture as the
//! half-angle branch.
//!
//! **What it moves, measured at ε = 1e-6, 1e-9 and 1e-12** (the
//! over-band set at ceiling + δ; `m10_10_evidence_interval`, pinned in
//! `m10_10_pins_interval`):
//!
//! | document | M10-9 | M10-10 | over the band at ceiling + δ |
//! | --- | --- | --- | --- |
//! | two-hole plate | `7.81e2 · ε` | **0.2368 / 0.2631 / 0.2631 of its REAL study** (the three rows) | `assert_bound` — the web assertion, `[−2.1e-9, 2.0e-4]` at `1e-9` |
//! | R1 annulus | `7.81e2 · ε` | **0.6963 / 0.8416 / 0.8415 of its real study** | `dihedral_wedge` `[1.0e-5, 5.3e-2]` at `1e-6`; `arc_diameter_clearance` `[−5.6e-8, 8.4e-4]` at the finer rows |
//! | R2 link | `4.93e2 · ε` | unmoved | `carrier_matches_mapped_source` |
//! | R2 filleted bracket | `3.87e2 · ε` | unmoved | `carrier_matches_mapped_source` |
//! | R2 rounded pad | `2.08e3 · ε` | `[2.4990e3, 2.5010e3] · ε` | `line_span` (identity-shaped) |
//!
//! The plate's and the annulus's whole-certifying CEILINGS stopped
//! scaling with ε — each is a fraction of the document's real study —
//! and what bounds each is DEPENDENCY WIDENING of a real margin, not a
//! flip (`work/sym/real-margin-dependency-widening`, the class E12
//! hands the ceiling to). On the plate the bound is the study's own web
//! assertion, whose margin `web − floor = 1e-4 + 2·Δhs − Δr_a − Δr_b`
//! is AFFINE: its true range at scale `s` of the study is `1e-4 ±
//! 1.6e-4·s`, so the flip first enters the box at `s = 0.625`, while
//! at the pinned ceiling `s ≈ 0.263` the true margin is `[5.79e-5,
//! 1.42e-4] > 0` everywhere and the enclosure is `[−2.09e-9, 2.00e-4]`
//! — widened by ~6e-5 on each side (pinned:
//! `m10_10_pins_interval::m10_10_the_plates_ceiling_is_dependency_widening_not_a_flip`);
//! at `1e-6` the same widened enclosure sits in the band. The annulus
//! is the same class: `arc_diameter_clearance` cannot be zero for any
//! `r > 0` — the widening finding's second site. The plate's rows are
//! the staged walk's own end (0.2368, 0.2630, 0.2631 with every
//! identity residual passed) to the bisection step, and passing
//! further residuals moves nothing. The LEAVES certify up to the real
//! flip: driven whole at 1024 leaves the plate's real study is 431
//! certified / 593 refused, every refusal the leaf budget, and a
//! refused leaf refined further is bounded by `{assert_bound}` alone
//! at every depth (`m10_10_the_plates_real_study_driven_whole`; both
//! reviews' refinements, adopted as `m10_10_r1_probes_interval` and
//! `m10_10_r2_probes_interval`; the tour's stop 1). So "not by ε" is
//! the ceiling's statement and "up to a genuine flip" the leaves'.
//! The other three still scale with ε — each is bounded by an
//! identity residual. On the link and the bracket the scaffold
//! residual stands because the term/coefficient BUDGET freezes their
//! carrier frames' squared components — the link's residual is
//! `sqrt(Σ)` over two frozen squared components, the bracket's both
//! components freeze before squaring — at any affordable width: the
//! per-node size cap is a cost wall and not a reach (raising
//! [`EARLY_AB_TERMS`] 512 → 4096 leaves the link at ceiling + δ
//! byte-identical, and with the budget raised to 32,768 terms /
//! degree 256 the bracket's bracket is unchanged at 16× the leaf cost;
//! both reviews, by execution). What they wait on is the scaffold
//! residual's retirement for arc carriers (PCURVE/D3). On the pad the
//! fillet's identity-shaped `line_span` is a `Min` over frozen
//! 60-term, degree-16 products (`work/sym/symbolic-tier-census`).
//! And the reach is the UNIT bulge: a parameter bulge is outside the
//! mechanism (R2's D-tab: `3.52e2 · ε` on and off alike) and a literal
//! bulge other than 1 leaves residue — at M10-10 R1's boss at bulge 2
//! stood at `carrier_matches_mapped_source` 6 of 54 and
//! `carrier_on_surface_2` 27 of 90 numeric with its ceiling unmoved;
//! SYM-5's rule E has since taken the first six and eighteen of the
//! twenty-seven and moved that ceiling `8.2611e2 → 9.3559e2 · ε`, so
//! what stands there now is 0 of 54 and 9 of 90 —
//! `work/sym/rule-d-reaches-the-unit-bulge-only`.
//!
//! **What it costs** (release, one whole-box leaf, algebra off → on):
//! plate at `1e2 · ε` 0.15 → 0.21 s; plate at its REAL study 0.02 →
//! 0.20 s (the affordability line is 1.6 s; with A1 every residual is
//! worked to the end, so the leaf costs what the whole walk costs);
//! bracket 0.51 → 0.81 s; annulus 0.09 → 0.17 s; pad 2.2 → 10.1 s;
//! link 0.42 → 5.0 s. The ring stays
//! at [`rational::COEFF_BITS`]: the plate's four residuals discharge there
//! once the zero normalization and A1's folds are in (the first three
//! needed the normalization, the fourth A1 — neither needed a wider
//! ring), and 512 and 1024 add no discharge (measured before the
//! normalization: 512 moved nothing, 1024 reached two of the first
//! three at 12 s per nominal replay — the coefficient growth was the
//! artefact, not the reach; A1's folds are width-independent and are
//! pinned at all three widths).
//!
//! **What the rules add to the plain form, and what still stands.** The
//! SHIPPED tier layers the atom algebra on top (the M10-8, M10-9 and M10-10
//! sections above): `sqrt(x)·sqrt(x) − x` and `sin² + cos² − 1` DO decide
//! as theorems under [`SymRules::shipped`] (rules A and B, over the top
//! residual and per node), and `sin`/`cos` of `q · atan X` fold to closed
//! forms (rule D). What still stands with the shipped set is what needs a
//! SIGN: `|x| − x` on a nonnegative `x` is rule C's, and rule C is
//! dial-off. These are limits of the tier and not bugs in it — over-refusal
//! is the safe direction, and every such margin falls to the numeric
//! channel exactly as before. What the PLAIN form alone reaches is
//! [`form`]'s own docs.
//!
//! # Rule E — the quotient's common factor (SYM-5)
//!
//! **What a normalisation costs the plain form, and what it buys
//! back.** [`form`]'s quotient cancels no common factor, so a unit
//! vector — `v / sqrt(v·v)`, three quotients over one atom `A` —
//! leaves `A` in both halves of everything built from it, each further
//! normalisation multiplies the shared power and each square doubles
//! it. [`SymRules::common_factor`] divides that factor out in the
//! EARLY walk, at every node: the monomial both halves share, and then
//! the whole quotient when the numerator is a rational multiple of the
//! denominator. Both are equalities of rational functions wherever the
//! denominator is non-zero, which clause 1 guarantees; `quotient`
//! carries the argument and the shapes it must not fold.
//!
//! **What it reaches, measured** (the tilted derived frame:
//! `Datum::Frame { u: (1,0,0), v: (0,1,t) }`, a cube extruded from it,
//! a `FaceFrame` on its cap, a boss on that —
//! `editor-core/tests/m10_derived_frame_tilted_interval`). With the
//! rule off the derived boss refuses under `Guided` at every rung
//! (`carrier_endpoint_start` at `[0, 1.8e-2]` under the plain form and
//! A0, `newell_plane_residual` straddling under the shipped set, 632
//! frozen on DEGREE with kids at total degree 69–128) while its
//! authored twin certifies; with it on the derived boss certifies
//! where the twin does at `ε/8` and `1e-3` — NOT at `5e-2`, where the
//! refusal left is the value channel's (a clause-1 `Invalid` margin,
//! `work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`)
//! and the tier has already proved the residual zero. Raising
//! the budget to 4,096 / 65,536 does NOT do it — 483 frozen and the
//! same refusal — so this is reach and not a cost wall. The frozen
//! forms carry `sqrt(P/P)` for a degree-8 `P`: the number one, held as
//! an opaque atom because neither half of `P/P` is a constant for A0
//! to read.
//!
//! **What it moves on the measured documents** (`m10_bulge_interval`,
//! `m10_10_pins_interval`): R1's boss at `bulge = 2`
//! `carrier_on_surface_2` 63/0/0/27 → 81/0/0/9 in THEOREMS — 18
//! numeric decisions became theorems — `witness_on_surface_2`
//! 7/0/0/3 → 9/0/0/1, and `carrier_matches_mapped_source` 72/0/48/6 →
//! 72/0/54/0 through the door; its whole-certifying CEILING moves
//! `8.2611e2 · ε → 9.3559e2 · ε` (1.13×), which is the move
//! `work/sym/rule-d-reaches-the-unit-bulge-only` measured a 512-bit
//! ring making and the rule makes at [`rational::COEFF_BITS`]. The
//! D-tab's `carrier_endpoint_start` 24/0/8/4 → 24/0/12/0 on both
//! spellings, and its `carrier_matches_mapped_source` 126/0/36/18 →
//! 126/0/42/12 on the literal against 126/0/38/16 on the parameter —
//! the one row where the two spellings part. **No count falls
//! anywhere, and no ceiling on the five measured documents moves by a
//! digit** (plate, annulus, link, bracket, pad: the bracket at both
//! ends and the counts at ceiling + δ are identical with the rule on
//! and off).
//!
//! **What it costs** — on the affordability line's OWN instrument, one
//! whole-box leaf (`m10_10_leaf_cost_with_and_without_the_algebra`),
//! release, rule E off → on: plate at `1e2 · ε` 0.132 → 0.358 s, plate
//! at its REAL study 0.141 → 0.340, annulus 0.120 → 0.287, bracket
//! 0.438 → **1.699**, link 3.312 → **2.427**, pad 3.850 → **14.404**.
//! The line is 1.6 s, so the bracket, the pad AND the link are over it
//! — the link at BOTH dials, and cheaper with the rule than without,
//! because the forms the rule shrinks are the ones the walk then
//! multiplies. That is a disclosed deviation and not a silence.
//!
//! On the OTHER instrument — one probe of the ceiling bisection, which
//! is not what the line is defined for — the same five read
//! 0.23 → 0.47, 0.17 → 0.34, 0.53 → 1.23, 3.42 → 2.28 and
//! 3.73 → 10.90. It SHIPS on the balance: a document class the tier
//! could not reach at all, a ceiling moved on a sixth, four pinned
//! splits raised, nothing lost, and the pad and the bracket are
//! documents the tier already carries at no dial.
//! [`SymRules::without_rule_e`] is M10-10's tier bit for bit.
//!
//! **The reach is the DOCUMENT's, not a class.** Reached: the tilt
//! about `v` above, non-unit authored axes, and derived frames stacked
//! two deep — under `Pinned` that document certifies at both dials and
//! the rule is what makes it affordable (219.4 s off against 1.1 s on),
//! and under `Guided` the rule takes its refusals 4 → 1, the one left
//! being the value channel's clause-1 `Invalid` on the boss. NOT
//! reached: a tilt about `u`, where the rule turns the degree wall
//! into a TERM wall, and a `FaceFrame` on a REVOLVED body's cap, which
//! neither dial certifies. [`quotient`]'s header carries the mechanism
//! and `editor-core/tests/m10_derived_frame_tilted_interval`'s
//! `sym5_the_reach_on_documents_the_unit_did_not_build` the numbers.
//!
//! # Rule F — the manifest sign (SYM-8)
//!
//! **What a `copysign` costs the tier, and what the form already
//! knows.** [`Vec3::orthonormal_basis`](crate::Vec3::orthonormal_basis)
//! is the branchless Pixar construction, and its first two lines are
//! `s = 1.copysign(n.z)` and `r = 1/(1 + |n.z|)` — a `copysign` and an
//! `abs` of one quantity, the normal's `z`. Both reach the DAG as
//! OPAQUE atoms, so a frame built through them carries two
//! indeterminates that stand for nothing the tier can cancel against.
//! On a `FaceFrame` over a body extruded from a frame tilted about `u`
//! (`u = (1,0,t)`) that `z` is `1/sqrt(P(t))` — an `Inv` of a `sqrt`
//! atom, positive wherever it has a value at all — and the `abs` over
//! it is an atom UNRELATED to the `sqrt` it was built from, so nothing
//! downstream cancels and the squares freeze.
//! [`SymRules::manifest_sign`] folds both in the early walk:
//! `copysign(Y, X) → abs(Y)` and `abs(X) → X` wherever the FORM shows
//! `X` positive, and `copysign(Y, X) → −abs(Y)`, `abs(X) → −X`
//! wherever it shows `X` NEGATIVE — the START cap of that same body,
//! whose `n.z` is `−1/sqrt(P(t))`. [`manifest`] carries the predicate
//! and its reflection, the four identities as equalities of reals
//! under clause 1, and the SIGNED-ZERO edge that makes both predicates
//! strict rather than `manifest::nonneg`'s non-negativity.
//!
//! **Where it sits against A/B/C/D/E.** At the node, in `combine`,
//! early walk only: A0's exact constant fold first, then this rule,
//! then rule C — the value-free rule before the one that reads a
//! value, so a discharge that can be a theorem is never counted
//! `sign_gated`. **What pins that order is a residual BOTH rules
//! take** — `geom-core`'s `sym_rule_f_rows`'s
//! `the_order_against_rule_c_is_pinned_by_a_residual_rule_c_would_take`
//! (`abs(1 + t²)` over a bracket) and
//! `a_shape_both_rules_take_is_what_pins_the_order` (`abs(2/t²)`): each
//! reads `theorem` at the shipped order and `sign_gated` with rule F
//! shut, and planting rule C before rule F reds both. A row rule C
//! cannot reach is green either way and pins nothing, which is what the
//! first cut of this section claimed and both reviews disproved.
//! Against rules A/B and E the order is STRUCTURAL rather than chosen:
//! they run after `combine` returns, on the form this rule left, and an
//! atom this rule prevents from being minted is not one they could have
//! folded later. The walk ledger
//! (`editor-core/tests/m10_sym_profile_interval`) is unmoved by the
//! rule on the slab and the plate — every form either walk builds is
//! digest-identical — which is the same statement as "it fires nowhere
//! on them", and is not a pin on the order.
//!
//! **What it reaches, measured** (the tilt-`u` derived frame,
//! `editor-core/tests/m10_derived_frame_tilted_interval`'s
//! `sym8_phase1_*` rows — the document SYM-5's rule E turned a DEGREE
//! wall into a TERM wall on and stopped). At `half = 1e-3` under
//! `Guided` the refused `carrier_endpoint_end` residual is
//! `sqrt(?#…)` over a FROZEN `Powi ^2` whose kid is 440 terms at
//! degree 27 over 298 at degree 28, and `440² > MAX_TERMS`: with the
//! rule on that node is built, the predicate goes 24/0/0/1 → 33/0/0/0
//! — every decision a THEOREM — and the document's refusal moves on to
//! a `newell_plane_residual` straddle, a wall this rule does not reach
//! (`work/sym/the-tilt-u-newell-residual-is-the-next-wall`). Under
//! `Pinned` the same document certifies at both dials and the rule
//! moves 122 decisions out of `numeric` into `symbolic_zero`
//! (754 → 876) at a sixth of the cost (2.6 → 0.4 s at `1e-3`,
//! 2.3 → 0.3 s at `5e-2`). The authored twin is untouched.
//!
//! **The negative arm (SYM-12) reaches the other sign of the SAME
//! shape, and no wider.** The positive arm alone refused a negative
//! coefficient outright, so the START cap of that same cube — whose
//! normal is the negation, `n.z = −1/sqrt(P(t))` — and the same frame
//! with `v` flipped (`FlipZ`, whose `n.z` is the end cap's negated)
//! read the tilt-`u` document's rule-F-OFF numbers. The negative arm
//! takes both to the END cap's rule-F-ON state BY NAME AND BY COUNT:
//! under `Guided` at `half = 1e-3` the refused `carrier_endpoint_end`
//! (28/0/0/1 on the start cap, 24/0/0/1 on `FlipZ`) is 33/0/0/0, every
//! decision a theorem, and the refusal moves on to a
//! `newell_plane_residual` straddle 32/0/0/1 — `FlipZ`'s is the end
//! cap's residual exactly mirrored (the same DAG, enclosure
//! `[−5.744e-2, 5.744e-2]`), the start cap's is its OWN residual
//! (`[−3.0416e-2, 3.0464e-2]`) that matches by the predicate's decision
//! count; under `Pinned` both certify at both dials and the arm moves
//! the start cap 108 decisions out of `numeric` (768 → 876
//! `symbolic_zero`, 12.2 → 1.0 s) and `FlipZ` 122 (754 → 876,
//! 7.8 → 1.2 s) (`m10_derived_frame_tilted_interval`'s
//! `m10_the_start_cap_and_flip_z_read_the_end_cap_under_the_negative_arm`
//! is the gating pin; `sym12_phase1_the_one_sided_documents_ladder`
//! the ladder). The end cap itself is bit-identical under the arm.
//!
//! **What the reach IS, stated no wider than the documents behind
//! it**: a `FaceFrame` whose `carrier_endpoint_end` residual carries
//! the frame's own `copysign`/`abs` atoms — a tilt about `u`, either
//! cap, either sign of `u` or `v` (`FlipX`, `u` flipped instead of
//! `v`, reads the same at `half = 2e-3`). It is NOT "every negative
//! `n.z`": a tilt about `v` with `v` flipped (`FlipV`,
//! `n.z = −1/sqrt(1 + t²)`) moves NOT ONE count at either lift, arm
//! on or off, because with rule F shut its `carrier_endpoint_end` is
//! already 32/0/0/0 and its first refusal already the Newell straddle
//! — the frame's atoms never reach a residual the tilt-`v` family
//! stops on (`sym12_a_negative_nz_the_arm_folds_and_does_not_reach`
//! carries both documents).
//!
//! **What neither arm reaches, and why.** A tilt about `u` AND `v`
//! (`tiltUV`), which both SYM-8 reviews predicted as the shape rule F
//! folds, moves no count at either lift, and SYM-12 rendered why
//! (`sym12_phase1_the_tilt_uv_document_rendered`): on that document
//! the DAG's `n.z` is not `1/sqrt(P)` but a quotient of two
//! polynomials in the parameter's offset (degrees 20 and 22, every
//! coefficient positive) whose terms carry the parameter at ODD powers
//! beside three `sqrt` atoms, one over a frozen node — no term is
//! signed by syntax, so the `copysign(1, n.z)` atom STANDS in the
//! rule-F-on render and what refuses the document at both dials is a
//! `Sub` the early walk freezes on its kids' size. Not the budget's:
//! the atom is never removed, so no budget reaches it. A frame whose
//! `n.z` is a bare parameter over a `sqrt` atom (`tiltNZ`) is declined
//! by both arms, as it must be: its sign is a fact of the box, not of
//! the form.
//!
//! **What it moves on the measured documents: nothing, with one
//! exception.** Every per-predicate split at the nominal is
//! BIT-IDENTICAL with the rule on and off on seven of the eight
//! (plate, annulus, bracket, link, R1's segment boss, both D-tabs; the
//! pad's nominal split with the shape report installed exhausts the
//! measuring box's memory at BOTH dials and is not takeable there),
//! and every whole-certifying ceiling is identical to the digit on all
//! EIGHT, with the over-band set at ceiling + δ identical too. The
//! exception is the pad's replay at the scale it certifies whole at:
//! `symbolic_zero` 858 → 854, `registered` 104 → 128, `numeric`
//! 991 → 971, `frozen` 2750 either way — the same 1953 decisions, 24
//! of them moving into the door, twenty out of `numeric` and FOUR out
//! of `symbolic_zero`. Those four are the unit's finding: opening an
//! atom the early walk was cancelling OVER can cost that walk a
//! theorem, which is
//! `work/sym/coefficient-ring-width-is-not-monotone-in-reach`'s class.
//! No decision is lost, the registry re-takes all four, and both counts
//! are pinned side by side (`m10_9_pins_interval`'s `Study` carries
//! `symbolic_zero` beside `registered` since SYM-8, so a later change
//! costing four more theorems reds).
//!
//! **The difference from rule E, stated rather than glossed**: rule E
//! also loses a theorem to an opened form, and that loss is
//! demonstrated at the SCALAR
//! (`sym_rule_e_rows::rule_e_can_cost_a_theorem_to_the_coefficient_ring`)
//! with no measured document paying it. Rule F's is realised ON a
//! measured document. The spec's Phase-1.3 stop clause reads on that,
//! and shipping the rule on anyway is a spec deviation RATIFIED by the
//! SYM orchestrator on 2026-09-21 — not a disclosure the lane made for
//! itself (`work/decide/SYM-8.md` carries the ruling and its reason).
//!
//! **What it costs — and this is the ONE place the numbers live**
//! (the rule table above points here rather than repeating them, and
//! the PR body quoted them from here). The affordability line's own
//! instrument, one whole-box leaf
//! (`m10_10_leaf_cost_with_and_without_the_algebra`), release, rule F
//! off → on: plate at `1e2 · ε` 0.493 → 0.501 s, plate at its REAL
//! study 0.527 → 0.482, annulus 0.439 → 0.401, bracket 2.490 → 2.368,
//! link 3.285 → 3.321, pad 19.734 → 18.796. On the
//! ceiling-bisection instrument the eight documents read 0.43 → 0.42,
//! 1.14 → 1.14, 0.32 → 0.31, 9.74 → 9.25, 2.06 → 1.99, 0.26 → 0.25,
//! 0.21 → 0.20 and 0.59 → 0.58 seconds a probe. The rule is free to
//! within the measurement's noise and slightly cheaper on most
//! documents — it removes indeterminates and mints none.
//! [`SymRules::without_rule_f`] is SYM-5's tier bit for bit.
//!
//! **The negative arm's cost on the same instrument** (SYM-12, release,
//! one whole-box leaf, rule F shut → shipped with both arms, on a box
//! shared with another lane at load ≈ 3): plate at `1e2 · ε`
//! 0.397 → 0.379 s, plate at its real study 0.403 → 0.376, bracket
//! 2.052 → 2.022, annulus 0.330 → 0.327, pad 16.212 → 16.022, link
//! 2.661 → 2.789; a second shipped run reads 0.378, 0.387, 1.998,
//! 0.328, 16.030, 2.710. What the numbers show: shipped is CHEAPER
//! than shut on five of the six leaves, by 0.9 % (annulus) to 6.7 %
//! (the plate's real study) — the pad by 0.190 s against a 0.008 s
//! shipped-shipped spread, so that one is not noise — and dearer on
//! the link alone, +0.128 s (4.8 %) against a 0.079 s spread. A rule
//! that fires on none of the eight and costs one coefficient-sign scan
//! per declined node cannot be what makes a leaf cheaper; the
//! differential also removes the first cut's `−N` allocation on every
//! declined node, which is the direction of five of the six. The
//! bracket, the link and the pad stand over the 1.6 s line as they have
//! since rule E, disclosed there.
//!
//! # Node ids are CONTENT HASHES (D9)
//!
//! A node's id is a 128-bit structural hash of `(op, children ids,
//! payload bits)` — never a sequence number. An id is therefore the same
//! under every rayon schedule and every insertion order, structural
//! sharing is free, and two builds of the same expression memoize the
//! same normal form. The hash-consing table is per-leaf-replay
//! ([`with_session`]), holds nothing across leaves, and is dropped with
//! the leaf; so are the early and door memos, the registry and the
//! parameter brackets.
//!
//! **The PLAIN memo is the exception, and it is per DRIVE when a drive
//! installs one** ([`DriveMemo`], [`with_session_memo`]). A node's
//! plain form is a function of its id, the budget and the two dials the
//! plain walk consults, and of nothing else: the walk reads no value,
//! every atom in it is opaque, and rule A0 is the only rule. So a form
//! one leaf built is the form every other leaf of that drive would
//! build — the argument this section already makes for two occurrences
//! of a node inside one leaf, applied across leaves. `DriveMemo`'s own
//! header carries the argument whole, the one premise that is not a
//! content hash (an `Opaque` id is a per-leaf SEQUENCE number, pinned
//! by execution), and the lock discipline.
//!
//! Distinct expressions colliding on a 128-bit content hash would be a
//! soundness break; this is the standard hash-consing assumption and it
//! is stated rather than hidden. Under a drive memo the population it
//! is made over is a DRIVE's nodes rather than one leaf's.
//!
//! # Freezing: the budget, and why it is sound
//!
//! A form whose term count or total degree exceeds the session's
//! [`SymBudget`], or whose coefficient arithmetic overflows the in-tree
//! rational, is FROZEN: the node becomes an indeterminate of its own,
//! keyed by its content hash. Cancellation THROUGH a frozen node is
//! lost; soundness is not, because an unknown function of the parameters
//! is exactly what an indeterminate denotes. Two structurally identical
//! frozen nodes still share an id and therefore still cancel. Every
//! freeze is counted ([`SymCounts::frozen`], which argues what the
//! column means on a leaf and on a drive).
//!
//! **The coefficients** — the exact rational, the bound it is refused
//! past and the freeze discipline that bound keeps — are
//! [`rational`]'s own docs.
//!
//! # Cost: where the tier's time goes, by count
//!
//! Measured with two instruments — `valgrind --tool=callgrind` over
//! one leaf replay, and the structural profile behind the test-only
//! `sym-profile-testing` feature (`profile`: forms per op with their
//! sizes, every freeze with the cause noted at the refusal site, the
//! ring's promotions, each walk's clock, and each walk's ORIGIN) — on
//! the M10-3 slab and the two-hole plate at their nominals; the rows
//! are `editor-core/tests/m10_sym_profile_interval`, the tables and
//! the re-run method
//! `work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive`.
//!
//! **Who asks for the forms.** The `Decide` impl has three callers of
//! the walks and only one is the tier deciding: its DECISION path asks
//! a form only where the numeric channel cannot answer; the
//! contradiction CHECK runs the discharge on every DEFINITE margin —
//! at an EXACT witness wherever debug assertions are on (dev, test,
//! and this workspace's release profile, so every profile measured
//! here and only the published build not), and at an INEXACT one in
//! every profile, because there it is a receipt column and not an
//! assertion ([`SymCounts::theorems_disputed`]); and the shape report,
//! when installed, renders
//! blocked residuals through the walks. On the slab at its nominal the
//! decision path builds 9,686 plain forms in 980 calls and 36 early
//! forms in 16; the assertion builds 918 plain and 1,958 early forms
//! in 510 calls each — a tenth of the plain walk's forms and 95 % of
//! the early walk's, so the slab's `reduce_steps` count (1,994 calls)
//! is the assertion's. Over the chamber drive it is 1.57 M of 19.1 M
//! plain forms and 3.18 M of 3.35 M early forms, 43 s of 162 s in
//! the walks. On the plate at its nominal the assertion freezes 488 of
//! 1,312 (360 of the plain walk's 1,044 `frozen`), the decision path
//! 824. The plain walk's freezes are the drive's frozen SET whoever
//! asked for them ([`SymCounts::frozen`]).
//!
//! **The tier's instructions are TERM STORAGE, then the walk itself;
//! arithmetic is second on the plate and degree is nowhere on the
//! slab.** In release, one bare replay at the nominal is 99.0 M
//! instructions on the slab and 713 M on the plate (`callgrind`, self
//! cost by function, partitioned by class): the allocator, the term
//! vector and the heap `Vec` each monomial is are 40 % on the slab
//! and 51 % on the plate; the coefficient ring 9 % and 15 %
//! (`num-bigint` 0.1 % and 0.7 % of those); the walk and the DAG
//! build — `form_in`, `intern` — 24 % and 6 %; the merge loops of
//! `Poly` 12 % and 18 %; the atom algebra's own code 0.3 % and 2 %;
//! and a session's teardown — dropping the memos and the table, which
//! no walk clock sees — 12 % on the slab and 6 % on the plate. The
//! slab's forms are tiny — 1.5 terms on average, 10 at most, total
//! degree up to 68, and NOT ONE freezes at any leaf of the chamber
//! drive (19.1 M plain forms, `frozen = 0`) — so what the slab pays
//! is volume times a fixed cost per form: at the nominal 10,604 plain
//! forms per leaf (9,686 the decision's) for 1,490 decisions, at
//! ~4.7 k instructions each (`plain_form` inclusive over the forms it
//! builds), over a DAG of 12,208 nodes interned afresh per leaf
//! (`intern` is 18 % of a release replay); a leaf of the drive
//! averages 8,488 nodes and 7,463 plain forms. The plain walk is 50 %
//! of a slab replay inclusive, the early walk 12 %, the per-node rule
//! A/B reduction 3 %; the answer is the same at the nominal and over a
//! leaf-sized box, because over 2 ε an identity's enclosure is still
//! not definite and the decision path builds the same forms either
//! way.
//!
//! **Across a DRIVE that volume is one DAG's worth of forms, computed
//! once per leaf** — which is what [`DriveMemo`] removes. Over the
//! M10-3 chamber drive (2,559 sessions) the plain walk computes
//! 19,099,919 forms for 18,833 DISTINCT ids: every leaf builds the same
//! 7,464 of them and a memo keyed by the id can answer 1,014 of every
//! 1,015. The plate at 256 leaves (511 sessions) is 9,005,864 forms for
//! 17,624 distinct ids — 17,624 per leaf, the same set every time. What
//! the memo comes to at the drive's end is that DAG and no more: on the
//! slab 18,833 forms and 41 atoms in 6.98 MB, on the plate 17,624 forms
//! and 359 atoms in 9.78 MB, pinned as ceilings by
//! `editor-core`'s `m10_sym_drive_memo_interval`.
//!
//! Measured on the drives at the test profile, one take on the
//! measuring box: the slab at 1,280 leaves 157.1 s → 78.2 s
//! sequentially and 40.1 s → 21.0 s over four workers; the plate at 256
//! leaves 247.5 s → 205.4 s and 65.0 s → 51.4 s. In RELEASE the slab is
//! 2.7–3.05× (both of SYM-7's reviewers re-took it): the test profile
//! spreads a quarter of the count over glue release inlines away, and
//! that glue is in both lanes.
//!
//! **The class the memo helps is a PLAIN-WALK-DOMINATED drive**, and
//! the three documents say so between them. The slab halves because the
//! plain walk is half of its replay and nearly all of it is
//! recomputation. The plate moves by a fifth, because half of it is the
//! per-node rule A/B reduction inside the EARLY walk, which consults
//! the leaf's registry and stays per leaf. And a boss on a derived
//! frame over a tilted datum — every leaf refused, the early walk the
//! whole cost — moves by NOTHING: 275.8 s with the memo on against
//! 276.1 s with it off at 48 leaves (R2's reading). A drive of that
//! shape pays the memo's bookkeeping and collects none of its win.
//!
//! The memo's own cost on a leaf that gains nothing from it — the
//! bookkeeping, and the `Arc` the forms are held behind so that a hit
//! hands back the allocation rather than a copy of it — is measured
//! rather than assumed: one bare replay at the nominal, in release
//! under callgrind, 98.78 M → 98.90 M instructions on the slab
//! (+0.1 %) and 711.14 M → 711.31 M on the plate. `Rc` is kept only
//! where a form cannot cross a leaf at all (rule D's closed forms).
//!
//! Those are the shares AFTER two changes to what a form costs to
//! hold and to normalise, each measured against the tree before it
//! with every count — forms, atoms, frozen, decisions by outcome, and
//! the digest chain of every form the walks build
//! (`m10_sym_profile_interval`'s walk ledger) — identical. A `Poly`'s
//! terms as one sorted vector in the map's order instead of a
//! `BTreeMap` ([`form`]'s header): a slab replay 141.5 M → 104.3 M
//! instructions, a plate replay 1,300 M → 976 M, the storage class
//! from 54 % to 38 % of the slab and 56 % to 38 % of the plate — the
//! tree nodes, their allocation and their teardown gone, the heap
//! monomial (measured inline at width four: under one percent more,
//! not taken) and the `Rc<Form>` per memo entry still there. The ring
//! skipping the gcd and the products by one on the dyadic shape
//! ([`rational`]'s `from_parts`): the slab 104.3 M → 99.0 M, the plate
//! 976 M → 713 M — the plate's `num-bigint` share from 13 % to 0.7 %,
//! because nearly all of its heap arithmetic was gcds against one and
//! products by one on 256-bit numerators. The chamber drive's test-
//! profile wall, one sequential take on the measuring box: 368 s →
//! 240 s → 225 s. The shares are a measurement with no guard and no
//! register: they are re-taken by running the named rows, and nothing
//! reds when they stop being true, because an instruction share is a
//! reading of one box on one day and not a contract the tier makes —
//! what the tier contracts (every count, every form's digest) is what
//! the walk-ledger row pins.
//!
//! On the plate the same storage share sits inside `reduce_steps` —
//! rules A/B per node, 51 % of the replay — and the freeze population
//! is what the budget note in `editor-core`'s `SymbolicDials` says it
//! is: 1,312 freezes over the three walks (1,044 in the plain walk),
//! **1,032 on DEGREE** with the kids already at total degree 40–117 in
//! one to seven terms, 280 on the coefficient bound (widest refused
//! 401 bits against [`rational::COEFF_BITS`]), and none on the term budget —
//! no form on either document comes within 40× of it. Rule D's fold is
//! 0.4 % of the plate's replay; the ring's heap path is 9 % of its
//! operations, so the `i128` inline path holds on both documents.
//!
//! # The census: which identity-shaped predicates this tier reaches
//!
//! Two greps over `crates/` and `demos/` — one for the names handed to a
//! funnel door, one for identity/gap-shaped string literals — and their
//! union minus the bare filter words and the test-harness names. **107
//! names.** The rule is written out in
//! `work/sym/symbolic-tier-census.md`, which also carries the full
//! table: one row per name, with its bucket, its evidence and its site.
//! Only the counts and the two families that matter are here.
//!
//! | bucket | count |
//! | --- | --- |
//! | IMPLICIT (S-CERT's frontier) | 4 |
//! | NOT A PREDICATE | 8 |
//! | EXPLICIT | 95 |
//!
//! **107 and not the 66 the previous sweep reported**, because that
//! number is not re-derivable from a rule written down anywhere and this
//! one states its own. The difference is filter width, not new
//! predicates.
//!
//! **IMPLICIT — 4**, and this is the census's load-bearing claim:
//! `ssi_on_locus` and `ssi_on_locus_foot` (a marched intersection
//! point's residual and the foot of its projection),
//! `plane_nurbs_on_locus` (a chart-image foot) and
//! `offset_reanchor_on_carrier` (an offset carrier re-anchored through a
//! solve) — EXACTLY the four S-CERT's frontier item already names, at
//! either filter width. A quantity found by iteration has no expression
//! in the parameters, so no normal form reaches it and its residual
//! widens with the box whatever this tier does.
//!
//! **NOT A PREDICATE — 8.** Seven are `pncad-py` TAG strings for error
//! and enum variants; `carrier_kind` is a diagnostic name on an
//! `Indeterminate` carrying `MarginDiag::Invalid`
//! (`topo/src/boolean/carrier_eq.rs`) — a structure contradiction, with
//! no margin ever classified.
//!
//! **EXPLICIT — 95.** Closed forms in the parameters over analytic
//! carriers. Nine carry a MEASURED symbolic/numeric split from
//! `editor-core/tests/m10_7_census_probe.rs` (at `Sym<Probe>`, through
//! the same funnel, over the M10 fixtures and the tour's plate):
//! `carrier_endpoint_start` and `carrier_endpoint_end` (56/16 each),
//! `carrier_matches_mapped_source` (288/72), `carrier_on_surface_1` and
//! `carrier_on_surface_2` (216/72 each), `carrier_circles_identity`
//! (6/0), `side_cylinders_cosurface` (4/0), `carrier_cyl_axis_parallel`
//! (3/0), and `side_planes_cosurface` at 0/8. The rest carry their site,
//! and many run at `f64` over no parameter box at all — a fact about
//! this repository's fixtures rather than about their margins.
//!
//! `side_planes_cosurface`'s 0/8 is not a miss: consecutive walls of a
//! rectangle are genuinely NOT cosurface, so all eight are definite
//! non-coincidences. Its margin,
//! `perp_dot(normalize(prev.b − prev.a), next.b − prev.a)`
//! (`sweep/src/swept.rs`), is a quotient of polynomials and the form
//! reaches it wherever the walls really are cosurface — which is what
//! `side_cylinders_cosurface` at 4/0 on the plate's holes shows.
//!
//! # What the census CANNOT see, and it is the expensive part
//!
//! Both sweeps filter on WORDS, so a predicate whose name contains none
//! of them is invisible to the instrument however much it decides. Five
//! such names appear in the driver's own K CSV:
//! `newell_plane_residual` (1,584 symbolic decisions),
//! `segment_straightness` (1,650), `witness_at_mid_parameter` (1,377),
//! `dihedral_wedge`, and `arc_diameter_clearance`.
//!
//! Two of them are not a footnote. **`dihedral_wedge` is what sets the
//! slab's certification ceiling** — it lands in the band over a wide box
//! — and `newell_plane_residual`'s INVALID arm is what a one-leaf replay
//! of a still wider box fails on. So the predicate that bounds
//! certification today is one this census was structurally unable to
//! name, which says the instrument answers "which identity-shaped
//! predicates does the tier reach" and NOT "which predicates bound
//! certification". Those are different populations and the second one is
//! `work/sym/real-margin-dependency-widening.md`.
//!
//! # No session, no tier — and what that does NOT mean
//!
//! Ids are computable without the table, so a [`Sym<T>`] built outside
//! [`with_session`] still carries a deterministic id — the lookup simply
//! misses, the form freezes, and the decision falls to the numeric
//! channel.
//!
//! **A node the session cannot expand contributes an unknown, never a
//! value.** A node minted before the session was installed is not IN the
//! session's table, so its form freezes to an indeterminate keyed by its
//! own id — and two occurrences of that same node share the id, so they
//! still cancel: `a − a` decides `Zero` for such an `a`, inside a session
//! that never saw it built. That is SOUND (one id is one expression, so the
//! cancellation is a real theorem about a real subexpression) but it is not
//! "off": the tier can only ever discharge FEWER identities than it would
//! with the full table, never more, and mixing a pre-session node into a
//! session's DAG cannot manufacture a theorem that a fully-recorded replay
//! would not also reach.

use core::cell::{Cell, RefCell};
use core::ops::{Add, Div, Mul, Neg, Sub};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::predicate::{Band, Decide, Indeterminate, MarginDiag, Sign};
use crate::real::{Bounds, CertifiedEnclosure, Real};
use crate::spline::{KnotVector, SpanLocate, SpanSet};
use crate::tolerance::Tol;

/// The atom algebra: the rule A/B reductions over a residual.
#[path = "sym/algebra.rs"]
mod algebra;
/// The seam pins: the discharge vocabulary's spellings held against
/// one another, one row per seam.
#[cfg(test)]
#[path = "sym/discharge_pins.rs"]
mod discharge_pins;
/// The normal form itself: the polynomial, the quotient of two of them,
/// and the pure operations on a form.
#[path = "sym/form.rs"]
mod form;
/// Rule F: the manifest sign — `copysign` and `abs` atoms whose sign
/// the form already shows, and the non-negativity predicate rule D
/// shares with it.
#[path = "sym/manifest.rs"]
mod manifest;
/// The DRIVE-scoped plain memo: the one piece of the tier's state that
/// outlives a leaf, and the argument that lets it.
#[path = "sym/memo.rs"]
pub mod memo;
#[cfg(feature = "sym-profile-testing")]
pub mod profile;
/// Rule E: the quotient's common factor — the shared monomial divided
/// out, and a constant ratio folded to its constant.
#[path = "sym/quotient.rs"]
mod quotient;
/// The coefficient tower: the exact rational the normal form's
/// coefficients are, the integer under it, and the bound they are
/// frozen at.
#[path = "sym/rational.rs"]
mod rational;
/// The shape report — the instrument that says, per decide site that
/// stayed numeric, what blocked it.
#[path = "sym/report.rs"]
pub mod report;
/// Rule C: the polynomial square root and the clause-3 fold, with the
/// one value read the tier makes (a parameter bracket in the ring).
#[path = "sym/signed.rs"]
mod signed;
/// Rule D: trig of `atan`, exact — the closed forms of `sin`/`cos` at
/// `q · atan(X)`.
#[path = "sym/trig.rs"]
mod trig;

use form::{Form, Poly, powi_form, within};
pub use memo::{DriveMemo, MemoSize};
use rational::Rat;

// ---------------------------------------------------------------- ids

/// A DAG node's identity: the 128-bit structural content hash of
/// `(op, children ids, payload bits)` (module docs).
///
/// Never a sequence number, so it is stable across rayon schedules,
/// insertion orders and repeats — D9 for free.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymId(u128);

impl SymId {
    /// The **empty child slot**, and nothing else. Reserved: the mixer
    /// never produces it, so it can never collide with a node's id.
    ///
    /// It used to be a second thing as well — the id every
    /// [`Sym::opaque`] value carried — and that was a soundness defect,
    /// because one id means one expression: `opaque(1.0) - opaque(2.0)`
    /// hashed to a difference of a node with ITSELF, whose form is the
    /// zero polynomial, and `sign_within` answered `Zero` on two values
    /// that are not equal. Two untracked reals are two unknowns, so each
    /// opaque value now mints its OWN indeterminate ([`SymOp::Opaque`])
    /// and this constant names one thing.
    const UNRECORDED: Self = Self(0);

    /// The id's bits — for a determinism check that has to compare two
    /// DAGs without owning their nodes.
    #[must_use]
    pub fn bits(self) -> u128 {
        self.0
    }
}

/// A 128-bit FNV-1a over the little-endian bytes of the words fed to it
/// — a fixed, platform-independent mixer, which is what D9 asks of an
/// identity that has to agree across builds.
///
/// **The third FNV in this tree, and deliberately not shared with the
/// other two.** `editor_core::eval::memo` runs two 64-bit FNV lanes for
/// evaluation CONTENT KEYS, and `editor_core::stackup`'s `Digest` runs
/// one for a pairing COMPARISON. All three are FNV-1a because FNV-1a is
/// a dozen lines with no dependency and a fixed spec, which is the
/// property each of them wants; that is a shared REASON, not shared
/// code. They are not one type because they answer to different
/// contracts: this one is a node identity that must agree across
/// processes and builds forever (a change to it changes every id and
/// every memoized form), the memo's keys never leave the process, and
/// the stackup digest is explicitly "never a content key". Hoisting
/// them together would put the loosest contract and the strictest one
/// behind one name, and `geom-core` cannot depend on `editor-core` in
/// any case. Said here so the duplication is a decision rather than an
/// oversight.
struct Hash128(u128);

impl Hash128 {
    const OFFSET: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
    const PRIME: u128 = 0x0000_0000_0100_0000_0000_0000_0000_013b;

    fn new() -> Self {
        Self(Self::OFFSET)
    }

    fn word(mut self, w: u64) -> Self {
        for byte in w.to_le_bytes() {
            self.0 ^= u128::from(byte);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
        self
    }

    fn wide(self, w: u128) -> Self {
        self.word(w as u64).word((w >> 64) as u64)
    }

    /// The digest, with zero folded away: `SymId(0)` is reserved for the
    /// unrecorded leaf, so no real node may claim it.
    fn finish(self) -> u128 {
        if self.0 == 0 { Self::OFFSET } else { self.0 }
    }
}

/// The symbol a document parameter enters the DAG as: a hash of its
/// name, so two evaluations of the same document agree on it without
/// carrying a string into a `Copy` scalar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParamSymbol(u64);

impl ParamSymbol {
    /// The symbol for a parameter name.
    #[must_use]
    pub fn of(name: &str) -> Self {
        let mut h = Hash128::new().word(0x5359_4d5f_5041_5241);
        for b in name.as_bytes() {
            h = h.word(u64::from(*b));
        }
        Self(h.finish() as u64)
    }
}

// -------------------------------------------------------------- nodes

/// What one DAG node computes. Everything outside the ring operations
/// is an OPAQUE atom (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SymOp {
    Param,
    /// **One untracked real**, minted by [`Sym::opaque`]: a value whose
    /// expression this session did not build, carrying a per-session
    /// sequence number in its payload so that each such value is its
    /// OWN unknown. Never keyed by the value's bits — two intervals
    /// that happen to be equal are still two different reals, and
    /// keying by bits would re-introduce the false theorem this op
    /// exists to prevent.
    Opaque,
    Lit,
    Pi,
    Add,
    Sub,
    Mul,
    Neg,
    /// An integer power, **EXPANDED into the form** rather than kept as
    /// an opaque atom.
    ///
    /// The unit's spec listed `powi` among the opaque atoms and this is
    /// a deliberate departure from it, disclosed as a deviation: an
    /// integer power of a rational function IS a rational function, so
    /// expanding it costs nothing in soundness and buys every
    /// cancellation that runs through a square. It is what makes a
    /// SQUARED DISTANCE cancel — `‖a − b‖²` reaching the form as a sum
    /// of squares rather than as an unknown — and squared distances are
    /// most of what the certification identities are written in. The
    /// expansion is budget-checked at every step ([`powi_form`]), so a
    /// large exponent freezes rather than allocating its way to the
    /// ceiling.
    Powi,
    /// `1/x` — an atom, so `Inv(b)·b` does not fold to one.
    Inv,
    Sqrt,
    Abs,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Floor,
    Atan2,
    Min,
    Max,
    Copysign,
    /// The multi-span enclosure hull ([`SpanLocate::enclosure_hull`]).
    /// Keyed by CHILD IDS rather than by their normal forms, because a
    /// hull is a function of the operands' ENCLOSURES and not of the
    /// reals they stand for — two expressions with equal normal forms
    /// can carry different enclosures, so keying it by form would claim
    /// an equality that does not hold.
    Hull,
}

impl SymOp {
    /// The op's tag in the content hash — an explicit number per
    /// variant, so reordering the enum cannot silently re-key a DAG.
    fn tag(self) -> u64 {
        match self {
            Self::Param => 1,
            Self::Lit => 2,
            Self::Pi => 3,
            Self::Add => 4,
            Self::Sub => 5,
            Self::Mul => 6,
            Self::Neg => 7,
            Self::Powi => 8,
            Self::Inv => 9,
            Self::Sqrt => 10,
            Self::Abs => 11,
            Self::Sin => 12,
            Self::Cos => 13,
            Self::Tan => 14,
            Self::Asin => 15,
            Self::Acos => 16,
            Self::Atan => 17,
            Self::Floor => 18,
            Self::Atan2 => 19,
            Self::Min => 20,
            Self::Max => 21,
            Self::Copysign => 22,
            Self::Hull => 23,
            Self::Opaque => 24,
        }
    }

    /// How many of the node's two child slots this op reads.
    fn arity(self) -> usize {
        match self {
            Self::Param | Self::Opaque | Self::Lit | Self::Pi => 0,
            Self::Neg
            | Self::Powi
            | Self::Inv
            | Self::Sqrt
            | Self::Abs
            | Self::Sin
            | Self::Cos
            | Self::Tan
            | Self::Asin
            | Self::Acos
            | Self::Atan
            | Self::Floor => 1,
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Atan2
            | Self::Min
            | Self::Max
            | Self::Copysign
            | Self::Hull => 2,
        }
    }
}

/// One DAG node: its op, its payload bits (`Lit`'s float bits,
/// `Param`'s symbol, `Powi`'s exponent) and up to two children.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SymNode {
    op: SymOp,
    payload: u64,
    kids: [SymId; 2],
}

impl SymNode {
    /// **The children this node actually has** — `kids` is a fixed pair
    /// and the arity says how much of it is the expression; every walk
    /// over the DAG wants this slice and none of them wants the
    /// padding.
    fn kids(&self) -> &[SymId] {
        &self.kids[..self.op.arity()]
    }

    fn id(&self) -> SymId {
        SymId(
            Hash128::new()
                .word(self.op.tag())
                .word(self.payload)
                .wide(self.kids[0].0)
                .wide(self.kids[1].0)
                .finish(),
        )
    }
}

// --------------------------------------------------------- the session

/// The freezing budget: the size a normal form may reach before the node
/// is frozen into an indeterminate of its own (module docs).
///
/// A run dial, not a constant of nature — the driver carries it, and the
/// frozen count is the evidence for whatever it is set to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymBudget {
    /// The most terms one form may hold.
    pub max_terms: usize,
    /// The largest total degree one form may reach.
    pub max_degree: u32,
}

impl SymBudget {
    /// A budget of zero terms: every form freezes, so every decision
    /// falls to the numeric channel. The differential that shows the
    /// tier's only effect is the identities it discharges.
    #[must_use]
    pub fn none() -> Self {
        Self {
            max_terms: 0,
            max_degree: 0,
        }
    }
}

/// What one session's decisions came to — the E12 receipt (`symbolic`
/// against `numeric`) and the honesty column beside it (`frozen`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SymCounts {
    /// Decisions answered `Zero` by the symbolic tier as unconditional
    /// theorems — the normal form read no value.
    pub symbolic_zero: u64,
    /// Decisions answered `Zero` through a clause-3 fold (rule C,
    /// [`SymRules::signed_root`]) — a theorem CONDITIONAL on a sign
    /// read over the leaf's box, the one value the tier reads
    /// ([`signed`]). Kept apart from `symbolic_zero` because the two
    /// claims differ in kind; the matching K token is `sign_gated`.
    pub sign_gated: u64,
    /// Decisions answered `Zero` through a REGISTERED IDENTITY
    /// ([`Sym::register_equal`], ERROR-DESIGN E12's provenance
    /// reserve) — not a theorem the tier proved but an AXIOM a
    /// constructor stated about what it built, verified at the leaf's
    /// witness and consulted by the early walk. Kept apart from BOTH
    /// theorem counts because it is a different kind of claim: a
    /// symbolic `Zero` rests on exact rational arithmetic alone, a
    /// registered one rests additionally on the registrant's own
    /// argument (`sweep::swept`'s arc carrier carries its proof in its
    /// doc comment). The matching K token is `registered`.
    ///
    /// The attribution is NECESSITY, not contact: a decision counts
    /// here only when the plain form and the early form have BOTH
    /// declined and the same walk with the registry applied answers
    /// (`Session::forms_door`). So the door can only ever move
    /// decisions out of `numeric` — never out of `symbolic_zero` or
    /// `sign_gated`, whose counts are M10-8's on every document.
    pub registered: u64,
    /// **Registrations the door REFUSED** — `Contradicted` or
    /// `Disputed` (the lane scalar's witness separated the two values,
    /// by a proof and by a slack respectively) or `Cyclic`. Counted
    /// because a refusal that leaves no trace is a defect nobody sees:
    /// a constructor registering a lie in a real document must show up
    /// in the receipt (R1 m4, R2 MINOR-2).
    ///
    /// **One column, and what it means depends on the LANE.** At
    /// `Sym<Interval>` — the lane the driver replays in — every
    /// contributing arm is a proof of a defect (`Contradicted`:
    /// disjoint certified enclosures; `Cyclic`), so a non-zero count on
    /// a real document is a finding. At `Sym<f64>` the count also
    /// collects `Disputed`, which may be nothing worse than the
    /// arithmetic running out of significand, so zero is not something
    /// to assert there.
    ///
    /// **This column is a BACKSTOP and not the loud channel, and the
    /// difference is measured.** A registrant that starts stating a
    /// small lie — one the exact witness still ADMITS, because the two
    /// certified enclosures meet — is never refused, so it never
    /// reaches this count; what moves is
    /// [`SymCounts::registered`], which collapses as the registry stops
    /// discharging. A registrant stating a GEOMETRIC lie is caught
    /// earlier still, by its own `debug_assert!` on the exact witness's
    /// refusal, which is live in every profile. What is left for this
    /// column is `Cyclic` and any future registrant that binds an exact
    /// refusal instead of asserting on it. The fixture-scale row asserts
    /// all three together
    /// (`editor-core/tests/m10_9_pins_interval.rs`,
    /// `m10_9_no_registrant_lies_on_any_measured_document`).
    pub registrations_refused: u64,
    /// **Decisions where a REGISTERED zero met a DEFINITE numeric
    /// sign** — the two channels in contradiction, which for a
    /// registered form means the axiom is wrong over this box. The
    /// numeric answer is returned (never the fold), and this column is
    /// how the run says so.
    ///
    /// It is not a K token: the sample the funnel records is the
    /// numeric channel's own `Definite(sign)`, a classified margin
    /// like any other, so the K vocabulary needs nothing new. What is
    /// new is the RECEIPT's statement that a stated identity was
    /// contradicted.
    ///
    /// **It does NOT count [`SymRegistration::Contradicted`]**, despite
    /// the shared word: that arm is the door REFUSING a registration at
    /// the moment it is stated, and it lands in
    /// [`SymCounts::registrations_refused`] with every other refusal.
    /// This column is about a registration the door ACCEPTED, later
    /// contradicted by the numeric channel at a decide site — two
    /// different events, one of which happens after the other could
    /// not.
    pub registrations_contradicted: u64,
    /// **Decisions where the form claimed ZERO under a DEFINITE numeric
    /// sign at an INEXACT witness** — the two channels in contradiction
    /// at a scalar whose value channel is one rounded number
    /// ([`crate::Witness::Inexact`]). Those scalars are exactly `f64`,
    /// [`crate::Probe`], and `Dual<T>` and `Sym<T>` over either of
    /// them; every other lane scalar in the tree is
    /// [`crate::Interval`]-backed and exact. The numeric answer is
    /// returned, the form's answer is not, and this column is how the
    /// run says the two disagreed.
    ///
    /// **BOTH discharge kinds land here**, and the name covers both:
    /// a [`Discharge::Theorem`] is zero unconditionally and a
    /// [`Discharge::SignGated`] zero over the leaf's box on the
    /// strength of one sign read — "a theorem CONDITIONAL on a sign
    /// read", as [`SymCounts::sign_gated`] puts it. Splitting them
    /// would report one suspicion as two kinds of fact: at an inexact
    /// witness rule C's sign read came from the SAME point channel the
    /// margin did, so the gated kind's premise is exactly as suspect as
    /// the plain kind's, and no shipped run can produce a gated one at
    /// all (`SymRules::shipped` has rule C dial-off). The row that
    /// drives one is
    /// `geom-core/tests/sym11_witness_kind_rows.rs`'s
    /// `sym11_a_gated_theorem_disputes_into_the_same_column`.
    ///
    /// **One decision can be charged HERE and in
    /// [`SymCounts::registrations_contradicted`] at once**, by design:
    /// the door is asked first and raises that column when a REGISTERED
    /// zero meets the definite sign, and the walks then run and raise
    /// this one if the form also discharges as a theorem without the
    /// registry. They are two different claims about one decision — an
    /// axiom this box denies, and a theorem this channel cannot see —
    /// and a reader summing the two columns is counting events, not
    /// decisions.
    ///
    /// **It is zero at [`crate::Interval`] by construction, and not by
    /// measurement**: there the witness is EXACT, a definite non-zero
    /// sign is a certified bracket that excludes zero, and the
    /// contradiction is a soundness defect in one of the two channels
    /// — asserted, never counted ([`Decide::sign_within`]). So a
    /// non-zero count here is always an inexact lane's, which is the
    /// unit-test lane and not a shipped one.
    ///
    /// **A refusal column, beside
    /// [`SymCounts::registrations_contradicted`], and NOT a discharge
    /// kind**: no decision lands here instead of in `numeric` — the
    /// decision is counted `numeric` as it always was, and this column
    /// is a second fact about it. It is not a K token either: the
    /// sample the funnel records is the numeric channel's own
    /// `Definite(sign)`, a classified margin like any other, so the K
    /// vocabulary needs nothing new. `sym/discharge_pins.rs` names it
    /// in `NOT_A_DISCHARGE_KIND` with that reason.
    ///
    /// **A dispute names no predicate on the receipt**, which is a
    /// disclosed gap rather than a property: a reader sees that some
    /// decision disputed and cannot see which
    /// (`work/sym/a-dispute-names-no-predicate-on-the-receipt`).
    ///
    /// What a non-zero count means is that the form is a theorem of
    /// the reals which this channel's arithmetic cannot see — a far
    /// placement whose rounding exceeds the band, rule F's
    /// amplification of a one-ulp sign error into a whole `2.0`, or a
    /// pole the point channel has no clause 1 to refuse. It is NOT
    /// evidence against the form: the rows that measure both
    /// mechanisms (`geom-core/tests/sym11_witness_kind_rows.rs`,
    /// `sweep/tests/sym11_far_placement_rows.rs`) drive the same
    /// residuals at `Sym<Interval>`, where every one of them is a
    /// plain theorem.
    pub theorems_disputed: u64,
    /// Decisions handed to the numeric channel.
    pub numeric: u64,
    /// **The nodes this receipt's subject could not resolve to a form**
    /// — frozen into indeterminates by a budget or an overflow, or, on
    /// a leaf, absent from its own table. A SET and not a count of
    /// work: a claim about the subject, like the decision columns
    /// beside it, and the same under every schedule but for one reading
    /// named below. [`memo`]'s header is where the one argument for
    /// that lives; here is what the number means on each receipt that
    /// carries it.
    ///
    /// - **On a LEAF of a drive: that leaf's NEED** — the frozen nodes
    ///   its own reasoning rested on, which is what a reader of a
    ///   refused leaf wants to know first: the drive's frozen set
    ///   inside the closure of what this leaf asked, the ids its own
    ///   table does not hold, and the freezes it could not publish.
    ///   Not the freezes it happened to compute: with the drive's memo
    ///   on, a leaf inherits forms another leaf froze, and counting the
    ///   work would report which leaf got to a node first.
    /// - **On a DRIVE: the distinct nodes frozen over it**
    ///   ([`DriveMemo::frozen`]) — every freeze any leaf published,
    ///   and the set a leaf's NEED is read against. It neither sums
    ///   the leaves' columns nor bounds them: the leaves' sets overlap,
    ///   so [`SymCounts::absorb`] does not add this one up, and a
    ///   leaf's own side carries ids this set never holds — a node its
    ///   table does not hold, or a freeze it could not publish — so a
    ///   leaf's column can EXCEED the drive's (a leaf reading 1 under a
    ///   drive that froze nothing at all is a row: `geom-core`'s
    ///   `sym_drive_memo::a_foreign_id_no_one_freezes_is_still_a_need`).
    ///   The driver writes the drive's own when the drive is done.
    /// - **Outside a drive** ([`with_session`], [`with_session_rules`]):
    ///   the session's own distinct freezes. The leaf is the whole
    ///   drive there, so this is the same quantity.
    /// - **Mid-replay, through [`session_counts`]: the WORK so far** —
    ///   the freezes this leaf has computed, not a NEED. That door
    ///   exists to price a leaf while it runs, where what has been paid
    ///   is the question; a NEED is a property of the finished leaf and
    ///   is written over this count at the leaf's end.
    ///
    /// **One reading is still the schedule's**, and it is inside the
    /// branch [`memo`]'s header has named since the memo landed: a
    /// freeze the TAINT caused, under a hit, is counted where the walk
    /// made it (`work/sym/a-taint-induced-freeze-under-a-hit-still-reads-by-order`,
    /// pinned by `geom-core`'s
    /// `sym_drive_memo::a_taint_induced_freeze_under_a_hit_is_read_by_order`).
    /// No leaf of a drive reaches it — a drive mints every node inside
    /// its own session — and `editor-core`'s
    /// `no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded`
    /// pins that over five drives.
    ///
    /// **The dial does not move it.** With the drive's memo off a leaf
    /// freezes every node of its closure that freezes at all, so its
    /// NEED is what it computed — measured, not only argued, by
    /// `editor-core`'s `the_plain_memo_moves_no_decision`, which
    /// compares the leaf receipts across the dial with no mask.
    pub frozen: u64,
}

impl SymCounts {
    /// The three decision counts added together.
    #[must_use]
    pub fn decisions(&self) -> u64 {
        self.symbolic_zero + self.sign_gated + self.registered + self.numeric
    }

    /// Adds another session's DECISION counts into this one.
    ///
    /// **`frozen` is not summed** — it is a set on both receipts and
    /// the leaves' sets overlap ([`SymCounts::frozen`] argues the
    /// column); the driver writes the drive's own once the drive is
    /// done.
    pub fn absorb(&mut self, other: Self) {
        self.symbolic_zero += other.symbolic_zero;
        self.sign_gated += other.sign_gated;
        self.registered += other.registered;
        self.registrations_refused += other.registrations_refused;
        self.registrations_contradicted += other.registrations_contradicted;
        self.theorems_disputed += other.theorems_disputed;
        self.numeric += other.numeric;
    }
}

/// **The tier's dials** — every mechanism the normal form layers on
/// the plain quotient form, each switchable so that its effect on a
/// document is a measurement rather than an assumption, and so that
/// every dial off ([`Self::none`]) is the plain quotient form bit for
/// bit: the atom-algebra rules A, B and C; the constant fold A0
/// (`const_fold`) and the early walk it runs in (`early`); rules A/B
/// applied PER NODE inside that walk (`early_ab`); rule D, trig of
/// `atan` in closed form with amendment A1's `atan2` and half-π folds
/// (`trig_of_atan`); and the registered-identity door (`registered`).
///
/// Every rule is an equality of reals under clause 1 of the theorem
/// ([`Decide::sign_within`]'s docs), so a zero reached through any of
/// them is still a zero of the real margin; what a rule can cost is
/// only a cancellation it fails to find. The rules are named A, B, C
/// and D where the tier's module docs discuss them; the door is an
/// axiom rather than a rule and is counted apart (`registered`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymRules {
    /// **A — `sqrt(X)² = X`.** An even power of a `sqrt` atom reduces
    /// to the power of its argument form. Pure algebra: sound for every
    /// real `X ≥ 0`, and `X ≥ 0` holds wherever the atom has a real
    /// value, which clause 1 guarantees before the identity test is
    /// ever asked.
    pub sqrt_square: bool,
    /// **B — `sin(θ)² + cos(θ)² = 1`** for atoms of ONE argument form.
    /// An even power of a `sin` atom rewrites to the same power of
    /// `1 − cos²` of the same argument, so any polynomial in the two
    /// that lies in the ideal of the Pythagorean identity reduces to
    /// zero. Unconditional.
    pub pythagoras: bool,
    /// **A0 — the exact constant fold**: `sqrt(c)` and `abs(c)` of a
    /// CONSTANT form whose value is a perfect-square rational (`sqrt`)
    /// or any rational (`abs`) fold to the exact rational. No value is
    /// read — the argument is a literal of the form itself — and the
    /// fold is sound (a constant atom replaced by the constant it
    /// denotes). It is what relieves the arc family's freezes: the
    /// blocking residuals were products of `sqrt(1)^58` and `sqrt` of
    /// exact-square dyadic constants ([`Self::shipped`]).
    ///
    /// WHERE it runs is the one subtlety: with [`Self::early`] on it
    /// runs in the early walk ALONGSIDE the plain form, which is how it
    /// ships; without, it REPLACES the plain form's constant atoms.
    /// Replacing is cheaper and was measured to LOSE theorems — the
    /// folded constants' products cross the coefficient bound
    /// ([`rational::COEFF_BITS`]) where the opaque atoms' did not, and a frozen
    /// form is opaque: two R2-bracket rows at the nominal, and M10-6's
    /// min-clearance boxes refusing whole. Alongside, the plain form
    /// is M10-7's exactly and the fold can only add.
    pub const_fold: bool,
    /// **The EARLY walk**: a SECOND memo built ALONGSIDE the plain
    /// form — never replacing it — in which rule C's fold runs at each
    /// `sqrt`/`abs` node (and, with [`Self::early_ab`], rules A/B run
    /// per node too). A decision is asked of the plain form first, so a
    /// plain theorem is never re-labelled; the early form can only ADD
    /// a discharge. Rule C rides this walk exclusively, because the
    /// atoms it folds sit nested inside other atoms' arguments, out of
    /// a top-residual reduction's reach. Its cost is a second walk of
    /// the DAG per decision the plain form did not answer, memoized per
    /// leaf.
    pub early: bool,
    /// **Rules A/B PER NODE in the early walk**, under a step cap
    /// ([`EARLY_STEPS`]) and a size cap ([`EARLY_AB_TERMS`]), with the
    /// un-reduced form kept where a reduction does not fit. This is
    /// how a nested atom is reached — `sqrt(…)²` inside another atom's
    /// argument, which a reduction over the top residual never sees —
    /// and it is what closes the ring behind rule D. Needs `early`, and
    /// applies whichever of `sqrt_square`/`pythagoras` is on.
    ///
    /// Affordable by construction rather than by luck: the substitution
    /// is LINEAR in the form (`algebra::poly_subst_square` accumulates
    /// one numerator over one common denominator instead of
    /// cross-multiplying term by term, which is what made the first
    /// cut cost 138 s per nominal plate replay), memoized per node by
    /// content-hash id like every form, and skipped outright on a form
    /// past the size cap, where a reduction could only feed a product
    /// the budget is about to freeze anyway.
    pub early_ab: bool,
    /// **D — trig of `atan`, exact**: a `sin`/`cos` node in the early
    /// walk whose argument form is `q · atan(X)`, `q` dyadic, rewrites
    /// to its closed form in `X` and the atom `sqrt(1 + X²)` — integer
    /// multiples by angle addition, halves on the positive branch,
    /// which the RANGE of `atan` fixes ([`trig`] carries the argument).
    /// Unconditional: no value is read, and a zero reached through it
    /// is a theorem. Nothing folds at any other argument shape. Needs
    /// `early`; the `sqrt` atoms it mints are rule A's shape, so it
    /// pays off with `early_ab`.
    pub trig_of_atan: bool,
    /// **C — `sqrt(X) = R` where `X = R²` as forms and `R` has a
    /// certified sign over the leaf's box** (and `abs(R) = ±R`
    /// likewise): clause 3 of the theorem, the one rule that reads a
    /// value. [`signed`] is the whole of how the value is read — the
    /// parameter brackets the analysis box already holds, enclosed in
    /// the ring — and why a zero reached through it is counted
    /// `sign_gated` rather than `symbolic_zero`. Needs `early`.
    pub signed_root: bool,
    /// **E — the quotient's COMMON FACTOR** ([`quotient`]): in the
    /// early walk every form has the monomial its numerator and
    /// denominator share divided out, and a numerator that is a
    /// rational multiple of its denominator folds to that rational.
    /// Both are equalities of rational functions wherever the
    /// denominator is non-zero, which clause 1 guarantees — a point
    /// where a form's denominator vanishes is one the value channel
    /// divided by zero at, and the whole-box certification has already
    /// refused there.
    ///
    /// It is what a NORMALISATION needs. `Vec3::normalize` is
    /// `self / self.norm()`, so a unit vector reaches the DAG as three
    /// quotients over one `sqrt(v·v)` atom and everything built from it
    /// carries that atom in both halves; the plain form cancels no
    /// common factor, so each further normalisation multiplies the
    /// shared power and each square doubles it. On a derived frame
    /// whose axes carry a parameter the forms reach total degree 128 in
    /// a handful of terms and freeze — and the already-unit vector's
    /// own norm is `sqrt(P/P)`, the literal number one carried as an
    /// opaque atom because neither half of `P/P` is a constant for A0
    /// to read.
    ///
    /// No step cap beside it: unlike rules A/B the fold cannot
    /// reintroduce anything, it is one pass over the terms, and every
    /// form it returns has at most the terms and at most the degree of
    /// the one it was given ([`quotient`]'s docs carry the argument).
    /// Needs `early`.
    pub common_factor: bool,
    /// **F — the MANIFEST SIGN** ([`manifest`]): in the early walk a
    /// `copysign(Y, X)` node becomes `abs(Y)` and an `abs(X)` node
    /// becomes `X` wherever the FORM of `X` is manifestly POSITIVE —
    /// a positive numerator over a non-negative denominator, with
    /// `sqrt`/`abs` atoms of manifestly positive arguments the only
    /// indeterminates a positive term may carry — and `−abs(Y)`, `−X`
    /// wherever the form is manifestly NEGATIVE (the same predicate of
    /// the negated numerator). All four are equalities of reals at
    /// every point clause 1 admits and none reads a value, so a zero
    /// reached through this rule is a THEOREM. One dial for both arms:
    /// they are one predicate read on `N` and on `−N`, and the census
    /// tells them apart by the argument's leading sign rather than by
    /// a second bit (a differential on a document reaches whichever
    /// arm that document's atoms have — the tilt-`u` END cap the
    /// positive one, its START cap the negative one — so a per-arm
    /// bit would separate nothing the documents do not already).
    ///
    /// It is rule C's shape without rule C's value read: where C folds
    /// `abs(R)` on a bracket of `R` the session holds, this folds it on
    /// a fact about the form, and a discharge through it is counted
    /// `symbolic_zero` rather than `sign_gated`.
    ///
    /// **Strict positivity, not non-negativity**, and the reason is
    /// `copysign`: it reads a SIGN BIT, so `copysign(1, −0.0) = −1`
    /// while `copysign(1, +0.0) = +1`, and at a real zero of `X` the
    /// node denotes no function of the real value of `X` at all. The
    /// predicate excludes that point. [`manifest`]'s header carries
    /// the argument and the shapes it must not fold. Needs `early`.
    pub manifest_sign: bool,
    /// **The REGISTERED-IDENTITY DOOR** (M10-9, ERROR-DESIGN E12's
    /// provenance reserve): the early walk consults the session's
    /// registry ([`Sym::register_equal`]), so a node a constructor
    /// registered against another takes that other node's form and the
    /// residual between them is the zero form.
    ///
    /// Not a rewrite RULE like the rest of this struct — the others are
    /// algebra the tier performs, this one is an axiom a constructor
    /// states — and it is a dial for exactly the reason they are: with
    /// it off the tier is M10-8's, bit for bit, so what the door buys a
    /// document is a measurement. Needs `early`: the registry is
    /// consulted in the early memo only, never in the plain one, so a
    /// theorem the plain form reaches is never re-labelled as an axiom.
    pub registered: bool,
}

impl SymRules {
    /// Every rule on — the full set, for measuring what each can reach.
    #[must_use]
    pub const fn all() -> Self {
        Self {
            sqrt_square: true,
            pythagoras: true,
            const_fold: true,
            early: true,
            early_ab: true,
            trig_of_atan: true,
            signed_root: true,
            common_factor: true,
            manifest_sign: true,
            registered: true,
        }
    }

    /// **The shipped set: the constant fold, the registered-identity
    /// door, and the form-level algebra — rule D with rules A/B per
    /// node — in the early walk ALONGSIDE the plain form**, over the
    /// bounded arbitrary-precision coefficient ring, with M10-7's plain
    /// form asked first and kept whole.
    ///
    /// Chosen by measurement, per mechanism, on the two-hole plate,
    /// R2's filleted bracket, R1's annulus, R2's rounded pad and R2's
    /// link (the module docs carry the tables):
    ///
    /// | mechanism | ceilings moved | cost per leaf | ships |
    /// | --- | --- | --- | --- |
    /// | A0 alongside (`const_fold` + `early`) | bracket 10.4×, annulus 39×, the shaft's ±0.1 study certifies whole; loses nothing | plate 0.35 → 0.65 s, bracket 1.47 → 2.7 s | **yes** |
    /// | A0 replacing (`const_fold` alone) | the same ceilings | plate 0.37 s, bracket 1.46 s | no: loses theorems to bound freezes |
    /// | the door (`registered`) | none alone; the `i = 0` sample of the scaffold residual and both endpoint pinnings | ~0 | **yes** |
    /// | D + A/B per node (`trig_of_atan`, `early_ab`, `sqrt_square`, `pythagoras`), with A1's `atan2` and half-π folds under D's dial | the plate's four identity residuals all go: the plate certifies 0.24–0.26 and the annulus 0.70–0.84 of their REAL studies, their ceilings bounded by dependency widening of real margins; pad 1.20× | plate 0.15 s at its real study, pad 2.1 → 10.5 s, link 0.43 → 4.1 s (with rule D's `sin`/`cos` pair built once) | **yes** |
    /// | A/B over the top residual (`sqrt_square`/`pythagoras` at `discharge`'s site, once the walks have declined) | none, alone or with rule D: the plate's nominal split is M10-9's under it alone and rule D's with D (`CAD_M10_10_RULES=top_only`, `d_top_only`); M10-8 measured it inert and it still is | +18% on the plate's `1e2·ε` leaf (0.131 → 0.154 s with rule D), +12% on the link (0.76 → 0.85 s) | ships only because it shares the per-node walk's dials — disclosed as M10-10's D17, not chosen |
    /// | C in the early walk (`signed_root`) | none; folds on no document at 256 bits | ~2× | no (inert; reads a value) |
    /// | E, the quotient's common factor (`common_factor`, SYM-5) | none on the five; R1's boss at bulge 2 `8.2611e2 → 9.3559e2 · ε` (1.13×), and a derived frame whose AXES carry a parameter certifies where its authored twin does, which no dial reached before | one whole-box leaf, release: plate 0.13 → 0.36 s, annulus 0.12 → 0.29, bracket 0.44 → 1.70, link 3.31 → 2.43, pad 3.85 → 14.40 | **yes**, with the bracket, the pad and the link over the 1.6 s line disclosed |
    /// | F, the manifest sign (`manifest_sign`, SYM-8) | none, on all EIGHT measured documents, to the digit; the tilt-`u` derived frame's `carrier_endpoint_end` 24/0/0/1 → 33/0/0/0 and its `Pinned` replay 122 decisions out of `numeric` at a sixth of the cost | free to the measurement's noise and cheaper on most — the six leaf numbers live once, in the module header's rule-F section | **yes**, with the pad's four `symbolic_zero` → `registered` ratified as a spec deviation |
    /// | F's NEGATIVE arm (the same dial, SYM-12) | none, on all EIGHT measured documents, to the digit, splits and ceilings both, and the walk ledger unmoved; the tilt-`u` cube's START cap and its `FlipZ` twin read the end cap's rule-F-on state by name and by count (`carrier_endpoint_end` 33/0/0/0, the refusal moved to the Newell straddle; `Pinned` the start cap 108 decisions out of `numeric` at 12.2 → 1.0 s, `FlipZ` 122 at 7.8 → 1.2 s) | the arm fires on none of the eight (no `copysign` atom reaches a decision there, and no `abs` atom's argument is manifestly signed), so what it costs there is one coefficient-sign scan per `abs`/`copysign` node the positive arm declined, and a negation plus the predicate only on a numerator whose every coefficient is negative; the release leaf instrument's reading is in the header's cost paragraph below the rule-F section, the one place those numbers live | **yes** |
    ///
    /// The pins in `m10_8_pins_interval.rs`, `m10_9_pins_interval.rs`
    /// and `m10_10_pins_interval.rs` hold each layer to what it
    /// measured, the earlier ones under [`Self::without_the_algebra`].
    #[must_use]
    pub const fn shipped() -> Self {
        Self {
            sqrt_square: true,
            pythagoras: true,
            const_fold: true,
            early: true,
            early_ab: true,
            trig_of_atan: true,
            signed_root: false,
            common_factor: true,
            manifest_sign: true,
            registered: true,
        }
    }

    /// Every rule off: the quotient normal form with every atom opaque,
    /// which is the tier exactly as it stood before the atom algebra.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            sqrt_square: false,
            pythagoras: false,
            const_fold: false,
            early: false,
            early_ab: false,
            trig_of_atan: false,
            signed_root: false,
            common_factor: false,
            manifest_sign: false,
            registered: false,
        }
    }

    /// **The shipped set with the form-level algebra OFF** — rules A/B
    /// per node, rule D, rule E and rule F shut, the constant fold, the
    /// early walk and the registered-identity door as they were: the
    /// tier exactly as M10-9 shipped it, bit for bit, and the
    /// differential every claim about what the algebra costs and what
    /// it buys is measured against.
    ///
    /// **SIX dials, and each was added the day its rule shipped.** A
    /// rule that rewrites a form in the EARLY walk is form-level
    /// algebra whatever its argument reads, so rule E (SYM-5) and rule
    /// F (SYM-8) belong here beside A/B and D; leaving one out makes
    /// this constructor a differential against a tier that never
    /// existed, silently, while its own doc still claims M10-9's.
    #[must_use]
    pub const fn without_the_algebra() -> Self {
        Self {
            sqrt_square: false,
            pythagoras: false,
            early_ab: false,
            trig_of_atan: false,
            common_factor: false,
            manifest_sign: false,
            ..Self::shipped()
        }
    }

    /// **The shipped set with the registered-identity door SHUT, and
    /// nothing else** — the differential every claim about what the
    /// DOOR buys is measured against ([`Self::registered`]), and the
    /// contract `m10_9_pins_interval`'s census asserts.
    ///
    /// **It is NOT M10-8's tier, and said so for three units before
    /// anyone checked.** M10-8's tier is A0 alone beside the door shut
    /// — `registered: false, ..without_the_algebra()`, which is what
    /// `m10_8_pins_interval`'s `a0_alone` builds. This constructor has
    /// carried rules A/B per node and rule D since M10-10 and rule E
    /// since SYM-5, so the old "M10-8's tier exactly, bit for bit" was
    /// already false when rule F arrived; SYM-8's reviews caught the
    /// sentence and read the whole of it onto rule F. The sentence is
    /// the defect and is retired here. Rule F stays ON, because a
    /// door differential that also shut a fold rule would measure two
    /// things at once — [`Self::without_the_algebra`] and
    /// [`Self::without_rule_e`], which DO name earlier tiers, shut it.
    #[must_use]
    pub const fn shipped_without_the_door() -> Self {
        Self {
            registered: false,
            ..Self::shipped()
        }
    }
    /// **The shipped set with rule E SHUT** — the quotient's common
    /// factor left uncancelled: M10-10's tier exactly, bit for bit, and
    /// the differential every claim about what rule E costs and what it
    /// buys is measured against ([`Self::common_factor`]). Rule F is
    /// shut with it, because M10-10's tier had none; [`Self::without_rule_f`]
    /// is the other half of the pair and keeps rule E on.
    #[must_use]
    pub const fn without_rule_e() -> Self {
        Self {
            common_factor: false,
            manifest_sign: false,
            ..Self::shipped()
        }
    }

    /// **The shipped set with rule F SHUT** — the `copysign` and `abs`
    /// atoms of a manifestly SIGNED argument (either arm) left opaque,
    /// every other rule as it is: SYM-5's tier exactly, bit for bit,
    /// because neither arm existed there, and the differential every
    /// claim about what rule F costs and what it buys is measured
    /// against ([`Self::manifest_sign`]).
    #[must_use]
    pub const fn without_rule_f() -> Self {
        Self {
            manifest_sign: false,
            ..Self::shipped()
        }
    }
}

impl Default for SymRules {
    fn default() -> Self {
        Self::shipped()
    }
}

/// A hasher for keys that ARE hashes: it takes the low 64 bits verbatim.
/// Deterministic and allocation-free; the map is never iterated, so no
/// ordering claim rides on it.
#[derive(Default)]
struct IdHasher(u64);

impl core::hash::Hasher for IdHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 = self.0.rotate_left(8) ^ u64::from(*b);
        }
    }

    fn write_u128(&mut self, i: u128) {
        self.0 = i as u64;
    }
}

type IdMap<V> = HashMap<SymId, V, core::hash::BuildHasherDefault<IdHasher>>;

/// A map keyed by an INDETERMINATE id (a 128-bit digest, so the same
/// verbatim hasher serves).
type IndetMap<V> = HashMap<u128, V, core::hash::BuildHasherDefault<IdHasher>>;

/// A SET of node ids, spelled once so that two of them are the same
/// type ([`DriveMemo`]'s frozen set is the one in the tier today).
///
/// `profile`'s own id sets stay `BTreeSet<u128>`: they are PUBLIC
/// fields of a public struct, and this alias is built on the private
/// [`IdHasher`], which a public interface may not name.
type IdSet = IdMap<()>;

/// What one opaque atom is: its op and the forms of its arguments —
/// what rule A needs (`sqrt`'s argument), what rule B needs (a `sin`'s
/// argument digest names its `cos` twin), and what the shape report
/// renders.
#[derive(Clone)]
struct AtomInfo {
    op: SymOp,
    /// R2's experiment: the node payload the atom's id was keyed with,
    /// without which a rule that rewrites an atom's ARGUMENT cannot
    /// re-mint the atom's id.
    payload: u64,
    args: [Option<Arc<Form>>; 2],
}

/// One leaf replay's DAG: the hash-consing table, the memoized forms and
/// the counts. Dropped with the leaf.
///
/// Everything here is this leaf's own, with ONE exception: `memo`, a
/// handle on the drive's shared plain forms when a driver installed one
/// ([`with_session_memo`]). The hash-consing table, the early and door
/// memos, the registry, the parameter brackets and the counts never
/// leave the leaf.
struct Session {
    budget: SymBudget,
    rules: SymRules,
    nodes: IdMap<SymNode>,
    /// The PLAIN quotient forms — every atom opaque, rule A0 only (the
    /// constant fold, which cannot cost a cancellation). Rules A/B are
    /// applied afterwards over the top residual ([`algebra::reduce`])
    /// and per node in `forms_early`; nothing ruled is memoized here.
    forms: IdMap<Arc<Form>>,
    /// The EARLY-reduced forms (`SymRules::early`), a second memo
    /// beside the plain one.
    forms_early: IdMap<Arc<Form>>,
    /// **The DOOR-reduced forms** ([`SymRules::registered`]): the early
    /// walk again, this time with the session's registry applied — a
    /// THIRD memo, beside the plain one and the early one, and the
    /// reason the receipt can say which decisions actually NEEDED the
    /// door. A zero found here that the early memo did not find is one
    /// the registration was necessary for; asking the two in order is
    /// what makes `registered` an honest column instead of "every form
    /// that happened to touch a registered node". Measured, that
    /// distinction is most of the column: on R2's pad at its nominal
    /// the sticky-flag attribution moved 40 of 368 theorems into it
    /// that A0 had already proved.
    ///
    /// Built lazily like the others, and never at all while the
    /// registry is empty — so a document with no registrants (all of
    /// straight geometry) pays nothing and serializes M10-8's bytes.
    forms_door: IdMap<Arc<Form>>,
    /// The `f64` bracket of each document parameter this leaf was
    /// evaluated over, by the parameter's indeterminate id — recorded
    /// by [`Sym::param_over`], read only by rule C ([`signed`]).
    params: IndetMap<(f64, f64)>,
    /// Every opaque atom minted so far, by its indeterminate id.
    atoms: IndetMap<AtomInfo>,
    /// **The registered-identity registry** ([`Sym::register_equal`]):
    /// `node -> the node it was registered equal to`, resolved
    /// transitively by [`Session::alias`]. Keyed by content hashes and
    /// per leaf replay, like everything else here; consulted by the
    /// EARLY walk only, and only under [`SymRules::registered`].
    registry: IdMap<SymId>,
    /// **Rule D's closed forms, by argument form** (`trig::Closed`):
    /// `sin` and `cos` of one argument are two nodes, and the recurrence
    /// that builds `(cos kψ, sin kψ)` over the shared denominator yields
    /// both at once — so a `sin_cos` pair minted from one node folded
    /// TWICE before this memo (R2's Q7). Keyed by the argument form's
    /// digest, per session like every other memo here.
    trig_closed: IndetMap<Option<Rc<trig::Closed>>>,
    counts: SymCounts,
    /// **The drive's shared plain memo** ([`DriveMemo`]), when a drive
    /// installed one ([`with_session_memo`]). The plain walk consults
    /// it on a miss in `forms` and the leaf publishes to it once, at its
    /// end; every other memo here is this leaf's alone.
    memo: Option<Arc<DriveMemo>>,
    /// **What this leaf owes the drive memo**, accumulated as the plain
    /// walk computes and handed over in ONE write lock at the leaf's end
    /// (`DriveMemo`'s header says why one and not one per node): the
    /// ids it built a plain form for, the atoms that walk minted, and
    /// the ids it froze. Empty when no drive installed a memo.
    plain_built: Vec<SymId>,
    plain_atoms: Vec<u128>,
    plain_frozen: Vec<SymId>,
    /// **The roots the PLAIN walk was asked for** — the leaf's NEED is
    /// counted over their closure ([`leaf_need`]). A SET and not a
    /// list: one decision asks for one root many times over a leaf
    /// (1,413 asks over ~120 distinct roots on the plate), and the
    /// closure walk wants each once. Kept only while a drive memo is
    /// installed, where the column is a NEED; empty otherwise.
    plain_roots: IdSet,
    /// **The ids this leaf's table did not hold when a node named them
    /// as a child** — a node minted before the session was installed,
    /// or outside one.
    ///
    /// A CANDIDATE set, reconciled against the table at the leaf's end
    /// ([`leaf_need`]): a node named before the session minted it and
    /// minted inside it afterwards is one the leaf holds by the time it
    /// reasons, and it needs nothing for it.
    ///
    /// The plain walk freezes such a node when it reaches it, and that
    /// freeze never leaves the leaf (`memo`'s unrecorded paragraph) —
    /// but whether the walk REACHES it is the drive memo's business: a
    /// hit at a recorded ancestor skips the subtree under it. So the
    /// leaf's own side of its NEED is counted from HERE, which its box
    /// decides, and not from the freezes it happened to make, which the
    /// schedule can take away ([`leaf_need`]). Collected at [`intern`],
    /// once per distinct node, and only while a drive memo is
    /// installed.
    ///
    /// **It is the one part of the column that is not free**, because
    /// it is paid on the DAG build rather than at the leaf's end: two
    /// table lookups per node minted under a memo. Measured in release
    /// against the same binary with the collection skipped, over
    /// 48-leaf drives, it is **+34 ms on the M10-3 slab (458 → 492 ms,
    /// ~7 %, 95 sessions of 18,833 nodes each)** and **+0.45 s on the
    /// two-hole plate (35.6 → 36.0 s, ~1.3 %)** — 0.4 ms on a slab leaf
    /// against the tier's own affordability line of 1.6 s a leaf. A
    /// session with no drive memo pays nothing.
    foreign: IdSet,
    /// **The freezes this leaf made and could NOT publish**: a node it
    /// never RECORDED, or one whose form it built out of such a node
    /// (`plain_tainted`). They are this leaf's own answer and no other
    /// leaf may take them (`memo`'s unrecorded paragraph), so they are
    /// in no drive's frozen set — and they are still what this leaf's
    /// reasoning rested on, which is why [`leaf_need`] counts them
    /// beside the set. Kept only while a drive memo is installed.
    plain_unpublished: IdSet,
    /// **The nodes whose plain form this leaf built out of an
    /// UNRECORDED one**, and which it therefore may not publish.
    ///
    /// A node absent from `nodes` is frozen into its own indeterminate,
    /// and that is this LEAF's answer, not the node's: a leaf that
    /// recorded it builds a real form. The freeze does not stay put —
    /// the recorded parent above it combines the indeterminate into its
    /// own form, under an id that is a content hash of the CHILDREN'S
    /// IDS and so is the same id the recording leaf uses. So the taint
    /// propagates up the walk and the publication guard follows it,
    /// rather than stopping at the unrecorded node itself. Kept only
    /// while a drive memo is installed; empty otherwise.
    plain_tainted: IdSet,
}

impl Session {
    /// **The one place a session is built** — every field of it, in one
    /// literal, so a field added here cannot leave a second literal
    /// somewhere else half-initialised (the `trig` test module kept one,
    /// and it is this now).
    fn new(budget: SymBudget, rules: SymRules, memo: Option<Arc<DriveMemo>>) -> Self {
        Self {
            budget,
            rules,
            nodes: IdMap::default(),
            forms: IdMap::default(),
            forms_early: IdMap::default(),
            forms_door: IdMap::default(),
            params: IndetMap::default(),
            atoms: IndetMap::default(),
            registry: IdMap::default(),
            trig_closed: IndetMap::default(),
            counts: SymCounts::default(),
            memo,
            plain_built: Vec::new(),
            plain_atoms: Vec::new(),
            plain_frozen: Vec::new(),
            plain_roots: IdSet::default(),
            foreign: IdSet::default(),
            plain_unpublished: IdSet::default(),
            plain_tainted: IdSet::default(),
        }
    }

    /// The node `id` denotes, following the registry to its end — `id`
    /// itself when nothing was registered for it.
    ///
    /// The chain is finite because [`Sym::register_equal`] refuses a
    /// registration that would close a cycle
    /// ([`SymRegistration::Cyclic`]); the cap below is belt to that
    /// braces, so a registry corrupted by a future edit degrades into a
    /// missed cancellation rather than a hang.
    fn alias(&self, id: SymId) -> SymId {
        let mut cur = id;
        for _ in 0..ALIAS_DEPTH {
            match self.registry.get(&cur) {
                Some(next) => cur = *next,
                None => return cur,
            }
        }
        cur
    }

    /// Whether `target` occurs in the expression `from` denotes, with
    /// the registry already applied — the cycle test
    /// [`Sym::register_equal`] runs before it records anything.
    ///
    /// Not [`Session::closure`] with a membership test after it, though
    /// the two walk the same DAG: this one follows the registry ALIAS
    /// (a cycle can close through a registration, which is the whole
    /// question here) and stops at the first hit, where the closure is
    /// unaliased and always complete. Sharing a walk between them would
    /// mean a visitor and a flag that changes what the walk means.
    fn reaches(&self, from: SymId, target: SymId) -> bool {
        let mut seen: IdMap<()> = IdMap::default();
        let mut stack = vec![from];
        while let Some(id) = stack.pop() {
            let id = self.alias(id);
            if id == target {
                return true;
            }
            if seen.insert(id, ()).is_some() {
                continue;
            }
            if let Some(node) = self.nodes.get(&id) {
                stack.extend(node.kids().iter().copied());
            }
        }
        false
    }

    /// **Every node under `roots` in this leaf's own DAG**, the roots
    /// included — what the plain walk would visit from them with no
    /// memo installed, which is why [`leaf_need`] counts over it.
    ///
    /// **It is MATERIALISED**, and the set is the walk's own seen-set
    /// rather than an extra: one `SymId` per distinct node under the
    /// roots, which on the two-hole plate is 17,624 ids — about half a
    /// megabyte with the map's slack — allocated at a leaf's end and
    /// freed with the leaf. The alternative is to re-walk the DAG once
    /// per frozen id, which is the same allocation in time.
    ///
    /// A node absent from the table has no children HERE, which is
    /// exactly what the plain walk does with an unrecorded one: it
    /// freezes it and stops.
    fn closure(&self, roots: impl Iterator<Item = SymId>) -> IdSet {
        let mut seen: IdSet = IdSet::default();
        let mut stack: Vec<SymId> = roots.collect();
        while let Some(id) = stack.pop() {
            if seen.insert(id, ()).is_some() {
                continue;
            }
            if let Some(node) = self.nodes.get(&id) {
                stack.extend(node.kids().iter().copied());
            }
        }
        seen
    }
}

/// How far [`Session::alias`] follows the registry before it gives up.
/// A registration chain is at most as long as the registrations one
/// leaf makes, which is a handful per arc.
const ALIAS_DEPTH: usize = 64;

/// The door's typed answer lives with the trait method that returns it
/// ([`Real::register_equal`]) rather than here: `real` is `sym`'s
/// SUBSTRATE, and a substrate that imports a type from its consumer to
/// name its own return value is a layering inversion (R1 m5 /
/// R2 MINOR-3). Re-exported so `geom_core::sym::SymRegistration` keeps
/// naming the same type.
pub use crate::real::SymRegistration;

thread_local! {
    /// The installed session, if any (module docs: no session, no tier).
    static SESSION: RefCell<Option<Session>> = const { RefCell::new(None) };

    /// **How many opaque values this leaf replay has minted** — the
    /// payload [`Sym::opaque`] stamps into its node, so that each
    /// untracked real is its own indeterminate.
    ///
    /// D9, argued here rather than assumed. A sequence number is only
    /// deterministic if the order that advances it is; this one is
    /// advanced by the ORDER A LEAF MINTS ITS OPAQUE VALUES, which is
    /// the order the evaluation service walks that leaf's recipe — a
    /// fixed, single-threaded walk per leaf, the same one whose node
    /// ids D9 already rests on. [`with_session`] resets it around every
    /// replay, so a leaf's sequence starts at 0 no matter which leaf
    /// ran before it or on which rayon worker, and two replays of the
    /// same leaf mint the same ids. The counter never crosses a leaf
    /// boundary, which is the property that makes it safe: it is
    /// per-replay state living beside a per-replay table.
    static OPAQUE_SEQ: Cell<u64> = const { Cell::new(0) };
}

/// Restores [`OPAQUE_SEQ`] when a [`with_session`] call leaves, by any
/// path including a panic — the counter is per-replay state, so a
/// session that unwound without restoring it would hand the next leaf a
/// sequence starting mid-count and break the D9 claim on `opaque`'s ids.
struct OpaqueSeqGuard(u64);

impl Drop for OpaqueSeqGuard {
    fn drop(&mut self) {
        OPAQUE_SEQ.set(self.0);
    }
}

/// Runs `f` with a fresh symbolic session installed on this thread,
/// answering its result beside the session's counts.
///
/// The table is per-call and dropped at the end of it, which is what
/// "one hash-consing table per leaf replay" means: a leaf's nodes never
/// reach another leaf, and the counts are that leaf's own. Nesting is
/// refused rather than silently flattened — an inner session would count
/// a different leaf's decisions into the outer one's receipt.
///
/// This door installs NO drive memo, so the session it makes holds
/// nothing across leaves at all; [`with_session_memo`] is the spelling
/// a driver uses to share the plain forms across the leaves of one
/// drive.
pub fn with_session<R>(budget: SymBudget, f: impl FnOnce() -> R) -> (R, SymCounts) {
    with_session_rules(budget, SymRules::shipped(), f)
}

/// [`with_session`] with the atom-algebra dials chosen ([`SymRules`]);
/// `with_session` is this at [`SymRules::shipped`] — ONE default, so
/// every legacy caller runs the shipped tier and nothing else.
pub fn with_session_rules<R>(
    budget: SymBudget,
    rules: SymRules,
    f: impl FnOnce() -> R,
) -> (R, SymCounts) {
    with_session_in(budget, rules, None, f)
}

/// [`with_session_rules`] with a DRIVE-scoped plain memo installed
/// ([`DriveMemo`]): the leaf's plain walk consults `memo` on a miss in
/// its own table and publishes what it computed to `memo` at its end.
///
/// Everything else about the session is unchanged — the hash-consing
/// table, the early and door memos, the registry and the parameter
/// brackets are this leaf's and are dropped with it. The decision
/// counts the call answers are this leaf's own; its
/// [`SymCounts::frozen`] is this leaf's NEED, counted at the leaf's end
/// against the drive's frozen set, and the drive's own column is
/// [`DriveMemo::frozen`].
///
/// **The memo is valid for one `(budget, rules)` pair** and refuses a
/// leaf that does not match it: a plain form is a function of the node
/// id and those two, so serving one across a budget change would hand
/// back a form the leaf would not have built.
///
/// The mismatch is a `debug_assert!`, which is loud in every profile
/// this workspace builds — `[profile.release]` keeps debug assertions
/// on. A build that turned them OFF would run the leaf with NO memo
/// instead: sound (it is the pre-memo tier) but quiet, and its freezes
/// would never be published, so the drive's `frozen` column would
/// under-count in exactly that build. No configuration in this repo
/// reaches it.
pub fn with_session_memo<R>(
    budget: SymBudget,
    rules: SymRules,
    memo: &Arc<DriveMemo>,
    f: impl FnOnce() -> R,
) -> (R, SymCounts) {
    let accepts = memo.accepts(budget, rules);
    debug_assert!(
        accepts,
        "a drive memo is valid for the budget and rules it was made for"
    );
    with_session_in(budget, rules, accepts.then(|| Arc::clone(memo)), f)
}

fn with_session_in<R>(
    budget: SymBudget,
    rules: SymRules,
    memo: Option<Arc<DriveMemo>>,
    f: impl FnOnce() -> R,
) -> (R, SymCounts) {
    let nested = SESSION.with(|s| s.borrow().is_some());
    // The opaque sequence is per-replay state, restored on the way out
    // so a nested or sequential call cannot inherit a partial count
    // (`OPAQUE_SEQ`'s docs carry the D9 argument).
    let _restore = OpaqueSeqGuard(OPAQUE_SEQ.replace(0));
    // A nested call in a release build runs with the OUTER session,
    // which is sound — ids are content hashes and the table is keyed by
    // them — and only muddles whose receipt the decisions land in.
    debug_assert!(!nested, "symbolic sessions do not nest");
    if nested {
        return (f(), SymCounts::default());
    }
    SESSION.with(|s| {
        *s.borrow_mut() = Some(Session::new(budget, rules, memo));
    });
    #[cfg(feature = "sym-profile-testing")]
    profile::session_start();
    let out = f();
    let mut sess = SESSION.with(|s| s.borrow_mut().take());
    #[cfg(feature = "sym-profile-testing")]
    if let Some(s) = &sess {
        profile::session_done(s.nodes.len(), s.atoms.len());
    }
    if let Some(s) = &mut sess {
        publish_to_memo(s);
        // **The leaf's `frozen` column is its NEED**, and this is where
        // it is written — after the leaf has published, so that the
        // freezes it paid for itself are in the set it is counted
        // against ([`SymCounts::frozen`] argues the column).
        if let Some(memo) = &s.memo {
            let need = leaf_need(&*s, memo);
            s.counts.frozen = need;
        }
    }
    let counts = sess.map_or_else(SymCounts::default, |s| s.counts);
    (out, counts)
}

/// Hands the leaf's plain walk to the drive memo under ONE write lock
/// (`DriveMemo`'s header says why one per leaf and not one per node).
///
/// A node the leaf took FROM the memo is not in `plain_built`, so what
/// is offered here is what this leaf computed; the memo keeps whichever
/// copy arrived first, and they are the same form.
fn publish_to_memo(sess: &Session) {
    let Some(memo) = &sess.memo else { return };
    // An id on a publication list with no entry in this leaf's own map
    // is a BUG in whoever put it there, not a case to pass over: every
    // push sits beside the insert that makes it findable. Loud in
    // debug; in release the entry simply does not reach the memo, which
    // costs a hit and cannot cost a decision.
    let found = |ok: bool, what: &str| {
        debug_assert!(
            ok,
            "the drive memo's publication list names a {what} this leaf never recorded"
        );
        ok
    };
    memo.publish(
        sess.plain_built.iter().filter_map(|id| {
            let f = sess.forms.get(id);
            found(f.is_some(), "plain form");
            f.map(|f| (*id, Arc::clone(f)))
        }),
        sess.plain_atoms.iter().filter_map(|id| {
            let a = sess.atoms.get(id);
            found(a.is_some(), "plain-walk atom");
            a.map(|a| (*id, a.clone()))
        }),
        sess.plain_frozen.iter().copied(),
    );
}

/// **The leaf's `frozen` column**: the distinct nodes its reasoning
/// rested on as indeterminates — inside the closure of this leaf's own
/// plain-walk roots, the drive's frozen set, the ids this leaf's table
/// does not hold, and the freezes it made and could not publish.
/// `memo`'s header argues why that is the same number under every
/// schedule and names the one reading that is not, and
/// [`SymCounts::frozen`] says what the column means.
///
/// The three sets are UNIONED and not summed: an id the leaf's table
/// does not hold is one the leaf also froze if its walk got there, and
/// a leaf that RECORDED the same id may have published a freeze of it,
/// so a node can be in all three and is needed once.
fn leaf_need(sess: &Session, memo: &DriveMemo) -> u64 {
    // Nothing froze anywhere — not over the drive, not in this leaf's
    // own unpublishable corner — so the answer is 0 without walking
    // anything. A document that freezes nothing pays one read lock per
    // leaf for its column (the M10-3 slab is that document).
    // Nothing this leaf's reasoning can rest on as an indeterminate:
    // no node of the drive froze, none of its own freezes stayed in the
    // leaf, and every id its DAG names is one it holds. The answer is 0
    // without walking anything — a document that freezes nothing pays
    // one read lock per leaf for its column (the M10-3 slab is that
    // document).
    if sess.foreign.is_empty() && sess.plain_unpublished.is_empty() && memo.frozen_is_empty() {
        return 0;
    }
    let reached = sess.closure(sess.plain_roots.keys().copied());
    // Every freeze this leaf made was made inside its own walk, so it
    // is under a root: the union below never counts a node outside the
    // closure, and a push from anywhere but the walk would break that.
    debug_assert!(
        sess.plain_unpublished
            .keys()
            .all(|id| reached.contains_key(id)),
        "a freeze was recorded outside the plain walk's own closure"
    );
    // **The leaf's own side**: the ids under its roots that it cannot
    // resolve to a form of its own — the ones its table does not hold,
    // whether or not its walk ever reached them — together with the
    // freezes it made and could not publish. Both restricted to the
    // closure, because a node the leaf built outside its own reasoning
    // is not part of it.
    let mut own: IdSet = IdSet::default();
    // `foreign` is collected as the DAG is built and is reconciled
    // HERE: a node named by a parent before the session minted it, and
    // minted inside it afterwards, is one this leaf's table holds by
    // the time it reasons — so the leaf resolved it and needs nothing
    // for it. The reconciliation is one lookup per candidate, and the
    // candidates are none on every drive measured.
    for id in sess.foreign.keys() {
        if reached.contains_key(id) && !sess.nodes.contains_key(id) {
            own.insert(*id, ());
        }
    }
    for id in sess.plain_unpublished.keys() {
        if reached.contains_key(id) {
            own.insert(*id, ());
        }
    }
    memo.need(&reached, &own)
}

/// The counts so far in the installed session (`None` outside one) — the
/// door a driver reads mid-replay when it prices a leaf.
///
/// **`frozen` here is the WORK so far, not the leaf's NEED**: the
/// freezes this replay has computed. A NEED is a property of the
/// finished leaf — it is read against the drive's set once the leaf has
/// published, and [`SymCounts::frozen`] carries both meanings — while
/// what a pricer asks mid-replay is what this leaf has already paid.
/// Every other column is the same one the leaf's receipt will carry.
#[must_use]
pub fn session_counts() -> Option<SymCounts> {
    SESSION.with(|s| s.borrow().as_ref().map(|s| s.counts))
}

/// **Whether a decision may be taken on a worker thread at all** — the
/// test every walk that maps deciding units onto rayon workers reads
/// before it maps (`topo::props`' face walks, `editor_core`'s node
/// schedule), and the one home of that test.
///
/// The K-funnel's frame and sample sink are thread-local too, but they
/// compose back: a unit run under `k_stats::detached` hands its
/// recording back as a value and the caller's fold splices it
/// (`k_stats::map_detached`). The two thread-locals read here do not,
/// and three things follow, only the first of which is about
/// recording:
///
/// - **The decision itself changes.** `Sym`'s `sign_within` consults
///   the session; with none installed `discharge` answers `None`, the
///   identity tier discharges nothing and the answer is the plain
///   numeric one. A unit on a worker would decide differently from its
///   siblings on the caller's thread — the one thing D9 forbids
///   outright.
/// - **The receipt is written in place.** `count_decision` and
///   `count_registration_contradicted` mutate the installed session's
///   [`SymCounts`], and [`Sym::opaque`] advances the per-replay
///   `OPAQUE_SEQ` counter — a sequence whose determinism rests on the
///   minting ORDER being a fixed single-threaded walk. Neither is a
///   value handed back, so neither can be spliced. (Node ids are NOT in
///   this list: `intern` is a content hash of the node, so the DAG a
///   replay builds is the same whatever order it is built in.)
/// - **The shape report is written in place too.** [`report`]'s rows
///   are pushed from `Sym::sign_within` into a thread-local on the
///   deciding thread, with or without a session, so a unit decided on a
///   worker is missing from the report the caller takes.
///
/// So a walk stays on the caller's thread — the serial walk, exactly —
/// for as long as either is installed. It is a property of the CALL and
/// not of the scalar: `Sym` with neither installed is as portable as
/// `f64`, and the test reads the thread's state rather than the type.
#[must_use]
pub fn decisions_are_thread_portable() -> bool {
    session_counts().is_none() && !report::active()
}

/// Records `node` in the installed session and answers its id. Outside a
/// session the id is still computed — it is a pure function of the node
/// — and nothing is stored.
fn intern(node: SymNode) -> SymId {
    let id = node.id();
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            // The kids are looked at ONCE per distinct node — on the
            // vacant arm, where this node is new to the table — and
            // only under a drive memo, where the leaf's `frozen` column
            // is a NEED and `Session::foreign` is what its own side is
            // counted from. Hash-consing means a node is interned many
            // times and minted once, so this is a walk of the DAG and
            // not of the build.
            if let std::collections::hash_map::Entry::Vacant(e) = sess.nodes.entry(id) {
                e.insert(node);
                if sess.memo.is_some() {
                    for kid in node.kids() {
                        if !sess.nodes.contains_key(kid) {
                            sess.foreign.insert(*kid, ());
                        }
                    }
                }
            }
        }
    });
    id
}

// ---------------------------------------------------- the normal form

/// The indeterminate π enters the form as: a fixed key, so `τ − 2π`
/// cancels while nothing reads a value of π anywhere.
const INDET_PI: u128 = 0x5049_5f49_4e44_4554_5f5f_5f5f_5f5f_5f5f;

/// The indeterminate ONE OPAQUE VALUE enters the form as, keyed by its
/// per-replay sequence number — a different key per call, which is what
/// makes two untracked reals two unknowns rather than one.
fn indet_opaque(seq: u64) -> u128 {
    Hash128::new()
        .word(0x4f50_4151_5545_5f5f)
        .word(seq)
        .finish()
}

/// The indeterminate a parameter symbol enters the form as.
fn indet_param(symbol: u64) -> u128 {
    Hash128::new()
        .word(0x5041_5241_4d5f_494e)
        .word(symbol)
        .finish()
}

/// The indeterminate an OPAQUE atom enters the form as: its op tag, its
/// payload and the DIGESTS of its argument forms — so two atoms whose
/// arguments are the same rational function are one indeterminate.
fn indet_atom(tag: u64, payload: u64, args: &[u128]) -> u128 {
    let mut h = Hash128::new()
        .word(0x4154_4f4d_5f49_4e44)
        .word(tag)
        .word(payload);
    for d in args {
        h = h.wide(*d);
    }
    h.finish()
}

/// The value an opaque UNARY atom takes at argument zero, where that
/// value is expressible in the form's own vocabulary — the fold that
/// lets `‖a − b‖` decide `Zero` when `a − b` does, which is the shape
/// most of the kernel's identity margins arrive in (`Margin::of` of a
/// distance is a `sqrt`).
fn unary_at_zero(op: SymOp) -> Option<Form> {
    match op {
        SymOp::Sqrt
        | SymOp::Abs
        | SymOp::Sin
        | SymOp::Tan
        | SymOp::Asin
        | SymOp::Atan
        | SymOp::Floor => Some(Form::zero()),
        SymOp::Cos => Some(Form::poly(Poly::one())),
        // acos 0 = π/2 — expressible, because π is an indeterminate of
        // the form rather than a number.
        SymOp::Acos => Some(Form::poly(Poly::term(
            vec![(INDET_PI, 1)],
            Rat::new(1, 2, 0)?,
        ))),
        // 1/0 is not a real; the numeric channel owns that refusal.
        _ => None,
    }
}

/// The PLAIN form of one node, given its children's forms — every atom
/// opaque, no rule applied — `None` for anything the caller must freeze
/// (an overflow, a budget, an unrepresentable literal, a reciprocal of
/// the zero form).
///
/// The atom algebra is NOT here: it runs later, once, over the top
/// residual ([`algebra::reduce`]), so it can never disturb a
/// cancellation the plain form already reaches. Every atom this mints
/// is recorded in the session ([`Session::atoms`]) so that reduction
/// can look its argument form back up.
fn combine(node: &SymNode, kids: [&Form; 2], sess: &mut Session, early: bool) -> Option<Form> {
    let (a, b) = (kids[0], kids[1]);
    let budget = sess.budget;
    // Where A0 applies: in the early walk when one is configured
    // (ALONGSIDE — the plain form stays M10-7's and can lose nothing),
    // otherwise in the plain form (REPLACING — cheaper, and measured
    // to lose theorems to coefficient freezes at the ring's bound:
    // `SymRules::const_fold`).
    let a0 = sess.rules.const_fold && (early || !sess.rules.early);
    // Rule C applies in the EARLY walk only (`SymRules::signed_root`).
    let c = early && sess.rules.signed_root;
    // Rule F, the manifest sign, likewise (`SymRules::manifest_sign`).
    let f_sign = early && sess.rules.manifest_sign;
    // An atom over a gated argument is gated: it stands for the value
    // of a form that is only box-wise equal to the expression.
    let gate = |mut f: Form| {
        f.gated |= a.gated;
        f
    };
    let atom1 = |op: SymOp, sess: &mut Session| {
        // A function OF an expression with no value has no value
        // either, and `a.is_zero()` is already false for a poisoned
        // argument, so the at-zero fold cannot fire on one.
        if a.poisoned {
            return Some(Form::poison());
        }
        if a.is_zero()
            && let Some(f) = unary_at_zero(op)
        {
            return Some(gate(f));
        }
        let id = indet_atom(op.tag(), node.payload, &[a.digest()]);
        mint_atom(sess, id, early, || AtomInfo {
            op,
            payload: node.payload,
            args: [Some(Arc::new(a.clone())), None],
        });
        Some(gate(Form::poly(Poly::indet(id))))
    };
    match node.op {
        SymOp::Param => Some(Form::poly(Poly::indet(indet_param(node.payload)))),
        // One untracked real: its OWN indeterminate, keyed by the
        // sequence number the node carries (`SymOp::Opaque`'s docs).
        SymOp::Opaque => Some(Form::poly(Poly::indet(indet_opaque(node.payload)))),
        SymOp::Lit => {
            Rat::of_f64(f64::from_bits(node.payload)).map(|c| Form::poly(Poly::constant(c)))
        }
        SymOp::Pi => Some(Form::poly(Poly::indet(INDET_PI))),
        // **The zero normalization the algebra needs** (early walk,
        // with a rule that expands atoms on): `0/d + x = x`, `0/d · x =
        // 0`. The quotient form cancels no common factor, so a zero
        // NUMERATOR keeps its denominator and drags it into every sum
        // it joins — an arc's `n̂ · apothem` at bulge one is `0/‖chord‖`,
        // its centre becomes `mid · ‖chord‖/‖chord‖`, and rule A then
        // expands the `‖chord‖²` that ride along into polynomials of
        // rising degree with 53-bit coefficients, which is what froze
        // the plate's odd-sample residuals at every ring width. Sound
        // by the argument the quotient form already rests on: `0/d` is
        // the zero rational function, and a point where `d` vanishes is
        // one clause 1 has already refused. Kept behind the algebra
        // dials so the tier with them off is the earlier one bit for
        // bit; the plain walk never takes it.
        SymOp::Add | SymOp::Sub | SymOp::Mul
            if early
                && (sess.rules.early_ab || sess.rules.trig_of_atan)
                && !a.tainted(b)
                && (a.is_zero() || b.is_zero()) =>
        {
            let gated = a.gated || b.gated;
            let mut f = match (node.op, a.is_zero()) {
                (SymOp::Mul, _) => Form::zero(),
                (SymOp::Add, true) => b.clone(),
                (SymOp::Add, false) => a.clone(),
                (_, true) => b.neg()?,
                (_, false) => a.clone(),
            };
            f.gated = gated;
            Some(f)
        }
        SymOp::Add => a.add(b, budget),
        SymOp::Sub => a.add(&b.neg()?, budget),
        SymOp::Mul => a.mul(b, budget),
        SymOp::Neg => a.neg(),
        SymOp::Inv => a.recip(),
        SymOp::Powi => {
            let n = node.payload as u32 as i32;
            match u32::try_from(n) {
                Ok(n) => powi_form(a, n, budget),
                Err(_) => powi_form(&a.recip()?, n.unsigned_abs(), budget),
            }
        }
        // A0: a sqrt/abs of a CONSTANT form folds exactly; then rule F
        // (early walk): `abs(X) = X` where the FORM shows `X` positive
        // and `−X` where it shows `X` negative, which reads no value;
        // then rule C (early walk): a sqrt of a
        // perfect square, or an abs, of a form with a CERTIFIED sign
        // folds to the signed root. The value-free rule is asked
        // before the one that reads a value, so a discharge that can
        // be a theorem is never counted `sign_gated`.
        SymOp::Sqrt | SymOp::Abs if (a0 || c || f_sign) && !a.poisoned => {
            let folded = (|| {
                if !a0 {
                    return None;
                }
                let n = a.num.as_constant()?;
                let d = a.den.as_constant()?;
                let c = n.mul(&d.recip()?)?;
                match node.op {
                    SymOp::Sqrt => c.sqrt_exact(),
                    _ => Some(c.abs()),
                }
            })();
            if let Some(k) = folded {
                return Some(gate(Form::poly(Poly::constant(k))));
            }
            if f_sign
                && node.op == SymOp::Abs
                && let Some(f) = manifest::fold_abs(a, sess)
            {
                return Some(gate(f));
            }
            if c && let Some(f) = signed::fold(node.op, a, &sess.params, budget) {
                return Some(gate(f));
            }
            atom1(node.op, sess)
        }
        // Rule D (early walk only): `sin`/`cos` of `q · atan(X)` in
        // closed form; any other argument shape keeps the atom.
        SymOp::Sin | SymOp::Cos if early && sess.rules.trig_of_atan && !a.poisoned => {
            #[cfg(feature = "sym-profile-testing")]
            let t0 = profile::clock();
            let folded = trig::fold(node.op, a, sess);
            #[cfg(feature = "sym-profile-testing")]
            profile::trig_done(t0);
            match folded {
                Some(f) => Some(gate(f)),
                None => atom1(node.op, sess),
            }
        }
        SymOp::Sqrt
        | SymOp::Abs
        | SymOp::Sin
        | SymOp::Cos
        | SymOp::Tan
        | SymOp::Asin
        | SymOp::Acos
        | SymOp::Atan
        | SymOp::Floor => atom1(node.op, sess),
        SymOp::Atan2 | SymOp::Min | SymOp::Max | SymOp::Copysign => {
            if a.tainted(b) {
                return Some(Form::poison());
            }
            // **Rule F** (early walk): `copysign(Y, X) = |Y|` wherever
            // the FORM of `X` is manifestly POSITIVE — the sign the
            // node asks for is one the form already shows, so the
            // opaque `copysign` atom is never minted (`manifest`
            // carries the predicate, the two identities and the
            // signed-zero edge that makes the predicate STRICT).
            if node.op == SymOp::Copysign
                && f_sign
                && manifest::positive(b, sess)
                && let Some(mut m) = manifest::magnitude(a, sess)
            {
                m.gated = a.gated || b.gated;
                return Some(m);
            }
            // The same rule's NEGATIVE arm: `copysign(Y, X) = −|Y|`
            // wherever the form of `X` is manifestly NEGATIVE (the
            // predicate of `−X`; `manifest` carries the reflection).
            if node.op == SymOp::Copysign
                && f_sign
                && manifest::negative(b, sess)
                && let Some(m) = manifest::magnitude(a, sess)
                && let Some(mut m) = m.neg()
            {
                m.gated = a.gated || b.gated;
                return Some(m);
            }
            // min(0, 0) and max(0, 0) are zero; a one-sided zero says
            // nothing, so only the both-zero fold is taken. copysign
            // carries `a`'s MAGNITUDE, so a zero first argument is zero
            // whatever the sign argument does (±0 is one real).
            // atan2(0, x) is 0 or π depending on the sign of x, so the
            // fold below is taken ONLY where the sign is a fact of the
            // form: atan2(0, N) with N non-negative BY SYNTAX is 0 —
            // rule D's second fold (amendment A1), early walk only
            // (`manifest::nonneg` carries the argument); a plain
            // parameter, a non-zero first argument, or a value-only
            // zero never folds, and every other atan2 stays an atom.
            let folds = match node.op {
                SymOp::Min | SymOp::Max => a.is_zero() && b.is_zero(),
                SymOp::Copysign => a.is_zero(),
                SymOp::Atan2 => {
                    early && sess.rules.trig_of_atan && a.is_zero() && manifest::nonneg(b, sess)
                }
                _ => false,
            };
            if folds {
                let mut z = Form::zero();
                z.gated = a.gated || b.gated;
                return Some(z);
            }
            let id = indet_atom(node.op.tag(), node.payload, &[a.digest(), b.digest()]);
            mint_atom(sess, id, early, || AtomInfo {
                op: node.op,
                payload: node.payload,
                args: [Some(Arc::new(a.clone())), Some(Arc::new(b.clone()))],
            });
            let mut f = Form::poly(Poly::indet(id));
            f.gated = a.gated || b.gated;
            Some(f)
        }
        // Keyed by the CHILD IDS, never by their forms (the op's docs).
        // A hull of something with no value has none either, so the
        // poison crosses this door like every other.
        SymOp::Hull if a.tainted(b) => Some(Form::poison()),
        SymOp::Hull => {
            let mut f = Form::poly(Poly::indet(
                Hash128::new()
                    .word(SymOp::Hull.tag())
                    .wide(node.kids[0].bits())
                    .wide(node.kids[1].bits())
                    .finish(),
            ));
            f.gated = a.gated || b.gated;
            Some(f)
        }
    }
}

/// The normal form of `root`, computed into `memo` inside `sess` — the
/// one walk both memos share, `early` choosing which.
///
/// **Two memos per session, and the distinction is the whole
/// architecture of the atom algebra.** The PLAIN walk (`early = false`,
/// every atom opaque, rule A0 only) builds the quotient normal form as
/// the tier stood before the algebra plus the constant fold; it is what
/// a decision is FIRST tested against, and a plain form that is zero is
/// an unconditional theorem. The EARLY walk (`early = true`,
/// `SymRules::early`) applies rules A/B per node under [`EARLY_STEPS`]
/// and rule C's fold at each `sqrt`/`abs` — ALONGSIDE the plain memo,
/// never replacing it, so a rule can only ADD a discharge and never
/// re-label one the plain form reached. That split is measured, not
/// assumed: the first cut of this unit let a ruled form REPLACE the
/// plain one and lost an `arc_span` cancellation and a straight edge's
/// endpoint theorem to it.
///
/// Iterative rather than recursive: an evaluation's DAG is as deep as
/// its expression tree, and a leaf replay's is thousands of nodes.
/// Termination is structural — a node's id is a hash of its children's
/// ids, so a cycle would need a hash preimage — and every popped id
/// leaves a form behind, so each is visited at most twice. Either walk
/// is the O(dag) construction; the early walk's per-node reduction is
/// bounded by its step cap, so its cost is a constant factor over the
/// plain walk, measured per document in `SymRules::shipped`'s docs.
fn form_in(
    sess: &mut Session,
    memo: &mut IdMap<Arc<Form>>,
    root: SymId,
    early: bool,
    registry: bool,
) -> Arc<Form> {
    // The freeze this WALK made. The running count is the leaf's WORK,
    // and it is what the column says only outside a drive: a leaf of a
    // drive has its column rewritten as its NEED at the leaf's end
    // ([`leaf_need`], which [`SymCounts::frozen`] argues).
    let frozen = |sess: &mut Session, id: SymId| -> Arc<Form> {
        if !early {
            sess.counts.frozen += 1;
        }
        Arc::new(Form::poly(Poly::indet(id.bits())))
    };
    // **One predicate for the plain walk**, and the drive's memo behind
    // it: the memo is the PLAIN walk's alone, because the early and door
    // walks consult this leaf's registry and its parameter brackets,
    // which a value-dependent refusal can make differ between leaves.
    let plain = !early && !registry;
    let drive = plain.then(|| sess.memo.clone()).flatten();
    // **The walk root, noted for the leaf's NEED** ([`leaf_need`]): the
    // column counts the drive's freezes inside the closure of what this
    // leaf ASKED, and the asking is here.
    if drive.is_some() {
        sess.plain_roots.insert(root, ());
    }
    // **One place that notes what this walk computed** — the profile's
    // distinct-id counter, and the drive memo's publication list.
    //
    // `publish` is the WRITE side of the guard the read side makes
    // below, and the two must agree: a node absent from this leaf's
    // table is frozen here by design, and publishing that freeze under
    // the node's CONTENT id would hand it to a leaf that recorded the
    // node and would have computed a real form for it — a decision
    // moved, and an order-dependent one (R1 M6 / R2 MINOR-2, both
    // demonstrated at the door). The guard is on the TAINT, not on the
    // unrecorded node alone: its recorded parent's id is a hash of the
    // children's ids, so the parent carries the same id in both leaves
    // and a different form (`Session::plain_tainted`).
    let note = |sess: &mut Session, id: SymId, froze: bool, publish: bool| {
        #[cfg(feature = "sym-profile-testing")]
        if plain {
            profile::record_plain_id(id.bits());
        }
        if drive.is_some() {
            if publish {
                sess.plain_built.push(id);
                if froze {
                    sess.plain_frozen.push(id);
                }
            } else if froze {
                // A freeze this leaf may not hand to the drive. It is
                // still a node this leaf's reasoning rested on, and
                // [`leaf_need`] counts it beside the drive's set.
                sess.plain_unpublished.insert(id, ());
            }
        }
    };
    let mut stack = vec![(root, false)];
    while let Some((id, expanded)) = stack.pop() {
        if memo.contains_key(&id) {
            continue;
        }
        // **The registered-identity door** ([`Sym::register_equal`]),
        // and the whole of where it acts: a node a constructor
        // registered against another takes THAT node's form, marked
        // `registered` so the decision it answers is counted as the
        // axiom it is. Consulted in the EARLY walk only, so the plain
        // form — the one a decision is asked of first — is M10-8's
        // exactly and no theorem is ever re-labelled.
        if registry {
            let to = sess.alias(id);
            if to != id {
                if let Some(f) = memo.get(&to).cloned() {
                    memo.insert(id, f);
                } else {
                    stack.push((id, false));
                    stack.push((to, false));
                }
                continue;
            }
        }
        let Some(node) = sess.nodes.get(&id).copied() else {
            // Not in this session's table: an unrecorded leaf, or a node
            // minted before the session was installed. An unknown
            // function of the parameters is exactly an indeterminate.
            #[cfg(feature = "sym-profile-testing")]
            profile::record_unrecorded(profile::Walk::of(early, registry));
            let f = frozen(sess, id);
            // NOT published, and everything built from it is tainted:
            // `note`'s own comment says why.
            if drive.is_some() {
                sess.plain_tainted.insert(id, ());
            }
            note(sess, id, true, false);
            memo.insert(id, f);
            continue;
        };
        let arity = node.op.arity();
        // **The drive memo, asked only for a node THIS leaf recorded.**
        // An id absent from the table is one minted before the session
        // was installed, and the walk freezes it above by design; taking
        // a drive-built form for it would move a decision the tier makes
        // about an unrecorded node. Asked before the children are
        // expanded, so a hit costs the subtree nothing.
        if !expanded
            && let Some(d) = &drive
            && let Some(f) = d.form(id)
        {
            d.seed_atoms(&f, &mut sess.atoms);
            memo.insert(id, f);
            continue;
        }
        if !expanded {
            let pending: Vec<SymId> = node
                .kids()
                .iter()
                .copied()
                .filter(|k| !memo.contains_key(k))
                .collect();
            if !pending.is_empty() {
                stack.push((id, true));
                stack.extend(pending.into_iter().map(|k| (k, false)));
                continue;
            }
        }
        let empty = Form::zero();
        let fa = if arity >= 1 {
            memo.get(&node.kids[0]).cloned()
        } else {
            None
        };
        let fb = if arity >= 2 {
            memo.get(&node.kids[1]).cloned()
        } else {
            None
        };
        let budget = sess.budget;
        // Built out of an unrecorded node? Then this leaf's form for
        // `id` is this leaf's alone, and the atoms this node mints are
        // keyed by its tainted argument digests.
        let taint = drive.is_some()
            && node
                .kids()
                .iter()
                .any(|k| sess.plain_tainted.contains_key(k));
        let atoms_before = sess.plain_atoms.len();
        let made = {
            let kids = [
                fa.as_deref().unwrap_or(&empty),
                fb.as_deref().unwrap_or(&empty),
            ];
            #[cfg(feature = "sym-profile-testing")]
            profile::clear_note();
            let combined = combine(&node, kids, sess, early);
            // The per-node A/B reduction (`SymRules::early_ab`),
            // bounded in steps and in the size of the form it is asked
            // over, falling back to the un-reduced form when it does
            // not fit.
            let combined = if early && sess.rules.early_ab {
                combined.map(|f| {
                    if f.num.terms().len() + f.den.terms().len() > EARLY_AB_TERMS {
                        return f;
                    }
                    #[cfg(feature = "sym-profile-testing")]
                    let t0 = profile::clock();
                    let reduced =
                        algebra::reduce_steps(&f, sess.rules, budget, &sess.atoms, EARLY_STEPS);
                    #[cfg(feature = "sym-profile-testing")]
                    profile::reduce_done(t0);
                    reduced
                        .filter(|g| within(budget, g))
                        // The gate has ONE home: `algebra::apply`
                        // carries the input form's gate through every
                        // substitution step, so a reduced form is gated
                        // if what it reduced was, and nothing here
                        // carries it a second time (R1 MIN-7). Checked
                        // rather than re-done: dropping a gate would
                        // report a weaker claim as a stronger one, the
                        // one direction the receipt may never move in.
                        .inspect(|g| {
                            debug_assert!(
                                g.gated || !f.gated,
                                "algebra::apply carries the gate through every step"
                            );
                        })
                        .unwrap_or(f)
                })
            } else {
                combined
            };
            // **Rule E**, after the per-node A/B reduction and before
            // the budget check. AFTER is the order that lets the rule
            // ACT, and the walk ledger
            // (`editor-core/tests/m10_sym_profile_interval`) is what
            // pins it: planted BEFORE the reduction, that ledger reds
            // in four lines — the slab's `Early/Assertion` digest, and
            // on the plate `Early/Decision` frozen 8 → 48 with its
            // digest, `Early/Assertion`'s digest and `Door/Decision`'s
            // — with the largest form the walk builds falling
            // 288 → 90, the plate's early-decision walk back where it
            // was before the rule. So under the other order the rule
            // does almost nothing there: this is a REACH requirement
            // and not only a convention. It is also the order
            // `trig::sqrt_atom` applies to a form it builds by hand,
            // which is what makes the two spellings of one arc key one
            // atom.
            // Before the budget check, because the rule can only
            // SHRINK a form in terms and degree, so one it cancels may
            // fit where the raw one would have frozen.
            let combined = if early && sess.rules.common_factor {
                combined.map(|f| quotient::cancel(&f))
            } else {
                combined
            };
            let made = combined.filter(|f| within(budget, f));
            #[cfg(feature = "sym-profile-testing")]
            profile::record_node(
                node.op,
                profile::Walk::of(early, registry),
                kids,
                made.as_ref(),
            );
            made
        };
        drop((fa, fb));
        if taint {
            sess.plain_tainted.insert(id, ());
            // Every atom this node minted is keyed by a tainted
            // argument's digest, so no untainted form can reference one.
            sess.plain_atoms.truncate(atoms_before);
        }
        let froze = made.is_none();
        let f = match made {
            Some(p) => Arc::new(p),
            None => frozen(sess, id),
        };
        note(sess, id, froze, !taint);
        memo.insert(id, f);
    }
    memo.get(&root)
        .cloned()
        .unwrap_or_else(|| Arc::new(Form::poly(Poly::indet(root.bits()))))
}

/// Records the atom `id` in the session, noting it for the drive memo
/// whenever the PLAIN walk REFERENCES it — the atoms a plain form's
/// indeterminates stand for, which a leaf that takes that form from the
/// memo needs and never mints itself.
///
/// **Noted on every plain-walk reference, not only on a fresh mint**,
/// and that is load-bearing rather than slack: the two walks share one
/// `atoms` map, and an atom's id is a hash of the op, the payload and
/// the ARGUMENT FORM's digest — so wherever a rule left a kid's early
/// form equal to its plain one, an earlier EARLY walk has already
/// minted the atom a later plain walk references. Noting only the mint
/// would publish a plain form whose indeterminate the memo cannot
/// explain, and the leaf that took it would lose the rule-A
/// substitution [`algebra::reduce`] looks the argument up for. The list
/// is deduplicated at the memo's `or_insert`.
fn mint_atom(sess: &mut Session, id: u128, early: bool, info: impl FnOnce() -> AtomInfo) {
    sess.atoms.entry(id).or_insert_with(info);
    if !early && sess.memo.is_some() {
        sess.plain_atoms.push(id);
    }
}

/// The plain quotient form of `root` — every atom opaque, no rule
/// applied, no value read. Memoized in the session's persistent table.
fn plain_form(sess: &mut Session, root: SymId) -> Arc<Form> {
    let mut memo = core::mem::take(&mut sess.forms);
    #[cfg(feature = "sym-profile-testing")]
    let t0 = profile::clock();
    let out = form_in(sess, &mut memo, root, false, false);
    #[cfg(feature = "sym-profile-testing")]
    profile::walk_done(profile::Walk::Plain, t0);
    sess.forms = memo;
    out
}

/// The most rule-A/B substitutions the early walk takes per node
/// before it gives the un-reduced form back — the bound that makes the
/// per-node reduction a fixed cost rather than a pass over the form.
/// One step clears every even power of ONE atom across the whole form,
/// so the count is the number of distinct reducible atoms a node's
/// form carries (plus the ones a substitution re-introduces), which on
/// the arc family is under a dozen.
const EARLY_STEPS: usize = 64;

/// The largest form (numerator terms plus denominator terms) the
/// per-node reduction is asked over. A form past it is left as it is:
/// the reduction is linear in the form but a substituted argument
/// multiplies into every term, and a form this size is one the budget
/// is about to freeze in its next product whether or not its squares
/// were cleared. The arc family's residuals are a handful of terms per
/// node; this is a cost wall, not a reach.
const EARLY_AB_TERMS: usize = 512;

/// The early-reduced form of `root` (`SymRules::early`), memoized in
/// its own table beside the plain one: the same walk as
/// [`plain_form`], with rules A/B applied per node under
/// [`EARLY_STEPS`] and rule C's fold at each `sqrt`/`abs`.
fn early_form(sess: &mut Session, root: SymId) -> Arc<Form> {
    let mut memo = core::mem::take(&mut sess.forms_early);
    #[cfg(feature = "sym-profile-testing")]
    let t0 = profile::clock();
    let out = form_in(sess, &mut memo, root, true, false);
    #[cfg(feature = "sym-profile-testing")]
    profile::walk_done(profile::Walk::Early, t0);
    sess.forms_early = memo;
    out
}

/// The DOOR form of `root` — [`early_form`]'s walk with the session's
/// registry applied ([`Sym::register_equal`]), memoized in its own
/// third table. Asked only after the plain and the early forms have
/// both declined, so a zero it finds is one the registration was
/// needed for.
fn door_form(sess: &mut Session, root: SymId) -> Arc<Form> {
    let mut memo = core::mem::take(&mut sess.forms_door);
    #[cfg(feature = "sym-profile-testing")]
    let t0 = profile::clock();
    let out = form_in(sess, &mut memo, root, true, true);
    #[cfg(feature = "sym-profile-testing")]
    profile::walk_done(profile::Walk::Door, t0);
    sess.forms_door = memo;
    out
}

/// How the symbolic tier discharged a decision.
///
/// **A sixth kind reds three pins**, and is not to be added without
/// them: `sym::discharge_pins` holds this enum against
/// [`SymCounts`]'s receipt columns and against
/// [`report::ShapeOutcome`]'s report rows, and `k_stats_doors`'s
/// `every_discharge_kind_retags_its_sample_with_a_token_of_its_own`
/// holds it against [`crate::k_stats::SampleOutcome`]'s K tokens —
/// whose own agreement with the lint that reads them is
/// `k-lint`'s `tests/outcome_vocabulary.rs`, the fourth row a kind
/// with a new token reaches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Discharge {
    /// An unconditional theorem: the form is the zero polynomial, no
    /// value read (`symbolic_zero`).
    Theorem,
    /// A theorem conditional on a clause-3 sign read over the leaf's
    /// box — the form is zero and was built through rule C's fold
    /// (`sign_gated`).
    SignGated,
    /// An AXIOM about the construction: the form is zero, and it is
    /// zero because a constructor registered two of its nodes as one
    /// real ([`Sym::register_equal`]). Verified at the leaf's witness
    /// when it was registered, counted apart from both theorem kinds
    /// (`registered`).
    Registered,
}

#[cfg(any(test, feature = "probe"))]
impl Discharge {
    /// **Every discharge kind, once** — the roster the seam pins
    /// enumerate instead of writing a `match` of their own.
    ///
    /// The `match` below answers nothing and exists to be EXHAUSTIVE:
    /// a kind missing from the list beside it is a compile error here,
    /// so a pin that iterates this roster iterates the whole
    /// vocabulary rather than the part someone remembered.
    fn all() -> [Self; 3] {
        let all = [Self::Theorem, Self::SignGated, Self::Registered];
        for kind in all {
            match kind {
                Self::Theorem | Self::SignGated | Self::Registered => {}
            }
        }
        all
    }
}

#[cfg(feature = "probe")]
impl Discharge {
    /// **The K sample token a decision this kind answered is retagged
    /// with** — the projection [`Sym::sign_within`] applies at its
    /// [`crate::k_stats::retag_at`], in ONE place, so the pin that
    /// holds the seam reads the production mapping rather than a copy
    /// of it.
    fn sample_outcome(self) -> crate::k_stats::SampleOutcome {
        match self {
            Self::Theorem => crate::k_stats::SampleOutcome::SymbolicZero,
            Self::SignGated => crate::k_stats::SampleOutcome::SignGated,
            Self::Registered => crate::k_stats::SampleOutcome::Registered,
        }
    }
}

/// **Every discharge kind with the K token it retags its sample
/// with**, in roster order — the seam
/// [`Discharge::sample_outcome`] crosses, published so that the pin
/// holding it can be an INTEGRATION suite.
///
/// It has to be one: [`Discharge`] is private to this module, and a
/// `probe`-gated `#[test]` inside the library is COMPILED by CI and
/// run by nothing — the sweep that runs the probe suites invokes
/// `--test all` (`scripts/k_probe_sweep.sh`), so a lib row under this
/// feature would report the same green whether it passed or never
/// executed. The pin is
/// `every_discharge_kind_retags_its_sample_with_a_token_of_its_own`,
/// in `geom-core`'s `k_stats_doors` suite.
///
/// A test-support door and not a widening of the shipped surface:
/// `probe` is the K-telemetry feature, off in every build that ships
/// (see its entry in this crate's `Cargo.toml`).
#[cfg(feature = "probe")]
#[must_use]
pub fn discharge_sample_outcomes() -> Vec<(String, crate::k_stats::SampleOutcome)> {
    Discharge::all()
        .into_iter()
        .map(|kind| (format!("{kind:?}"), kind.sample_outcome()))
        .collect()
}

/// **The identity test**: is this node's expression identically zero in
/// the parameters — as an unconditional theorem, or as one conditional
/// on a certified sign?
///
/// Three tiers, in the order that keeps a stronger claim from being
/// re-labelled as a weaker one: the PLAIN form (every atom opaque,
/// rules A0 only) is the zero polynomial — a theorem; the EARLY form
/// (rules A/B per node, rule C's fold) is zero — a theorem if no fold
/// took part, `SignGated` if one did; the top residual reduces to zero
/// under rules A/B — a theorem. Each tier is memoized per session, so
/// a decision pays the walk it needs once.
///
/// `None` outside a session, and at a zero-term budget — the tier
/// switched off inside the scalar.
fn discharge(id: SymId) -> Option<Discharge> {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        if sess.budget.max_terms == 0 {
            return None;
        }
        let plain = plain_form(sess, id);
        if plain.is_zero() {
            return Some(Discharge::Theorem);
        }
        let rules = sess.rules;
        if rules.early {
            let e = early_form(sess, id);
            if e.is_zero() {
                return Some(if e.gated {
                    Discharge::SignGated
                } else {
                    Discharge::Theorem
                });
            }
        }
        // Rules A and B (unconditional) over the residual, once —
        // BEFORE the door, because a zero they reach is a THEOREM and
        // labelling one an axiom would understate what the tier proved.
        // (An earlier cut asked the door here and said in its own
        // comment that it asked last; under `SymRules::all()` that
        // attributed A/B theorems to `registered`. R1 m1 / R2 MINOR-4.)
        if rules.sqrt_square || rules.pythagoras {
            #[cfg(feature = "sym-profile-testing")]
            let t0 = profile::clock();
            let reduced = algebra::reduce(&plain, rules, sess.budget, &sess.atoms);
            #[cfg(feature = "sym-profile-testing")]
            profile::reduce_top_done(t0);
            if reduced.as_ref().is_some_and(|f| f.is_zero()) {
                return Some(Discharge::Theorem);
            }
        }
        // THE DOOR, asked LAST and only where there is a registration
        // to ask about ([`Sym::register_equal`]): only where every walk
        // above has declined is the registration what answered, and
        // that is exactly the claim `SymCounts::registered` makes.
        //
        // **A GATED door form does not discharge.** A zero that rests
        // BOTH on a constructor's axiom and on rule C's box-wise sign
        // read is two weakenings at once, and the receipt has one
        // column for each and none for the pair; reporting it as either
        // alone would overstate one of them. So it falls to the numeric
        // channel — the conservative direction, and unreachable in a
        // shipped run because `signed_root` is dial-off
        // (`SymRules::shipped`). Pinned rather than assumed.
        if rules.registered && rules.early && !sess.registry.is_empty() {
            let d = door_form(sess, id);
            if d.is_zero() && !d.gated {
                return Some(Discharge::Registered);
            }
        }
        None
    })
}

/// **Is this node's DOOR form the zero form** — the registry applied,
/// and nothing else asked?
///
/// The one question a decision the numeric channel has already proved
/// NON-ZERO still has to ask (`Decide for Sym<T>`). A plain or early
/// zero under a definite numeric sign is a contradiction between two
/// channels that read no axioms, and stays what it was: a debug
/// assertion, because it can only be a bug in the tier itself. A
/// REGISTERED zero under a definite sign is a different animal — it
/// means the axiom a constructor stated is false over this box — and it
/// is the one M10-9 makes possible, so it is checked in release,
/// counted, and never folded.
///
/// Answers `false` immediately when the registry is empty, which is
/// every document with no arc in it and therefore most of the corpus:
/// the cost of the check is paid only where a registration exists to be
/// wrong.
fn door_zero(id: SymId) -> bool {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let Some(sess) = slot.as_mut() else {
            return false;
        };
        if sess.budget.max_terms == 0
            || !(sess.rules.registered && sess.rules.early)
            || sess.registry.is_empty()
        {
            return false;
        }
        let d = door_form(sess, id);
        d.is_zero() && !d.gated
    })
}

/// Records a registration the door refused, for the session's receipt.
fn count_registration_refused() {
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            sess.counts.registrations_refused += 1;
        }
    });
}

/// Records a decision whose REGISTERED zero contradicted a definite
/// numeric sign.
fn count_registration_contradicted() {
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            sess.counts.registrations_contradicted += 1;
        }
    });
}

/// **Does this node's form claim zero where the value channel answered
/// a definite non-zero sign?** — the theorem-vs-numeric contradiction,
/// as ONE predicate, so the two arms of
/// [`Decide::sign_within`]'s charge cannot come to differ on what a
/// contradiction is.
///
/// Both discharge kinds count: a [`Discharge::Theorem`] is zero
/// unconditionally, a [`Discharge::SignGated`] zero over the leaf's box
/// on the strength of one sign read, and over THIS box — the one the
/// value channel just classified — both say the margin is zero where it
/// says it is not. [`Discharge::Registered`] is deliberately absent:
/// a registered zero contradicted by a definite sign is the door's own
/// event and is counted at the door
/// ([`SymCounts::registrations_contradicted`], raised just above this
/// call).
///
/// It RUNS THE WALKS ([`discharge`]), which is the cost each arm pays.
fn contradicts(id: SymId) -> bool {
    matches!(
        discharge(id),
        Some(Discharge::Theorem | Discharge::SignGated)
    )
}

/// Records a decision whose form was a THEOREM or a GATED theorem under
/// a definite numeric sign at an INEXACT witness.
fn count_theorem_disputed() {
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            sess.counts.theorems_disputed += 1;
        }
    });
}

/// Records how one decision was answered, for the session's receipt.
fn count_decision(discharge: Option<Discharge>) {
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            match discharge {
                Some(Discharge::Theorem) => sess.counts.symbolic_zero += 1,
                Some(Discharge::SignGated) => sess.counts.sign_gated += 1,
                Some(Discharge::Registered) => sess.counts.registered += 1,
                None => sess.counts.numeric += 1,
            }
        }
    });
}

// --------------------------------------------------------- the scalar

/// **The symbolic tier's lane scalar**: the value `T` computes as
/// today, plus a handle into the expression DAG (module docs).
///
/// Every [`Real`] operation computes the value at `T` VERBATIM — so a
/// `Sym<T>` run is bit-identical to a `T` run in its numeric channel by
/// construction — and mints one content-hashed node beside it. Only
/// [`Decide::sign_within`] behaves differently, and only in the one
/// direction E12 sanctions: a margin whose expression is identically
/// zero answers `Zero` without consulting the enclosure.
///
/// `Copy`, because the handle is an id and evaluation code is
/// arithmetic-dense (the same reason [`Real`] demands it).
#[derive(Clone, Copy, Debug)]
pub struct Sym<T> {
    /// The numeric channel — the value a plain `T` run would carry.
    pub value: T,
    node: SymId,
}

impl<T> Sym<T> {
    /// This value's DAG node.
    #[must_use]
    pub fn node(self) -> SymId {
        self.node
    }

    /// A value carrying NO tracked expression: **its own fresh
    /// indeterminate**, so its form is an unknown and every decision
    /// that depends on it is the numeric one.
    ///
    /// The door for a lane that legitimately has no expression to track
    /// — a bracket handed back by an engine that ran at another scalar
    /// — where fabricating a node would claim an algebraic relationship
    /// that was never computed.
    ///
    /// **Each call mints a DIFFERENT unknown, and that is the whole
    /// point.** Two untracked reals are two unknowns: `x` and `y` with
    /// nothing known about either. If they shared an id they would be
    /// one unknown, `x − x` would be the zero polynomial, and
    /// `opaque(1.0) − opaque(2.0)` would decide `Zero` — a theorem
    /// about two values that are not equal. They are also never keyed
    /// by the VALUE: two enclosures that happen to be bit-equal are
    /// still two separate reals, and keying by bits would say they are
    /// one.
    ///
    /// The sequence that separates them is [`OPAQUE_SEQ`], whose docs
    /// carry the D9 argument for why a counter here is still
    /// schedule-independent.
    #[must_use]
    pub fn opaque(value: T) -> Self {
        let seq = OPAQUE_SEQ.with(|c| {
            let n = c.get();
            c.set(n.wrapping_add(1));
            n
        });
        #[cfg(feature = "sym-profile-testing")]
        profile::record_opaque(indet_opaque(seq));
        Self::nullary(value, SymOp::Opaque, seq)
    }

    /// A value bound as the document PARAMETER `symbol` — the one door
    /// that introduces an indeterminate. No bracket is recorded, so
    /// rule C ([`signed`]) can fold nothing over this parameter; a
    /// caller holding the box's bounds uses [`Self::param_over`].
    #[must_use]
    pub fn param(symbol: ParamSymbol, value: T) -> Self {
        Self {
            value,
            node: intern(SymNode {
                op: SymOp::Param,
                payload: symbol.0,
                kids: [SymId::UNRECORDED; 2],
            }),
        }
    }

    /// [`Self::param`] over the bracket `[lo, hi]` the value was built
    /// from — the analysis box's own two `f64`s, which the caller that
    /// mints a parameter axis already holds. The bracket is recorded in
    /// the installed session for rule C's sign read ([`signed`]); it is
    /// the ONLY value the symbolic tier ever reads, and it is read as
    /// two floats through a ring enclosure, never as the lane scalar.
    /// Outside a session the bracket is dropped and this is `param`.
    #[must_use]
    pub fn param_over(symbol: ParamSymbol, value: T, lo: f64, hi: f64) -> Self {
        SESSION.with(|s| {
            if let Some(sess) = s.borrow_mut().as_mut() {
                sess.params.insert(indet_param(symbol.0), (lo, hi));
            }
        });
        Self::param(symbol, value)
    }

    /// Mints the node for a nullary op.
    fn nullary(value: T, op: SymOp, payload: u64) -> Self {
        Self {
            value,
            node: intern(SymNode {
                op,
                payload,
                kids: [SymId::UNRECORDED; 2],
            }),
        }
    }

    /// Mints the node for a one-child op.
    fn unary(self, value: T, op: SymOp, payload: u64) -> Self {
        Sym {
            value,
            node: intern(SymNode {
                op,
                payload,
                kids: [self.node, SymId::UNRECORDED],
            }),
        }
    }

    /// Mints the node for a two-child op.
    fn binary(self, other: Self, value: T, op: SymOp) -> Self {
        Sym {
            value,
            node: intern(SymNode {
                op,
                payload: 0,
                kids: [self.node, other.node],
            }),
        }
    }
}

impl<T: Real> Sym<T> {
    /// **The registered-identity door** (M10-9; ERROR-DESIGN E12's
    /// "kept in reserve — discharge by provenance", taken): records
    /// that `self` and `other` denote ONE function of the parameters,
    /// because the constructor that calls this GUARANTEES it.
    ///
    /// The tier's own `Zero` is a THEOREM — exact rational arithmetic
    /// from the parameter symbols down, no value read. A registered
    /// identity is an AXIOM: it rests on the registrant's argument
    /// about what it built, and this door is where that argument enters
    /// the tier. The two are counted apart for exactly that reason
    /// ([`SymCounts::registered`], `SampleOutcome::Registered`), and a
    /// registrant that cannot state its argument in its doc comment has
    /// no business calling this.
    ///
    /// # What it does, and does not, do
    ///
    /// **It aliases NODES, and only nodes.** The LEFT node takes the
    /// RIGHT node's normal form: the caller registers the quantity it
    /// DERIVED against the one the construction HOLDS
    /// (`norm.register_equal(radius)`). It is not a form-level equation
    /// and there is no axiom store: an identity between two
    /// independently built expressions discharges only where the
    /// registrant builds the same content-hashed node the consumer
    /// builds, which is a testable condition and is tested.
    ///
    /// **The value channel is untouched.** Nothing here reads, writes
    /// or derives a value except the witness check below, which only
    /// answers yes or no. `self.value` and `other.value` stay exactly
    /// what their operations produced, at every lane — which is why the
    /// cheaper spelling of the same wish, having the constructor
    /// normalize by the declared radius (`(q - c) / r` instead of
    /// `(q - c) / ||q - c||`), is REJECTED: it would change the `f64`
    /// lane's bits and the numeric enclosure's dependency structure to
    /// buy a symbolic cancellation, which is paying in the one currency
    /// this tier promised not to spend.
    ///
    /// **It is consulted in the EARLY walk only.** The plain quotient
    /// form — what a decision is asked of first — never sees the
    /// registry, so every theorem the tier proved before this door
    /// still counts as one and the door can only ADD a discharge
    /// ([`SymRules::registered`]).
    ///
    /// # The witness, and the refusals
    ///
    /// The lane scalar is asked first ([`Real::register_equal`]): at
    /// [`crate::Interval`] the two certified enclosures must MEET, at
    /// `f64` the two values must agree to the run's ε relative to the
    /// larger magnitude — which is why `tol` is a parameter here, and
    /// why it is handed down rather than read ([`Real::register_equal`]).
    /// Where they do not, the door records nothing and answers the lane
    /// scalar's own refusal arm, forwarded: `Contradicted` from the
    /// exact witness, `Disputed` from an inexact one. A constructor that
    /// does not build what it claims therefore cannot state it, and the
    /// answer says whether the refusal is a PROOF of that or an
    /// arithmetic that could not tell
    /// ([`SymRegistration::Disputed`]). A registration
    /// that would close a cycle is refused
    /// [`SymRegistration::Cyclic`] — `form_in`'s termination rests on
    /// a node's id being a hash of its children's, and the registry is
    /// the one thing that could break that by hand.
    ///
    /// The door is not the soundness argument on its own, and is not
    /// claimed to be. Two further things hold it up: the numeric
    /// channel runs FIRST at every decide site and short-circuits on a
    /// definite non-zero sign, so **no registration can turn a margin
    /// the enclosure proved non-zero into a `Zero`**; and the `f64`
    /// witness pass evaluates every residual at the point against its
    /// own band, where widening cannot hide a construction that lied.
    ///
    /// # Order against memoization
    ///
    /// A registration INVALIDATES the door memo — `forms_door` is
    /// cleared — so a registrant may register after a consumer has
    /// already decided, and the next decision sees the record. The
    /// alternative (refusing a late registration) was rejected: the
    /// evaluation service interleaves construction and decisions, so
    /// "the registrant registers before any consumer builds" is not a
    /// property a constructor can promise. What is NOT retroactive is
    /// history: a decision already answered numerically stays answered,
    /// which is a fact about when it was asked and not a miss. Clearing
    /// costs the early walk of whatever is asked next, and only on a
    /// registration that CHANGES the registry — a repeat answers
    /// [`SymRegistration::Already`] and clears nothing.
    ///
    /// # D9
    ///
    /// The registry is keyed by content hashes and lives in the
    /// per-leaf session, like the node table; a leaf's registrations
    /// are made by the same fixed single-threaded walk of its recipe
    /// that mints its nodes, so the record is identical across repeats
    /// and across the rayon schedule, exactly as [`OPAQUE_SEQ`]'s
    /// argument runs.
    #[must_use = "a registration can be REFUSED, and a refusal a caller \
                  drops is a lie nobody sees"]
    pub fn register_equal(self, other: Self, tol: Tol) -> SymRegistration {
        // The witness first: a claim the value channel refused
        // (`Contradicted` or `Disputed`) or could not witness
        // (`Unwitnessed`) never reaches the registry at all. A refusal
        // is COUNTED — the receipt is where a constructor that states a
        // lie becomes visible — and the value channel's ARM is FORWARDED unchanged,
        // because which refusal it is is a fact about the lane scalar's
        // witness rather than about the registry: `Contradicted` is a
        // proof (`Interval`'s disjoint certified enclosures),
        // `Disputed` an inexact witness that could not tell (`f64`,
        // `Probe`). Both refuse identically here — nothing is recorded.
        match self.value.register_equal(other.value, tol) {
            refusal @ (SymRegistration::Contradicted | SymRegistration::Disputed) => {
                count_registration_refused();
                return refusal;
            }
            SymRegistration::Unwitnessed => return SymRegistration::Unwitnessed,
            _ => {}
        }
        SESSION.with(|s| {
            let mut slot = s.borrow_mut();
            let Some(sess) = slot.as_mut() else {
                return SymRegistration::Witnessed;
            };
            if sess.budget.max_terms == 0 || !sess.rules.registered {
                return SymRegistration::Witnessed;
            }
            let (a, b) = (sess.alias(self.node), sess.alias(other.node));
            if a == b {
                return SymRegistration::Already;
            }
            if sess.reaches(b, a) {
                sess.counts.registrations_refused += 1;
                return SymRegistration::Cyclic;
            }
            sess.registry.insert(a, b);
            // Only the DOOR memo can hold a form the new record would
            // have changed; the plain and early memos never consult the
            // registry, so they stay whole (and M10-8's, bit for bit).
            sess.forms_door.clear();
            SymRegistration::Recorded
        })
    }
}

impl<T: Real> Add for Sym<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.binary(rhs, self.value + rhs.value, SymOp::Add)
    }
}

impl<T: Real> Sub for Sym<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.binary(rhs, self.value - rhs.value, SymOp::Sub)
    }
}

impl<T: Real> Mul for Sym<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.binary(rhs, self.value * rhs.value, SymOp::Mul)
    }
}

/// Division mints `a · Inv(b)` (module docs): `Inv` is an opaque atom,
/// so `(a/b)·b` does not fold back to `a` and the tier claims nothing
/// about it.
impl<T: Real> Div for Sym<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let inv = rhs.unary(rhs.value, SymOp::Inv, 0);
        self.binary(inv, self.value / rhs.value, SymOp::Mul)
    }
}

impl<T: Real> Neg for Sym<T> {
    type Output = Self;

    fn neg(self) -> Self {
        self.unary(-self.value, SymOp::Neg, 0)
    }
}

impl<T: Real> Real for Sym<T> {
    /// **The LANE SCALAR's.** A `Sym<T>` carries `T`'s value channel
    /// unchanged — the tier reads no value and widens no enclosure —
    /// so a comparison at `Sym<T>` proves exactly what one at `T`
    /// proves. This is the const [`Decide::sign_within`] below charges
    /// the theorem-vs-numeric contradiction by.
    const WITNESS: crate::real::Witness = T::WITNESS;

    fn from_f64(x: f64) -> Self {
        Self::nullary(T::from_f64(x), SymOp::Lit, x.to_bits())
    }

    fn zero() -> Self {
        // The same node a `from_f64(0.0)` mints, so the two spellings
        // of the additive identity share one id.
        Self::nullary(T::zero(), SymOp::Lit, 0f64.to_bits())
    }

    fn one() -> Self {
        Self::nullary(T::one(), SymOp::Lit, 1f64.to_bits())
    }

    fn pi() -> Self {
        Self::nullary(T::pi(), SymOp::Pi, 0)
    }

    /// τ's VALUE is `T::tau()` — the base scalar's own constant, so the
    /// numeric channel is untouched — while its NODE is `2·π`, which is
    /// what τ is as a real. Both enclose the same real number, which is
    /// all the identity test ever claims.
    fn tau() -> Self {
        let two = Self::from_f64(2.0);
        let pi = Self::pi();
        Sym {
            value: T::tau(),
            node: intern(SymNode {
                op: SymOp::Mul,
                payload: 0,
                kids: [two.node, pi.node],
            }),
        }
    }

    fn sqrt(self) -> Self {
        self.unary(self.value.sqrt(), SymOp::Sqrt, 0)
    }

    fn abs(self) -> Self {
        self.unary(self.value.abs(), SymOp::Abs, 0)
    }

    fn is_poison(self) -> bool {
        self.value.is_poison()
    }

    /// **The one scalar that RECORDS** rather than only witnessing —
    /// the door itself ([`Sym::register_equal`], which carries the
    /// whole of the contract).
    fn register_equal(self, other: Self, tol: Tol) -> SymRegistration {
        Sym::register_equal(self, other, tol)
    }

    fn powi(self, n: i32) -> Self {
        self.unary(self.value.powi(n), SymOp::Powi, u64::from(n as u32))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.value.sin_cos();
        (self.unary(s, SymOp::Sin, 0), self.unary(c, SymOp::Cos, 0))
    }

    fn tan(self) -> Self {
        self.unary(self.value.tan(), SymOp::Tan, 0)
    }

    fn asin(self) -> Self {
        self.unary(self.value.asin(), SymOp::Asin, 0)
    }

    fn acos(self) -> Self {
        self.unary(self.value.acos(), SymOp::Acos, 0)
    }

    fn atan(self) -> Self {
        self.unary(self.value.atan(), SymOp::Atan, 0)
    }

    fn atan2(self, x: Self) -> Self {
        self.binary(x, self.value.atan2(x.value), SymOp::Atan2)
    }

    fn min(self, other: Self) -> Self {
        self.binary(other, self.value.min(other.value), SymOp::Min)
    }

    fn max(self, other: Self) -> Self {
        self.binary(other, self.value.max(other.value), SymOp::Max)
    }

    fn floor(self) -> Self {
        self.unary(self.value.floor(), SymOp::Floor, 0)
    }

    fn copysign(self, sign: Self) -> Self {
        self.binary(sign, self.value.copysign(sign.value), SymOp::Copysign)
    }
}

/// The bracket is the value channel's, verbatim: the DAG carries no
/// numbers of its own and is never read here.
impl<T: Bounds> Bounds for Sym<T> {
    fn lo(self) -> f64 {
        Bounds::lo(self.value)
    }

    fn hi(self) -> f64 {
        Bounds::hi(self.value)
    }
}

/// Certification delegates: whether the computation was defined on the
/// whole box is a question about the numeric channel, and the symbolic
/// tier neither widens nor narrows the answer.
impl<T: CertifiedEnclosure> CertifiedEnclosure for Sym<T> {
    fn certified_bracket(self) -> Option<(f64, f64)> {
        self.value.certified_bracket()
    }

    fn crossing_bracket(self) -> (f64, f64) {
        self.value.crossing_bracket()
    }
}

/// Span selection is STRUCTURE selection and reads the value channel;
/// the hull mints a `Hull` node keyed by the two operands' ids (never by
/// their forms — see [`SymOp::Hull`]).
impl<T: SpanLocate> SpanLocate for Sym<T> {
    fn locate_spans<'a>(self, knots: &'a KnotVector) -> SpanSet<'a> {
        self.value.locate_spans(knots)
    }

    fn enclosure_hull(self, other: Self) -> Self {
        self.binary(other, self.value.enclosure_hull(other.value), SymOp::Hull)
    }
}

/// **The whole of the tier's effect on decision-making** (E12): the
/// symbolic step happens INSIDE the scalar, so `k_stats::decide`, its
/// private `classify` and every funnel site are untouched by
/// construction.
///
/// The two clauses of the theorem, in order:
///
/// 1. **the computation was defined on the whole input box**, checked
///    in TWO places because one scalar cannot see both halves of it.
///
///    *The value side.* [`MarginDiag::Invalid`] is the arm every scalar
///    returns for a domain violation it can see —
///    [`crate::Interval::sign_within`] for an uncertified enclosure,
///    `f64` and [`crate::Probe`] for NaN — so the numeric channel
///    already answers that question and this impl needs no bracket door
///    of its own. Without it, `sqrt(-1) − sqrt(-1)` decides `Zero` on an
///    expression with no real value.
///
///    *The form side*, and it is not decoration:
///    **[`Form::poisoned`]**. At `f64` and `Probe` the only thing
///    `Invalid` catches is NaN, and a violation can hide behind a
///    perfectly finite value — `atan(1/(x−x)) − atan(1/(x−x))` is
///    `atan(+inf) − atan(+inf)` = `π/2 − π/2` = a finite `0.0`, with no
///    real behind it anywhere. So a form built through a division by the
///    ZERO polynomial is poisoned, the poison propagates through every
///    combinator, and a poisoned form is never zero. That is the half
///    the value channel structurally cannot supply at a point scalar.
///
///    (At `Interval` the same expression violates the domain visibly —
///    the division is empty and the decoration drops — so the value side
///    catches it there. Both halves are present at every base scalar
///    because neither one is sufficient at all of them.)
/// 2. **the node's normal form is the zero polynomial**, computed with
///    exact rational coefficients from the parameter symbols down.
///    Nothing in that computation reads a value.
///
/// Together they say: the margin is a real number, and that real number
/// is zero at every parameter point of the box. `Sign::Zero` is then a
/// theorem, not a measurement — which is why no band is consulted and
/// why the answer does not depend on the box's width.
///
/// **The numeric channel runs FIRST, always**, which costs one
/// `sign_within` on the symbolic path and buys two things: clause 1 above
/// and an honest K sample. At `Probe` the base scalar records the margin
/// it classified before this impl overrides the answer, so the funnel's
/// sample carries a real number and is merely RE-TAGGED
/// ([`crate::k_stats`]'s `retag_at`) rather than replaced
/// by one with no margin in it — at the index taken BEFORE the base
/// scalar ran, so the row re-tagged is this decision's own.
///
/// **And a definite non-zero numeric answer short-circuits the form.**
/// A certified enclosure that excludes zero is a proof that the margin
/// is not zero, so no normal form over the parameters can be the zero
/// polynomial — asking for one is work whose answer is already known.
/// Building it was a measurable share of the tier's cost (a reviewer
/// clocked one leaf replay at 57 ms against 1.4 ms numeric), and the
/// forms skipped here are exactly the expensive ones: the margins that
/// are NOT identities, which is most of them.
///
/// **And the shortcut is kept honest PER WITNESS KIND**
/// ([`crate::Witness`], declared on the lane scalar). The sentence
/// above — "a certified enclosure that excludes zero is a proof" — is
/// a claim about the VALUE CHANNEL, and it is true at exactly the
/// scalars whose witness is exact:
///
/// - **At an EXACT witness** ([`crate::Interval`]) the enclosure
///   proves it, so a form that IS zero under a definite numeric sign
///   means the two channels contradict each other: an enclosure does
///   not contain its real, or the form is wrong. That is a soundness
///   bug in one of them and not a fast path to take quietly, and the
///   `debug_assert!` below is the answer.
/// - **At an INEXACT witness** (`f64`, [`crate::Probe`]) the same
///   answer is a comparison of ONE ROUNDED NUMBER against the band,
///   and it is wrong in three measured ways: at a far placement the
///   rounding exceeds the band; under rule F a one-ulp error in a
///   sign argument becomes a whole `2.0` at the margin, because
///   `copysign`'s output is `±1` however small its argument's error
///   was; and at a pole the channel evaluates `1/0` to an infinity
///   and has no clause 1 to refuse with. The form's theorem is
///   correct wherever the function is defined, so the disagreement is
///   a DISPUTE: the numeric answer is kept, the run says so on the
///   receipt ([`SymCounts::theorems_disputed`]), and nothing panics.
///
/// It is the same partition [`Sym::register_equal`] already forwards
/// for the registry door — [`SymRegistration::Contradicted`] from an
/// exact witness, [`SymRegistration::Disputed`] from an inexact one —
/// drawn for the theorem channels, and both read the one marker.
///
/// Everything else is `T::sign_within` verbatim.
impl<T: Decide> Decide for Sym<T> {
    fn sign_within(self, band: Band) -> Result<Sign, Indeterminate> {
        // Where this decision's own K sample will land, read before the
        // base scalar records it (`k_stats::sink_mark`).
        #[cfg(feature = "probe")]
        let mark = crate::k_stats::sink_mark();
        let numeric = self.value.sign_within(band);
        let domain_violation =
            matches!(&numeric, Err(e) if matches!(e.margin, MarginDiag::Invalid));
        let definitely_nonzero = matches!(&numeric, Ok(Sign::Positive | Sign::Negative));
        if definitely_nonzero {
            // **A REGISTERED zero here is a CONTRADICTED AXIOM**, and it
            // is checked in release rather than asserted in debug: a
            // constructor stated an identity this box's numeric answer
            // denies, and the numeric answer wins — but the run has to
            // SAY so. Never a fold; counted; the receipt reports it
            // (`SymCounts::registrations_contradicted`). Counted at
            // every witness kind, never asserted at any: what the
            // witness kind decides is how strong the denial is, and
            // this column records the event rather than grading it.
            //
            // Ordering matters: this asks the DOOR memo only, and only
            // where a registration exists, so a document with no arc
            // pays one `is_empty()`.
            if door_zero(self.node) {
                count_registration_contradicted();
            }
            // **The contradiction, charged by WITNESS KIND.** Each arm
            // asks [`contradicts`], which RUNS THE DISCHARGE — the
            // plain walk and the early one — and the cost profile
            // charges those walks to `Origin::Assertion`, apart from
            // the decision's. `T::WITNESS` is a const, so the arm this
            // lane takes is fixed at monomorphization and the other one
            // is not compiled.
            //
            // **WHEN each arm pays for that walk is NOT the same, and
            // the asymmetry is the point.** The exact arm asks inside a
            // `debug_assert!`, so with debug assertions off it asks
            // nothing and the walk does not happen; the inexact arm
            // asks unconditionally, because the count is a receipt
            // column and a column that exists only under debug
            // assertions is a column a run cannot be asked for. This
            // workspace ships `[profile.release] debug-assertions =
            // true`, so both arms walk in every profile it builds.
            #[cfg(feature = "sym-profile-testing")]
            let origin = profile::set_origin(profile::Origin::Assertion);
            match T::WITNESS {
                // An EXACT witness is a certified bracket that
                // excludes zero: a proof that the margin is not zero,
                // which no normal form over the parameters can
                // contradict. If one does, an enclosure does not
                // contain its real or the form is wrong — a soundness
                // bug in one of the two channels, and this codebase
                // fails loud on it.
                crate::Witness::Exact => debug_assert!(
                    !contradicts(self.node),
                    "the numeric channel proved this margin nonzero and the form says it is \
                     identically zero: the two channels contradict each other"
                ),
                // An INEXACT witness compared one rounded number. The
                // form's theorem holds wherever the function is
                // defined; what this channel answered is a point
                // reading the band could not save. So the numeric
                // answer is kept — the ratified numeric-first order is
                // untouched — and the disagreement is COUNTED.
                crate::Witness::Inexact => {
                    if contradicts(self.node) {
                        count_theorem_disputed();
                    }
                }
            }
            #[cfg(feature = "sym-profile-testing")]
            profile::set_origin(origin);
            count_decision(None);
            report::record(&numeric, None, None, self.value.enclosure_probe());
            return numeric;
        }
        let symbolic = if domain_violation {
            None
        } else {
            discharge(self.node)
        };
        count_decision(symbolic);
        if let Some(how) = symbolic {
            #[cfg(feature = "probe")]
            crate::k_stats::retag_at(mark, how.sample_outcome());
            report::record(&numeric, Some(how), None, self.value.enclosure_probe());
            return Ok(Sign::Zero);
        }
        // The shape report wants the residual that BLOCKED — rendered
        // only when the instrument is installed, so an ordinary replay
        // never pays for it.
        if report::active() {
            #[cfg(feature = "sym-profile-testing")]
            let origin = profile::set_origin(profile::Origin::Report);
            let text = report::render_node(self.node);
            #[cfg(feature = "sym-profile-testing")]
            profile::set_origin(origin);
            report::record(&numeric, None, text, self.value.enclosure_probe());
        }
        numeric
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::predicate::Margin;
    use crate::tolerance::Tol;

    /// **The SHIPPED budget**, so these rows exercise the dials a drive
    /// actually runs at. They used to use `max_degree: 16` while the
    /// shipped value was 128 — a unit test that never touched the
    /// configuration it was defending.
    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        }
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness tolerance has a linear band")
    }

    /// The parameter, at `f64`: a point value with a symbol on it.
    fn p(name: &str, v: f64) -> Sym<f64> {
        Sym::param(ParamSymbol::of(name), v)
    }

    fn decides_zero(m: Sym<f64>) -> bool {
        crate::k_stats::decide("sym_test", Margin::of(m), band()) == Ok(Sign::Zero)
    }

    #[test]
    fn a_literal_difference_is_the_zero_form() {
        let (out, counts) = with_session(budget(), || {
            let x = p("w", 0.37);
            let a = x + Sym::from_f64(2.0) * x;
            let b = Sym::from_f64(3.0) * x;
            decides_zero(a - b)
        });
        assert!(out, "3x written two ways is the same polynomial");
        assert_eq!(counts.symbolic_zero, 1);
        assert_eq!(counts.numeric, 0);
    }

    /// The shape most identity margins arrive in: a NORM of a vector
    /// that is componentwise zero. `sqrt` is an opaque atom, so this
    /// only works because an atom over a zero form folds to `f(0)`.
    #[test]
    fn a_norm_of_a_zero_vector_decides_symbolically() {
        let (out, _) = with_session(budget(), || {
            let t = p("depth", 0.5);
            let one = Sym::from_f64(1.0);
            // (P + t·d) − (P + t·d), componentwise, then the norm.
            let comp = |k: f64| {
                let base = Sym::from_f64(k);
                let a = base + t * one;
                let b = base + one * t;
                a - b
            };
            let (x, y, z) = (comp(3.0), comp(-1.5), comp(0.0));
            let n = (x * x + y * y + z * z).sqrt();
            decides_zero(n)
        });
        assert!(out, "the norm of a componentwise-zero vector is zero");
    }

    /// A COINCIDENCE at the nominal is not an identity: two segments
    /// collinear at `p = 0` only. It never decides symbolically, at any
    /// parameter value.
    #[test]
    fn a_coincidence_at_the_nominal_never_decides_symbolically() {
        for v in [0.0, 1e-12, 0.5] {
            let (_, counts) = with_session(budget(), || {
                let x = p("w", v);
                decides_zero(x * x)
            });
            // At v = 0 the NUMERIC channel answers `Zero`, and that is
            // the point: a coincidence is decided by the enclosure, at
            // the width the enclosure has, and widens with the box. The
            // tier never claims it.
            assert_eq!(counts.symbolic_zero, 0, "at {v}");
            assert_eq!(counts.numeric, 1, "at {v}");
        }
    }

    /// The cross product of a direction with itself: zero in every
    /// component, by different routes through the same symbols.
    #[test]
    fn a_self_cross_product_is_identically_zero() {
        let (out, _) = with_session(budget(), || {
            let d = [p("a", 0.3), p("b", -0.7), p("c", 0.1)];
            let cross = [
                d[1] * d[2] - d[2] * d[1],
                d[2] * d[0] - d[0] * d[2],
                d[0] * d[1] - d[1] * d[0],
            ];
            cross.into_iter().all(decides_zero)
        });
        assert!(out, "d x d is the zero vector, symbolically");
    }

    /// π's own identity: `τ − 2π` is zero as a real, and the form says
    /// so without reading either constant's value.
    #[test]
    fn tau_is_two_pi_in_the_form() {
        let (out, _) = with_session(budget(), || {
            decides_zero(<Sym<f64> as Real>::tau() - Sym::from_f64(2.0) * <Sym<f64> as Real>::pi())
        });
        assert!(out);
    }

    /// The documented limits, pinned as limits: no factoring past the
    /// quotient, and no trigonometric identity beyond rule B. `sin(2θ)
    /// − 2·sinθ·cosθ` is not the zero form — `sin(2θ)` is an atom of
    /// another argument and nothing relates it to the pair — and with
    /// the rules OFF the Pythagorean pair is not either, which is what
    /// makes `SymRules::none` the pre-algebra tier.
    #[test]
    fn the_opaque_atoms_are_opaque() {
        let (_, counts) = with_session(budget(), || {
            let x = p("w", 3.0);
            let (s, c) = x.sin_cos();
            let (s2, _) = (x + x).sin_cos();
            decides_zero(s2 - Sym::from_f64(2.0) * s * c)
        });
        // Numerically zero at this point — the numeric channel answers
        // it, as it always did. What is pinned is that the TIER claims
        // nothing: the double-angle identity is outside every rule.
        assert_eq!(counts.symbolic_zero, 0, "no double-angle identity");
        assert_eq!(counts.sign_gated, 0);
        assert_eq!(counts.numeric, 1);
        let (_, counts) = with_session_rules(budget(), SymRules::none(), || {
            let x = p("w", 3.0);
            let (s, c) = x.sin_cos();
            decides_zero(s * s + c * c - Sym::from_f64(1.0))
        });
        assert_eq!(counts.symbolic_zero, 0, "rule B off: both atoms opaque");
        assert_eq!(counts.numeric, 1);
    }

    /// **Rule B**: the Pythagorean pair of ONE argument form is the zero
    /// form, whatever the argument; of two different arguments it is
    /// not.
    #[test]
    fn the_pythagorean_pair_of_one_argument_is_a_theorem() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let x = p("w", 3.0);
            let y = p("h", 0.25);
            let (s, c) = (x * y + Sym::from_f64(2.0)).sin_cos();
            let same = decides_zero(s * s + c * c - Sym::from_f64(1.0));
            let (s2, _) = y.sin_cos();
            let mixed = decides_zero(s2 * s2 + c * c - Sym::from_f64(1.0));
            (same, mixed)
        });
        assert!(out.0, "sin²θ + cos²θ − 1 is the zero form");
        assert_eq!(counts.symbolic_zero, 1, "{counts:?}");
        assert_eq!(counts.sign_gated, 0);
        // The mixed pair decides NUMERICALLY (it is not a theorem, and
        // at this point it is not zero either).
        assert_eq!(counts.numeric, 1, "{counts:?}");
    }

    /// **Rule A**: an even power of a `sqrt` atom is its argument, so
    /// `sqrt(X)·sqrt(X) − X` and `sqrt(X)³ − X·sqrt(X)` are theorems.
    #[test]
    fn a_square_root_squared_is_its_argument() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let (x, y) = (p("w", 3.0), p("h", 0.25));
            let arg = x * x + y * y + Sym::from_f64(1.0);
            let s = arg.sqrt();
            let a = decides_zero(s * s - arg);
            let b = decides_zero(s * s * s - arg * s);
            let c = decides_zero(s.powi(2) - arg);
            (a, b, c)
        });
        assert_eq!(out, (true, true, true));
        assert_eq!(counts.symbolic_zero, 3, "{counts:?}");
        assert_eq!(counts.sign_gated, 0, "rule A reads no value");
    }

    /// A parameter with its bracket recorded, at `f64` — the door rule C
    /// reads through ([`Sym::param_over`]).
    fn p_over(name: &str, v: f64, lo: f64, hi: f64) -> Sym<f64> {
        Sym::param_over(ParamSymbol::of(name), v, lo, hi)
    }

    /// **Rule C, clause 3: `sqrt(r²) − r` is a theorem CONDITIONAL on
    /// `r`'s sign**, and is counted as one. With `r`'s bracket strictly
    /// positive the fold takes `sqrt(r²) → r` and the decision is
    /// `sign_gated` — never `symbolic_zero`, because it holds on the box
    /// and not identically. `abs(r) − r` folds the same way. The
    /// residual sits one power below rule A (`sqrt` to the FIRST
    /// power), which is why an unconditional rule cannot reach it.
    #[test]
    fn rule_c_discharges_a_signed_root_as_sign_gated() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
            let sq = decides_zero((r * r).sqrt() - r);
            let abs = decides_zero(r.abs() - r);
            (sq, abs)
        });
        assert_eq!(out, (true, true));
        assert_eq!(
            counts.sign_gated, 2,
            "both are clause-3 theorems: {counts:?}"
        );
        assert_eq!(
            counts.symbolic_zero, 0,
            "and neither is an unconditional one"
        );
        assert_eq!(counts.numeric, 0);
    }

    /// **Rule C's negative sign**: `sqrt(r²) + r` folds when `r` is
    /// DEFINITELY negative (`sqrt(r²) = −r` there), and not otherwise.
    #[test]
    fn rule_c_folds_the_negated_root_under_a_negative_sign() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p_over("r", -1.25e-3, -2.0e-3, -1.0e-3);
            (
                decides_zero((r * r).sqrt() + r),
                decides_zero(r.abs() + r),
                // The same residual with the WRONG sign is not zero, and
                // the fold does not make it one: it decides numerically.
                decides_zero((r * r).sqrt() - r),
            )
        });
        assert_eq!(out, (true, true, false));
        assert_eq!(counts.sign_gated, 2, "{counts:?}");
        assert_eq!(counts.numeric, 1);
    }

    /// **Rule C's refusals**: a bracket that STRADDLES zero never folds
    /// (the sign is not certified), a parameter with no bracket
    /// recorded never folds, and with the rule off the atom stays
    /// opaque — in every case the decision is the numeric channel's
    /// own and `sign_gated` stays zero.
    #[test]
    fn rule_c_never_folds_without_a_certified_sign() {
        // Straddling.
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p_over("r", 1.25e-3, -1.0e-3, 2.0e-3);
            (decides_zero((r * r).sqrt() - r), decides_zero(r.abs() - r))
        });
        assert_eq!(out, (true, true), "numerically zero at the point");
        assert_eq!(
            counts.sign_gated, 0,
            "a straddling bracket folds nothing: {counts:?}"
        );
        assert_eq!(counts.numeric, 2);
        // A zero endpoint is not strictly signed.
        let (_, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p_over("r", 1.25e-3, 0.0, 2.0e-3);
            decides_zero((r * r).sqrt() - r)
        });
        assert_eq!(counts.sign_gated, 0, "{counts:?}");
        // No bracket at all (`Sym::param`).
        let (_, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p("r", 1.25e-3);
            decides_zero((r * r).sqrt() - r)
        });
        assert_eq!(counts.sign_gated, 0, "no bracket, no read: {counts:?}");
        assert_eq!(counts.numeric, 1);
        // The rule off (the shipped set is measured, not assumed:
        // `SymRules::shipped`'s docs).
        let (_, counts) = with_session_rules(
            budget(),
            SymRules {
                signed_root: false,
                ..SymRules::all()
            },
            || {
                let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
                decides_zero((r * r).sqrt() - r)
            },
        );
        assert_eq!(counts.sign_gated, 0, "{counts:?}");
        assert_eq!(counts.numeric, 1);
    }

    /// **A plain theorem is never re-labelled by rule C.** The early
    /// walk runs ALONGSIDE the plain form, and a decision the plain form
    /// answers is `symbolic_zero` even when a gated fold would also have
    /// reached it.
    #[test]
    fn a_plain_theorem_stays_unconditional_beside_rule_c() {
        let (_, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
            // `sqrt(r²)·sqrt(r²) − r²`: rule A reaches it in the early
            // walk too, and the plain form does not — but with r's sign
            // certified the early walk's FIRST fold is C's, so this is
            // gated; the plain-zero row below is the one that must not
            // be.
            decides_zero((r * r).sqrt() - (r * r).sqrt());
        });
        assert_eq!(
            counts.symbolic_zero, 1,
            "x − x is the zero form: {counts:?}"
        );
        assert_eq!(counts.sign_gated, 0);
    }

    /// **The candidate shape the plate's ceiling has**: `sqrt(X) − R`
    /// with `X = R²` as forms where `X` is NOT a syntactic square —
    /// `(a + 2r)²` expanded to `a² + 4ar + 4r²` under the root — folds
    /// under rule C when `a + 2r` has a certified sign. This is
    /// `‖q − c‖ = r` with the endpoint at `c + r·(1, 0)` scaled by 2.
    #[test]
    fn rule_c_recovers_the_root_of_an_expanded_square() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let a = p_over("a", 0.5, 0.25, 0.75);
            let r = p_over("r", 1.25e-3, 1.0e-3, 2.0e-3);
            let x = a * a + Sym::from_f64(4.0) * a * r + Sym::from_f64(4.0) * r * r;
            decides_zero(x.sqrt() - (a + Sym::from_f64(2.0) * r))
        });
        assert!(out);
        assert_eq!(counts.sign_gated, 1, "{counts:?}");
        assert_eq!(counts.symbolic_zero, 0);
    }

    /// **Division is IN the normal form** (the quotient of polynomials,
    /// not an opaque reciprocal): `(x/y)·y − x` is the zero form.
    ///
    /// This is the shape the kernel's own endpoint-pinning identity
    /// arrives in — an extruded strut's carrier is metered in metres and
    /// its direction normalized, so the residual is
    /// `w·(‖w‖ · ‖w‖⁻¹ − 1)` — and holding the reciprocal opaque leaves
    /// exactly that identity undischarged.
    #[test]
    fn a_reciprocal_cancels_because_the_form_is_a_quotient() {
        let (_, counts) = with_session(budget(), || {
            let x = p("w", 3.0);
            let y = p("h", 2.0);
            decides_zero((x / y) * y - x);
            // And the shape the kernel actually builds: a direction
            // normalized by a norm, re-metered by the same norm.
            let w = [p("a", 0.0), p("b", 0.0), p("d", 1.0)];
            let n = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
            let residual = w[2] / n * n - w[2];
            decides_zero(residual)
        });
        assert_eq!(counts.symbolic_zero, 2, "{counts:?}");
    }

    /// A poisoned expression never decides `Zero` however zero its form
    /// is: clause 1 of the theorem, on the value channel.
    #[test]
    fn a_domain_violation_never_certifies_symbolically() {
        let (out, _) = with_session(budget(), || {
            let neg = Sym::from_f64(-1.0);
            let r = neg.sqrt();
            decides_zero(r - r)
        });
        assert!(!out, "sqrt(-1) - sqrt(-1) is not a certified zero");
    }

    /// A budget of zero terms is the tier switched off inside the
    /// scalar: nothing is asked of the DAG at all.
    #[test]
    fn a_zero_budget_decides_everything_numerically() {
        let (_, counts) = with_session(SymBudget::none(), || {
            let x = p("w", 0.37);
            decides_zero(x - x)
        });
        assert_eq!(counts.symbolic_zero, 0);
        assert_eq!(counts.numeric, 1);
        assert_eq!(counts.frozen, 0, "nothing is even computed");
    }

    /// Freezing is SOUND, not silent: a form driven past the term
    /// budget decides numerically and the freeze is counted.
    #[test]
    fn an_over_budget_form_freezes_and_is_counted() {
        let tight = SymBudget {
            max_terms: 2,
            max_degree: 16,
        };
        let (out, counts) = with_session(tight, || {
            let (x, y, z) = (p("a", 1.0), p("b", 2.0), p("c", 3.0));
            let wide = x + y + z;
            // The sum has three terms: over budget, so it freezes into
            // an atom. The DIFFERENCE of two identical frozen atoms is
            // still zero, which is sound — same node, same real.
            let out = decides_zero(wide - wide);
            (out, session_counts())
        });
        assert!(out.0, "identical frozen nodes still cancel");
        assert!(counts.frozen >= 1, "the freeze is counted: {counts:?}");
        assert_eq!(
            out.1.map(|c| c.frozen),
            Some(counts.frozen),
            "the mid-replay door reports the same freezes the session ends with"
        );
    }

    /// D9: the node ids are content hashes, so two sessions building the
    /// same expression in different orders agree bit for bit, and no
    /// table is shared between them.
    #[test]
    fn node_ids_are_bit_identical_across_sessions_and_orders() {
        let build_forward = || {
            let x = p("w", 1.0);
            let y = p("h", 2.0);
            (x * y + x).node().bits()
        };
        let build_backward = || {
            let y = p("h", 2.0);
            let x = p("w", 1.0);
            let m = x * y;
            (m + x).node().bits()
        };
        let (a, _) = with_session(budget(), build_forward);
        let (b, _) = with_session(budget(), build_backward);
        let (c, _) = with_session(SymBudget::none(), build_forward);
        assert_eq!(a, b);
        assert_eq!(a, c, "the id does not depend on the budget");
        // And outside any session at all.
        assert_eq!(a, build_forward());
    }

    // ------------------------------------------- the registered door

    /// The arc-rim SHAPE the door exists for, in miniature: a "radius"
    /// `r`, a "rim vector" `v` whose norm the construction guarantees
    /// is `r`, and the residual a carrier's endpoint pinning produces —
    /// `v·(r/‖v‖ − 1)`, componentwise zero exactly when `‖v‖ = r`.
    ///
    /// `v` is built from parameters so its norm is a genuine `sqrt`
    /// atom, and `r` an `abs` atom, so the two are unrelated
    /// indeterminates to every rule the tier ships — which is the whole
    /// point: this residual is no theorem the tier can reach, and it is
    /// the plate's ceiling.
    fn rim(vx: f64, vy: f64, rv: f64) -> (Sym<f64>, Sym<f64>, [Sym<f64>; 2]) {
        let (x, y) = (p("vx", vx), p("vy", vy));
        let r = p("r", rv).abs();
        let n = (x * x + y * y).sqrt();
        let one = Sym::from_f64(1.0);
        let scale = r / n - one;
        (n, r, [x * scale, y * scale])
    }

    /// **How one margin was answered, read from the RECEIPT** — because
    /// the answer cannot say. At a point scalar the residual of a true
    /// identity is numerically zero as well, so every row below decides
    /// `Zero` whatever the tier does; what separates a theorem, an
    /// axiom and a band decision is which column the decision landed
    /// in, and that is what these rows read.
    fn how(m: Sym<f64>) -> &'static str {
        let before = session_counts().expect("inside a session");
        let _ = decides_zero(m);
        let after = session_counts().expect("inside a session");
        if after.registered > before.registered {
            "registered"
        } else if after.sign_gated > before.sign_gated {
            "sign_gated"
        } else if after.symbolic_zero > before.symbolic_zero {
            "theorem"
        } else {
            "numeric"
        }
    }

    /// **The door discharges the rim residual, and counts it apart.**
    /// Without the registration every component is a numeric decision;
    /// with it every one is `registered`, and `symbolic_zero` does not
    /// move by one.
    #[test]
    fn a_registered_identity_decides_zero_and_is_counted_apart() {
        let run = |register: bool| {
            with_session(budget(), || {
                let (n, r, resid) = rim(3.0, 4.0, 5.0);
                if register {
                    assert_eq!(
                        n.register_equal(r, Tol::witness()),
                        SymRegistration::Recorded
                    );
                }
                resid.map(how)
            })
        };
        let (how_off, off) = run(false);
        assert_eq!(how_off, ["numeric", "numeric"], "{off:?}");
        assert_eq!(off.registered, 0);
        let (how_on, on) = run(true);
        assert_eq!(how_on, ["registered", "registered"], "{on:?}");
        assert_eq!(on.registered, 2);
        assert_eq!(
            on.symbolic_zero, off.symbolic_zero,
            "the door moves decisions out of `numeric` and out of nothing else"
        );
        assert_eq!(on.numeric + 2, off.numeric);
    }

    /// **A GATED door form does not discharge**, and this is its pin.
    ///
    /// The door is asked LAST — plain form, early walk, the A/B
    /// reduction of the top residual, then the registry. But the walk
    /// the door runs is the EARLY one, and with rule C on
    /// (`SymRules::signed_root`) that walk can reach zero through a
    /// clause-3 SIGN READ, which is a conditional claim rather than an
    /// identity. A zero resting BOTH on a constructor's axiom and on a
    /// box-wise sign read is two weakenings at once, and the receipt
    /// has a column for each and none for the pair — so it falls to the
    /// numeric channel, which is the conservative direction.
    ///
    /// The pair below is one shape, twice. `z` and `y` are independent
    /// parameters of equal value. Registering `z = sqrt(y·y)` makes
    /// `z − y` reach zero only under rule C's fold, and the decision
    /// must stay NUMERIC; registering `z = y` makes the same margin the
    /// zero form outright, and it is `registered`. Same door, same
    /// registry, same margin — the gate is the only difference.
    #[test]
    fn a_gated_door_form_does_not_discharge() {
        let rules = SymRules {
            signed_root: true,
            ..SymRules::shipped()
        };
        let run = |gated: bool| {
            with_session_rules(budget(), rules, || {
                let y = p("y", 2.0);
                let z = p("z", 2.0);
                let to = if gated { (y * y).sqrt() } else { y };
                assert_eq!(
                    z.register_equal(to, Tol::witness()),
                    SymRegistration::Recorded,
                    "both registrations are witnessed at the point"
                );
                how(z - y)
            })
        };
        let (gated, gc) = run(true);
        assert_eq!(
            gated, "numeric",
            "a door form that is zero only under a clause-3 sign read is not a \
             discharge: {gc:?}"
        );
        assert_eq!(gc.registered, 0, "{gc:?}");
        let (plain, pc) = run(false);
        assert_eq!(
            plain, "registered",
            "and the same margin through an UNGATED registry is: {pc:?}"
        );
        assert_eq!(pc.registered, 1, "{pc:?}");
    }

    /// **The value channel is untouched**: every value in the residual
    /// is bit-identical with the registration and without it, so
    /// `u_ref` is still `v / ‖v‖`. The rejected cheaper spelling is the
    /// third row, and it differs in the bits — which is why it is
    /// rejected.
    #[test]
    fn a_registration_changes_no_value_and_the_cheap_spelling_would() {
        let bits = |register: bool| {
            with_session(budget(), || {
                let (n, r, resid) = rim(0.3, 0.4, 0.5000000001);
                if register {
                    let _ = n.register_equal(r, Tol::witness());
                }
                [resid[0].value.to_bits(), resid[1].value.to_bits()]
            })
            .0
        };
        assert_eq!(bits(false), bits(true), "no value moves");
        // The rejected spelling, at the same numbers: normalizing by the
        // DECLARED radius instead of by the computed norm.
        let cheap = with_session(budget(), || {
            let (x, y) = (p("vx", 0.3), p("vy", 0.4));
            let r = p("r", 0.5000000001).abs();
            let one = Sym::from_f64(1.0);
            let scale = r / r - one;
            [(x * scale).value.to_bits(), (y * scale).value.to_bits()]
        })
        .0;
        assert_ne!(
            cheap,
            bits(true),
            "the `(q - c) / r` spelling changes the numeric channel's bits, which is the \
             whole reason the door exists instead of it"
        );
    }

    /// **A lying registration is refused, typed, and the decisions stay
    /// numeric** — the planted `‖q − c‖ ≡ 2r`. This lane's witness is
    /// `f64`'s, which is inexact, so the arm is `Disputed`: the claim is
    /// false, and a comparison at a slack cannot say that it is.
    #[test]
    fn a_lying_registration_is_refused_typed() {
        let (how_, counts) = with_session(budget(), || {
            let (n, r, resid) = rim(3.0, 4.0, 5.0);
            let two_r = Sym::from_f64(2.0) * r;
            assert_eq!(
                n.register_equal(two_r, Tol::witness()),
                SymRegistration::Disputed,
                "5 is not 10, and the INEXACT witness at this lane says so at the point"
            );
            resid.map(how)
        });
        assert_eq!(how_, ["numeric", "numeric"]);
        assert_eq!(counts.registered, 0, "nothing was recorded: {counts:?}");
        assert_eq!(counts.numeric, 2);
    }

    /// **A registration made AFTER a consumer decided still discharges
    /// the next one** — the door invalidates the memo rather than
    /// silently missing (the order clause of `Sym::register_equal`).
    #[test]
    fn a_registration_after_a_decision_discharges_the_next_one() {
        let (rows, counts) = with_session(budget(), || {
            let (n, r, resid) = rim(3.0, 4.0, 5.0);
            let first = how(resid[0]);
            assert_eq!(
                n.register_equal(r, Tol::witness()),
                SymRegistration::Recorded
            );
            let second = how(resid[0]);
            // Idempotent, and a repeat invalidates nothing.
            assert_eq!(
                n.register_equal(r, Tol::witness()),
                SymRegistration::Already
            );
            [first, second]
        });
        assert_eq!(
            rows,
            ["numeric", "registered"],
            "asked before the registration and asked again after it: {counts:?}"
        );
        assert_eq!((counts.registered, counts.numeric), (1, 1));
    }

    /// **A registration that would close a cycle is refused, typed** —
    /// the property `form_in`'s termination argument rests on. `x·x`
    /// at `x = 1` has the same VALUE as `x`, so the witness passes and
    /// the cycle test is what refuses.
    #[test]
    fn a_cyclic_registration_is_refused_typed() {
        with_session(budget(), || {
            let x = p("w", 1.0);
            let bigger = x * x;
            assert_eq!(
                x.register_equal(bigger, Tol::witness()),
                SymRegistration::Cyclic
            );
            // The other direction is not a cycle: `bigger` contains
            // `x`, `x` does not contain `bigger`.
            assert_eq!(
                bigger.register_equal(x, Tol::witness()),
                SymRegistration::Recorded
            );
        });
    }

    /// **The door OFF is M10-8's tier**: the same decisions in the same
    /// columns, and a registration that records nothing and says so.
    #[test]
    fn the_door_off_records_nothing_and_reproduces_the_tier() {
        let (rows, counts) =
            with_session_rules(budget(), SymRules::shipped_without_the_door(), || {
                let (n, r, resid) = rim(3.0, 4.0, 5.0);
                assert_eq!(
                    n.register_equal(r, Tol::witness()),
                    SymRegistration::Witnessed
                );
                resid.map(how)
            });
        assert_eq!(rows, ["numeric", "numeric"]);
        assert_eq!((counts.registered, counts.numeric), (0, 2));
    }

    /// **The slack's SHAPE, away from the origin** — adopted from R1's
    /// SYM-6 review row `r1_the_slack_is_relative_and_floored_at_1e9`,
    /// because nothing else in the suite asserted the
    /// relative-and-floored spelling anywhere but near 1.
    ///
    /// Three claims at one scale, `a = 10⁹`, at whatever ε row the
    /// process runs at:
    ///
    /// - **RELATIVE**: a gap of `k · ε · a` is witnessed for `k` below
    ///   one and `Disputed` above it, so the slack tracks the magnitude
    ///   rather than a constant;
    /// - **FLOORED at one**: the same `k` sweep near zero is compared
    ///   ABSOLUTELY at ε, so the relative form does not shrink to no
    ///   slack at all where the magnitudes do;
    /// - **and a TRUE identity survives**: two values 1000 ULP apart at
    ///   10⁹ differ by ~1e-7, which an ABSOLUTE ε would refuse at the
    ///   1e-9 and 1e-12 rows. That refusal is the measurement that
    ///   killed the absolute spelling (CI run 34048088597), and this is
    ///   the row that keeps it dead.
    #[test]
    fn the_slack_is_relative_and_floored_at_1e9() {
        let tol = Tol::witness();
        let eps = tol.eps();
        let a = 1.0e9_f64;
        for (k, want) in [
            (0.99_f64, SymRegistration::Witnessed),
            (1.01_f64, SymRegistration::Disputed),
        ] {
            let b = a + k * eps * a;
            let got = <f64 as Real>::register_equal(a, b, tol);
            println!("   k={k} eps={eps:e} a={a:e} b-a={:e} -> {got:?}", b - a);
            assert_eq!(got, want, "k={k} at eps={eps:e}: the slack is k·ε·|a|");
        }
        // The floor: near zero the comparison is ABSOLUTE at ε.
        assert_eq!(
            <f64 as Real>::register_equal(1.0e-30, 1.0e-30 + 0.99 * eps, tol),
            SymRegistration::Witnessed,
            "inside the floor at eps={eps:e}"
        );
        assert_eq!(
            <f64 as Real>::register_equal(1.0e-30, 1.0e-30 + 1.01 * eps, tol),
            SymRegistration::Disputed,
            "outside the floor at eps={eps:e}"
        );
        // A true identity at 1e9 whose two sides differ by rounding only.
        let rounded = f64::from_bits(a.to_bits() + 1000);
        println!("   1000 ulp at 1e9 is {:e}", rounded - a);
        assert_eq!(
            <f64 as Real>::register_equal(a, rounded, tol),
            SymRegistration::Witnessed,
            "a true identity at 1e9 must be witnessed at eps={eps:e}; an absolute ε \
             refuses it, which is why the slack is relative"
        );
    }

    /// **Outside a session the claim is witnessed and nothing is
    /// recorded**, and at a bare scalar the door is a no-op that still
    /// answers — the `Real`-level hook's default and its overrides.
    #[test]
    fn the_hook_is_a_no_op_off_the_symbolic_scalar() {
        assert_eq!(
            <f64 as Real>::register_equal(1.0, 1.0 + 1e-15, Tol::witness()),
            SymRegistration::Witnessed
        );
        assert_eq!(
            <f64 as Real>::register_equal(1.0, 2.0, Tol::witness()),
            SymRegistration::Disputed,
            "an INEXACT witness never answers Contradicted"
        );
        assert_eq!(
            <f64 as Real>::register_equal(f64::NAN, 1.0, Tol::witness()),
            SymRegistration::Unwitnessed
        );
        // Outside `with_session` there is no table to record in.
        let a = Sym::<f64>::from_f64(2.0);
        let b = Sym::<f64>::from_f64(2.0);
        assert_eq!(
            a.register_equal(b, Tol::witness()),
            SymRegistration::Witnessed
        );
    }

    /// **Claim 9 — the axiom agrees with the tier where the tier can
    /// reach it.** Written as polynomials the squared identity
    /// `‖v‖² − r²` IS a plain-form theorem, with no registration
    /// anywhere; written through the root — `sqrt(X)·sqrt(X) − X`, the
    /// shape the construction actually produces — it is a theorem of
    /// rule A per node, and with the algebra off it is not, because the
    /// atom is then opaque. That gap is exactly what the door states
    /// for the UNSQUARED identity, and the rows here are what make the
    /// axiom consistent with the tier rather than merely asserted.
    #[test]
    fn the_squared_identity_is_a_plain_form_theorem() {
        let (row, _) = with_session(budget(), || {
            let (x, y) = (p("vx", 3.0), p("vy", 4.0));
            how((x * x + y * y) - (x * x + y * y))
        });
        assert_eq!(row, "theorem", "the squared identity, as polynomials");
        let (row, _) = with_session(budget(), || {
            let (x, y) = (p("vx", 3.0), p("vy", 4.0));
            let n2 = x * x + y * y;
            let root = n2.sqrt();
            how(n2 - root * root)
        });
        assert_eq!(
            row, "theorem",
            "through the root, rule A per node reaches it"
        );
        let (row, _) = with_session_rules(budget(), SymRules::without_the_algebra(), || {
            let (x, y) = (p("vx", 3.0), p("vy", 4.0));
            let n2 = x * x + y * y;
            let root = n2.sqrt();
            how(n2 - root * root)
        });
        assert_eq!(
            row, "numeric",
            "with the algebra off the atom is opaque and the root form stays numeric"
        );
    }

    /// The arc carrier's SECOND same-object identity, in miniature —
    /// the SPAN identity `carrier.eval(θ) = q_to` (M10-9 amendment A1;
    /// `sweep::swept::register_span_identity`). The far endpoint is
    /// reached by rotating the rim vector through the span, so the
    /// residual carries `cos`/`sin` atoms of `4·atan|b|` that no rule
    /// relates to the polynomial `q_to − c` is: it is registered per
    /// COMPONENT, because the consumer asks
    /// `carrier.eval(t1).distance(end)`.
    ///
    /// Answers the three pairs `[eval(θ), q_to, eval(θ) − q_to]`.
    fn span(theta: f64, off: f64) -> [[Sym<f64>; 2]; 3] {
        let (vx, vy) = (p("vx", 3.0), p("vy", 4.0));
        let b = p("b", theta);
        let (sn, cs) = (Sym::from_f64(4.0) * b.abs().atan()).sin_cos();
        // The rotated rim vector, as a circle carrier's `eval` builds
        // it, plus the centre.
        let (cx, cy) = (p("cx", 1.0), p("cy", -2.0));
        let (px, py) = (cx + (vx * cs - vy * sn), cy + (vx * sn + vy * cs));
        // The far vertex the construction actually holds, and it is
        // built INDEPENDENTLY — a lamina vertex, not a function of the
        // carrier — which is what makes the registration an axiom
        // rather than a tautology, and what keeps it out of the cyclic
        // arm. Its value is the same real, computed the same way;
        // `off` displaces it into a claim that is FALSE.
        let (s0, c0) = (4.0 * theta.abs().atan()).sin_cos();
        let (qx, qy) = (
            p("qx", 1.0 + (3.0 * c0 - 4.0 * s0) + off),
            p("qy", -2.0 + (3.0 * s0 + 4.0 * c0) + off),
        );
        [[px, py], [qx, qy], [px - qx, py - qy]]
    }

    /// **The span identity discharges the far endpoint's residual, per
    /// component, and a planted lie about it is refused typed.**
    #[test]
    fn the_span_identity_discharges_and_its_planted_lie_is_refused() {
        let (rows, counts) = with_session(budget(), || {
            let [p_end, q_to, resid] = span(0.4, 0.0);
            for (a, b) in p_end.into_iter().zip(q_to) {
                assert_eq!(
                    a.register_equal(b, Tol::witness()),
                    SymRegistration::Recorded
                );
            }
            resid.map(how)
        });
        assert_eq!(rows, ["registered", "registered"], "{counts:?}");
        assert_eq!(counts.symbolic_zero, 0, "no rule reaches this one");
        // The planted lie: the same registration against a far vertex
        // displaced by a geometric amount.
        let (rows, counts) = with_session(budget(), || {
            let [p_end, q_to, resid] = span(0.4, 1.0e-3);
            for (a, b) in p_end.into_iter().zip(q_to) {
                assert_eq!(
                    a.register_equal(b, Tol::witness()),
                    SymRegistration::Disputed,
                    "the inexact witness separates a displaced far vertex"
                );
            }
            resid.map(how)
        });
        assert_eq!(rows, ["numeric", "numeric"]);
        assert_eq!(counts.registered, 0, "nothing recorded: {counts:?}");
    }

    /// **D9**: the registry is content-hash keyed, so two runs of the
    /// same leaf register the same ids and count the same — and the
    /// node the registrant registers IS the node the consumer built
    /// (the same-object condition, testable because ids are content
    /// hashes).
    #[test]
    fn the_registration_is_deterministic_and_the_ids_are_the_consumers() {
        let run = || {
            with_session(budget(), || {
                let (n, r, resid) = rim(3.0, 4.0, 5.0);
                let _ = n.register_equal(r, Tol::witness());
                // The registrant's node, recomputed: `Vec3::norm` is
                // `norm_squared().sqrt()` and ids are content hashes, so
                // the consumer's divisor is the very node registered.
                let (x, y) = (p("vx", 3.0), p("vy", 4.0));
                let again = (x * x + y * y).sqrt();
                assert_eq!(again.node().bits(), n.node().bits());
                (resid.map(how), n.node().bits())
            })
        };
        let a = run();
        let b = run();
        assert_eq!(a, b, "identical across repeats");
        assert_eq!(a.0.0, ["registered", "registered"]);
    }

    // ------------------------------------------------ rule D (M10-10)

    /// `atan X` at a point `x`, with `X` a parameter so the atom is a
    /// function of a symbol and nothing folds on a constant.
    fn atan_of(x: f64) -> (Sym<f64>, Sym<f64>) {
        let x = p("bulge", x);
        (x, x.atan())
    }

    /// **Rule D decides the closed forms of `sin`/`cos` at `k·atan X`
    /// for `k ∈ {1, 2, 3, 4}`** — each against the closed form spelled
    /// by hand through the scalar's own ops, at a positive, a negative
    /// and a zero-valued `X` — and each is counted a THEOREM: no value
    /// read, so `symbolic_zero` and nothing else.
    #[test]
    fn rule_d_decides_the_multiples_against_their_closed_forms() {
        for xv in [1.0, -0.6, 2.75, 0.0] {
            let (rows, counts) = with_session(budget(), || {
                let (x, phi) = atan_of(xv);
                let one = Sym::from_f64(1.0);
                let s = (one + x * x).sqrt();
                let q = one + x * x;
                let k = |n: f64| Sym::from_f64(n) * phi;
                let mul = |n: f64| Sym::from_f64(n);
                [
                    // sin φ = X/S, cos φ = 1/S.
                    how(k(1.0).sin_cos().0 - x / s),
                    how(k(1.0).sin_cos().1 - one / s),
                    // sin 2φ = 2X/(1+X²), cos 2φ = (1−X²)/(1+X²).
                    how(k(2.0).sin_cos().0 - mul(2.0) * x / q),
                    how(k(2.0).sin_cos().1 - (one - x * x) / q),
                    // sin 3φ = (3X − X³)/((1+X²)·S), cos 3φ = (1 − 3X²)/((1+X²)·S).
                    how(k(3.0).sin_cos().0 - (mul(3.0) * x - x * x * x) / (q * s)),
                    how(k(3.0).sin_cos().1 - (one - mul(3.0) * x * x) / (q * s)),
                    // sin 4φ = 4X(1−X²)/(1+X²)², cos 4φ = (1 − 6X² + X⁴)/(1+X²)².
                    how(k(4.0).sin_cos().0 - mul(4.0) * x * (one - x * x) / (q * q)),
                    how(k(4.0).sin_cos().1 - (one - mul(6.0) * x * x + x * x * x * x) / (q * q)),
                ]
            });
            assert_eq!(rows, ["theorem"; 8], "at X = {xv}: {counts:?}");
            assert_eq!(
                (counts.sign_gated, counts.registered),
                (0, 0),
                "a rule-D zero reads no value and rests on no axiom: {counts:?}"
            );
        }
    }

    /// **Halves and quarters, on the positive branch**: the identities
    /// a half-angle satisfies against the whole angle — `sin φ =
    /// 2·sin(φ/2)·cos(φ/2)`, `cos φ = 2·cos²(φ/2) − 1`, `cos(φ/2) =
    /// 2·cos²(φ/4) − 1` — and the pushforward's own spelling `cos(sθ)
    /// − 1 = −2·sin²(sθ/2)` at `θ = 4·atan X`, `s = i/8`, for every
    /// sample of the certifier's schedule, decide `Zero` at a positive
    /// and a NEGATIVE `X`. The sign of the half-angle's sine rides in
    /// `X` as a form; nothing here reads it.
    #[test]
    fn rule_d_decides_the_halves_on_the_positive_branch() {
        for xv in [0.8, -1.0, 3.5] {
            let (rows, counts) = with_session(budget(), || {
                let (_, phi) = atan_of(xv);
                let one = Sym::from_f64(1.0);
                let two = Sym::from_f64(2.0);
                let half = phi * Sym::from_f64(0.5);
                let quarter = phi * Sym::from_f64(0.25);
                let mut rows = vec![
                    how(phi.sin_cos().0 - two * half.sin_cos().0 * half.sin_cos().1),
                    how(phi.sin_cos().1 - (two * half.sin_cos().1 * half.sin_cos().1 - one)),
                    how(half.sin_cos().1 - (two * quarter.sin_cos().1 * quarter.sin_cos().1 - one)),
                ];
                // The pushforward's spelling at every schedule sample.
                let theta = Sym::from_f64(4.0) * phi;
                for i in 0..=8 {
                    let s = Sym::from_f64(f64::from(i) / 8.0);
                    let cos_m1 = -(two * (s * theta * Sym::from_f64(0.5)).sin_cos().0.powi(2));
                    rows.push(how(((s * theta).sin_cos().1 - one) - cos_m1));
                }
                rows
            });
            assert!(
                rows.iter().all(|r| *r == "theorem"),
                "at X = {xv}: {rows:?} {counts:?}"
            );
            assert_eq!((counts.sign_gated, counts.registered), (0, 0));
        }
    }

    /// **The two spellings of one arc meet.** The certifier's carrier
    /// sample `cos t`, `sin t` at `t = 4·atan|b|·(i/8)` against the
    /// pushforward's `sin(s·θ)` and `1 − 2·sin²(s·θ/2)` at `θ =
    /// 4·atan b`, `s = i/8`, for a bulge that is a LITERAL (the circle
    /// kernel's `1`, so `|b|` folds under A0) and for a parameter
    /// bulge — where `atan|b|` and `atan b` are two atoms and the
    /// residual stays numeric, which is the honest limit this rule
    /// draws: the turn sign the carrier's axis carries is a `Sign`,
    /// not a form.
    #[test]
    fn rule_d_meets_the_carrier_and_the_pushforward_at_every_sample() {
        let (rows, _) = with_session(budget(), || {
            let b = Sym::from_f64(1.0);
            let theta = Sym::from_f64(4.0) * b.atan();
            let span = Sym::from_f64(4.0) * b.abs().atan();
            let one = Sym::from_f64(1.0);
            let two = Sym::from_f64(2.0);
            (0..=8)
                .map(|i| {
                    let s = Sym::from_f64(f64::from(i) / 8.0);
                    let t = Sym::zero() + (span - Sym::zero()) * s;
                    let (st, ct) = t.sin_cos();
                    let sin = (s * theta).sin_cos().0;
                    let cos_m1 = -(two * (s * theta * Sym::from_f64(0.5)).sin_cos().0.powi(2));
                    (how(st - sin), how(ct - (cos_m1 + one)))
                })
                .collect::<Vec<_>>()
        });
        assert!(
            rows.iter().all(|r| *r == ("theorem", "theorem")),
            "literal bulge: {rows:?}"
        );
        let (rows, _) = with_session(budget(), || {
            let b = p("bulge", 0.7);
            let theta = Sym::from_f64(4.0) * b.atan();
            let span = Sym::from_f64(4.0) * b.abs().atan();
            let s = Sym::from_f64(3.0 / 8.0);
            let (st, _) = (span * s).sin_cos();
            how(st - (s * theta).sin_cos().0)
        });
        assert_eq!(
            rows, "numeric",
            "a parameter bulge: `atan|b|` and `atan b` are two atoms, and no rule here \
             reads the sign that would relate them"
        );
    }

    /// **Nothing folds at an argument that is not `q·atan(X)`**: an
    /// `atan2`, an `atan` plus a constant, a non-dyadic multiple, a
    /// product of two `atan`s — each a TRUE identity of the reals that
    /// rule D must leave to the numeric channel, because the closed
    /// form it states is not the one that holds there.
    #[test]
    fn rule_d_folds_nothing_at_any_other_argument() {
        let (rows, counts) = with_session(budget(), || {
            let (x, phi) = atan_of(0.9);
            let one = Sym::from_f64(1.0);
            let s = (one + x * x).sqrt();
            let c = Sym::from_f64(0.3);
            [
                // atan2(X, 1) = atan X, but the op is `Atan2`.
                how(x.atan2(one).sin_cos().0 - x / s),
                // sin(φ + c) = sin φ·cos c + cos φ·sin c: `c` is opaque.
                how((phi + c).sin_cos().0 - (x / s * c.sin_cos().1 + one / s * c.sin_cos().0)),
                // A non-dyadic multiple has no closed form here.
                how((phi / Sym::from_f64(3.0)).sin_cos().1 - (phi / Sym::from_f64(3.0)).sin_cos().1),
                // A product of two atans is degree two in the atom.
                how((phi * phi).sin_cos().0 - (phi * phi).sin_cos().0),
            ]
        });
        assert_eq!(rows[0], "numeric", "atan2 never folds: {counts:?}");
        assert_eq!(
            rows[1], "numeric",
            "atan plus a constant never folds: {counts:?}"
        );
        // The last two are `a − a`: the zero form by node identity,
        // which is a theorem whether or not the atom folds — they are
        // here so the shapes are exercised, and what they pin is that
        // no fold PANICS or POISONS on them.
        assert_eq!(&rows[2..], ["theorem", "theorem"]);
    }

    /// **Rule D's second fold: `atan2(0, N) = 0` for an `N` non-negative
    /// BY SYNTAX** — a `sqrt` atom, an even power, and the chart phase's
    /// own `r²/sqrt(r²)` with `r = nominal + δ` (a perfect square over a
    /// `sqrt` atom) decide `Zero` as theorems; and it NEVER folds at
    /// `atan2(0, X)` for a plain parameter, at `atan2(Y, N)` with `Y` a
    /// numeric zero that is not the zero form (a coincidence of two
    /// parameters at one nominal), or at `atan2(0, 0)` as a form.
    #[test]
    fn rule_d_folds_atan2_of_the_zero_form_over_a_manifestly_nonnegative_form_and_nothing_else() {
        let (rows, counts) = with_session(budget(), || {
            let x = p("x", 0.37);
            let d = p("delta", 1.0e-5);
            let zero = Sym::zero();
            let r = Sym::from_f64(1.25e-3) + d;
            let r2 = r * r;
            [
                how(zero.atan2(x.sqrt())),
                how(zero.atan2(x * x)),
                how(zero.atan2(r2 / r2.sqrt())),
                how(zero.atan2(x.abs() * x.sqrt() + x * x)),
            ]
        });
        assert_eq!(rows, ["theorem"; 4], "{counts:?}");
        assert_eq!((counts.sign_gated, counts.registered), (0, 0));
        let (rows, _) = with_session(budget(), || {
            let x = p("x", 0.37);
            let y = p("y", 0.0);
            let x2 = p("x2", 0.37);
            let zero = Sym::zero();
            [
                // A plain parameter has no sign the form knows.
                how(zero.atan2(x)),
                // A numeric zero in the first slot is not the zero form.
                how(y.atan2(x.sqrt())),
                // Two parameters equal at the nominal: a coincidence.
                how((x - x2).atan2(x.sqrt())),
                // `atan2(0, 0)` as a form claims nothing.
                how(zero.atan2(zero)),
                // An odd power in a positive-coefficient sum.
                how(zero.atan2(x * x + x)),
            ]
        });
        assert_eq!(rows, ["numeric"; 5]);
    }

    /// **Rule D's third fold: `sin`/`cos` at an exact half-multiple of
    /// π** — `cos π = −1`, `sin π = 0`, `cos(π/2) = 0`, `sin(3π/2) =
    /// −1`, `cos(2π) = 1` decide as theorems; `cos(π/3)` and
    /// `cos(π + atan X)` never fold.
    #[test]
    fn rule_d_folds_trig_at_half_multiples_of_pi_and_nothing_else() {
        let (rows, counts) = with_session(budget(), || {
            let pi = Sym::<f64>::pi();
            let one = Sym::from_f64(1.0);
            let half = Sym::from_f64(0.5);
            [
                how(pi.sin_cos().1 + one),
                how(pi.sin_cos().0),
                how((pi * half).sin_cos().1),
                how((pi * Sym::from_f64(1.5)).sin_cos().0 + one),
                how(Sym::tau().sin_cos().1 - one),
            ]
        });
        assert_eq!(rows, ["theorem"; 5], "{counts:?}");
        assert_eq!((counts.sign_gated, counts.registered), (0, 0));
        let (rows, _) = with_session(budget(), || {
            let pi = Sym::<f64>::pi();
            let x = p("x", 0.3);
            // A value-equal spelling on the other side keeps each row
            // a genuine identity that must stay NUMERIC.
            [
                how((pi / Sym::from_f64(3.0)).sin_cos().1 - Sym::from_f64(0.5)),
                how((pi + x.atan()).sin_cos().1 + x.atan().sin_cos().1),
            ]
        });
        assert_eq!(rows, ["numeric"; 2]);
    }
}
