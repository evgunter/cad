---
id: an-opaque-minted-before-a-session-shares-a-name-with-the-sessions-first
kind: issue
title: A value with no expression minted before a session shares a name with the session's first: their difference is a proven Zero
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Found independently by both reviewers of PR 7394 (BOXFINISH, the
decide path), each with a probe (`origin/boxfinish/review-decide-path-r1`
rows p6 to p8; `origin/boxfinish/review-decide-path-r2`). Pre-existing
on main; filed by BOXFINISH.

`Sym::opaque` names a value with no expression from a count
(`OPAQUE_SEQ`) that restarts when a session starts. A value minted
before the session and the first one minted inside it therefore carry
one node id, and the tier reads them as one unknown:

- at `Sym<f64>`, `outside − inside` with values 5e-10 apart returns
  `(Zero, proven)`: a false proven zero (demonstrated);
- at `Sym<Interval>`, in band, PR 7394's enclosure `debug_assert!`
  fires on it, and is the only thing that sees it;
- with a definite numeric sign the numbers answer and nothing shows
  (the merge base's check after a definite sign asserted there; that
  check is gone by ratification).

**Reach is not established.** Whether any shipped path mints a value
outside a session and hands it into one was not traced; the node-key
unit's census found 53,065 mints inside a session and outside any
node, and none outside a session, over `editor-core`'s suite.

**In flight:** BOXFINISH's PR 7470 (`boxfinish/node-key`) re-names a
value with no expression by a scope and its ordinal, and keeps "a mint
under no scope ... for the life of the thread outside a session"; it
files the per-thread name as its own row
(`a-value-with-no-expression-minted-outside-a-session-is-named-per-thread`,
on that branch). Its reviewer is asked whether this collision is
closed, kept or changed on its head. When that PR lands, fold the two
rows into one.

**Closing test:** a value minted before a session and any value
minted inside it never cancel, at `Sym<f64>` and `Sym<Interval>`, in a
release build.

**From PR 7470's review (2026-11-23), by execution** (probes on
`origin/boxfinish/review-node-key` @ `15438db91`,
`review_node_key_mint_probes`; values `[0, 1]` so only the name
decides): the collision is KEPT on that PR's head. Probe A: a thread's
first out-of-session mint and a session's first root mint share an id,
and the session proves `Zero` of their difference. Probes F and H: two
sessions' root mints, and one scope's mint outside against inside a
session, share ids as well. With point values the numeric channel
separates first and `decide_sign`'s exact-witness `debug_assert!`
fires, so the hazard is silent only where the enclosures overlap and
in a release build. Three further shapes (a scope opened twice outside
a session; two `map_detached` calls under one scope; a scope entered
from inside itself) were new on that head and are that PR's fix pass
to close. The reviewer found no production reach: every session
opener evaluates inside its session and projects the value out.
