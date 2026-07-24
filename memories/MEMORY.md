# Memory Index

- [Boolean consumer findings (M3 PR 5)](boolean-consumer-findings.md) — historical record of the demo tour's PR 5-era findings; finding 1 (silent wrong component) RESOLVED by the PR 5 fix pass; chord re-description workaround OBSOLETE post-PR 5.5 (raw extrudes work — review_m3_pr55_e2e)
- [CAD project state](cad-project-state.md) — DESIGN.md is the ratified contract; M0–M3 ALL COMPLETE (M3 exit 13/13, 2026-07-23); NAMING-DESIGN #74 + SOLVER-DESIGN #79 ratified; pre-M4 design DONE (NAMING #74 + SOLVER #79); next = M4-PLAN ratification with Evan; merge gate = scripts/gate.sh (hosted CI DOWN, --auto merges immediately); references live in the MAIN checkout; name pending
- [CAD working style](cad-working-style.md) — discuss → ratify into DESIGN.md → commit; propose firmly, welcome pushback; no escape hatches; fail loud
- [Evan profile](evan-profile.md) — differential-geometry fluent; define CAD jargon, don't simplify math; probes fudged invariants
- [Git workflow](git-workflow.md) — merge-only, no history rewriting; messy commits fine, documentation in PR descriptions; agents self-merge to main
- [Interval square poison](interval-square-poison.md) — interval squares of possibly-zero quantities MUST use powi(2), never x*x (spurious negative lo → sqrt poison); 3 occurrences in M2
- [Name candidates (Q9)](name-candidates.md) — Evan's shortlist Intension / Noumenon / Selvage with justifications; full slate + availability 2026-07-23; revisit before Q9 ratification
- [Multi-agent capabilities](multi-agent-capabilities.md) — nested subagent spawning verified; worktree isolation for parallel implementers; custom agent types go in .claude/agents/
- [Orchestration model](orchestration-model.md) — top-level agent orchestrates/meta-reviews, subagents code+review; high-confidence design PRs self-merge (retroactive review), fundamental forks wait; commit crucial state before stopping (orchestrator-only); session-start monitors checklist
- [Orchestrator handoff](orchestrator-handoff.md) — when and how to switch orchestrators: drain the pipeline, flush state, then mngr create/capture (confirm Fable)/start --message-file/capture again
- [Review and dependency policy](review-and-dependency-policy.md) — reviews must run real e2e demos, not just read diffs; deps fine to add with ~2-week minimum release age
- [Worktree disk hygiene](worktree-disk-hygiene.md) — each worktree's target/ is 4-8 GB and cargo can't safely share artifacts across parallel builds; remove merged-branch worktrees at every pipeline seam
- [Subagent death recovery](subagent-death-recovery.md) — resume dead agents from transcript first; isolation worktrees under .claude/worktrees/ survive with uncommitted work; implementers push after every unit
- [Clone placement](clone-placement.md) — working git clones never in /tmp scratchpad; use ~/.local/share/cad-work/ or .claude/worktrees/; scratchpad = disposable artifacts only
