# LIB-U10 spec — docs, tutorials, corpus-as-examples (binding)

Mandate: LIBRARY-DESIGN §L5 U10, the program's FINAL unit — the
docs-and-onboarding deliverable: "the corpus is the example set;
the tour's run_body ladder (validate → measure → tessellate →
cross-check → export) documented as the canonical user journey."
Plus the recorded north star (LIB-LOG, Evan 2026-08-09): every
demo authorable through the Python bindings — U10 documents what
IS true today and pins the gap list for what is not. The Q9
rename is NOT this unit (placeholder-until-last-minute stands;
`pncad` appears throughout, greppable by design).

## 0. Discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first. Slot
rules: `scripts/with-build-slot.sh -- cargo ...`; `--express
SECS` ≤10-min rows; long rows default mutex, BLOCKING foreground
waits (timeout 590000, re-issue; setsid+poll past the cap);
NEVER park. Cold clippy both lanes + greps BEFORE opening.
Commit AND push per chunk. NO Co-Authored-By, no model names.
Merge origin/main before opening; confirm checks STARTED.

## 1. Fence

In scope: rustdoc across `crates/pncad` + `crates/quantity` +
`crates/pncad-py` (crate/module-level docs, doctested examples);
a new top-level `docs/GUIDE.md` (or a docs/guide/ set — measured
call); the pncad-py README + .pyi docstrings; `demos/tour`
narration/doc comments where they SERVE the guide (the
demo-purpose block governs); the corpus docs' doc-comments.
OUT: code behavior changes of ANY kind (docs-only unit — any
example that needs a code change is a FINDING, not an edit);
CI; renders; the Q9 rename.

## 2. Deliverables

1. **The canonical journey, written once**: GUIDE.md walks the
   run_body ladder — author (PATHS algebra) → validate (the
   tier ladder AS the user journey, per §L6) → measure
   (mass_properties with pads) → tessellate → cross-check →
   export — in BOTH languages side by side (Rust via pncad,
   Python via pncad-py), using a corpus body as the worked
   example. Every code block is a doctest or a tested example
   file (nothing untested in the guide — the U1/pncad doctest
   convention scaled up).
2. **The corpus as the example set**: an index (in GUIDE.md or
   adjacent) mapping every tour scene + corpus doc to what it
   demonstrates (the authoring feature, the kernel behavior, the
   pitfall it pins) — the map a newcomer browses. plate_param is
   the parametric flagship; bracket.py the Python flagship.
3. **The fail-loud tour**: a section documenting the refusal
   experience as a FEATURE — what typed refusals look like at
   each layer (authoring, replay, edit-door, evaluation,
   validation, contact), with executed examples of reading them.
   The library's differentiator, presented as such.
4. **The north-star audit**: a table of every tour scene —
   authorable through Python today? (yes / no + the named gap
   feeding the gap list). No new doors built; the audit IS the
   deliverable. Cite the demo-purpose rule.
5. **Crate-front docs**: pncad's lib.rs doc becomes the real
   front door (what this is, the four legs, where the guide
   lives); quantity + pncad-py fronts likewise; stale
   placeholder prose (the U1-era "real docs are U10" notes)
   replaced everywhere — grep for them.
6. **Onboarding**: a QUICKSTART section (install/build incl.
   maturin for Python, first model in ten lines each language,
   where to go next). State the placeholder-name situation in
   one honest line.

## 3. Acceptance

- Every guide code block executes (doctests green; example
  files run in the batteries or a documented runner).
- Byte-identity: zero behavior diffs anywhere (docs-only —
  exports/tests unchanged; tour narration edits only under the
  demo-purpose rule, listed).
- The north-star audit's yes-rows each verified by an executed
  Python snippet; the no-rows each name their gap with a
  pointer (issue or LIB-LOG item).
- Cold clippy both lanes (doctests compile); stub/name checks
  green.
- Report the doc-rot risks found (stale claims in existing
  rustdoc contradicted by merged reality) — fixed in-unit where
  docs-only, findings where not.

## 4. PR discipline

One PR. Report ≤150 lines to
`~/.local/share/cad-work/lib-u10-report.md`, per-phase figures.
Open, do NOT merge. Final message: PR number + report path only.
Forks: report, smallest faithful reading, flag.
