# DOC-LEDGER sweep 7

Three sweeps carried this number. They are distinct sweeps of distinct
subjects, kept in one file so that a citation reading "DOC-LEDGER sweep 7"
resolves to one place.

## Sweep 7 — 2026-09-06: S-CERT leaves the tracker

S-CERT — certified-enclosure soundness — closed 2026-09-06 on
`docs/S-CERT-EXIT-WALK.md`, ratified by Ev's merge of `[ev]` PR #1924
(`c7b0014de`), which is ratification by the S-MATE convention. Sweep 5's
rule. Eleven files: `work/cert/`'s `program.md`, `plan.md`, `log.md`, the
last two unit rows and six closed item rows. Twenty-four items were
re-homed on the walk's own PR, ids unchanged, each carrying its own note;
the territory passed to PROPS on the same PR. A/B ordinals 700–714 stay in
`docs/MODEL-AB-LOG.md`.

Sweep SHA `b33ca36ac`.

    git show b33ca36ac:work/cert/<FILE>
    git show b33ca36ac:docs/S-CERT-EXIT-WALK.md

Done-state of record: this note and the walk at the SHA above — the plan's
eleven exit-shape clauses walked verbatim against main `37eaf5b9b` (five
MET, six MET-WITH-RECORDED-HONESTY, none CARRIED), with five points left
open with Ev and standing as walked.

## Sweep 7 — 2026-09-08: GATES leaves the tracker

GATES — the CI gate scripts, code-quality Track K's `scripts/gates/*` half
— opened 2026-09-06 and closed 2026-09-08 on Ev's ratification of
`docs/GATES-EXIT-WALK.md` (PR #2185, "lgtm!"). Sweep 5's rule:
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

## Sweep 7 — 2026-09-06: FILLET leaves the tracker

FILLET — blend completion, second pass — closed 2026-09-06 on Ev's
ratification of `docs/FILLET-EXIT-WALK.md` (PR #1973, in chat). Sweep 5's
rule: `work/fillet/` whole. Eight dualled units, ordinals 2000–2007; three
E openers; five `[ev]` rulings (PRs 1733, 1734, 1735, 1736, 1916); the H7
vocabulary ratified on PR 1819. Residue was re-homed first — to SEAT,
DOCM, FIX, PROPS and `work/issues/` (the blend kernel and the profile
fillet door had no live program at the time; those rows became BLEND's
opening slate) — each row carrying its own note.

Sweep SHA `efe21acb8f599dd146fbaadc0251dc3981ebbf9a`.

    git show efe21acb8f599dd146fbaadc0251dc3981ebbf9a:work/fillet/<FILE>
    git show efe21acb8f599dd146fbaadc0251dc3981ebbf9a:docs/FILLET-EXIT-WALK.md

Done-state of record: this note, the walk at the SHA above, the A/B rows in
`docs/MODEL-AB-LOG.md`, and the design at `crates/sweep/README.md` (A3 as
amended by H4/H5/H7), `crates/profile/README.md` (the `NoCornerOfPair`
envelope) and `crates/topo/README.md` (`rim_of`).
