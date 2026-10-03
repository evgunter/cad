# Design problem — what `topo` reports when it has decided something but reports an Indeterminate

Repo: `evgunter/cad` (a B-rep CAD kernel in Rust; the GUI is a thin client over
the API). Read `docs/prompts/designer.md` in full first; it binds you. Then
`CLAUDE.md` and the parts of `docs/DESIGN.md` relevant to what you find
(Q1's trilean and escalation, D4's message policy, the funnel / escalation log).

## What needs deciding

The kernel's predicates answer through a funnel (`geom_core::k_stats` and its
doors) that decides a sign or escalates, and every escalation lands on a
frame's escalation log. In `topo`, about sixty sites build a
`geom_core::Indeterminate` BY HAND (a struct literal, or the helper
`crate::invalid_margin::invalid`, or closures in `boolean/mod.rs`) AFTER a
predicate has already returned a definite answer, usually with
`margin: MarginDiag::INVALID` ("NaN or a poisoned enclosure"). These never reach
any escalation log. A measurement lane classified every site on main:

- (a) about 12 sites are real indeterminacies that simply skipped the funnel;
- (b) about 28 report a DEFINITE contradiction — a declared contact the
  measurement refutes, a decided-nonzero offset where the declaration said
  coincident — dressed as an `INVALID` margin, which tells the user "the margin
  was poison" about a margin that was measured;
- (c) about 23 others: a sign that "cannot happen" (e.g. a negative of a
  nonnegative quantity — no door admits a two-valued `{Zero, Positive}`
  answer), two definite verdicts that disagree, an out-of-lane case, a broken
  internal invariant.

Some of these are user-visible today (refusal text saying "margin is invalid
(NaN …)" about a decided zero; an `INVALID` in a public error's `margin`
field). And at least six readers treat `is_invalid()` as meaning "decided
exactly zero" — so the fabricated value is already load-bearing.

What has to be decided: what each class should be, as a type and as what the
user sees — what a definite contradiction carries instead of a fabricated
margin (and whether it belongs on any log); what an "impossible sign" or a
disagreement between two definite verdicts is (an escalation, a kernel defect,
something else) and through which door; what happens to the readers that
depend on `INVALID` meaning zero; whether one rule covers all sixty sites or
the classes need different homes; and whether the framing — "route the mints
through the funnel" — is the right one at all.

## Where to look

- The measurement: `/root/.local/share/cad-work/designers-mints/mints-retake.md`
  (the per-site table, the user-visible measurements, an inventory of the
  payload types that exist today). Paths in it are relative to the checkout.
- The item: `work/cleave/topo-mints-indeterminates-outside-the-funnel.md`
  (its earlier sections were written before the re-take; the re-take wins on
  facts).
- The funnel: `crates/geom-core/src/` (`k_stats`, `predicate.rs`: `Decided`,
  `MarginDiag`, `Indeterminate`, the doors `decide_positive`, `decide_nonzero`,
  `decide_nonzero_reported`, `gate_measured`); PR 2928's escalation-channel
  unit, which retired the same shape in `geom-brep` (`git log --grep 2928`).
- Contradiction carriers in `topo`: `boolean/contact_verify.rs`,
  `boolean/carrier_eq.rs`, `boolean/plane_eq.rs`, `boolean/mod.rs`,
  `refusal_routes.rs`; editor-core's refusal stories
  (`crates/editor-core/src/eval/mod.rs`).

Deliver the report `docs/prompts/designer.md` §5 specifies (≤150 lines, the two
sections). Do not write code or commit anything; this is a read-and-reason
task. Work in a private scratch directory if you need one
(`~/.local/share/cad-work/designer-<your-label>/`), never the session
scratchpad. Do not build the workspace; reading is enough (if you must run one
test to check a premise, use your own CARGO_TARGET_DIR under that directory and
wrap it in `local-scripts/with-build-slot.sh --`).
