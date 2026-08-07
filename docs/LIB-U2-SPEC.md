# LIB-U2 spec — PATHS algebra implementation + demo rework (binding)

Mandate (LIBRARY-DESIGN.md §L5 U2, authorized §L8): implement the
RATIFIED PartialPath authoring algebra — `docs/PATHS-DESIGN.md`
§§1–7 are BINDING design, read them first and completely — as the
generator-layer surface (D8) lowering to the existing v1 form
(`ProfileLoop`: segments + declared tangency flags), then rework
the demo corpus's profile authoring onto it. This spec is binding:
deviations are REPORTED (numbered, with the executed blocker),
never improvised silently. Where PATHS-DESIGN under-specifies a
behavior, that is **a finding to bring back to the orchestrator,
not a silent fix** (the doc's own §5 rule).

**Explicitly NOT this unit**: the v2 profiles-as-programs
REPRESENTATION switch (#104). No persistence/schema changes of any
kind — the lowering targets what exists. The v2 design
conversation is queued AFTER this unit precisely so your
implementation and rework REPORT can inform it (§6).

## 0. Output discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes;
reports ≤150 lines. The 64k output limit kills agents that draft
whole files in one Write. Run every build/battery row as a
synchronous FOREGROUND Bash call, one at a time, reading each
result before the next; NEVER arm waiters, monitors, or background
chains for your own builds/tests.

## 1. The fence (two orchestrators share this repo)

- **PR-1 touches `crates/profile` only.** PR-2 touches
  `demos/tour` (and, if the U1 façade has merged by then, the one
  prelude line that surfaces the algebra). Nothing else.
- No CI edits; no `docs/M6-*`/`docs/M7-*`; no
  `crates/step-import`/`step-export`; no `scripts/`; no
  montage/render regeneration (tour SOURCE edits only).
- **Do not touch `SectionSegments`/loft/sweep sections** — U3's
  unit. The algebra targets `ProfileLoop`; extrude/revolve/fillet
  are this unit's consumers.
- No new document-schema fields, no `editor-core` changes.

## 2. PR-1 — the algebra (crates/profile)

Default placement: a new module in `crates/profile` (e.g.
`profile::path`), so the lowering shares the junction predicates
and `ProfileLoop` internals; a separate crate is a reported
deviation if you find a real coupling reason.

- **Representation exactly per PATHS-DESIGN §5**: ONE struct
  (`pos: Option<PosData>`, `ang: Option<f64>`) under type-level
  lattice markers `Tip<P, A>`; the position marker carries the
  plain-vs-directed flavor; fields private; binders are the only
  constructors; each binder written once, generic over the slot it
  does not touch; `.tangent()` exists only at
  `Tip<HasPos<WithIncoming>, NoAng>`.
- **Constructor inventory, core + sugar, exactly per PATHS-DESIGN
  §§2–4** — including the closure family (last-to-first junction
  gets its own check at `close()`; fillet-closure sugar) and the
  §6 decided points: mixed authoring OUT (a loop is algebra-
  authored or raw, never both), NO concatenation operator,
  declared cusps REFUSED typed (tabled to #131), PQ4 mid-side
  closure REFUSED. Generic over `Real` like the rest of
  `profile` unless the doc says otherwise.
- **Lowering**: to `ProfileLoop` segments + declared tangency
  flags. `LoopBuilder` remains the raw layer the lowering
  VERIFIES against — the existing junction predicates re-verify
  every declared flag at build (verified-never-trusted; nothing
  is trusted because the algebra produced it).
- **Exactness**: constructor-derived quantities (tangent
  directions, arc centers, fillet geometry) are closed forms per
  the doc — no iteration anywhere.

Tests (the unit's spine, not an afterthought):

- **Differential vs LoopBuilder**: for a representative family of
  loops (lines/arcs/fillets/tangent legs, closures of each kind),
  the algebra-lowered `ProfileLoop` is IDENTICAL — bit-level on
  coordinates the authoring determines exactly — to the
  hand-built `LoopBuilder` equivalent, and both build/validate
  identically.
- **Property tests (proptest)**: every authored point lies on the
  final path; lowered loops always pass the junction verifier
  (tangency-by-construction means the flags can never be caught
  lying); refusals are typed, never panics (`clippy::panic`
  denied).
- **Off-lattice unreachability**: compile-fail coverage for the
  illegal states (double director, `.tangent()` on a plain point,
  leading fillet, use-after-close). `compile_fail` doctests
  suffice; if you adopt `trybuild`, the ~2-week release-age
  dependency policy applies (memories/review-and-dependency-policy).
- Test-binary budget: at most two new `[[test]]` targets.

## 3. PR-2 — the demo corpus reworked

Sequenced AFTER U1's façade PR merges (orchestrator decision LB1):
merge `origin/main` first, then rework.

- Every tour scene whose extrude/revolve/fillet profiles the
  algebra can express moves to algebra authoring — wholesale per
  loop (mixed authoring is out). Scenes' loft/sweep sections stay
  untouched (U3).
- **Zero geometry diffs**: every scene pin, volume, ε row is
  byte-identical before/after. The rework changes how profiles
  are SAID, not what they are. Any changed number is a defect.
- Where a loop CANNOT be expressed (spline legs, cusps, mid-side
  seams — the doc's out-of-scope list), it stays raw with a
  one-line comment naming the gap; list these in the report.

## 4. Verification ladder (foreground, one row at a time)

1. `cargo build -p profile`, clippy, then `cargo test -p profile`.
2. Differential + property + compile-fail suites.
3. (PR-2) tour battery: all scenes, pins, ε rows — unchanged.
4. `scripts/test-fast.sh` locally for iteration; **hosted CI is
   the only gate**.

## 5. PR discipline

- Commit AND push after every coherent unit of work.
- **Merge `origin/main` immediately before opening each PR, and
  re-merge whenever main moves while it is open** (a CONFLICTING
  PR runs NO checks). After any push, confirm checks actually
  STARTED (`gh pr checks` shows rows).
- PR bodies carry the sanitized logical documentation: lattice →
  code mapping, constructor inventory vs the doc (any gaps),
  differential-test census, reported deviations (numbered).
- **NO Co-Authored-By trailer in lane commits** (A/B blinding);
  if a model mention lands in a PUSHED commit, STOP and report —
  never rewrite history yourself.
- Report to the orchestrator per your dispatch message; the
  orchestrator runs the review pass and merges. Do not self-merge.

## 6. The v2 evidence report (required, feeds the next design conversation)

Alongside the PR-2 report, a short section: for each scene
reworked, which constructor ARGUMENTS wanted to be expressions/
dimensions rather than literals (the parametric pressure points),
what the algebra programs look like at corpus scale, and any place
the v1 lowering felt like a wall rather than a floor. Observations
only — no schema proposals, no implementation.
