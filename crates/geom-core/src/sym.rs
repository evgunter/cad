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
//! closes the ring behind rule D (the M10-10 section below). No rule
//! folds an atom on a sign of the box: the one that did (M10-8's rule
//! C, `sqrt(X) = R` where `X = R²` and `R`'s sign is certified over the
//! box) folded on no measured document and is retired (DECIDE-10),
//! because a sign of the box is not a decision the construction takes.
//!
//! # The registered-identity door (M10-9)
//!
//! **What the tier cannot prove, a constructor can state.** E12 keeps
//! a recourse in reserve for exactly the rim residual above —
//! discharge by PROVENANCE — and [`Sym::register_equal`] is it: a
//! record that two DAG nodes denote one function of the parameters,
//! read by the node that made it and the nodes it feeds
//! ([`MintScope::node`]), made by the site that GUARANTEES it
//! (`sweep::swept::register_rigidity`, whose doc comment carries
//! the proof), verified in the lane scalar at the moment it is made,
//! refused typed when it lies, and consulted by a THIRD normal-form
//! walk beside the plain one and the early one.
//!
//! A registered `Zero` rests on the registrant's argument as well as on
//! exact rational arithmetic: zero wherever the construction takes the
//! decisions it took AND the registrant's theorem holds. The witness
//! does not establish that theorem — it refuses only a lie visible at
//! the point or one whose two certified enclosures are disjoint over the
//! box, and it cannot tell an identity from a coincidence (ERROR-DESIGN
//! E12) — so the zero is exactly as good as the proof in the
//! registrant's doc comment, which is why the call sites are an
//! allowlist (below). It is counted with every other zero
//! ([`SymCounts::symbolic`]). The door is asked only where the plain
//! form and the early form have both declined, and what it buys a
//! document is the differential with [`SymRules::registered`] off.
//!
//! **The unit of scope is the CONSTRUCTOR, not the identity.** Each
//! registrant states the identities ITS construction guarantees about
//! nodes it builds identically to the consumer's, and the registry's
//! alias is transitive, so facts stated at two constructions chain. A
//! profile arc's construction (the path door's, through
//! `Arc2::register_endpoints`) states the endpoint facts its algebra
//! proves in the sketch: the rim at each end is the radius, and the
//! carrier's end is the far vertex. The sweep that places the arc states only rigidity
//! (`sweep::swept::register_rigidity`, `register_placed_carrier_end`):
//! the placed rim is the sketch rim, and the placed carrier at its span
//! is the sketch carrier's end, placed — the latter same-object because
//! `Curve3::eval`'s `Circle` arm DELEGATES to a `Real`-bounded door
//! (`Curve3::circle_at`) the registrant calls too, so the node it
//! states the identity about is the node the certifier asks about.
//!
//! The revolve's latitude carriers (`sweep::revolve::surfaces` and
//! `::full`) state nothing: their `u_ref` is the revolve's seam
//! direction, so a latitude through its point is an identity the plain
//! form proves.
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
//! at `θ = 4·atan b` (the lowering's stored sweep) and the carrier's
//! `cos t`, `sin t` at `t = (i/8)·σ·θ` (the span, the sweep signed by
//! the decided turn σ) — are then rational functions of the same atoms,
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
//! **What it reached on the plate, at the nominal** — a record, in the
//! receipt's columns of the day (theorem / gated / registered /
//! numeric), whose first three DECIDE-10 made one, `symbolic`
//! (`m10_10_pins_interval` pins the sums): `carrier_on_surface_2`
//! 108/0/0/72 → 180/0/0/0 and `witness_on_surface_2` 12/0/0/8 →
//! 20/0/0/0 as THEOREMS; `carrier_matches_mapped_source` 180/0/16/56 →
//! 180/0/72/0, every sample through the DOOR — rule D makes the trig
//! meet, and the rim identity `‖q − c‖ = r` the registrant states is
//! what closes it, so the count was `registered` then. The fourth
//! residual the staged walk names, `pcurve_map_residual`, 0/0/0/36 →
//! 0/0/36/0 through the door as well: it carries the chart's phase
//! `atan2(0, r²/sqrt(r²))` from the cylinder chart derivation
//! (`pcurve_cache::stable_azimuth`, whose `u_ref` on the extrude's
//! wall is the start's own radial), which the A1 fold takes as the
//! zero form — `atan2` of the zero form over a form non-negative by
//! syntax, positive wherever the arc exists, and the `r² = 0` box is
//! one clause 1 refuses first — and on the far side of the window's cut
//! line (`pcurve_chart_cut_side`'s other side) the azimuth's `+ π`
//! leaves `cos π = −1`; the rim identity closes
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
//! — widened by ~6e-5 on each side (the extrude's closing pcurve mint
//! now refuses the plate before the assertion is read, at `3.9e2·ε`:
//! `work/sym/pcurve-certificate-checks-widen-past-the-band-over-a-parameter-box.md`,
//! pinned at `m10_10_pins_interval::m10_10_the_plates_web_margin_is_real_and_the_closing_mint_refuses_first`);
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
//! And the reach was measured at the UNIT bulge: a parameter bulge was
//! outside the mechanism while the carrier's span was spelled `atan|b|`
//! (R2's D-tab, `3.52e2 · ε` on and off alike; the span now shares the
//! pushforward's atom and the nominal split did not move) and a literal
//! bulge other than 1 leaves residue — at M10-10 R1's boss at bulge 2
//! stood at `carrier_matches_mapped_source` 6 of 54 and
//! `carrier_on_surface_2` 27 of 90 numeric with its ceiling unmoved;
//! rule E has since taken the first six and eighteen of the
//! twenty-seven, rule G the other nine, and rule G's exact quotient the
//! boss's last value-free residual (`arc_span`), so the boss's ceiling
//! is `0.5024`, `0.7267` and `0.7271` of its REAL study at ε = 1e-6,
//! 1e-9 and 1e-12, bounded by a real margin (`dihedral_wedge`). What stands at a parameter bulge is the sign of
//! `b` and of the apothem, and freezes: the ring at `fl(0.4)`, and the
//! term and degree budgets on the dyadic `0.5` control —
//! `work/decide/rule-d-reaches-the-unit-bulge-only`.
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
//! SIGN OF THE BOX: `|x| − x` on a nonnegative `x` is zero only by a fact
//! about the bracket, not by a decision the construction takes, and no
//! rule folds it (M10-8's rule C did, and is retired). These are limits
//! of the tier and not bugs in it — over-refusal is the safe direction,
//! and every such margin falls to the numeric channel exactly as
//! before. What the PLAIN form alone reaches is
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
//! `work/decide/rule-d-reaches-the-unit-bulge-only` measured a 512-bit
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
//! **Where it sits against A/B/D/E.** At the node, in `combine`,
//! early walk only, after A0's exact constant fold.
//! Against rules A/B and E the order is STRUCTURAL rather than chosen:
//! they run after `combine` returns, on the form this rule left, and an
//! atom this rule prevents from being minted is not one they could have
//! folded later. The walk ledger
//! (`editor-core/tests/m10_sym_profile_interval`) is unmoved by the
//! rule on the slab and the plate — every form either walk builds is
//! digest-identical — which is the same statement as "it fires nowhere
//! on them", and is not a pin on the order.
//!
//! **What it reached, measured on Duff's basis** (a record, in SYM-8's
//! columns: `symbolic_zero` is `symbolic` now) on the tilt-`u` derived
//! frame (`editor-core/tests/m10_derived_frame_tilted_interval`'s
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
//! 7.8 → 1.2 s). The end cap itself is bit-identical under the arm.
//!
//! **Those numbers are Duff's basis's, and that basis is gone.**
//! [`Vec3::orthonormal_basis`](crate::Vec3::orthonormal_basis) now
//! crosses the normal with a world axis and transfers no sign, so it
//! mints no `copysign(1, n.z)` for either arm to fold. On every document
//! above, and on the measured eight, shutting rule F's two arms moves
//! nothing: the start cap and `FlipZ` certify as the end cap does with
//! the arms on or off (`m10_derived_frame_tilted_interval`'s
//! `m10_the_start_cap_and_flip_z_certify_as_the_end_cap_does_and_rule_f_is_inert`
//! gates that). The arm has no measured document consumer
//! (`work/sym/the-negative-arm-lost-its-document-consumer`).
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
//! stops on (`sym12_rule_f_is_inert_on_the_reviews_negative_nz_documents`
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
//! **What it moved on the measured documents: nothing, with one
//! exception** — a record, in SYM-8's receipt columns (`symbolic_zero`
//! and `registered` are one column, `symbolic`, since DECIDE-10).
//! Every per-predicate split at the nominal is BIT-IDENTICAL with the
//! rule on and off on seven of the eight (plate, annulus, bracket,
//! link, R1's segment boss, both D-tabs; the
//! pad's nominal split with the shape report installed exhausts the
//! measuring box's memory at BOTH dials and is not takeable there),
//! and every whole-certifying ceiling is identical to the digit on all
//! EIGHT, with the over-band set at ceiling + δ identical too. The
//! exception is the pad's replay at the scale it certifies whole at:
//! `symbolic_zero` 889 → 885, `registered` 104 → 128, `numeric`
//! 1078 → 1058, `frozen` 2750 either way — the same 2071 decisions, 24
//! of them moving into the door, twenty out of `numeric` and FOUR out
//! of `symbolic_zero` (the rows' stored values; the replay's
//! `numeric` and `frozen` currently measure off them by a drift not
//! yet attributed,
//! `work/sym/ignored-sym-receipt-rows-drifted-red-on-main-unattributed`).
//! In today's single column the receipt reads `symbolic` 993 → 1013
//! and `numeric` 1078 → 1058: the four do not show. What they record is
//! a fact about the WALKS, not the receipt — opening an atom the early
//! walk was cancelling OVER can cost that walk a cancellation, which is
//! `work/sym/coefficient-ring-width-is-not-monotone-in-reach`'s class —
//! and no decision is lost: the door re-takes all four.
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
//! [`SymRules::without_rules_f_g_and_the_read`] is SYM-5's tier bit for bit;
//! [`SymRules::without_rule_f`] shuts rule F alone.
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
//! # Rule G and the decision read (DECIDE-3)
//!
//! **Rule G — the canonical square root** ([`SymRules::canonical_root`]):
//! a `Sqrt` atom's key is a function of its argument's VALUE CLASS, so
//! the normal's own root `S = sqrt(P)` and a candidate norm's
//! `sqrt(1/S²)` are ONE indeterminate and cancel, where the first cut
//! of this tier kept them as two and left the number one standing in
//! every denominator of a refused residual. [`root`] owns the whole of
//! it: the quotient split, the side condition it rests on and the
//! adversary that decided what may not be a source. Its companion
//! rewrite `|X|² = X²` lives in [`algebra`] beside rule A and behind
//! rule G's dial, because nothing mints an `Abs` where a root used to
//! stand until rule G does.
//!
//! **Rule G's exact quotient** ([`SymRules::root_quotient`]): a root
//! whose argument's denominator divides its numerator exactly is
//! minted over the polynomial quotient — rule E's cancellation carried
//! past the monomial, at the one door every root goes through. It is
//! what meets R1's boss's `arc_span` identity, whose root carries the
//! chord's polynomial to the fourth power in both halves. It trades a
//! meeting for it: the re-keyed root no longer meets the split
//! spelling `sqrt(N)/sqrt(D)` of the same value. [`root`] carries the
//! argument, the trade and what it does not reach.
//!
//! **The decision read** ([`SymRules::decision_read`]): a `Select`
//! whose decision is certified one-signed over the leaf's box takes
//! that arm, and so does a `min`/`max` whose comparison is — `max(A,
//! B)` IS `select(B − A, A, B)`. It is ordered BEHIND every value-free
//! fold AT ITS NODE and answers there: two nodes whose early forms are
//! equal mint one atom with the read shut and cancel, while with it on
//! each is read first and the same zero is reached through their arms.
//! A zero is a zero either way. At an exact witness the arm the read
//! takes is the arm the construction takes at every point of the box
//! (at an inexact one the read is not asked: `Session::witness`), so a
//! zero through it is E12's claim — zero wherever the construction takes the decisions
//! it took — like every other discharge, and the receipt counts it
//! with them; what the read buys a document is the differential
//! against [`SymRules::without_the_reads`]. [`signed`] owns the
//! enclosure.
//!
//! # Node ids are CONTENT HASHES (D9)
//!
//! A node's id is a 128-bit structural hash of `(op, children ids,
//! payload bits)` — never a sequence number. An id is therefore the same
//! under every rayon schedule and every insertion order, structural
//! sharing is free, and two builds of the same expression memoize the
//! same normal form. Outside a drive the hash-consing table is the
//! session's ([`with_session`]) and is dropped with it. The early memo,
//! the per-node reduction's memo (`reduce_per_node`) and the parameter
//! brackets are a leaf's; the registry, the door memo and the retry
//! memos are a document node's and are dropped with its scope
//! ([`MintScope::node`]).
//!
//! **The expression table, the plain forms and the atom records are per
//! DRIVE when a drive installs a table** ([`DriveMemo`],
//! [`with_session_memo`] and [`with_session_memo_retry`]). The table
//! serves the plain walk and nothing else — never the early or door
//! walk, and never a retry's, and the plain rung is the one rung no
//! retry re-asks. A node's
//! plain form is a function of its id, the budget and the two dials the
//! plain walk consults, and of nothing else: the walk reads no value,
//! every atom in it is opaque, and rule A0 is the only rule. So a form
//! one leaf built is the form every other leaf of that drive would
//! build — the argument this section already makes for two occurrences
//! of a node inside one leaf, applied across leaves. `DriveMemo`'s own
//! header carries the argument whole, the one premise that is not a
//! content hash (an `Opaque` id is the name it was minted under,
//! [`mint`]), and the lock discipline.
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
//! **A freeze is not the end of the ladder: a refused decision may
//! RETRY** ([`SymRetry`]). The ladder makes one attempt per rung, and a
//! decision every rung refuses is re-asked — on the early, top-residual
//! and door rungs only, never the plain one — with a rule that opens an
//! atom shut, or at a wider coefficient bound, each attempt in its own
//! memos.
//!
//! The argument is this section's own, read backwards. Freezing is
//! sound because an indeterminate denotes an unknown function, and it
//! is also a REACH mechanism: two spellings that freeze at the same
//! node cancel through it, without the ring ever seeing what the node
//! was. So opening a node — by widening the ring, or by a rule that
//! folds the atom — can LOSE a discharge the frozen node gave, and
//! widening the tier is not monotone in what it proves
//! (`work/sym/coefficient-ring-width-is-not-monotone-in-reach`; R1's
//! boss loses ten decisions to an `abs` fold, R2's link sixteen of one
//! predicate to rule G). A LADDER cannot lose one, because the first
//! attempt has already answered wherever it can and the second is asked
//! only into its silence — so `symbolic` can only rise and `numeric`
//! can only fall.
//!
//! Each attempt is sound on its own terms: the ring's bound is a cost
//! and not a soundness condition (the integers are exact at every
//! width), and a rule set with fewer rules is a subset of the same
//! algebra. A retry's zero is therefore the same claim as the first
//! attempt's and lands in the same column, with
//! [`SymCounts::retried`] counting it beside.
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
//! `work/tier/symbolic-tier-costs-95-percent-of-the-m10-3-drive`.
//!
//! **Who asks for the forms.** The `Decide` impl has two callers of
//! the walks and only one is the tier deciding: its DECISION path asks
//! a form only where the numeric channel cannot answer, and the shape
//! report, when installed, renders blocked residuals through the walks.
//! A definite numeric sign asks the tier nothing; the test instrument
//! (`sym::contradiction`) walks the margins a build decided definite,
//! after the build, and no shipped build contains it. The instruction
//! and form counts below are an UPPER bound on the tier's cost: each
//! includes a walk of every definite margin (918 plain and 1,958 early
//! forms on the slab's nominal, 1.57 M of the chamber drive's 19.1 M
//! plain forms), which no build pays
//! (`work/tier/symbolic-tier-costs-95-percent-of-the-m10-3-drive` owes
//! the re-take). The plain walk's freezes are the drive's frozen SET
//! whoever asked for them ([`SymCounts::frozen`]).
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
//! per-node rule A/B reduction inside the EARLY walk, which reads the
//! leaf's parameter brackets and stays per leaf. And a boss on a derived
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
//! rules A/B per node, the largest share of the replay when it was
//! taken, before the per-node reduction was memoised per session
//! (`reduce_per_node`) — and the freeze population
//! is what the budget note in `editor-core`'s `SymbolicDials` says it
//! is: 1,312 freezes over the three walks (1,044 in the plain walk),
//! **1,032 on DEGREE** with the kids already at total degree 40–117 in
//! one to seven terms, 280 on the coefficient bound (widest refused
//! 401 bits against [`rational::COEFF_BITS`]), and none on the term budget —
//! no form on either document comes within 40× of it. Rule D's fold is
//! 0.4 % of the plate's replay; the ring's heap path is 9 % of its
//! operations, so the `i128` inline path holds on both documents.
//!
//! **What the RETRY LADDER costs, and where** ([`SymRetry`]; SYM-9's
//! Phase 1 tables are in the unit's PR). A retry is paid ONLY on a
//! decision every rung of the first attempt refused, so the cost is a
//! second walk per refusal and nothing at all where there are none.
//! **At the nominal, two of the five documents measured there refuse
//! nothing the tier is asked** — the two-hole plate and R1's annulus,
//! whose whole `numeric` column is the numeric channel certifying a
//! non-zero sign, where the tier is never consulted; R1's segment boss
//! refuses one decision; R2's filleted bracket refuses 74 of 2,021 and
//! R2's link 107 of 1,102.
//!
//! **On the affordability line's instrument** — one whole-box leaf
//! (`m10_10_leaf_cost_with_and_without_the_algebra`), release, the
//! fastest of three takes — the shipped rules at one attempt per rung
//! against the same rules with `SymRetry::kept_atom`: plate at `1e2·ε`
//! 0.35 → 0.35 s, annulus 0.37 → 0.38, bracket 2.86 → 3.88, link
//! 17.28 → 19.71, pad at `1e2·ε` 131.3 → 147.7. The line is 1.6 s. The
//! bracket, the link and the pad are over it at the first attempt, and
//! the ladder takes no document from under it to over it; it adds 36 %,
//! 14 % and 12.5 % on those three, and it recovers six decisions on the
//! bracket (the rule-A attempt), twelve on the link (the rule-G
//! attempt) and nothing on the pad. **It
//! SHIPS ON ACROSS THE LINE as a disclosed trade, as rule E did** (the
//! rule-E section above): the shipped rules alone put those three over
//! the line — the bracket at 10× and the link at 75× their M10-9 leaf —
//! the ladder adds 1.0 s, 2.4 s and 16 s there and changes no
//! certification, and the rule-G attempt returns exactly the ten
//! decisions the default rule G costs the link's `carrier_on_surface_2`.
//! The rule-A attempt ships for its reach — the bracket's six decisions
//! out of `numeric`, for about 0.4 s of the bracket's 1.0 s — on the
//! same balance, named. `editor_core::drive::DEFAULT_SYM_RETRY` carries
//! the table per mask, the reads-off reading that says the decision read
//! is not what the ladder costs, and the argument.
//!
//! **What a ladder holds**, per attempt, as a session ends
//! (`profile::SymProfile::retry_forms`), at the nominal: the segment
//! boss 80 forms an attempt against a DAG of 12,638 nodes, the bracket
//! 2,716 against 28,996, the link 5,259 against 19,564; the plate and the
//! annulus none, because no attempt is made. [`RETRY_FORMS`] is set
//! against those numbers.
//!
//! **The freeze causes under the refusals** are what the ladder's
//! shapes were chosen against, and the instrument is the per-decision
//! attribution ([`profile::DecisionRecord`], `rung_table`). On the
//! bracket's 74 refused decisions their own walks froze 204 nodes on
//! the coefficient bound, 161 on degree and 35 on terms; on the link's
//! 107, 90 / 154 / 152. The ring is a leading cause on both, and a
//! 512-bit ring retry recovers, predicate by predicate, no more than the
//! kept-atom attempts do (the bracket's same six, eight of the link's
//! twelve) at 4.50× the bracket's nominal replay where the kept-atom
//! ladder is 1.47× — so the freeze cause is not the shape the retry
//! that recovers it takes.
//!
//! # The census: which identity-shaped predicates this tier reaches
//!
//! Two greps over `crates/` and `demos/` — one for the names handed to a
//! funnel door, one for identity/gap-shaped string literals — and their
//! union minus the bare filter words and the test-harness names. **106
//! names** at M10-7, **104** since two retired. The rule is written out in
//! `work/tier/symbolic-tier-census.md`, which also carries the full
//! table: one row per name, with its bucket, its evidence and its site.
//! Only the counts and the two families that matter are here.
//!
//! | bucket | count |
//! | --- | --- |
//! | IMPLICIT (S-CERT's frontier) | 3 |
//! | NOT A PREDICATE | 8 |
//! | EXPLICIT | 93 |
//!
//! **106 at M10-7, and not the 66 the previous sweep reported**, because that
//! number is not re-derivable from a rule written down anywhere and this
//! one states its own. The difference is filter width, not new
//! predicates.
//!
//! **IMPLICIT — 3**, and this is the census's load-bearing claim:
//! `ssi_on_locus` and `ssi_on_locus_foot` (a marched intersection
//! point's residual and the foot of its projection, the plane × NURBS
//! lane's chart-image foot among them) and `offset_corner_on_surface`
//! (an offset corner found by a root solve) — EXACTLY the quantities
//! S-CERT's frontier item
//! (`work/tier/param-box-certification-of-implicit-quantities.md`)
//! names, at either filter width. A quantity found by iteration has no
//! expression in the parameters, so no normal form reaches it and its
//! residual widens with the box whatever this tier does.
//!
//! **NOT A PREDICATE — 8.** Seven are `pncad-py` TAG strings for error
//! and enum variants; `carrier_kind` is a diagnostic name on an
//! `Indeterminate` carrying `MarginKind::Invalid`
//! (`geom-brep/src/carrier_eq.rs`) — a structure contradiction, with
//! no margin ever classified.
//!
//! **EXPLICIT — 93.** Closed forms in the parameters over analytic
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

use core::cell::RefCell;
use core::ops::{Add, Div, Mul, Neg, Sub};
use std::collections::HashMap;
use std::sync::Arc;

use crate::predicate::{Band, Decide, Decided, Indeterminate, Sign};
use crate::real::{Bounds, CertifiedEnclosure, Real};
use crate::spline::{KnotVector, SpanLocate, SpanSet};
use crate::tolerance::Tol;

/// The atom algebra: the rule A/B reductions over a residual.
#[path = "sym/algebra.rs"]
mod algebra;
/// The test instrument that checks the tier's algebra and the registry
/// against the definite signs a build decided (test-only).
#[cfg(feature = "sym-revalue-testing")]
pub mod contradiction;
/// The deferring session: decisions taken numerically, the Zeros a
/// reader asks about discharged afterwards.
#[path = "sym/deferred.rs"]
pub mod deferred;
/// The normal form itself: the polynomial, the quotient of two of them,
/// and the pure operations on a form.
#[path = "sym/form.rs"]
mod form;
/// Probe (M16): the form-enclosure reading, counted beside today's answer.
pub mod m16;
/// Rule F: the manifest sign — `copysign` and `abs` atoms whose sign
/// the form already shows, and the non-negativity predicate rule D
/// shares with it.
#[path = "sym/manifest.rs"]
mod manifest;
/// The DRIVE-scoped plain memo: the one piece of the tier's state that
/// outlives a leaf, and the argument that lets it.
#[path = "sym/memo.rs"]
pub mod memo;
/// The name a value with no expression is minted under: the scope open
/// on the minting thread and the value's ordinal there.
#[path = "sym/mint.rs"]
pub mod mint;
#[cfg(feature = "sym-profile-testing")]
pub mod profile;
/// The structural question: whether an expression is zero as a function
/// of the document's variables, asked of the expression.
#[path = "sym/question.rs"]
mod question;
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
/// The re-valuation instrument: a decision's DAG evaluated at another
/// parameter point, against the build there (test-only).
#[cfg(feature = "sym-revalue-testing")]
pub mod revalue;
/// Rule G: the canonical square root — the one door every `Sqrt` atom
/// is minted through, and the `D ≥ 0` side condition its quotient
/// split rests on.
#[path = "sym/root.rs"]
mod root;
/// The decision read — the one value the tier reads (a parameter
/// bracket in certification arithmetic) and the arm it certifies at the
/// decision door and at `min`/`max` — and the exact polynomial square
/// root, which reads no value.
#[path = "sym/signed.rs"]
mod signed;
/// Rule D: trig of `atan`, exact — the closed forms of `sin`/`cos` at
/// `q · atan(X)`.
#[path = "sym/trig.rs"]
mod trig;
/// Rule H, angle sums: the canonical `sin`/`cos` argument and the
/// product rewrite over it.
mod turn;

use form::{Form, Poly, powi_form, within};
pub use memo::{DriveMemo, MemoSize};
pub use mint::{MintGuard, MintScope};
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

/// The symbol a document variable enters the DAG as: its minted id
/// (VARIABLES-DESIGN VR8), so two evaluations of the same document agree
/// on it, and a rename, which moves no id, moves no symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParamSymbol(u64);

impl ParamSymbol {
    /// The symbol for the variable whose id is `id`.
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

// -------------------------------------------------------------- nodes

/// What one DAG node computes. Everything outside the ring operations
/// is an OPAQUE atom (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SymOp {
    Param,
    /// **One untracked real**, minted by [`Sym::opaque`]: a value whose
    /// expression this session did not build, carrying the name it was
    /// minted under ([`mint`]: its scope and its ordinal there) so that
    /// each such value is its OWN unknown. Never keyed by the value's
    /// bits — two intervals
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
    /// `x − sin x` ([`Real::less_sin`]): one function, so one atom,
    /// re-valued through the door that valued it.
    LessSin,
    Atan2,
    Min,
    Max,
    Copysign,
    /// The value-level decision door ([`Real::select_le_zero`]): the
    /// only THREE-child op. Keyed like every other indeterminate atom —
    /// by its children's normal forms — so two selects over equal forms
    /// are one unknown and two over different ones are two.
    Select,
    /// The multi-span enclosure hull ([`SpanLocate::enclosure_hull`]).
    /// Keyed by CHILD IDS rather than by their normal forms, because a
    /// hull is a function of the operands' ENCLOSURES and not of the
    /// reals they stand for — two expressions with equal normal forms
    /// can carry different enclosures, so keying it by form would claim
    /// an equality that does not hold.
    Hull,
    /// **The free parameter of a limb-2 identity**
    /// ([`Decide::free_identity`]): a carrier's parameter left free, one
    /// indeterminate whatever value it was minted at. Never bracketed —
    /// no session records a range for it — so the decision read
    /// ([`signed`]) encloses no form that holds it, and it reaches the
    /// zero test alone.
    Free,
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
            Self::Select => 25,
            Self::Free => 26,
            Self::LessSin => 27,
        }
    }

    /// How many of the node's three child slots this op reads.
    fn arity(self) -> usize {
        match self {
            Self::Param | Self::Opaque | Self::Lit | Self::Pi | Self::Free => 0,
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
            | Self::Floor
            | Self::LessSin => 1,
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Atan2
            | Self::Min
            | Self::Max
            | Self::Copysign
            | Self::Hull => 2,
            Self::Select => 3,
        }
    }
}

/// One DAG node: its op, its payload bits (`Lit`'s float bits,
/// `Param`'s symbol, `Powi`'s exponent) and up to three children — the
/// third read by [`SymOp::Select`] alone, and `UNRECORDED` on every
/// other op.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SymNode {
    op: SymOp,
    payload: u64,
    kids: [SymId; 3],
}

impl SymNode {
    /// **The one spelling of a literal's node**: what
    /// [`Real::from_f64`], `zero` and `one` intern, and what
    /// [`SpanLocate::plain_constant`] recomputes to recognise one.
    fn literal(x: f64) -> Self {
        SymNode {
            op: SymOp::Lit,
            payload: x.to_bits(),
            kids: [SymId::UNRECORDED; 3],
        }
    }

    /// **The children this node actually has** — `kids` is a fixed pair
    /// and the arity says how much of it is the expression; every walk
    /// over the DAG wants this slice and none of them wants the
    /// padding.
    fn kids(&self) -> &[SymId] {
        &self.kids[..self.op.arity()]
    }

