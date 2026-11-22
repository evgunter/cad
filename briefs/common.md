## Standing rules for this lane (HELDRECORDS orchestrator)

- Read `CLAUDE.md`, then `docs/prompts/implementer-discipline.md` IN FULL, by path; it binds you. Read the row file(s) named above in full, and `crates/topo/README.md`'s tier 3 / tier 3′ sections.
- Foreground rule, both halves: never arm waiters or background chains for your own builds/tests and never end a turn with background work running; and launch any job that would outlive a 600 s foreground call `setsid`-detached (record its PID), then poll its output file in the foreground. Hosted CI waits are polled in the foreground too.
- One `CARGO_TARGET_DIR` of your own, outside the checkout (e.g. `/home/user/<lane>-target`).
- Commit and push after every coherent step (merge-only: no rebase, no squash, no force-push; bring main in by `git merge origin/main`).
- Account identifiers stay off GitHub: no email address or personal identifier in commits, PRs or files, except `evgunter` / `evgunter@gmail.com`. No model names in commits, PR text or code.
- Fable is unavailable: you are on Opus. The model-comparison and design-fork experiments are NOT run this sitting: write nothing in `docs/MODEL-COMPARE-LOG.md` or `docs/DESIGN-FORK-LOG.md`.
- Do not edit `work/heldrecords/log.md` or the row's status header; the orchestrator writes state-sync. You MAY add a dated `## <date> — <what>` section to the row(s) you work, and you MUST file findings outside your fence as rows (`python3 scripts/work.py new ...`, discipline §6); `python3 scripts/work.py lint` must pass.
- Before opening the PR: `git fetch origin && git merge origin/main`, and run `python3 scripts/work.py territory --base origin/main`; name coupled seams in the PR body.
- Open the PR non-draft against the base named above, body per the discipline (what, why, sweep hit-list with dispositions and blind spot, pins, what moved). Wait for `gate ok` (poll in the foreground), fix reds.
- A design fork you cannot settle from ratified text (`docs/DESIGN.md`, `crates/topo/README.md`) is not yours to decide: stop and report it with the options you see. Do not change ratified text's decisions.
- When done (PR open, `gate ok` green) or blocked, report to the orchestrator with `mcp__claude-code-remote__send_message` to session `session_018u3gVKp4QiofNJ9Z1mzgD8`, ≤150 lines: PR number, head sha, what was built, what moved, rows filed, what was not done. Then stop; do not merge.
