---
id: named-copies-of-fixture-doors-under-other-names
kind: issue
title: Seven name1 copies of fixture::minted and five in_copy copies survive the named-copy sweep
status: open
opened: 2026-09-19
---


## Finding

`work/edit/editor-core-suites-redefine-the-name-table-helpers` swept
`crates/editor-core/tests/` for redefinitions of `fixture`'s doors by
grepping **the door's own names** (`fn <name>(` / `let <name> = |` over
a hand-written list of sixteen). PR #2867 re-ran that grep and reports
"`face_of`, `edge_of`, `vertex_of`, `ends`, `face_vertices`,
`face_edges` and `minted` have **no** named copies at all". That is
false for `minted`, and the list itself missed a door.

**`fn name1` IS `fixture::minted`, byte-for-byte, in seven files** —
same signature, same body, a different function name:

- `crates/editor-core/tests/m4_pr3_names.rs:27`
- `crates/editor-core/tests/m4_pr3_names_bool.rs:38`
- `crates/editor-core/tests/ring_r1_names_probe.rs:31`
- `crates/editor-core/tests/m4_pr7_appearance.rs:38`
- `crates/editor-core/tests/m4_pr4_resolve.rs:68`
- `crates/editor-core/tests/m4_pr4_appearance_hook.rs:45`
- `crates/editor-core/tests/bool7_shadow_exec.rs:395`

The first three are files #2867 touched — it deleted their `table`
copy and left the `minted` copy fifteen lines away. Two more are
`minted` with the node partially applied: `shelled` in
`lib_g17_shell_node.rs:50` and `lib_g17_r2_probes.rs:17`.

**`fn in_copy` is a copy of `fixture::in_copy`** (`fixture/mod.rs:96`)
under the SAME name, in five files — so the row's own name grep would
have found it had `in_copy` been on the hand-written list:

- `mate1_member_vocab.rs:59`, `mate1r2_probes.rs:57`,
  `fix_pattern_mate_crossing.rs:75`, `rev_fix_xsplit_unreachable.rs:55`,
  `mate1_r1_probes.rs:49`

These five have **diverged from the door**: each hardcodes
`kind: EntityKind::Face` where `fixture::in_copy` carries `of.kind`
through, so the copies are silently wrong for an edge or vertex master.

**The literal form.** 87 sites outside `tests/fixture/` spell
`StableName { kind: EntityKind::<K>, node, path: vec![seg] }` directly
— 68 Face, 18 Edge, 1 Vertex — which is exactly `fname` / `ename` /
`vname` / `minted`. #2867's shape sweep ran this pattern but filtered
it to `kind: EntityKind::Vertex`, because the row's ruling named only
"the eight `StableName` literals that spell a vertex name". Two of the
Edge sites are in files #2867 touched: `blend5_rim_support.rs:348`
(`let arc = |seg: RoleSeg| StableName { kind: EntityKind::Edge, node:
revolve, path: vec![seg] }`, which is `let arc = |seg| ename(revolve,
seg)`) and `display_contract.rs:1211` (`ename(RecipeNodeId(7),
RoleSeg::Cap(CapEnd::End))`).

## Why it matters

The class the EDIT row named is "a copy of a door", and the instrument
it was measured with is "the door's NAME, from a list typed by hand".
A copy under another name is invisible to it, and `in_copy` shows the
list itself is the leak: a door absent from the list keeps its copies
even when they are same-named. The `in_copy` divergence is the
concrete cost — five copies that answer `Face` where the door answers
the master's kind.

## What a taker owes

Take the door list from `fixture/mod.rs`'s public surface rather than
by hand, and sweep by BODY shape as well as by name (a `-> StableName`
function whose body is one of the doors' bodies, modulo partial
application). Read each site before changing it, as the sibling rows
do. The `in_copy` copies need their `EntityKind::Face` checked against
each caller before the door's `of.kind` replaces it.

## Territory

`crates/editor-core/tests/*` — tcost's and tint's. Filed by the
`review/helpers-rv` lane reviewing EDIT's PR #2867; sibling of
`inline-name-table-reads-bypass-the-fixture-door`.