    /// An opaque value's node: its ordinal in the payload and its mint
    /// scope's name in the first child slot. The slot holds no node:
    /// the op's arity is zero, so [`Self::kids`], which is what a walk
    /// of a node's children reads, is empty, and a reader of the raw
    /// slots reads them by an op that has children.
    fn opaque(scope: u128, ordinal: u64) -> Self {
        SymNode {
            op: SymOp::Opaque,
            payload: ordinal,
            kids: [SymId(scope), SymId::UNRECORDED, SymId::UNRECORDED],
        }
    }

    /// The indeterminate an [`SymOp::Opaque`] node is.
    fn opaque_indet(&self) -> u128 {
        indet_opaque(self.kids[0].0, self.payload)
    }

    fn id(&self) -> SymId {
        SymId(
            Hash128::new()
                .word(self.op.tag())
                .word(self.payload)
                .wide(self.kids[0].0)
                .wide(self.kids[1].0)
                .wide(self.kids[2].0)
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
    /// **Decisions the symbolic tier answered `Zero`**: the margin's
    /// form is zero at every value where the construction takes the
    /// decisions it took (ERROR-DESIGN E12). One column, and not one per
    /// way of reaching the zero: a symbolic discharge answers that the
    /// margin is zero and nothing about how. What a rule, the decision
    /// read or the registry buys a document is the differential against
    /// the tier with that dial off. The matching K token and report row
    /// are `symbolic` too.
    pub symbolic: u64,
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
    /// reaches this count; what moves is what the registry buys — the
    /// differential of [`SymCounts::symbolic`] against the tier with
    /// the registry off — which collapses as the registry stops
    /// discharging. A registrant stating a GEOMETRIC lie is caught
    /// earlier still, by its own `debug_assert!` on the exact witness's
    /// refusal, which is live in every profile. What is left for this
    /// column is `Cyclic` and any future registrant that binds an exact
    /// refusal instead of asserting on it. The fixture-scale row asserts
    /// all three together
    /// (`editor-core/tests/m10_9_pins_interval.rs`,
    /// `m10_9_no_registrant_lies_on_any_measured_document`).
    pub registrations_refused: u64,
    /// Decisions handed to the numeric channel.
    pub numeric: u64,
    /// **Decisions closed by a RETRY** ([`SymRetry`]) — the first
    /// attempt's rungs all declined and one of the ladder's answered.
    ///
    /// A column BESIDE [`SymCounts::symbolic`] and not a second
    /// discharge column: a retry's zero is the same claim as the first
    /// attempt's (`SymRetry` argues that once), so it is counted in
    /// `symbolic`, and this says how many of those the ladder is
    /// carrying. It has no K token for the same reason —
    /// the sample's vocabulary is about what a decision CLAIMS, and a
    /// retry claims nothing new.
    ///
    /// Zero under every session door but [`with_session_retry`] and
    /// [`with_session_memo_retry`], the two that install a ladder.
    pub retried: u64,
    /// **The nodes this receipt's subject could not resolve to a form**
    /// — frozen into indeterminates by a budget or an overflow. A SET
    /// and not a count of work: a claim about the subject, like the
    /// decision columns beside it, and the same under every schedule.
    /// [`memo`]'s header is where the argument for that lives; here is
    /// what the number means on each receipt that carries it.
    ///
    /// - **On a LEAF of a drive: that leaf's NEED**, the drive's frozen
    ///   nodes inside the closure of what this leaf's plain walk was
    ///   asked. Not the freezes it happened to compute: a leaf takes
    ///   forms another leaf froze, and counting the work would report
    ///   which leaf got to a node first.
    /// - **On a DRIVE: the distinct nodes frozen over it**
    ///   ([`DriveMemo::frozen`]), the set a leaf's NEED is read
    ///   against. It does not sum the leaves' columns: their sets
    ///   overlap, so [`SymCounts::absorb`] does not add this one up.
    ///   The driver writes the drive's own when the drive is done.
    /// - **Outside a drive** ([`with_session`], [`with_session_rules`]):
    ///   the session's own distinct freezes. The leaf is the whole
    ///   drive there, so this is the same quantity.
    /// - **Mid-replay, through [`session_counts`]: the WORK so far**,
    ///   the freezes this replay has computed. That door prices a leaf
    ///   while it runs; a NEED is a property of the finished leaf and
    ///   is written over this count at the leaf's end.
    ///
    /// **The dial does not move it.** With the drive's table serving
    /// nothing a leaf freezes every node of its closure that freezes at
    /// all, so its NEED is what it computed (`editor-core`'s
    /// `the_plain_memo_moves_no_decision` compares the leaf receipts
    /// across the dial with no mask).
    pub frozen: u64,
}

impl SymCounts {
    /// The decision counts added together.
    #[must_use]
    pub fn decisions(&self) -> u64 {
        self.symbolic + self.numeric
    }

    /// Adds another session's DECISION counts into this one.
    ///
    /// **`frozen` is not summed** — it is a set on both receipts and
    /// the leaves' sets overlap ([`SymCounts::frozen`] argues the
    /// column); the driver writes the drive's own once the drive is
    /// done.
    pub fn absorb(&mut self, other: Self) {
        self.symbolic += other.symbolic;
        self.registrations_refused += other.registrations_refused;
        self.numeric += other.numeric;
        self.retried += other.retried;
    }
}

/// **The tier's dials** — every mechanism the normal form layers on
/// the plain quotient form, each switchable so that its effect on a
/// document is a measurement rather than an assumption, and so that
/// every dial off ([`Self::none`]) is the plain quotient form bit for
/// bit: the atom-algebra rules A and B; the constant fold A0
/// (`const_fold`) and the early walk it runs in (`early`); rules A/B
/// applied PER NODE inside that walk (`early_ab`); rule D, trig of
/// `atan` in closed form with amendment A1's `atan2` and half-π folds
/// (`trig_of_atan`); and the registered-identity door (`registered`).
///
/// Every rule is an equality of reals under clause 1 of the theorem
/// ([`Decide::sign_within`]'s docs), so a zero reached through any of
/// them is still a zero of the real margin; what a rule can cost is
/// only a cancellation it fails to find. The rules are named A, B, D,
/// E, F and G where the tier's module docs discuss them (M10-8's C is
/// retired); the door is an axiom rather than a rule, and what it buys
/// is the differential with it off (`registered`).
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
    /// form — never replacing it — in which the per-node rules run (and,
    /// with [`Self::early_ab`], rules A/B per node too). A decision is
    /// asked of the plain form first, so the early form can only ADD a
    /// discharge. The per-node rules ride this walk because the atoms
    /// they fold sit nested inside other atoms' arguments, out of a
    /// top-residual reduction's reach. Its cost is a second walk of the
    /// DAG per decision the plain form did not answer, memoized per
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
    /// **H — ANGLE SUMS, exact** ([`turn`]): in the early walk every
    /// `sin`/`cos` atom is minted over a canonical argument — its
    /// rational-turn part `q·π` split off by angle addition and read
    /// off the first octant, the rest's sign normalised — and a
    /// product of two such atoms of one class is rewritten as `sin`/
    /// `cos` of the sum and difference of their arguments, per node.
    /// Unconditional: every rewrite is an identity of the reals and no
    /// value is read. It is what closes a ring turned by `k · turn/N`
    /// for an `N` whose `cos(turn/N)` is irrational. Needs `early`.
    pub angle_sum: bool,
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
    /// **Strict positivity, not non-negativity**, and the reason is
    /// `copysign`: it reads a SIGN BIT, so `copysign(1, −0.0) = −1`
    /// while `copysign(1, +0.0) = +1`, and at a real zero of `X` the
    /// node denotes no function of the real value of `X` at all. The
    /// predicate excludes that point. [`manifest`]'s header carries
    /// the argument and the shapes it must not fold. Needs `early`.
    pub manifest_sign: bool,
    /// **G — the CANONICAL SQUARE ROOT** ([`root`]): in the early walk
    /// every `sqrt` atom is minted through one door that keys it on its
    /// argument's VALUE CLASS — the quotient split `sqrt(N/D) =
    /// sqrt(N)/sqrt(D)` under the `D ≥ 0` side condition [`root`]
    /// argues, each half's rational content taken out (`s` exactly,
    /// `sqrt(f)` a constant atom) over the primitive integer
    /// polynomial, and `sqrt(R²) = |R|`.
    ///
    /// It reads no value on its own: the content split is arithmetic
    /// on the coefficients, and the side condition's first three
    /// sources are facts about the form, and no source reads a value
    /// ([`root`]'s header says why).
    ///
    /// **A canonical FORM, not a rewrite that fires somewhere**: what
    /// it changes is the identity of the indeterminate, so two
    /// spellings of one real — the walk's `sqrt(1/S²)` and the
    /// normal's own `S` — are one atom and can cancel. Needs `early`.
    ///
    /// **It carries one companion rewrite**, rule A's `abs(X)² = X²`
    /// ([`algebra`]): nothing mints an `Abs` where a root used to stand
    /// until this rule does, and a root that reduced through
    /// `sqrt(R²)² → R²` has to keep reducing once it is spelled `|R|`.
    /// The companion is behind THIS dial and not rule A's, so
    /// [`Self::without_canonical_root`] is the tier as it stood.
    pub canonical_root: bool,
    /// **Rule G's COMPANION REWRITE, `|X|² = X²`** ([`algebra`]'s
    /// `find_square`, the `Abs` arm) — rule A's substitution at the
    /// atom rule G leaves where a root's argument was a perfect
    /// square. Read as `canonical_root && sqrt_square && abs_square`,
    /// so it is off wherever rule G is and this dial only ever takes
    /// it away: a tier with `canonical_root` off that carried the
    /// rewrite never existed, and `find_square`'s own comment argues
    /// that once.
    ///
    /// It has a dial of its own because it is the half of rule G whose
    /// trade is MEASURED apart: on R2's link it buys 52 decisions of
    /// `carrier_on_surface_2` and costs 10, by opening the square of
    /// an `abs` node the document wrote into an expansion that does
    /// not cancel where the closed atom did
    /// (`work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`).
    /// That is the kept-atom RETRY's first shape, and a retry needs a
    /// mask bit to turn off.
    pub abs_square: bool,
    /// **Rule G's MAGNITUDE DOOR, `sqrt(R²) = |R|`** ([`root`]'s
    /// `magnitude_of_root`, step 3 of the canonical form) — read as
    /// `canonical_root && root_magnitude`, so like [`Self::abs_square`]
    /// it only ever takes the step away from a tier that has rule G.
    /// With it off the primitive part keeps its `Sqrt` atom and the
    /// content split above it stands.
    ///
    /// The second measured half of the link's trade: six of its
    /// sixteen go this way and are re-taken through the door by the rim
    /// registrant's axiom, so they stay zeros (SYM-9's split receipt
    /// read them as six fewer theorems and six more `registered`; the
    /// row above). The kept-atom retry's second shape.
    pub root_magnitude: bool,
    /// **Rule G's EXACT QUOTIENT** ([`root`]'s `exact_quotient_root`):
    /// a root whose argument `N/D` has a denominator that divides its
    /// numerator EXACTLY — `N = Q·D` as polynomials, the division's
    /// exact remainder reaching zero ([`form`]'s `Poly::div_exact`) — is
    /// minted over the polynomial `Q`, before the split is asked. It is
    /// rule E's quotient carried past the monomial at the one door every
    /// root goes through: a factor both halves share that is a
    /// POLYNOMIAL (a chord's `(a + x)⁴`) is invisible to rule E, and the
    /// split then keys the halves' factors apart (`|p³|/p²` for `|p|`)
    /// or declines on the ring. Read as `canonical_root &&
    /// root_quotient`, so like [`Self::abs_square`] it only ever takes
    /// the step AWAY from a tier that has rule G.
    ///
    /// An equality of reals wherever `D ≠ 0`, which clause 1 grants
    /// ([`quotient`]'s four-source argument), and no value is read, so
    /// a zero through it is a THEOREM. **It is a trade, not only a
    /// gain**: the root re-keyed to `sqrt(Q)` no longer meets the split
    /// spelling `sqrt(N)/sqrt(D)` of the same value — meeting both needs
    /// a factorisation of `N` — and no measured document moves on that
    /// shape. [`root`]'s header carries the argument, the trade and what
    /// it does not reach.
    pub root_quotient: bool,
    /// **The DECISION READ** ([`signed::decision`], [`signed::order`]):
    /// in the early walk a `Select` whose decision is certified
    /// one-signed over the leaf's box takes that arm, and a `min`/`max`
    /// whose comparison is certified takes the arm it picks — `max(A,
    /// B)` IS `select(B − A, A, B)`, so the two are one read.
    ///
    /// At an exact witness the arm a certified read takes is the arm
    /// the construction takes at every point of the box, so a zero
    /// through it holds wherever the construction takes the decisions
    /// it took: E12's claim, the same claim every other discharge makes.
    /// At an inexact witness (`f64`, a probe) the construction compares
    /// a rounded value and can take the other arm, so the read is not
    /// asked there (`Session::witness`), whatever this dial says.
    ///
    /// **Ordered behind every value-free fold AT ITS NODE** — after A0,
    /// after rule F, and over kids whose roots rule G has already
    /// minted — and answered at the node: a parent that would have
    /// cancelled two equal nodes as one atom sees their arms, and
    /// reaches the same zero through them. Needs `early`.
    pub decision_read: bool,
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
    /// consulted in the early memo only, never in the plain one, so the
    /// plain walk, the cheapest, answers what it reaches without paying
    /// for the registry.
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
            angle_sum: true,
            common_factor: true,
            manifest_sign: true,
            canonical_root: true,
            abs_square: true,
            root_magnitude: true,
            root_quotient: true,
            decision_read: true,
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
    /// | E, the quotient's common factor (`common_factor`, SYM-5) | none on the five; R1's boss at bulge 2 `8.2611e2 → 9.3559e2 · ε` (1.13×), and a derived frame whose AXES carry a parameter certifies where its authored twin does, which no dial reached before | one whole-box leaf, release: plate 0.13 → 0.36 s, annulus 0.12 → 0.29, bracket 0.44 → 1.70, link 3.31 → 2.43, pad 3.85 → 14.40 | **yes**, with the bracket, the pad and the link over the 1.6 s line disclosed |
    /// | F, the manifest sign (`manifest_sign`, SYM-8) | none: shut alone (every other rule as shipped) it moves no receipt, split or ceiling on the eight measured documents, the five `m10_9` studies or the tilted-frame family. It was measured on Duff's basis, whose `copysign(1, n.z)` it folded (the tilt-`u` frame's `carrier_endpoint_end` 24/0/0/1 → 33/0/0/0 then); the axis-order basis mints no `copysign`, and on this tree the rule has no measured document consumer (`work/sym/the-negative-arm-lost-its-document-consumer`) | one coefficient-sign scan per `abs`/`copysign` node; the six leaf numbers live once, in the module header's rule-F section | **yes** |
    /// | F's NEGATIVE arm (the same dial, SYM-12) | none, as rule F: the tilt-`u` cube's START cap and its `FlipZ` twin certify as the end cap does with the dial on or off (`m10_derived_frame_tilted_interval`'s `m10_the_start_cap_and_flip_z_certify_as_the_end_cap_does_and_rule_f_is_inert`); its Duff-era reach is in the module header | a negation plus the predicate only on a numerator whose every coefficient is negative; the release leaf instrument's reading is in the header's cost paragraph below the rule-F section, the one place those numbers live | **yes** |
    /// | G, the canonical root (`canonical_root`, DECIDE-3) | the tilted derived boss certifies at both halves and both lifts and the tilt-`u` one outright; the link, the bracket and the pad gain theorems and the plate's ledger loses its `Early/Assertion` and `Door/Decision` freezes | the differential is `without_canonical_root`; the numbers live in the PR that shipped it and in [`root`] | **yes** |
    /// | G's exact quotient (`root_quotient`, DECIDE-4) | R1's boss at bulge 2 `1.0309e3 · ε` → **0.5024 / 0.7267 / 0.7271 of its REAL study** at ε = 1e-6 / 1e-9 / 1e-12, bounded by `dihedral_wedge` (a real margin), its `arc_span` 5/0/0/1 → 6/0/0/0; no other split moves at the nominal on the plate, bracket, annulus, link, both D-tabs or the two controls; it trades the split spelling `sqrt(N)/sqrt(D)` of a re-keyed root (no measured document moves on it) | one whole-box leaf, release, best of 3, off → on: plate 0.339 → 0.349 s, plate at its real study 0.342 → 0.346, annulus 0.364 → 0.364, bracket 3.81 → 3.82, link 19.4 → 19.3, pad 144.5 → 145.7, boss 0.245 → 0.250; every receipt but the boss's unmoved | **yes**, with the bracket, the pad and the link over the 1.6 s line either way |
    /// | the decision read (`decision_read`, DECIDE-3) | the frame's conditioning comparisons, which no form settles; it answers at its node, and a zero through its arm is a zero (its differential is [`Self::without_the_reads`]; on the tilted derived-frame family the end cap refuses at `newell_plane_residual` with the read off) | the deep enclosure runs at every `Select` and `min`/`max`; the pin suites' wall time is the cost row `work/decide/decision-read-triples-the-plate-pin-suites-wall-time` | **yes**, with that cost disclosed |
    /// | H, angle sums (`angle_sum`, G) | a ring stepped by an exact `turn/N` proves every side seat at the door's 256 terms, `N ∈ {4, 5, 6, 8, 9, 12, 16, 24}`, the members at a rational cosine included (0/12 at `N = 12` shut); at the nominal, `carrier_matches_mapped_source` gains 2 on the bracket (339 → 341), 8 on the link (174 → 182) and 4 on the literal D-tab (188 → 192), every other split on the eight documents unmoved; no ceiling moves | one whole-box leaf, release, best of 3, off → on (`without_angle_sum`), on a faster machine than the other rows' timings: plate 0.104 → 0.110 s, bracket 1.149 → 1.161, annulus 0.089 → 0.097, pad 0.521 → 0.547, link 0.231 → 0.234, boss 0.115 → 0.123; M10-4's stackup hulls certify in one leaf as before | **yes** |
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
            angle_sum: true,
            common_factor: true,
            manifest_sign: true,
            canonical_root: true,
            abs_square: true,
            root_magnitude: true,
            root_quotient: true,
            decision_read: true,
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
            angle_sum: false,
            common_factor: false,
            manifest_sign: false,
            canonical_root: false,
            abs_square: false,
            root_magnitude: false,
            root_quotient: false,
            decision_read: false,
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
    /// **EIGHT dials, and each was added the day its rule shipped.** A
    /// rule that rewrites a form in the EARLY walk is form-level
    /// algebra whatever its argument reads, so rule E (SYM-5), rule F
    /// (SYM-8), rule G ([`Self::canonical_root`]) and the decision read
    /// ([`Self::decision_read`]) belong here beside A/B and D; leaving
    /// one out makes this constructor a differential against a tier
    /// that never existed, silently, while its own doc still claims
    /// M10-9's.
    #[must_use]
    pub const fn without_the_algebra() -> Self {
        Self {
            sqrt_square: false,
            pythagoras: false,
            early_ab: false,
            trig_of_atan: false,
            angle_sum: false,
            common_factor: false,
            manifest_sign: false,
            canonical_root: false,
            // Read as a conjunction with `canonical_root`, so they say the
            // same thing either way; spelled false here so that two
            // constructors of one tier are one VALUE and a row may
            // compare them.
            abs_square: false,
            root_magnitude: false,
            root_quotient: false,
            decision_read: false,
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
    /// buys is measured against ([`Self::common_factor`]). Rule F, rule
    /// G and the decision read are shut with it, because M10-10's tier
    /// had none of the three; [`Self::without_rules_f_g_and_the_read`] is the other
    /// half of the pair and keeps rule E on.
    #[must_use]
    pub const fn without_rule_e() -> Self {
        Self {
            common_factor: false,
            angle_sum: false,
            manifest_sign: false,
            canonical_root: false,
            // Read as a conjunction with `canonical_root`, so they say the
            // same thing either way; spelled false here so that two
            // constructors of one tier are one VALUE and a row may
            // compare them.
            abs_square: false,
            root_magnitude: false,
            root_quotient: false,
            decision_read: false,
            ..Self::shipped()
        }
    }

    /// **The shipped set with rule G SHUT** — every `sqrt` atom keyed
    /// on the argument form the walk arrived with, as it was before the
    /// canonical root: the differential every claim about what rule G
    /// costs and what it buys is measured against
    /// ([`Self::canonical_root`]). The decision read stays ON, because
    /// a differential that also shut a read would measure two things at
    /// once; [`Self::without_the_reads`] is the other half of the pair.
    #[must_use]
    pub const fn without_canonical_root() -> Self {
        Self {
            canonical_root: false,
            // Read as a conjunction with `canonical_root`, so they say the
            // same thing either way; spelled false here so that two
            // constructors of one tier are one VALUE and a row may
            // compare them.
            abs_square: false,
            root_magnitude: false,
            root_quotient: false,
            ..Self::shipped()
        }
    }

    /// **The shipped set with rule G's EXACT QUOTIENT shut** — every
    /// root keyed on its argument's quotient as the walk left it: the
    /// differential what [`Self::root_quotient`] buys and costs is
    /// measured against, and the tier SYM-9 shipped bit for bit.
    #[must_use]
    pub const fn without_root_quotient() -> Self {
        Self {
            root_quotient: false,
            ..Self::shipped()
        }
    }

    /// **The shipped set with rule H SHUT** — every `sin`/`cos` atom
    /// keyed on the argument form the walk arrived with and no product
    /// of them rewritten: the differential every claim about what rule
    /// H costs and what it buys is measured against
    /// ([`Self::angle_sum`]).
    #[must_use]
    pub const fn without_angle_sum() -> Self {
        Self {
            angle_sum: false,
            ..Self::shipped()
        }
    }

    /// **The shipped set with the DECISION READ shut** — the decision
    /// door and `min`/`max` left opaque wherever no value-free fold
    /// reaches them: the differential every claim about what the read
    /// buys is measured against ([`Self::decision_read`]). Rule G
    /// stays on, for the reason [`Self::without_canonical_root`] gives.
    #[must_use]
    pub const fn without_the_reads() -> Self {
        Self {
            decision_read: false,
            ..Self::shipped()
        }
    }

    /// **The shipped set with rule F SHUT and nothing else** — the
    /// `copysign` and `abs` atoms of a manifestly SIGNED argument (either
    /// arm) left opaque, every other rule as shipped: the differential
    /// every claim about what rule F costs and what it buys is measured
    /// against ([`Self::manifest_sign`]).
    #[must_use]
    pub const fn without_rule_f() -> Self {
        Self {
            manifest_sign: false,
            ..Self::shipped()
        }
    }

    /// **The shipped set with rule F, rule G and the decision read
    /// SHUT** — SYM-5's tier exactly, bit for bit, because it had none
    /// of the three. Not a differential of any one rule; it is the tier
    /// a row compares against when its claim is about what the three
    /// bought together ([`Self::without_rule_f`] is rule F alone).
    #[must_use]
    pub const fn without_rules_f_g_and_the_read() -> Self {
        Self {
            manifest_sign: false,
            canonical_root: false,
            // Read as a conjunction with `canonical_root`, so they say the
            // same thing either way; spelled false here so that two
            // constructors of one tier are one VALUE and a row may
            // compare them.
            abs_square: false,
            root_magnitude: false,
            root_quotient: false,
            decision_read: false,
            ..Self::shipped()
        }
    }
}

impl Default for SymRules {
    fn default() -> Self {
        Self::shipped()
    }
}

impl SymRules {
    /// **This set NARROWED by `mask`**: every rule that is on in both,
    /// off everywhere else — the kept-atom retry's rule set
    /// ([`SymRetry::without`]).
    ///
    /// Spelled field by field rather than over a bitfield so that a
    /// dial added to this struct is a compile error here until someone
    /// says which side of the mask it takes.
    #[must_use]
    pub const fn masked_by(self, mask: Self) -> Self {
        Self {
            sqrt_square: self.sqrt_square && mask.sqrt_square,
            pythagoras: self.pythagoras && mask.pythagoras,
            const_fold: self.const_fold && mask.const_fold,
            early: self.early && mask.early,
            early_ab: self.early_ab && mask.early_ab,
            trig_of_atan: self.trig_of_atan && mask.trig_of_atan,
            angle_sum: self.angle_sum && mask.angle_sum,
            common_factor: self.common_factor && mask.common_factor,
            manifest_sign: self.manifest_sign && mask.manifest_sign,
            canonical_root: self.canonical_root && mask.canonical_root,
            abs_square: self.abs_square && mask.abs_square,
            root_magnitude: self.root_magnitude && mask.root_magnitude,
            root_quotient: self.root_quotient && mask.root_quotient,
            decision_read: self.decision_read && mask.decision_read,
            registered: self.registered && mask.registered,
        }
    }
}

/// **The RETRY LADDER**: the second attempts a REFUSED decision may
/// make, on THREE of the four rungs — the early walk, rules A/B over
/// the top residual, and the registered-identity door. The plain rung
/// is never retried.
///
/// The first attempt is the tier as it stands — the session's
/// [`SymRules`] over the ring at [`rational::COEFF_BITS`] — and nothing
/// here moves it. A retry is asked ONLY where every rung of the first
/// attempt declined, so it can take a decision out of
/// [`SymCounts::numeric`] and never move one the first attempt
/// answered. The plain rung is left out for COST: it applies no rule a
/// mask can shut (A0 only), so a masked attempt would re-walk it to the
/// same answer. A wider-ring attempt could reach past a freeze the
/// first plain walk hit; no shipped ladder makes one, and that reach is
/// unmeasured. The top-residual rung is IN, because its residual is the plain form — unchanged on
/// every attempt — and what an attempt changes there is only the rules
/// the reduction applies and the ring it runs at, so a zero it reaches
/// is a theorem exactly as the first attempt's would be.
///
/// **Why a retry rather than a wider tier.** Widening the ring, or
/// turning a rule on, is not monotone in what the tier discharges
/// (`work/sym/coefficient-ring-width-is-not-monotone-in-reach`): a node
/// the walk cannot build FREEZES into an indeterminate of its own and
/// therefore matches ITSELF on both sides of an identity, so opening it
/// — by a wider ring, or by a rule that folds the atom — can LOSE a
/// discharge the frozen node gave. R1's boss loses ten decisions to an
/// `abs` fold at 256 bits; R2's link loses sixteen theorems of one
/// predicate to rule G
/// (`work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`).
/// A ladder cannot lose one, because the first attempt has already
/// answered wherever it can and the second is asked only into its
/// silence.
///
/// **Each attempt is a sound discharge on its own terms.** The ring's
/// bound is a COST and not a soundness condition — [`rational`]'s
/// integers are exact at every width, and a form that is the zero
/// polynomial over coefficients of 512 bits is the zero polynomial —
/// and a rule set with fewer rules is a subset of the same algebra, so
/// a zero it reaches is a zero the full set would reach if it reached
/// anything at all. A retry's zero is therefore the same claim as the
/// first attempt's, and lands in [`SymCounts::symbolic`];
/// [`SymCounts::retried`] counts it beside, never instead of.
///
/// **An attempt identical to one already made is not walked**
/// ([`Self::attempts`]): one whose rules AND ring bound equal the first
/// attempt's — a mask that takes away only rules the session has already
/// shut, or a ring retry at exactly [`rational::COEFF_BITS`] — or equal
/// an earlier retry's, would build the same forms again for nothing. So
/// a rules differential taken at a narrow tier with a ladder installed
/// pays no ladder at all. Only EQUAL attempts are dropped: a ring retry
/// NARROWER than the first attempt's is a different attempt, sound and
/// pointless, and is walked as the caller asked.
///
/// **Not a field of [`SymBudget`]**, which is where the unit's spec put
/// it: `SymBudget` is built as a struct literal at 53 sites in 39
/// files across four crates on the base this unit is cut from (`git
/// grep 'SymBudget {'` less the declaration, its `impl` and the
/// functions that return one), most of them outside this unit's
/// territory, and more fields would be a struct-update rewrite of every
/// one of them.
/// This carries the dials through its own door ([`with_session_retry`])
/// and leaves every existing caller building the session it builds
/// today.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymRetry {
    /// **The WIDER RING one retry runs at**, in bits — `None` for no
    /// ring retry. The FIRST attempt's bound is
    /// [`rational::COEFF_BITS`] and this never moves it: the readings
    /// that set that constant stand, and this ladder adds attempts
    /// rather than widening the one everything else is measured at.
    pub bits: Option<u64>,
    /// **The KEPT-ATOM attempts, in the order they are taken**, each a
    /// MASK over the session's rules: an attempt runs
    /// [`SymRules::masked_by`], every rule that is on in both, so a
    /// field `false` in a mask names a rule that attempt goes WITHOUT
    /// and a field `true` leaves it as the session has it. `None` in a
    /// slot is no attempt there.
    ///
    /// **Several masks and not one, and the reason is measured.**
    /// SYM-9's Phase 1 took each shape alone and then together, as
    /// retries at the nominal: rule A shut recovers six decisions on
    /// R2's bracket, rule G shut recovers twelve on R2's link, and the
    /// two shut in ONE mask recovers the bracket's six and NONE of the
    /// link's twelve. So one mask cannot express what the measurement
    /// chose. Why the joint mask loses the twelve is NOT executed on
    /// the link: the working hypothesis is that the twelve close
    /// through rule A's `sqrt(X)² = X` once rule G has stopped
    /// re-keying the atom, and what would confirm it is the render of
    /// those residuals under both masks with their atom keys read off
    /// (`work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`,
    /// shape 2).
    pub without: [Option<SymRules>; MASKS],
    /// **The GROWTH GUARD** — the most forms ONE attempt's two memos may
    /// hold, for one node, before the ladder stops offering that
    /// attempt to the rest of that node's decisions, which stay
    /// numeric. The memos are the node's and start empty with it
    /// ([`MintScope::node`]), so what the guard withholds from a node
    /// is a function of that node's own decisions. [`RETRY_FORMS`] by default; a dial so that the guard is
    /// a row and not a comment (`the_growth_guard_withholds_an_attempt_at_its_cap`).
    ///
    /// **The check is BEFORE an attempt is walked**, so it bounds the
    /// memo to the cap plus ONE attempt's walk: a walk adds at most one
    /// form per node id to each of the two memos, so an attempt's memos
    /// never exceed `max_forms` plus twice the ids the session's walks
    /// can visit.
    pub max_forms: usize,
}

/// How many kept-atom masks a [`SymRetry`] may carry. Two, because two
/// is what SYM-9's measurement chose and an array is what keeps
/// [`SymRetry`] `Copy`; a third shape wants a measurement of its own
/// before it wants a slot.
pub const MASKS: usize = 2;

impl Default for SymRetry {
    /// [`SymRetry::none`] — so a struct update from the default never
    /// inherits a growth guard of zero.
    fn default() -> Self {
        Self::none()
    }
}

impl SymRetry {
    /// No retry at all: the ladder is the first attempt and stops — the
    /// tier every session door but [`with_session_retry`] and
    /// [`with_session_memo_retry`] installs.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            bits: None,
            without: [None; MASKS],
            max_forms: RETRY_FORMS,
        }
    }

