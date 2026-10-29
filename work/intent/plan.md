# INTENT — build D10 (plan)

D10 (`docs/DESIGN.md`) is the target. The program closes
`d10-one-way-to-say-intent-is-unbuilt` last, which releases the
refactor hold every other program carries.

## Stages

Each stage leaves the tree building and every suite green: no stage
lands half of a representation change.

1. **Variables** (this program's opening slate). The representation
   (`variables-are-identities-with-labels`, designers first), then the
   build (`no-dimensioned-literal-in-a-slot`, which also closes
   `equal-literals-lower-to-one-identity-token`), derived parameters
   (`parameters-defined-by-formulas`) and the GUI's minting and offer
   (`typing-a-value-mints-or-offers-a-variable`).
2. **Operations and one dependency.** Nodes read and define variables;
   consuming and reading edges, name references and `Measure` refs
   become reads; a `Select` operation defines `Face`/`Edge` variables
   and owns the N5 ladder; the product becomes an explicit list
   (A10's sink rule retires).
3. **Spaces and placement.** Frames as variables, with frames and
   directions as their own variable kinds (a datum's origin and axes are
   not independent reals; stage 1 spells them as `Scalar` variables in
   the interim). Whether any value is varied in analysis stays the
   user's choice, by its tolerance (and, if a situation needs it, a
   per-study selection), never by its kind; a placement is the
   bundle of mates pinning one copy; the world frame and export;
   gauges, offsets, `Transform`-as-placement and absolute datums
   retire; the per-space computing frame; overconstraint refuses
   (`mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`
   closes here, with A11 (4)).
4. **The coincidence door.** Canonical carrier forms per kind; every
   coincidence decided from values recorded at one door; the
   `unproven-coincidence` lint; booleans glue on Zero; declared pairs,
   `ContactClass` on mates, the undeclared refusals, the axis
   declaration channel and stored tangent-joint flags retire.
5. **Assertions and the at-rest lints.** `Assert` with `=`; the
   quieting rule; the at-rest census as a check; interference as its
   own finding.
6. **Tangency constructions.** A surface's trace in a sketch plane and
   continuation tangent to it; coaxiality through one axis variable.

**State (2026-10-29):**
- **Stages 1–5 are built.** Stage 3 closed 2026-10-29 with G (#5954). Stage 4 I and stage 5 C were built and closed under ATREST.
- **Stage 6 is sliced** in `docs/INTENT-STAGE6-SPEC.md` and ruled on #6004 and #6006. Its umbrella is `intent-stage6-is-built`.
  - Merged: A (#6023), B (#6041).
  - In flight: C (#6043), then D.
  - E is the viewer's tool.
- **The exit walk ran on 2026-10-29** (#5997). What remains is listed on `d10-one-way-to-say-intent-is-unbuilt`.

## Successors (2026-10-11)

At 124 budget points against 45, INTENT split along its dependency
seam (Ev approved the division in chat, 2026-10-11). The rows moved by
`git mv`, keeping ids and bodies. INTENT keeps the D10 stage chain and
every row that waits on it: the stage umbrellas,
`d10-one-way-to-say-intent-is-unbuilt`, the exit walk and the refactor
hold. (Stage 4 I and stage 5 C later moved to ATREST, and were built
there.)

The rest went to programs that start now:
- **COINCIDE** (P1): the door and its lint as stage 4 left them.
- **REFS** (P1): DM4 unit 2, indexed reads and the façade's slot words.
- **INLANE** (P2): every carrier's values in the lane scalar. Ev placed
  this track below INTENT's.
- **FLUSH** (existing): six topo glue rows, under its charter of
  declared contact at the join.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`,
`docs/DUAL-REVIEW-PROTOCOL.md`). The representation units are design
decisions whose impact is broad and hard to change later, so they draw
a dual review.

## Exit criteria

D10's last paragraph lists the clauses it supersedes; the program is
done when no code path implements one of them and the companion pages
state D10's design. An exit walk checks that list.
