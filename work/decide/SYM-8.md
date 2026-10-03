---
id: SYM-8
kind: unit
title: the manifest sign: abs and copysign atoms whose sign the form already shows, measured first on the tilt-U wall
status: closed
opened: 2026-09-14
branch: sym/8-manifest-sign
refs: [derived-frame-placement-freezes-on-the-symbolic-lane, coefficient-ring-width-is-not-monotone-in-reach]
priority: P1
cost: H
pr: 2616
closed: 2026-09-21
---



## What

SYM-5 PR-2's review found the next wall on the derived-frame item: a frame tilted about the OTHER axis refuses identically with rule E on and off, the degree wall become a TERM wall behind `copysign(1, 1/sqrt(P(t)))` and `abs(1/sqrt(P(t)))` atoms of a quantity the form already shows positive. Phase 1 measures the wall and re-takes the ring item's recorded loss with a hand-planted fold; Phase 2 is rule F, the manifest sign, shipped only if no split or ceiling moves down anywhere. Block SYM-B2 slot 1 (H / NUMERIC, pre-draw); the full v6 dual. Spec: `docs/SYM-8-SPEC.md`.

## Dispatched (2026-09-14, ~22:05Z)

Block SYM-B2 slot 1, arm OPUS per the block's draw (byte 56 ⇒ fable at
slot 2); the v6 dual at the PR. Inside the program's paths
(`sym.rs`/`sym/*` and the announced tests-family overlap); `linalg/vec.rs`
is not touched.

## The manifest-POSITIVE predicate, written down before it is coded (Phase 1.2)

A `Form` is a quotient `N/D` of polynomials over the session's
indeterminates (parameter symbols, π, opaque reals, and the atoms).
A poisoned form is refused outright.

**Manifestly non-negative polynomial** — `manifest::nonneg`'s (then
`trig::manifestly_nonneg`'s)
per-polynomial half, unchanged: every term has a non-negative
coefficient and a monomial each of whose indeterminates is raised to an
EVEN power or is a `Sqrt`/`Abs` atom (any power), so every term is a
product of non-negative reals wherever it has a value; or the
polynomial is a perfect square (`signed::poly_sqrt`).

**Manifestly positive indeterminate** — a `Sqrt` atom or an `Abs` atom
whose ARGUMENT form is manifestly positive (`sqrt(X) > 0` and
`|X| = X > 0` for `X > 0`). Nothing else: a parameter, an opaque real,
a frozen node, `π`, and a `Sqrt`/`Abs` atom over an argument that is
only non-negative are all excluded.

**Manifestly positive polynomial** — every term has a non-negative
coefficient and a non-negative monomial (the term-wise branch above,
and NOT the perfect-square branch), and at least ONE term has a
strictly positive coefficient and a monomial whose every indeterminate
is manifestly positive — the empty monomial included, which is a
positive constant. Then `p = (a positive term) + (non-negative terms)`
is `> 0` wherever it has a value.

The perfect-square branch is deliberately not carried over: `p = r²` is
non-negative but vanishes at every real zero of `r`, and its terms may
be negative, so the decomposition the positivity argument rests on is
unavailable there.

**Manifestly positive FORM** — `N/D` where `N` is a manifestly positive
polynomial and `D` is a manifestly non-negative one. `D` needs only
non-negativity because a point where `D` vanishes is a point the value
channel divided by zero at, and clause 1 — the whole-box certification
— has already refused there (`quotient`'s four-source argument for the
same side condition). So `D > 0` and `N > 0` wherever the form has a
value, hence `N/D > 0` there.

**What does NOT count**: a sum of squares alone (`x² + y²` — zero at
the origin); a bare even power; any form with a parameter of unknown
sign, an opaque real or a frozen node at an odd power, or a negative
coefficient on any term; a `sqrt`/`abs` atom over an argument that is
not itself manifestly positive; the zero form; poison.

**The signed-zero edge, and why strict positivity closes it.**
`copysign` reads a SIGN BIT, not a sign: IEEE gives
`copysign(1, +0.0) = +1` and `copysign(1, −0.0) = −1`, so at a real
zero of `X` the node `copysign(Y, X)` does not denote a function of the
REAL value of `X` at all — which of the two it takes is decided by how
the value channel happened to spell the zero. A fold under mere
NON-negativity would therefore be a claim about that spelling and not
an identity of reals, and on the `−0.0` branch it is false. Strict
positivity excludes the point: a manifestly positive `X` is `> 0` at
every point of the box where the form has a value (clause 1 having
refused every point where it does not), so `sign(X) = +1` there
unambiguously and `copysign(Y, X) = |Y|` is an equality of reals with
no value read. `abs(X) = X` would be sound under non-negativity alone;
it is held to the same predicate here, because
`coefficient-ring-width-is-not-monotone-in-reach` records
`abs(X) = X` on a syntactically non-negative `X` losing ten decisions
on R1's boss, and because one predicate with one argument is what a
measurement can narrow.

## The pad's four: RULED (SYM orchestrator, 2026-09-21)

Both blinded reviews (R1 Opus, R2 Fable; MERGEABLE-AFTER-FIXES on both
sides) read the spec's Phase 1.3 stop clause — *"If a split moves DOWN
or a ceiling falls under the fold, that is the unit's first finding and
the rule's predicate is narrowed until it does not"* — as **literally
tripped**: on R2's rounded pad, at the scale it certifies whole at,
rule F moves `symbolic_zero` 858 → 854, `registered` 104 → 128 and
`numeric` 991 → 971, so four decisions go from a theorem the tier
proved to an axiom a constructor stated. Both judged the ship-on
defensible; neither judged it the lane's to make.

**The ruling: rule F ships ON, and the clause is a spec deviation
ratified here.** It is the SYM orchestrator's call, made on
2026-09-21, and Ev may overrule it. The reasons, on the record:

- no decision is LOST — the 1953 decisions are the same 1953, and
  nothing rises into `numeric`; twenty leave it;
- no per-predicate split moves at any measured document's nominal, and
  no whole-certifying ceiling moves by a digit on any of the eight,
  with the over-band set at ceiling + δ identical too;
- the four are re-taken by the registry, which is a weaker instrument
  than the tier's own arithmetic and still a discharge;
- the spec's own remedy is unavailable: the lane showed by execution
  that narrowing cannot separate the pad's atom from the tilt-`u`
  document's — both are `abs(n.z)` of one construction — so "narrow
  until it does not" and "stop" are the same instruction here, and
  stopping costs the reach the unit was drawn for;
- what moved is now GUARDED rather than commented:
  `m10_9_pins_interval`'s `Study` pins `symbolic_zero` beside
  `registered` on all five documents, so a later change costing four
  more theorems reds.

This is recorded as "spec clause not met, ruled" and not as a
disclosure. The distinction matters because a disclosure is a lane
saying what it did; this is a program saying what it will accept.