    /// **The kept-atom ladder SYM-9 measured**: two attempts, one with
    /// rule G shut and then one with rule A's `sqrt(X)² = X` shut, and no
    /// wider-ring attempt. It is the drive's default ladder;
    /// `editor_core::drive::DEFAULT_SYM_RETRY`
    /// carries the measurement that chose the shapes, the order (a tie
    /// on reach and inside the takes' spread on cost, so either would
    /// do; rule G's attempt goes first as SYM-9 shipped it) and the cost
    /// against the 1.6 s line it ships across.
    ///
    /// The rule-G mask spells rule G's conjunct dials (`abs_square`,
    /// `root_magnitude`, `root_quotient`) shut with it, as every constructor that shuts
    /// `canonical_root` does, so the attempt it runs is ONE `SymRules`
    /// value — [`SymRules::without_canonical_root`] on the shipped set.
    #[must_use]
    pub const fn kept_atom() -> Self {
        Self {
            bits: None,
            without: [
                Some(SymRules {
                    canonical_root: false,
                    abs_square: false,
                    root_magnitude: false,
                    root_quotient: false,
                    ..SymRules::all()
                }),
                Some(SymRules {
                    sqrt_square: false,
                    ..SymRules::all()
                }),
            ],
            max_forms: RETRY_FORMS,
        }
    }

    /// The attempts beyond the first, IN THE ORDER THEY ARE TAKEN, each
    /// as its ordinal (1 for the first retry), the rules it runs and the
    /// ring bound it runs at: the masks in slot order, then the ring.
    ///
    /// **An attempt identical to one already made is dropped** — to the
    /// first attempt, `(first, COEFF_BITS)`, or to an earlier retry. Its
    /// walks would build the same forms under the same rules at the same
    /// bound, so they cannot answer where the other did not; offering it
    /// would only pay for the walk.
    fn attempts(self, first: SymRules) -> impl Iterator<Item = (u8, SymRules, u64)> {
        let candidates = self
            .without
            .into_iter()
            .map(move |m| m.map(|mask| (first.masked_by(mask), rational::COEFF_BITS)))
            .chain(core::iter::once(self.bits.map(|bits| (first, bits))))
            .flatten();
        let mut made = [(first, rational::COEFF_BITS); MASKS + 2];
        let mut n = 1;
        let mut out = [None; MASKS + 1];
        for c in candidates {
            if made[..n].contains(&c) {
                continue;
            }
            made[n] = c;
            out[n - 1] = Some((u8::try_from(n).unwrap_or(u8::MAX), c.0, c.1));
            n += 1;
        }
        out.into_iter().flatten()
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
    args: [Option<Arc<Form>>; 3],
}

/// **One replay's symbolic state**, and whose each piece is.
///
/// - **The drive's** ([`DriveMemo`], `memo`): the expression table,
///   the plain forms and the atom records. `nodes`, `forms` and
///   `atoms` are this leaf's caches of it: every node and plain form
///   in them is in the drive's table and every atom record is its
///   id's, so a hit or a miss changes a cost and no answer. With no
///   drive installed they are the table.
/// - **The leaf's input**: `params`, the box's brackets, and the dials
///   (`budget`, `rules`, `retry`) and `witness`, the same for every
///   leaf of a drive.
/// - **Caches valid for the leaf**: `forms_early`, `trig_closed`,
///   `reductions`, `budget_frozen`. Each is a function of its key, the
///   dials and the brackets.
/// - **A node's** ([`NodeState`], `node`): the registry, the counts,
///   the retry memos their guard counts, the door memo and the plain
///   roots. A node's scope opens with none of another node's.
///
/// - **Names**: `mints`, the count each closed mint scope stopped at
///   ([`mint`]), beside the thread's open scope; a value with no
///   expression is named by the scope that minted it.
///
/// So a node's decisions read nothing whose lifetime is the replay but
/// the box.
struct Session {
    budget: SymBudget,
    rules: SymRules,
    /// **The retry ladder this session offers a refused decision**
    /// ([`SymRetry`]) — [`SymRetry::none`] under every door but
    /// [`with_session_retry`] and [`with_session_memo_retry`].
    retry: SymRetry,
    /// **The open node's state** ([`NodeState`]), and the states of the
    /// scopes it was opened inside, outermost first. With no node scope
    /// open, `node` is the root scope's.
    node: NodeState,
    outer: Vec<NodeState>,
    nodes: IdMap<SymNode>,
    /// The PLAIN quotient forms — every atom opaque, rule A0 only (the
    /// constant fold, which cannot cost a cancellation). Rules A/B are
    /// applied afterwards over the top residual ([`algebra::reduce`])
    /// and per node in `forms_early`; nothing ruled is memoized here.
    forms: IdMap<Arc<Form>>,
    /// The EARLY-reduced forms (`SymRules::early`), a second memo
    /// beside the plain one.
    forms_early: IdMap<Arc<Form>>,
    /// The `f64` bracket of each document parameter this leaf was
    /// evaluated over, by the parameter's indeterminate id — recorded
    /// by [`Sym::param_over`] and read by the decision read
    /// ([`signed`]) alone.
    params: IndetMap<(f64, f64)>,
    /// The count each closed mint scope stopped at, by the scope's
    /// name, so a scope opened again in this session mints on from
    /// there and no name is minted twice ([`mint`]).
    mints: IndetMap<u64>,
    /// **The witness kind of the scalar this session decides at**
    /// ([`crate::Witness`]), noted by the first decision and the same at
    /// every later one. The decision read takes an arm only at an EXACT
    /// witness: there the construction's comparison is a certified sign
    /// of the same real the read encloses, so the arm the read takes is
    /// the arm the construction takes. At an inexact witness the
    /// construction compares a ROUNDED value, which can lie on the other
    /// side of zero from the exact one, and the read declines.
    witness: Option<crate::real::Witness>,
    /// Every opaque atom minted so far, by its indeterminate id.
    atoms: IndetMap<AtomInfo>,
    /// **Rule D's closed forms, by argument form** (`trig::Closed`):
    /// `sin` and `cos` of one argument are two nodes, and the recurrence
    /// that builds `(cos kψ, sin kψ)` over the shared denominator yields
    /// both at once — so a `sin_cos` pair minted from one node folded
    /// TWICE before this memo (R2's Q7). Keyed by the argument form's
    /// digest, per session like every other memo here.
    trig_closed: IndetMap<Vec<trig::Held>>,
    /// **The per-node reduction's answers, by input form**
    /// ([`reduce_per_node`]): each bucket, under the input's digest,
    /// holds the forms reduced with that digest, compared WHOLE, beside
    /// the rules and the ring bound each was reduced under and what the
    /// reduction answered — a refusal included. Capped at
    /// [`REDUCTION_FORMS`] entries (`reductions_held`).
    ///
    /// **Why the whole form and not the digest, where `trig_closed`
    /// above keys on the digest alone**: a wrong hit here hands a walk
    /// another form's reduction, which is a wrong FORM and a decision
    /// built on it; the compare costs one pass over a form the
    /// reduction would otherwise walk several times, and a hit saves
    /// the reduction. `trig_closed` keys what the walk itself builds in
    /// a handful of places; bringing it to the same standard is not
    /// this memo's to do.
    reductions: IndetMap<Vec<Reduction>>,
    reductions_held: usize,
    /// How many asks an entry answered — read by this module's rows.
    #[cfg(test)]
    reduction_hits: u64,
    /// **The drive's table** ([`DriveMemo`]), when a drive installed
    /// one ([`with_session_memo`]): a node or a plain form new to this
    /// leaf's caches is written to it as it is made, and a miss in the
    /// caches asks it.
    memo: Option<Arc<DriveMemo>>,
    /// **The table an ask's state reads nodes in** (`question`): the
    /// installed session's, for [`Session::node_of`] and nothing else.
    /// An ask's rules and budget are its own, so it neither reads a
    /// form of the drive's nor writes one. `None` in every session but
    /// an ask's.
    nodes_in: Option<Arc<DriveMemo>>,
    /// **The Zeros a deferring session left for later**
    /// ([`with_deferred_session`](self::deferred::with_deferred_session)): `None` in every other session, which
    /// discharges each decision as it is taken.
    deferred: Option<std::collections::HashMap<u32, deferred::Pending>>,
    /// **The nodes the first early walk froze on the session's budget**
    /// ([`froze_on_its_budget`]): a form past a term or degree bound, as
    /// against one the ring's guards refused
    /// ([`form::Refusal::Ring`]).
    budget_frozen: IdSet,
    /// Which table this session's nodes are in ([`table`]): the
    /// drive's, or one of its own.
    table: u64,
    /// **The nodes whose plain form rests on an id no table names**: a
    /// value minted outside every session on this table, and every node
    /// whose form was built over one. Such a form is this leaf's
    /// reading and not a function of the node's id (a leaf whose table
    /// holds the node builds a real form), so it is not published to
    /// the drive's table and leaves this leaf's caches before they go
    /// back to the drive. It decides nothing symbolically that the node
    /// itself would not: the unknown is an indeterminate of its own.
    unknown: IdSet,
    /// Times the retry ladder's growth guard withheld an attempt (read
    /// by the contradiction instrument's report).
    #[cfg(feature = "sym-revalue-testing")]
    withheld: u64,
    /// Whether the contradiction instrument has walked this session: a
    /// decision after it would read the instrument's forms.
    #[cfg(feature = "sym-revalue-testing")]
    instrumented: bool,
    /// Every identity registered in this session, in the order made,
    /// whichever node's: what the contradiction instrument asks its
    /// door question under, after the build. No decision reads it: the
    /// instrument re-asks decisions already taken, from outside every
    /// node's scope, and asks under the union so that an identity ANY
    /// node stated is tested against every definite sign. It is the
    /// instrument's input and not a registry a node can see.
    #[cfg(feature = "sym-revalue-testing")]
    stated: Vec<(SymId, SymId)>,
}

/// **What one node holds in the session while its scope is open**
/// ([`MintScope::node`]): everything a decision reads or writes that is
/// the node's and not the leaf's or the drive's. A node's scope opens
/// empty but for the registry it is handed, so what the node decides
/// and counts is a function of its inputs and of nothing a node that
/// does not feed it did.
#[derive(Default)]
struct NodeState {
    /// **The registered identities this node reads**
    /// ([`Sym::register_equal`]): `node -> the node it was registered
    /// equal to`, resolved transitively by [`Session::alias`] and
    /// consulted by the door walk alone. The identities of the records
    /// the scope has read ([`reads`], and the records it was opened
    /// over) and the ones it has registered since; nothing of the scope
    /// it was opened inside.
    registry: IdMap<SymId>,
    /// The registrations made inside this scope, in the order made, as
    /// the registry recorded them: what [`NodeRecord`] carries.
    identities: Vec<(SymId, SymId)>,
    /// The brackets recorded inside this scope ([`NodeRecord`]).
    brackets: Vec<(u128, (f64, f64))>,
    /// **The door-reduced forms** ([`SymRules::registered`]): the early
    /// walk with `registry` applied, a memo beside the early one so
    /// that a decision the early form answers never pays for the
    /// registry. A function of `registry`, so it is the node's and a
    /// registration clears it; never built while the registry is empty.
    forms_door: IdMap<Arc<Form>>,
    /// **The retry attempts' memos, one entry per attempt beyond the
    /// first** ([`SymRetry`]). An attempt's forms are built under its
    /// own rules or ring bound and are never read into the first
    /// attempt's. [`SymRetry::max_forms`] guards each entry, so the
    /// guard counts this node's forms; a registration clears every
    /// attempt's door memo as it clears `forms_door`.
    retries: Vec<RetryMemo>,
    /// What this node's decisions came to. `frozen` is the plain
    /// freezes made while the scope was open.
    counts: SymCounts,
    /// **The roots the plain walk was asked for**: the `frozen` column
    /// is counted over their closure ([`leaf_need`]). Kept only while a
    /// drive memo is installed.
    plain_roots: IdSet,
    /// The records this scope has read ([`reads`]): what
    /// [`NodeRecord`] carries as `over`.
    over: EachOnce,
    /// The records the scopes opened inside this one read. They are in
    /// this scope's record (`over`) and not in its registry: a family's
    /// value is built over what its members read.
    over_inside: EachOnce,
    /// **What the records carried into this scope came to** ([`carry`]):
    /// the counts of units inside it that were served and did not run.
    /// On the scope's record, and not in the session's receipt, which
    /// counts what ran.
    carried: SymCounts,
    /// What ran on behalf of a unit whose held record is carried
    /// instead ([`carry`]): in the session's receipt, and taken off the
    /// scope's record, where the held record already holds it.
    unrecorded: SymCounts,
    /// The roots of the records carried into this scope.
    carried_roots: Vec<SymId>,
}

/// **Records, each once**, in the order first put: one record reached
/// two ways is one address. Two equal records at two addresses are two.
#[derive(Default)]
struct EachOnce {
    list: Vec<Arc<NodeRecord>>,
    /// `list`'s addresses, compared and never dereferenced.
    held: std::collections::HashSet<*const NodeRecord>,
}

impl EachOnce {
    /// Puts `record` unless it is held; whether it was put.
    fn put(&mut self, record: &Arc<NodeRecord>) -> bool {
        let new = self.held.insert(Arc::as_ptr(record));
        if new {
            self.list.push(Arc::clone(record));
        }
        new
    }
}

impl NodeState {
    /// Adds what a scope opened inside this one came to: its counts
    /// and its roots.
    fn absorb(&mut self, inner: &NodeState) {
        self.counts = with_work(self.counts, inner.counts);
        for id in inner.plain_roots.keys() {
            self.plain_roots.insert(*id, ());
        }
        self.carried.absorb(inner.carried);
        self.unrecorded.absorb(inner.unrecorded);
        self.carried_roots
            .extend(inner.carried_roots.iter().copied());
    }
}

/// **What a node's evaluation leaves for the nodes it feeds and for
/// the leaf's receipt** ([`MintScope::node`]): the identities it
/// registered, what its decisions came to, and the roots its `frozen`
/// column is counted over, and the parameter brackets recorded while
/// its scope was open. It travels with the node's value.
///
/// **What it is a function of**: the node's inputs and the brackets of
/// the parameters its forms reach. The evaluator's content key hashes
/// each input's number and expression and no bracket, so two replays
/// hold equal records under one key only where their boxes agree on
/// those parameters; whatever serves a node across boxes owes that
/// check.
#[derive(Clone, Default)]
pub struct NodeRecord {
    /// The brackets [`Sym::param_over`] recorded inside the scope, by
    /// the parameter's indeterminate, in the order recorded: a replay
    /// served this node's result has not run the node, and holds these
    /// brackets only if it is handed them.
    brackets: Vec<(u128, (f64, f64))>,
    identities: Vec<(SymId, SymId)>,
    counts: SymCounts,
    roots: Vec<SymId>,
    /// **The records the node read that registered an identity**
    /// ([`reads`]), each once, in the order read, with the records each
    /// of those read before it: what the node's registry was built
    /// from. A reader of this node's value reads these before the
    /// node's own, so what reaches a node through another is what that
    /// other's value was built over, and not what stands in a results
    /// map when the reader runs.
    over: Vec<Arc<NodeRecord>>,
}

/// The record's own content and how many records it was built over. A
/// record's `over` is closed, so printing each entry's own `over` in
/// turn would print every record once per path that reaches it.
impl core::fmt::Debug for NodeRecord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NodeRecord")
            .field("brackets", &self.brackets)
            .field("identities", &self.identities)
            .field("counts", &self.counts)
            .field("roots", &self.roots)
            .field("over", &self.over.len())
            .field(
                "identities_over",
                &self
                    .over
                    .iter()
                    .map(|record| record.identities.len())
                    .sum::<usize>(),
            )
            .finish()
    }
}

/// Equal content, and built over records of equal content in the same
/// order. `over` is closed, so each of its entries is compared by its
/// own content and not by its own `over`, which this record's `over`
/// already lists: the comparison is linear in what the record holds.
impl PartialEq for NodeRecord {
    fn eq(&self, other: &Self) -> bool {
        self.own() == other.own()
            && self.over.len() == other.over.len()
            && self
                .over
                .iter()
                .zip(&other.over)
                .all(|(a, b)| Arc::ptr_eq(a, b) || a.own() == b.own())
    }
}

impl NodeRecord {
    /// The record's own content, apart from what it was built over.
    #[allow(clippy::type_complexity)] // four fields, named where they are declared
    fn own(
        &self,
    ) -> (
        &[(u128, (f64, f64))],
        &[(SymId, SymId)],
        SymCounts,
        &[SymId],
    ) {
        (&self.brackets, &self.identities, self.counts, &self.roots)
    }

    /// What the node's decisions came to, the scopes opened inside it
    /// included. `frozen` is zero: that column is a set, read off a
    /// leaf's roots when the leaf is done ([`SymCounts::frozen`]).
    #[must_use]
    pub fn counts(&self) -> SymCounts {
        self.counts
    }

    /// **The records of what one reader reads, as one record**: the
    /// identities one after another in the order given, the counts
    /// added, the roots together. The order is the caller's and has to
    /// be the document's (node ids, a family's members by index), the
    /// same in every replay: which identity a cycle drops and which
    /// node an alias chain ends at follow it.
    #[must_use]
    pub fn gather<'a>(records: impl IntoIterator<Item = &'a NodeRecord>) -> Self {
        let mut all = Self::default();
        let mut over = EachOnce::default();
        for record in records {
            all.brackets.extend(record.brackets.iter().copied());
            all.identities.extend(record.identities.iter().copied());
            all.counts.absorb(record.counts);
            all.roots.extend(record.roots.iter().copied());
            for read in &record.over {
                over.put(read);
            }
        }
        all.over = over.list;
        all.roots.sort_unstable_by_key(|id| id.bits());
        all.roots.dedup();
        all
    }

    /// This record's identities alone: what a reader of a result
    /// receives where the result's counts are entered elsewhere (a
    /// part's run is counted once in a leaf, however many instance
    /// nodes read it).
    #[must_use]
    pub fn identities_alone(&self) -> Self {
        Self {
            identities: self.identities.clone(),
            ..Self::default()
        }
    }

    /// How many identities the node registered.
    #[must_use]
    pub fn registrations(&self) -> usize {
        self.identities.len()
    }

    /// How many identities the node read: the registrations of every
    /// record it was opened over, each record once by address, so two
    /// equal records at two addresses count twice. Read by rows; no
    /// decision reads it.
    #[must_use]
    pub fn read(&self) -> usize {
        self.over.iter().map(|record| record.identities.len()).sum()
    }
}

impl Session {
    /// Records `a ≡ b` in the open node's registry as
    /// [`Sym::register_equal`] does past its witness: resolved through
    /// the registry first, and dropped where the two are already one
    /// node or the record would close a cycle. Answers whether the
    /// registry changed.
    fn record_identity(&mut self, a: SymId, b: SymId) -> Option<(SymId, SymId)> {
        let (a, b) = (self.alias(a), self.alias(b));
        if a == b || self.reaches(b, a) {
            return None;
        }
        self.state(a, b);
        Some((a, b))
    }

    /// Writes `a -> b`, already resolved and checked, into the open
    /// node's registry. Only the DOOR memos can hold a form the record
    /// would have changed, the first attempt's and every retry's; the
    /// plain and early memos never consult the registry.
    fn state(&mut self, a: SymId, b: SymId) {
        self.node.registry.insert(a, b);
        self.node.forms_door.clear();
        for r in &mut self.node.retries {
            r.door.clear();
        }
    }

    /// What the session's decisions have come to so far: every open
    /// scope's counts.
    fn counts_so_far(&self) -> SymCounts {
        self.outer
            .iter()
            .fold(self.node.counts, |sum, outer| with_work(sum, outer.counts))
    }
}

/// `a` and `b` added column by column, `frozen` included: inside one
/// session `frozen` is the plain freezes made so far, a count of work
/// that adds, where [`SymCounts::absorb`] leaves a receipt's set alone.
fn with_work(mut a: SymCounts, b: SymCounts) -> SymCounts {
    a.absorb(b);
    a.frozen += b.frozen;
    a
}

/// Closes the node scope [`node_scope`] opened, by any path including a
/// panic: the enclosing scope's state is the open one again and has
/// absorbed this one's.
struct NodeGuard {
    /// The depth of `Session::outer` with this scope open.
    depth: usize,
}

impl NodeGuard {
    fn close(&self) -> Option<NodeRecord> {
        SESSION.with(|s| {
            let mut slot = s.borrow_mut();
            let sess = slot.as_mut()?;
            // Scopes close innermost first; another depth is a guard
            // closed out of turn, which would hand a node the wrong
            // state. A build without the assertion closes nothing here:
            // the scope's work gets an empty record and the state
            // stays as the scopes still open left it.
            debug_assert_eq!(
                sess.outer.len(),
                self.depth,
                "a node scope was closed out of turn"
            );
            if sess.outer.len() != self.depth {
                return None;
            }
            let outer = sess.outer.pop()?;
            let mut inner = core::mem::replace(&mut sess.node, outer);
            #[cfg(feature = "sym-profile-testing")]
            profile::retry_memos(&inner.retries.iter().map(RetryMemo::len).collect::<Vec<_>>());
            // What the scope read, and what the scopes inside it read.
            let mut over = core::mem::take(&mut inner.over);
            for read in &core::mem::take(&mut inner.over_inside).list {
                over.put(read);
            }
            sess.node.absorb(&inner);
            // What a scope opened inside a node's scope registered is
            // on that node's RECORD (a family holds its members', a
            // part's run its nodes'), and in nobody's registry: a scope
            // reads the records it is opened over and nothing else.
            if !sess.outer.is_empty() {
                sess.node
                    .identities
                    .extend(inner.identities.iter().copied());
                sess.node.brackets.extend(inner.brackets.iter().copied());
                for read in &over.list {
                    sess.node.over_inside.put(read);
                }
            }
            let mut roots: Vec<SymId> = inner
                .plain_roots
                .keys()
                .copied()
                .chain(inner.carried_roots.iter().copied())
                .collect();
            roots.sort_unstable_by_key(|id| id.bits());
            roots.dedup();
            // What ran, less what ran for a unit whose held record is
            // carried, plus what the carried records hold.
            let ran = inner.counts;
            let (held, twice) = (inner.carried, inner.unrecorded);
            Some(NodeRecord {
                brackets: inner.brackets,
                identities: inner.identities,
                counts: SymCounts {
                    frozen: 0,
                    symbolic: (ran.symbolic + held.symbolic).saturating_sub(twice.symbolic),
                    numeric: (ran.numeric + held.numeric).saturating_sub(twice.numeric),
                    retried: (ran.retried + held.retried).saturating_sub(twice.retried),
                    registrations_refused: (ran.registrations_refused + held.registrations_refused)
                        .saturating_sub(twice.registrations_refused),
                },
                roots,
                over: over.list,
            })
        })
    }
}

/// **The open node scope reads `records`**, the records of the values
/// it reads, in the order given: each record that registered an
/// identity enters the scope's registry once, after the records that
/// record itself read ([`NodeRecord`]'s `over`). Nothing outside a
/// session or outside a node scope's work.
///
/// **The order is part of the answer.** Records merge one after
/// another, and which identity a cycle drops and which node an alias
/// chain ends at follow that order. The caller's order has to be the
/// document's (a node's reads in field order), the same in every
/// replay.
pub fn reads<'a>(records: impl IntoIterator<Item = &'a Arc<NodeRecord>>) {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let Some(sess) = slot.as_mut() else { return };
        for record in records {
            for read in record.over.iter().chain([record]) {
                if read.identities.is_empty() || !sess.node.over.put(read) {
                    continue;
                }
                for &(a, b) in &read.identities {
                    let _ = sess.record_identity(a, b);
                }
            }
        }
    });
}

/// **The open scope's record carries `held` in place of `ran`**: a unit
/// inside the scope (a member of a family) was served the result of an
/// earlier run, whose record is `held`, and its own scope has just
/// closed here having run only `ran` (what it did before it was
/// served). The scope's record holds the unit as a run of it would have
/// left it: `held`'s identities and brackets at the unit's place in the
/// order, its counts and roots, and the records it was built over.
///
/// The session's receipt is not moved: it counts what ran, and `ran`
/// ran. Nothing where no node scope is open (the root scope leaves no
/// record), or outside a session.
pub fn carry(held: &Arc<NodeRecord>, ran: &NodeRecord) {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let Some(sess) = slot.as_mut() else { return };
        if sess.outer.is_empty() {
            return;
        }
        let node = &mut sess.node;
        // `ran`'s scope closed inside this one and left its identities
        // and brackets last.
        let kept = node.identities.len().saturating_sub(ran.identities.len());
        node.identities.truncate(kept);
        node.identities.extend(held.identities.iter().copied());
        let kept = node.brackets.len().saturating_sub(ran.brackets.len());
        node.brackets.truncate(kept);
        node.brackets.extend(held.brackets.iter().copied());
        node.carried.absorb(held.counts);
        node.unrecorded.absorb(ran.counts);
        node.carried_roots.extend(held.roots.iter().copied());
        for read in &held.over {
            node.over_inside.put(read);
        }
    });
}

