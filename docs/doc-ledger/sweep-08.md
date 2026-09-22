# Sweep 8 — 2026-09-06: SEAT leaves the tracker

Sweep SHA: `326be1bf4` (`main`'s tip immediately before the deletion),
so every path below is recoverable at
`git show 326be1bf4:work/seat/<FILE>`,
`git show 326be1bf4:docs/SEAT-EXIT-WALK.md` and
`git show 326be1bf4:docs/VERB-SEAT-DESIGN.md`.

Sweep 5's rule. The walk rode `[ev]` PR #1997 as PROPOSED and Ev
ratified it in session on 2026-09-06 ("1997 looks good"), merged
`6fe98d312`; the one open text correction it carried, `[ev]` PR #1983
(VERB-SEAT-DESIGN §1 S3 names `carrier_pair_relation` and the lily's
three declarations), was ratified the same way ("1983 looks good too")
and merged before this sweep. Both merged without comment, so the three
points the walk put to Ev stand as walked: the S3 text as corrected on
#1983; SEAT-9's answer to the PR-1904 question (no over-ε geometry was
ever handed back — tier-3 validation re-certified at the run's ε — and
the residual named there, the transform lane's re-certification
against a stored tolerance, is filed for its owner); and the
compound-`Bounds` allowlist entries in `geom-core/src/real.rs` keep
their retroactive-review flag. Fifteen files, one program:

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `seat` | SEAT — the verb-seat program | 2026-09-06 | this entry (the walk: twelve units delivered — SEAT-1 #1399, SEAT-2 #1521, SEAT-3 #1531, SEAT-4 #1547, SEAT-DV #1564, SEAT-5 #1581, SEAT-6 #1593, SEAT-7 #1910, SEAT-8 #1950, SEAT-FW #1974, SEAT-DN #1987, SEAT-9 #1995 — the plan's wave cut complete; VERB-SEAT-DESIGN walked clause by clause (§1 S1–S4, §2 V1–V4, §3 P1–P3, VS-Q1–Q6 with VS-Q4 revised at #1870, §5, §6) with every clause executed and its text now beside the code; the A/B record for ordinals 1000–1011 in `docs/MODEL-AB-LOG.md`, rows SEAT1 … SEAT9 with blocks SEAT-B1/B2/B3 concluded and their draws published — twelve FAIR pairs, five tally candidates queued for the blinded coding; the open list dispositioned below) |

- `work/seat/program.md`, `plan.md`, `log.md` — the charter (paths
  `crates/verbs/*`, `topo/src/query.rs`, `topo/src/flush.rs`,
  `editor-core/src/verbs/*`, the names seat's `geompred.rs`/`flush.rs`,
  the `seat*` suites, the design doc; band 1000–1099), the three-wave
  plan (the kernel query seat, one verb vocabulary, lowered parameter
  identity) with its side units, and the narrative from the program's
  opening through every unit's MERGED entry, the three block closes and
  the walk's opening.
- `work/seat/SEAT-6.md` … `SEAT-9.md`, `SEAT-DN.md`, `SEAT-FW.md` — the
  unit items that lived in the tracker (SEAT-1 … SEAT-5 and SEAT-DV
  closed before the tracker migration; their record is their ledger
  rows), each closed with a `## Closed` pointer.
- `work/seat/direction-normalization-two-doors-one-home.md`,
  `flush-detector-widening-to-curved-rungs.md`,
  `parameter-identity-channel-to-boolean.md`,
  `seat6-germ-end-to-end-awaits-seat7.md`,
  `shell-doors-take-tolerance-beside-tol.md` (Ev's ruling (i) on #1904
  recorded in it), `verb-seat-design-s3-names-the-planar-verifier.md`
  — closed, by the units and the two `[ev]` rulings.
- `docs/SEAT-EXIT-WALK.md` — the ratified walk, replaced by this entry.
- `docs/VERB-SEAT-DESIGN.md` — the ratified design (#1388, S3 corrected
  at #1983, VS-Q4 revised at #1870), MOVED beside the code it governs as
  `crates/verbs/README.md` in present tense with every clause id kept;
  DESIGN.md's companion table points there.

Residue was re-homed in this sweep, ids unchanged — three items:
`two-verb-seats-do-not-compose` (the #1345 items (2)/(3) the design
deferred until a replay consumer exists) to `work/issues/`;
`flush-pair-relation-has-no-caller` (S-BOOL's module) to `work/bool/`;
`two-d-director-doors-skip-the-finiteness-question` (the five-member
finiteness class, FIX's `is-finite-length-homed-in-the-query-seat` the
ruling that closes it) to `work/fix/`. Every other SEAT-originated
residue was filed where it belongs at the moment of disclosure
(`work/issues/`: `axis-flavoured-declarations-have-no-channel`,
`node-tag-space-census-blind-to-tags-outside-sentinels`,
`two-public-verb-types-verbs-and-profile`,
`dirty-pr-gets-no-actions-run`,
`approx-surface-tolerance-is-now-always-the-runs-eps`,
`offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart`;
`work/lib/lib-g17-is-parked-on-a-fired-trigger`;
`work/curved/c5a2-ledger-sample-143-collides-with-seatfw`) and does not
move. What opens with this sweep: nothing — `crates/verbs` has no
program and its next change is LIB-G17's `Node::Shell`, whose enabler
(`VerbRecord::Shell`) SEAT-9 put in place. The A/B band 1000–1099
stays claimed in `docs/MODEL-AB-LOG.md`'s ordinal-bands section as
always.

### Inbound references

Append-only logs (`docs/MODEL-AB-LOG.md`, the other programs' `log.md`
files) keep their `work/seat/` and `docs/VERB-SEAT-DESIGN.md`
citations; they resolve here as before. Live pointers were re-pointed
in this sweep: DESIGN.md's companion table row and its programs
sentence; four `work/issues/` items whose `refs` named a SEAT unit id
now name the unit's PR instead. Prose citations of "VERB-SEAT-DESIGN
§n" resolve against `crates/verbs/README.md` by clause id.
