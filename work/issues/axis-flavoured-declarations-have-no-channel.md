---
id: axis-flavoured-declarations-have-no-channel
kind: issue
title: Axis-flavoured declarations (coaxial, structural-parallel) have no identity channel: ParamSource carries stored scalar fields only, so CoaxialEvidence and SPHSPH's option (a) cannot be served by it
status: open
opened: 2026-09-04
refs: [SEAT-6, 1604, 1372]
---


(SEAT orchestrator) Class finding from SEAT-6's dual review (PR 1593),
filed per the durable-home rule; both arms converged on it and it sits
outside SEAT's fence (the germ lanes are CURVED's now).

**The finding.** `ParamSource` (VERB-SEAT-DESIGN §3, P1) is lowered
*expression* identity for the stored scalar fields of minted
descriptions — `SurfaceField` names radii and a half-angle and, by
design, no placement datum (origins, axes, seam references are not
motion-invariant, and a token on one would have to compose through
rigid placement, the structure this channel deliberately does not
carry). VERBS-CYLSPH's coaxial cylinder×sphere arm (PR 1604,
`topo/src/boolean/join.rs::cs_pair_frame`) takes a `CoaxialEvidence`
whose comment says the parameter-identity channel is its honest
carrier; and §3 P2 says SPHSPH's structural-parallelism option "reads
the same channel at its own position". Neither is true as landed:
coaxiality and parallelism are claims about axes and centres —
placement data — so nothing `SurfaceField` can hold serves them. After
SEAT-6's seam merge `cs_pair_frame` is still called with
`CoaxialEvidence::None` in both operand orders and its sentence is
amended to say exactly this.

**What a fix needs.** A second declaration source for axis-flavoured
facts, distinct from field identity: either a placement-level
declaration (the document names two carriers' axes as one axis, the
way `BooleanDeclarations` names contacts) or a sketch/frame-level
identity that survives placement through composition (the
`SourceExpr::Placed` discipline `GeomSource` already has). Which of
those is right is a design question with a `[ev]` shape; P2's SPHSPH
sentence should be corrected when it is answered.

**Second-order note (SEAT-6's reviews, both arms).** Evidence reaches
`pair_section_frame` as one positional argument per typed position, so
the shared dispatch's signature grows once per consumer (two after the
CYLSPH merge). A per-pair evidence record resolved once at
`germ_section_frame` would grow by variant instead.
