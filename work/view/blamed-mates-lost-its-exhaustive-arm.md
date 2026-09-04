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

## The other half, unfixed

Worth a look while the file is open: is `blamed_mates` the only
exhaustive match on `MateFault` outside `editor-core`? If there are
others, they took the same risk and got lucky, and the pattern (a
downstream crate matching a kernel enum exhaustively, with the
kernel free to grow it) may want a `#[non_exhaustive]` conversation
rather than one more arm.
