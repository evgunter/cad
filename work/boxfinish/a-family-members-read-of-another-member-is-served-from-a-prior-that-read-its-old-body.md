---
id: a-family-members-read-of-another-member-is-served-from-a-prior-that-read-its-old-body
kind: unit
title: A family member's read of another member is served from a prior that read its old body: a stale result on the editor's incremental path, at every scalar
status: closed
opened: 2026-11-23
priority: P0
cost: M
branch: boxfinish/member-read-key
pr: 7512
closed: 2026-11-24
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

## Closed

PR 7512 (2026-11-24). A node's content and naming keys are derived from
one list of its reads, each as the read sees it
(`eval::wire::seen_reads`, a `SeenRead` per read: the results' entry for
a plain read, the family whole, or the member an index picks, with the
picked member's place). So a member's read of another member keys on
that member, and an evaluation over a prior recomputes it when that
member moved; twenty read rows are red on main without it. A clearance's
verdict that cites a point is served from a prior only where the
rebuild's point is this run's (`ClearanceVerdict::point`), and a holding
verdict is served across boxes. A prior run under another twist, sweep
or symbol setting is refused at the door (`PriorRefusal::Options`); the
tessellation memo compares the computing frame too. The format version
is 25.

What it left, each with its own row:
`a-nodes-op-and-its-keys-walk-its-reads-separately` (the op still
resolves its reads at its own call sites, held equal to the list by
rows); `a-clearances-rebuild-is-fixed-by-its-point-only-while-expressions-agree`
(by reading: an edited expression of the same enclosure and nominal
rebuilds other bodies under the same keys); on FORMWORDS,
`a-one-body-node-reading-two-members-of-one-family-reaches-an-unreachable`.
