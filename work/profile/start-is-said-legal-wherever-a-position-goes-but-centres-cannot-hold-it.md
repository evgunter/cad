---
id: start-is-said-legal-wherever-a-position-goes-but-centres-cannot-hold-it
kind: issue
title: PATHS-DESIGN §2 says Start is legal wherever a position argument goes, but Center.c, Via.q and Circle.centre are [S; 2] and cannot hold it
status: open
opened: 2026-11-02
priority: P4
cost: C
---

Found by a designer weighing `work/vseam/a-profile-cannot-refer-to-its-own-earlier-parts`
(2026-11-02); filed by the VSEAM orchestrator.

`docs/PATHS-DESIGN.md` §2 says `Start` "is legal wherever a … position
argument goes". In editor-core, `ProgramArcData::Center.c`, `Via.q` and
`LoopProgram::Circle.centre` are `[S; 2]`, so they cannot hold it, and
the code accepts `Start` only as a target. Either the sentence narrows
to targets, or the in-profile reference design (which proposes `Start`
as a centre) makes it true. Wait on that row's ruling before editing.
