---
id: viewer-free-move-misses-pattern-placed-mates
kind: issue
title: Free-move admission misses pattern-placed mates — mates_naming is instance-keyed against a member-keyed vocabulary
status: open
opened: 2026-09-04
---


## The gap

`crates/viewer/src/display.rs:181` `mates_naming` scans for
`a.node == instance || b.node == instance`. A mate reference's HEAD is
the node the pick was on — for a pattern copy that is the PATTERN node,
not the instance the member stands on
(`crates/editor-core/src/mate/solve.rs:159` `member_of`: a copy's member
is the pattern's INPUT instance). So a mate authored on a pattern copy
is invisible to `mates_naming(doc, <that pattern's input instance>)`.

`free_move_check` (`display.rs:351`) is the consumer that matters: it
refuses `MateConstrained` exactly when a mate names the instance, and
the module's own sentence is that an instance a mate names has no free
pose to probe. A pattern copy's mate DOES fix its master's pose — the
copy rides the master, and the solve places the master through it — so
free-move is offered on an instance the solve already constrains, which
is the accepted-but-inert op `display.rs`'s header says G3 forbids.

Measured on this branch (throwaway row, not committed): after the mate
tool commits one mate on copy 1 of a circular pattern over `post_b`,

```
mates_naming(post_b) = []; free_move_check(post_b) = Ok(())
```

## Why it is the same class as issue 1412

1412 was an instance-keyed predicate (`is_instance`) gating a door whose
vocabulary is member-keyed. This is the same shape one function over: an
instance-keyed SCAN answering a question whose subject is a member. The
fix presumably reads the member vocabulary here too —
`member_of(doc, a).map(|m| m.instance) == Some(instance)` rather than
`a.node == instance` — but that is a BEHAVIOUR change to the free-move
admission gate with its own evidence to write, so it is not folded into
PR 1748 (whose scope is the mate tool's pick gate).

Not swept beyond `crates/viewer/src`: the pattern I searched was
`grep -i instance` over that directory, which finds prose and the three
`a.node ==` shapes but would miss any instance-keyed scan spelled
without the word.

Signed: (CHROME fix-pass lane, PR 1748 style review)

## Added by the FIX orchestrator (2026-09-04)

FIX's PR 1749 review found this independently, hours apart, and filed a
duplicate that has been deleted in favour of this file — this one has
the executed row and precedence on main. Four things from that reading
that are not above:

**1. Neither door catches it, and they fail differently.**
`free_move_check(instance)` returns `Ok` because the scan misses (this
issue). `free_move_check(pattern)` refuses `NotAnInstance`, because a
pattern node is not an instance. So a user reaching for the copy and a
user reaching for the master both get a wrong answer, by two unrelated
mechanisms — which is why reading either door alone does not show the
hole.

**2. The fix has a door now.** PR 1749 (FIX,
`split-crossings-skip-pattern-mate-ends`) made
`editor_core::mate::member_of_head` public for exactly this reason: it
collapsed three spellings of "is this name's head a member?" onto one
predicate, after establishing that a second hand-written spelling is
not merely redundant but *harmful* (a gate matching the head's spelling
rather than the vocabulary mints an interface crossing for a mate that
never solved). `mates_naming` is the fourth spelling and the one that
was live. Resolving each reference's head through `member_of_head` and
comparing MEMBERS is the shape; the behaviour change this issue names
is still owed its own evidence.

**3. The distinction from 1412, sharpened.** 1412 refuses something it
should allow (the pick gate excludes heads the rider admits); this
allows something it should refuse. Opposite directions, same key error,
which is why fixing 1412 does not touch this.

**4. Where to look next**, generalising this issue's stated blind spot
(`grep -i instance`, which misses any instance-keyed scan spelled
without the word): **any site comparing `name.node` to an instance id.**
That pattern catches the shape rather than the vocabulary, and it is
what found this one — the line mentions neither `InstantiatePart` nor
`Pattern`, so no textual sweep for the member vocabulary reaches it.

Cross-refs: `mate-member-vocabulary-restated-in-refactor` (the same
class, `refactor.rs`, discharged by PR 1749).
