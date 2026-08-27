# PCURVE P-1a — the representation, the meter, the record (spec)

Orchestrator work order for PCURVE-PLAN item 1's first half.
Substrate: the P-1 exploration (2026-08-27; file:line evidence in
the lane report). Ratified ground is `docs/PCURVE-UNIFY-DESIGN.md`
(U2) — **not re-litigated here**.

**Scope: `geom-brep` ONLY, behind a thin shim.** Consumers, the
scaffold fence, and the test rewrites are P-1b. The split is the
substrate's, at the crate boundary, because three parts of P-1 are
independent design tasks (the Mapped/Seam meter rewrite, the
`Certificate` bit-diff measurement, and the fence) and the taxonomy
is far larger than the design estimated: 163 `EdgeGeometry::` sites
across 6 crates in `src/` plus 152 in 53 test files, 16
multi-variant match groups, and 22 deref sites broken by
`EdgeGeometry` being `Copy` where `Pcurve` is not.

## The five rulings (the substrate offered each with an alternative;
these are the ones that bind)

**D1 — the collapsed form's certified statement: ONE meter, plus the
seam predicates as an optional obligation.** `|C(t) − S(P(t))| ≤ ε`
(C4 verbatim) is the statement; the two half-plane/side predicates
are retained as an obligation on periodic charts rather than as a
second form. The alternative — keep `Seam` a peer variant — is
cheaper and **is rejected on Evan's own ratification**, which says
"Seam folds in as drafted"; keeping it a peer forfeits the headline
the migration exists for (one conventional form).

**D2 — the certificate bit diff: state it, measure it, never launder
it.** `Certificate.max_residual` MAY move at the conventional arms,
because the three forms do not measure the same thing today and the
iso residual's arithmetic order differs from `Pcurve::eval`'s. The
unit ships a **measured red/green row naming which fixtures move and
by how many ULPs**. Silently re-expressing the iso arm's
`v0 + (v1 - v0) * frac` is FORBIDDEN — if a value moves, the row
says so. Pinning bit-identity per lane is rejected: it re-imports
the per-class branching the migration exists to remove. **Separately
preserved and NOT traded against this**: D9 bit-replay of the mint
pass, and `adopt.rs`'s `bitwise_iso_match`, which quantifies over
CARRIER NURBS PAYLOAD bits and is untouched by this unit.

**D3 — the scaffold fence is TRANSIENCE** (Evan, 2026-08-27,
correcting the design's "pre-body" wording without reopening his Q2
choice). `MappedCurve` as a description is legal only through the
scaffolding door; tier 3 refuses it at rest. **The conversions this
implies — `ops.rs:1063,1078` and fillet's six strut sites — are
P-1b's**, not this unit's; P-1a provides the record and the door
they will convert to.

**D4 — where the conventional pcurve is minted**: at certification
time via `chart_pcurve` for analytic charts, and taken from the
caller for spline charts (every constructor already knows its iso).
`nurbs_iso_derive`'s conventional arms retire. The
caller-supplied-only alternative is rejected: it is a simpler
contract but moves every constructor in the same PR, which is the
sprawl this split exists to avoid.

**D5 — P-1a / P-1b, each its own unit with its own review.**

## What P-1a builds

1. **The enum reshape**: `EdgeGeometry`'s conventional variants
   collapse to (surface, `Pcurve`); the two intrinsic forms are
   untouched. Resolve the `Copy` loss deliberately — state the
   chosen shape (borrow, `Arc`, or index) and why, at the 22 deref
   sites' cost.
2. **`General`**: the curve-in-UV arm at the honest Fitted grade.
   `certify_fitted` already exists and is CALLERLESS, and its own
   docs name U2's `General` arm as its waiting consumer — this is
   the cheap piece, not the risky one.
3. **The unified meter + retained seam predicates** (D1), with the
   bit-diff row (D2).
4. **The authority record type**: the minimal shape that satisfies
   tier 3's prefer-intrinsic reads (`TransverseNotIntrinsic` /
   `TangentNotIntrinsic` today read MappedCurve's negative space).
   P-1a defines and stores it; P-1b switches the predicates onto it.
5. **The `chart_pcurve` conversion door** (D4).
6. **A thin shim** keeping the 6 consumer crates compiling
   unchanged, so P-1b is a separate reviewable diff rather than a
   big-bang.

## Acceptance

1. `geom-brep` carries ONE conventional description form; `General`
   certifies at the Fitted grade with its three outcomes honest.
2. The bit-diff row exists, is red-then-green, and NAMES the moved
   fixtures and their ULP deltas — or states plainly that nothing
   moved, measured rather than assumed.
3. D9 bit-replay of the mint pass holds; `bitwise_iso_match` is
   untouched (say so with evidence, do not merely assert it).
4. The shim keeps every consumer crate compiling with no behaviour
   change; the workspace is green with no test rewritten to
   accommodate the reshape (rewrites are P-1b's, and a rewrite here
   is a scope leak to report, not to perform).
5. ε-row three-outcome honesty on every new row; any new metered
   predicate name is an orchestrator ruling, not a silent mint.

## Process

Implementer arm: **block PCURVE-1 slot 1 = OPUS** (drawn 2026-08-27,
byte 251, mod 4 = 3 ⇒ fable at slot 4; slots 1–3 opus). Difficulty
pre-logged: **L**, task-class **STRUCTURAL** (the decided predicates
are reused; the meter rewrite is a representation change, and any
new numeric decision is a STOP-and-report).

Review: **protocol v6 — a CROSS-MODEL DUAL, R1/R2 model assignment
RANDOMIZED by a `/dev/urandom` byte drawn AT REVIEW DISPATCH**
(parity 0 = R1 opus + R2 fable, parity 1 = R1 fable + R2 opus), byte
and assignment recorded in the row; ordinal claimed on main at
review dispatch. Both reviewer briefs carry v6 item 5's **lane
isolation, READ side**: pushing is never delayed, but until its own
report is delivered a reviewer must not fetch, check out, or read
the other same-unit review lane's branches, scratchpads, or CI
artifacts, and any accidental glimpse is disclosed.

Standard brief lines: OUTPUT DISCIPLINE; the verbatim foreground
sentence AND its `setsid` exception for anything outliving a 600 s
call; lane-private publish paths; no `Co-Authored-By` in lane
commits; comments state the invariant; k-lint discipline;
merge-main + BUILD THE UNION; hosted CI is the only gate and the
PR states which ε/compile-mode points it actually drew.
