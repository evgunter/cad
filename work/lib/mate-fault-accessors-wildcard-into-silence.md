---
id: mate-fault-accessors-wildcard-into-silence
kind: issue
title: Ten MateFault accessors in pncad-py wildcard into None, so a new fault arm that names a mate is silently invisible
status: closed
opened: 2026-09-04
refs: [blamed-mates-lost-its-exhaustive-arm]
closed: 2026-09-08
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

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/lib/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Closed (2026-09-08, LIB-PROJ)

`crates/pncad-py/src/py/mate.rs` holds **no wildcard match at all**:
`grep -n "_ =>" crates/pncad-py/src/py/mate.rs` answers nothing.

**The seventeen `MateFault` accessors read off one record.** The
flattening is `crates/pncad-py/src/mate_payload.rs` — the
`edit_payload.rs` shape, sited outside `py/` for the reason that
module states, so the drift alarm compiles on the default no-Python
row rather than only under `--features python`. Its match over
`MateFault` is exhaustive with no wildcard, names all thirteen arms,
and says at each arm what it carries and what it does not. Seventeen
per-accessor matches would have named those thirteen arms seventeen
times and charged a new kernel arm seventeen edits; one record charges
it one. Measured: an arm added to `MateFault` kernel-side fails the
build at `crates/pncad-py/src/mate_payload.rs:178` on
`cargo check -p pncad-py` with no features.

**`Unleverable` answers `mate` and nothing else**, which is the answer
this item asked for first. It names one mate, so `mate` is `Some`; its
`refusal` is a `LeverRefusal`, a nested refusal of a type the façade
does not re-export, so it crosses as prose like the other nested
refusals and is filed as
`mate-fault-arms-carry-payload-that-does-not-cross` rather than
guessed at here.

Three more wildcards in the same file went with them, same class and
same file: `MatePrimitive::offset`, `Subgroup`'s three accessors, and
`ClusterMaintenance`'s seven. Those are few arms over few accessors,
so they became exhaustive matches in place rather than records — a
record there would have added a layer without removing a match.

The `#[non_exhaustive]` question is untouched and is still DOCM's.
Nothing in `crates/editor-core/` was edited.
