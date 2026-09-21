# LANE-0 — the `f64`-only offset-fit absence becomes an `Option` hook, out of the lane traits

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-21).** Binds
the implementer of unit LANE-0; deleted at merge per `docs/DOC-LEDGER.md`.
Read `docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/lane-0-offset-fit-hook.md`; the ruling is
`work/scalar/H5.md` §RATIFIED, ruling 3 (PR 2701).

## 0. The finding, and what the tree says

Three lane methods carry a different absence from the rest: `Probe`,
`Interval` and `Sym` — all `CertifiedBounds` — answer `None` from
`PropsQuadLane::recertify_approx` and `::approx_offset_surface`
(`crates/topo/src/props.rs`, the `Interval`/`Probe`/`Sym` impls around
`:1930-2090`) and from `PcurveFittedLane::remap_certificate`
(`crates/geom-brep/src/pcurve_cache.rs`, the impls around `:1690-1855`),
because `offset_fit` is written at `f64` (`crates/geom-core/src/offset_fit.rs`
`:205`, `:707`, `:1057`). That is "not derivable at this scalar", and it
shares a `None` with "a `Dual` may not certify". The consumers already
treat it as an error, never a skip: `tier3_local_checks_marked`
(`crates/topo/src/validate.rs` ~`:3551`, the call ~`:3609`) pushes
`ValidationError::ApproxLaneUnsupported` (~`:3618`); `mint_offset`
(`crates/topo/src/replace_face.rs` ~`:1293`, call ~`:1304`) returns
`ReplaceFaceError::ApproxLaneUnsupported` (`:1323`); `map_approx`
(`crates/topo/src/transform.rs`, find it) returns
`TransformError::ApproxLaneUnsupported` (`:155`). Re-locate every line at
your merge base.

Ruling 3 makes every certified sub-operation a plain function and the
mixed passes take their door as a parameter. This unit does the smallest
instance of that — the three `f64`-only methods — because they are the
one absence the no-trait tree cannot express with a bound (an
`ApproxSurface<T>` is representable at every scalar, so refusing by type
would be false to the tree), and because taking them out first lets
LANE-1 and LANE-4 delete the traits without carrying it.

## 1. What this unit delivers

- **The hook.** One type, `OffsetFitLane` (name it in `topo`'s
  vocabulary; `crates/topo/src/props.rs` beside `QuadHook`, or a module
  of its own if `props.rs` is the wrong home — say which and why),
  carrying the three operations with the same signatures the lane
  methods have today (`recertify`, `mint`, `remap`, or their current
  names). Its ONE constructor is the `f64` fit: the bodies the three
  `f64` impls hold now move into it verbatim.
- **The three passes** take `Option<&OffsetFitLane>` (or by value if it
  is `Copy`) in place of reading the method off `T`: `tier3_local_checks_marked`,
  `mint_offset`, `map_approx`. `None` takes exactly today's absence path
  (the three `ApproxLaneUnsupported` errors, same payloads, same
  messages). Callers: the seam arms supply `Some` where the scalar is
  `f64` (`AtRestPolicy for f64`, the `ShellLane` → seam arm for `f64`,
  `verbs/shell.rs` ~`:248-258`) and `None` elsewhere; `f64`-concrete
  public callers (`pncad`, `pncad-py`, the prelude — find them) supply
  `Some`. Read every caller up to the concrete instantiation and list
  them in the PR body with what each passes.
- **The traits shrink**: the three methods and their fifteen impls
  (three per trait method) are deleted; `PropsQuadLane` keeps
  `quad_cut_face` and `datum_lo`, `PcurveFittedLane` keeps its other
  four — LANE-1 and LANE-4 take the rest. The docs on the two traits say
  what they still carry, present tense, no history.
- **Nothing else moves.** `offset_fit` stays `f64` (its generic
  rewrite is PROPS' and is not this unit); `AtRestOutcome`,
  `ShellLane`, the census, `mint_pcurves` are LANE-2/3/4's. If the
  plumbing to reach a `Some` at an `f64` caller forces a signature
  change beyond the three passes and their direct callers, stop and
  say so in the PR body with the count — the class corrects from E to
  M there, not silently.

**What must not change:** every fit result, certificate and offset
surface at `f64`, bit for bit (the same bodies behind a hook); every
refusal at the other scalars, same variant, same payload, same
`Display` text; the k-lint predicate counts (no decision is added or
moved); no golden or digest.

## 2. Docs

The hook's doc says what the absence means (the fit is written at
`f64`; a `None` is "not derivable here", never "may not certify") and
points at ruling 3 by row id, one sentence. The three error variants'
docs say the hook is where `Some` comes from.

## 3. The pin

- A row per pass that `None` refuses with the same variant and payload
  as the merge base (today NO test body carries an `Approx` face at a
  refusing scalar — `grep ApproxLaneUnsupported crates/*/tests` finds
  only the `TransformError` twin, `verbs_offc_consumer.rs:956` — so the
  first two rows are new; build the smallest body with an `Approx` face
  and validate/offset/transform it at `Probe` or `Interval` with
  `None`, asserting the error).
- A row per pass that `Some(hook)` at `f64` reproduces the merge base's
  result by bits (`to_bits` on the certificate/offset payload), on the
  fixtures the existing `f64` rows use — cite those rows if they
  already pin it.
- D9 differential: `topo`, `geom-brep`, `sweep`, `editor-core`, `pncad`,
  `pncad-py` suites green unchanged.
- Red first: delete the `Some` at one `f64` seam arm and show the row
  that reds.

## 4. Sweep

The class: a lane method whose `None` means "not implemented at this
scalar" rather than "refused at this scalar". Read every remaining
method on the three kernel lane traits and editor-core's seven
(`Lane`, `MinClearanceLane`, `ShellLane`, `SectionScalar`, `AxisScalar`,
`SeedScalar`, `ChartCoherenceLane`) and say for each which absence it
is; the PR body carries the table. State the blind spot (an absence
spelled as an `Err` variant rather than a `None`).

## 5. Fence

This program claims no paths. This unit reaches TOPO's
`crates/topo/src/validate.rs`, SHELL's `crates/topo/src/{replace_face.rs,transform.rs}`,
TRIM's `crates/geom-brep/src/pcurve_cache.rs`, the unowned
`crates/topo/src/props.rs`, WIRE's `crates/editor-core/src/verbs/shell.rs`
(the seam arm), LIB's `crates/pncad*` where an `f64` caller supplies
the hook, and TINT/TCOST's tests. Announced by the orchestrator in
`work/scalar/log.md` and on the PR. Merge `origin/main` immediately
before opening the PR; `python3 scripts/work.py territory --base
origin/main` in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p topo -p geom-brep -p sweep -p editor-core -p pncad`
at default features and `-p topo -p geom-brep --features interval`;
the `pncad-py` suite as `.github/workflows/ci.yml` runs it if a Python
door changes; `cargo clippy --workspace --all-targets -- -D warnings`
at default AND `--all-features`; `cargo fmt --all --check` at the root
and in `demos/tour`, `demos/wild`, `benches`; `scripts/doc-gate.sh`.
Private `CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`; disk is shared.
Hosted CI is the verification of record; poll to conclusion in the
foreground and report the run id. Report ≤120 lines: the hook as
landed, the three passes' signatures, the caller table (who passes
`Some`), the absence table of §4, the pin rows and the red-first
evidence, deviations, findings filed outside the fence with row names,
the class as it turned out.
