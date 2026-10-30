# CAVITYDIAG: why the cavity gate refuses three valid bodies

Row: `work/clear/shell-cavity-gate-over-refuses-three-valid-bodies.md`
(P1). Program: CLEAR. Branch: `clear/cavitydiag`.

Review: decided by what this unit becomes. A diagnosis with scalar or
classification fixes like RIMSCALAR's gets a single FULL review with
mutation. A new presentation arm gets DUAL.

The work is **diagnosis first**. Find each body's cause before changing
anything.

## Context

SHELL-4's cavity gate runs the self-intersection certificate on a
shell's moved clone. On SHELL-4's branch at `7d97070458` it turns three
valid bodies' rows red; the row file has the details:
1. the corpus pot's operand, at `Interval` only (a sphere/cylinder
   vertex, `Departure`);
2. the klein elbow's cavity, at both scalars (`Departure`, 0 splits);
3. the cone frustum's cavity, at both scalars (`Budget(Pairs)`).

## Relevant history

- RIMSCALAR (#6090) found a class: exact-value tests on enclosures make
  `Interval` diverge from `f64`.
- The corner arm (CORNER, #6045) reads only the plane and the cylinder
  (`reading`). The torus was excluded until it is argued, and the
  sphere and cone may not be read either.

## What is true when this lands

1. **Each body's cause is named.** For each body, give:
   - the refusing pair, the shared cell, and the arm;
   - why it refuses;
   - which class the cause is in:
     - (a) a scalar or exact-value divergence, like RIMSCALAR's;
     - (b) a chart the corner or curved presentation does not read
       (sphere, torus, cone);
     - (c) a budget or cost cause, such as cells the walk cannot
       discharge cheaply;
     - (d) something else.
   - Reproduce each failure on `origin/main`, by running the sweep on the
     operand or cavity directly. SHELL-4's branch is evidence; do not
     depend on it.
2. **Class (a) is fixed here**, soundly. That means a decision on the
   enclosure at a funnelled site with an audit row, or structurally, and
   no new ε without a verdict that needs it. Revert any concession alone
   and diff the verdicts. Every self-intersection and pipe row keeps its
   verdict class at both scalars, and no f64 receipt moves.
3. **Class (b) and (c) are not built here.** For each, write the design
   problem precisely in the row:
   - which chart;
   - what the presentation would need to argue;
   - for a budget, where the cells go and why they don't discharge.

   Stop there. That is a design question for the orchestrator.
4. **Rows:**
   - each body that is fixed holds at both scalars, as a self-intersection
     row on the operand or cavity, built by the generic builders, with
     its mutation shown;
   - each body that is not fixed is pinned typed with its refusal, citing
     the row.

## Ground

- `crates/topo/src/injective.rs`, `crates/topo/src/clearance.rs` and the
  sweep tests.
- Fixtures: `lib_g17_shell_node` (editor-core), `torax_interval` and
  `sf2b_interval_probe` (sweep).
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record.

## Report

≤120 lines:
- each body's cause, with its class and evidence;
- what was fixed, and its soundness argument;
- the design problem for anything left;
- the rows;
- the PR number and its CI result.

If nothing is in class (a), open no PR: commit the row's analysis on
the branch and report.
