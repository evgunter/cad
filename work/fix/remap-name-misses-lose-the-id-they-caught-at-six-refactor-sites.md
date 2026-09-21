---
id: remap-name-misses-lose-the-id-they-caught-at-six-refactor-sites
kind: issue
title: refactor.rs discards remap_name's Err(RecipeNodeId) at six sites, so a miss inside a name's PATH segment is reported as the whole name being stranded
status: closed
opened: 2026-09-11
refs: [2378]
branch: fix/remap-name-carries-the-id
pr: 2945
priority: P1
cost: E
closed: 2026-09-21
---


## Finding

Found by WIRE's `names-refusal-carries-cause` lane (PR 2378) in its
`map_err(|_| ..)` sweep over `crates/editor-core/src/`; outside its
fence, filed here by the WIRE orchestrator because
`crates/editor-core/src/refactor.rs` is FIX's by its `paths`. Accurate
at `af8bbca`.

Six sites — `refactor.rs:831`, `:1504`, `:1544`, `:1807`, `:1840`,
`:1858` — call `remap_name(…, &node_map)` and `map_err(|_| …)` the
result into `RemapMiss::Name`, `SplitError::NameStraddlesCut` or
`InlineError::StrandedPartName`, discarding `remap_name`'s
`Err(RecipeNodeId)`.

**Why the id is not redundant with the name the raised error carries.**
A `StableName` embeds other names in its `path`, so the node that could
not be remapped may be in a **path segment** rather than the name's own
`node` field. The raised error names the outer name; the discarded id
names the node that actually failed. For a nested name those are
different, and the one a reader needs to act on is the one thrown away.

The lane classed this `reported` rather than `not-this-unit` precisely
because of that: it is not the information-free `TryFromIntError` shape
that the rest of the sweep's hits are.

## What a taker owes

The id carried on each of the three error kinds, or a stated argument
that the outer name suffices — which would need to say what a reader
does when the outer name is nested and the miss is two segments down.
PR 2378's argument for carrying is worth reading first rather than
re-deriving.

**Where else to look**, since six sites in one file is a class and not
an accident: every other caller of `remap_name` in `refactor.rs`, and
the same question for any sibling remapper that returns a located error
into a call site that raises a name-shaped one.

## Ruled (FIX orchestrator, 2026-09-20): carry the id

The row offered "the id carried on each of the three error kinds, or a
stated argument that the outer name suffices". The seat rules **carry
it**, on FIX's own ground (`crates/editor-core/src/refactor.rs` is
FIX's by `paths`, territory names no other owner).

**The argument for the alternative does not survive its own question.**
"The outer name suffices" has to answer what a reader does when the
outer name is nested and the miss is two segments down — and there is
no answer: a `StableName` embeds other names in its `path`, so the
raised error names the outer name while the discarded id names the node
that actually failed, and for a nested name those are different. PR
2378's lane classed this `reported` rather than `not-this-unit` for
exactly that reason; read its argument before re-deriving one.

**What the lane establishes rather than assumes.**

- The six sites (`:831`, `:1504`, `:1544`, `:1807`, `:1840`, `:1858`)
  are accurate at `af8bbca` — re-derive at your merge base, and sweep
  the rest of `remap_name`'s callers in the file while you are there:
  six sites in one file is a class, not an accident.
- **Whether any of the three error kinds crosses to Python.** Adding a
  field to a type `pncad-py` mirrors reaches LIB's ground and the tag
  inventory; if it does, that is a disclosure and possibly a second
  unit, not something to absorb quietly.
- Whether a sibling remapper returns a located error into a
  name-shaped one at some other call site — the row names that as where
  else to look.

## Closed (2026-09-21) — PR 2945, the id carried at ten sites

`RemapMiss::Name` is a struct variant `{ name, missing }`; `remap_face`
returns `Result<FaceName, RecipeNodeId>` like `remap_name` rather than
collapsing the id on the way out, leaving `remap_node` the one place
the id is paired with its name. `SplitError::NameStraddlesCut`,
`SplitError::PartNameReachesRemainder` and
`InlineError::StrandedPartName` carry it, and their `Display` impls say
the node.

**Ten sites, not six.** The row's list was accurate at `af8bbca` and the
lane re-derived at its own merge base, as the standing instruction asks:
the extra four route through `remap_face`, which did not exist when the
row was written. `grep -n '|_|'` over the file now returns two value
drops and a test panic — no discard left.

**Two judgement calls the seat accepts:**

- **`Option` on `NameStraddlesCut`.** Its two raise sites differ in
  shape: the rewrites know one node, the straddle CLASSIFICATION weighs
  the whole derivation set against the cut and singles out none. `None`
  says that instead of inventing a culprit — the same refusal to make a
  payload claim more than the code knows that this program's wave-3
  lane showed about recourse transitivity.
- **`PartNameReachesRemainder` pulled in off-row.** `RemapMiss::Name`
  converts INTO it, so omitting it would have re-dropped the id one
  level up. Its precondition grew a witness (`find` rather than
  `is_subset`); verified at adjudication that `derivation_nodes` returns
  a `BTreeSet`, so "lowest-numbered out-of-cut node" is deterministic,
  not incidental.

**One round back before merge, and the reason is worth keeping.** The
first push REPLACED `asm4_split_inline.rs`'s flat stranded case with a
nested one. The lane's reasoning for the swap was correct — under a flat
name `missing == name.node` by construction, so the old row was blind to
this defect and could never have gone red — but that file is S-TCOST's
and S-TINT's, and **narrowing the shapes another program's suite covers
is not a side effect a FIX unit gets to have**: a case removed for
convenience is invisible to its owners after merge, a case added is not.
Both shapes now run over one shared setup, and the pair states the
property better than either alone — the id and the name coincide in the
ordinary shape and come apart in the nested one.

**Fences:** `asm4_split_inline.rs` and `edit_instance_crossing_names.rs`
(S-TCOST, S-TINT); `crates/pncad-py/src/py/refactor.rs` (LIB, three
or-patterns binding `..`, no Python-visible change, tag inventory
unmoved). All three announced.

**Filed:** `work/wire/anchor-rewrite-collision-refuses-without-naming-the-colliding-name`
(WIRE — the same class one door over: a collision collapses to `None`
and the refusal is a static string naming no name) and
`work/lib/split-and-inline-name-refusals-do-not-project-the-missing-node`
(LIB — whether the new id is projected to Python; the `node` slot is
free on two arms and spent on the third, so it needs a slot decision
rather than a copy).