impl Drop for NodeGuard {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// **A scope of its own for a unit of work that is no document node**:
/// a part's run, the product's gather. Opened over no identity, it
/// decides and counts apart from every node, and the record it leaves
/// goes where the unit's result goes, so that every decision of a
/// replay is on a record some reader of its result receives.
///
/// The product's gather reads no registration this way, though the
/// instances it gathers feed it: its doors hold pieces and no node
/// value
/// (`work/boxfinish/a-documents-registrations-do-not-reach-its-products-gather`).
pub fn apart<R>(work: impl FnOnce() -> R) -> (R, NodeRecord) {
    node_scope([], work)
}

/// [`MintScope::node`]'s session half: `work` in a node scope of its
/// own, opened over `upstream`'s registrations in the order given and
/// over nothing of the scope it is opened inside, and the record it
/// leaves. Outside
/// a session there is no state to scope and the record is empty.
pub(crate) fn node_scope<'a, R>(
    upstream: impl IntoIterator<Item = &'a NodeRecord>,
    work: impl FnOnce() -> R,
) -> (R, NodeRecord) {
    let opened = SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        let outer = core::mem::take(&mut sess.node);
        sess.outer.push(outer);
        for record in upstream {
            for &(a, b) in &record.identities {
                let _ = sess.record_identity(a, b);
            }
        }
        Some(NodeGuard {
            depth: sess.outer.len(),
        })
    });
    let out = work();
    let record = opened.and_then(|guard| {
        let record = guard.close();
        core::mem::forget(guard);
        record
    });
    (out, record.unwrap_or_default())
}

/// One memoized per-node reduction ([`Session::reductions`]).
struct Reduction {
    input: Arc<Form>,
    rules: SymRules,
    bits: u64,
    /// The ids the reduction looked up in the atom table and did not
    /// find ([`algebra::reduce_noting`]), sorted — the entry answers
    /// only while every one of them is still absent.
    absent: Box<[u128]>,
    out: Option<Arc<Form>>,
}

/// **The per-node reduction memo's cap** ([`Session::reductions`]):
/// past this many entries a session stops inserting, and a reduction
/// it would have kept is computed on every ask, as it was before the
/// memo. The memo is a pure cache — an entry answers exactly what the
/// reduction would build ([`reduce_per_node`]) — so the cap moves no
/// form and no decision, only the time a leaf past it takes.
///
/// **Measured**, the most entries one whole-box leaf's session holds
/// (`profile::SymProfile::reduction_memo`, release leaf instrument):
/// R2's rounded pad 2,161 at `1e2·ε` and 4,217 with the kept-atom
/// ladder; R2's link 1,270 and 2,376 at `1e1·ε`; R2's bracket 575 and
/// 1,289. An entry is kept only for a form carrying a squared atom, at
/// most one per distinct form per rule set and ring bound, and a form
/// the walk reduces is under `EARLY_AB_TERMS` terms. The cap sits more
/// than ten times above the pad with its ladder, so no measured
/// document reaches it, and one that does degrades into the time the
/// reduction cost before the memo, not into memory.
const REDUCTION_FORMS: usize = 50_000;

/// **One retry attempt's two memos** — the early walk's and the door
/// walk's, under that attempt's rules and ring bound.
///
/// The plain walk has no entry here: the plain rung is never retried
/// ([`SymRetry`]), so the plain form a decision is asked of first is
/// the first attempt's on every attempt.
#[derive(Default)]
struct RetryMemo {
    early: IdMap<Arc<Form>>,
    door: IdMap<Arc<Form>>,
}

impl RetryMemo {
    /// The forms this attempt is holding, both walks.
    fn len(&self) -> usize {
        self.early.len() + self.door.len()
    }
}

/// **The GROWTH GUARD's default** ([`SymRetry::max_forms`]): past this
/// many forms in one attempt's two memos the attempt is not offered
/// again to the node that built them, and the decisions that would
/// have asked it stay numeric.
///
/// A retry pays a second walk of the DAG per refused decision, and the
/// forms it builds are a second population beside the first attempt's —
/// so without a cap a leaf whose refusals are many and whose DAG is
/// large would hold two of everything. **Measured**, the most one
/// attempt of `SymRetry::kept_atom` holds as a session ends
/// (`profile::SymProfile::retry_forms`): R2's rounded pad 7,465 forms
/// against a DAG of 32,698 nodes (one whole-box leaf at `1e2·ε`), R2's
/// link 5,259 against 19,564 and R2's bracket 2,716 against 28,996 (the
/// nominal), R1's segment boss 80; the plate and the annulus none. And
/// an attempt holds at most one form per node id in each of its two
/// memos — about 65,000 on the pad. The cap sits above both, so no measured
/// document reaches it, and a document that does degrades into missed
/// cancellations rather than into memory.
///
/// **The check is before the walk** (`ladder`), so an attempt admitted
/// just under the cap can add one walk's worth past it: the bound on an
/// attempt's memos is this plus two forms per node id. The row that
/// reaches it is `the_growth_guard_withholds_an_attempt_at_its_cap`.
const RETRY_FORMS: usize = 200_000;

impl Session {
    /// **The one place a session is built** — every field of it, in one
    /// literal, so a field added here cannot leave a second literal
    /// somewhere else half-initialised (the `trig` test module kept one,
    /// and it is this now).
    fn new(
        budget: SymBudget,
        rules: SymRules,
        retry: SymRetry,
        memo: Option<Arc<DriveMemo>>,
    ) -> Self {
        // A leaf of a drive starts with the caches an earlier leaf
        // left (`memo`'s header): entries of the drive's table.
        let warm = memo.as_ref().map(|m| m.warm()).unwrap_or_default();
        let table = memo.as_ref().map_or_else(memo::table_token, |m| m.token());
        Self {
            budget,
            rules,
            retry,
            node: NodeState::default(),
            outer: Vec::new(),
            nodes: warm.nodes,
            forms: warm.forms,
            forms_early: IdMap::default(),
            params: IndetMap::default(),
            mints: IndetMap::default(),
            atoms: warm.atoms,
            witness: None,
            trig_closed: IndetMap::default(),
            reductions: IndetMap::default(),
            reductions_held: 0,
            #[cfg(test)]
            reduction_hits: 0,
            memo,
            nodes_in: None,
            deferred: None,
            budget_frozen: IdSet::default(),
            unknown: IdSet::default(),
            table,
            #[cfg(feature = "sym-revalue-testing")]
            withheld: 0,
            #[cfg(feature = "sym-revalue-testing")]
            instrumented: false,
            #[cfg(feature = "sym-revalue-testing")]
            stated: Vec::new(),
        }
    }

    /// **The node `id` names**: in this leaf's cache, or in the drive's
    /// table where another leaf recorded it. `None` for an id minted
    /// outside every session on this table.
    fn node_of(&self, id: SymId) -> Option<SymNode> {
        self.nodes
            .get(&id)
            .copied()
            .or_else(|| self.memo.as_ref().or(self.nodes_in.as_ref())?.node(id))
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
            match self.node.registry.get(&cur) {
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
            if let Some(node) = self.node_of(id) {
                stack.extend(node.kids().iter().copied());
            }
        }
        false
    }

