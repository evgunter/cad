# M10-P — the profile-parameter lift implementation (unit spec)

**Status: BINDING at dispatch** (orchestrator-authored; the design
is `docs/PROFILE-LIFT-DESIGN.md` PP1–PP6, ratified #1151 WITH A
RECORDED HEDGE — Evan: "not totally sure… but proceed" — which
binds posture: additive machinery, cheap reversal, no rework of
the f64 path). Branch `m10/m10-p-profile-lift`. Sizing **M–L**.
Read `docs/prompts/implementer-discipline.md` and
PROFILE-LIFT-DESIGN in full first.

## Grounding (substrate facts, surveyed 2026-08-29)

- `profile::replay<T: ArcCarrierScalar>` (`path/program.rs:1978`,
  `ArcCarrierScalar = Decide + Bounds`) is ALREADY generic and
  type-checks at `Interval`/`Dual64`; **no test instantiates it
  off f64** — the generic path is latent.
- The elaboration ladder: `program.resolve(&param_env::<f64>())`
  (`eval/mod.rs:1285`) → `wire::prepare_profile` (replay + f64
  validate + `anchor::derive_naming`, `wire.rs:434-459`) →
  `wire_profile`'s `anchor::embed_profile::<T>` per-coordinate
  `from_f64` lift (`wire.rs:464-483`, `anchor.rs:227-257`). The
  sweep/loft duplicate ladder is `profile_section`
  (`wire.rs:1487-1540`) — in scope, do not miss.
- Discrete decisions currently `pub(crate)` and discarded: the S8
  fillet-candidate pick (`fillet_select::nearest_joint` — the one
  `Bounds` read; its docs state lanes may legally pick different
  pockets), `ArcFilletCandidate`'s `fit_in`/`fit_out` signs, the
  corner-existence gates. `canonicalize_loop`'s `lex_min` decides
  under an exact ulp band — total at f64, indeterminate at
  `Interval` on every input.
- `Profile::validate` already runs at `T`, including declared-
  tangency re-verification. `derive_naming` is f64 bit-matching;
  its output (`LoopAnchor`) is purely structural.
- Content key: profile arm feeds the f64-resolved step stream
  (`feed_step`, `eval/mod.rs:1721`); `ContentBits for Dual` exists
  since M10-DI (both channels).

## Scope

1. **The structure record (PP2)** — additive `profile` API: replay
   (and the fillet resolution inside it) emits, per loop, the
   canonical rotation + reversal, `LoopRole`, per-segment kinds,
   the chosen fillet-candidate index and fit signs per fillet
   step, the corner-gate outcomes, and the tangent-joint set.
   Derived value; never persisted; content-keyed structurally.
2. **Guided replay (PP1)** — a replay mode taking the record:
   every consumed decision's deciding predicate re-runs at `T`;
   agree → proceed; indeterminate → typed abort (a new typed
   refusal naming the decision — the E6 driver's bisect cue);
   definite-other → typed structure-flip refusal naming the
   flipped decision. No lane ever selects structure.
3. **Pinned canonicalization + naming (PP3/PP4)** — guided
   validation consumes the f64 canonical permutation, verifying
   value-channel consistency, never re-running `lex_min`/
   orientation decides; naming taken verbatim from the f64 pass.
4. **The editor-core seams (PP5)** — pass 2 wiring: resolve the
   program at `ParamEnv<T>`, guided replay + validate at `T`, in
   BOTH ladders (`wire_profile` and `profile_section`); `T`-valued
   geometric feeds enter the content key through `ContentBits`
   (key-format tag bump); the f64 stream stays in the key.
   Activation: pass 2 runs when the evaluation requests it (a
   typed evaluation option consumed by the analysis lanes;
   default OFF — the f64 build path must not change).
5. **First commit = the latent-generic instantiation tests**:
   plain `replay::<Interval>` and `replay::<Dual64>` rows on
   representative programs, BEFORE any new machinery, so latent
   breaks in the generic path surface separately (the design's
   own sequencing requirement).
6. **The bit-identity fence (PP6)** — differential pins: guided
   replay at `T = f64` reproduces plain replay bitwise on every
   corpus profile; pass 2 OFF reproduces today's evaluation
   bit-identically (merge-base differential is the review's
   signal); the `scalar_channels` skeleton comparison extends to
   the guided path.

**Out of scope**: Expr-izing the sketch plane (VQ8); any naming
change; persistence; the driver (M10-3 consumes this); seeding
surfaces (M10-4).

## Review claims to falsify

1. Pass 2 OFF: evaluation bit-identical to merge base at f64,
   Interval, Probe (differential — unique signal).
2. Guided replay at f64 ≡ plain replay bitwise, whole corpus.
3. No structure selection at `T` anywhere: plant a hairline
   fillet lens (the fillet_select different-pockets case) and
   show guided mode verifies-or-aborts, never re-picks.
4. Indeterminate re-verification aborts TYPED at `Interval` on a
   wide-parameter box (construct one); nothing silently keeps the
   nominal structure.
5. The canonical permutation is pinned: an `Interval` guided
   validate never runs `lex_min` decides (structurally absent,
   not accidentally agreeing).
6. `profile_section` (sweep/loft) has pass-2 parity with
   `wire_profile` — the duplicate ladder did not fork.
7. e2e: drive a parameterized profile document through the public
   doors at `Interval` with a genuinely wide parameter and report
   what certifies, what aborts, and the friction.

## Acceptance

Hosted CI green on the unit's own head (the interval lane is this
unit's axis — state the drawn point; if the draw misses interval
AND the diff's filenames do not force it, run the interval
editor-core+profile suites locally before merge per the standing
calculus). PR body carries the two-ladder parity statement and the
structure-record field census.
