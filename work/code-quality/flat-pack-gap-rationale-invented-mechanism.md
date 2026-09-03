---
id: flat-pack-gap-rationale-invented-mechanism
kind: issue
title: "demos/tour assembly.rs: the FLAT_PACK_GAP rationale says transform_rigid re-mints face keys; topo::transform says the opposite"
status: open
opened: 2026-09-02
github: 1561
refs: [1553, 1506]
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
