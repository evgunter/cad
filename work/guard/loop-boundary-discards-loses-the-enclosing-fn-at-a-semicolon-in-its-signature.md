---
id: loop-boundary-discards-loses-the-enclosing-fn-at-a-semicolon-in-its-signature
kind: issue
title: loop-boundary-discards keys a discard to the wrong fn when its fn's signature holds a ';' (an array type), which resets the pending name before the body opens
status: open
opened: 2026-10-01
---


## What

Found by TOPO PR 3592, whose test helper `review_d18::far_loop` was
declared `fn far_loop(body: &Body<f64>, ends: [VertexKey; 2]) -> … {`
with a let-else discard in its body. `scripts/gates/loop-boundary-discards.sh`
reported it as `UNREG|crates/topo/src/review_d18.rs|…|(no enclosing fn)|…`.

The enclosing-fn reader (the `phase == 1` block of the gate's awk
program) records a `fn <name>` as `pending`, and pushes it when the
next `{` opens. It clears `pending` on any `;`, so that a bodiless
declaration (`fn f();` in a trait) is not pushed. The `;` inside an
array type in the signature (`[T; N]`) clears it too. The body's `{`
then pushes nothing, and the discard is keyed to the next fn out, or
to `(no enclosing fn)` at file level.

The gate still reds on the site, so nothing passes silently. But the
register key it asks for names the wrong owner, and an entry written
to that key would absorb a later discard in the outer fn. The
header's "WHAT THE MATCHER CANNOT SEE" list does not name this case.
PR 3592 changed `far_loop` to take a slice, and registered it under
its own name.

## Shape

Clear `pending` on a `;` only at bracket and paren depth 0, or only
when no `(` of the signature is still open. Then add a self-test case
that plants a discard in a fn whose signature holds `[T; N]`.
