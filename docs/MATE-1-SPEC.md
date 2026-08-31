# MATE-1 — issue 945: mates × patterns (the A11 member-vocabulary rider, implemented)

**Binding at dispatch** (S-MATE program, `docs/S-MATE-PLAN.md`;
difficulty logged pre-draw in the plan's opening commit: **M/L**,
recorded numeric M). Read `docs/prompts/implementer-discipline.md`
in full before starting. The primary specification is the RULING —
the member-vocabulary rider in `docs/ASSEMBLY-DESIGN.md` (A11,
"Member vocabulary") plus the ruling-of-record comment on issue 945
— not the issue body (both of its original readings are superseded
by the ruling).

## Situation

Two shipped mechanisms together make it impossible to mate anything
to a patterned part (issue 945's body has the walkthrough):

1. `head_of` (`crates/editor-core/src/mate/solve.rs:123`) accepts
   only a live `Node::InstantiatePart` head; a pattern-placed
   entity's name is headed by the PATTERN node
   (`{ node: pattern, path: [Instance { i, of }] }`), so a mate
   naming a patterned instance refuses `MateFault::DanglingHead`.
2. A pattern consumes its input instance's root, so a mate naming
   the pattern MASTER by the instance's own name refuses
   `AssemblyError::Reference { why: Vanished }` at the gate.

The rider rules exactly what changes: a mate reference head is a
live `InstantiatePart` OR a pattern-placed instance (`Pattern` node
+ `Instance(i)` qualifier) at its pattern-derived pose; rules 3–4
bind as written (gauge-ineligible, never a tree child); rule 1's
coset algebra is unchanged — the member frame is an ordinary frame
conjugated through the derived, static offset, and per-instance
freedom is never created. Mechanism 2 is CORRECT and stays: the
canonical spelling is `Instance(i)` heads, and the consumed master's
faces refuse `Vanished` honestly.

## Deliverables

1. **`head_of`'s member vocabulary**: accept a pattern-placed
   instance head per the rider. The reading edges (A12) and the
   relative-freedom partition (A9) follow from whatever `head_of`
   accepts — state in the PR what edge a pattern-member mate
   contributes and why that is the rider's "joins the other member
   into the pattern's cluster."
2. **The member frame in the solve**: a pattern-placed member's
   frame is its pattern-derived pose — composed through the derived
   static offset, never solved per-instance. The pattern instance
   is gauge-ineligible and never a tree CHILD; a mate to
   `Instance(i)` places the OTHER member (rule 2).
3. **The loop behavior**: a second tree mate from a sibling
   instance closes a loop → non-tree → declaring → verified (the
   stud-stack behavior rule 4 promises). One row demonstrates it:
   consistent loop verifies, inconsistent loop dies at the closing
   mate's verification naming it.
4. **The two ratified pins, each its own row**:
   - mates never solve pattern parameters — a seat satisfiable only
     at a different spacing is CONTRADICTORY with the measured
     clash, recourse = edit the parameter;
   - `Instance(i)` heads are canonical — the master-name spelling
     still refuses `Vanished` (a pinned refusal, not a fixed one).
5. **Red-first acceptance from the issue's own shape**: the
   four-legs-one-top document (pattern the leg, mate the top to
   `Instance(i)`) — `DanglingHead` on main, demonstrated in the PR
   body; solves after, with the solved pose asserted against an
   independently hand-composed frame (translation AND rotation, the
   ASM-DEMO precedent — derive it in the test from the pattern's
   own parameters, not from the solver's output).
6. **Class sweep** (discipline §5): the genus is "a consumer
   dispatching on the head node's KIND where the rider now admits
   two kinds" — sweep the mate/assembly lanes for sites that
   pattern-match `InstantiatePart` heads (solve, gate, minting,
   update re-verification, selector resolution); hit list with
   per-hit disposition in the PR body, blind spots stated.

## Acceptance

- The red-first row green; the two pin rows green; the loop row
  green both directions (verify / die-at-closing-mate).
- The existing mate/assembly suites green
  (`editor-core` incl. `asm_r2a_mate_solve`, plus the crate's
  assembly gate tests); no demo edits.
- ε posture (the issue-1356 discipline): say in the PR which CI
  lane/ε the gate drew, and whether the unit argues
  band-independence (this unit is structural — if any numeric
  tolerance enters, say where and why) or pinned a lane with a
  `CI-Config:` trailer on the head commit.

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits
  (blinding; if one lands in a PUSHED commit, note it in the PR
  body and carry on — never rewrite history, never stop the unit).
- Keyword hygiene: write "issue 945" spelled out; never a closing
  keyword before a `#`-reference. The orchestrator closes the issue
  after merge.
- Scope fence: `crates/editor-core/src/mate.rs`,
  `crates/editor-core/src/mate/`, `assembly.rs` only where the
  canonical-spelling pin or minting path requires it, and
  `crates/editor-core/tests/`. Nothing else — no
  `crates/topo` (census/props are other units' ground), no
  parameters/analysis/eval or `product.rs` (M10's), no schema
  bump (the rider adds no serialized state; if you find it does,
  STOP and report — schema is contested territory), no `pncad`/
  `pncad-py` surface changes (record any façade gap as a
  finding for LIB instead), no `docs/MODEL-AB-LOG.md`, no
  `docs/S-MATE-*.md`.
- The refusal vocabulary is untouched per the ruling; any refusal
  whose REACH changes is classified against the D2 addendum in the
  PR body.
- Commit and push after every coherent unit of work (branch
  `mate/1-member-vocab`).
