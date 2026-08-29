# The profile-parameter lift (M10-P): guided replay at the lane scalar

**Status: DRAFT — design conversation, awaiting Evan's sign-off.**
The design pass the M10 plan's post-ratification amendment added:
profile GEOMETRY becomes a function of the parameters at the lane
scalar, while structure selection stays f64-once. This elaborates
the asymmetry `docs/PROFILES-V2-DESIGN.md` recorded and reserved
("deserves Evan's explicit eyes, because under v2 the SAME document
parameter can feed both kinds of slot") — M10's E4/E6 are where it
stops being hypothetical: today a `Dual` seed on a profile
dimension propagates no tangent, and an interval profile parameter
does not widen the E6 leaf replay. Decisions PP1–PP6. Substrate
facts (surveyed 2026-08-29) cited inline; the implementing unit
dispatches after ratification.

## What the substrate already gives us

`profile::replay<T: Decide + Bounds>` is ALREADY generic and
type-checks at `Interval` and `Dual64` today (unexercised — no
caller instantiates it off f64). There is **no f64-only geometric
formula** in the replay path: the tangent-fit solves, arc-carrier
boundaries, bulge computations and the arrival/incoming family are
all scalar-generic. `Profile::validate` already runs at `T`,
including the declared-tangency re-verification (verified, never
trusted). What is genuinely f64-shaped: the discrete decisions
INSIDE replay (the S8 fillet-candidate pick — whose own docs state
that in a hairline-asymmetric lens *lanes may legally pick
different pockets*, the sharpest argument for f64-once selection —
the `fit_in`/`fit_out` signs, the corner-existence gates) and the
canonicalization step (`lex_min` under an exact ulp-wide band:
total at f64, essentially always indeterminate at `Interval`).
Nothing exposes those decisions today (`pub(crate)`, discarded
inside `arc_fillet::resolve`); PROFILES-V2's "elaboration decides
leg parameters, never topology" has no representation in code.

## PP1 — Guided replay: the f64 elaboration is the witness; the
## lane pass consumes its decisions and re-verifies every one

**Decision**: profile evaluation becomes two passes with one
structure. Pass 1 (unchanged): resolve + replay + validate + anchor
at f64 — C6's structure selection, now additionally EMITTING its
decisions as a structure record (PP2). Pass 2 (new, analysis
lanes): resolve the program's Exprs at `ParamEnv<T>` and replay at
`T` in **guided mode** — every discrete decision (candidate pick,
fit signs, corner gates, joint set) is CONSUMED from the record
instead of re-decided, and **re-verified at `T`**: the predicate
that decided it at f64 re-runs at the lane scalar; agreement
proceeds, indeterminate aborts typed (the E6 driver's cue to
bisect the leaf), definite-disagreement refuses typed (the
`FlipCrossing` shape — the leaf provably leaves the nominal's
structure). This is the W-contract posture one level down: the f64
elaboration is the witness, the lane pass is verification against
it, and no lane ever selects structure (fillet_select's
different-pockets hazard forecloses re-deciding by construction,
not by luck).

- Preserved verbatim: C6 (structure once, identically for every
  lane), the junction checks re-running under every binding
  (PROFILES-V2 consequence 2 — now at `T` too), verified-never-
  trusted flags (consequence 3 — already at `T`).
- Foreclosed: per-lane structure selection; any silent keep-the-
  nominal-structure when the lane disagrees.

## PP2 — The structure record: additive profile API, derived,
## never persisted

**Decision**: `replay` (and the fillet resolution inside it) gains
an additive output — per loop: the canonical rotation + reversal,
`LoopRole`, per-segment kinds, the chosen fillet candidate index
and fit signs per fillet step, the corner-gate outcomes, and the
tangent-joint set. A plain derived value (D3: rebuilt on demand,
content-keyed with everything else), new API in `profile` since
the data is currently `pub(crate)` and discarded. The f64 pass
populates it; guided replay consumes it; nothing persists it.

## PP3 — Canonicalization is pinned, never re-decided at `T`

**Decision**: pass 2's validation consumes the f64 canonical
permutation (rotation + orientation flip) from the record and
verifies value-channel consistency, rather than re-running
`lex_min`/orientation decides — which are total at f64 by the
exact band's design and would be indeterminate at `Interval` on
every input. The incidental truth that lanes agree today (because
`embed` lifts exact bits) becomes a stated obligation with a
mechanism.

## PP4 — Naming stays f64; `T` geometry changes no name

**Decision**: `derive_naming` (f64 bit-matching against program
order) runs on pass 1 exactly as today; names are program-
structural indices; the lane pass takes the naming verbatim. The
survey's conclusion made normative: T-valued geometry changes no
name PROVIDED the canonical permutation is pinned (PP3) — the two
decisions are one commitment.

## PP5 — Content keys: the f64 stream stays the identity; lane
## geometry feeds through `ContentBits`

**Decision**: the resolved f64 program stream remains in the key
(structure identity, lane-independent, as today); when pass 2
runs, the `T`-resolved geometric feeds enter through
`ContentBits::feed` like every other slot value — composing with
DUAL-DESIGN DL2 (both channels fed, the seed rides the tangent
bits), so seeded profile passes get the same sound memo story as
magnitude slots. Key-format tag bumps once (keys never persist).

## PP6 — Scope and the bit-identity fence

**Decision**: pass 2 exists for the analysis lanes (E6 leaf replay
at `Interval` with interval-valued parameters; E4 seeding at
`Dual`). The f64 build path is UNCHANGED BIT-FOR-BIT — pass 1 is
today's pipeline; a differential pin asserts guided replay at
`T = f64` reproduces plain replay bitwise (D9), and the existing
`scalar_channels` skeleton comparison extends to the guided path.
Sweep/loft's duplicate profile ladder (`profile_section`) is in
scope — the survey's named do-not-miss. NOT in scope: Expr-izing
the sketch plane (VQ8 stays deferred), any naming change, any
persistence change, the sketch solver (plan Q1 stands).

## Sizing and sequencing

One implementation unit after ratification, M–L: additive profile
API (the record + guided mode), ~8 editor-core seams (resolve at
`ParamEnv<T>`, the two wire ladders, content-key feeds), the
differential pins. Unexercised-generic risk is real (no test
instantiates `replay` off f64 today) — the unit's first commit
should be the `Interval`/`Dual64` instantiation tests of plain
replay, before guided mode exists, so the generic path's latent
breaks surface separately from the new machinery. M10-3 and
M10-4 consume this unit for profile-driven parameters; their
magnitude-parameter valves stand meanwhile.
