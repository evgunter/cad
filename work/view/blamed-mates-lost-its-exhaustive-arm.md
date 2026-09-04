---
id: blamed-mates-lost-its-exhaustive-arm
kind: issue
title: "main does not compile at --workspace --features interval: blamed_mates lost its exhaustive arm when MateFault grew Unleverable"
status: open
opened: 2026-09-04
---

## The breakage

`main` (at `a2bb84386`) does not compile:

```
error[E0004]: non-exhaustive patterns: `&MateFault::Unleverable { .. }` not covered
   --> crates/viewer/src/tree.rs:317:11
```

`crates/editor-core/src/mate.rs:636-643` grew `MateFault::Unleverable
{ mate, refusal }` in `77f504727`. `crates/viewer/src/tree.rs:316`'s
`blamed_mates` matches `MateFault` **exhaustively and on purpose** —
its own doc comment says so:

> Exhaustive on purpose: a fault arm the kernel grows must decide here
> whether it names a mate, rather than falling into a wildcard and
> silently drawing every reached row as downstream of nothing.

The new variant did not get its arm, so the design worked exactly as
intended — it refused to compile rather than defaulting — but the
commit that added the variant landed anyway.

## Why CI did not catch it on `main`

`ci.yml`'s `filter` job draws ONE point of {lane} × {eps} × {k-lint
row} per run, and `main` push runs deliberately carry only what is
unique to them. The draw that would have built `viewer` at
`--workspace --features interval` did not come up on the runs between
`77f504727` and now, so a red tree reported green. Every PR whose
merge ref draws the interval lane inherits the failure — which is how
it was found (FILLET-E3, PR 1763, run 33840944595, job `build +
archive (interval)`).

This is the interesting half of the finding: **the sampling can hide a
hard compile break on `main` for an unbounded number of merges**, and
nothing in the sampling argument accounts for that. A compile of every
crate is not a "configuration" in the sense the sampling note means —
it is the precondition for any of the sampled rows to mean anything.
Whether the build tier should be exempt from the draw is a CI
question, not this issue's, but it is the question this issue raises.

## The repair — landed twice, identically, independently

FILLET-E3 (PR 1763) wrote the arm because its head asks for
`lane=interval` and it could not otherwise report a green run.
Independently and within the same hour, `main` landed the **identical
arm** — `| MateFault::Unleverable { mate, .. } => vec![*mate]` — which
showed up as a merge conflict against E3's copy. E3 took main's
version; the file is now byte-identical to main's and no FILLET
change survives in it.

That the two were written the same way without contact is the useful
evidence here: the decision the exhaustive match demands is **forced by
the variant's shape**. `Unleverable`'s first field is documented "The
mate.", it names exactly one, and it is the mate whose datum is too
small to lever the verdict — structurally identical to
`ClassNotAdmitted`, `SelfMate` and the six others in that group. The
only arms deciding otherwise are `Band` (names no mate) and
`Contradictory` (names two).

So the code half of this issue is CLOSED on arrival. What remains open
is the process half below, which the duplicate work also demonstrates:
two agents spent effort on one line because a red `main` was invisible
until each independently drew the lane that builds it.

## The other half — swept (VIEW orchestrator, 2026-09-04)

The question was whether `blamed_mates` is the only exhaustive match on
`MateFault` outside `editor-core`. **It is not: there are two, and the
second is not this program's.**

- `crates/viewer/src/tree.rs:316` — `blamed_mates`, this issue's.
- `crates/pncad-py/src/tags.rs:400` — the mate-fault tag function, and
  **LIB's** ground (`crates/pncad-py/*`). It took the identical risk
  and it did get its arm: `MateFault::Unleverable { .. } =>
  "mate_datum_too_small_to_lever"` at `:411`. So both exhaustive
  matches are correct as of this sweep, and both were repaired by
  someone who happened to be looking.

Everything else that names `MateFault` outside `editor-core` wildcards:
`crates/pncad-py/src/py/mate.rs` (eight `_ => None` arms at :261, :524,
:535, :546, :605, :614, :623, :632, :641) and
`crates/viewer/src/app.rs:2880`. Those cannot break the build — and
that is the point worth carrying, because **they fail the other way**:
a new fault arm that names a mate returns `None` from every one of
those accessors, silently, which is exactly the "drawing every reached
row as downstream of nothing" that `blamed_mates`'s doc comment says
its exhaustiveness exists to prevent. The wildcards are not the safe
choice here; they are the same defect with the compiler switched off.

**The `#[non_exhaustive]` question is real and is not VIEW's to answer.**
`crates/editor-core/src/mate.rs` is DOCM's glob, and there is already a
convention to argue from: `pncad-py`'s own module doc names
`select_refusal_tag`'s enum as a documented `#[non_exhaustive]`
exception (`tags.rs:34`, `:137`), so the tree has both patterns and no
stated rule for choosing. Announced to DOCM and LIB rather than
decided here.

**The CI half stays open and is CIW's**: a draw that can hide a hard
compile break on `main` for an unbounded number of merges. This issue
states it well and this program is not the owner; the announce is
owed with the others.
