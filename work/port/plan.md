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
  `editor-core/src/assembly.rs`, which is DOCM's; both are announced
  there, and the widen-vs-contract call comes before the diff because
  widening is a public error-channel change that travels out through
  `pncad-py`.
- **What the Python surface can express.** `python-cannot-set-options-structs`
  (four doors, the pattern already set by PR 1493) and `D341` (a
  bindings census with no mold for `Node`'s ~18 variants). `S107` asks
  what a Python error should be called and is Ev's.
- **The exchange residues.** `S415`, three unrelated findings over four
  crates.
- **The declared floor.** `msrv-floor-is-declared-and-never-compiled` is
  a question — does the repository promise an MSRV? — and the edit is one
  line or one CI row once it is answered.

## Territory — none, and why

This program claims **no paths**. `crates/pncad-py/*` and
`crates/pncad/*` are LIB's; `crates/step-*` and `crates/stl/*` are
EXCH's; `assembly.rs` is DOCM's. Each unit is announced to the owner,
and **the owner may simply take the row instead** — that is a good
outcome, not a failure of this program, and a row that leaves this way
moves by `git mv` like any other claim.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `S415` | **M** | Three unrelated residues over four crates; two flagged unsure, shared-constant question is a judgement call | `crates/step-import/src/assemble.rs`, `crates/step-import/src/parse.rs`, `crates/step-export/src/lib.rs`, `crates/stl/src/options.rs`, `crates/pncad-py/src/*` |
| `python-cannot-set-options-structs` | **M** | Four doors, pattern already established by PR 1493; multi-file, census anchors to add | `crates/pncad-py/src/py/value.rs:999`, `crates/pncad-py/src/py/` STL and eval doors, `crates/pncad-py/src/surface_census.rs`, `.pyi` + py tests |
| `D341` | **M** | New `match Node` census anchored on ~18 variants; census shape needs deciding. | `crates/pncad-py/src/surface_census.rs` (`verb_spelling` mold), `crates/pncad-py/src/py/*`; reads `crates/editor-core/src/node.rs` `Node<P>` |
| `assembly-door-raises-only-the-head-of-each-refusal-list` | **M** | Decide widen-vs-contract first; widening is a public error-channel change through pncad-py | `crates/editor-core/src/assembly.rs` (`assemble_gathered`, the two `into_iter().next()` sites), `AssemblyError` in `crates/editor-core/src/`, `crates/pncad-py/src/py/` façade |
| `product-table-answers-a-tie-before-kind-the-operand-the-reverse` | **M** | One-arm change, but which order is right was put out of MSOLVE-5's scope and needs deciding. | `crates/editor-core/src/assembly.rs` (`resolve_face`, `display_contract`), `crates/editor-core/tests/msolve5_read_below_a_root.rs` |
| `msrv-floor-is-declared-and-never-compiled` | **M** | Decision about what repo promises; edit itself is one line or row | `Cargo.toml:50`, `rust-toolchain.toml`, plus a gate row in `.github/workflows/*` or `local-scripts/ci-local.sh` if the floor is a promise |
| `S107` | **H** | Is itself an unruled Ev question; nothing may start until ruled. | — (if ruled a defect, work lands in `crates/pncad-py/src/errors.rs`, `pncad.pyi`; Track U's) |

## Order

`S415` opens — three small residues, announced to EXCH, and the one row
here with no decision in front of it beyond a shared-constant call.

Then the Python pair, `python-cannot-set-options-structs` and `D341`, in
that order: the first repeats an established pattern across four doors,
the second builds the census that would have caught the next missing one.

The two assembly rows go together and after a decision, not before: both
change what a door says, both are DOCM's file, and answering one without
the other leaves the two doors disagreeing in a new way.

`msrv-floor-…` is a question for Ev with a one-line edit behind it, and
`S107` is a ruling; both go on the next `[ev]` sitting rather than into a
lane.

## Review posture

Style review with a correctness arm on every unit that changes a public
refusal or a binding signature — these are the doors where a change is
visible outside the repository, so the reviewer's question is whether
the new message is answerable by the person who receives it.
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
