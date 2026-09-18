---
id: tier3-accepts-a-ring-outside-its-outer-loop
kind: unit
title: tier 3 accepts a face whose ring lies OUTSIDE its outer loop (check 9 tests contact, not nesting): a shell_open glue with the host/guest roles inverted validates with the correct volume
status: closed
opened: 2026-09-08
pr: 2529
branch: topo/tier3-ring-nesting
closed: 2026-09-14
---


Found by SHELL-5's R1 review (PR 2159), by a scratch mutant:
`shell_open`'s rim loop (`crates/topo/src/shell.rs`, the
`(host, guest)` assignment) with a void designation treated as an
outer-shell one — `kfmrh(designated, counterpart)` where the lifted
counterpart's boundary ENCLOSES the designated face's. The glue's
precondition `validate::ring_outer_contact` reports `Disjoint` (it
decides contact, not which loop is inside), `kfmrh` makes the larger
loop a ring of the smaller face, and the verb's closing
`validate_geometric` accepts the body: tier 3 green, the flux volume
correct (the face set and windings are the same either way). Only
`verbs_shell::opening_the_hollow_boxs_void_ceiling_cups_the_inner_wall_only`'s
structural assertion "the designated void face dies" caught it; the
R1 e2e row (`shell5_r1_probes::r1_e2e_hollow_twice_then_open_the_inner_wall`)
passed on the mutant.

**Correction (this unit's fix pass, 2026-09-14).** That last sentence
is no longer true of the tree, and the premise it rests on was never
true of `shell_open`. Both blinded reviews re-installed the SHELL-5 R1
mutant at this unit's head and measured the verb, not the validator:
`shell_open` refuses the inverted pick with
`ShellError::Corrupt { key: Edge(..) }` from the naming record's
`ring_rows` walk — the glued ring's entities have no source row on an
inverted pick, and the record is built before the verb's closing
`validate_geometric` runs — so the R1 e2e row now FAILS on the mutant,
at the verb, and the closing validate never sees the inverted body.
What the unit's nesting arm buys is therefore not a refusal inside
`shell_open`; it is the statement at rest, which makes an inverted
`kfmrh` glue loud wherever else it is minted. The comment at
`shell.rs`'s `(host, guest)` assignment says this in that shape.

Consequences: (1) check 9 (`RingMeetsOuter`) is a contact check and
no tier-3 check states ring-inside-outer, so a face with a ring
surrounding its outer loop is a body every structural tier blesses;
(2) `shell.rs`'s comment at the role assignment — "the disjointness
check below and tier 3's windings are what verify it" — overclaims:
the role is verified by nothing at rest. The hole path is better off:
`pair_rings` decides nesting by mean radius and refuses the inverted
case typed (`"the designated face's hole does not sit inside the
cavity counterpart's"`, measured on the same mutant). The fix is a
nesting decide (the `encloses` shape) either in check 9 or as a second
precondition of the glue, and the comment corrected either way.
Placed by the SHELL orchestrator (2026-09-08).

## Brief (TOPO, 2026-09-13) — block TOPO-B2 slot 0, dual at review

**The answer to give.** Tier 3 states ring-inside-outer: a ring that
does not lie strictly inside its face's outer loop is a refusal
(`ValidationError`, a new variant beside `RingMeetsOuter`, typed, with
the face and ring named), decided in the face's chart domain rather
than by contact. Check 9 keeps its contact arms; the nesting decide is
its second half, or a check of its own if the file's structure reads
better that way — phase 1 decides and says why.

**Instrument (hypothesis — verify).** `shell.rs`'s `encloses` (mean
radius about the joint centroid, `decide("shell_rim_nesting")`) is the
precedent for the SHAPE and not the instrument: a mean-radius test is
not a containment statement. The kernel already has a chart-domain
parity walk (`crates/topo/src/ray_parity.rs`, TOPO's file) — read it
end to end and decide whether a ring vertex's parity against the outer
loop's projected polygon (in the face surface's chart) is the decide,
with its margin metered and escalated the way check 9's arms escalate
(`RingContactEscalated`'s shape). Every planar face is exact there;
state what the decide does on curved charts (iso-rectangle patches;
where `locus_gap` has no inversion) as an enumerated residue in the
same D4-honesty style as check 9's "NOT matched" list, not a gesture.

**Red-first row.** The SHELL-5 R1 mutant, rebuilt here as a fixture:
`kfmrh(designated, counterpart)` where the lifted counterpart's
boundary ENCLOSES the designated face's boundary, so the larger loop
becomes a ring of the smaller face; tier 3 accepts it today (assert
that on the merge base — the row is red-first), refuses typed at the
head. A control row: the correctly nested case (`ops_holed_box`) stays
green; the whole tier-3 corpus stays green (a nesting decide that
refuses a valid body is the failure mode to fear — run every tier-3
suite in `topo`, `sweep`, `editor-core` locally before pushing).

**Seams.** `shell.rs` is SHELL's: the comment at the role assignment
("the disjointness check below and tier 3's windings are what verify
it") is corrected in this PR by announced seam (one comment); whether
`shell_open`'s glue adopts the validator's decide as a second
precondition is SHELL's call — report it, do not do it. `pair_rings`'s
hole path is not touched.

**Class receipt.** Grep every tier-3 check that reads "inside",
"nesting" or "encloses" in prose and say which assert it; the
`shell.rs` comment is the one this row names.

Branch `topo/tier3-ring-nesting`. PR title: "TOPO: tier 3 states
ring-inside-outer — check 9 gains a nesting decide". Do not close the
item; the dual runs at review.
