---
id: mate-lever-needs-the-parts-extent
kind: issue
title: A mate's lever arm cannot reach the mated parts' extent
status: open
opened: 2026-09-03
---

ERROR-DESIGN E3's amendment (ratified at revision E12) replaces the
`max(·, 1 m)` lever with an UPPER BOUND ON THE OPERANDS' EXTENT, with no
floor, and names two sites: `editor_core::eval::measure`'s `arm` and its
sibling `editor_core::mate::Alignment::lever_arm`.

**The measure site ships the amendment whole.** Its operands are FACES,
a `Carrier` now carries the face's own reach (the boundary walk in
`reach_of`), and a validated face has positive extent by construction —
so there is no floor and no predicate minted to replace one.

**The mate site ships half of it.** A mate's operands are the mated
PARTS, which have extent too — but an `Alignment` carries only the
authored DATUM (`MateFrame` origins, `MatePrimitive::authored_lengths`),
and a datum authored at the origin with no length is the COMMON
spelling: `Coaxial` on two frames at their parts' own origins names no
scale at all. Levering that at zero prices every tilt at zero and reads
every pair as parallel — an answer, in the direction that reports rather
than refuses.

MEASURED (M10-7, on this tree): removing the floor outright turns twelve
rows of `crates/editor-core/tests/asm_r2a_mate_solve.rs` into refusals,
every one of them a document a user may legitimately author.

**What shipped** (M10-7, deviation D2): the datum's own extent wherever
it has one — so no absolute constant participates for any mate that
names a scale, which was the amendment's actual complaint — and the
metre only where the datum names no length at all, documented as D4 ¶4's
session-box order of magnitude rather than as a lever.

**What is owed**: the mated parts' extent reaching this door. It is not
a one-line change — `solve_document` works over the recipe's structure
with no geometry in hand (A11's "no geometry inspection"), so the extent
would have to be authored beside the datum or resolved through the part
store, and either is a schema question rather than a plumbing one.
