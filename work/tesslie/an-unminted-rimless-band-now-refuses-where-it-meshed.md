---
id: an-unminted-rimless-band-now-refuses-where-it-meshed
kind: issue
title: the curved lane refuses every unminted rimless band DomainUnread, where it meshed them before the lift read
status: open
opened: 2026-11-21
priority: P3
cost: E
refs: [the-pole-to-pole-band-reads-its-columns-off-the-area-vector-beside-the-lift]
---

Found by PR 7303's review: a contract change, stated so it is not
silent.

Since PR 7303, `mesh::walk` reads a rimless loop's columns (the wrap strip
since PR 7202, every other pole-to-pole band since 7303) off the loop's
stored lift, through `trimmed::face_domain`. A body whose rows were never
minted (built through the Euler doors and not passed through
`topo::mint_pcurves`) therefore refuses `DomainUnread` (a lift gap) on such
a face, where before it meshed or refused another way. Measured over the
mesh and sweep suites, the rimless loops with no lift are the one-seam
sphere (#7202) and the two-coincident-edge slit (#7303), both re-pinned;
the suites' other unminted bodies (the d9 golden corpus, four probe rows)
carry rims, and their walk reads no lift. Every body a public door
finishes is minted, so no finished body moves; the change is to what
`tessellate` answers for an at-rest body assembled by hand, which its
contract does not re-validate.

Stated in the docs where the lane says it (`mesh::walk`'s module docs on
pole-to-pole bands, `TessellateError::DomainUnread`). What closing it
takes: decide whether `tessellate`'s contract should say it requires a
minted body (and the Euler-built fixtures mint before meshing), or
whether the curved lane should mint a rowless loop's lift itself. Either
way this row records the move, and the decision is the contract's.


