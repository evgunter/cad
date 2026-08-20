# Semantics differences vs inari (each one deliberate and documented)

The kernel's contract (M0 ruling, `crates/geom-core/src/interval.rs`):
decoration is the poison channel; decoration < `Def` never decides;
partial domain misses clamp the value and poison the decoration; NaN/±inf
construct NaI. This crate mirrors all of that. The differences below are
the complete list found by design review + the differential harness;
anything new the harness finds is a bug, not a difference.

## D1 — Value tightness (systematic, one-directional)

We cannot set the rounding mode from portable Rust, so every inexact
endpoint is padded outward: ≤ 1 ulp per arithmetic/sqrt endpoint
(Lemma P1), ≤ 4 ulp per transcendental endpoint (PAD_ULPS, derivations
§2). inari is correctly rounded (0 extra ulp). Our result therefore
always CONTAINS inari's; it is never tighter. Exceptions that stay
exact: extremum bounds (`±1.0` for sin/cos, clipped ranges of
asin/acos/atan/atan2), zero lower bound of even powers over straddling
intervals, FMA-witnessed exact squares/products, exact sums (TwoSum
witness), FMA-witnessed exact quotients, `sqrt` of witnessed perfect
squares.

The exact-quotient witness was added during the M5 PR 1 adoption, when
padding *every* division unconditionally proved to be a divergence with
kernel-visible consequences rather than a mere tightness gap: `v / |v|`
for an axis-aligned `v` stopped being exactly unit, which widened every
coordinate taken against such a frame and made the kernel's exact-order
band (topo's null-edge sort — a design that rests on axis-aligned splits
over dyadic geometry classifying EXACTLY) escalate where it used to
decide. The witness is `mul_exact`'s, mirrored: `fma(q, b, -a) == 0`
above the 2Prod validity floor proves `q·b = a`, hence `a/b = q` exactly.

## D2 — Decoration on atan2's upper closed half-plane ray boxes

Box `y ∈ [0, d], x ∈ [a, b], b < 0` (negative x-axis approached only
from y ≥ 0): the restriction of atan2 to this box is defined, continuous
(value π on the ray, one-sided approach), and bounded → we return `Com`.
inari returns `Dac` (`N1_P0`/`N1_Z` in its case table) — conservative
but weaker. 1788 permits any decoration below the tightest true one, so
BOTH are compliant; ours is tighter and provably correct. This is the
single place our decoration EXCEEDS the oracle's; the harness allowlists
exactly this configuration (`x.hi < 0 && y.lo == 0`) and would fail on
any other.

## D3 — Conservative extremum/pole localization (possible over-poison)

Our pole test for `tan` (and extremum tests for sin/cos) is an
outward-rounded grid test: it can say "possibly a pole" for a pole-free
interval that comes within ~|x|·2^-52 of a pole, and for ALL intervals
with |x| ≳ 2^52 ≈ 4·10^15 (and loses it on SOME inputs from about
|x| ≈ 2^32 — see `consts::grid_possibly_hits`, which records both
thresholds and why the earlier one is the load-bearing
one). Consequences: `tan` may return Entire/`Trv` where
inari proves `Com` and a finite range; sin/cos may include ±1 where
inari's bound is fractionally smaller. Direction: more poison / more
width — sound by the escalate-never-guess policy. inari, with MPFR
reduction, essentially never over-poisons. (Kernel angles are O(τ), far
inside the exact-localization regime.)

## D4 — atan2 value hull for origin-containing boxes

inari returns quadrant-tight hulls for boxes touching/containing the
origin (e.g. `[0,b]×[0,d] → [0, π/2]`, dec `Trv`). We return the full
`[-π, π]`-padded hull with the same `Trv`. Same poison, wider value.
Rationale: at `Trv` the value cannot decide anything in the kernel
anyway, so tightness there buys nothing; if a future consumer cares,
the quadrant case table can be ported.

## D5 — Empty vs NaI construction taxonomy

Identical to the kernel wrapper, restated: `point(NaN)`, `point(±inf)`,
`from_bounds` with NaN/inverted/no-real-member bounds → NaI (`Ill`), not
inari's `Result`-based refusal (inari `try_from` returns `Err`; the
kernel maps those to NaI explicitly — we build the mapping in).
Full-domain misses (sqrt/asin/acos of fully-outside intervals,
`x/[0,0]`, atan2 of the degenerate origin) → Empty (`Trv`), matching
inari exactly.

## D6 — No `Dac`-preserving unbounded arithmetic subtleties beyond 1788

Overflow to an infinite bound caps the decoration at `Dac` (Com requires
boundedness) — same rule as inari. Noted here only because our padded
arithmetic overflows one ulp earlier than inari's in the extreme binade.

## D7 — `hull()` keeps `min(dec)`; 1788/inari give set operations `Trv`

IEEE 1788 assigns set operations (convexHull, intersection) the
decoration `Trv` — they are not function evaluations, so nothing
functional is asserted. inari follows that; our `intersection` does
too. Our `hull()` deliberately does NOT: it propagates `min` of the
operand decorations (ignoring empty operands). Rationale: in the
kernel's poison-channel reading, a decoration is a record of
domain-violation history, and hulling two enclosures whose histories
are clean produces an enclosure whose history is clean — `min(dec)`
can never exceed either input, so no poison is laundered; what it
does is let clean values stay clean through hull-shaped code paths
(the `copysign`-style tangent hull in
`crates/geom-core/src/interval.rs` is that path) instead of poisoning
them structurally. This over-asserts
relative to strict 1788 decoration semantics (which would say `Trv`), it
is the single such place, and consumers wanting 1788-strict behavior can call
`intersection`-style code or drop the decoration themselves. Flagged
by adversarial review (it was undocumented — a process violation of
this file's "complete list" claim, now corrected); behavior kept,
divergence documented, and pinned by a unit test.

## D8 — `floor` decoration uses restriction-continuity (like D2)

`floor` over a box with `floor(lo) == floor(hi)` is CONSTANT on the
box, so we return up to `Com`. inari additionally demotes to `Dac`
when the left endpoint is itself an integer (`x.inf == floor(x.inf)`),
i.e. it charges the AMBIENT discontinuity at that point even though
the restriction to the box is constant. Same philosophical split as
D2 (restriction vs ambient continuity); ours is the tighter correct
decoration under 1788's "restriction of f to x" wording. The
differential harness allowlists exactly this configuration
(`floor(lo) == floor(hi) && lo == floor(lo)`).

## Non-differences (verified by the harness)

- Decoration ordering and min-propagation through every operation.
- Domain-clamp poison (`Trv`) for partial misses of sqrt/asin/acos.
- `tan` over a genuine pole: Entire + `Trv` both sides.
- Division: straddling divisor → Entire + `Trv`; zero-touching divisor →
  half-line + `Trv`; `[0,0]` divisor → Empty.
- `powi` poison-through-`n=0`, even-power zero floor, negative-exponent
  pole semantics.
- NaI/Empty propagation through every entry point.
