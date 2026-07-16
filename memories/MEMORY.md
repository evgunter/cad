# Memory Index

- [CAD project state](cad-project-state.md) — DESIGN.md is the ratified contract (D1–D9); M0 COMPLETE 2026-07-16 (geom-core + topo, all Q1 residue ratified, only K's value open); M1 next (wants missing Mäntylä chapters); name pending
- [CAD working style](cad-working-style.md) — discuss → ratify into DESIGN.md → commit; propose firmly, welcome pushback; no escape hatches; fail loud
- [Evan profile](evan-profile.md) — differential-geometry fluent; define CAD jargon, don't simplify math; probes fudged invariants
- [Git workflow](git-workflow.md) — merge-only, no history rewriting; messy commits fine, documentation in PR descriptions; agents self-merge to main
- [Multi-agent capabilities](multi-agent-capabilities.md) — nested subagent spawning verified; worktree isolation for parallel implementers; custom agent types go in .claude/agents/
- [Orchestration model](orchestration-model.md) — top-level agent orchestrates/meta-reviews, subagents code+review; continue until real design forks; commit crucial state before stopping (orchestrator-only)
- [Review and dependency policy](review-and-dependency-policy.md) — reviews must run real e2e demos, not just read diffs; deps fine to add with ~2-week minimum release age
