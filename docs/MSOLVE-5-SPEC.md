# MSOLVE-5 — The at-rest gate refuses a mate read below a product root in the operand's voice (spec)

**Program:** MSOLVE (`work/msolve/plan.md`), unit `MSOLVE-5`
(`work/msolve/MSOLVE-5.md`). **Ruling of record:** the MSOLVE
orchestrator, 2026-09-06, on `work/msolve/assembly-gate-refuses-
vanished-on-a-mate-read-below-a-pattern.md` — read it in full; its
measurement (the solve places the mate, the product gathers, the gate
says the name VANISHED) is the premise. **Track:** kernel change to
what a refusal says, not to what is admitted — one style review plus
a correctness arm (§Review). No A/B row.

## What the tree says now

`mint` (`crates/editor-core/src/assembly.rs`, Pass 4 of
`product_recorded` in `product.rs`) resolves each live mate's two
`SitedRef { at, name }` through `resolve_face(names, mate, side,
name)` against the PRODUCT's aggregate `NameTable` — the rows of each
root's own table, re-keyed verbatim by `product::carry_names`. The
operand `at` is never read. `None` from that lookup is
`RefusedRef::Vanished` ("no entity of the product answers to it").

A mate read at an operand that is not a product root has its name
spelled in the OPERAND's table. Under a `Transform` or a `Part`
selection the root's rows carry that spelling verbatim (N1: a
pass-through adds no segment), so the product answers. Under a
`Pattern` every row is wrapped `Instance(i)` at the pattern node
(`names/emit.rs::name_pattern`), so the bare spelling has no row and
the gate refuses `Vanished` — for a mate the solve placed
(`Determining`) on a product that gathered. The word is wrong: the
entity did not vanish, it is spelled at a node the product does not
list, and the gate never asked that node. `mate1_member_vocab.rs::
the_master_name_spelling_still_refuses_vanished` pins the refusal as
ratified — the canonical spelling is the `Instance(i)` head — and
`msolve2_member_chain.rs::the_gate_on_a_mate_read_below_the_outer_
pattern_still_says_vanished` pins the mis-description as the
measurement MSOLVE-2 owed. Both keep refusing; both move their word.

`RefusedRef::NodeGone` is constructed nowhere in the workspace: a dead
arm with a live Python tag (`ref_node_gone`).

## What the unit builds

**1. The gate asks the operand before it says "vanished".** When the
product's table does not answer, `resolve_face` (or a sibling it
calls with the evaluation in hand — `mint` already takes it) reads
the operand's OWN table: the `name_table` of `at`'s live value
(`names::interrogate` reads it the same way: `value_of(ev, at)?.
name_table`). Two questions, in this order:

- The operand's table does not answer to the name → `Vanished`. The
  name names nothing where the mate reads it; the word is honest.
- The operand's table answers, and `at` is not a root the product
  gathered → the new arm:
  ```rust
  RefusedRef::ReadBelowARoot {
      /// The operand the mate reads at — a live node whose table
      /// answers to the name, and which is not a product root.
      at: RecipeNodeId,
  }
  ```
  with a `Display` in the operand's voice: the reference is read at
  node `at`, which is not a root of the product; the product spells
  its entities at its roots, and under a pattern that spelling is the
  `Instance(i)` row. (The `render_reference` prefix — "mate N's a
  reference (a …) does not name a face of the product" — stays; the
  `why` sentence is what moves.)
- The operand's table answers and `at` IS a product root → by
  construction every row of a root's table reaches the product
  (`carry_names`). Measure it: if it is unreachable through doors,
  say so at the arm and answer `Vanished`; if a document reaches it,
  that is the stop clause.

`Entry::Tied` and a non-face entry at the operand are not this
question — the product's lookup already answers `Ambiguous`/
`NotAFace` for its own rows, and an operand that is not a root has no
face to mint on either way: `ReadBelowARoot` is the answer whenever
the operand's table has ANY entry for the name.

**2. `NodeGone` goes.** Delete the variant, its `Display` arm, the
Python tag `ref_node_gone`, the `.pyi` line and the census row, and
say in the PR that no site constructed it. If a site does, keep it
and cite the site.

