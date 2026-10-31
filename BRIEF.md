# Confirmation brief: SECT PR 6153 fix pass (the one-pass check)

You are a single confirmation reviewer on new reasoning a fix pass added.
FULL on the delta. Head `cbf4d5b4328b29bfbe7fb864faf99db4dc68681f` of https://github.com/evgunter/cad/pull/6153
(branch `sect/torus-rows-at-1e-6`): `git fetch origin sect/torus-rows-at-1e-6 && git checkout --detach cbf4d5b4328b29bfbe7fb864faf99db4dc68681f`.
The previous review was on `96368e32`. Read the PR body (its fix-pass section)
via the GitHub MCP tools; not its comments or reviews. Post nothing, push nothing.

**Read first:** `docs/prompts/reviewer-style-lane.md`; `CLAUDE.md`; C2 (limb 3)
in `crates/geom-brep/README.md`; `crates/geom-brep/src/ssi/one_arc.rs` (what
limb 3 proves). Builds: `CARGO_TARGET_DIR` outside the checkout,
`CARGO_INCREMENTAL=0`, opt-level 2 for marching rows, foreground, anything past
600 s `setsid`-detached and polled, nothing left running. **Budget: about 90 minutes.**

## What to falsify

1. **The degree check** (`pair_section/winds.rs`, `WindsRefusal::Unordered`,
   `once_round`, `gated`, `tube_chain`'s per-box graph axis). The argument: the
   carrier's turns are m × the component's; nonzero carrier turns ⇒ essential for
   any m; a tube hull clearing a centre ⇒ null about that channel; any other
   null reading needs m ≠ 0, shown by `once_round` (≥ 4 runs of consecutive boxes
   whose non-neighbouring runs are certainly apart) or `gated` (one box whose two
   neighbour meets lie on opposite sides of a slice along limb 3's graph axis, a
   slice no other box reaches). A wrong guard here reads an essential component
   as null **silently**. Is each guard a proof that the component's walk has
   degree ±1 (or at least m ≠ 0)? Construct a pose (or a synthetic chain) where a
   guard passes and m = 0 or the component is essential with zero carrier turns.
   Are "certainly apart" and "no other box reaches" decided by certified
   readings, never f64 comparisons?
2. **The refusal handling** (`farthest`, `winds::clears`, the box meet/hull testing
   `is_certified`; parts from `Box3::split`; `reach` ignoring a refused box) and
   the tighter `reach` (least `farthest(box, witness)` over boxes whose span holds
   the witness): sound?
3. **Rows:** the new fast and unit rows go red when they should (mutate a guard);
   the oracle fuzz row reads few components (28/24/16 comparisons) — is that
   enough to say anything, and can it be made to read more?
4. Run the slow rows the PR edits at the default ε, 1e-6 and 1e-12 (opt-level 2,
   `--run-ignored all`): the PR reports 81/85 at each, the 4 failing being GREEN's
   open P0 with identical stops.

## Report

Final message, ≤100 lines: verdict; findings MAJOR/MINOR/NOTE with `file:line`,
evidence, executed or by inspection; tests worth keeping as code.
