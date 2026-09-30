# VDOC — the viewer's prose, citations and censuses (plan)

**STATUS: OPEN (2026-09-17), AND NOT DISPATCHING.** Opened in VIEW's
re-scope, on twenty-two rows that arrived by `git mv` with their bodies
unchanged. The opening condition for the first unit is in §Order and is
a condition, not a note. Live state is `work/vdoc/log.md`'s tail and the
item files beside this plan, never this file.

Branch prefix (the #396 convention): **`vdoc/`** — unit branches
`vdoc/<unit>-<slug>`, orchestrator branch `vdoc/orchestrator`.
Away-channel tag `(VDOC orchestrator)`. A/B ordinal band
**VDOC = 5500–5599**.

## Charter

**No row here changes what the viewer does.** Every one is a defect in
a claim the tree makes about itself, and every fix is a re-derivation
against the tree or the assertion that would have caught the
difference. Four shapes, read off the rows:

- **A citation points at code that moved.**
  `stale-file-citations-after-the-split` (24 open files citing
  `app.rs:NNNN` or `session.rs:NNNN` for code the split moved, five
  wrong about the file), `tree-rs-header-growth-moved-five-cited-subjects`,
  `renamed-module-leaves-citations-in-two-other-programs`,
  `edge-cost-claims-name-a-search-that-is-gone`,
  `viewer-prose-calls-the-frame-entry-point-update` (naming a function
  that has never existed).
- **A count does not re-derive under its own rule.**
  `census-table-in-the-viewer-readme-is-not-its-own-population`,
  `viewer-readme-multi-field-write-sweep-count-does-not-reproduce`,
  `the-citation-receipts-summary-numbers-are-not-re-derivable`.
- **A sentence certifies a population, or a rule, that it cannot.**
  `a-module-named-for-its-spine-type-is-unfalsifiable` (a naming rule
  that cannot fail for a large enough type, closing its own original
  finding by re-description),
  `every-crate-root-reexport-is-a-second-path-not-the-only-one`,
  `readme-and-type-docs-restate-one-argument-for-landing-and-landedrun`,
  `viewer-lib-marker-claims-the-log-carries-its-sentence`,
  `the-readme-splits-camera-rs-across-two-decision-rows`,
  `cfg-test-bare-spans-have-no-stated-disposition`,
  `four-debug-walks-are-spelled-and-placed-two-ways` (two spellings
  written against one rule in one diff, with nothing deciding between
  them), `session-shims-and-test-imports` (two spellings of every moved
  path), `sweep-blind-spots-the-precheck-sweep-could-not-see` (what a
  sweep could not see, kept as a file so it survives the deletion of
  the directory that ran it).
- **A behaviour is asserted nowhere, so a claim about it cannot fail.**
  `datum-view-ok-path-is-asserted-nowhere`,
  `hover-route-for-an-absent-chooser-has-no-test`,
  `the-guard-that-decides-whether-a-preference-is-kept-has-no-test`,
  `viewer-suites-hold-hand-written-complete-variant-lists`,
  `a-doc-comment-names-a-test-row-and-nothing-checks-it-exists`.

**The test that separates this program from its three siblings, and it
is a sharp one.** Take any row here, apply its fix, and run the viewer:
nothing a person could observe has changed. That is false of VNEWS
(whose fixes change the words a reader sees), false of VGEOM (whose
fixes change the numbers and the picture), and false of VSEAM (whose
fixes change what the session holds and when).

Applying it the other way: **a row whose fix a user could notice is not
this program's, however much prose it also touches.** The two rows
nearest that line are `session-shims-and-test-imports` and
`four-debug-walks-…`; both are behaviour-preserving by construction —
a re-pointed import and a moved `Debug` impl — and both are here
because their subject is what the tree SAYS about where a thing lives.

## Order

**This program's opening condition.** Its spine is
`stale-file-citations-after-the-split`, and every unit VNEWS, VGEOM and
VSEAM land invalidates more of it: a diff that shifts a file moves the
citations that point into it, and a repoint taken before those diffs
land is a repoint that has to be taken again. The VIEW register states
the mechanism twice over — *an out-of-fence citation table is a
statement about ONE diff against ONE base, and it expires the moment
another diff touches the same file*, and *a table is never applied as
written: placing it means re-deriving at placing time, by subject*.

**So: the spine dispatches when the last of VNEWS, VGEOM and VSEAM has
closed its slate or stood down, and not before.** Until then this
program files, holds and routes; it does not cut units against the
crate's moving files. Two exceptions, both because their subject does
not move with the code:

1. `renamed-module-leaves-citations-in-two-other-programs` — a report
   to two other programs about their own rows. It is a routing act, it
   edits no citation across the fence, and it is worth less the longer
   it waits. Dispatchable now.
2. `a-doc-comment-names-a-test-row-and-nothing-checks-it-exists` — the
   GATE half is a gate, not a repoint, and a gate written now reds for
   every sibling unit that breaks a name while it is being written,
   which is the whole point of it. Dispatchable now; the population it
   asserts is re-derived at merge, not at filing.

After the condition lifts, in order:

3. **The spine**, `stale-file-citations-after-the-split`, re-derived
   from scratch. The register's instrument is #2083's and is not
   optional: enumerate every `file:line` in every row a branch touches,
   `sed -n Np` each, and read whether the subject is there. The
   cheapest check that catches the largest class is `wc -l` — fifteen
   of the thirty-one out-of-fence citations measured in VIEW cited past
   the end of the file.
4. **The three counts** —
   `census-table-in-the-viewer-readme-is-not-its-own-population`,
   `viewer-readme-multi-field-write-sweep-count-does-not-reproduce`,
   `the-citation-receipts-summary-numbers-are-not-re-derivable`. One
   unit, because the register's rule is that a citation fix is
   class-wide over the file or it makes the file worse.
5. **The unfalsifiable sentences** —
   `a-module-named-for-its-spine-type-is-unfalsifiable`,
   `every-crate-root-reexport-is-a-second-path-not-the-only-one`,
   `readme-and-type-docs-restate-one-argument-for-landing-and-landedrun`,
   `viewer-lib-marker-claims-the-log-carries-its-sentence`,
   `the-readme-splits-camera-rs-across-two-decision-rows`,
   `cfg-test-bare-spans-have-no-stated-disposition`. Each rewrite owes
   the sweep rule that produces its population, written at the
   sentence — the register's rule, earned on this crate's README.
6. **The four test gaps** — `datum-view-ok-path-is-asserted-nowhere`,
   `hover-route-for-an-absent-chooser-has-no-test`,
   `the-guard-that-decides-whether-a-preference-is-kept-has-no-test`,
   `viewer-suites-hold-hand-written-complete-variant-lists`. Each is
   verified by MUTATION, not by reading: asserted-somewhere is not
   asserted-here, and only a perturbation tells them apart.
7. **The two re-spellings** — `session-shims-and-test-imports` (32 test
   files, and two of the thirteen names are not at the crate root at
   all, so for those a re-point is not a substitution) and
   `four-debug-walks-are-spelled-and-placed-two-ways`.
8. **`sweep-blind-spots-the-precheck-sweep-could-not-see`** — its three
   blind spots are in `session.rs`, which is VSEAM's. Filed there as a
   hand-off rather than taken here.

**A standing duty, not a unit.** The register's rule that *a diff that
shifts a file owns the citations that shift broke* binds the three
sibling programs, not this one. This program does not absorb damage
particular branches did — that is precisely how
`stale-file-citations-after-the-split` reached four classes without
fixing one.

## Inbound

No row in `review` is this program's.

## The discipline a lane is held to

**`docs/prompts/implementer-discipline.md` and
`docs/prompts/reviewer-style-lane.md`**, handed to every lane by path.
Read both before writing a dispatch; they are the standing obligations
and they are the only ones.

**The rule register this section used to inherit by reference is
deleted** (2026-09-21, Ev's ruling; it lived in `work/view/plan.md`).
Eighty-seven rules in eighteen days, of which the ones that both named
a real problem and would have been prevented by an advance warning
turned out to be already written — in the two files above, and in
`memories/agent-lane-operations.md`. The rest were retrospective
categorisation: true after the fact, useless before it. It is
recoverable at `66d7357417` if a row here cites one of its rules.

So a dispatch from this program carries the two prompt docs by path,
plus whatever this program's own `log.md` tail says about the ground
the unit lands on — not a register.

## Review posture

**Inherited from VIEW unchanged (Ev, in-chat, 2026-09-04, reaffirmed
2026-09-04 evening; `docs/MODEL-AB-LOG.md`'s roster line).** No A/B
duals, no row in `docs/MODEL-AB-LOG.md`; the band stays claimed and
empty. The default is a style review against
`docs/prompts/reviewer-style-lane.md`. The correctness arm's trigger
applies here in its gate shape: a gate that silently never fires is
indistinguishable from a gate that passes, so any unit adding one takes
the adversarial lane.

## Exit shape

Every citation in every open row that cites into `crates/viewer`
re-derived by subject, every census in `crates/viewer/README.md`
re-derivable from the rule written beside it, and every behaviour the
README asserts held by a test that a mutation reds. The walk convention
applies; residue re-homes per `work/README.md`.
