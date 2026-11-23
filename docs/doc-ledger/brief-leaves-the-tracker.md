# BRIEF leaves the tracker — 2026-10-26

BRIEF (the standing discipline: `docs/prompts/` and the memories it
leans on) was cut from META on 2026-10-26 and closed the same day. Ev
ruled on every one of its four rows in chat, and each row's `## Closed`
section quotes the ruling:

- `four-lane-practices-have-no-home-outside-the-ledger`: all four
  proposed practices declined. Nothing goes into `docs/prompts/`.
- `an-items-stated-sweep-pattern-may-not-match-its-own-instance`:
  declined.
- `unit-numbered-test-suite-prefixes-collide-across-program-generations`:
  superseded, because BLEND has closed and no live collision remains.
- `orchestration-model-prescribes-a-usage-instrument-with-no-file`:
  the instruction was deleted from `memories/orchestration-model.md`.

`docs/prompts/*` returns to META's `paths`. The program set no
`## Exit criteria`, so it closes without an exit walk.

Recover the rows and the program files from the commit that deletes
`work/brief/` (its parent holds them):

    git log --diff-filter=D --format=%H -1 -- work/brief/program.md
    git show <that sha>^:work/brief/<FILE>
