# Design problem — a split plane that misses a concave edge by less than ε

Repo: `evgunter/cad` (a B-rep CAD kernel in Rust; the GUI is a thin client over
the API). Read `docs/prompts/designer.md` in full first; it binds you. Then
`CLAUDE.md` and the parts of `docs/DESIGN.md` relevant to what you find.

## What needs deciding

`split` cuts a solid by a plane. On the notched block (`NOTCHED` in
`crates/sweep/tests/m3_pr3_split.rs`, height 1; the notch's concave tip line is
at y = 1 exactly), cut by `plane_y(1 + δ)` (origin (0, 1 + δ, 0), normal +y) at
`Tol::witness()` (ε = 1e-9), measured on main (record in
`work/cleave/split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence.md`):

- δ = 0: split reads the tip ON and returns a half that touches itself at the
  tip (a pinch).
- δ ∈ {1e-13 … 1e-10}: split reads the tip ON and returns the same pinch half as
  δ = 0. Since PR 3856 (pinch copies share the tip's point) the
  pseudomanifold door passes that half at every δ in the range; before it, the
  door refused δ = 0 and δ > 0 alike, never telling them apart.
- δ = 5e-10: split refuses, but its text calls the input a kernel defect (an
  attachment-gate residual inside the ambiguity band).
- δ ≥ 1e-9: split refuses typed — "a vertex lies within tolerance of the split
  plane … move the split plane or the geometry".

The design contract says, in D1's tier 3′ (i), that near-coincidence never
silently becomes contact. It also has a tolerance policy (ε, the band, Q1's
trilean escalation) under which a margin below ε currently DECIDES ON. Both are
ratified text. On this input they pull different ways: a plane that misses the
tip by less than ε yields a body whose touching is, depending on how one reads
the contract, either an artefact of the band or exactly the δ = 0 answer.

What has to be decided: what `split` (and any other door that turns an
"on the cutting surface" verdict into topology) should answer when a vertex or
edge is within ε of the cutting surface but not provably on it — and in
particular when the ON reading would create contact (a pinch, a touching
between parts of one body) that a reading of "not on" would not. Include
whether δ = 0 and 0 < δ < ε are, or can be, told apart by the kernel at all;
whether the right answer differs between an ON verdict that only adds a vertex
or edge and one that creates contact; what the user sees in each case (an
answer, a typed refusal naming what, an answer carrying a declaration); where
the decision lives (split's ON test, the band policy, a gate after the cut);
and whether the framing above — a conflict between two ratified clauses — is
the right one, or one clause already decides it.

## Where to look

- The item and its measurement (path above); PR 3856's change to pinch copies
  (`git log --grep 3856`); the TQUERY row it came from,
  `work/tquery/split-halves-have-no-contact-records-so-no-pseudomanifold-self-check.md`
  if present (else `git log --all -- '*split-halves-have-no-contact-records*'`).
- `docs/DESIGN.md`: D1's validity tiers (tier 3′ and its (i)), the tolerance /
  band / Q1 trilean sections, and Ev's ruling in PR 3642 (split derives the
  tangent side) — search the doc and `git log --grep 3642`.
- Split's ON verdicts: `crates/topo/src/splitting/` (`classify.rs`, `mod.rs`,
  `finish.rs`, `rules.rs`), where `Band::linear(tol)` builds the band; the
  kernel-defect text's source (search for "There is no way through").
- Contact declarations and the pseudomanifold door: `crates/topo/src/census.rs`,
  `validate_pseudomanifold`, and how a Boolean's declared contacts are made and
  carried (`BooleanDeclarations`).

Deliver the report `docs/prompts/designer.md` §5 specifies (≤150 lines, the two
sections). Do not write code or commit anything; this is a read-and-reason
task. Work in a private scratch directory if you need one
(`~/.local/share/cad-work/designer-<your-label>/`), never the session
scratchpad. Do not build the workspace; reading is enough (if you must run one
test to check a premise, use your own CARGO_TARGET_DIR under that directory and
wrap it in `local-scripts/with-build-slot.sh --`).
