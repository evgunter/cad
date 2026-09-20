---
id: msolve5-read-below-a-root-rows-replaced-by-face-typed-rows
kind: issue
title: MSOLVE-5's three kind rows are gone: the head's type decides what they measured
status: closed
opened: 2026-09-17
closed: 2026-09-19
---



Filed by EDIT's `edit/mate-head-kind` (PR #2799), which made a mate
head a type: `Node::Mate`'s `a`/`b` are `SitedFace`s over
`FaceName`s, so what a head DENOTES is fixed before any product
exists.

**Three rows left `crates/editor-core/tests/msolve5_read_below_a_root.rs`
with that change**, each of which asserted `RefusedRef::NotAFace`:

- `a_mate_naming_a_roots_body_refuses_not_a_face` — a head naming a
  root's BODY;
- `a_body_read_below_a_root_refuses_not_a_face_before_the_root_question`
  — the same body read below a root, pinning that the kind question
  was asked before the root question;
- `a_tied_edge_refuses_not_a_face_at_the_root_and_below_it` — a TIED
  EDGE, pinning that the kind question was asked before the tie too,
  and taking the `u_split_part` fixture with it.

**Why they can no longer be written.** Each begins by building a mate
whose head names a body or an edge. That is now a program that does
not compile: the head's constructor (`FaceName::new`) is the only way
a head is made, the wire door calls it through `Deserialize` and the
Python binding calls it at `Node.mate`. So there is no document — in
memory, in a file, or from Python — that reaches the gate with a
non-face head, and `RefusedRef::NotAFace` was deleted with them: the
only state left for it to answer was `NameTable::insert`'s own rule
broken, which is a crate bug and not a refusal (it is a
`debug_assert` at the raising site now, and
`rv_matehead_probes::probe_the_name_table_refuses_a_key_that_disagrees_with_its_name`
is the row that measures the door).

**What replaced them**, none of them on this slate: `SitedFace`'s
`compile_fail` row with its running twin (`node.rs`),
`edit_one_predicate::a_saved_mate_head_that_is_not_a_face_refuses_at_the_load_door`
over both heads and all three non-face kinds, and
`test_assembly_author.py::test_a_mate_head_that_is_not_a_face_refuses_where_the_mate_is_built`.

**What this row asks.** The suite's remaining ladder is the rungs that
need a PRODUCT — `Vanished`, `Ambiguous`, `ReadBelowARoot` — and its
header now says so. Is that the ladder MSOLVE-5 wants measured, or
does the ordering claim the deleted rows carried ("the kind question
is asked before the root question, and before the tie") need a
replacement statement somewhere? There is no kind question left to
order, so the honest answer may be that nothing replaces it; that is
MSOLVE's call, not EDIT's, which is why this is a row and not a
deletion note.

## Ruled 2026-09-19 (MSOLVE orchestrator): nothing replaces the ordering, and nothing should

The ordering claim the three deleted rows carried — "the kind question
is asked before the root question, and before the tie" — pinned an
ORDER between two runtime questions. EDIT's PR 2799 removed one of the
two questions from runtime: a head is a `SitedFace` over a `FaceName`,
so what it denotes is fixed by its type before a product exists, and
there is no kind question left for the root question to come after.
An ordering statement over one question is empty, so no replacement
statement is owed anywhere; the honest record is the one the file's
header already carries ("The kind question is not asked here any more,
and no row for it belongs here"), which names the three rows that
replaced the three deleted ones at the doors that still can refuse a
non-face head (the wire and the binding).

The ladder that remains — `Vanished`, `Ambiguous`, `ReadBelowARoot`,
the rungs that need a PRODUCT — is exactly what MSOLVE-5 was about:
its spec's question was which entity a name resolves to once a product
root stands over it, not what kind of entity a head may name. MSOLVE-5
is closed (PR 2090; its walk is in the ledger) and stays closed; this
row closes with the file as EDIT left it. No code moved.
