# VREV — a v-reversal door on `NurbsSurface`, exact or refused

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-12).** Binds
the implementer of unit `sweep-test-rebuilds-validated-net-for-v-reversal`;
deleted at merge per `docs/DOC-LEDGER.md`. Read
`docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/sweep-test-rebuilds-validated-net-for-v-reversal.md`.

## 0. The finding

`crates/sweep/tests/review_probes_m8_4.rs`, `seam_on_chart(reverse_v)`:
takes a validated `NurbsSurface<f64>` off a body, permutes control and
weights by `j ↦ nv − 1 − j`, carries BOTH knot vectors verbatim, and
pushes the result through `NurbsSurface::new(..).unwrap()` — a `Result`
that is unreachable on this data (a permutation changes no count and no
weight), and a hand-rebuilt net where `geom` should hand out a door.
The row leaves open whether the fix is a door or a validated-parts
admission.

**What the test actually built, and when it is the same surface.**
Reversing the net in `v` while carrying `knots_v` verbatim yields
`S'(u, v) = S(u, lo + hi − v)` — the same point set with `v` reversed —
**exactly when `knots_v` is mirror-symmetric about its midpoint**
(`k_i + k_{m−i} = lo + hi` for every `i`). The fixture's bowed wall is a
three-section degree-2 loft, so its `v` knots are `{lo,lo,lo,hi,hi,hi}`
and the test was right by luck of the fixture. For a non-symmetric
`knots_v` the same recipe is a DIFFERENT surface, and the general
reflection `k ↦ lo + hi − k` is not exact in `f64` — a door that
computed it would mint structure off by an ulp, which D9 forbids for a
structural map (`scalar_lift.rs`'s "carried verbatim, no arithmetic").

## 1. What this unit delivers

**The door**: `NurbsSurface::reversed_v(&self) -> Result<Self, ..>` in
`crates/geom/src/surfaces/nurbs.rs` — the same surface with its `v`
orientation reversed: the net's rows reversed in `v`, weights by the
same permutation, `knots_u` verbatim, `knots_v` verbatim; **defined only
when `knots_v` is exactly mirror-symmetric**, checked by the exact `f64`
identity above (compare `k_i + k_{m−i}` against `lo + hi` bit for bit;
argue in the doc why that comparison is well-defined and why the
clamped ends always pass it), and refusing typed otherwise with a
reason that names the asymmetric pair. Construction goes through
`from_validated_parts` with the count argument stated: a permutation of
a validated net changes no count and no weight. `reversed_u` comes by
the crate's conjugation (`transposed().reversed_v()?.transposed()`),
with a row that the two agree with a direct permutation.

Choose the error type in the crate's vocabulary (`KnotAlgebraError`
carries the knot-algebra refusals; a new variant there, or a small
enum beside the door, is the implementer's call — say why).

**Rejected in the spec, so the PR does not re-argue it**: a general
reflecting door (inexact structure; no consumer); a pub
`from_validated_parts` (an admission every caller could abuse, when the
door can say exactly what it admits).

**The caller**: `seam_on_chart` calls `reversed_v()` and unwraps the
refusal at the test boundary with its payload visible. The row's
`.expect("the v-reversed chart is the same point set")` on
`set_face_surface` stays: it is now a claim the door makes.

## 2. The pin

- **Same point set**: a row over a surface with a NON-uniform but
  symmetric `knots_v` (e.g. `{0,0,0,¼,¾,1,1,1}`) that `reversed_v()`
  evaluates to `S(u, lo + hi − v)` at a grid of parameters — bit for bit
  where `lo + hi − v` is exact (dyadic parameters), and the reversal
  involutive (`reversed_v().reversed_v()` is the source, bit for bit).
- **Refusal**: a surface with an asymmetric `knots_v` (e.g.
  `{0,0,0,¼,1,1,1}`) refuses, naming the pair; the row also shows why —
  evaluate the hand-permuted net on verbatim knots at one parameter and
  show it is NOT `S(u, lo + hi − v)`, so the refusal protects something.
- **Conjugation**: `reversed_u` agrees with a direct row permutation.
- The existing `review_probes_m8_4` rows are green unchanged.

## 3. Sweep

The class: a test or demo that rebuilds a validated NURBS net by hand
and re-validates it (`NurbsSurface::new(..).unwrap()` /
`NurbsCurve*::new(..).unwrap()` over parts taken from an existing
curve or surface). PR 1782's style review swept `crates/sweep/`; sweep
the rest of `crates/*/tests`, `demos/`, `tools/` and `benches/`, hit
list with disposition in the PR body, blind spot stated.

## 4. Fence

`crates/geom/src/surfaces/nurbs.rs` (PROPS') and
`crates/sweep/tests/review_probes_m8_4.rs` (S-TCOST's and S-TINT's);
announced by the orchestrator. Merge `origin/main` before opening the
PR; `python3 scripts/work.py territory --base origin/main` output in
the PR body.

## 5. Verification and report

Local: `cargo nextest run -p geom -p sweep` at default features; the
interval lane rides the gate. Hosted CI is the verification of record;
poll to conclusion in the foreground. Report ≤150 lines: the door's
signature and error type, the symmetry check's argument, the pins, the
sweep's hit list and blind spot, deviations, rows filed and where, PR
number, head SHA, CI run id and conclusion.