**3. Consumers.** `RefusedRef`'s `Display`; `pncad-py`: `refused_ref_
tag` (`ref_read_below_a_root`), the `RefusedRef` pyclass gains an `at`
getter (the node id, projected the way the surrounding `mate` getters
project a `RecipeNodeId`), the `.pyi`, the tag census in `src/tests.rs`,
`tests/test_assembly_author.py`'s reachability record (say whether
Python can author the document — the mate door takes an operand since
MSOLVE-1). The viewer renders `AssemblyError`'s `Display` into the
at-rest badge (`session.rs::badge`) and needs no code; one viewer row
pins that the badge names the operand.

**4. The docs.** `crates/editor-core/ASSEMBLY.md` A5: one sentence
that the gate resolves a reference against the product's table and,
failing that, asks the operand whether the name is spelled there, so
"vanished" means vanished. `RefusedRef`'s doc lists its arms with the
question each answers. Present tense.

**5. The rows.** `crates/editor-core/tests/msolve5_read_below_a_root.rs`
(registered in `tests/all.rs`), through ordinary doors:
- the issue's own document — `P(T(top))`, mate read at `T`, name
  `top/…` — solves `Determining`, the product gathers, and the gate
  refuses `Reference { side: B, why: ReadBelowARoot { at: T } }`
  whose message names `T`;
- the same document read AT `P` with the `Instance(0, top/…)`
  spelling: the gate holds (the control — copy 0 is `T(top)`);
- a mate read at a `Part { Instance(i) }` root over the pattern: the
  gate holds (a pass-through root's rows are verbatim);
- a mate read at `T` naming a face `top` does not have: `Vanished`
  (the first question, not the second);
- a name at a live root that names nothing: `Vanished` unchanged
  (`asm_r2b_assembly.rs::a_mate_reference_that_names_nothing_refuses_
  typed` stays as it is);
- the two pins that move, renamed to say what they now pin: the
  master spelling under a pattern root refuses `ReadBelowARoot { at:
  leg }` (the ratified substance — refuses, canonical spelling is the
  `Instance(i)` head — is unchanged; state that in the doc comment);
  the nested document's mate read at the `Part` below the outer
  pattern refuses `ReadBelowARoot { at: part }` and its message names
  the operand, not a vanished name.
A viewer row in `crates/viewer/tests/` on the badge for the issue's
document. A Python row on the tag and the `at` getter if the document
is authorable from Python; if it is not, the reachability record says
so.

## Acceptance

- **A1** The issue's document refuses `ReadBelowARoot { at }` naming
  the operand, with the solve's answer and the product unchanged.
- **A2** `Vanished` is raised only when the operand's own table does
  not answer to the name; the order of the two questions is pinned by
  the fourth row above.
- **A3** Nothing is admitted that was refused, and nothing refused
  that was admitted: every document that assembled before assembles
  after, and the two moved pins still refuse. Only the `why` word
  moves, and each moved expectation is named in the PR.
- **A4** `NodeGone` is gone (or its constructing site is cited).
- **A5** The Python tag, getter, census and `.pyi` agree; the viewer
  badge names the operand.
- **A6** Every existing suite unchanged but the moved expectations.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted
  CI is the verification of record; poll it in the foreground; never
  end a turn with background work active. The repo has four Cargo
  workspaces (`.`, `benches/`, `demos/tour/`, `demos/wild/`): a public
  signature change is checked in all of them, by hand, before the
  push (`work/issues/no-local-script-builds-all-four-cargo-
  workspaces.md`). `pncad-py`'s clippy runs with `--features python
  --all-targets` too.
- Merge-only; push early and often; the PR through the GitHub MCP
  tools (no `gh`). Private `CARGO_TARGET_DIR` and scratch outside the
  worktree; `git status` before every `git add`; never `git add -A`;
  build narrowly.
- Fence: `crates/editor-core/src/assembly.rs` (`resolve_face`, the
  variant, `Display`), `product.rs` only if `mint`'s call needs a
  second argument it does not have, `crates/pncad-py` (tags,
  `py/assembly.rs`, census, `.pyi`, tests), `crates/pncad/src/
  document.rs` if the re-export list is exhaustive, tests,
  `ASSEMBLY.md` A5. Nothing in `mate/*`, `names/*`, `eval/*`, the
  walk, the edit door, or the viewer's code.
- No consumer walk: the question is asked of the operand's table and
  the product's root list, not by walking the DAG upward.
- **Stop clause.** If the operand's table cannot be read at the mint
  (the evaluation handed to `mint` does not hold non-root values, or
  reading it needs a door outside the fence); if a document reachable
  through the doors has the operand's table answering, `at` a product
  root, and the product's table silent; or if the two questions
  cannot tell a vanished name from an unrooted one on some authorable
  document — STOP, write what you measured in the PR as a draft, and
  end your turn.

## Out of scope

Minting a mate read below a pattern on copy 0's row — the ratified
pin says the master spelling REFUSES and the canonical head is
`Instance(i)`; reopening that is an `[ev]` conversation, not a unit.
Refusing at the edit door (a pattern inserted after the mate would
have to refuse or strand it; the gate is where the product's spelling
is known). The lever (`[ev]`, PR 2086). `Ambiguous`/`NotAFace`
semantics.

## Review

One style review plus a correctness arm, claims verbatim:

- **C1** A1 on the issue's document and on the nested one: the gate
  names the operand; the solve's role and the product are unchanged
  by the unit (compare against main).
- **C2** The two questions are asked in the stated order with no `_`
  arm; a name absent from the operand's table stays `Vanished`, a
  name present there at a non-root is `ReadBelowARoot`.
- **C3** A3: no admission moved — enumerate the suites' gate-holding
  documents and confirm each still holds; the two moved pins still
  refuse.
- **C4** The Python and viewer surfaces (A5); the census is
  exhaustive; `NodeGone`'s deletion touched every site (A4).
