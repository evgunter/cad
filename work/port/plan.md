# PORT — the crate-boundary doors (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

Branch prefix: **`port/`**. Away-channel tag `(PORT orchestrator)`.
A/B ordinal band **PORT = 4600–4699**.

## Charter

A boundary door is where the kernel stops being a Rust API and becomes
something a user meets: `pncad`'s façade, the Python bindings, the STEP
and STL readers and writers, and the declared floor the repository builds
against. The rows here are all about what those doors *say* and what they
*let you do*:

- **What a refusal says.** `assembly-door-raises-only-the-head-of-each-refusal-list`
  (a list of refusals, one raised), and
  `product-table-answers-a-tie-before-kind-the-operand-the-reverse` (two
  doors ordering the same two facts oppositely). Both reach
  `editor-core/src/assembly.rs`, which is **EDIT's** (DOCM's until it
  closed at sweep 14); both are announced there, and the widen-vs-contract
  call comes before the diff because widening is a public error-channel
  change that travels out through `pncad-py`.
- **What the Python surface can express.** `python-cannot-set-options-structs`
  (four doors, the pattern already set by PR 1493) and `D341` (a
  bindings census with no mold for `Node`'s ~18 variants). `S107` asked
  what a Python error should be called; Ev ruled it a defect on
  2026-09-15, and the work it released is
  `python-dimensionerror-names-the-quantity-check-not-the-dimension-check`.
  That row and `load-path-stringifies-structured-refusals` are one
  door — the load path destringifies the document layer's refusals, and
  which class they arrive as is the naming row's decision.
- **The exchange residues.** `S415`, three unrelated findings over four
  crates.
- **The declared floor.** `msrv-floor-is-declared-and-never-compiled` is
  a question — does the repository promise an MSRV? — and the edit is one
  line or one CI row once it is answered.

## Territory — none, and why

This program claims **no paths**. `crates/pncad-py/*` and
`crates/pncad/*` are LIB's; `crates/step-*` and `crates/stl/*` are
EXCH's; `assembly.rs`, `node.rs` and `persist/wire.rs` are **EDIT's**,
which opened 2026-09-13 as DOCM's successor on the same fence and whose
own `keep_out` names this program. Each unit is announced to the owner,
and **the owner may simply take the row instead** — that is a good
outcome, not a failure of this program, and a row that leaves this way
moves by `git mv` like any other claim.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `S415` | **M** | Three unrelated residues over four crates; two flagged unsure, shared-constant question is a judgement call | `crates/step-import/src/assemble.rs`, `crates/step-import/src/parse.rs`, `crates/step-export/src/lib.rs`, `crates/stl/src/options.rs`, `crates/pncad-py/src/*` |
| `python-cannot-set-options-structs` | **M** | Four doors, pattern already established by PR 1493; multi-file, census anchors to add | `crates/pncad-py/src/py/value.rs:999`, `crates/pncad-py/src/py/` STL and eval doors, `crates/pncad-py/src/surface_census.rs`, `.pyi` + py tests |
| `D341` | **M** | New `match Node` census anchored on ~18 variants; census shape needs deciding. | `crates/pncad-py/src/surface_census.rs` (`verb_spelling` mold), `crates/pncad-py/src/py/*`; reads `crates/editor-core/src/node.rs` `Node<P>` |
| `assembly-door-raises-only-the-head-of-each-refusal-list` | **H** (was **M**; corrected by the lane, PR 2635) | Widen-vs-contract **decided** (widen, 2026-09-15 — the row says why); dispatches with `product-table-…` as one unit. **The estimate was wrong**: widening retires two public `AssemblyError` variants, so the change spans EDIT's `assembly.rs` and LIB's whole façade — the tag vocabulary, the stub, the guide and the binding census — which is the `H` clause's "spanning several programs' territory". | `crates/editor-core/src/assembly.rs` (`assemble_gathered`, the two `into_iter().next()` sites), `AssemblyError` in `crates/editor-core/src/`, `crates/pncad-py/src/py/` façade, `crates/pncad-py/src/tags.rs`, `pncad.pyi`, `docs/guide/assembly.md`, `crates/pncad/tests/all.rs` |
| `product-table-answers-a-tie-before-kind-the-operand-the-reverse` | **H** (was **M**; corrected by the lane, PR 2635 — the letter follows its unit, which dispatches as one) | Order **decided** (kind-first, 2026-09-15 — the row says why); one-arm change plus ONE control assertion (the tied-face row does not move). **`M` was right for this row alone**; it carries `H` because it dispatches as one unit with the row above and a class letter is a unit's, not a half-unit's. | `crates/editor-core/src/assembly.rs` (`resolve_face`, `display_contract`), `crates/editor-core/tests/msolve5_read_below_a_root.rs` |
| `msrv-floor-is-declared-and-never-compiled` | **E** | **Answered** (Ev, 2026-09-15): a check that the two strings are equal. Was **M** for the decision in front of it; with that gone it is one gate script. | `Cargo.toml` (`rust-version`), `rust-toolchain.toml`, a new `scripts/gates/` row with its two calls in `.github/workflows/ci.yml` |
| `load-path-stringifies-structured-refusals` | **H** | Arrived 2026-09-13 at DOCM's exit sweep, after the table above was first written. Structured kernel refusals stringified at the load door; carries a two-sweep class obligation. **The one row this program gives a full review.** | `crates/editor-core/src/persist/wire.rs` (`Error::custom`), `crates/pncad-py/src/tags.rs`, `crates/pncad-py/src/py/value.rs`, `py/flush.rs` |
| `python-dimensionerror-names-the-quantity-check-not-the-dimension-check` | **M** | **Ruled a defect** (Ev, 2026-09-15) — `S107`'s successor, filed the day the ruling landed. The rename is small; the re-documentation it deletes is the bulk. | `crates/pncad-py/src/errors.rs`, `src/py/mod.rs`, the door docstrings in `src/py/*`, `pncad.pyi`, `src/tests.rs` + the py suite |
| `S107` | — | **Closed 2026-09-15** carrying its verdict. A ruling is never work; the row above is the work it released. | — |

## Order

`S415` opens — three small residues, announced to EXCH, and the one row
here with no decision in front of it beyond a shared-constant call.

Then the Python pair, `python-cannot-set-options-structs` and `D341`, in
that order: the first repeats an established pattern across four doors,
the second builds the census that would have caught the next missing one.

The two assembly rows go together and after a decision, not before: both
change what a door says, both are EDIT's file, and answering one without
the other leaves the two doors disagreeing in a new way. **The decision
is made** (orchestrator, 2026-09-15 — widen the list, order kind-first;
each row carries the reasoning), and they dispatch as one unit,
**`PORT-DOORS-1`**, which both rows now parent to.

`msrv-floor-…` is now the smallest row on the slate and can go whenever a
lane is free — Ev answered it on 2026-09-15 and only the gate is left.

The two naming rows —
`python-dimensionerror-names-the-quantity-check-not-the-dimension-check`
and `load-path-stringifies-structured-refusals` — are **one door and one
spec**, not two units run in sequence. The first frees the name
`DimensionError`; the second decides what the load door's structured
refusals arrive as, and that answer is only available if the first row's
shape question is settled in the same breath. Specced apart, the second
lane inherits a decision the first lane did not know it was making.

## Review posture

**Style review is the default; a full review is reserved for the hardest
units.** (Ev, in-chat, 2026-09-15, revising the posture this section
carried at the cut — which asked for a correctness arm on every unit
changing a public refusal or a binding signature, and so was a full
review on almost every row here.)

The style lane's brief is `docs/prompts/reviewer-style-lane.md` and it is
handed to every reviewer here unchanged. What this program adds to it is
one standing question, because it is the question the charter is about:
**is the new refusal answerable by the person who receives it?** A
reviewer on any row that changes what a door says should read the message
from outside the repository, where the reader has no access to the
kernel's types.

**Which units get the second arm is the orchestrator's call** (Ev,
in-chat, 2026-09-15, delegating it after the first two units). Not a
fixed list of rows — the list this section carried before named
`load-path-…` alone and was wrong within a day, when `PORT-DOORS-1`
turned out to retire two public enum variants. Four triggers, any one
of which earns it:

