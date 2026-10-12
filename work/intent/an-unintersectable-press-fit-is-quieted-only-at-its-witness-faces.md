---
id: an-unintersectable-press-fit-is-quieted-only-at-its-witness-faces
kind: issue
title: An overlap the kernel cannot intersect is sited only at the census's witness faces, so a press fit whose pin passes right through stays unquietable
status: closed
opened: 2026-10-08
closed: 2026-10-08
priority: P0
design: true
refs: [interference-at-rest-is-a-finding]
---

A deliberate press fit: a pin in a bore with 2 µm interference,
asserted `Gap(bore, pin) ≤ −2 µm`. The census finds a bore-rim vertex
inside the pin, but intersecting the two copies gives a 2 µm
cylindrical shell, which can refuse as a sliver or on a curved pair the
join has no arm for. Stage 5 B's site is the overlap's bounding faces
(FORK-S5-2 (b)), so with the intersection refused the finding had no
site and the intended press fit could not be quieted.

D10 and `docs/INTENT-STAGE5-SPEC.md` §3 now site such a finding at the
faces incident to the census's witnesses (FORK-S5-2 (c)), and an
assertion whose two faces, one of each copy, are among them quiets it.
That is partial: the witness is the first vertex or pierce event in
arena order, and a pin that passes right through the bore is incident
to no witness with its own cylindrical face, so its press fit stays
loud.

The full fix is a site for an overlap the kernel cannot intersect that
does not depend on which witness came first: every face of each copy
the census's containment and pierce tests touch for that pair, or an
intersection that does not refuse at a thin shell (a localisation that
reports the bounding face pair without building the shell). Weighed by
a designer pair before it is built; lands with or after stage 5 B.

## Closed

Not done, by Ev's ruling (2026-10-08): "re your sliver unquietable
example, tbh that sounds like the problem is eps too big? if a press fit
is eps scale then there's really nothing sensible we can do, because our
representation might be far enough off to change things meaningfully.
so uh idk if it should be quietable, hm". A sliver refusal means the
overlap is about ε thick, where the representation cannot tell a press
fit from contact or a gap and cannot verify `Gap ≤ −2 µm` either; a
press fit well above ε intersects. No witness-face site is built. What
remains, an overlap on a curved pair the join has no arm for, is DS6's
capability frontier (loud, unquietable, never refusing), and the join's
missing curved arms are already tracked on their own rows (for example
`work/sect/cylinder-sphere-germ-pair-has-no-join-lane.md`,
`work/sect/along-edge-ring-on-a-curved-face-has-no-join-arm.md`,
`a-sphere-crossing-a-sphere-face-off-every-edge-refuses-spheres-meet` (JOIN, closed by PR 4344)).
