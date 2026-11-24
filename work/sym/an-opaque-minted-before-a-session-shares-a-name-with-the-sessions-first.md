---
id: an-opaque-minted-before-a-session-shares-a-name-with-the-sessions-first
kind: issue
title: A value with no expression minted before a session shares a name with the session's first: their difference is a proven Zero
status: closed
opened: 2026-11-23
priority: P1
cost: M
closed: 2026-11-24
pr: 7470
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

**After PR 7470's fix pass** (`geom_core::sym::mint`, rows in
`geom-core`'s `a_value_with_no_expression_is_named_where_it_was_minted`).
Outside a session a value's name is its minting thread's token, unique
in the process, and that thread's count of such mints, which never
restarts and which no scope enters; inside a session it is the open
mint scope and its ordinal, and a session's root scope is a name of its
own.

- **Closed**, each by a row red on the reviewed head `090c8510`:
  probe A and probe H
  (`a_value_minted_before_a_session_and_one_minted_inside_it_do_not_cancel`),
  the three shapes new on that head (`outside_a_session_a_thread_gives_no_name_twice`,
  `two_detached_maps_under_one_scope_share_no_name`), and two threads'
  mints outside any session, which the token separates.
- **Kept**: probe F and its scoped form. Two sessions give one scope's
  k-th mint one name (it is what makes a node's names the same in every
  replay), so a value carried from one session into another by any
  route but a node's result served under its content key can meet its
  namesake. No caller does that. A name unique across sessions would
  not be the same in two replays, and decisions are taken over
  root-scope values (the reads after an evaluation: 53,065 such mints
  over `editor-core`'s suite).
- **The cost**: a name given outside a session is not the same in two
  runs (which thread mints is the schedule's).

I agree with the reviewer on reach, by the session openers:
`drive::classify`, the K probe's symbolic pass, `eval::leaf::replay_leaf`
and `read_leaf_with`, `coincide`'s replay session, the tour's
`chaintol`, and the kernel's own (`topo`'s and `sweep`'s) each build
and decide inside the session they open and hand out numbers.
The row PR 7470 first filed beside this one
(`a-value-with-no-expression-minted-outside-a-session-is-named-per-thread`)
is not landed: what it described is closed, and what is kept is here.

## Closed

PR 7470 (2026-11-24): the section above. What was a hazard is closed by
rows red on that PR's reviewed head; what is kept (two sessions' names
agree) is the replay determinism the mint is for, and is stated in
`geom_core::sym::mint`'s module doc. (BOXFINISH orchestrator)
