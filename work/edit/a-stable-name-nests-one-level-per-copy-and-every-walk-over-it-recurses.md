---
id: a-stable-name-nests-one-level-per-copy-and-every-walk-over-it-recurses
kind: issue
title: editor-core: a StableName nests one NameRef per pattern copy, part instance or merge, and its drop, Debug and descent walks recurse without bound
status: closed
closed: 2026-09-30
branch: edit/name-nesting-stack-safe
pr: 3512
opened: 2026-09-30
priority: P1
cost: M
---


(EDIT, found by the recursion sweep of `edit/part-depth-bound`.)

## What

A `StableName` (`crates/editor-core/src/names/role.rs`) holds its
role path flat, but three segments hold a whole name inside:
`RoleSeg::Instance { of: NameRef }` (a pattern copy),
`RoleSeg::InPart { of: NameRef }` (an instantiated part's entity) and
`RoleSeg::Merged(Vec<StableName>)`. `NameRef` is an `Arc` of a name, so
a chain of K patterns, each over the one before, names its faces K
levels deep, and a part nested D documents deep adds D more. Nothing
bounds either.

Every walk over a name recurses once per level: the `Drop` of the last
`Arc`, `Debug`, `Hash`, `Ord`/`Eq` on names that share no `Arc`, serde,
and the descent walks `names::face_descends_from`, `seam_through`
(`names/seam_pair.rs`), `fold_descent` (`eval/wire.rs`),
`constituents_through_wrappers` (`names/merged.rs`) and
`resolve::walk_names`. The viewer renders a disagreeing pick with
`format!("{name} ({:?})", name.path)` (`viewer/src/idpass.rs`,
`name_and_path`), a `Debug` of the whole nesting.

Evaluation itself does not recurse on it (the peak below is flat), so
the crash lands after it: when the evaluation is dropped, or a name is
printed.

The fix pass's mutual-recursion sweep found three more walks over the
same nesting: the content key's `feed_stable_name` ↔ `feed_role_seg`
(`crates/editor-core/src/eval/mod.rs`), and the union emitter's
`collapse` ↔ `orient` ↔ `seam_line` and `member_faces`
(`crates/editor-core/src/names/emit_union.rs`). The selector patterns
that match names nest the same way: a `SegPat` holds `args:
Vec<NamePat>` (`crates/editor-core/src/names/select.rs`), a Python
caller can nest them as deep as it likes, and their matching and
derived impls recurse once per level.

## Evidence

One document: a square extrude, then K `Node::Pattern`s, each over the
one before with `count: Expr::count(1)`. Evaluated, then the top
pattern's result formatted with `{:?}`, then the evaluation dropped;
the stack each step touched, measured by painting a 512 MiB thread's
stack:

| profile | K | evaluate | `Debug` | drop |
|---|---|---|---|---|
| release | 1 000 | 10 KiB | 799 KiB | 62 KiB |
| release | 10 000 | 10 KiB | 8 111 KiB | 765 KiB |
| dev | 1 000 | 243 KiB | 1 303 KiB | 270 KiB |

So about 80 bytes per level to drop in release and 270 in dev, and
0.8 to 1.3 KiB per level to print: a chain of about 3 500 patterns is
past the wasm32 build's 1 MiB stack in dev, 13 000 in release, and
printing one name crosses it near 700 levels in dev. A part chain at
`MAX_DEPTH` (1024 documents) puts 1024 `InPart` levels into every name
its top instance carries.

Not probed: whether a mate head naming a face that deep survives a
save and a load (the load door's `serde_json` recursion limit of 128
would refuse it typed on the way back in).

## What would close it

A `Drop` for the nesting that does not recurse, and walks that carry
their own stack (or a bound on nesting that every walk fits, refused
typed where a name is minted); `name_and_path` rendering the path
without `Debug`. A row that evaluates a chain of patterns deeper than
the smallest stack allows today, drops the evaluation and renders a
name, on the wasm32 build's stack.

## The load door since PR 3510

The load door now scans a body against
`persist::nesting::BODY_NESTING` (271 JSON brackets) and reads within
it with serde_json's own recursion limit off, where it used to stop at
serde_json's 128. So a name nested between about 32 and 65 `InPart`
levels, which refused at load before, now loads (measured in a
`Datum::FaceFrame`'s face, saved from a snapshot and from an edit log;
66 refuses at the scan), and every walk over a loaded name meets it.

The scan is type-agnostic (the review of PR 3510, its Q7): one limit
for every value in a body. Raising `BODY_NESTING` so deeper names load
would hand `MetaValue`'s recursive load
(`a-metadata-value-nested-deep-enough-kills-the-process`), and any
other recursive reader of a body, the same budget. A name that must
load deeper wants its own counter, as `persist::nesting::Child` counts
an expression's levels, rather than a larger scan limit.

## Built (2026-09-30)

No bound: every walk over a name, and over a selector pattern, is
stack-safe at any depth (`crates/editor-core/src/names/nest.rs`).

- `StableName`'s `Drop`, `Clone`, `Debug`, `PartialEq`, `Hash`, `Ord`
  and serde impls are hand-written, one level at a time. `Clone`,
  `PartialEq`, `Ord`, serde and `Debug` under `{:?}` / `{:#?}` give the
  derived impls' answers, checked at every variant against a copy of
  the derived types (`names/nest_reference.rs`); no other `Debug` flag
  reaches the levels (the hex flags cannot be read on stable). `Hash`
  is consistent with `Eq` and is not the derived sequence. `Drop` moves
  the path of every name it is the last holder of onto its own stack.
  `Eq`, `Ord` and `Hash` recurse natively for 64 levels first (about
  128 KiB of stack in dev, pinned on a quarter of the wasm32 stack).
- Rebuilding walks (the rewrite through `SegRewrite`, the union's
  collapse) share one driver, `nest::descend`: a level lists the held
  names it descends into, each is answered once by address, and the
  level runs once, so a walk is linear in the names it visits (a
  counted-steps row pins it). `NamePat`'s `Clone`, `PartialEq` and
  `Debug` are the name's walks (`copy_nested`, `eq_nested`,
  `render_nested`); its `Drop` and `matches` are its own.
