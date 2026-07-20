# Memory Index

- [CAD project state](cad-project-state.md) — DESIGN.md is the ratified contract; M0 + M1 COMPLETE; M2 in progress: PRs 1–2/7 merged, PR 3 implemented-unreviewed — M2-LOG's handoff-point state snapshot is the resumption contract; references live in the MAIN checkout; name pending
- [CAD working style](cad-working-style.md) — discuss → ratify into DESIGN.md → commit; propose firmly, welcome pushback; no escape hatches; fail loud
- [Evan profile](evan-profile.md) — differential-geometry fluent; define CAD jargon, don't simplify math; probes fudged invariants
- [Git workflow](git-workflow.md) — merge-only, no history rewriting; messy commits fine, documentation in PR descriptions; agents self-merge to main
- [Multi-agent capabilities](multi-agent-capabilities.md) — nested subagent spawning verified; worktree isolation for parallel implementers; custom agent types go in .claude/agents/
- [Orchestration model](orchestration-model.md) — top-level agent orchestrates/meta-reviews, subagents code+review; high-confidence design PRs self-merge (retroactive review), fundamental forks wait; commit crucial state before stopping (orchestrator-only); session-start monitors checklist
- [Orchestrator handoff](orchestrator-handoff.md) — when and how to switch orchestrators: drain the pipeline, flush state, then mngr create/capture (confirm Fable)/start --message-file/capture again
- [Review and dependency policy](review-and-dependency-policy.md) — reviews must run real e2e demos, not just read diffs; deps fine to add with ~2-week minimum release age