    /// **Whether a node under `from` in this table has an op `hit`
    /// answers**, unaliased: the one walk the opaque-atom reading and the
    /// free-identity exception ask.
    fn reaches_op(&self, from: SymId, mut hit: impl FnMut(SymOp) -> bool) -> bool {
        let mut seen: IdSet = IdSet::default();
        let mut stack = vec![from];
        while let Some(id) = stack.pop() {
            if seen.insert(id, ()).is_some() {
                continue;
            }
            if let Some(node) = self.node_of(id) {
                if hit(node.op) {
                    return true;
                }
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
    /// An id no table names has no children here, as in the plain
    /// walk, which freezes it and stops.
    fn closure(&self, roots: impl Iterator<Item = SymId>) -> IdSet {
        let mut seen: IdSet = IdSet::default();
        let mut stack: Vec<SymId> = roots.collect();
        while let Some(id) = stack.pop() {
            if seen.insert(id, ()).is_some() {
                continue;
            }
            if let Some(node) = self.node_of(id) {
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
}

/// Runs `f` with a fresh symbolic session installed on this thread,
/// answering its result beside the session's counts.
///
/// This door installs no drive's table, so the session's own tables
/// are the whole of what it knows and are dropped with it: a node
/// minted in it means nothing in another session. Nesting is refused
/// rather than silently flattened, since an inner session would count
/// a different replay's decisions into the outer one's receipt.
/// [`with_session_memo`] is the spelling a driver uses for the leaves
/// of one drive.
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
    with_session_in(budget, rules, SymRetry::none(), None, f)
}

/// [`with_session_rules`] with a RETRY LADDER installed ([`SymRetry`]):
/// a decision every rung of the first attempt refuses is re-asked at a
/// wider ring, or with a rule that opens an atom shut, or both.
///
/// One of the two doors that install one — this and, with a drive's
/// plain memo, [`with_session_memo_retry`]. Every other door here runs
/// [`SymRetry::none`], so a caller that has not asked for the ladder
/// builds the session it built before this unit — which is what makes
/// the ladder's effect on a document a differential and not an
/// assumption.
pub fn with_session_retry<R>(
    budget: SymBudget,
    rules: SymRules,
    retry: SymRetry,
    f: impl FnOnce() -> R,
) -> (R, SymCounts) {
    with_session_in(budget, rules, retry, None, f)
}

/// [`with_session_rules`] on a DRIVE's table ([`DriveMemo`]): the
/// leaf records its nodes, plain forms and atom records in `memo` as
/// it makes them and reads what the drive's other leaves made. The
/// early memo, the parameter brackets and every node's state stay the
/// leaf's. The decision counts the call answers are this leaf's own;
/// its [`SymCounts::frozen`] is this leaf's NEED, counted at the
/// leaf's end against the drive's frozen set, and the drive's own
/// column is [`DriveMemo::frozen`].
///
/// **The memo is valid for one `(budget, rules)` pair** and refuses a
/// leaf that does not match it: a plain form is a function of the node
/// id and those two, so serving one across a budget change would hand
/// back a form the leaf would not have built.
///
/// The mismatch is a `debug_assert!`, which is loud in every profile
/// this workspace builds — `[profile.release]` keeps debug assertions
/// on. A build that turned them OFF would run the leaf with NO memo
/// instead: sound but quiet, and its freezes would never be published,
/// so the drive's `frozen` column would under-count in exactly that
/// build. No configuration in this repo reaches it.
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
    with_session_in(
        budget,
        rules,
        SymRetry::none(),
        accepts.then(|| Arc::clone(memo)),
        f,
    )
}

/// [`with_session_memo`] with a RETRY LADDER installed beside the
/// drive's plain memo — the door a DRIVE uses, and the only one that
/// takes both.
///
/// **The table is unaffected by the ladder and the check above does
/// not widen.** It holds PLAIN forms and the plain rung is never
/// retried ([`SymRetry`]), so a form it serves is the form this leaf
/// would build at any ladder; what a retry builds is its node's and
/// is dropped with the node's scope.
pub fn with_session_memo_retry<R>(
    budget: SymBudget,
    rules: SymRules,
    retry: SymRetry,
    memo: &Arc<DriveMemo>,
    f: impl FnOnce() -> R,
) -> (R, SymCounts) {
    let accepts = memo.accepts(budget, rules);
    debug_assert!(
        accepts,
        "a drive memo is valid for the budget and rules it was made for"
    );
    with_session_in(budget, rules, retry, accepts.then(|| Arc::clone(memo)), f)
}

fn with_session_in<R>(
    budget: SymBudget,
    rules: SymRules,
    retry: SymRetry,
    memo: Option<Arc<DriveMemo>>,
    f: impl FnOnce() -> R,
) -> (R, SymCounts) {
    let nested = session_installed();
    // A nested call in a release build runs with the OUTER session,
    // which is sound — ids are content hashes and the table is keyed by
    // them — and only muddles whose receipt the decisions land in. It
    // keeps the outer session's mint scope too: a root scope reopened
    // at zero would mint the outer session's names again.
    debug_assert!(!nested, "symbolic sessions do not nest");
    if nested {
        return (f(), SymCounts::default());
    }
    // The session's root mint scope, counting from zero (`mint`).
    let _root = mint::SessionRoot::open();
    SESSION.with(|s| {
        *s.borrow_mut() = Some(Session::new(budget, rules, retry, memo));
    });
    #[cfg(feature = "sym-profile-testing")]
    profile::session_start();
    // A leaf that panics takes its session with it: left installed, it
    // would refuse every later session on this thread as nested and
    // keep the drive's table alive.
    struct Installed;
    impl Drop for Installed {
        fn drop(&mut self) {
            drop(SESSION.with(|s| s.borrow_mut().take()));
        }
    }
    let installed = Installed;
    let out = f();
    let mut sess = SESSION.with(|s| s.borrow_mut().take());
    drop(installed);
    #[cfg(feature = "sym-profile-testing")]
    if let Some(s) = &sess {
        profile::session_done(s.nodes.len(), s.atoms.len());
        profile::retry_memos(
            &s.node
                .retries
                .iter()
                .map(RetryMemo::len)
                .collect::<Vec<_>>(),
        );
        profile::reduction_memo(s.reductions_held);
    }
    // **The leaf's `frozen` column is its NEED**, read here, when every
    // form it built is in the drive's table with its freeze
    // ([`SymCounts::frozen`] argues the column).
    if let Some(s) = &mut sess
        && let Some(memo) = s.memo.clone()
    {
        s.node.counts.frozen = leaf_need(s, &memo);
        for id in s.unknown.keys() {
            s.forms.remove(id);
        }
        // The leaf's caches go back to the drive for the next leaf.
        memo.rest(memo::Warm {
            nodes: core::mem::take(&mut s.nodes),
            forms: core::mem::take(&mut s.forms),
            atoms: core::mem::take(&mut s.atoms),
        });
    }
    let counts = sess.map_or_else(SymCounts::default, |s| s.node.counts);
    (out, counts)
}

/// **The leaf's `frozen` column**: the drive's frozen nodes inside the
/// closure of the roots this leaf's plain walk was asked for.
/// `memo`'s header argues why that is the same number under every
/// schedule, and [`SymCounts::frozen`] says what the column means.
fn leaf_need(sess: &Session, memo: &DriveMemo) -> u64 {
    // A drive that froze nothing answers 0 without a walk: one read
    // lock per leaf (the M10-3 slab is that document).
    if memo.frozen_is_empty() && sess.unknown.is_empty() {
        return 0;
    }
    let reached = sess.closure(sess.node.plain_roots.keys().copied());
    // An unknown under the roots is a node this leaf's reasoning rests
    // on as an indeterminate, in no drive's set.
    let unknown = sess
        .unknown
        .keys()
        .filter(|id| reached.contains_key(id) && sess.node_of(**id).is_none())
        .count() as u64;
    memo.need(&reached) + unknown
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
    SESSION.with(|s| s.borrow().as_ref().map(Session::counts_so_far))
}

/// **Whether a decision may be taken on a worker thread at all** — the
/// test every walk that maps deciding units onto rayon workers reads
/// before it maps (`topo::props`' face walks, `editor_core`'s node
/// schedule), and the one home of that test.
///
/// **It covers more than decisions.** It is false while a symbolic
/// session is installed, which changes what a decision answers, AND
/// while the shape report is installed, which changes nothing a
/// decision answers but records every one in a thread-local of the
/// deciding thread. The name is the session half's; the report half is
/// the third bullet below.
///
/// The K-funnel's frame and sample sink are thread-local too, but they
/// compose back: a unit run under `k_stats::detached` hands its
/// recording back as a value and the caller's fold splices it
/// (`k_stats::map_detached`). The two thread-locals read here do not,
/// and three things follow. The first is about the answer, not about
/// what is recorded; the other two are about recording:
///
/// - **The decision itself changes.** `Sym`'s `sign_within` consults
///   the session; with none installed `discharge` answers `None`, the
///   identity tier discharges nothing and the answer is the plain
///   numeric one. A unit on a worker would decide differently from its
///   siblings on the caller's thread — the one thing D9 forbids
///   outright.
/// - **The receipt is written in place.** `count_decision` mutates
///   the installed session's [`SymCounts`], and [`Sym::opaque`] advances
///   the count of the mint scope open on its thread ([`mint`]), which a
///   session keeps while the scope is closed. Neither is a
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
    !session_installed() && !report::active()
}

/// **Which expression table the installed session's nodes are in**: a
/// token for the drive's table, the same in every leaf of the drive,
/// or for the session's own where no drive installed one; `None`
/// outside a session. A value built under one table means nothing
/// under another, so whatever hands a value from one session to
/// another compares these.
#[must_use]
pub fn table() -> Option<u64> {
    SESSION.with(|s| s.borrow().as_ref().map(|s| s.table))
}

/// **Whether a symbolic session is installed on this thread** — the one
/// home of that test. A value minted outside every session on the
/// installed session's table is an unknown there (`Session::unknown`:
/// it proves nothing, and nothing built over it is shared), so
/// whatever keeps a `Sym` value across calls reads this
/// before it serves one: [`decisions_are_thread_portable`] does, and so
/// does the NURBS speed meter's memo, which serves and fills only while
/// it is false.
#[must_use]
pub fn session_installed() -> bool {
    SESSION.with(|s| s.borrow().is_some())
}

/// **A run whose decision logs can be set aside**: `work`'s value, and
/// a mark of this thread's per-decision `Sym` logs as they stood before
/// it (the shape report's decisions, and under `sym-revalue-testing`
/// the re-valuation recorder's). [`Aside::discard`] cuts both back to
/// the mark, so a reading its caller drops leaves no decision in them;
/// [`Aside::keep`] keeps what `work` recorded. The k_stats
/// channels have their own door (`k_stats::detached`).
///
/// The profile (`sym-profile-testing`) is not a decision log: it charges
/// the forms walked and the time spent, and a dropped reading's work was
/// done, so it keeps it. Two per-decision records are not set aside
/// either: the session's [`SymCounts::symbolic`] count (the E12 receipt)
/// and the deferring session's list of undischarged Zeros
/// ([`deferred::with_deferred_session`]) keep a dropped reading's
/// decisions.
pub fn aside<R>(work: impl FnOnce() -> R) -> (R, Aside) {
    let mark = Aside {
        shapes: report::mark(),
        #[cfg(feature = "sym-revalue-testing")]
        decisions: revalue::mark(),
    };
    (work(), mark)
}

/// [`aside`]'s mark of the per-decision logs.
#[must_use = "an aside that is neither discarded nor dropped on purpose keeps its decisions"]
pub struct Aside {
    shapes: usize,
    #[cfg(feature = "sym-revalue-testing")]
    decisions: Option<usize>,
}

impl Aside {
    /// Keeps every decision the set-aside run recorded: the reading is
    /// the caller's.
    pub fn keep(self) {}

    /// Cuts the per-decision logs back to the mark: every decision the
    /// set-aside run recorded is gone from them, and none before it is.
    pub fn discard(self) {
        report::truncate(self.shapes);
        #[cfg(feature = "sym-revalue-testing")]
        revalue::truncate(self.decisions);
    }
}

/// Records `node` in the installed session and answers its id. Outside a
/// session the id is still computed — it is a pure function of the node
/// — and nothing is stored.
fn intern(node: SymNode) -> SymId {
    let id = node.id();
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            // A node new to this leaf's cache is recorded in the
            // drive's table too. Hash-consing means a node is interned
            // many times and is new once.
            if let std::collections::hash_map::Entry::Vacant(e) = sess.nodes.entry(id) {
                e.insert(node);
                if let Some(memo) = &sess.memo {
                    memo.record_node(id, node);
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

/// The indeterminate ONE OPAQUE VALUE enters the form as, keyed by the
/// name it was minted under ([`mint`]): its scope and its ordinal
/// there, which no other mint a form can meet shares.
fn indet_opaque(scope: u128, ordinal: u64) -> u128 {
    Hash128::new()
        .word(0x4f50_4151_5545_5f5f)
        .wide(scope)
        .word(ordinal)
        .finish()
}

/// The indeterminate a limb identity's free parameter enters the form
/// as ([`SymOp::Free`]): keyed apart from every parameter symbol and
/// opaque value, and recorded in no session's brackets.
const INDET_FREE: u128 = 0x4652_4545_5f50_4152_414d_5f5f_5f5f_5f5f;

/// The indeterminate a parameter symbol enters the form as.
fn indet_param(symbol: u64) -> u128 {
    Hash128::new()
        .word(0x5041_5241_4d5f_494e)
        .word(symbol)
        .finish()
}

/// The indeterminate a `Hull` node enters the form as: keyed by its
/// two CHILD IDS, never by their forms (the op's docs).
fn indet_hull(a: SymId, b: SymId) -> u128 {
    Hash128::new()
        .word(SymOp::Hull.tag())
        .wide(a.bits())
        .wide(b.bits())
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

/// The exact rational a form stands for, where both halves of the
/// quotient are constants — the reading A0's folds are made of, in
/// one place so `sqrt`, `abs`, the decision door and `min`/`max` all
/// ask it the same way.
fn constant_value(f: &Form) -> Option<Rat> {
    if f.poisoned {
        return None;
    }
    f.num.as_constant()?.mul(&f.den.as_constant()?.recip()?)
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
        | SymOp::Floor
        | SymOp::LessSin => Some(Form::zero()),
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

/// **The value a sine, cosine or tangent atom takes at a rational
/// multiple of π, where that value is rational** (D10: a rational
/// fraction of a turn is a constant): `0`, `±1/2` or `±1` for a sine or
/// cosine, `0` or `±1` for a tangent. π is an indeterminate of the
/// form, so the argument `q·π` is read off it exactly and the fold is
/// an equality of reals.
fn trig_at_rational_pi(op: SymOp, a: &Form) -> Option<Form> {
    if !matches!(op, SymOp::Sin | SymOp::Cos | SymOp::Tan) || a.poisoned {
        return None;
    }
    let [(mono, c)] = a.num.terms() else {
        return None;
    };
    if mono.as_slice() != [(INDET_PI, 1)] {
        return None;
    }
    // `q`, the multiple of π, as an integer fraction `p / s`.
    let q = c.mul(&a.den.as_constant()?.recip()?)?;
    let (rational::Int::Small(num), rational::Int::Small(den)) = (&q.num, &q.den) else {
        return None;
    };
    let shift = q.exp2.unsigned_abs();
    let (p, s) = if q.exp2 >= 0 {
        (num.checked_mul(1i128.checked_shl(shift)?)?, *den)
    } else {
        (*num, den.checked_mul(1i128.checked_shl(shift)?)?)
    };
    // `q·π` is `q/2` turns.
    let turns = crate::Turns::new(
        i64::try_from(p).ok()?,
        u64::try_from(s.checked_mul(2)?).ok()?,
    )?;
    let halves = |h: i8| Rat::new(i128::from(h), 2, 0).map(|r| Form::poly(Poly::constant(r)));
    match op {
        SymOp::Sin => halves(turns.sin_halves()?),
        SymOp::Cos => halves(turns.cos_halves()?),
        _ => halves(2 * turns.tan_rational()?),
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
fn combine(node: &SymNode, kids: [&Form; 3], sess: &mut Session, early: bool) -> Option<Form> {
    // Rule H's deferred fold reaches every argument but a ring op's
    // (`turn::settle`), so an atom, a reciprocal and a decision read
    // the value. A reciprocal's is the first line: a zero it settles to
    // is refused where it is made, and the output's settle (`turn::over`)
    // stands behind it.
    let held: [Option<Form>; 3];
    let kids = if early && sess.rules.angle_sum {
        let settles = |i: usize| match node.op {
            SymOp::Add | SymOp::Sub | SymOp::Mul | SymOp::Neg => false,
            SymOp::Powi => (node.payload as u32 as i32) < 0,
            SymOp::Select => i == 0,
            _ => true,
        };
        held = std::array::from_fn(|i| settles(i).then(|| turn::settle(kids[i], sess)).flatten());
        [0, 1, 2].map(|i| held[i].as_ref().unwrap_or(kids[i]))
    } else {
        kids
    };
    let (a, b, third) = (kids[0], kids[1], kids[2]);
    let budget = sess.budget;
    // Where A0 applies: in the early walk when one is configured
    // (ALONGSIDE — the plain form stays M10-7's and can lose nothing),
    // otherwise in the plain form (REPLACING — cheaper, and measured
    // to lose theorems to coefficient freezes at the ring's bound:
    // `SymRules::const_fold`).
    let a0 = sess.rules.const_fold && (early || !sess.rules.early);
    // Rule F applies in the EARLY walk only (`SymRules::manifest_sign`).
    let f_sign = early && sess.rules.manifest_sign;
    // Rule G, the canonical root, likewise (`SymRules::canonical_root`).
    let g_root = early && sess.rules.canonical_root;
    // The decision read, likewise (`SymRules::decision_read`), and only
    // at an exact witness (`Session::witness`).
    let read =
        early && sess.rules.decision_read && sess.witness == Some(crate::real::Witness::Exact);
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
            return Some(f);
        }
        if let Some(f) = trig_at_rational_pi(op, a) {
            return Some(f);
        }
        let id = indet_atom(op.tag(), node.payload, &[a.digest()]);
        mint_atom(sess, id, early, || AtomInfo {
            op,
            payload: node.payload,
            args: [Some(Arc::new(a.clone())), None, None],
        });
        Some(Form::poly(Poly::indet(id)))
    };
    match node.op {
        SymOp::Param => Some(Form::poly(Poly::indet(indet_param(node.payload)))),
        // A limb identity's free parameter: one indeterminate, with no
        // bracket (`SymOp::Free`'s docs).
        SymOp::Free => Some(Form::poly(Poly::indet(INDET_FREE))),
        // One untracked real: its OWN indeterminate, keyed by the name
        // the node carries (`SymOp::Opaque`'s docs).
        SymOp::Opaque => Some(Form::poly(Poly::indet(node.opaque_indet()))),
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
            Some(match (node.op, a.is_zero()) {
                (SymOp::Mul, _) => Form::zero(),
                (SymOp::Add, true) => b.clone(),
                (SymOp::Add, false) => a.clone(),
                (_, true) => b.neg()?,
                (_, false) => a.clone(),
            })
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
        // and `−X` where it shows `X` negative, which reads no value.
        SymOp::Sqrt | SymOp::Abs if (a0 || f_sign || g_root) && !a.poisoned => {
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
                return Some(Form::poly(Poly::constant(k)));
            }
            if f_sign
                && node.op == SymOp::Abs
                && let Some(f) = manifest::fold_abs(a, sess)
            {
                return Some(f);
            }
            // **Rule G**, last of the `sqrt` folds: the ones above
            // answer the node outright where they fire, and this one
            // decides how the atom that is left is KEYED. `trig`'s
            // hand-built roots and the registrant's forms reach the
            // same door ([`root::mint`]), which is what makes the
            // keying uniform rather than per-site.
            if g_root {
                if node.op == SymOp::Sqrt
                    && let Some(f) = root::canonical(a, sess)
                {
                    return Some(f);
                }
                // An `abs` NODE goes through rule G's atom door too,
                // and for the same reason: `|Y|` and `|−Y|` are one
                // real, so the atom is keyed on the sign-normalised
                // argument and a root of a perfect square meets the
                // node whichever way round the document spelled it.
                // The door mints; it folds nothing, so every rule
                // above keeps its own predicate.
                if node.op == SymOp::Abs
                    && let Some(f) = root::magnitude_atom(a, sess)
                {
                    return Some(f);
                }
            }
            atom1(node.op, sess)
        }
        // Rule H (early walk only): `sin`/`cos` over the canonical
        // argument, rule D's closed form asked of its rest; where it
        // does not fit, rule D (early walk only): `sin`/`cos` of `q ·
        // atan(X)` in closed form; any other argument shape keeps the
        // atom.
        SymOp::Sin | SymOp::Cos
            if early && (sess.rules.angle_sum || sess.rules.trig_of_atan) && !a.poisoned =>
        {
            if sess.rules.angle_sum
                && let Some(f) = turn::mint(node.op, a, sess)
            {
                return Some(f);
            }
            if !sess.rules.trig_of_atan {
                return atom1(node.op, sess);
            }
            #[cfg(feature = "sym-profile-testing")]
            let t0 = profile::clock();
            let folded = trig::fold(node.op, a, sess);
            #[cfg(feature = "sym-profile-testing")]
            profile::trig_done(t0);
            match folded {
                Some(f) => Some(f),
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
        | SymOp::Floor
        | SymOp::LessSin => atom1(node.op, sess),
        SymOp::Atan2 | SymOp::Min | SymOp::Max | SymOp::Copysign => {
            if a.tainted(b) {
                return Some(Form::poison());
            }
            // `copysign` carries `a`'s MAGNITUDE, so a zero first
            // argument is zero whatever the sign argument does (±0 is
            // one real).
            if node.op == SymOp::Copysign && a.is_zero() {
                return Some(Form::zero());
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
                && let Some(m) = manifest::magnitude(a, sess)
            {
                return Some(m);
            }
            // The same rule's NEGATIVE arm: `copysign(Y, X) = −|Y|`
            // wherever the form of `X` is manifestly NEGATIVE (the
            // predicate of `−X`; `manifest` carries the reflection).
            if node.op == SymOp::Copysign
                && f_sign
                && manifest::negative(b, sess)
                && let Some(m) = manifest::magnitude(a, sess)
                && let Some(m) = m.neg()
            {
                return Some(m);
            }
            // min(0, 0) and max(0, 0) are zero; a one-sided zero says
            // nothing, so only the both-zero fold is taken.
            // atan2(0, x) is 0 or π depending on the sign of x, so the
            // fold below is taken ONLY where the sign is a fact of the
            // form: atan2(0, N) with N non-negative BY SYNTAX is 0 —
            // rule D's second fold (amendment A1), early walk only
            // (`manifest::nonneg` carries the argument); a plain
            // parameter, a non-zero first argument, or a value-only
            // zero never folds, and every other atan2 stays an atom.
            // **A0 at `min`/`max`**: two rational CONSTANTS compare
            // EXACTLY, so the node is one of them. It reads no value —
            // the comparison is arithmetic on the coefficient ring, the
            // same fold A0 already makes at `sqrt`, at `abs` and at the
            // decision door — and what it reaches is a THEOREM.
            //
            // Without it a frame's conditioning floor over an
            // axis-aligned normal stayed an opaque atom chain on
            // geometry with no parameter in it at all, and the only
            // thing that could answer it was the certified READ: a
            // fact of the form reported as one conditional on the
            // leaf's box. `work/decide/a0-leaves-max-and-min-of-constants-opaque`
            // is the row that measured that and this is its fix.
            // **A0 at `min`/`max` of EQUAL forms**: `min(A, A)` and
            // `max(A, A)` are `A`, whatever `A` is worth. One digest
            // comparison, no value, and it subsumes the both-zero fold
            // below on the arm A0 is on.
            if a0 && matches!(node.op, SymOp::Min | SymOp::Max) && a.digest() == b.digest() {
                return Some(a.clone());
            }
            if a0
                && matches!(node.op, SymOp::Min | SymOp::Max)
                && let Some(x) = constant_value(a)
                && let Some(y) = constant_value(b)
                && let Some(d) = y.add(&x.neg()?)
            {
                // `x ≤ y` exactly: the difference is non-negative.
                let x_le_y = !d.is_negative();
                let pick = match (node.op, x_le_y) {
                    (SymOp::Min, true) | (SymOp::Max, false) => x,
                    _ => y,
                };
                return Some(Form::poly(Poly::constant(pick)));
            }
            let folds = match node.op {
                SymOp::Min | SymOp::Max => a.is_zero() && b.is_zero(),
                SymOp::Atan2 => {
                    early && sess.rules.trig_of_atan && a.is_zero() && manifest::nonneg(b, sess)
                }
                _ => false,
            };
            if folds {
                return Some(Form::zero());
            }
            // **The decision read at `min`/`max`**, behind every fold
            // above that reads no value (`SymRules::decision_read`):
            // `max(A, B)` IS `select(B − A, A, B)`, so the arm is the
            // same certified read the decision door takes, and it is
            // asked last, behind every fold that reads no value.
            if read
                && matches!(node.op, SymOp::Min | SymOp::Max)
                && let Some(f) = signed::order(node.op, a, b, sess, budget)
            {
                return Some(f);
            }
            let id = indet_atom(node.op.tag(), node.payload, &[a.digest(), b.digest()]);
            mint_atom(sess, id, early, || AtomInfo {
                op: node.op,
                payload: node.payload,
                args: [Some(Arc::new(a.clone())), Some(Arc::new(b.clone())), None],
            });
            Some(Form::poly(Poly::indet(id)))
        }
        // The decision door: an indeterminate of its three arguments'
        // forms, EXCEPT where rule A0 can read the decision exactly.
        // Which arm the door reads is a question about the decision's
        // VALUE, and the form holds that value whenever it is a
        // CONSTANT: the comparison is `d <= 0` on an exact rational, so
        // the arm is determined and the atom is not needed. Without
        // this fold every frame minted through
        // [`Vec3::orthonormal_basis`](crate::Vec3::orthonormal_basis)
        // is opaque to the tier even where its normal is an axis
        // direction, and every identity over a face built on that frame
        // freezes. A non-constant decision keeps the atom; a
        // both-candidates-equal fold would still have to prove the
        // decision describable, and does not happen here.
        SymOp::Select => {
            if a.tainted(b) || a.tainted(third) || b.tainted(third) {
                return Some(Form::poison());
            }
            if a0 && let Some(c) = constant_value(a) {
                let arm = if c.is_zero() || c.is_negative() {
                    b
                } else {
                    third
                };
                return Some(arm.clone());
            }
            // **The decision read**, behind A0 — the only fold at this
            // door that reads no value (`SymRules::decision_read`).
            // Where the decision's sign is certified over the whole
            // box the arm is determined there, and the form the door
            // takes is the arm's.
            if read && let Some(le) = signed::decision(a, sess) {
                return Some(if le { b } else { third }.clone());
            }
            let id = indet_atom(
                node.op.tag(),
                node.payload,
                &[a.digest(), b.digest(), third.digest()],
            );
            mint_atom(sess, id, early, || AtomInfo {
                op: node.op,
                payload: node.payload,
                args: [
                    Some(Arc::new(a.clone())),
                    Some(Arc::new(b.clone())),
                    Some(Arc::new(third.clone())),
                ],
            });
            Some(Form::poly(Poly::indet(id)))
        }
        // Keyed by the CHILD IDS, never by their forms (the op's docs).
        // A hull of something with no value has none either, so the
        // poison crosses this door like every other.
        SymOp::Hull if a.tainted(b) => Some(Form::poison()),
        SymOp::Hull => Some(Form::poly(Poly::indet(indet_hull(
            node.kids[0],
            node.kids[1],
        )))),
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
/// and the rest of the algebra — ALONGSIDE the plain memo,
/// never replacing it, so a rule can only ADD a discharge and never
/// lose one the plain form reached. That split is measured, not
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
    first: bool,
) -> Arc<Form> {
    // The walk [`early_form`] runs: the one whose freezes
    // [`froze_on_its_budget`] reads the cause of.
    let notes_causes = first && early && !registry;
    // The freeze this WALK made. The running count is the leaf's WORK,
    // and it is what the column says only outside a drive: a leaf of a
    // drive has its column rewritten as its NEED at the leaf's end
    // ([`leaf_need`], which [`SymCounts::frozen`] argues).
    let frozen = |sess: &mut Session, id: SymId| -> Arc<Form> {
        if !early {
            sess.node.counts.frozen += 1;
        }
        Arc::new(Form::poly(Poly::indet(id.bits())))
    };
    // **One predicate for the plain walk**, and the drive's memo behind
    // it: the table is the PLAIN walk's alone, because the early walk
    // reads this leaf's parameter brackets and the door walk a node's
    // registry.
    let plain = !early && !registry;
    let drive = plain.then(|| sess.memo.clone()).flatten();
    // **The walk root, noted for the leaf's NEED** ([`leaf_need`]): the
    // column counts the drive's freezes inside the closure of what this
    // leaf ASKED, and the asking is here.
    if drive.is_some() {
        sess.node.plain_roots.insert(root, ());
    }
    // **One place that notes what the plain walk computed**: the
    // profile's distinct-id counter, and the form's publication to the
    // drive's table with its freeze (`DriveMemo::publish_form`).
    let note = |id: SymId, form: &Arc<Form>, froze: bool| {
        #[cfg(feature = "sym-profile-testing")]
        if plain {
            profile::record_plain_id(id.bits());
        }
        if let Some(d) = &drive {
            d.publish_form(id, form, froze);
        }
    };
    let mut stack = vec![(root, false)];
    while let Some((id, expanded)) = stack.pop() {
        if memo.contains_key(&id) {
            continue;
        }
        // **The registered-identity door** ([`Sym::register_equal`]),
        // and the whole of where it acts: a node a constructor
        // registered against another takes THAT node's form.
        // Consulted in the EARLY walk only, so the plain form — the one
        // a decision is asked of first — is M10-8's exactly and pays
        // nothing for the registry.
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
        let Some(node) = sess.node_of(id) else {
            // In no table this session reads: minted outside every
            // session on it. An unknown function of the parameters is
            // an indeterminate, and it is this leaf's alone
            // (`Session::unknown`).
            if plain {
                sess.unknown.insert(id, ());
            }
            #[cfg(feature = "sym-profile-testing")]
            profile::record_unrecorded(profile::Walk::of(early, registry));
            let f = frozen(sess, id);
            memo.insert(id, f);
            continue;
        };
        let arity = node.op.arity();
        // **The drive's table**, asked before the children are
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
        let fc = if arity >= 3 {
            memo.get(&node.kids[2]).cloned()
        } else {
            None
        };
        let budget = sess.budget;
        let made = {
            let kids = [
                fa.as_deref().unwrap_or(&empty),
                fb.as_deref().unwrap_or(&empty),
                fc.as_deref().unwrap_or(&empty),
            ];
            #[cfg(feature = "sym-profile-testing")]
            profile::clear_note();
            #[cfg(feature = "sym-profile-testing")]
            let t0 = profile::clock();
            form::take_refusal();
            let combined = combine(&node, kids, sess, early);
            let refusal = form::take_refusal();
            // The per-node A/B reduction (`SymRules::early_ab`),
            // bounded in steps and in the size of the form it is asked
            // over, falling back to the un-reduced form when it does
            // not fit.
            // From here the form is SHARED (`Arc`), so that a form the
            // reduction's memo and rule E hand back unchanged is one
            // allocation in the walk memo and the reduction memo alike.
            // Rule H's product rewrite, before the A/B reduction, which
            // then finds no `sin` square it has not already met.
            let combined = if early && sess.rules.angle_sum {
                combined.map(|f| {
                    turn::linearise(&f, sess)
                        .filter(|g| within(budget, g))
                        .unwrap_or(f)
                })
            } else {
                combined
            };
            let combined = combined.map(Arc::new);
            let combined = if early && sess.rules.early_ab {
                combined.map(|f| {
                    if f.num.terms().len() + f.den.terms().len() > EARLY_AB_TERMS {
                        return f;
                    }
                    #[cfg(feature = "sym-profile-testing")]
                    let t0 = profile::clock();
                    let reduced = reduce_per_node(sess, &f);
                    #[cfg(feature = "sym-profile-testing")]
                    if t0.is_some() {
                        let outcome = match &reduced {
                            Some(g) if !within(budget, g) => "refused",
                            None => "refused",
                            Some(g) if **g == *f => "unchanged",
                            Some(_) => "reduced",
                        };
                        profile::reduce_outcome(
                            t0,
                            f.digest(),
                            outcome,
                            even_powers(&f, &sess.atoms),
                        );
                    }
                    reduced.filter(|g| within(budget, g)).unwrap_or(f)
                })
            } else {
                combined
            };
            // **Rule E**, after the per-node A/B reduction and before
            // the budget check. AFTER is the order that lets the rule
            // act on what the reduction leaves, and the walk ledger
            // (`editor-core/tests/m10_sym_profile_interval`) pins it:
            // planted BEFORE the reduction, the plate's
            // `Early/Decision` digest reds (the same 308 calls build the
            // same 24,376 forms, none frozen, with other contents) and
            // the slab's lines hold. It is also the order
            // `trig::sqrt_atom` applies to a form it builds by hand,
            // which is what makes the two spellings of one arc key one
            // atom.
            // Before the budget check, because the rule can only
            // SHRINK a form in terms and degree, so one it cancels may
            // fit where the raw one would have frozen.
            let combined = if early && sess.rules.common_factor {
                combined.map(quotient::cancel_shared)
            } else {
                combined
            };
            let over = match &combined {
                Some(f) => !within(budget, f),
                None => refusal == Some(form::Refusal::Budget),
            };
            if over && notes_causes {
                sess.budget_frozen.insert(id, ());
            }
            let made = combined.filter(|f| within(budget, f));
            #[cfg(feature = "sym-profile-testing")]
            profile::record_node(
                node.op,
                profile::Walk::of(early, registry),
                id.bits(),
                kids,
                made.as_deref(),
                t0,
            );
            made
        };
        drop((fa, fb, fc));
        let froze = made.is_none();
        let f = match made {
            Some(p) => p,
            None => frozen(sess, id),
        };
        // A form built over an unknown is this leaf's reading of it,
        // not the node's: a leaf that holds the node builds another.
        // It is not published (`Session::unknown`).
        if plain && node.kids().iter().any(|k| sess.unknown.contains_key(k)) {
            sess.unknown.insert(id, ());
        } else {
            note(id, &f, froze);
        }
        memo.insert(id, f);
    }
    memo.get(&root)
        .cloned()
        .unwrap_or_else(|| Arc::new(Form::poly(Poly::indet(root.bits()))))
}

/// Records the atom `id` in the session, and in the drive's table
/// whenever the PLAIN walk REFERENCES it: the atoms a plain form's
/// indeterminates stand for, which a leaf that takes that form from the
/// table needs and never records itself.
///
/// **On every plain-walk reference, not only on a fresh record**: the
/// walks share one `atoms` map, and an atom's id is a hash of the op,
/// the payload and the ARGUMENT FORM's digest, so wherever a rule left
/// a kid's early form equal to its plain one an earlier EARLY walk has
/// already recorded the atom a later plain walk references.
fn mint_atom(sess: &mut Session, id: u128, early: bool, info: impl FnOnce() -> AtomInfo) {
    let info = sess.atoms.entry(id).or_insert_with(info);
    if !early && let Some(memo) = &sess.memo {
        memo.publish_atom(id, info);
    }
}

/// **The per-node reduction, memoized per session on its input form** —
/// [`algebra::reduce_steps`] at [`EARLY_STEPS`], answered from
/// [`Session::reductions`] when this session has already reduced the
/// same form under the same rules and ring bound.
///
/// **The memo answers what the reduction would build, bit for bit, by
/// construction.** The reduction reads its input form, the rules, the
/// budget, the step cap, the ring bound (`rational::coeff_bound`) and
/// the session's atom table, which it only LOOKS UP, at the ids a form
/// carries to a power past one ([`algebra::squared`]). The budget and
/// the step cap are fixed for the session; the rules and the ring bound
/// are in the key, because a retry attempt runs its walks under its
/// own. A record that is in the table never changes: `mint_atom` and
/// the drive memo's seeding only insert an id that is absent, under a
/// key that is a hash of the op, the payload and the argument's digest.
/// A lookup that FOUND NOTHING can change — an atom minted since — so
/// the entry keeps every id the reduction looked up and missed
/// ([`algebra::reduce_noting`]), and answers only while all of them are
/// still absent; a stale entry is dropped and the reduction run again.
/// Nothing about which indeterminates those are (a parameter, `π`, an
/// opaque, a frozen node's own id, rule B's `cos` twin) enters the
/// argument.
///
/// **Keyed on the form, not on its digest**: the digest picks the
/// bucket, and a hit needs the whole form equal (`Form`'s equality: the
/// canonical terms and the poison flag), so a collision costs
/// a miss and never an answer.
///
/// **A form none of whose squared ids is in the atom table is its own
/// reduction** — `find_square` finds nothing in it, so the reduction
/// answers the form unchanged — and is answered so without a lookup
/// and without an entry, the same `Arc` back.
fn reduce_per_node(sess: &mut Session, f: &Arc<Form>) -> Option<Arc<Form>> {
    #[cfg(feature = "sym-profile-testing")]
    profile::reduce_begin();
    if !algebra::squared(f).any(|id| sess.atoms.contains_key(&id)) {
        return Some(Arc::clone(f));
    }
    let (rules, bits) = (sess.rules, rational::coeff_bound());
    let key = f.digest();
    if let Some(bucket) = sess.reductions.get_mut(&key)
        && let Some(at) = bucket
            .iter()
            .position(|r| r.rules == rules && r.bits == bits && *r.input == **f)
    {
        let atoms = &sess.atoms;
        if bucket[at].absent.iter().all(|id| !atoms.contains_key(id)) {
            #[cfg(feature = "sym-profile-testing")]
            profile::reduce_hit();
            #[cfg(test)]
            {
                sess.reduction_hits += 1;
            }
            return bucket[at].out.clone();
        }
        bucket.swap_remove(at);
        sess.reductions_held -= 1;
    }
    let (mut absent, mut twins) = (Vec::new(), Vec::new());
    let out = algebra::reduce_noting(
        f,
        rules,
        sess.budget,
        &sess.atoms,
        EARLY_STEPS,
        &mut absent,
        &mut twins,
    )
    .map(|g| if g == **f { Arc::clone(f) } else { Arc::new(g) });
    // Rule B's `cos` twins are recorded here, where the form that
    // carries them is kept: every atom id in a memoized form is in the
    // atom table, so no later read of the table depends on whether
    // something else happened to build `cos θ`.
    for (id, info) in twins {
        absent.retain(|a| *a != id);
        mint_atom(sess, id, true, || info);
    }
    if sess.reductions_held < REDUCTION_FORMS {
        absent.sort_unstable();
        absent.dedup();
        sess.reductions.entry(key).or_default().push(Reduction {
            input: Arc::clone(f),
            rules,
            bits,
            absent: absent.into_boxed_slice(),
            out: out.clone(),
        });
        sess.reductions_held += 1;
    }
    out
}

/// Which atom kinds `f` carries to an even power — what rules A/B and
/// rule G's companion `|X|² = X²` could substitute — as the cost
/// profile's label for a per-node reduction.
#[cfg(feature = "sym-profile-testing")]
fn even_powers(f: &Form, atoms: &IndetMap<AtomInfo>) -> &'static str {
    let (mut sqrt, mut abs, mut sin) = (false, false, false);
    for id in algebra::squared(f) {
        match atoms.get(&id).map(|a| a.op) {
            Some(SymOp::Sqrt) => sqrt = true,
            Some(SymOp::Abs) => abs = true,
            Some(SymOp::Sin) => sin = true,
            _ => {}
        }
    }
    match (sqrt, abs, sin) {
        (false, false, false) => "no even power",
        (true, false, false) => "sqrt",
        (false, true, false) => "abs",
        (false, false, true) => "sin",
        (true, true, false) => "sqrt+abs",
        (true, false, true) => "sqrt+sin",
        (false, true, true) => "abs+sin",
        (true, true, true) => "sqrt+abs+sin",
    }
}

/// The plain quotient form of `root` — every atom opaque, no rule
/// applied, no value read. Memoized in the session's persistent table.
fn plain_form(sess: &mut Session, root: SymId) -> Arc<Form> {
    let first = (sess.rules, rational::COEFF_BITS);
    walk(sess, root, WalkKind::Plain, 0, first)
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
/// [`EARLY_STEPS`], the rest of the shipped algebra, and the decision
/// read at each `Select`/`min`/`max`.
fn early_form(sess: &mut Session, root: SymId) -> Arc<Form> {
    let first = (sess.rules, rational::COEFF_BITS);
    walk(sess, root, WalkKind::Early, 0, first)
}

/// The three normal-form walks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WalkKind {
    /// Every atom opaque, rule A0 only.
    Plain,
    /// The session's rules per node.
    Early,
    /// The early walk with the registry applied.
    Door,
}

/// **Which memo a walk reads and fills**: the first attempt's three,
/// or one retry attempt's early or door table (`NodeState::retries`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MemoSlot {
    Plain,
    Early,
    Door,
    RetryEarly(usize),
    RetryDoor(usize),
}

impl MemoSlot {
    /// The memo `kind` fills at `attempt`. The plain walk has only the
    /// first attempt's: the plain rung is never retried ([`SymRetry`]).
    fn of(kind: WalkKind, attempt: u8) -> Self {
        match (kind, usize::from(attempt).checked_sub(1)) {
            (WalkKind::Plain, _) => Self::Plain,
            (WalkKind::Early, None) => Self::Early,
            (WalkKind::Door, None) => Self::Door,
            (WalkKind::Early, Some(k)) => Self::RetryEarly(k),
            (WalkKind::Door, Some(k)) => Self::RetryDoor(k),
        }
    }
}

impl Session {
    /// The memo `slot` names, growing the retry tables to reach it.
    fn memo_slot(&mut self, slot: MemoSlot) -> &mut IdMap<Arc<Form>> {
        let retry = |r: &mut Vec<RetryMemo>, k: usize| {
            if r.len() <= k {
                r.resize_with(k + 1, RetryMemo::default);
            }
        };
        match slot {
            MemoSlot::Plain => &mut self.forms,
            MemoSlot::Early => &mut self.forms_early,
            MemoSlot::Door => &mut self.node.forms_door,
            MemoSlot::RetryEarly(k) => {
                retry(&mut self.node.retries, k);
                &mut self.node.retries[k].early
            }
            MemoSlot::RetryDoor(k) => {
                retry(&mut self.node.retries, k);
                &mut self.node.retries[k].door
            }
        }
    }
}

/// **The walk's scope**: the memo taken out of its slot, the session's
/// rules swapped for the attempt's, the profile's attempt set — and all
/// three put back when the scope ends, by any path including a panic,
/// the way [`rational::with_coeff_bound`] restores the ring bound.
///
/// A plain assignment after the walk would leave a session that
/// unwound mid-walk holding the attempt's rules and an empty memo. The
/// session is thread-local and outlives the unwind (no door here tears
/// it down on a panic), so the next decision on that thread would run
/// the wrong tier over an empty table.
struct WalkScope<'s> {
    sess: &'s mut Session,
    slot: MemoSlot,
    memo: IdMap<Arc<Form>>,
    kept: SymRules,
    #[cfg(feature = "sym-profile-testing")]
    outer: u8,
}

impl Drop for WalkScope<'_> {
    fn drop(&mut self) {
        self.sess.rules = self.kept;
        let memo = core::mem::take(&mut self.memo);
        *self.sess.memo_slot(self.slot) = memo;
        #[cfg(feature = "sym-profile-testing")]
        profile::set_attempt(self.outer);
    }
}

/// **The one walk door**: `kind`'s form of `root` on `attempt` (0 the
/// first, `k` the `k`th retry), under `rules` at the ring bound `bits`,
/// in that attempt's memo.
///
/// The rules are swapped onto the session for the walk and swapped back
/// — `form_in` and everything below it reads `sess.rules`, and a
/// parameter carried past all of them would be a signature change
/// through six modules to reach the same read.
fn walk(
    sess: &mut Session,
    root: SymId,
    kind: WalkKind,
    attempt: u8,
    (rules, bits): (SymRules, u64),
) -> Arc<Form> {
    let slot = MemoSlot::of(kind, attempt);
    let memo = core::mem::take(sess.memo_slot(slot));
    // **The first attempt swaps nothing and scopes nothing**: it runs
    // under the session's own rules at the ring's own bound, which is
    // what every caller hands it (`plain_form`, `early_form`,
    // `rungs` at attempt 0). Only a retry swaps the rules
    // in and widens the ring; the scope below still restores the memo
    // on every attempt.
    let first = attempt == 0;
    debug_assert!(
        !first || (rules == sess.rules && bits == rational::COEFF_BITS),
        "the first attempt is the session's rules at COEFF_BITS"
    );
    let kept = if first {
        sess.rules
    } else {
        core::mem::replace(&mut sess.rules, rules)
    };
    #[cfg(feature = "sym-profile-testing")]
    let (t0, outer) = (profile::clock(), profile::set_attempt(attempt));
    let mut scope = WalkScope {
        sess,
        slot,
        memo,
        kept,
        #[cfg(feature = "sym-profile-testing")]
        outer,
    };
    let (early, registry) = match kind {
        WalkKind::Plain => (false, false),
        WalkKind::Early => (true, false),
        WalkKind::Door => (true, true),
    };
    let WalkScope { sess, memo, .. } = &mut scope;
    let out = if first {
        form_in(sess, memo, root, early, registry, true)
    } else {
        rational::with_coeff_bound(bits, || form_in(sess, memo, root, early, registry, false))
    };
    let out = if early && rules.angle_sum {
        turn::settle(&out, sess).map_or(out, Arc::new)
    } else {
        out
    };
    #[cfg(feature = "sym-profile-testing")]
    profile::walk_done(
        match kind {
            WalkKind::Plain => profile::Walk::Plain,
            WalkKind::Early => profile::Walk::Early,
            WalkKind::Door => profile::Walk::Door,
        },
        t0,
    );
    out
}

/// **Which RUNG of the ladder answered a decision** — the plain form,
/// the early walk, rules A/B over the top residual, or the
/// registered-identity door.
///
/// Every rung answers the same claim; the order is cost and reach
/// ([`rungs`]). Recorded per decision by the cost profile
/// (`profile::DecisionRecord`) so that "where the refusals are" is a
/// count and not a reading of the code, and named by the test
/// instrument's report (`sym::contradiction`, test-only). **Nameable outside this
/// crate only under `sym-profile-testing` or `sym-revalue-testing`**,
/// the test-only features those two live behind: no shipped build can
/// read which rung answered, and no decision type carries it.
#[cfg(any(feature = "sym-profile-testing", feature = "sym-revalue-testing"))]
pub use rung::Rung;
#[cfg(not(any(feature = "sym-profile-testing", feature = "sym-revalue-testing")))]
use rung::Rung;

mod rung {
    /// See the re-export's docs.
    #[cfg_attr(
        not(any(feature = "sym-profile-testing", feature = "sym-revalue-testing")),
        expect(
            unreachable_pub,
            reason = "re-exported `pub` only under the profile's feature"
        )
    )]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Rung {
        /// The PLAIN form — every atom opaque, rule A0 only. Never
        /// retried.
        Plain,
        /// The EARLY walk — the session's rules per node.
        Early,
        /// Rules A/B over the TOP residual, once the walks have
        /// declined.
        Top,
        /// The registered-identity DOOR.
        Door,
    }
}

/// **The identity test whose `Zero` leaves the tier**, and the check on
/// it: the rung and the attempt that proved `id` zero, every rung asked
/// and the RETRY LADDER offered ([`SymRetry`]). Every proven `Zero` the
/// tier hands out passes here: a decision's ([`Decide::sign_within`])
/// and a deferring session's reader's (`deferred::discharge`), so the
/// check below is on each of them.
///
/// **At an exact witness the zero lies in `enclosure`**, the certified
/// enclosure of the margin's number (`Decide::enclosure_probe`, read
/// where the decision was taken). The number is the interval evaluation
/// of the DAG the form was taken from, op by op, outward-rounded. A form
/// that is zero makes that DAG's value 0 at every admissible point of the
/// box, and every leaf (`Param`, `Opaque`, `Hull`, `Lit`) stands for a
/// point of its own interval, so by inclusion isotonicity the enclosure
/// holds 0. The decision read ([`signed`]) takes an arm only on a
/// certified one-sided enclosure over the parameter brackets, which is
/// the arm the value channel's own `min` / `max` / `select` contains. An
/// enclosure that excludes 0 is then an algebra, enclosure or registrant
/// defect (DESIGN [failure.taxonomy] row 5). Every exact witness reads an
/// enclosure (`Interval`, and `Dual<Interval>` through its value), so a
/// missing one is a defect too. At an inexact witness the number is one
/// rounded reading and there is nothing to check.
///
/// **The one exception is a margin whose number is a measured bound**:
/// [`Decide::free_identity`]'s. It is recognised on the failing path only,
/// by a [`SymOp::Free`] in the node's DAG, which is the door's mark and
/// not an exact one: a margin built FROM a free identity's result (the
/// identity plus another term) is exempt too, and an identity whose
/// closure never calls `free` is not. Neither is reached by the door's one
/// caller (`geom_brep::edge_nurbs::limb2_enclosure_as`, which decides the
/// door's margin as returned and always mints the variable); both are
/// pinned (`sym_contradiction_instrument_rows`).
fn discharge_checked(id: SymId, enclosure: Option<(f64, f64)>) -> Option<(Rung, u8)> {
    let (rung, attempt) = discharge_in(id, Path::Decision)?;
    let exact = SESSION.with(|s| {
        s.borrow()
            .as_ref()
            .is_some_and(|s| s.witness == Some(crate::Witness::Exact))
    });
    if exact {
        debug_assert!(
            enclosure.is_some(),
            "`{}`: the symbolic tier proved this margin zero at the {rung:?} rung (attempt \
             {attempt}) at an exact witness that reads no enclosure",
            crate::k_stats::current_predicate(),
        );
        if let Some((lo, hi)) = enclosure {
            debug_assert!(
                (lo <= 0.0 && 0.0 <= hi) || reaches(id, |op| op == SymOp::Free),
                "`{}`: the symbolic tier proved this margin zero at the {rung:?} rung \
                 (attempt {attempt}) and its certified enclosure [{lo:e}, {hi:e}] excludes 0",
                crate::k_stats::current_predicate(),
            );
        }
    }
    Some((rung, attempt))
}

/// **Which question the identity test answers**, and so which rungs it
/// asks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Path {
    /// Whether the tier proves the margin zero, to DECIDE it: every
    /// rung, the registered-identity door last, and the retry ladder
    /// into the first attempt's silence.
    Decision,
    /// Whether the tier's own algebra proves the margin zero, for the
    /// test instrument that checks it against a definite numeric sign
    /// (`sym::contradiction`, test-only): every attempt, and no door, because a
    /// registered zero under a definite sign is the instrument's own
    /// event.
    #[cfg(feature = "sym-revalue-testing")]
    Algebra,
}

/// [`Session::reaches_op`] in the installed session; `false` outside one.
fn reaches(id: SymId, hit: impl FnMut(SymOp) -> bool) -> bool {
    SESSION.with(|s| {
        s.borrow()
            .as_ref()
            .is_some_and(|sess| sess.reaches_op(id, hit))
    })
}

/// **Whether the early form of `id` keeps an opaque value**
/// ([`SymOp::Opaque`]): a value computed apart from the parameter
/// symbols, which the tier keeps as an unknown of its own. Read off the
/// form's indeterminates, through every atom's arguments; where the
/// walk froze a node on its budget, or poisoned the form, the form says
/// nothing past that node, and the node's DAG is read for an opaque
/// value instead. A frozen node over the parameter symbols alone is no
/// opaque value: it is a form too big for this budget. `None` outside a
/// session.
fn keeps_an_opaque_atom(id: SymId) -> Option<bool> {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        let opaque: std::collections::HashSet<u128> = sess
            .nodes
            .values()
            .filter(|n| n.op == SymOp::Opaque)
            .map(SymNode::opaque_indet)
            .collect();
        if opaque.is_empty() {
            return Some(false);
        }
        let reaches_opaque =
            |sess: &Session, from: SymId| sess.reaches_op(from, |op| op == SymOp::Opaque);
        let early = early_form(sess, id);
        if early.poisoned {
            return Some(reaches_opaque(sess, id));
        }
        Some(any_indet(sess, &early, |x| {
            opaque.contains(&x)
                || (sess.node_of(SymId(x)).is_some() && reaches_opaque(sess, SymId(x)))
        }))
    })
}

/// **Whether the early form of `id` froze a node on its budget**: an
/// indeterminate of the form, read through every atom's arguments, is a
/// node the walk froze because its form outgrew the session's term or
/// degree bound (`form_in`'s freeze, [`Session::budget_frozen`]). A node
/// the ring's guards froze (a coefficient past its bound, an exponent
/// overflow) is not: a larger budget would not read it. A poisoned form
/// is no freeze: it has no value to be too big for. `None` outside a
/// session.
fn froze_on_its_budget(id: SymId) -> Option<bool> {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        let early = early_form(sess, id);
        if early.poisoned {
            return Some(false);
        }
        Some(any_indet(sess, &early, |x| {
            sess.budget_frozen.contains_key(&SymId(x))
        }))
    })
}

