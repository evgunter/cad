# M5 PR 1 spec (binding): interval-transcendentals adoption

Status: BINDING for the PR 1 implementer. Deviations must be
reported in the implementation report, never improvised silently.
Authority: DESIGN.md crate-table inari row (adoption GREEN-LIT,
Evan in-session 2026-07-27), CURVED-DESIGN C9/OQ8, M5-PLAN PR 1 +
R2. The M0 poison-channel contract is the acceptance bar.

## What this PR is

Swap the kernel's `T = Interval` transcendental backend from
`inari` (LGPL-encumbered via gmp-mpfr-sys/rug behind the `interval`
cargo feature) to the in-repo `interval-transcendentals` crate
(#115: proven per-function libm error pads, dual-oracle certified,
MIT-clean). Pure backend swap: no semantic change to the interval
scalar's contract, no new capabilities, no kernel API change. The
crate joins the workspace (un-excluded). inari's full retirement
from the tree completes when PR 2 (the C9 ring) also lands — this
PR does not need to remove it if the wrapper still uses inari's
ring ops (see the two acceptable shapes below).

## The contract being preserved (M0, PR #7 — restated as the bar)

1. **Poison through values, never decisions**: the decoration-as-
   poison channel (`decoration < Def ⇒ Indeterminate(Invalid)`) —
   whatever the new backend's equivalent signal is, the wrapper
   maps it to the SAME `Indeterminate` behavior at the SAME
   operations. Silent domain clamps never decide anything.
2. **Containment is the interval contract**: every operation's
   result encloses the true image. The new pads are wider than
   inari's MPFR-tight results in places — wider is sound; NARROWER
   than truth anywhere is a correctness bug. The differential lane
   asserts containment relations, not width equality.
3. **Tight `pown` powi override** stays (interval squares of
   possibly-zero quantities: x² lo ≥ 0 — the M2 poison lesson).
4. **`Bounds` certification trait semantics unchanged**: poison-
   visible NaN brackets for empty AND NaI/invalid.
5. **Sliver-band terminal semantics unchanged** (an enclosure
   wholly inside (ε, Kε) never refines — escalates as a genuine
   sliver).
6. **`sin_cos` stays the primitive** (sin/cos are projections,
   overridable only bit-identically).
7. **D9**: same build + same inputs ⇒ bit-identical interval
   endpoints. The new backend is libm-over-f64 — confirm no
   platform-conditional paths.

## Two acceptable shapes (implementer picks, reports which and why)

- **Shape A (preferred if clean)**: the wrapper type re-homes onto
  interval-transcendentals' interval type entirely (ring ops +
  transcendentals from the crate). inari drops from the kernel's
  dependency tree in THIS PR; the dev-dependency differential
  oracle may keep it.
- **Shape B**: ring ops stay on inari's type, transcendentals
  route through interval-transcendentals via endpoint conversion.
  Acceptable only if Shape A hits a real blocker (report it);
  leaves inari in-tree until PR 2.

Either way: the `interval` cargo feature boundary stays exactly
where it is (no build-configuration change for consumers); the
gmp build dependency disappears from interval builds under Shape A
(record the measured build-time delta honestly — #115 claimed ~93×
against the gmp stack; restate as measured, not inherited).

## What may legitimately change (and how it is handled)

Interval ENDPOINT VALUES of transcendental results will differ
(pad-widened vs MPFR-tight). Consequences:
- Tests pinning exact endpoint bit-patterns of transcendental
  results are updated to the new backend's values WITH a comment
  naming the change of backend — never loosened to approximate
  comparisons.
- Tests asserting containment/poison/trilean OUTCOMES must pass
  unchanged. If any predicate verdict on the existing battery
  flips (a wider enclosure turning definite into indeterminate),
  STOP and report — that is a finding about margin headroom, not
  a test to fix. Expected: zero flips (the 8b K-probe showed a
  12-decade empty margin gap; pad widening is ulps).
- The interval roundtrip/persistence rows and the full battery at
  3ε + Interval are the regression net.

## Acceptance (all foreground, one row at a time)

1. Full battery green at ε ∈ {1e-6, 1e-9, 1e-12} + the Interval
   suites; corpus persistence + interval roundtrip rows green.
2. The M0 interval scalar suites (poison propagation, powi
   containment, sliver-terminal) green with zero OUTCOME changes.
3. The #115 differential lane re-pointed: kernel-wrapper results
   vs BOTH oracles (inari as dev-dep, computable where wired) on
   the existing case corpus — containment mutual-consistency
   asserted.
4. A poison-conservation test: for every transcendental at a
   domain-violating input, the wrapper yields Indeterminate
   exactly as the inari-backed wrapper did (enumerate the
   functions; pin each).
5. `cargo tree` (or equivalent) evidence in the PR writeup: under
   Shape A, no gmp-mpfr-sys/rug/inari in any non-dev dependency
   path of any build configuration.
6. CI: the hosted matrix rows unchanged in count or grown (never
   shrunk); watcher floor stays ≥ 18 (bump if rows added).

## Out of scope

The C9 ring (PR 2); any new Real methods; any change to predicate
code, k_stats, or certification; quarantine-text retirement in
DESIGN.md (orchestrator does it when PR 1 + PR 2 are both in).

