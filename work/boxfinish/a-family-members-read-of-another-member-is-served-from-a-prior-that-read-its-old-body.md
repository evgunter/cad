---
id: a-family-members-read-of-another-member-is-served-from-a-prior-that-read-its-old-body
kind: unit
title: A family member's read of another member is served from a prior that read its old body: a stale result on the editor's incremental path, at every scalar
status: review
opened: 2026-11-23
priority: P0
cost: M
branch: boxfinish/member-read-key
pr: 7512
---


**A live wrong answer on main**, demonstrated on 2026-11-23 by the
reviewer of PR 7470 (the node key), from a row that PR's lane filed by
reading (`a-nodes-content-key-leaves-out-inputs-its-op-reads`, on
`boxfinish/node-key` until that PR lands; its fifth bullet).

**Reproduction** (`origin/boxfinish/review-node-key` at `b6e55adbf`,
`crates/editor-core/tests/review_node_key_stale_member_read.rs`,
`a_members_read_of_another_member_is_served_from_a_prior_that_read_its_old_body`):
a free length `p`; `k = index(2)`; `tall`, a family of extrudes of
depth `0.25 + scalar(k)·p` (member 0 does not move with `p`);
`reader = Union([tall[(k+1) mod 2]])`, so `reader[0]` reads `tall[1]`.
Evaluate at `p = 0.25`, set `p = 0.5`, evaluate over the prior:
`reader`'s heights are `[0.5, 0.25]`; a fresh evaluation gives
`[0.75, 0.25]`. `reader[0]` was served the prior's body.

**Cause.** Inside a member's evaluation `members::overlay` stands the
member bound at the reader's own index in for the upstream family, so
the upstream key the reader hashes is `tall[0]`'s, which did not move;
the index value 1 is hashed, and the key of the member it picks is
not. The content key is then not the proof of what the result depends
on (DESIGN `[determinism]`), on the editor's ordinary incremental
path, at `f64` and every other scalar.

**The unit.** The key of a node that reads a family at an index
hashes the key of the member the index picks. And the same audit's
other four unhashed inputs are settled, each by construction or by
showing it cannot change between two evaluations that share a prior:
the clearance witness (the `f64` rebuild and its parameter point), the
computing twist, the boolean sweep strategy, a part row's options.
One that is live is fixed here the same way (hashed, or the prior is
refused when it differs); one that is not gets the sentence that says
why, where the key is computed.

**Ground.** `crates/editor-core/src/eval/members.rs` is TALLY's, and
`eval/mod.rs` BORDERS' and TALLY's; neither program has an
orchestrator. BOXFINISH takes it: its lanes found it, and its store of
node results per drive rests on this key.

**Letter M; single FULL review** (a demonstrated wrong answer with its
red row in hand; the claims are what else the key leaves out).
