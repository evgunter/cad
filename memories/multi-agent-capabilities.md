---
name: multi-agent-capabilities
description: Nested subagent spawning is available and empirically verified in this environment
metadata:
  type: reference
---

Verified 2026-07-15 by live test: subagents can spawn their own subagents
(main agent → general-purpose → Explore chain worked, correct results
propagated back up). `claude` and `general-purpose` subagent types carry
the full toolset including `Agent`, so delegation trees recurse through
them; `Explore`/`Plan` lack the `Agent` tool and are leaves. Subagents
support `isolation: "worktree"` — parallel implementation agents each get
their own git worktree, composing with the merge-only PR workflow
([[git-workflow]]). The `Workflow` tool's child workflows are limited to
one nesting level, but agents inside workflows can still use `Agent`
normally. No custom agent types defined yet; add them as
`.claude/agents/*.md` in-repo if specialized roles are wanted (they
version-control like [[cad-project-state]]'s conventions).
