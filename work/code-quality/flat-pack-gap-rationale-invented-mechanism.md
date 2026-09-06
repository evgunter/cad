---
id: flat-pack-gap-rationale-invented-mechanism
kind: issue
title: demos/tour assembly.rs: the FLAT_PACK_GAP rationale says transform_rigid re-mints face keys; topo::transform says the opposite
status: closed
opened: 2026-09-02
github: 1561
refs: [1553, 1506]
closed: 2026-09-03
branch: smell/x-prose-tracker
---

## From GitHub issue 1561

Opened 2026-09-02; 0 comments.

**Found by BOOL-13's review (PR [#1553](https://github.com/evgunter/cad/pull/1553), R2 style lane Q2), pre-existing — not that unit's to touch (`demos/tour` is outside its fence).**

`demos/tour/src/assembly.rs:369-371` justifies authoring `FLAT_PACK_GAP` into every placement frame with:

> both furniture bodies carry declared contacts, and `transform_rigid` re-mints face keys, so a moved product would carry `ContactRecords` naming faces that no longer exist.

`crates/topo/src/transform.rs` states the opposite twice — the module header ("leaving the topology — and every arena key — untouched") and the function doc at ~:363 ("identical arena keys … nothing is re-minted, so downstream key handles remain meaningful").

So the comment asserts a mechanism that does not exist. The DESIGN choice it defends (author the gap into the frames rather than move the gathered body) may still be right for another reason, but the stated reason is false. This is the same invented mechanism that commit `1806b3fb5` ("Review fallout: an invented mechanism …") swept in four places; this fifth instance was introduced at `d2085b965` and missed by that sweep.

**What the taker owes:** either restate the real reason (if one exists — e.g. montage layout, or keeping the product's own frame at the origin) or drop the sentence; and grep `demos/` and `crates/` for `re-mint` claims about `transform_rigid` so the class closes rather than the instance.

Refs #1506 (where both commits landed), #1553 (the review that found it).

## Home

`work/code-quality/` — an invented-mechanism comment with a named class sweep is prose debt, the findings register's ground; `demos/tour` is in no open program's territory.

## Closed

Landed on `smell/x-prose` (SMELL Track X), comment-only.

**The instance.** `demos/tour/src/assembly.rs:365-375` no longer argues
from a mechanism. The invented sentence is replaced by the reason the
stop's own prose already gives at `assembly.rs:1572-1576` — *"where a
layout's placements are its subject"* — plus the one consequence that
is checkable at the call site: `layout_scene` ships `assemble`'s own
output (`assembly.rs:632-640`), so a post-hoc body map would hand the
renderer a body no gate had seen, at coordinates no document records.
Nothing was invented to replace it and the behaviour is unchanged.

For the record, the mechanism was false in both halves, not only in its
premise: `ContactRecords` are pure arena keys — `VfContact { vertex,
face }`, `PatchContact { face_a, face_b }`
(`crates/topo/src/boolean/mod.rs:190-243`) — and `transform_rigid` is
key-stable, so a moved product's records would have stayed valid.

**The class sweep.** Every `transform_rigid` mention in `demos/` and
`crates/` was read with its ±12-line neighbourhood against
`re-?mint|mints?|minted|invalidat|stale|no longer (exist|valid|mean)|
fresh key|new key` joined with `keys?|FaceKey|VertexKey|EdgeKey|arena|
handle|name`, and separately against direct spellings of the false
claim (`re-mints (face|arena|…) keys`, `keys? no longer`, `naming
faces that no longer exist`, `invalidates the arena keys`).
**No other instance survives** — this was the fifth and last.

What the pattern *would* catch and what it *would not*: it catches the
claim wherever the word `transform_rigid` sits within twelve lines of
it, which is how every hit in the corpus is written. It does NOT catch
the claim made about "the rigid transform", "the placement op" or "the
body map" without the identifier; it does not catch it stated more than
twelve lines from the identifier; and it cannot tell a claim from its
correction, so every hit was read rather than counted.

What the sweep *did* find is the true neighbour of this claim, correct
everywhere it appears: rigid transforms re-mint each moved edge's
**witness** (#84, `crates/topo/src/transform.rs:34-45,434-449`) while
leaving topology and keys alone. `demos/README.md:571`,
`demos/tour/src/cutaway.rs:4,63,94`,
`demos/tour/src/crosslap.rs:106,164-165` and
`demos/tour/src/twopeg.rs:518,554-556` all say witness and are right;
`crates/editor-core/src/names/emit.rs:143,292` state the key-stability
outright. Nothing was filed on another track: the sweep reached no
path outside Track X's fence that needed an edit.

The standing correction — `transform_rigid` is key-stable, and what it
re-mints is the edge witness — needs no relocation: it survives in the
code that owns it (`crates/topo/src/transform.rs:3,363-365`,
`crates/editor-core/src/names/emit.rs:143,292`,
`demos/tour/src/twopeg.rs:554-556`).
