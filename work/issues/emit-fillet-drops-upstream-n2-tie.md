---
id: emit-fillet-drops-upstream-n2-tie
kind: issue
title: emit_fillet does not propagate an upstream N2 tie — a legitimate tie surfaces as NamingError::Duplicate
status: open
opened: 2026-08-20
github: 708
refs: []
---

## From GitHub issue 708

Opened 2026-08-20; 0 comments.

Tracked out of `docs/SMELL-SCAN-2026-08.md` §S15 (Track D unit **D5**), the one row of that roll-up that is bug-shaped rather than prose-shaped.

## The gap

`crates/editor-core/src/names/emit_fillet.rs` — `name_fillet`'s `up()` closure reads the target's name for a source entity through `NameTable::name_of`, which returns the entity's `StableName` and nothing else. It never inspects the `Entry` that name resolves to.

`Entry::Tied(candidates)` is the ratified N2 shape: one name, ≥ 2 equally-admissible entities, naming succeeds and *referencing* is what reports `Ambiguous`. Every member of a tie carries the same `StableName` in the reverse map. So if a fillet's target table carries a tie and two of its tied members are both sources for the fillet — two blend faces off two tied edges, say — both minted entities are handed the *same* upstream name, both build the same `RoleSeg`, and the second `NameTable::insert` refuses `DuplicateName` → `NamingError::Duplicate`.

That error's own docs (`table.rs`, `DuplicateName`) say it means "a duplicate-name insertion outside the tie path (the no-silent-aliasing bug, typed)". A legitimate upstream tie would therefore be reported as an emission bug in this crate.

## Why it is not a live bug today

**Nothing mints a first tie**, and the evidence is the *shape* of every `insert_tied` call site, not their number. There are seven in production, and each one is downstream of an existing `Entry::Tied`:

| Site | How it reaches `insert_tied` |
|---|---|
| `names/emit.rs:188` (`emit_pattern`) | matches on the upstream `Entry`; the `Tied` arm re-ties |
| `names/emit.rs:267` (`emit_placed_union`) | `rows` comes from the upstream `Entry`; ties only when ≥ 2 survive the move |
| `names/emit.rs:327` (`emit_instantiated_part`) | matches on the upstream `Entry`; the `Tied` arm re-ties |
| `names/emit.rs:563` | matches `Entry::Tied(es)`, re-inserts it verbatim |
| `names/emit_topo.rs:182` (`TieRows::flush`) | rows reach the tie lane only via `put(.., from_tie, ..)`, and `from_tie` is `matches!(lookup, Some(Entry::Tied(_)))` |
| `product.rs:526` | `rows` comes from the upstream `Entry`; ties only when ≥ 2 survive the filter |
| `eval/anchor.rs:326` | matches `Entry::Tied(ents)`, clones it |

Plus three in test code (`appearance.rs:527`, `tests/lib_sel1_geoselect.rs:625`, `tests/m4_pr4_resolve.rs:313`), each hand-building a tie to exercise the resolve ladder.

So `Entry::Tied` is unreachable through the public recipe surface, and so is this refusal. *(An earlier revision of this issue said "three callers" and named only the `emit.rs` ones — a frozen enumeration, which is exactly the defect the parent finding is about. The conclusion was right and the evidence was not; corrected here.)*

The gap is that `emit_fillet` is the emitter that did **not** get the propagation arm the others have. It will break on the day the first producer lands, and it will break by reporting the tie as an aliasing bug.

## What closing it takes

More than a one-line arm, which is why this is tracked rather than fixed in the D5 PR:

- `up()` must return the `Entry`, not the name, and every call site must decide what a tie means for its role.
- `RoleSeg` holds one `Box<StableName>` per slot (`BlendFace(name)`, `TrimEdge { edge, support }`, …). A tied source means either N segments and an `insert_tied` of the minted entity under all of them, or a decision that a fillet of a tied source is itself tied. That is a naming-design question (which of `FromA`/`Seam` covariance survives a tie), not a mechanical edit.
- `check_total` and the `Retired`-survivor guard both walk single names today.
- `emit_topo`'s `TieRows` deferral is the shape to copy: it defers rows to a stage boundary precisely because a tie cannot be inserted one member at a time.

The fix should land **with** the unit that mints the first tie, so the propagation shape can be decided against a real producer rather than a hypothetical one — same posture as `topo/src/attach.rs`'s KNOWN HAZARD block for sense inheritance. A KNOWN HAZARD block in `emit_fillet.rs`'s module docs points here.

## Home

`work/issues/`: `crates/editor-core/src/names/emit_fillet.rs` is not in any open program's `paths` — SEAT's naming territory is `names/geompred.rs` and `names/flush.rs` only — and S-BLEND, which owned the fillet emitters, is closed.
