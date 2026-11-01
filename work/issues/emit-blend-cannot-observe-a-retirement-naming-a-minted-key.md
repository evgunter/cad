---
id: emit-blend-cannot-observe-a-retirement-naming-a-minted-key
kind: issue
title: editor-core: emit_blend's retired-set guard cannot observe a retirement naming a minted key
status: open
opened: 2026-09-13
priority: P0
cost: D
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
