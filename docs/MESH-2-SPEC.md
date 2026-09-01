# MESH-2 — issue 555: sub-floor engineered zeros refuse an ordinary annular cap

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **S/M**, recorded numeric M). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 555 is the primary specification; issue 284 and the R1 review
of PR 301 are its cited history.

## Situation

`crates/mesh/src/planar.rs` banks a hole its module docs state
(near lines 51–100 at opening): with off-plane noise ν, the far
point's engineered exact-zero v-coordinate has float residue ~ν²,
which for ν ≲ 4e-22 is nonzero **sub-floor** relative to spade's
`MIN_ALLOWED_VALUE` = 2⁻¹⁴², and the face refuses
`TessellateError::Triangulation` — at every δ, so no caller can turn
a tolerance to escape. The Klein bottle's inner-tube top rim (an
ordinary planar slit annulus, no translator involved, body passes
tiers 1–3) hits it, and it is a roundoff lottery: sweeping the
bulb's flare half-angle × bottom-rim radius — parameters that do not
touch the cap — refuses at (30°, 0.85) and (34°, 1.00) and nowhere
else. The pin at `demos/tour/src/klein.rs` (wall 7, the
`Triangulation` match near line 1024) is CI-gated since the tour
suite was armed, and carries its own retire instruction.

`mitigate_underflow` (spade's own tool for exactly this) appears
nowhere in `crates/`.

## Deliverables

1. **Red-first**: a committed row reproducing the refusal on the
   Klein inner-tube rim shape (a minimal in-crate fixture is fine if
   it reproduces the sub-floor signature; the demo pin is the
   e2e witness), plus the parameter-lottery demonstration in the PR
   body (the two refusing lattice points vs their neighbors).
2. **The fix, sited deliberately**: the issue's own analysis prefers
   the projection site — where "this coordinate is an engineered
   zero" is known — over a blanket `mitigate_underflow` filter over
   every chart coordinate. Decide by reading both options against
   `planar.rs`'s no-value-snapping doctrine, and write the defense
   IN the module prose: a structural zero snapped to zero is
   refusing to invent a nonzero the construction never had — the
   doctrine's own argument, not an exception to it. If you find the
   blanket filter is genuinely better, that is a spec deviation to
   argue, not a silent choice.
3. **The Klein pin's retire instruction honored**: wall 7's entry
   narrative updates to banked-case-closed (the retire instruction at
   the pin says check-and-say, never silently delete); the demo
   suite goes green on the previously-refusing lattice points.
4. **ε-three-outcome honesty** on new rows; the fix touches no
   tolerance (sub-floor is an absolute-float property, not an ε
   one) — say so in the PR as a checked claim and record which
   lane/ε the gate drew (the issue-1356 discipline).
5. **Class sweep** (discipline §5): other engineered-zero
   projections in `planar.rs`/`trimmed.rs` that could go sub-floor;
   hit list with dispositions, blind spots stated.

## Acceptance

- The red-first row red on the old code with the
  `Triangulation`/sub-floor signature, green under the fix.
- The Klein wall-7 pin updated per its instruction and the tour
  suite green (it is a CI gate now — `cd demos/tour && cargo test
  --release` locally before pushing; wrap in the build slot).
- No committed pin/count/render moves other than the wall-7
  narrative and any count the fix legitimately changes on the
  previously-refusing bodies — decide correctness per discipline §3
  and say what moved and why.

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: "issue 555" spelled out; the orchestrator closes
  it after merge.
- Scope fence: `crates/mesh/src/planar.rs` (and `trimmed.rs` only if
  the class sweep finds an in-scope instance there), mesh's test
  suites, `demos/tour/src/klein.rs`'s wall-7 entry per its retire
  instruction. NOT: `walk.rs` (contended by later units), sizing,
  `docs/MODEL-AB-LOG.md`, `docs/S-MESH-*.md`, no SMELL table edits.
- Any refusal retired is classified against the D2 addendum in the
  PR body (this unit retires one: valid input, previously refused —
  a row-2 capability landing).
