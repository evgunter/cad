---
id: mate-fault-accessors-wildcard-into-silence
kind: issue
title: Ten MateFault accessors in pncad-py wildcard into None, so a new fault arm that names a mate is silently invisible
status: open
opened: 2026-09-04
refs: [blamed-mates-lost-its-exhaustive-arm]
---

Split out of `work/view/blamed-mates-lost-its-exhaustive-arm.md`, whose
code half is closed. Filed in `work/issues/` rather than in `work/lib/`
or `work/docm/` because a VIEW branch may not edit another program's
slate (`docs/prompts/implementer-discipline.md` §6). **The ground is
LIB's (`crates/pncad-py/*`) and the ratification is DOCM's
(`crates/editor-core/src/mate.rs`)**; the claim of this file is that
the row belongs on one of those boards.

## The two exhaustive matches, and the ten that are not

`MateFault` is matched exhaustively in exactly two places outside
`editor-core`, and **both got their `Unleverable` arm only because
someone happened to be looking**:

- `crates/viewer/src/tree.rs:316` — `blamed_mates`. Its arm is
  `:325`. It is exhaustive on purpose and its doc comment says why:
  a fault arm the kernel grows must decide there whether it names a
  mate, rather than falling into a wildcard and silently drawing every
  reached row as downstream of nothing.
- `crates/pncad-py/src/tags.rs:400` — the mate-fault tag function. Its
  arm is `:411`, `"mate_datum_too_small_to_lever"`.

`crates/pncad-py/src/py/mate.rs` holds **ten** accessor matches over
`MateFault`, every one of them ending `_ => None` (`:261`, and the runs
at `:524-546` and `:605-668`). (The parent item counted nine at nine
named lines; the count as of 2026-09-04 is ten. The number is not the
finding.)

## Why the wildcards are the worse half

They cannot break the build, and that is the point. **They fail the
other way**: a new fault arm that names a mate returns `None` from
every one of those accessors, silently — which is precisely the
*"drawing every reached row as downstream of nothing"* that
`blamed_mates`'s doc comment says its exhaustiveness exists to
prevent. The wildcards are not the safe choice here; they are the same
defect with the compiler switched off, and they are ten times as
numerous as the shape that was caught.

`MateFault::Unleverable` is the live test of that claim: it landed,
`blamed_mates` and `tags.rs` refused to compile until they decided,
and the ten accessors compiled unchanged and answer `None` for it
today. Whether `None` is the RIGHT answer for `Unleverable` at each of
the ten is the first thing this row owes — not a sweep to
exhaustiveness for its own sake.

## The `#[non_exhaustive]` question, which is DOCM's

The tree already has both patterns and **no stated rule for choosing
between them**: `pncad-py`'s own module doc names `select_refusal_tag`'s
enum as a documented `#[non_exhaustive]` exception (`tags.rs:34`,
`:137`, `:829`), while `MateFault` is exhaustively matched across a
crate boundary by two consumers that depend on the compiler to force a
decision. Marking `MateFault` `#[non_exhaustive]` would take that
forcing away from the two sites that want it; leaving it bare keeps a
cross-crate compile coupling that has already broken `main` once
(`work/issues/ci-draw-can-hide-a-compile-break-on-main.md`).

`crates/editor-core/src/mate.rs` is DOCM's territory, so the rule is
DOCM's to state. VIEW has no standing to pick and does not.

Signed: (VIEW orchestrator)
