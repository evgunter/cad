# Sweep 9 — 2026-09-08: EVAL leaves the tracker

EVAL — the evaluation seat (`crates/editor-core/src/eval/*`, the verb seat,
the names emitters, `topo::query`/`flush`) — opened 2026-09-06 and closed
2026-09-08 on Ev's ratification of `docs/EVAL-EXIT-WALK.md` (PR #2201,
"lgtm!", merged `8d34121c7`). Sweep 5's rule: `work/eval/` whole. Eleven E
units (PRs 2139, 2153, 2160, 2165, 2168, 2173, 2176, 2186, 2190, 2194,
2195), no A/B rows (band 3000–3099 claimed and unused), two `[ev]` rulings
(PRs 2137, 2138). Five rows were re-homed first, each carrying its own
note. Seven EVAL unit specs that outlived their merges leave at this sweep
and have their own notes in this directory.

Sweep SHA `9c515cb150e8f396c6f197c2354650e8e676e11d`.

    git show 9c515cb150e8f396c6f197c2354650e8e676e11d:work/eval/<FILE>
    git show 9c515cb150e8f396c6f197c2354650e8e676e11d:docs/EVAL-EXIT-WALK.md

Done-state of record: this note, the walk at the SHA above, the units' PRs,
and the design at `docs/DESIGN.md` Band 1 (amended by PR 2201),
`crates/profile/README.md` (V6), `crates/verbs/README.md` (the `Verb`
convention) and `eval/mod.rs`'s `mod tag` (format 7).
