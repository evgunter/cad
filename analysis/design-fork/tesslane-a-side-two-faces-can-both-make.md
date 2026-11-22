# Design fork: a side two faces can both make (TESSLANE), 2026-11-22

Protocol: ddd201f411f (`git log -1 --format=%h origin/main -- docs/DESIGN-FORK-PROTOCOL.md`, after a fetch).

Dispatch draws, two /dev/urandom bytes:

- arm byte: 82 (below 86 → three designers, otherwise two) → **three designers**
- label byte: 152 (not 255; 152 mod 3 = 2) → **Fable = C; the two Opus designers are A and B**

Problem statement handed to every designer: one text, held as `tesslane-a-side-two-faces-can-both-make-problem.md` beside this file. Each designer works in its own git worktree of `tesslane/ruled-density` at `a020a3edca` (PR 7096's closing head, not yet merged: its hosted run was queued behind about 90 others) and is told to read no pull request's page or thread and no branch under `analysis/`. The three share one build directory (11 GB of disk free at dispatch).

The row this fork weighs: `work/tesslane/the-mesh-cures-a-side-two-faces-can-both-make-three-ways.md` (P1, design), filed at PR 7096's closing.

## First reports

