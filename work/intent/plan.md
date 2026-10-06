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
   directions as their own variable kinds: a datum's origin and axes are
   not perturbable reals by type and are never analysis axes (stage 1
   spells them as untoleranced `Scalar` variables in the interim, which
   the tolerance rule keeps out of analysis); a placement is the
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

Stages 2–6 are not yet sliced into rows. Each is sliced, and split into
its own program where it outgrows this one's budget, when the stage
before it is in review.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`,
`docs/DUAL-REVIEW-PROTOCOL.md`). The representation units are design
decisions whose impact is broad and hard to change later, so they draw
a dual review.

## Exit criteria

D10's last paragraph lists the clauses it supersedes; the program is
done when no code path implements one of them and the companion pages
state D10's design. An exit walk checks that list.