- JSON: a writing door (`names::write_door`: save, canonical bytes,
  `to_json`) writes a name one level at a time; a read door
  (`names::read_door`: load, `from_json`, Python's name text) reads it
  one level at a time from a raw slice of its text. A malformed nested
  name refuses at its own line and column in the derived form's words
  (a hole the reader meets where no name belongs is read again in
  place). Save lays the body out as `to_string_pretty` does for every
  container within `BODY_NESTING` brackets and compact past it, so a
  file is linear in its names' depth; no file an earlier build loads
  changes a byte.
- The load door (#3510's `persist::nesting::read`) reads everything but
  names within `BODY_NESTING`, and names at any depth: a body nested
  past the limit is scanned again with every `path` array blanked, and
  the reader proves each blanked object is a name by reading it as one;
  an object it reads at any other type is counted again. One flat JSON
  walker (`persist::jsontext::tokens`) serves the scan, the blanking,
  the search inside a name's text, the pretty layout and the canonical
  bytes.
- Walks: `seam_through`, `face_descends_from`,
  `constituents_through_wrappers`, `fold_descent`, `walk_names`,
  `member_faces` and the content key's `feed_stable_name` keep their
  own stacks.

Premises checked: on `origin/main` a bare name 42 levels deep does not
read back through serde_json. A document holding a fillet selection
1 000 pattern copies deep saves, and its own load door refuses it
(`Parse`). Python's name text refuses at 50 copies. The union collapse
and the split's re-map also recursed once per level, which the row did
not list.

Filed: `work/edit/a-rank-rewrite-can-ask-the-rewriter-for-a-name-inside-another-documents-part.md`,
`work/edit/a-name-through-a-non-json-serializer-recurses-once-per-level.md`,
`work/lib/a-python-pattern-builder-copies-the-whole-pattern-per-wrap.md`.
