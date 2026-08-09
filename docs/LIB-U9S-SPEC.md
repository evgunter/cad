# LIB-U9S spec — Python bindings scaffold (binding)

Mandate: the schema-independent HALF of LIBRARY-DESIGN §L5 U9 —
the PyO3/maturin crate skeleton, the typed-quantity Python
classes, and Doc/evaluate bindings against the CURATED document
surface. The profile-AUTHORING surface (the PATHS algebra in
Python) is EXPLICITLY OUT: it ships only against the v2 program
representation (LQ4: "Python never ships the opaque-profile
intermediate state"), i.e. after SWITCH-E merges — a later unit
binds it. Design authorities: LIBRARY-DESIGN §L4 (the two-layer
type story: ty-static + runtime-at-boundary; typed quantities;
stubs CI-checked — the CI half deferred, see fence), §L3 (Python
speaks Doc/DocEdit/evaluate/persist, never an arena key — the
LB13-narrowed curated surface is now exactly this boundary,
enforced by test), D9 (bit-identical replay as a headline
property), the U9 backlog notes in LIB-LOG (serde_json::Value
exception; BooleanOp naming).

## 0. Discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes;
report ≤150 lines. Slot rules: `scripts/with-build-slot.sh --
cargo ...`; `--express SECS` for ≤10-min rows; long rows default
mutex, BLOCKING foreground waits (timeout 590000, re-issue;
setsid + foreground-poll past the cap); NEVER park. Cold clippy
both lanes + greps BEFORE opening. Commit AND push per chunk. NO
Co-Authored-By, no model names. Merge origin/main before opening;
confirm checks STARTED.

## 1. Fence

In scope: a NEW crate (`crates/pncad-py` or similar, reported) +
its member line; `.pyi` stubs beside it; local maturin builds.
OUT: CI (no wheel job, no stub-check job — both are follow-ups
recorded in the PR body; hosted CI must stay green with the new
member building as a plain rlib/cdylib without Python present —
verify `--workspace` builds don't require a Python toolchain, or
gate the crate behind a non-default feature and SAY so);
profile/PATHS bindings of any kind; editor-core/pncad source
changes (consume the curated surface as-is — if it is
insufficient, that is a FINDING, not an edit); persistence
schema; renders.

## 2. Deliverables

1. **The crate**: PyO3 + maturin, abi3 (per §L5 U9), f64 lane
   only. Dependency versions: the ~2-week release-age policy;
   list versions + release dates in the PR body.
2. **Typed quantities** (§L4): Python `Length`/`Angle`/`Count`
   mirroring crates/quantity — `25 * mm` constructs a Length;
   arithmetic mirrors the Rust infallible subset (same-dim
   add/sub, scalar scaling; dimension errors raise TYPED
   exceptions carrying the structured error, never strings);
   canonical meters/radians underneath.
3. **The document surface**: Doc construction, DocEdit
   application, evaluate, per-node typed results (ValuePayload
   variants as Python-visible typed values — bodies opaque
   handles with mass_properties/validate doors; the read-back
   doors from U5 where curated), persist save/load round-trip.
   Typed exceptions per §L4 (never strings).
4. **`.pyi` stubs** for everything, `ty`-clean (run ty if
   installable under the age policy; else report). The stub
   lattice for PATHS is NOT in scope (v2).
5. **The D9 doctest seed**: one Python test that builds a small
   doc, evaluates, and asserts an exact expected volume at full
   bit precision — the future cross-platform bit-replay pin's
   first form (single-platform for now; the claim stays scoped).
6. **A smoke example**: the one-shot user journey (§L3's "build
   a bracket, export STEP" spirit) as a Python script in the
   crate's examples — using a document, not kernel bypass.

## 3. Acceptance

- `maturin build` (or `develop`) succeeds locally; the Python
  test suite (pytest or unittest, reported) runs green under the
  slot wrapper.
- Hosted CI stays green WITHOUT Python (the fence's gating
  requirement — prove by the PR's own checks).
- The serde_json::Value backlog item: MEASURED disposition
  (does any bound door surface it? if yes, wrap or defer with a
  typed placeholder — reported, not improvised).
- Zero changes outside the new crate + member line.

## 4. PR discipline

One PR. Report ≤150 lines to
`~/.local/share/cad-work/lib-u9s-report.md`, per-phase figures.
Open, do NOT merge. Final message: PR number + report path only.
Genuine forks (the crate's feature-gating shape and the
ValuePayload exposure inventory are the likely spots): report,
smallest faithful reading, flag.