## SEAM FACTS (from the orchestrator's seam survey, 2026-07-27;
## verify against source before relying on any line number)

**Swap surface is exactly three files.** The sole inari consumer is
`crates/geom-core/src/interval.rs` (wrapper `Interval(DecInterval)`
at :91). Cargo edges: `crates/geom-core/Cargo.toml:16,20`
(`interval = ["dep:inari", "inari/gmp"]`) and root `Cargo.toml:57`.
Grep-verified: ZERO direct `inari::`/`DecInterval`/`Decoration` use
anywhere else in crates/, demos/, tools/ (four prose mentions in
comments only). All other crates forward the `interval` feature.

**Shape A is the expected shape**: re-home the wrapper onto
`interval_transcendentals::DInterval` entirely (it provides ring
ops with outward rounding, NOT just transcendentals — src/arith.rs,
src/round.rs). The crate stays its OWN workspace (like tools/k-lint,
demos/tour); geom-core takes a path dependency
(`interval = ["dep:interval-transcendentals"]`). Its gmp-backed
inari dev-oracle then never enters kernel builds. Remove inari from
root Cargo.toml. Leave `.cargo/config.toml` target-cpu unchanged
(report the possible relaxation, don't do it).

**API mapping** (inari → DInterval): `DecInterval::try_from((l,h))`
→ `from_bounds` (same NaN/inverted ⇒ NaI shape); `NAI`/`EMPTY` →
`nai()`/`empty()`; `PI`/`TAU` → `consts::pi()`/`tau()` (bit-identical
1-ulp enclosures at Com — the π/τ endpoint-bit pins at
interval.rs:820-833 hold); `Interval::ENTIRE` → `entire()`;
`const_interval!` → `from_bounds` at the KinkJacobian sites
(:545, :573, :605, :635, :662); `inf()/sup()` → `lo()/hi()` (NaN
for empty AND NaI already — the Bounds poison-visibility contract
at :398-412 is natural); `min/max` → `min_i/max_i`; `convex_hull` →
`hull` (crate's hull keeps min-of-decorations — matches
`tangent_hull`'s deliberate non-1788 semantics at :505-521, cite
semantics-diffs D7); `contains` same; test-only `subset`/`wid` —
add tiny helpers or compute locally in tests.

**Required crate addition (small, in-crate, allowed)**: a
decoration-cap primitive (DInterval has no public `set_dec`; fields
are pub(crate)) for `cap_decoration` (:489-494), `tangent_hull`,
and the hand-written 3-way `copysign` (:275-295). Suggested:
`pub fn with_dec_capped(self, cap: Decoration) -> Self`. Keep the
crate's invariants (src/interval.rs:33-40) intact; add unit tests
in-crate for anything added.

**Semantics divergences — the authoritative list is
`interval-transcendentals/docs/semantics-diffs.md` (D1–D8).**
Expected kernel test-pin changes, each updated WITH a
backend-change comment citing the D-number:
- D8 `floor`: interval.rs:1230 (`at_edge` asserts `Dac`) flips to
  `Com` (crate pins the divergence itself at tests/edges.rs:182).
- D2/D4 `atan2`: check the 25-row boundary table
  (interval.rs:1051-1116) and any atan2 decoration/hull pins
  against D2 (upper-half-plane ray ⇒ Com) and D4 (origin-box full
  [-π,π] hull).
- `powi(n=0)` preserves input decoration — matches the
  non-laundering pins at :898/:917 (no change expected).
- `sin_cos` is literally `(sin(), cos())` — the pair-vs-projection
  bit-identity pin at :980-991 holds.
- `repr_bits` (:143-152): 5 decoration variants map 1:1; keep the
  same u8 encoding so bit_identity.rs:62-65 is untouched.

**Width-sensitive pins to verify individually** (4-ulp pads +
1-ulp arithmetic should satisfy all; any FAILURE of a width BUDGET
is a stop-and-report, any exact-VALUE pin updates with comment):
interval.rs:841-848 (1-ulp exact ops), :968 (sin width ≤1e-15),
:1291-1304 (reduce_periodic wid <1e-14), :1476 (pythagorean wid
<1e-13); review_m0_pr4.rs:343-348 (bitwise agreement +,*,sqrt),
:354-357 (div 1-ulp), :375-393 (powi-diverges probe); review_m0_
pr5.rs:727; review_m2_pr1_interval.rs:94-95,:186-187,:207,:237,
:240,:426,:491; profile/tests/interval_lane.rs:30-31;
topo/tests/interval_body.rs:78-80,:106; editor-core
m4_pr1_eval.rs:167, m4_pr2_eval_interval.rs:103; sweep
mass_props_interval.rs:44,157, review_m2_pr7_interval.rs:49;
topo review_m3_pr6.rs:123,184. Big-argument honesty (sin/cos
degrade to [-1,1] for |x| ≳ 4e15) — confirm no kernel test lands
there.

**CI**: lanes (ci.yml:158-175, scripts/ci-local.sh interval rows)
keep identical invocations — they just build gmp-free now. Also
run the crate's OWN fast suites (tests/edges.rs + unit tests) in
your battery; run tests/certify.rs (the 300k differential lane vs
inari) if ~/.cache/gmp-mpfr-sys makes the dev-oracle build cheap —
it should be warm on this machine; report either way.