/// **Whether any indeterminate `form` reaches answers `hit`** — the
/// one walk over a form's indeterminates, descending through every
/// atom's argument forms ([`Session::atoms`]) and asking `hit` of each
/// distinct non-atom indeterminate it meets, once.
fn any_indet(sess: &Session, form: &Form, mut hit: impl FnMut(u128) -> bool) -> bool {
    let mut stack = vec![form];
    let mut seen = std::collections::HashSet::new();
    while let Some(form) = stack.pop() {
        for poly in [&form.num, &form.den] {
            for (mono, _) in poly.terms() {
                for &(x, _) in mono {
                    if !seen.insert(x) {
                        continue;
                    }
                    if let Some(atom) = sess.atoms.get(&x) {
                        stack.extend(atom.args.iter().flatten().map(|a| &**a));
                    } else if hit(x) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// The identity test along `path`, inside the installed session: the
/// rung and the attempt that proved `id` zero.
fn discharge_in(id: SymId, path: Path) -> Option<(Rung, u8)> {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        if sess.budget.max_terms == 0 {
            return None;
        }
        #[cfg(feature = "sym-profile-testing")]
        let mark = profile::decision_begin();
        let out = ladder(sess, id, path);
        #[cfg(feature = "sym-profile-testing")]
        profile::decision_end(mark, out);
        out
    })
}

/// **The ladder**: the plain rung once, then the rungs of the first
/// attempt, then — only into their silence — each retry's rungs in
/// [`SymRetry::attempts`]'s order. Both paths climb it; the door rung is
/// the decision path's alone ([`rungs`]).
fn ladder(sess: &mut Session, id: SymId, path: Path) -> Option<(Rung, u8)> {
    // **THE PLAIN RUNG, and the first attempt's alone.** No attempt
    // above the first builds a plain form ([`SymRetry`]).
    let plain = plain_form(sess, id);
    if plain.is_zero() {
        return Some((Rung::Plain, 0));
    }
    let first = (sess.rules, rational::COEFF_BITS);
    if let Some(rung) = rungs(sess, id, &plain, 0, first, path) {
        return Some((rung, 0));
    }
    for (attempt, rules, bits) in sess.retry.attempts(sess.rules) {
        // **The GROWTH GUARD** (`SymRetry::max_forms`), checked BEFORE
        // the attempt is walked: an attempt whose memos are at the cap
        // is not offered again to this node, and the decision stays
        // numeric.
        let k = usize::from(attempt) - 1;
        if sess
            .node
            .retries
            .get(k)
            .is_some_and(|m| m.len() >= sess.retry.max_forms)
        {
            #[cfg(feature = "sym-revalue-testing")]
            {
                sess.withheld += 1;
            }
            continue;
        }
        if let Some(rung) = rungs(sess, id, &plain, attempt, (rules, bits), path) {
            sess.node.counts.retried += 1;
            return Some((rung, attempt));
        }
    }
    None
}

/// The EARLY rung, the TOP-RESIDUAL rung and, on the decision path, the
/// DOOR rung of one attempt — and, for an attempt above the first, in
/// that attempt's own memos, under its rules and its ring bound. Every
/// rung answers the same claim, so the order is cost and reach: the
/// early walk is the one every decision already pays for, the
/// top-residual reduction runs once over a form already built, and the
/// door is a third walk asked only where a registration exists.
fn rungs(
    sess: &mut Session,
    id: SymId,
    plain: &Form,
    attempt: u8,
    (rules, bits): (SymRules, u64),
    path: Path,
) -> Option<Rung> {
    if rules.early && walk(sess, id, WalkKind::Early, attempt, (rules, bits)).is_zero() {
        return Some(Rung::Early);
    }
    // Rules A and B over the residual, once. The residual is the PLAIN
    // form on every attempt — the plain rung is never retried — but the
    // reduction runs under the attempt's rules and at its ring bound, so
    // a retry can close here what the first attempt's ring refused.
    if rules.sqrt_square || rules.pythagoras {
        #[cfg(feature = "sym-profile-testing")]
        let t0 = profile::clock();
        let reduced = rational::with_coeff_bound(bits, || {
            algebra::reduce(plain, rules, sess.budget, &sess.atoms)
        });
        #[cfg(feature = "sym-profile-testing")]
        profile::reduce_top_done(t0);
        if reduced.as_ref().is_some_and(|f| f.is_zero()) {
            return Some(Rung::Top);
        }
    }
    // THE DOOR, asked last and only where there is a registration to
    // ask about ([`Sym::register_equal`]).
    if path == Path::Decision
        && rules.registered
        && rules.early
        && !sess.node.registry.is_empty()
        && walk(sess, id, WalkKind::Door, attempt, (rules, bits)).is_zero()
    {
        return Some(Rung::Door);
    }
    None
}

/// Records a registration the door refused, for the session's receipt.
fn count_registration_refused() {
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            sess.node.counts.registrations_refused += 1;
        }
    });
}

/// Notes the witness kind of the scalar a decision is taken at
/// ([`Session::witness`]). A session decides at one scalar, so a second
/// kind is a session reused across lanes, and the early forms it built
/// under the first kind would answer for the second: asserted in every
/// profile, because with the assertion off a mixed session would keep
/// the first kind and could run the decision read at a point. The
/// check is one comparison of two one-byte kinds per decision, beside
/// the session borrow the decision takes anyway.
fn note_witness(witness: crate::real::Witness) {
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            let seen = *sess.witness.get_or_insert(witness);
            assert_eq!(
                seen, witness,
                "one symbolic session decided at two witness kinds"
            );
        }
    });
}

