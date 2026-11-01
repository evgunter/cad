# GATES leaves the tracker — 2026-09-08

GATES — the CI gate scripts, code-quality Track K's `scripts/gates/*` half
— opened 2026-09-06 and closed 2026-09-08 on Ev's ratification of
`docs/GATES-EXIT-WALK.md` (PR #2185, "lgtm!"). The closed-program rule (`five-closed-programs-leave-the-tracker`):
`work/gates/` whole — `program.md`, `plan.md`, `log.md` and 26 closed item
files. Twenty-six rows landed over twenty-five PRs (2029–2069, 2077, 2156,
2157, 2170, 2174). Infra-only: no A/B rows; band 3100–3199 stays
allocated. Two rows were re-homed to `work/code-quality/` first (`D212`,
`directory-prefix-skips-have-no-subject-check`), each carrying its own
note.

Sweep SHA `5ce54b35bf355241de5fa5e3bb0cfeb264bf52af`.

    git show 5ce54b35bf355241de5fa5e3bb0cfeb264bf52af:work/gates/<FILE>
    git show 5ce54b35bf355241de5fa5e3bb0cfeb264bf52af:docs/GATES-EXIT-WALK.md

Done-state of record: this note, the walk at the SHA above, and the design
at `scripts/gates/README.md` (listed in `docs/DESIGN.md`'s companion
table) and in `scripts/gates/lib.sh`'s own headers.
