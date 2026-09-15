# RATE-PAIR — SupSpeed and InfSpeed beside Margin; metered takes the inf; a sup door

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-15).** Binds
the implementer of unit `rate-pair-in-geom-core`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is `work/scalar/rate-pair-in-geom-core.md`; the
ruling is `work/scalar/D283.md` §RATIFIED, route A (PR 2457); the
evidence is `work/scalar/log.md` "The rate census".

## 0. The ruling this executes

The parameter-space ↔ model-space crossing is unified by a TYPED RATE,
not a typed length, and the rate's bound direction is the semantic
content. `Margin::metered(span, rate)`'s doc promises "a certified speed
LOWER bound"; three shipped sites pass a certified UPPER bound through it
(`pcurve_cache::trim_containment` via `chart_stretch_sup`, the
`pcurve_iso_*` slack meters via `nurbs_stretch_bounds`, and the
v-channel of `topo::pcurves::pcurve_loop_continuity` via `v_meter`) —
correct in each case (an overshoot or an escape is safe on the sup
side), argued in prose, checked by nothing.

## 1. What this unit delivers

**Two types in `crates/geom-core/src/predicate.rs`, beside `Margin`:**
`SupSpeed<T>` and `InfSpeed<T>` — meters per parameter unit, a certified
upper / lower bound on a speed. `#[repr(transparent)]` newtypes over
`T: Real`, `Copy`, no `PartialOrd`/`PartialEq` (the `Real` surface's
rule), a plain constructor (`new(T)` — a dimension-and-direction TAG,
not a positivity witness: the 0/∞ guards stay where they are, because
the sites disagree on what zero means), `get()`, and the two
conversions, each ONE operation bit-identical to the bare arithmetic
(D9, as every `Margin` door argues): `to_meters(span) = span * s` and
`to_param(m) = m / s`. A surface's pair is two values, not a type.

**`Margin::metered(span, rate: InfSpeed<T>)`** — the doc's promise made a
type; **and the sup-side sibling** (name it in the crate's vocabulary —
`Margin::metered_sup`, `escape`, or what reads best) taking `SupSpeed<T>`
for overshoot/escape metering, with the dimensional argument stated
once: dividing or multiplying by the sup under-states a chart extent and
over-states a model-space escape, which is the safe side for a refusal
claim. `docs/predicate-dimension-audit.md`'s clause (iii) rows cite the
new door where they cite `metered` for a sup.

**The producers typed at the mint:** `NurbsCurve3::speed_lower_bound`
(`InfSpeed`), `pcurve_cache::param_rate` (`InfSpeed`; the exact
closed-form rates — line 1, circle radius, ellipse minor — are inf
bounds by being exact), `chart_stretch_sup` and `nurbs_stretch_bounds`
(`SupSpeed` pairs), `topo::pcurves::v_meter` and `azimuth_arm`'s spline
arm where it is a stretch sup, `topo::split` / `splitting/classify`'s
`meter`/`scale` (`InfSpeed`), `PatchRegularity::{speed_u, speed_v}`
(`SupSpeed<f64>`; `speed_lever()` returns one; `thinness()` stays
unguarded by its contract — go through `to_param`... no: `thinness` is
an area rate over a linear rate, so it stays `floor / speed_lever().get()`
with the doc saying why the door does not apply), and `plane_nurbs_ssi`'s
local `speed` (`SupSpeed<f64>`, minted right after the #762 guard, its
three divisions through `to_param`). The SSI RECEIPT's lane tag is the
SECOND unit's — do not touch `Exhaustiveness`.

**The consumers moved:** the three blurred sites onto the sup door;
every `metered` caller passes an `InfSpeed`. `Margin::levered` with an
angular ARM (m per radian: `azimuth_arm` on analytic kinds, `azimuth_lever`,
`chart_bound`'s `u_arm`) is NOT a rate per parameter unit and stays.
`levered_inv`'s curvature uses stay.

**Out by the ruling (list as not-this-unit with the pointer):**
second-order rates (`mesh/sizing.rs`, `nurbs_cert.rs` `split_steps`),
param→param rates (`chords.rs` `nurbs_tighten`), pointwise jet speeds
(`ssi/system.rs`, `march.rs`, the projection Newton acceptances,
`enters_material_order2`), `coherence.rs` `gap_is_noise`'s zero lever,
`chart_region.rs` `certified_arms` (inf per kind gated through
`Margin::of` — file it on its owner as the class's next member, do not
convert here), the NaN-fold divergence (`work/curved/ssi-lever-arm-min-fold-hides-poison.md`).

**What must not change:** every margin value at every retyped site, bit
for bit — the doors are one operation each; the k-lint gate's predicate
counts do not move; the corpus and tour are bit-identical (a digest row
committed before the change, as PERF's units did, is the cheap proof).
`PatchRegularity`'s fields keep their meaning and the `Display`/`Debug`
of every receipt that prints a speed keeps its text.

## 2. Docs

`Margin`'s module doc gains the rate pair beside its constructor doors,
with the direction rule stated once: inf for "definitely apart" claims
(a length under-stated), sup for overshoot and escape (a chart extent
under-stated). Each producer's doc says which bound it is and why (one
sentence; the ones that already do keep theirs). The three PROPS/TRIM
rows this closes (`work/props/metered-margin-doc-promises-an-inf-bound-three-sites-pass-a-sup.md`,
`work/trim/loop-continuity-meters-u-through-levered-and-v-through-metered.md`
— its v-channel half; say what the u/v asymmetry becomes) close in the
PR with `## Closed` sections.

## 3. The pin

- The D9 differential and the digest row (§1).
- A compile-fail doctest: a `SupSpeed` cannot be passed to `metered`,
  an `InfSpeed` cannot be passed to the sup door — the direction is a
  type fact.
- One row per door that `to_meters`/`to_param` are bit-identical to the
  bare `*`/`/` on a sample including subnormals, `inf` and NaN (poison
  flows through values).
- A row at `Interval` that the doors are the interval ops (enclosure
  contains the real product/quotient).

## 4. Sweep

The class: a parameter ↔ meters conversion spelled by hand beside a
predicate. The census in `work/scalar/log.md` is the hit list; re-take
it against the tree at your merge base (it was read 2026-09-12), put it
in the PR body with a disposition per site (converted / stays with the
reason / filed), and state the blind spot (rates routed through a helper
that names neither `speed` nor a `Margin` door; non-`f64` spellings).

## 5. Fence

This program claims no paths. This unit reaches PROPS'
`crates/geom-core/src/predicate.rs`, `crates/geom-brep/src/{certify.rs,offset_meters.rs}`
and `docs/predicate-dimension-audit.md`'s rows; TRIM's
`crates/geom-brep/src/{pcurve_cache.rs,ssi.rs}` and `crates/topo/src/pcurves.rs`;
TOPO's `crates/topo/src/{split.rs,splitting/classify.rs,chord_join.rs}`.
Announced by the orchestrator; merge `origin/main` before opening the
PR; territory output in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p geom-core -p geom-brep -p topo` at default
features and `-p geom-core --features interval`; `cargo clippy
--workspace --all-targets -- -D warnings` (a `Margin` door's signature
changed). Hosted CI is the verification of record; poll to conclusion
in the foreground. Report ≤120 lines: the two types and the door
names, the producers and consumers typed, the differential's and
digest's receipt, the sweep table, deviations, rows closed and filed and
where, PR number, head SHA, CI run id and conclusion.
