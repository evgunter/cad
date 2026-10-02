# SYM-15 — the chain's wedge margin poisons: name what goes to NaN, and fix it or say it (spec)

**Program:** SYM (`work/sym/plan.md`). **Item:**
`work/sym/a-chain-of-two-or-more-joints-poisons-its-transversality-margin.md`
(P1; Ev asked for the chain on #3073). **Unit:** `work/sym/SYM-15.md`.
**Branch:** `sym/15-wedge-poison`, cut from `main` (the tier is on `main`
since #2468). **Implementer:** Opus. **Review tier:** single FULL review
(§Review).

**Read first, in full:**
- `docs/prompts/implementer-discipline.md`;
- the item whole, and `memories/refusal-text-is-not-cause.md`;
- `demos/tour/src/chain.rs` and `demos/tour/src/chaintol.rs`: the chain,
  its `CERTIFIABLE_FRACTION`, `PIN_RADIUS` and
  `CERTIFIED_TIP_OVER_PIN_RADIUS`, and the pin
  `the_wall_is_the_wedge_not_the_arm`;
- `crates/geom-brep/src/dihedral.rs`: the `dihedral_wedge` predicate
  (`decide_reported("dihedral_wedge", …)`) and the transversality arm,
  and every input its margin is built from. The item cites
  `crates/topo/src/dihedral.rs`, but the code has since moved;
- `crates/geom-brep/src/certify.rs`, the lane that re-certifies a
  mapped edge (follow the call from the `transform` op's "failed
  re-certification" to the predicate);
- `work/sym/a-chain-of-three-joints-straddles-dihedral-arm.md`, the row
  ordered behind this one.

## The claim

At two, three and four links, the first refusal just above the chain's
certifiable box is the same at every link count: `dihedral_wedge`
indeterminate, "margin is invalid (NaN or a poisoned enclosure)", on
`EdgeKey(1v1)` at sample 4. It is measured at 1.02× and 1.10× of each
link count's fraction, at the default ε and at `1e-6`.

A poisoned margin says some upstream quantity went to poison, and the
refusal names only the branch it could not take.

Two things are not known:
- **What goes to NaN.** The item's hypothesis, unconfirmed, is that the
  pin's cylinder gradient straddles zero once the positional box exceeds
  the pin radius.
- **Why the certifiable box moves with ε** (`1.110e-1` at the default,
  `1.083e-1` at `1e-6`), when the arm's straddle does not move at all.

## Phase 1 — find where the poison is minted

1. **Read the margin's inputs at the wall.** Use the two-link chain at
   1.02× its certifiable fraction, default ε, `EdgeKey(1v1)`, sample 4.
   For every quantity the `dihedral_wedge` margin is built from:
   - give its enclosure;
   - say which is the first to go NaN or poisoned;
   - trace it back to the operation that produced the poison, with
     `file:line`.

   Say whether that operation is in the tier (`geom_core::sym*`), in
   `geom-brep`'s dihedral predicate or certification, or in the interval
   arithmetic it calls.
2. **The ε dependence.** Name what upstream of the poison depends on ε,
   so that the certifiable box moves with it. Show it with the box's
   edge taken at a third ε.
3. **The classification.** Is this a real defect (a poison where a
   definite or straddling enclosure is due), or a legitimate empty or
   undefined enclosure (for example a normalisation of a gradient whose
   box contains zero)? Give one minimal row reproducing it outside the
   chain.

**Stop rules.**
- **If the poison is minted outside the tier** (in `geom-brep`'s
  dihedral predicate or certification), go on only when the fix is local to the minting
  operation and changes no other predicate's verdict. Otherwise stop
  after Phase 1 and report: the row re-homes, as its own Home says.
- **If the poison is legitimate,** Phase 2 is only the refusal saying so
  (the cause, not the branch), plus the pins.

## Phase 2 — the answer Phase 1 justifies

- **A real defect: fix it at the minting operation.**
  - Re-take `chain::CERTIFIABLE_FRACTION` at two, three and four links,
    at the default ε and at `1e-6`.
  - Re-baseline every pin that moves, and say each, in particular
    `the_wall_is_the_wedge_not_the_arm` and
    `CERTIFIED_TIP_OVER_PIN_RADIUS`.
  - Name the new first refusal at the new wall; it may be
    `dihedral_arm`'s straddle, the row ordered behind this one.
  - Ev's standing words: "never skip out on a change that would make
    the code better because it would require rebaselining".
- **A legitimate enclosure:** the refusal names what is undefined and
  why, and its row pins that text.
- **Rows:**
  - the minimal shape from Phase 1;
  - a row that reds if the poison comes back.

## Scope

- **Files:**
  - the minting operation's file;
  - `crates/geom-brep/src/dihedral.rs` or `certify.rs` if the cause or
    the message lives there;
  - `demos/tour/src/chain.rs` and `chaintol.rs` for the pins;
  - tests; the unit and item files.
- **Territory:** `crates/geom-brep` and the demos are other programs'
  ground.
  Run `python3 scripts/work.py territory --base origin/main` and
  announce the seams in the PR body.
- **Not in scope:**
  - the arm's straddle (the next row);
  - the chain's construction;
  - any tolerance's value.

## Review

**Single FULL review**, recorded in `work/sym/log.md` at spec time. It
is a defect at one minting site, settled by execution: the margin's
inputs, the minimal row and the re-taken fractions. Claims to falsify:
1. the minting site named;
2. the ε dependence;
3. the classification (defect or legitimate);
4. the re-baselines;
5. plus `docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`.
- At merge the orchestrator closes the unit, deletes this spec (with a
  note under `docs/doc-ledger/`), and closes the item or re-homes it with
  what is left.
