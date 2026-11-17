# Design fork: a cone face through its apex with a split side (TESSLANE), 2026-11-17

Protocol: ddd201f411f (`git log -1 --format=%h origin/main -- docs/DESIGN-FORK-PROTOCOL.md`, after a fetch).

Dispatch draws, two /dev/urandom bytes:

- arm byte: 10 (below 86 → three designers, otherwise two) → **three designers**
- label byte: 110 (not 255; 110 mod 3 = 2) → **Fable = C; the two Opus designers are A and B**

Problem statement handed to every designer: one text, held as `tesslane-cone-apex-split-side-problem.md` beside this file. Each designer works in its own git worktree of `tesslane/lattice-clearance` at `ba68f8ed0a` (PR 7043's head, where the faces refuse) with its own build directory, and is told to read no pull request's thread and no branch under `analysis/`.