/// Records how one decision was answered, for the session's receipt:
/// proven by the tier, or handed to the numeric channel.
fn count_decision(proven: bool) {
    SESSION.with(|s| {
        if let Some(sess) = s.borrow_mut().as_mut() {
            if proven {
                sess.node.counts.symbolic += 1;
            } else {
                sess.node.counts.numeric += 1;
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
    /// Private, so it is written only by this module's constructors,
    /// each from the op it mints (and [`Decide::free_identity`], from
    /// its caller's reading): no code outside `sym.rs` can pair a node
    /// with a number of its choosing. That is all privacy gives. It does
    /// not make the number its expression's evaluation: [`Sym::param`]
    /// binds one symbol at whatever value each call is handed (two calls
    /// at two values are one symbol at two numbers), and an opaque minted
    /// before a session can share a node with one the session mints
    /// (`work/sym/an-opaque-minted-before-a-session-shares-a-name-with-the-sessions-first.md`).
    /// The enclosure check on a proven zero (`discharge_checked`) stands
    /// on the number being the expression's wherever neither happens.
    value: T,
    node: SymId,
}

impl<T: Copy> Sym<T> {
    /// The numeric channel — the value a plain `T` run would carry.
    #[must_use]
    pub fn value(self) -> T {
        self.value
    }
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
    /// What separates them is the name each is minted under: the mint
    /// scope open on this thread and the value's ordinal there.
    /// [`mint`] argues why no two values a form can meet share one, and
    /// why the name is the same under every schedule and in every
    /// replay (D9).
    #[must_use]
    pub fn opaque(value: T) -> Self {
        let (scope, ordinal) = mint::take();
        #[cfg(feature = "sym-profile-testing")]
        profile::record_opaque(indet_opaque(scope, ordinal));
        Self {
            value,
            node: intern(SymNode::opaque(scope, ordinal)),
        }
    }

    /// A value bound as the document PARAMETER `symbol` — the one door
    /// that introduces an indeterminate. No bracket is recorded, so the
    /// decision read ([`signed`]) can certify no decision over this
    /// parameter; a caller holding the box's bounds uses
    /// [`Self::param_over`].
    #[must_use]
    pub fn param(symbol: ParamSymbol, value: T) -> Self {
        Self {
            value,
            node: intern(SymNode {
                op: SymOp::Param,
                payload: symbol.0,
                kids: [SymId::UNRECORDED; 3],
            }),
        }
    }

    /// [`Self::param`] over the bracket `[lo, hi]` the value was built
    /// from — the analysis box's own two `f64`s, which the caller that
    /// mints a parameter axis already holds. The bracket is recorded in
    /// the installed session for the decision read ([`signed`]); it is
    /// the ONLY value the symbolic tier ever reads, and it is read as
    /// two floats through a certification enclosure, never as the lane
    /// scalar.
    /// Outside a session the bracket is dropped and this is `param`.
    #[must_use]
    pub fn param_over(symbol: ParamSymbol, value: T, lo: f64, hi: f64) -> Self {
        SESSION.with(|s| {
            if let Some(sess) = s.borrow_mut().as_mut() {
                sess.params.insert(indet_param(symbol.0), (lo, hi));
                if !sess.outer.is_empty() {
                    // One scope holds at most one environment (a part's
                    // run builds its own once), and an environment binds
                    // a variable once: a second bracket for one symbol
                    // in one scope is a caller binding it twice.
                    debug_assert!(
                        sess.node
                            .brackets
                            .iter()
                            .all(|&(id, held)| id != indet_param(symbol.0) || held == (lo, hi)),
                        "one scope recorded two brackets for one parameter"
                    );
                    sess.node.brackets.push((indet_param(symbol.0), (lo, hi)));
                }
            }
        });
        Self::param(symbol, value)
    }

    /// Mints the literal `x`, carrying `value` (`x` read at `T`).
    fn literal(value: T, x: f64) -> Self {
        Self {
            value,
            node: intern(SymNode::literal(x)),
        }
    }

    /// Mints the node for a nullary op.
    fn nullary(value: T, op: SymOp, payload: u64) -> Self {
        Self {
            value,
            node: intern(SymNode {
                op,
                payload,
                kids: [SymId::UNRECORDED; 3],
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
                kids: [self.node, SymId::UNRECORDED, SymId::UNRECORDED],
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
                kids: [self.node, other.node, SymId::UNRECORDED],
            }),
        }
    }

    /// Mints the node for a three-child op ([`SymOp::Select`]).
    fn ternary(self, b: Self, c: Self, value: T, op: SymOp) -> Self {
        Sym {
            value,
            node: intern(SymNode {
                op,
                payload: 0,
                kids: [self.node, b.node, c.node],
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
    /// A registered identity is an AXIOM: it rests on the registrant's
    /// argument about what it built, checked at the leaf's witness, and
    /// this door is where that argument enters the tier. A zero through
    /// it is E12's claim like any other — zero wherever the construction
    /// takes the decisions it took — and is counted with the rest
    /// ([`SymCounts::symbolic`]); what the registry buys a document is
    /// the differential with [`SymRules::registered`] off. A registrant
    /// that cannot state its argument in its doc comment has no
    /// business calling this.
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
    /// # Whose the record is
    ///
    /// The registry is the open node's ([`MintScope::node`]): a
    /// registration is read by the decisions of the node that made it
    /// and of the nodes it feeds, to which it travels in the node's
    /// record ([`NodeRecord`]), and by no other node's. So what a node
    /// proves through the door is a function of its inputs, whatever
    /// ran before it in the replay and on whichever worker (D9).
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
                sess.node.counts.registrations_refused += 1;
                return SymRegistration::Cyclic;
            }
            sess.state(a, b);
            sess.node.identities.push((a, b));
            #[cfg(feature = "sym-revalue-testing")]
            sess.stated.push((a, b));
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

/// `F` over a symbolic scalar: what [`Real::view`] at `Sym<T>` hands to
/// `T`'s, so a `Sym`'s view borrows wherever its numeric channel's does.
struct SymOf<F>(core::marker::PhantomData<F>);

impl<F: crate::value_view::OverScalar> crate::value_view::OverScalar for SymOf<F> {
    type At<S: Real> = F::At<Sym<S>>;

    fn map<S: Real, U: Real>(x: &F::At<Sym<S>>, f: impl Fn(S) -> U) -> F::At<Sym<U>> {
        F::map(x, |s: Sym<S>| Sym {
            value: f(s.value),
            node: s.node,
        })
    }
}

impl<T: Real> Real for Sym<T> {
    /// **Never the same: no certificate is read by its support at the
    /// symbolic tier.** A derivation here mints fresh unknowns (a hull
    /// bound enters as [`Sym::opaque`]), so deriving again on the same
    /// support does not give the same nodes, and the decision record a
    /// derivation writes is this tier's purpose: a read would drop it.
    /// Every certificate at `Sym` is derived where it is asked.
    fn same_value(self, other: Self) -> bool {
        let _ = (self, other);
        false
    }

    /// **The LANE SCALAR's.** A `Sym<T>` carries `T`'s value channel
    /// unchanged — the tier reads no value and widens no enclosure —
    /// so a comparison at `Sym<T>` proves exactly what one at `T`
    /// proves. This is the const [`Decide::sign_within`] below reads to
    /// check a symbolic zero against its enclosure.
    const WITNESS: crate::real::Witness = T::WITNESS;

    const NAME: &'static str = "symbolic";

    /// The lane scalar's: the tier changes how a margin decides, not
    /// whether the scalar certifies.
    type Carries<C: Copy + core::fmt::Debug + Send + Sync + 'static> = T::Carries<C>;

    /// The node, whatever the lane scalar: the tier reads no value.
    type Expression = SymId;

    fn expression(self) -> Option<SymId> {
        Some(self.node)
    }

    /// The symbolic scalar over its numeric channel's value: the
    /// identity over the unknowns is what a decision at it reads, so it
    /// is kept, node for node. Over `f64` or `Interval` that is the
    /// scalar itself.
    type Value = Sym<T::Value>;

    fn without_tangent(self) -> Sym<T::Value> {
        Sym {
            value: self.value.without_tangent(),
            node: self.node,
        }
    }

    /// The numeric channel's view, node for node: borrowed where it is.
    fn view<F: crate::value_view::OverScalar>(
        x: &F::At<Self>,
    ) -> crate::value_view::ValueView<'_, F::At<Sym<T::Value>>> {
        T::view::<SymOf<F>>(x)
    }

    fn from_f64(x: f64) -> Self {
        Self::literal(T::from_f64(x), x)
    }

    fn from_computed(x: f64) -> Self {
        Self::opaque(T::from_computed(x))
    }

    /// The lane scalar's.
    fn told_reading(&self) -> crate::told::Reading {
        self.value.told_reading()
    }

    fn zero() -> Self {
        // The same node a `from_f64(0.0)` mints, so the two spellings
        // of the additive identity share one id.
        Self::literal(T::zero(), 0.0)
    }

    fn one() -> Self {
        Self::literal(T::one(), 1.0)
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
                kids: [two.node, pi.node, SymId::UNRECORDED],
            }),
        }
    }

    fn sqrt(self) -> Self {
        self.unary(self.value.sqrt(), SymOp::Sqrt, 0)
    }

    /// The plain formula's form, carrying `T`'s value
    /// ([`Real::difference_dot`]): the same real, read to its digits.
    fn difference_dot(q: [Self; 3], o: [Self; 3], n: [Self; 3]) -> Self {
        let plain = crate::real::plain_difference_dot(q, o, n);
        Sym {
            value: T::difference_dot(q.map(|x| x.value), o.map(|x| x.value), n.map(|x| x.value)),
            node: plain.node,
        }
    }

    /// The plain formula's form, carrying `T`'s value
    /// ([`Real::harmonic_belly`]): the same real, read to its digits.
    fn harmonic_belly(curve: crate::StoredHarmonic<Self>, o: [Self; 3], n: [Self; 3]) -> Self {
        let plain = crate::real::plain_harmonic_belly(curve, o, n);
        let value = |p: [Self; 3]| p.map(|x| x.value);
        Self {
            value: T::harmonic_belly(curve.map(|x| x.value), value(o), value(n)),
            node: plain.node,
        }
    }

    /// The plain formulas' forms, carrying `T`'s values
    /// ([`Real::quartic_sign_algebra`]): the same reals, read to their
    /// digits.
    fn quartic_sign_algebra(p: Self, q_hat: Self, s: Self) -> [Self; 2] {
        let plain = crate::real::plain_quartic_sign_algebra(p, q_hat, s);
        let value = T::quartic_sign_algebra(p.value, q_hat.value, s.value);
        let [disc, depth] = plain;
        [
            Sym {
                value: value[0],
                node: disc.node,
            },
            Sym {
                value: value[1],
                node: depth.node,
            },
        ]
    }

    fn abs(self) -> Self {
        self.unary(self.value.abs(), SymOp::Abs, 0)
    }

    fn is_poison(self) -> bool {
        self.value.is_poison()
    }

    fn is_bounded(self) -> bool {
        self.value.is_bounded()
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

    fn less_sin(self) -> Self {
        self.unary(self.value.less_sin(), SymOp::LessSin, 0)
    }

    /// The `Sin` node over `τ/12`, valued `1/2`: the tier folds a
    /// constant angle itself (`trig_at_rational_pi`, rule H's settle).
    fn sin_of_a_twelfth() -> Self {
        (Self::tau() / Self::from_f64(12.0)).unary(T::sin_of_a_twelfth(), SymOp::Sin, 0)
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

    fn select_le_zero(self, when_le: Self, when_gt: Self) -> Self {
        self.ternary(
            when_le,
            when_gt,
            self.value.select_le_zero(when_le.value, when_gt.value),
            SymOp::Select,
        )
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
impl<T: crate::real::CertifiedBounds> CertifiedEnclosure for Sym<T> {
    fn certified_bracket(self) -> Option<(f64, f64)> {
        self.value.certified_bracket()
    }

    fn carried<C: Copy + core::fmt::Debug + Send + Sync + 'static>(
        record: C,
    ) -> <Self as Real>::Carries<C> {
        T::carried(record)
    }

    fn held<C: Copy + core::fmt::Debug + Send + Sync + 'static>(
        held: &<Self as Real>::Carries<C>,
    ) -> &C {
        T::held(held)
    }
}

/// Span selection is STRUCTURE selection and reads the value channel;
/// the hull mints a `Hull` node keyed by the two operands' ids (never by
/// their forms — see [`SymOp::Hull`]).
impl<T: SpanLocate> SpanLocate for Sym<T> {
    fn locate_spans<'a>(self, knots: &'a KnotVector) -> Option<SpanSet<'a>> {
        self.value.locate_spans(knots)
    }

    fn enclosure_hull(self, other: Self) -> Self {
        self.binary(other, self.value.enclosure_hull(other.value), SymOp::Hull)
    }

    fn point_value(self) -> Option<f64> {
        self.value.point_value()
    }

    fn structure_positive(self) -> bool {
        self.value.structure_positive()
    }

    fn structure_bracket(self) -> Option<(f64, f64)> {
        self.value.structure_bracket()
    }

    /// A constant only when this value's node is the literal of its
    /// value ([`SymNode::literal`], recomputed by its content hash, so
    /// nothing is interned and no session is read): any other node, a
    /// constant expression included, may carry a symbol, and `None` is
    /// the conservative answer. Not read through the node's form
    /// (`constant_value`): expanding a form runs the session's work and
    /// moves the `SymCounts` a digest records, from inside a meter.
    fn plain_constant(self) -> Option<f64> {
        let v = self.value.plain_constant()?;
        (self.node == SymNode::literal(v).id()).then_some(v)
    }

    fn split_projective<A: crate::spline::ProjectiveSplit<Self>>(arm: A) -> A::Out {
        T::split_projective(crate::spline::projective::SymArm(arm))
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
///    *The value side.* [`MarginKind::Invalid`](crate::MarginKind::Invalid) is the arm every scalar
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
/// **And a definite non-zero numeric answer is the answer.** A
/// certified enclosure that excludes zero is a proof that the margin is
/// not zero, and at a point scalar the rounded reading is what the
/// construction compares; either way nothing the tier could say would
/// change the sign returned, so the tier is asked nothing there: no
/// form, no registry walk, no count. Whether the algebra or a
/// registration WOULD have claimed `Zero` on a margin the numbers
/// decided is a question about the kernel, asked by the test
/// instrument (`sym::contradiction`, test-only) and by no shipped build (ERROR-DESIGN
/// E12, "the two channels are checked where an answer is returned").
///
/// **Where the tier answers `Zero` at an EXACT witness, the certified
/// enclosure contains 0** — asserted in debug builds, at every rung, a
/// retry's and the registry door's included ([`Witness`](crate::Witness);
/// the assertion's site argues it).
///
/// Everything else is `T::sign_within` verbatim.
impl<T: Decide> Decide for Sym<T> {
    fn sign_within(self, band: Band) -> Result<Decided, Indeterminate> {
        #[cfg(feature = "sym-revalue-testing")]
        let noted = revalue::note(self.node, self.value);
        let decided = self.decide_sign(band);
        #[cfg(feature = "sym-revalue-testing")]
        revalue::settle(noted, &decided);
        decided
    }

    /// The value channel `self`'s and the node the identity's
    /// ([`Decide::free_identity`]): the identity is built over a free
    /// variable ([`SymOp::Free`]) minted at the value the caller reads
    /// the carrier at, and its own value channel is discarded — the
    /// margin's numeric reading is the caller's, never the identity's
    /// evaluation over the box.
    fn free_identity<F>(self, identity: F) -> Self
    where
        F: FnOnce(&dyn Fn(Self) -> Self) -> Self,
    {
        let free = |at: Self| Self::nullary(at.value, SymOp::Free, 0);
        Sym {
            value: self.value,
            node: identity(&free).node,
        }
    }
}

impl<T: Decide> Sym<T> {
    /// [`Decide::sign_within`]'s body, apart so the re-valuation hook
    /// can read the outcome it returns.
    fn decide_sign(self, band: Band) -> Result<Decided, Indeterminate> {
        // Where this decision's own K sample will land, read before the
        // base scalar records it (`k_stats::sink_mark`).
        #[cfg(feature = "probe")]
        let mark = crate::k_stats::sink_mark();
        note_witness(T::WITNESS);
        let numeric = self.value.sign_within(band);
        if let Some(deferred) = deferred::defer(self.node, numeric, self.value.enclosure_probe()) {
            count_decision(false);
            return deferred;
        }
        if matches!(
            &numeric,
            Ok(Decided {
                sign: Sign::Positive | Sign::Negative,
                ..
            })
        ) {
            count_decision(false);
            report::record(&numeric, false, None, self.value.enclosure_probe());
            return numeric;
        }
        let domain_violation = matches!(&numeric, Err(e) if e.margin.is_invalid());
        let m16_pre = (!domain_violation && m16::on()).then(|| m16::prebuild(self.node));
        let m16_t0 = std::time::Instant::now();
        let answered = if domain_violation {
            None
        } else {
            discharge_checked(self.node, self.value.enclosure_probe())
        };
        if let (Some(pre), Some(enc)) = (m16_pre, self.value.enclosure_probe()) {
            let today = match (&numeric, answered.is_some()) {
                (_, true) => "ladder-zero",
                (Ok(_), false) => "value-zero-in-band",
                (Err(_), false) => "undecided",
            };
            if let Some(sign) = m16::probe(
                self.node,
                enc,
                band,
                pre,
                today,
                m16_t0.elapsed().as_nanos(),
            ) {
                let margin = match numeric {
                    Ok(Decided { margin, .. }) | Err(Indeterminate { margin, .. }) => margin,
                };
                return Ok(Decided {
                    sign,
                    margin: margin.as_proven(),
                });
            }
        }
        count_decision(answered.is_some());
        if let Some(level) = std::env::var("BOXFINISH_M15").ok() {
            let name = crate::k_stats::current_predicate();
            let wanted = std::env::var("BOXFINISH_M15_NAMES")
                .map_or(true, |n| n.split(',').any(|x| x == name));
            if wanted && (answered.is_none() || level == "2") {
                let text = report::render_node(self.node);
                let cut = |t: &str| t.chars().take(1500).collect::<String>();
                println!(
                    "M15 decide {name} numeric={} enc={:?} asked={} answered={answered:?} sizes={:?} plain=<{}> early=<{}> explain=<{}>",
                    match &numeric {
                        Ok(d) => format!("{:?}", d.sign),
                        Err(_) => "Undecided".to_owned(),
                    },
                    self.value.enclosure_probe(),
                    !domain_violation,
                    text.as_ref().map(|t| t.sizes),
                    text.as_ref().map_or(String::new(), |t| cut(&t.plain)),
                    text.as_ref()
                        .and_then(|t| t.early.as_ref())
                        .map_or(String::new(), |t| cut(t)),
                    text.as_ref()
                        .and_then(|t| t.explain.as_ref())
                        .map_or(String::new(), |t| t.chars().take(6000).collect::<String>()),
                );
            }
        }
        if answered.is_some() {
            #[cfg(feature = "probe")]
            crate::k_stats::retag_at(mark, crate::k_stats::SampleOutcome::Symbolic);
            report::record(&numeric, true, None, self.value.enclosure_probe());
            // The form decided; what the numeric channel saw is still
            // the reading a refusal reports.
            let margin = match numeric {
                Ok(Decided { margin, .. }) | Err(Indeterminate { margin, .. }) => margin,
            };
            return Ok(Decided {
                sign: Sign::Zero,
                margin: margin.as_proven(),
            });
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
            report::record(&numeric, false, text, self.value.enclosure_probe());
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
        Sym::param(ParamSymbol::new(test_utils::symbol_id(name)), v)
    }

    fn decides_zero(m: Sym<f64>) -> bool {
        crate::k_stats::decide("sym_test", Margin::of(m), band()) == Ok(Sign::Zero)
    }

    /// **Sine, cosine and tangent of a rational multiple of π fold where
    /// the value is rational** (`trig_at_rational_pi`), so a copy spun
    /// by `turn/4` is parallel as a theorem: each row's residual against
    /// its rational value is the zero form. An irrational value (the
    /// sine of an eighth of a turn) stays an atom and decides nothing
    /// symbolically.
    #[test]
    fn trig_of_a_rational_multiple_of_pi_folds_where_it_is_rational() {
        fn pi() -> Sym<f64> {
            <Sym<f64> as Real>::pi()
        }
        fn lit(x: f64) -> Sym<f64> {
            Sym::from_f64(x)
        }
        type Row = (&'static str, fn() -> Sym<f64>, f64);
        let rows: [Row; 7] = [
            ("sin(π/2)", || (pi() / lit(2.0)).sin(), 1.0),
            ("cos(π/2)", || (pi() / lit(2.0)).cos(), 0.0),
            ("cos(π)", || pi().cos(), -1.0),
            ("sin(−3π/2)", || (-(pi() * lit(1.5))).sin(), 1.0),
            ("sin(π/6)", || (pi() / lit(6.0)).sin(), 0.5),
            ("cos(2π/3)", || (pi() * lit(2.0) / lit(3.0)).cos(), -0.5),
            ("tan(π/4)", || (pi() / lit(4.0)).tan(), 1.0),
        ];
        for (what, f, value) in rows {
            let (zero, counts) = with_session(budget(), || decides_zero(f() - lit(value)));
            assert!(zero && counts.symbolic == 1, "{what} = {value}: {counts:?}");
        }
        let (_, counts) = with_session(budget(), || {
            decides_zero((pi() / lit(4.0)).sin() - lit(core::f64::consts::FRAC_1_SQRT_2))
        });
        assert_eq!(
            counts.symbolic, 0,
            "sin(π/4) is irrational and folds to no constant"
        );
    }

    /// **A pair of spellings of one constant whose product the shipped
    /// ring REFUSES and a 512-bit ring does not.** Each factor is
    /// `1/3`'s `f64`, a 53-bit odd mantissa; six of them multiply to
    /// 318 bits, so `COEFF_BITS` refuses the fifth product on the left
    /// and the one product on the right, and both nodes freeze into
    /// indeterminates of their own — two different ones, because their
    /// content hashes differ.
    fn two_spellings_past_the_ring() -> Sym<f64> {
        let c = || Sym::<f64>::from_f64(1.0 / 3.0);
        let six = c() * c() * c() * c() * c() * c();
        let cubed = c() * c() * c();
        six - cubed * cubed
    }

    /// The measured ladder.
    fn kept_atom() -> SymRetry {
        SymRetry::kept_atom()
    }

    #[test]
    fn a_wider_ring_retry_closes_what_the_first_attempts_ring_refused() {
        // The two spellings have the same `f64` value, so the NUMERIC
        // channel answers `Zero` inside the band either way and the
        // answer alone says nothing: what the row reads is the RECEIPT,
        // which is where a theorem and a measurement are told apart.
        let (_, first) = with_session(budget(), || decides_zero(two_spellings_past_the_ring()));
        assert_eq!(
            first.numeric, 1,
            "at COEFF_BITS both spellings freeze into indeterminates of their own, the \
             difference is not the zero form, and the decision falls to the numeric channel"
        );
        assert_eq!(first.symbolic, 0);
        assert_eq!(first.retried, 0, "no ladder is installed");

        let retry = SymRetry {
            bits: Some(512),
            ..SymRetry::none()
        };
        let (_, with_ladder) = with_session_retry(budget(), SymRules::shipped(), retry, || {
            decides_zero(two_spellings_past_the_ring())
        });
        assert_eq!(
            with_ladder.symbolic, 1,
            "at 512 bits both products fit, both spellings are one constant and the \
             difference is the zero polynomial — the same identity of reals either way, so \
             a retry's theorem is a theorem and lands in the column the first attempt's would"
        );
        assert_eq!(
            with_ladder.retried, 1,
            "and `retried` says the ladder is what carried it"
        );
        assert_eq!(with_ladder.numeric, 0);
    }

    /// The forms each retry attempt's memos hold RIGHT NOW, in the
    /// session this thread has installed — the evidence that a ladder
    /// was walked, which the receipt does not carry (`retried` counts
    /// decisions a retry CLOSED, not attempts made).
    fn retry_memo_sizes() -> Vec<usize> {
        SESSION.with(|s| {
            s.borrow()
                .as_ref()
                .map(|sess| sess.node.retries.iter().map(RetryMemo::len).collect())
                .unwrap_or_default()
        })
    }

    /// `(1/3)^n` spelled two ways — a left-associated chain against
    /// `(1/3)^a · (1/3)^(n−a)` — so the ring refuses the two at
    /// different nodes and they freeze into different indeterminates.
    /// Each factor is a 53-bit odd mantissa, so the product needs about
    /// `53·n` bits.
    fn power_two_ways(n: usize, a: usize) -> Sym<f64> {
        let c = || Sym::<f64>::from_f64(1.0 / 3.0);
        let chain = |k: usize| (1..k).fold(c(), |acc, _| acc * c());
        chain(n) - chain(a) * chain(n - a)
    }

    fn ring(bits: u64) -> SymRetry {
        SymRetry {
            bits: Some(bits),
            ..SymRetry::none()
        }
    }

    fn columns(c: &SymCounts) -> [u64; 3] {
        [c.symbolic, c.numeric, c.retried]
    }

    /// **THE NEGATIVE ROW**: a decision the first attempt refuses AND
    /// every retry refuses stays numeric.
    ///
    /// What the RECEIPT shows is the decision in `numeric` and nothing
    /// in `retried`, which is the same receipt a session with no ladder
    /// writes — the receipt records what each decision CLAIMS, and a
    /// ladder that closed nothing claims nothing. That the attempts WERE
    /// made is a fact about the session's work and not a claim, so the
    /// row reads it where it lives: the retry memos hold the forms the
    /// two attempts built.
    #[test]
    fn a_decision_every_attempt_refuses_stays_numeric() {
        // 318 bits is past 256 and past 300; the ring retry is offered
        // and declines, and the kept-atom retries cannot reach a
        // constant product at all.
        let retry = SymRetry {
            bits: Some(300),
            ..kept_atom()
        };
        let ((_, walked), counts) =
            with_session_retry(budget(), SymRules::shipped(), retry, || {
                (
                    decides_zero(two_spellings_past_the_ring()),
                    retry_memo_sizes(),
                )
            });
        assert_eq!(
            columns(&counts),
            [0, 1, 0],
            "neither attempt reaches it — 318 bits is past both bounds — so it stays \
             numeric and nothing is counted retried"
        );
        assert_eq!(
            walked.len(),
            3,
            "all three attempts were offered — the two masks and the ring"
        );
        assert!(
            walked.iter().all(|n| *n > 0),
            "and each walked, building forms in its own memo: {walked:?}"
        );
    }

    /// **An attempt that cannot differ from the first is not walked.** A
    /// session whose rules are already narrower than every mask — M10-9's
    /// `without_the_algebra`, which has rule A and rule G shut — offered
    /// the measured ladder: both masks reduce to the session's own rules,
    /// so both attempts are the first attempt again and neither is made.
    #[test]
    fn an_attempt_identical_to_the_first_is_not_walked() {
        let first = SymRules::without_the_algebra();
        assert_eq!(first.masked_by(kept_atom().without[0].unwrap()), first);
        assert_eq!(first.masked_by(kept_atom().without[1].unwrap()), first);
        assert_eq!(kept_atom().attempts(first).count(), 0);
        let ((_, walked), counts) = with_session_retry(budget(), first, kept_atom(), || {
            (
                decides_zero(two_spellings_past_the_ring()),
                retry_memo_sizes(),
            )
        });
        assert_eq!(counts.numeric, 1);
        assert!(
            walked.is_empty(),
            "no retry memo was ever made, because no attempt was: {walked:?}"
        );
        // And a ring retry at exactly the first attempt's bound is the
        // first attempt again too; so is a second mask identical to the
        // first. (A NARROWER ring is a different attempt and is kept —
        // `attempts` drops only equal ones.)
        let same = SymRetry {
            bits: Some(rational::COEFF_BITS),
            without: [kept_atom().without[0], kept_atom().without[0]],
            ..SymRetry::none()
        };
        let offered: Vec<u8> = same
            .attempts(SymRules::shipped())
            .map(|(k, _, _)| k)
            .collect();
        assert_eq!(offered, [1], "one distinct attempt, numbered 1");
        let narrower = SymRetry {
            bits: Some(rational::COEFF_BITS - 1),
            ..SymRetry::none()
        };
        assert_eq!(
            narrower.attempts(SymRules::shipped()).count(),
            1,
            "a narrower ring is not the first attempt, so it is offered"
        );
    }

    /// **Every atom id a kept form carries is in the atom table.** The
    /// early walk writes `sin²t` as `1 − cos²t` (rule B, with rule H
    /// shut so the square reaches it), and nothing here builds
    /// `cos t`: the twin is recorded where the form that carries it is
    /// kept, so no later read of the table depends on whether another
    /// node, or another leaf of the drive, happened to build it.
    ///
    /// Mutant: `reduce_per_node` dropping the twins it is handed
    /// leaves the form's `cos` id out of the table.
    #[test]
    fn rule_bs_cos_twin_is_recorded_with_the_form_that_carries_it() {
        let rules = SymRules {
            angle_sum: false,
            ..SymRules::shipped()
        };
        let ((), _) = with_session_rules(budget(), rules, || {
            let s = p("t", 0.3).sin_cos().0;
            let _ = decides_zero(s * s - p("k", 0.3_f64.sin().powi(2)));
            let params = [
                indet_param(test_utils::symbol_id("t")),
                indet_param(test_utils::symbol_id("k")),
            ];
            SESSION.with(|slot| {
                let slot = slot.borrow();
                let sess = slot.as_ref().expect("inside the session");
                let mut atoms_met = 0;
                for form in sess.forms_early.values() {
                    for poly in [&form.num, &form.den] {
                        for mono in poly.monos() {
                            for &(id, _) in mono {
                                if params.contains(&id) {
                                    continue;
                                }
                                atoms_met += 1;
                                assert!(
                                    sess.atoms.contains_key(&id),
                                    "an early form carries an id the atom table does not hold"
                                );
                            }
                        }
                    }
                }
                assert!(
                    sess.atoms.values().any(|a| a.op == SymOp::Cos),
                    "rule B wrote the cos twin: the row reached its subject"
                );
                assert!(atoms_met > 0, "the early forms carry atoms");
            });
        });
    }

    /// **THE GROWTH GUARD** (`SymRetry::max_forms`): an attempt whose
    /// memos are at the cap is withheld, checked before the walk. Two
    /// decisions that only a 512-bit retry closes: with the cap at one
    /// form, the first decision's retry fills the memo past it and the
    /// second decision's is never offered.
    #[test]
    fn the_growth_guard_withholds_an_attempt_at_its_cap() {
        let bag = || {
            decides_zero(power_two_ways(5, 2));
            decides_zero(power_two_ways(6, 2));
            retry_memo_sizes()
        };
        let (_, open) = with_session_retry(budget(), SymRules::shipped(), ring(512), bag);
        assert_eq!(
            columns(&open),
            [2, 0, 2],
            "uncapped, both close on the retry"
        );
        let capped = SymRetry {
            max_forms: 1,
            ..ring(512)
        };
        let (sizes, counts) = with_session_retry(budget(), SymRules::shipped(), capped, bag);
        assert_eq!(
            columns(&counts),
            [1, 1, 1],
            "the first closes on its retry; the second's attempt is withheld and it stays \
             numeric"
        );
        assert!(
            sizes[0] >= 1,
            "the memo passed the cap on the one walk the guard allowed: {sizes:?}"
        );
    }

    /// **A retry's forms never reach the first attempt's memos**, at the
    /// scalar. Decision 1 is closed by a 512-bit retry, which builds its
    /// `c⁵` node as a 265-bit constant in the RETRY memo. Decisions 2′
    /// and 2 share that node and close ONLY while it is FROZEN at 256
    /// bits — `c⁵·c⁵` is 530 bits and refuses at every bound below 1024,
    /// so an opened `c⁵` freezes at two different outer nodes and the
    /// identity is lost. 2′ closes at the plain rung, 2 at the early rung
    /// (an `abs` fold keeps the plain rung out). A retry's form leaking
    /// into either memo turns one of them numeric.
    #[test]
    fn a_retrys_forms_never_reach_the_first_attempts_memos() {
        let c = || Sym::<f64>::from_f64(1.0 / 3.0);
        let c5 = || c() * c() * c() * c() * c();
        let two = || Sym::<f64>::from_f64(2.0);
        let z = || {
            let w = p("z", 0.3);
            w * w + Sym::from_f64(1.0)
        };
        let bag = || {
            decides_zero(power_two_ways(5, 2));
            decides_zero((c5() * c5()) * two() - c5() * (c5() * two()));
            decides_zero(z().abs() * ((c5() * c5()) * two()) - z() * (c5() * (c5() * two())));
        };
        let (_, none) = with_session(budget(), bag);
        let (_, ladder) = with_session_retry(budget(), SymRules::shipped(), ring(512), bag);
        assert_eq!(
            columns(&none),
            [2, 1, 0],
            "2 and 2' close on the frozen node; 1 refuses"
        );
        assert_eq!(
            columns(&ladder),
            [3, 0, 1],
            "the retry closes 1, and 2/2' are still closed by the first attempt on the frozen \
             node"
        );
    }

    /// `X = Π(1 − wᵢ)` over `n` parameters: `2ⁿ` terms, mixed signs so
    /// rule F never folds `|X|`.
    fn mixed(n: usize) -> Sym<f64> {
        let mut x = Sym::<f64>::from_f64(1.0);
        for i in 0..n {
            x = x * (Sym::from_f64(1.0) - p(&format!("w{i}"), 0.1));
        }
        x
    }

    /// Manifestly positive, so rule F folds `|Z| = Z` in the EARLY walk
    /// only: the plain rung keeps the `abs` atom and the top residual
    /// cannot open it, so only a walk decides a margin carrying it.
    fn z() -> Sym<f64> {
        let w = p("z", 0.3);
        w * w + Sym::from_f64(1.0)
    }

    /// A document `abs` node whose square rule G's companion rewrite
    /// opens into `X²` (64 → 729 terms), so the next product `X²·X²`
    /// freezes on the term budget, where `|X|⁴` as a kept atom is one
    /// monomial — the SHAPE of the link's loss, built at the scalar.
    fn closes_only_with_rule_g_off() -> Sym<f64> {
        let x = mixed(6);
        let a = || x.abs();
        let q = (a() * a()) * (a() * a());
        let q2 = ((a() * a()) * a()) * a();
        z().abs() * q - z() * q2
    }

    /// Rule A's own harm: `sqrt(X)² = X` with `X` of 256 terms opens
    /// into `X·X` (6561 terms), past the term budget; `s⁴` kept as an
    /// atom power is one monomial.
    fn closes_only_with_rule_a_off() -> Sym<f64> {
        let x = mixed(8);
        let s = || x.sqrt();
        let q = (s() * s()) * (s() * s());
        let q2 = ((s() * s()) * s()) * s();
        z().abs() * q - z() * q2
    }

    fn mask(f: fn(&mut SymRules)) -> SymRetry {
        let mut m = SymRules::all();
        f(&mut m);
        SymRetry {
            without: [Some(m), None],
            ..SymRetry::none()
        }
    }

    /// **Each of the measured ladder's two masks closes a decision the
    /// other cannot, at the scalar** — the reason there are two. The
    /// rule-G shape: rule G shut, the rewrite shut or rule A shut each
    /// close it (all three keep `|X|²` from opening), the ring cannot
    /// (the freeze is on terms), and the measured ladder closes it on its
    /// first attempt.
    #[test]
    fn a_decision_that_closes_only_with_rule_g_off() {
        let run = |r: SymRetry| {
            with_session_retry(budget(), SymRules::shipped(), r, || {
                decides_zero(closes_only_with_rule_g_off())
            })
            .1
        };
        assert_eq!(columns(&run(SymRetry::none())), [0, 1, 0]);
        let closed = [1, 0, 1];
        assert_eq!(columns(&run(mask(|m| m.canonical_root = false))), closed);
        assert_eq!(columns(&run(mask(|m| m.abs_square = false))), closed);
        assert_eq!(columns(&run(mask(|m| m.sqrt_square = false))), closed);
        assert_eq!(columns(&run(ring(512))), [0, 1, 0]);
        assert_eq!(columns(&run(kept_atom())), closed);
    }

    /// And the rule-A shape: only rule A shut closes it; rule G shut
    /// does not, nor the ring; the measured ladder closes it on the
    /// attempt that shuts rule A.
    #[test]
    fn a_decision_that_closes_only_with_rule_a_off() {
        let run = |r: SymRetry| {
            with_session_retry(budget(), SymRules::shipped(), r, || {
                decides_zero(closes_only_with_rule_a_off())
            })
            .1
        };
        assert_eq!(columns(&run(SymRetry::none())), [0, 1, 0]);
        assert_eq!(columns(&run(mask(|m| m.sqrt_square = false))), [1, 0, 1]);
        assert_eq!(columns(&run(mask(|m| m.canonical_root = false))), [0, 1, 0]);
        assert_eq!(columns(&run(ring(512))), [0, 1, 0]);
        assert_eq!(columns(&run(kept_atom())), [1, 0, 1]);
    }

    /// **A refused ladder leaves every column where it was**: a mixed
    /// bag — a decision past every bound the ladder offers, a
    /// non-identity, and a plain theorem — decided with no ladder and
    /// with one. Every decision column and `frozen` are identical, and
    /// nothing is counted retried.
    #[test]
    fn a_refused_ladder_leaves_every_column_where_it_was() {
        let bag = || {
            let a = decides_zero(power_two_ways(10, 5));
            let b = decides_zero(closes_only_with_rule_g_off() + Sym::from_f64(1.0e-3));
            let x = p("w", 0.37);
            let c = decides_zero(x + Sym::from_f64(2.0) * x - Sym::from_f64(3.0) * x);
            (a, b, c)
        };
        let (o1, none) = with_session(budget(), bag);
        let (o2, ladder) = with_session_retry(budget(), SymRules::shipped(), ring(300), bag);
        assert_eq!(o1, o2);
        assert_eq!(columns(&none), columns(&ladder));
        assert_eq!(
            none.frozen, ladder.frozen,
            "a retry's freezes are not counted into `frozen`"
        );
        assert_eq!(ladder.retried, 0);
    }

    /// The ring bound is restored after a panic inside a retry attempt.
    #[test]
    fn the_coeff_bound_is_restored_after_a_panic_inside_it() {
        assert_eq!(rational::coeff_bound(), rational::COEFF_BITS);
        let r = std::panic::catch_unwind(|| {
            rational::with_coeff_bound(1024, || {
                assert_eq!(rational::coeff_bound(), 1024);
                panic!("planted");
            })
        });
        assert!(r.is_err());
        assert_eq!(rational::coeff_bound(), rational::COEFF_BITS);
    }

    /// **A retry never moves what the first attempt proved.** The
    /// same margin, decided with the ladder and without it, is the same
    /// discharge in the same column — the ladder is asked only into the
    /// first attempt's silence, so a plain theorem is never re-asked.
    #[test]
    fn the_ladder_leaves_the_first_attempts_answers_exactly_as_they_were() {
        let margin = || {
            let x = p("w", 0.37);
            let a = x + Sym::from_f64(2.0) * x;
            let b = Sym::from_f64(3.0) * x;
            decides_zero(a - b)
        };
        let (plain, without) = with_session(budget(), margin);
        let (laddered, with_ladder) =
            with_session_retry(budget(), SymRules::shipped(), kept_atom(), margin);
        assert!(plain && laddered);
        assert_eq!(
            without.symbolic, with_ladder.symbolic,
            "a plain theorem is the plain rung's on both runs"
        );
        assert_eq!(
            with_ladder.retried, 0,
            "the ladder was never entered, so it carried nothing"
        );
    }

    /// **A mask is a field-by-field AND**, so a retry can only ever run
    /// FEWER rules than the session — never a rule the session shut.
    #[test]
    fn a_retry_mask_can_only_take_rules_away() {
        let shut = SymRules::none();
        for mask in [SymRules::all(), SymRules::shipped(), SymRules::none()] {
            assert_eq!(
                shut.masked_by(mask),
                shut,
                "no mask turns a rule ON that the session has off"
            );
        }
        let all = SymRules::all();
        assert_eq!(all.masked_by(SymRules::all()), all);
        assert_eq!(all.masked_by(SymRules::none()), SymRules::none());
        assert_eq!(
            SymRules::shipped().masked_by(SymRules::all()),
            SymRules::shipped(),
            "an all-true mask is the identity"
        );
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
        assert_eq!(counts.symbolic, 1);
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
            assert_eq!(counts.symbolic, 0, "at {v}");
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
    /// quotient, and with rule H shut no trigonometric identity beyond
    /// rule B. `sin(2θ) − 2·sinθ·cosθ` is not the zero form there —
    /// `sin(2θ)` is an atom of another argument and nothing relates it
    /// to the pair (rule H's product rewrite does:
    /// `angle_addition_is_a_theorem_under_rule_h`) — and with the rules
    /// OFF the Pythagorean pair is not either, which is what makes
    /// `SymRules::none` the pre-algebra tier.
    #[test]
    fn the_opaque_atoms_are_opaque() {
        let (_, counts) = with_session_rules(budget(), SymRules::without_angle_sum(), || {
            let x = p("w", 3.0);
            let (s, c) = x.sin_cos();
            let (s2, _) = (x + x).sin_cos();
            decides_zero(s2 - Sym::from_f64(2.0) * s * c)
        });
        // Numerically zero at this point — the numeric channel answers
        // it, as it always did. What is pinned is that the TIER claims
        // nothing: the double-angle identity is outside every rule.
        assert_eq!(counts.symbolic, 0, "no double-angle identity");
        assert_eq!(counts.numeric, 1);
        let (_, counts) = with_session_rules(budget(), SymRules::none(), || {
            let x = p("w", 3.0);
            let (s, c) = x.sin_cos();
            decides_zero(s * s + c * c - Sym::from_f64(1.0))
        });
        assert_eq!(counts.symbolic, 0, "rule B off: both atoms opaque");
        assert_eq!(counts.numeric, 1);
    }

    /// **The decision door's node**: the value channel is `T`'s door
    /// verbatim; the DAG carries the tier's only THREE-child op, hash-
    /// consed like every other node and reaching the form as an atom
    /// keyed by its three arguments' normal forms. Two selects over
    /// equal forms are one unknown, over different ones two, and which
    /// arm the value read is never claimed as a theorem.
    #[test]
    fn select_is_a_three_child_atom_over_its_arguments_forms() {
        let (out, _) = with_session(budget(), || {
            let x = p("w", 3.0);
            let y = p("h", 0.25);
            let d = x - y;
            // The value channel is the plain `f64` door: 3 − 0.25 > 0.
            let picked = d.select_le_zero(x, y);
            let same = d.select_le_zero(x, y);
            let swapped = d.select_le_zero(y, x);
            (
                picked.value,
                picked.node == same.node,
                picked.node == swapped.node,
            )
        });
        assert_eq!(out.0, 0.25, "the value channel is f64's door");
        assert!(out.1, "the same three children are one node");
        assert!(!out.2, "swapping the arms is a different node");
        // The atom is keyed by FORMS, so a decision written differently
        // but equal as a form gives the same unknown — and a select
        // against either arm is not a theorem, however it falls.
        let (out, counts) = with_session(budget(), || {
            let x = p("w", 3.0);
            let y = p("h", 0.25);
            let s = (x - y).select_le_zero(x, y);
            let alias = ((x + x) - (y + x)).select_le_zero(x, y);
            (decides_zero(s - alias), decides_zero(s - y))
        });
        assert!(out.0, "equal argument forms are one unknown");
        assert!(out.1, "and it is numerically y here");
        assert_eq!(counts.symbolic, 1);
        assert_eq!(counts.numeric, 1, "no arm is claimed symbolically");
    }

    /// **Rule A0 reads the decision door when its form is a CONSTANT.**
    /// A select whose decision is a literal — every axis-aligned normal
    /// puts one there, because the frame constructor's comparison is
    /// exact arithmetic on the normal's own components — is decided,
    /// and the arm it reads is a theorem rather than an unknown. The
    /// tie goes to the `when_le` arm, the same way the value door's
    /// does.
    ///
    /// Without this the frame of every axis-aligned face is an opaque
    /// atom and every identity over a face built on that frame freezes.
    #[test]
    fn a_constant_decision_folds_to_the_arm_it_reads() {
        let (out, counts) = with_session(budget(), || {
            let x = p("w", 3.0);
            let y = p("h", 0.25);
            // −1 ≤ 0 reads `when_le`; +1 reads `when_gt`; the point tie
            // reads `when_le`.
            let neg = Sym::from_f64(-1.0).select_le_zero(x, y);
            let pos = Sym::from_f64(1.0).select_le_zero(x, y);
            let tie = Sym::from_f64(0.0).select_le_zero(x, y);
            (
                (neg.value, pos.value, tie.value),
                (
                    decides_zero(neg - x),
                    decides_zero(pos - y),
                    decides_zero(tie - x),
                ),
            )
        });
        assert_eq!(out.0, (3.0, 0.25, 3.0), "the value channel is f64's door");
        assert!(
            out.1.0 && out.1.1 && out.1.2,
            "each arm is a theorem: {:?}",
            out.1
        );
        assert_eq!(
            counts.numeric, 0,
            "a constant decision needs no numeric fallback"
        );
        assert_eq!(counts.symbolic, 3);
    }

    /// `sin`/`cos` of `num/den` of a turn, the way a spin reads an exact
    /// fraction ([`crate::Turns`]).
    fn turned(num: i64, den: u64) -> (Sym<f64>, Sym<f64>) {
        let t = crate::Angle::from_turns(crate::Turns::new(num, den).expect("a fraction"));
        t.sin_cos()
    }

    /// **Rule H, angle addition**: `cos a cos b − sin a sin b` is
    /// `cos(a + b)` and `sin a cos b + cos a sin b` is `sin(a + b)` as
    /// theorems, and so is the double angle `sin 2a = 2 sin a cos a`, at
    /// symbolic angles; with the rule shut all three are
    /// numeric.
    #[test]
    fn angle_addition_is_a_theorem_under_rule_h() {
        let rows = |rules| {
            with_session_rules(budget(), rules, || {
                let (a, b) = (p("a", 0.3), p("b", 1.1));
                let ((sa, ca), (sb, cb)) = (a.sin_cos(), b.sin_cos());
                let (s, c) = (a + b).sin_cos();
                let (s2, _) = (a + a).sin_cos();
                [
                    decides_zero(ca * cb - sa * sb - c),
                    decides_zero(sa * cb + ca * sb - s),
                    decides_zero(s2 - Sym::from_f64(2.0) * sa * ca),
                ]
            })
        };
        let (on, counts) = rows(SymRules::shipped());
        assert_eq!((on, counts.symbolic), ([true; 3], 3), "{counts:?}");
        let (_, off) = rows(SymRules::without_angle_sum());
        assert_eq!((off.symbolic, off.numeric), (0, 3), "{off:?}");
    }

    /// The members `k` of a ring of `n` whose step `k/n → (k + 1)/n`
    /// is NOT proved symbolically, for both coordinates, with the
    /// session's receipt.
    fn ring_unproved(n: u64, rules: SymRules) -> (Vec<i64>, SymCounts) {
        with_session_rules(budget(), rules, || {
            let r = p("r", 2.0);
            let (s1, c1) = turned(1, n);
            let symbolic = || session_counts().map_or(0, |c| c.symbolic);
            let mut unproved = Vec::new();
            for k in 0..i64::try_from(n).unwrap() {
                let (sk, ck) = turned(k, n);
                let (sn, cn) = turned(k + 1, n);
                let (x, y) = (r * ck, r * sk);
                let before = symbolic();
                decides_zero(c1 * x - s1 * y - r * cn);
                decides_zero(s1 * x + c1 * y - r * sn);
                if symbolic() - before != 2 {
                    unproved.push(k);
                }
            }
            unproved
        })
    }

    /// **Rule H, a ring's neighbours**: member `k`'s point at `k/N` of a
    /// turn, turned by `1/N`, is member `k + 1`'s, as a theorem for every
    /// `k`, the closing member's included, though `cos(turn/N)` is
    /// irrational for all but `N ∈ {1, 2, 3, 4, 6}`. Shut, a ring of 5 or
    /// of 12 leaves members numeric.
    #[test]
    fn a_rings_neighbours_are_theorems_under_rule_h() {
        for n in [5, 6, 7, 8, 10, 12, 16] {
            let (unproved, counts) = ring_unproved(n, SymRules::shipped());
            assert!(unproved.is_empty(), "N = {n}: {unproved:?} {counts:?}");
        }
        for n in [5, 12] {
            let (unproved, _) = ring_unproved(n, SymRules::without_angle_sum());
            assert!(!unproved.is_empty(), "N = {n} shut proves every member");
        }
    }

    /// **Rule H, a ring whose members' cosines are rational at some
    /// member**: a ring of 9 or 24 meets members at a third of a turn,
    /// whose cosine `−1/2` is a constant atom of the rewrite until the
    /// walk ends, so its products with the irrational steps beside it
    /// are angle sums like any other. Red if a member at a rational
    /// cosine is left numeric.
    #[test]
    fn a_ring_member_at_a_rational_cosine_beside_irrational_steps_is_a_theorem() {
        for n in [9, 24] {
            let (unproved, counts) = ring_unproved(n, SymRules::shipped());
            assert!(unproved.is_empty(), "N = {n}: {unproved:?} {counts:?}");
        }
    }

    /// **Rule H's canonical constant**: two spellings of one irrational
    /// value — `sin(π/3)` and `cos(π/6)`, `cos(7π/5)` and `−cos(2π/5)` —
    /// are one atom, and `cos(π/6)²` folds to `3/4`.
    #[test]
    fn spellings_of_one_constant_angle_are_one_atom_under_rule_h() {
        let (out, counts) = with_session_rules(budget(), SymRules::shipped(), || {
            let pi = <Sym<f64> as Real>::pi;
            let lit = Sym::from_f64;
            let c6 = (pi() / lit(6.0)).cos();
            [
                decides_zero((pi() / lit(3.0)).sin() - c6),
                decides_zero(
                    (pi() * lit(7.0) / lit(5.0)).cos() + (pi() * lit(2.0) / lit(5.0)).cos(),
                ),
                decides_zero(c6 * c6 - lit(0.75)),
                // `sin(π/6)` is an atom until the walk ends, and folds
                // at its output.
                decides_zero(turned(1, 12).0 - lit(0.5)),
                // A rational-radian angle is symbolic, so its product with a
                // constant atom is the split, left as it is.
                decides_zero(
                    (lit(1.0) + pi() / lit(5.0)).sin()
                        - lit(1.0).sin() * (pi() / lit(5.0)).cos()
                        - lit(1.0).cos() * (pi() / lit(5.0)).sin(),
                ),
            ]
        });
        assert_eq!((out, counts.symbolic), ([true; 5], 5), "{counts:?}");
    }

    /// **Rule H's deferred fold, at each place it is read**
    /// ([`turn::settle`]): `sin(π/6)` is an atom of the early walk, so
    /// each row below is red where its place leaves the atom standing.
    /// An atom's argument (`sqrt(s·s)`, `min(s, ½)`) and a decision
    /// (`select(s − ½)`) read `1/2` and prove; a reciprocal of the zero
    /// it settles to, `1/(s − ½)` or `(s − ½)⁻¹`, is refused, so its
    /// product with `s − ½` is not the theorem `1` a cancellation of
    /// the unsettled atom would make of it.
    #[test]
    fn rule_hs_deferred_fold_is_read_where_the_walk_reads_a_value() {
        let (rows, _) = with_session_rules(budget(), SymRules::shipped(), || {
            let (x, y) = (p("x", 0.7), p("y", 0.3));
            let (s, _) = turned(1, 12);
            let half = Sym::from_f64(0.5);
            let one = Sym::from_f64(1.0);
            let proved = |m: Sym<f64>| {
                let before = session_counts().map_or(0, |c| c.symbolic);
                decides_zero(m) && session_counts().map_or(0, |c| c.symbolic) > before
            };
            [
                ("sqrt(s·s) − s", proved((s * s).sqrt() - s)),
                ("min(s, ½) − ½", proved(s.min(half) - half)),
                (
                    "select(s − ½, x, y) − x",
                    proved((s - half).select_le_zero(x, y) - x),
                ),
                (
                    "(s − ½)/(s − ½) − 1",
                    proved((s - half) * (one / (s - half)) - one),
                ),
                (
                    "(s − ½)·(s − ½)⁻¹ − 1",
                    proved((s - half) * (s - half).powi(-1) - one),
                ),
            ]
        });
        assert_eq!(
            rows,
            [
                ("sqrt(s·s) − s", true),
                ("min(s, ½) − ½", true),
                ("select(s − ½, x, y) − x", true),
                ("(s − ½)/(s − ½) − 1", false),
                ("(s − ½)·(s − ½)⁻¹ − 1", false),
            ]
        );
    }

    /// **A denominator shown to vanish is poison**, the one answer of
    /// rule H's two rewrites ([`turn::settle`], [`turn::linearise`]):
    /// `1/(1 − 2·sin(π/6))` settled, and `1/((1 − 2·sin(π/6))·(1 +
    /// sin(π/6)))` linearised, whose denominator is the zero form over
    /// the angles (`cos(2π/3) = cos(4π/3)`). No document reaches either
    /// through the walk, which settles a reciprocal's argument first.
    #[test]
    fn a_denominator_rule_h_shows_to_vanish_is_poison() {
        let ((settled, linearised), _) = with_session_rules(budget(), SymRules::shipped(), || {
            SESSION.with(|slot| {
                let mut slot = slot.borrow_mut();
                let sess = slot.as_mut().expect("a session");
                let sixth = Form::poly(Poly::term(vec![(INDET_PI, 1)], Rat::new(1, 6, 0).unwrap()));
                let s = turn::mint(SymOp::Sin, &sixth, sess).expect("the atom");
                let lin = |c: i128, k: i128| {
                    Form::poly(Poly::constant(Rat::new(c, 1, 0).unwrap()))
                        .add(
                            &s.mul(
                                &Form::poly(Poly::constant(Rat::new(k, 1, 0).unwrap())),
                                sess.budget,
                            )
                            .unwrap(),
                            sess.budget,
                        )
                        .unwrap()
                };
                let (a, b) = (lin(1, -2), lin(1, 1));
                let den = a.mul(&b, sess.budget).unwrap();
                let settled = turn::settle(&Form::quotient(Poly::one(), a.num.clone()), sess);
                let linearised = turn::linearise(&Form::quotient(Poly::one(), den.num), sess);
                (settled.map(|f| f.poisoned), linearised.map(|f| f.poisoned))
            })
        });
        assert_eq!((settled, linearised), (Some(true), Some(true)));
    }

    /// **Rule H proves no false angle sum**: `cos a cos b − sin a sin b`
    /// against `cos(a − b)`, member 10 of a ring of 12 against the
    /// whole turn, member 3 of a ring of 9 (at a rational cosine) turned
    /// by a ninth against member 6 (at another), and `sin(π/6)` against
    /// `1/3`, are not the zero form.
    #[test]
    fn rule_h_proves_no_false_angle_sum() {
        let (out, counts) = with_session_rules(budget(), SymRules::shipped(), || {
            let (a, b) = (p("a", 0.3), p("b", 1.1));
            let ((sa, ca), (sb, cb)) = (a.sin_cos(), b.sin_cos());
            let r = p("r", 2.0);
            let (s1, c1) = turned(1, 12);
            let (sk, ck) = turned(10, 12);
            let ((s9, c9), (s3, c3), (_, c6)) = (turned(1, 9), turned(3, 9), turned(6, 9));
            [
                decides_zero(ca * cb - sa * sb - (a - b).cos()),
                decides_zero(ck * (r * c1) - sk * (r * s1) - r),
                decides_zero(c9 * (r * c3) - s9 * (r * s3) - r * c6),
                decides_zero(turned(1, 12).0 - Sym::from_f64(1.0) / Sym::from_f64(3.0)),
            ]
        });
        assert_eq!((out, counts.symbolic), ([false; 4], 0), "{counts:?}");
    }

    /// MEASUREMENT (rule H's budget): a chain of `m` rotations about
    /// alternating axes, `Rz(t₀)·Rx(t₁)·Rz(t₂)·…` applied to a point,
    /// against the same chain with `Rz(t₀)` written as two rotations by
    /// half of it: a product of `m` atoms of one class is up to
    /// `2^(m−1)` terms. Prints, per `m` and per budget, each
    /// coordinate's verdict, the receipt and the time.
    #[test]
    #[ignore = "measurement: cargo nextest run -p geom-core --run-ignored only measure_rule_h_turn_chain --no-capture"]
    fn measure_rule_h_turn_chain() {
        type V = [Sym<f64>; 3];
        type Pair = (Sym<f64>, Sym<f64>);
        fn rz((s, c): Pair, v: V) -> V {
            [c * v[0] - s * v[1], s * v[0] + c * v[1], v[2]]
        }
        fn rx((s, c): Pair, v: V) -> V {
            [v[0], c * v[1] - s * v[2], s * v[1] + c * v[2]]
        }
        for symbolic in [false, true] {
            for max_terms in [256, 4096] {
                for m in 2..=8u64 {
                    let b = SymBudget {
                        max_terms,
                        max_degree: 128,
                    };
                    let t0 = std::time::Instant::now();
                    let (out, counts) = with_session_rules(b, SymRules::shipped(), || {
                        let theta = |i: u64| p(&format!("t{i}"), 0.1 + 0.2 * i as f64);
                        let angle = |i: u64| -> Pair {
                            if symbolic {
                                theta(i).sin_cos()
                            } else {
                                turned(1, 5 + i)
                            }
                        };
                        let half = |i: u64| -> Pair {
                            if symbolic {
                                (theta(i) / Sym::from_f64(2.0)).sin_cos()
                            } else {
                                turned(1, 2 * (5 + i))
                            }
                        };
                        let chain = |first: &dyn Fn(V) -> V| {
                            let mut v: V = [p("x", 1.0), p("y", 2.0), p("z", 3.0)];
                            for i in (1..m).rev() {
                                v = if i % 2 == 0 {
                                    rz(angle(i), v)
                                } else {
                                    rx(angle(i), v)
                                };
                            }
                            first(v)
                        };
                        let one = chain(&|v| rz(angle(0), v));
                        let two = chain(&|v| rz(half(0), rz(half(0), v)));
                        [0, 1, 2].map(|i| decides_zero(one[i] - two[i]))
                    });
                    println!(
                        "chain {} m = {m} terms {max_terms}: {out:?} symbolic {} numeric {} frozen {} {:.3} s",
                        if symbolic { "symbolic" } else { "constant" },
                        counts.symbolic,
                        counts.numeric,
                        counts.frozen,
                        t0.elapsed().as_secs_f64()
                    );
                }
            }
        }
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
        assert_eq!(counts.symbolic, 1, "{counts:?}");
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
        assert_eq!(counts.symbolic, 3, "{counts:?}");
    }

    /// **A root and a magnitude over a signed bracket are not folded.**
    /// `sqrt(r²) − r` and `abs(r) − r` are zero wherever `r > 0`, and
    /// `r`'s bracket here is strictly positive — but that is a sign of
    /// the box, not a decision the construction took, so a zero through
    /// it would not be E12's claim. With every rule on, both decide
    /// numerically. (`max(r, −r) − r` over the same bracket IS proven at
    /// an exact witness: `max` is a decision the construction takes —
    /// `sym_root_rows`'s `a_max_is_a_decision_and_a_magnitude_is_not`.)
    #[test]
    fn a_root_and_a_magnitude_over_a_signed_bracket_are_not_folded() {
        let (out, counts) = with_session_rules(budget(), SymRules::all(), || {
            let r = Sym::param_over(
                ParamSymbol::new(test_utils::symbol_id("r")),
                1.25e-3,
                1.0e-3,
                2.0e-3,
            );
            (decides_zero((r * r).sqrt() - r), decides_zero(r.abs() - r))
        });
        assert_eq!(out, (true, true), "numerically zero at the point");
        assert_eq!(
            (counts.symbolic, counts.numeric),
            (0, 2),
            "the tier proves neither: {counts:?}"
        );
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
        assert_eq!(counts.symbolic, 2, "{counts:?}");
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
        assert_eq!(counts.symbolic, 0);
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
    /// `Zero` whatever the tier does; what separates a proof from a band
    /// decision is which column the decision landed in, and that is
    /// what these rows read.
    fn how(m: Sym<f64>) -> &'static str {
        let before = session_counts().expect("inside a session");
        let _ = decides_zero(m);
        let after = session_counts().expect("inside a session");
        if after.symbolic > before.symbolic {
            "proven"
        } else {
            "numeric"
        }
    }

    /// **The structural question does not read the registry**
    /// ([`crate::Expression::proves_zero`]): two values a constructor
    /// registered equal decide Zero through the session's door, and the
    /// question, asked of their difference, proves nothing. An answer
    /// taken from the installed session's ladder is red here.
    #[test]
    fn the_structural_question_does_not_read_the_registry() {
        use crate::real::Expression;
        with_session(budget(), || {
            let (n, r) = (p("n", 1.0), p("r", 1.0));
            assert_eq!(
                n.register_equal(r, Tol::witness()),
                SymRegistration::Recorded
            );
            let residual = n - r;
            let decided = residual.sign_within(band()).unwrap();
            assert!(decided.margin.proven(), "the door proves the pair");
            assert!(
                residual
                    .expression()
                    .and_then(Expression::proves_zero)
                    .is_none()
            );
        });
    }

    /// **The kept-atom shapes close without the ladder at the question's
    /// budget**: the two identities the retry ladder was measured on
    /// (rule G shut, rule A shut) are proved by an ask with the ladder
    /// and by one without it. At 64 terms the form each retry would keep
    /// as an atom is already frozen into one.
    #[test]
    fn the_kept_atom_shapes_close_without_the_ladder_at_the_questions_budget() {
        for build in [
            closes_only_with_rule_g_off as fn() -> Sym<f64>,
            closes_only_with_rule_a_off,
        ] {
            let answers = with_session(budget(), || {
                let id = build().node();
                [SymRetry::kept_atom(), SymRetry::none()]
                    .map(|retry| question::asked(id, question::BUDGET, retry))
            })
            .0;
            assert_eq!(answers, [true, true]);
        }
    }

    /// **The door discharges the rim residual.** Without the
    /// registration every component is a numeric decision; with it
    /// every one is proven, and what the registration buys is exactly
    /// those two out of `numeric`.
    #[test]
    fn a_registered_identity_decides_zero() {
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
        let (how_on, on) = run(true);
        assert_eq!(how_on, ["proven", "proven"], "{on:?}");
        assert_eq!(
            (on.symbolic, on.numeric + 2),
            (off.symbolic + 2, off.numeric),
            "the door moves decisions out of `numeric` and nothing else"
        );
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
        assert_eq!(
            (counts.symbolic, counts.numeric),
            (0, 2),
            "nothing was recorded: {counts:?}"
        );
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
            ["numeric", "proven"],
            "asked before the registration and asked again after it: {counts:?}"
        );
        assert_eq!((counts.symbolic, counts.numeric), (1, 1));
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
        assert_eq!((counts.symbolic, counts.numeric), (0, 2));
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
        assert_eq!(row, "proven", "the squared identity, as polynomials");
        let (row, _) = with_session(budget(), || {
            let (x, y) = (p("vx", 3.0), p("vy", 4.0));
            let n2 = x * x + y * y;
            let root = n2.sqrt();
            how(n2 - root * root)
        });
        assert_eq!(
            row, "proven",
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

    /// The arc carrier's far-end identity, in miniature — the carrier at
    /// its span is the far vertex, `carrier.eval(θ) = q_to` (M10-9
    /// amendment A1; stated today as a chain of
    /// `sweep::swept::register_placed_carrier_end` and the lowering's
    /// `Arc2::register_endpoints`). The far endpoint is
    /// reached by rotating the rim vector through the span, so the
    /// residual carries `cos`/`sin` of the span `4·atan b`, which no
    /// rule relates to the independently built far vertex: it is
    /// registered per
    /// COMPONENT, because the consumer asks
    /// `carrier.eval(t1).distance(end)`.
    ///
    /// Answers the three pairs `[eval(θ), q_to, eval(θ) − q_to]`.
    fn span(theta: f64, off: f64) -> [[Sym<f64>; 2]; 3] {
        let (vx, vy) = (p("vx", 3.0), p("vy", 4.0));
        let b = p("b", theta);
        let (sn, cs) = (Sym::from_f64(4.0) * b.atan()).sin_cos();
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
        let (s0, c0) = (4.0 * theta.atan()).sin_cos();
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
        assert_eq!(rows, ["proven", "proven"], "{counts:?}");
        let (rows, counts) = with_session(budget(), || span(0.4, 0.0)[2].map(how));
        assert_eq!(
            rows,
            ["numeric", "numeric"],
            "no rule reaches this one without the registration: {counts:?}"
        );
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
        assert_eq!(rows, ["numeric", "numeric"], "nothing recorded: {counts:?}");
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
        assert_eq!(a.0.0, ["proven", "proven"]);
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
    /// read.
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
            assert_eq!(rows, ["proven"; 8], "at X = {xv}: {counts:?}");
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
                rows.iter().all(|r| *r == "proven"),
                "at X = {xv}: {rows:?} {counts:?}"
            );
        }
    }

    /// **The two spellings of one arc meet.** The certifier's carrier
    /// sample `cos t`, `sin t` at `t = σ·θ·(i/8)` — the span, the stored
    /// sweep `θ = 4·atan b` signed by the decided turn σ
    /// (`sweep::swept::arc_span`) — against the pushforward's
    /// `sin(s·θ)` and `1 − 2·sin²(s·θ/2)` at `s = i/8`, for a LITERAL
    /// bulge (the circle kernel's `1`) and for a parameter bulge of
    /// either sign. About the turn-signed axis the carrier turns by
    /// `σ·t` in the sketch plane, so its sine enters with σ. Both sides
    /// read the one `atan b` atom, so the parameter bulge meets too:
    /// no value of `b` is read.
    #[test]
    fn rule_d_meets_the_carrier_and_the_pushforward_at_every_sample() {
        let spellings = |b: Sym<f64>, sigma: f64| {
            let theta = Sym::from_f64(4.0) * b.atan();
            let span = if sigma < 0.0 {
                Sym::zero() - theta
            } else {
                theta
            };
            let one = Sym::from_f64(1.0);
            let two = Sym::from_f64(2.0);
            (0..=8)
                .map(|i| {
                    let s = Sym::from_f64(f64::from(i) / 8.0);
                    let t = Sym::zero() + (span - Sym::zero()) * s;
                    let (st, ct) = t.sin_cos();
                    let sin = (s * theta).sin_cos().0;
                    let cos_m1 = -(two * (s * theta * Sym::from_f64(0.5)).sin_cos().0.powi(2));
                    (
                        how(Sym::from_f64(sigma) * st - sin),
                        how(ct - (cos_m1 + one)),
                    )
                })
                .collect::<Vec<_>>()
        };
        let (rows, _) = with_session(budget(), || spellings(Sym::from_f64(1.0), 1.0));
        assert!(
            rows.iter().all(|r| *r == ("proven", "proven")),
            "literal bulge: {rows:?}"
        );
        for (bv, sigma) in [(0.7, 1.0), (-0.7, -1.0)] {
            let (rows, _) = with_session(budget(), || spellings(p("bulge", bv), sigma));
            assert!(
                rows.iter().all(|r| *r == ("proven", "proven")),
                "a parameter bulge at {bv}, turn {sigma}: {rows:?}"
            );
        }
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
        assert_eq!(&rows[2..], ["proven", "proven"]);
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
        assert_eq!(rows, ["proven"; 4], "{counts:?}");
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
    /// −1`, `cos(2π) = 1` decide as theorems. So does `cos(π/3) = 1/2`,
    /// by the fold of a rational multiple of π whose value is rational
    /// (`trig_at_rational_pi`). `cos(π/5)`, which is irrational, and
    /// `cos(π + atan X)` never fold.
    #[test]
    fn rule_d_folds_trig_at_rational_multiples_of_pi_and_nothing_else() {
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
                how((pi / Sym::from_f64(3.0)).sin_cos().1 - half),
            ]
        });
        assert_eq!(rows, ["proven"; 6], "{counts:?}");
        // Rule H shut: with it, both rows are angle sums and theorems
        // (`m10_10_r2_probes`' `r2_half_pi_folds_only_the_exact_form`).
        let (rows, _) = with_session_rules(budget(), SymRules::without_angle_sum(), || {
            let pi = Sym::<f64>::pi();
            let x = p("x", 0.3);
            // A value-equal spelling on the other side keeps each row
            // a genuine identity that must stay NUMERIC.
            [
                how((pi / Sym::from_f64(5.0)).sin_cos().1
                    + (pi * Sym::from_f64(4.0) / Sym::from_f64(5.0)).sin_cos().1),
                how((pi + x.atan()).sin_cos().1 + x.atan().sin_cos().1),
            ]
        });
        assert_eq!(rows, ["numeric"; 2]);
    }
    /// A session with `s = sqrt(x + 1)` minted, and the form `s² + s`
    /// over it, which rule A takes to `x + 1 + s`.
    fn sqrt_session() -> (Session, u128, Arc<Form>) {
        let mut sess = Session::new(budget(), SymRules::shipped(), SymRetry::none(), None);
        let (s, arg) = sqrt_atom();
        mint_sqrt(&mut sess, s, &arg);
        (sess, s, s_squared_plus_s(s))
    }

    fn sqrt_atom() -> (u128, Form) {
        let x = Form::poly(Poly::indet(indet_param(1)));
        let arg = x.add(&Form::poly(Poly::one()), budget()).unwrap();
        (indet_atom(SymOp::Sqrt.tag(), 0, &[arg.digest()]), arg)
    }

    fn mint_sqrt(sess: &mut Session, s: u128, arg: &Form) {
        mint_atom(sess, s, true, || AtomInfo {
            op: SymOp::Sqrt,
            payload: 0,
            args: [Some(Arc::new(arg.clone())), None, None],
        });
    }

    fn s_squared_plus_s(s: u128) -> Arc<Form> {
        let mut p = Poly::zero();
        p.insert(vec![(s, 2)], Rat::one()).unwrap();
        p.insert(vec![(s, 1)], Rat::one()).unwrap();
        Arc::new(Form::poly(p))
    }

    /// The reduction itself, under the session's rules at the ring
    /// bound in force.
    fn direct(sess: &Session, f: &Form) -> Option<Form> {
        algebra::reduce_steps(f, sess.rules, sess.budget, &sess.atoms, EARLY_STEPS)
    }

    /// The memo's answer, and whether an ENTRY answered it.
    fn ask(sess: &mut Session, f: &Arc<Form>) -> (Option<Form>, bool) {
        let hits = sess.reduction_hits;
        let out = reduce_per_node(sess, f);
        (out.map(Arc::unwrap_or_clone), sess.reduction_hits > hits)
    }

    /// **The reduction memo answers what the reduction builds**: a form
    /// carrying a `sqrt` atom squared is reduced once and answered by
    /// its entry the second time, both answers the reduction's own.
    /// Another rule set or another ring bound is another key, and a
    /// form none of whose squared ids is in the atom table is its own
    /// answer, the same allocation, with no entry.
    #[test]
    fn the_reduction_memo_answers_what_the_reduction_builds() {
        let (mut sess, s, f) = sqrt_session();
        let want = direct(&sess, &f);
        assert_ne!(want.as_ref(), Some(&*f), "the square is substituted");
        assert_eq!(ask(&mut sess, &f), (want.clone(), false));
        assert_eq!(
            ask(&mut sess, &f),
            (want.clone(), true),
            "the second ask is the entry's"
        );

        let wide = rational::with_coeff_bound(512, || ask(&mut sess, &f));
        assert_eq!(
            wide,
            (want.clone(), false),
            "another ring bound is another key"
        );
        sess.rules = SymRules::without_the_algebra();
        assert_eq!(
            ask(&mut sess, &f),
            (Some((*f).clone()), false),
            "another rule set is another key, and without rule A nothing is substituted"
        );
        assert_eq!(sess.reductions[&f.digest()].len(), 3);

        sess.rules = SymRules::shipped();
        let linear = Arc::new(Form::poly(Poly::indet(s)));
        let out = reduce_per_node(&mut sess, &linear).unwrap();
        assert!(
            Arc::ptr_eq(&out, &linear),
            "its own answer, the same allocation"
        );
        assert!(!sess.reductions.contains_key(&linear.digest()));
    }

    /// **The whole form is in the key**: an entry planted in `f`'s
    /// digest bucket for ANOTHER input, under the same rules and ring
    /// bound, with an answer the reduction would never give, is a miss.
    /// Reds with the whole-form compare dropped from the key.
    #[test]
    fn a_bucket_mate_with_another_input_is_a_miss() {
        let (mut sess, s, f) = sqrt_session();
        let want = direct(&sess, &f);
        sess.reductions
            .entry(f.digest())
            .or_default()
            .push(Reduction {
                input: Arc::new(Form::poly(Poly::indet(s))),
                rules: sess.rules,
                bits: rational::coeff_bound(),
                absent: Box::new([]),
                out: None,
            });
        sess.reductions_held += 1;
        assert_eq!(
            ask(&mut sess, &f),
            (want, false),
            "a bucket-mate is not this form"
        );
    }

    /// **Every ladder attempt is its own key**: the kept-atom ladder's
    /// two masks over the shipped set, and a ring retry, each miss the
    /// first attempt's entry and answer the reduction under THAT
    /// attempt's rules and ring. Reds with the rules or the ring bound
    /// dropped from the key.
    #[test]
    fn every_ladder_attempt_is_its_own_key() {
        let (mut sess, _, f) = sqrt_session();
        let first = sess.rules;
        assert!(!ask(&mut sess, &f).1);
        let retry = SymRetry {
            bits: Some(512),
            ..SymRetry::kept_atom()
        };
        for (_, rules, bits) in retry.attempts(first) {
            sess.rules = rules;
            let want = rational::with_coeff_bound(bits, || direct(&sess, &f));
            let got = rational::with_coeff_bound(bits, || ask(&mut sess, &f));
            assert_eq!(got, (want, false), "attempt {rules:?} at {bits}");
        }
        sess.rules = first;
        assert!(
            ask(&mut sess, &f).1,
            "the first attempt's entry still answers"
        );
    }

    /// **An atom minted after a reduction is a miss**: `s²` is reduced
    /// while `s` is not yet in the atom table (nothing to substitute),
    /// then `s` is minted. The entry noted `s` as a lookup that found
    /// nothing, so it no longer answers, and the reduction runs again
    /// and substitutes. Reds with the entry's absent-set check dropped.
    #[test]
    fn an_atom_minted_after_a_reduction_is_a_miss() {
        let mut sess = Session::new(budget(), SymRules::shipped(), SymRetry::none(), None);
        let (s, arg) = sqrt_atom();
        let f = s_squared_plus_s(s);
        // An unrelated `cos` atom in the table, squared beside `s`: the
        // memo keeps an entry only for a form one of whose squared ids
        // IS in the table, and rule B never substitutes a `cos`.
        let other = indet_atom(SymOp::Cos.tag(), 0, &[Form::poly(Poly::one()).digest()]);
        mint_atom(&mut sess, other, true, || AtomInfo {
            op: SymOp::Cos,
            payload: 0,
            args: [Some(Arc::new(Form::poly(Poly::one()))), None, None],
        });
        let mut p = f.num.clone();
        p.insert(vec![(other, 2)], Rat::one()).unwrap();
        let f = Arc::new(Form::poly(p));
        let before = direct(&sess, &f);
        assert_eq!(
            before.as_ref(),
            Some(&*f),
            "no record of `s`, nothing substituted"
        );
        assert_eq!(ask(&mut sess, &f), (before.clone(), false));
        assert_eq!(ask(&mut sess, &f), (before, true));
        mint_sqrt(&mut sess, s, &arg);
        let after = direct(&sess, &f);
        assert_ne!(
            after.as_ref(),
            Some(&*f),
            "with `s` minted the square is substituted"
        );
        assert_eq!(
            ask(&mut sess, &f),
            (after, false),
            "the stale entry does not answer"
        );
        assert_eq!(
            sess.reductions[&f.digest()].len(),
            1,
            "it was replaced, not kept"
        );
    }
}
