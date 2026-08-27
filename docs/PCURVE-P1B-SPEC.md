# PCURVE P-1b — the consumers, the fence, the deletions (spec)

Orchestrator work order for PCURVE-PLAN item 1's second half.
P-1a (#1073 at `9fa321d4`) built the representation, the meter and
the authority record in `geom-brep` behind a thin shim. **P-1b spends
the shim.** Ratified ground is `docs/PCURVE-UNIFY-DESIGN.md` (U2) —
not re-litigated; note its fence criterion is **TRANSIENCE**, as
corrected 2026-08-27.

## What P-1b does

1. **Move the consumers onto `EdgeDescription`.** The six crates
   speak `EdgeGeometry` today because P-1a kept it as the shim
   vocabulary. Retire it: 163 `EdgeGeometry::` sites in `src/` and
   152 across 53 test files, 16 multi-variant match groups. **Pay the
   22 deref sites** — P-1a resolved the `Copy` loss by BORROW
   specifically so this unit pays them against a type it can borrow
   rather than one it must `Arc` or index.
2. **Build the transience fence** (U2's Q2 as corrected). `MappedCurve`
   as a description is legal ONLY through the scaffolding door;
   **tier 3 refuses it at rest.** The sites that must convert:
   `boolean/ops.rs:1063` and `:1078`, and fillet's six strut sites.
   "Pre-body" was measured NOT to fence it — do not reintroduce that
   wording.
3. **Switch tier 3's prefer-intrinsic predicates onto
   `EdgeAuthority`.** `TransverseNotIntrinsic` and
   `TangentNotIntrinsic` currently read `MappedCurve`'s negative
   space; P-1a stored `EdgeAuthority::{Derived, Declared}` with
   `is_declared()` for exactly this. The read moves; the verdicts must
   not.
4. **Retire what the migration makes dead**: `nurbs_iso_derive`'s
   conventional arms (P-1a's mint inverted the dependency);
   `replace_face.rs:1249`'s "a v-row is not an `IsoCurve`" refusal
   (free with the representation change, unclaimed by any plan); the
   shim `EdgeGeometry` field on `EdgeCurve`; and the two `CertCheck`
   shim aliases (`IsoResidual` / `SeamSurface` → `ChartResidual`).
5. **The test rewrites P-1a was forbidden.** They were a scope leak
   there and are the deliverable here — including
   `sweep/tests/offc_r1_probes.rs`, which asserts `IsoResidual` on a
   refusal and pins the alias in place.
6. **Close P-1a's one named deferral**: the interval checks still
   report `sample: 0` because `step-import`'s `tier_gate.rs:286` pins
   their `Display` string. Moving them is a consumer test rewrite —
   i.e. this unit's. Use `NOT_A_SAMPLE`, which P-1a already renders as
   words rather than `4294967295`.

## Binding constraints

- **The verdicts do not move.** This unit is a representation change
  at the consumers, not a behaviour change. Any row whose outcome
  changes is a finding to REPORT, not a re-baseline to perform — with
  one exception, stated: if retiring a conventional arm legitimately
  retires a refusal (item 4), that row executes its own retirement
  text and says so.
- **`bitwise_iso_match` and D9 bit-replay** stay untouched, as in
  P-1a. Demonstrate, do not assert.
- **No new metered predicate.** P-1a needed none and neither should
  this; a new name is an orchestrator ruling.
- **The seam quantity question is CLOSED** (P-1a: `|C − S(P)|` is the
  right quantity, re-baselined deliberately, price pinned). Do not
  reopen it. The foot-point mint that would erase the `sec α` excess
  is a NAMED FOLLOW-UP, deliberately out of scope.

## Acceptance

1. `EdgeGeometry` is gone; one conventional description form reaches
   every consumer; the 22 deref sites are paid.
2. The fence holds: a `MappedCurve` description at rest is refused by
   tier 3, with a red-then-green row proving it, and the scaffolding
   door still passes.
3. Tier 3's two prefer-intrinsic predicates read `EdgeAuthority` and
   their verdicts are unchanged — pinned by the existing rows, not
   new ones.
4. Every retirement in item 4 executes its own text.
5. ε-row three-outcome honesty on anything new; hosted CI green; the
   PR states which ε/compile-mode points it actually DREW. **Note that
   P-1a's six-anchor bits row has only ever run in an interval-lane
   CI draw** — if this unit's heads draw default, say so, since that
   closes a gap P-1a flagged honestly rather than papered over.

## Process

Implementer arm: **block PCURVE-1 slot 3 = OPUS** (block byte 251,
mod 4 = 3 ⇒ fable at slot 4; slots 1–2 consumed by P-1a and the
census gap-2 unit). Difficulty pre-logged **L**, task-class
**STRUCTURAL**.

Review: **protocol v6 cross-model dual**, R1/R2 randomized by a
`/dev/urandom` byte drawn AT REVIEW DISPATCH, recorded in the row.
Ordinal from PCURVE's band (200–299). **Both reviewers must push
their probes to a named branch** (`pcurve/p1b-r1-probes` /
`-r2-probes`) so adoption is authorship-preserving — P-1a lost R1's
probes as files because the brief only said "push early and often".
Both briefs carry v6 item 5's lane-isolation READ rule, and the
foreground-polling rule stated as *you issue the call and read the
result* — two reviewers parked on watchers during P-1a.

**State-sync per the 2026-08-27 shape**: this unit's ledger row and
log entries ride its own PR as a final commit, added only AFTER both
reports are delivered (the row names the arm), merged without a fresh
CI run since they are docs-only on an already-green head.
