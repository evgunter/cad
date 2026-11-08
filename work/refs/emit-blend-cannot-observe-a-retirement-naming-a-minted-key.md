---
id: emit-blend-cannot-observe-a-retirement-naming-a-minted-key
kind: issue
title: editor-core: emit_blend's retired-set guard cannot observe a retirement naming a minted key
status: closed
opened: 2026-09-13
closed: 2026-11-06
priority: P0
cost: M
---


## Finding

`editor_core::names::emit_blend::name_blend` builds its retired set from
the kernel's record —

```rust
let retired_e: BTreeSet<EdgeKey> = rec.dead.edges.iter().copied().collect();
```

— and consults it in exactly one place: the arm that handles an OUTPUT
entity which is not a recorded mint. A would-be survivor whose key the
record lists as retired refuses `NamingError::Emission`.

A retirement naming a key the carve MINTED and then killed is therefore
invisible here. Such a key is in neither arena the emitter walks: it is
not in the output (the carve killed it) and it is not a survivor, so the
row loop never asks about it, and the entry sits in `retired_e` matching
nothing. The emitter accepts the table and reports success.

`emit_shell.rs` copies the guard and its argument verbatim, so the same
blind spot is in the shell's emitter.

## Why this is not just the unreachable-guard note already in the tree

`sweep::blend::naming`'s module docs (the paragraph `emit_blend`'s docs
cite) say the guard cannot fire while `topo::Body`'s arenas reissue no
retired key, and that is true and is why the guard is kept. This is a
different statement: even if the kernel DID record a minted key as a
retirement — a real kernel bug, and the one BLEND unit 8 fixed in the
ladder rim phase — the document layer would not notice. The guard is
written as though a bad retired-set entry must collide with an output
key, and a minted-and-killed key never does.

## Evidence

BLEND unit 8 (PR 2505) found the ladder rim phase pushing a fresh
`split_edge` key into `rec.dead.edges` on 14 shipped test rows at its
merge base. Every one of those rows carved to a valid body and named
nothing wrong at the document layer; the defect was visible only from the
kernel side, against the source body. Restoring the defect (the unit's
mutant) reds kernel rows and leaves a document-layer naming row green.

## Possible shapes of a fix (EVAL's call)

- check the retired set against the TARGET table rather than against the
  output body: every retired key should resolve to an upstream name, and
  a minted key does not;
- or state at the guard that it covers the arena-reissue case only, and
  name the kernel-side postcondition (`sweep::blend::surgery`'s
  `blend_surgery` epilogue, added by BLEND unit 8) as the home of the
  other direction.

## Home

`crates/editor-core/src/names/emit_blend.rs` and `emit_shell.rs` are
EVAL's (BLEND's `program.md` `keep_out` says so), and no open program's
`paths` covers them today, so this is filed here for EVAL to claim
rather than into another program's directory.

Moved from `work/issues/` to `work/refs/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Closed

The second shape. Both emitters' module docs now say their retired-set
guard sees only a retired key that is IN the output, and name the
kernel check of the other direction: `sweep::blend::surgery::blend_surgery`'s
epilogue for the blend (already there, now the function
`records_rooted` it calls), and for the shell a new
`#[cfg(debug_assertions)]` postcondition, `topo::shell`'s
`record_rooted`, called at the end of `shell_open`, the one place that
holds the operand, the result and the record together. Both are
`debug_assertions` checks, which the release profile keeps on
(`Cargo.toml`).

The shell's `dead` is in RESULT keys and legitimately names keys the
construction minted — the cavity twins its chart reduction and glue
kill — so the blend's predicate ("a retirement names a SOURCE key")
would be wrong there. `ShellRetired`'s doc already states the shell's
rule: scaffolding is in no row and not listed. The postcondition holds
that: every retired face, edge and vertex is a key the operand carries
or the result half of some row; every row's source half is a key the
operand carries; and no retired key of any arena resolves in the
result. The last closes a gap `emit_shell`'s guard leaves: it consults
the retired set only for a key no row mints, so a retired key that is
a row's result half and still live passed both. The destructure is
exhaustive over `ShellNaming`, `RimNaming` and `HoleRim`, so a new row
kind has to say where its halves are. `dead.loops`, `surfaces` and
`shells` are checked for absence alone: a rim's `ring` is a result
loop with no source half, and a cavity twin's loops, surface and shell
are minted with no row.

It fires on no existing row (topo, sweep, editor-core, verbs and
step-import shell rows run locally in debug). Both postconditions take
public-field records, so each direction has a permanent
`#[should_panic]` row handing it a record no construction writes:
`shell::tests::record_rooted` (a retirement no row mints, a row rooted
at a minted key, a retired key the result still carries) and
`blend::surgery::tests` (a retirement, and a birth row, rooted at a key
the source never carried). Before those rows, two mutants were run once
and reverted: pushing the band surgery's split-minted middle piece into
`dead.edges` (BLEND unit 8's defect, on the shell), and recording the
end seam piece as its own source; each reds
`shell_curved_mouth::a_tubes_wall_opens_to_two_seamed_bands`.

The check codifies that `dead` holds twin keys, which
`Shelled::lineage` had read as operand cells, stating a fate for every
retired edge and vertex. It now states one only for a key no row mints
(`ShellNaming::row_mints`: the result half of a row whose two columns
differ), and the tube row pins that a retired twin seam has no fate.

The sweep over every other record that publishes kills found no
retirement claiming source keys without a check: the boolean's and
split's `killed_lineage` / `carved_lineage` / `joined_lineage` and the
graft's `dead_edges` are hop maps OF minted-and-killed keys by design,
`EdgeJoin::gone` is a join row, and `merge_faces::MergedGroup` records
kills of its own input and mints only rings.
