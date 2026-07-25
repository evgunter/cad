---
name: subagent-death-recovery
description: When a subagent dies mid-task (usage limit, crash) — resume from transcript first; its isolation worktree under .claude/worktrees/ likely survived with all uncommitted work
metadata:
  type: feedback
---

When a subagent dies mid-task (e.g. Fable usage limit), do NOT assume
its work is lost and respawn a fresh implementer.

**Why:** During M4 PR 3 (2026-07-23) the naming implementer died at a
usage limit mid-verification. The orchestrator wrongly declared the
uncommitted editor-core half lost and spawned a replacement. Evan
asked "can the agent not be resumed from transcript?" — it could, and
the resumed agent recovered everything within minutes. Two facts the
orchestrator had missed: (1) SendMessage to a dead agent resumes it
from its transcript, which contains every edit it made, so it can
re-materialize or locate its own work far faster than a fresh agent
can redo it; (2) subagents spawned with `isolation: "worktree"` work
under `<main checkout>/.claude/worktrees/agent-<id>/` — NOT the
session scratchpad — and that worktree (including uncommitted
changes) survives the agent's death. Searching only the scratchpad
gives a false "worktree gone" conclusion; check
`git worktree list` from the main checkout.

**How to apply:** On subagent death: (1) `git worktree list` in the
main checkout to find its isolation worktree; (2) SendMessage the
dead agent id to resume it from transcript, pointing it at its
surviving worktree; (3) only spawn a fresh implementer if both the
transcript and worktree are truly gone. Standing prevention policy:
implementers must commit AND push after every coherent unit — no
batching work for a final push. See [[orchestration-model]],
[[multi-agent-capabilities]], [[worktree-disk-hygiene]].

**Resume resets cwd (2026-07-25, caught by hourly sweep)**: a
resumed subagent's Bash cwd resets to the ORCHESTRATOR worktree —
it does not stay in the agent's clone. Post-resume battery rows
without an explicit `cd <clone> && ...` IN THE SAME COMMAND run
against the wrong tree and produce green-but-invalid numbers
(main-equivalent code), silently validating nothing. Five stray
cargo processes were found building the orchestrator worktree this
way. PREVENTION: every resume/nudge message AND every prompt's
verification section must require prefixing every Bash command
with the clone cd; on any post-resume battery claim, verify the
transcript rows carried the cd before trusting numbers. The
orchestrator sweep check `readlink /proc/<cargo-pid>/cwd` catches
it live.
