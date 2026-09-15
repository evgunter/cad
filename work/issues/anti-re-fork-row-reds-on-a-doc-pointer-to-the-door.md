---
id: anti-re-fork-row-reds-on-a-doc-pointer-to-the-door
kind: issue
title: face_normal.rs's anti-re-fork row reds on a comment that merely cites the door
status: closed
opened: 2026-09-15
closed: 2026-09-15
---


## Finding

`crates/topo/src/face_normal.rs`, `the_planar_sense_flip_lives_in_one_place`
(`:~324`): for every file under `topo/src` other than its own home, the
row asserts `!(text.contains("Surface::Plane {") && text.contains("from_chart"))`
over the file's RAW text. Doc comments are text, so a file that already
destructures a plane surface anywhere — including in a `#[cfg(test)]`
fixture — reds the moment any comment in it *names* the constructor.

Observed: SENSE-DOORS added one doc line to
`crates/topo/src/boolean/rim_wedge.rs`'s `classify_shared_rim` pointing
at `OutwardNormal::from_chart`'s doc for why the sense is a bit and not
a `T` ±1 — the pointer the door's own design asks callers to make. The
file's test fixtures build planes, so the row went red on a comment that
mints nothing. The lane worked around it by citing the TYPE instead of
its constructor and saying in the comment why, which is the perverse
incentive: the guard pushes authors away from naming the one door they
are supposed to name.

The row's own "What it cannot match" list enumerates four FALSE
NEGATIVES and is silent on this direction. Note the file is aware of the
hazard in the small — it spells the plane pattern only inside the check
so that its home is not its own first counter-example — so the fix is
the same awareness one level out: the sibling inventory row
(`every_hand_multiply_of_the_face_sign_is_inventoried`) already reads
through `test_utils::source::code_only`, which drops comments and
literal bodies, and this row reading raw text is the difference between
them. Whether `code_only` is the right instrument here is the judgement
the fixing lane owes: it would make the guard see mints and not
mentions, at the cost of a mint hidden inside a macro body or a literal.

Found by SENSE-DOORS (SCALAR, 2026-09-15). `work.py territory` reports
no program claiming `crates/topo/src/face_normal.rs` — TOPO's `keep_out`
disclaims it to S-BOOL and CURVED, and neither program's `paths` glob
covers it — so it lands here rather than on a slate that would have to
disown it.

## A second instance, in code (SENSE-FOLD, 2026-09-15)

`crates/topo/src/boolean/contact_verify.rs` mints two outward normals
from implicit gradients and the two faces' `sense` bits (the tangency
verifier's `n1`/`n2`). The fold there is CURVED — no plane is
destructured for it — but the file carries `Surface::Plane {` in a
test fixture (`:~475`), so spelling the fold as
`OutwardNormal::from_chart(..)` at the site reds the row. SENSE-FOLD
routed it through a by-value curved sibling in `face_normal.rs`
(`implicit_outward_normal`), which is a fine home for it, but the
reason it could not be written at the site is this row's coarseness
and not the design: the row cannot tell a planar re-fork from a curved
mint that happens to share a file with a planar test fixture. Whatever
instrument the fixing lane picks should distinguish the two, or the
next curved door in a file with a planar fixture pays the same detour.

## A third instance, and the guard shaping the API (SENSE-FOLD fix pass, 2026-09-15)

`crates/topo/src/readback.rs` re-worded its "form the outward normal
as `sense · axis`" instruction to name the constructor a reader should
call, `OutwardNormal::from_chart(axis, sense)` — in three doc
comments. The file destructures `Surface::Plane {` in code, so the raw
text row went red on prose again. And the second instance had already
shaped the API: `implicit_outward_normal` existed in `topo` as a
`pub(crate)` alias of a one-line `geom_brep` expression BECAUSE the
verifier's file could not name `from_chart`; the fix pass moved that
fold to its one home (`geom_brep::implicit_outward_normal`, beside
`implicit_gradient`) and the alias is gone.

## Closed (SENSE-FOLD fix pass, 2026-09-15)

`the_planar_sense_flip_lives_in_one_place` reads each file through
`test_utils::source::code_only` — the instrument the finding named —
so it sees mints and not mentions: a doc comment may name the
constructor, as the door's design asks. The row's doc says so and adds
the blind spot the view buys (a mint inside a `macro_rules!` body or
assembled from a string literal). `boolean/reduce.rs`'s detour ("not
named here: the guard row reads this file's text") is undone;
`boolean/rim_wedge.rs`'s first-instance wording cites the type and
stays correct as written.