1. **The unit retires or renames something published** — a public enum
   arm, a binding signature, a stable tag value — so code outside this
   repository changes meaning.
2. **The unit's central claim has no mechanical guard**: nothing reds
   when the guarantee *degrades* rather than when it is violated
   outright.
3. **The defect was found by execution rather than by reading**, so the
   tree does not show it and a reviewer reading the tree will not
   either.
4. **The unit changes the subject matter of a ratified design page.**
   This one was paid for: `PORT-DOORS-1`'s single falsified claim was a
   stale citation on `crates/editor-core/ASSEMBLY.md`, and neither of
   that lane's two sweeps was shaped to look there — **a lane's
   instruments are shaped by its own diff**, so a ratified page citing
   a symbol the diff renames is outside every instrument the lane
   naturally builds. The cheap habit that needs no review at all is a
   `git grep` over `crates/**/*.md` and `docs/**/*.md` for every public
   item a unit renames; the second arm is for when that habit is not
   enough.

**The converse matters as much**, and is why the old list over-fired —
but it holds **per CLAIM, not per unit**, and this section applied it
per unit until `python-cannot-set-options-structs` disproved it. A
claim that is compiler-enforced does not earn the second arm however
public its surface; a unit earns it when **any** load-bearing claim of
it is unguarded, and a neighbouring claim being compiler-enforced buys
that one nothing.

`python-cannot-set-options-structs` is the worked example, because it
is the unit that paid for the distinction. Its field-presence claim is
as guarded as a claim gets: `surface_census.rs`'s destructure anchor
reds `E0063` at the door and `E0027` at the census when a struct gains
a field, and the lane measured it rather than asserting it. Its other
claim — that an omitted keyword forwards the Rust default — was
guarded by nothing, and **a presence anchor cannot guard a forwarded
VALUE**. That is not an oversight in the anchor; it is what a
destructure is. Which is how two STL doors sat under a green census
writing `solid ` and eighty zero bytes where the kernel's own defaults
say a part name and a producer line, for as long as they had existed.

So: **a claim about a forwarded value is never compiler-enforced by a
presence anchor**, and a unit carrying one earns the second arm even
when the census beside it cannot be fooled.

On this slate today: `load-path-stringifies-structured-refusals` earns
it on triggers 2 and 3, and its paired naming row rides that review
since they spec together. `PORT-DOORS-1` earned it on 1 and 4 and was
granted it. `python-cannot-set-options-structs` was dispatched style-only
on the per-unit reading above and was given the second arm after the
fact; under the corrected reading it earned it from the start, on its
forwarded-value claim. Everything else is style.

A lane that finds its row harder than this section assumed says so in
its PR and asks for the second arm; this section is corrected in the
same PR, and so is the class column.

## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
