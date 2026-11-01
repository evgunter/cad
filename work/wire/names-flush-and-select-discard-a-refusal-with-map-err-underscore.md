---
id: names-flush-and-select-discard-a-refusal-with-map-err-underscore
kind: issue
title: names/flush.rs and names/select.rs discard a typed refusal with map_err(|_| ..), the shape MSOLVE-3 closed in mate/
status: closed
opened: 2026-09-06
pr: 2378
branch: wire/names-refusal-carries-cause
closed: 2026-09-11
---


Reported by MSOLVE-3's implementer lane (PR 2081), outside its fence;
filed by the MSOLVE orchestrator. SEAT's ground (`names/select.rs`,
`names/flush.rs` are theirs by DOCM's keep_out).

`crates/editor-core/src/names/flush.rs` and `names/select.rs` carry
`map_err(|_| …)` arms that drop a typed inner refusal and raise a
different kind in its place — the relabeling shape MSOLVE-3 removed
from `mate/*` (`memories/refusal-text-is-not-cause.md`). Whether each
site's replacement kind is the honest one, or the inner kind should be
carried, is the owner's read; the lane's grep is the receipt.

## Re-homed to WIRE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

WIRE is the evaluation seat's successor. `docs/DOC-LEDGER.md` sweep 9
parked three of these rows in `work/issues/` as "the successor's opening
slate" when EVAL closed on 2026-09-08, naming two more already there;
this program is that successor, and it takes the rest of the seat's
ground with them.

Its class at the cut was **E** — two `map_err` sites; may add one
variant to carry inner kind. The class is a dispatch estimate made by
reading the row against the tree on 2026-09-11, not a verdict on the
finding, and a lane that finds it wrong says so in its PR. The id, the
`track:` letter where the row carries one, and the body above are
unchanged by the move.

## Closed (2026-09-11) — PR 2378, merged

Four sites now carry the cause: `names/flush.rs`, `names/select.rs`,
`names/discriminate.rs` and `pncad-py/src/py/select.rs`. The shape is
`SelectRefusal::Band(BandError)` / `NamingError::Band(BandError)` with an
`impl From<BandError>` on each, so the call sites read
`Band::linear(tol)?` — **the closure is gone entirely**, which is the
defect's habitat: `map_err(|source| …)` is one keystroke from
`map_err(|_| …)` and `?` has nowhere to put the underscore. A structural
repair, not three instance repairs.

### The measurement was the unit

The item delegated "is each site's replacement kind the honest one" to
the owner. That could not be answered without knowing whether
`Band::linear`'s failure has a unique cause, and **it does not**:
`Tolerance::validate` admits any finite ε > 0 and any finite K > 1, and
at subnormal ε with K below 1.5 the increment rounds away, so
`K·ε == ε`, the band collapses, and `BandError::Empty` is raised with
nothing overflowing. `Band::linear`'s `# Errors` says "only … overflows
to infinity". The two reachable arms want **opposite** repairs, so a
refusal naming neither sends half its readers the wrong way.

Filed as `work/props/band-linear-errors-doc-is-false-empty-is-reachable-at-subnormal-eps.md`,
which also carries `Band::angular_at`'s worse sibling (reachable at
ε = 1e-9 with a large lever arm — an ordinary tolerance and an extreme
*caller argument*).

The lane's second argument stands on its own: D9 chose a typed error
**over a panic**, so a `Result` whose `Err` is destroyed one frame up is
strictly worse than the panic D9 declined.

### Class estimate E → H

Four sites not two, two crates, three refusal surfaces, ten files, and
the decision required refuting a doc in a third crate. `plan.md`'s slate
row is corrected.

### Full review: 5 MAJOR, all repaired

The review raised the posture from light and earned it.

- **The test did not guard its premise.** It asserted over
  `Band::new(ε, K·ε)`, a hand re-derivation; a mutation making `Empty`
  unreachable from `Band::linear` **entirely** left it green. Now
  `crates/editor-core/tests/wire_band_cause.rs` calls the real doors and
  both mutations redden it.
- **"editor-core has no test door" was false** — `m4_pr6_eps_diff.rs`
  already re-execs with a pathological ε. The lane used it, and the row
  is still unreachable for a **stronger and structural** reason it then
  pinned: a profile cannot be authored at a bandless tolerance
  (`Doc::apply` refuses), a bare frame datum fails at `eval/wire.rs:702`,
  so `select_where` short-circuits before `Band::linear`. **The names
  band arms are defensive in practice**, which sharpens the payload
  argument rather than weakening it: nobody will debug one live, so the
  text is the only evidence that will ever exist.
- **The fix reminted its own defect on the public Python contract** —
  `tags.rs` mapped `Band { .. } => "band"` flat while `py/select.rs` says
  "the fields are the contract; the message is prose", so a caller
  branching on `reason` still could not tell the arms apart. Both arms
  now delegate to `band_error_tag`; `TAG_INVENTORY` gains `delegates:`.
- The sweep **missed a live instance in a file it swept** — see below.
- Two fence crossings were undisclosed, including the largest source
  change.

### Residues, each with a file

- `work/shell/clearance-reports-a-no-bodies-payload-as-a-bad-body-index.md`
  — now carries both of that file's destroyed causes, including
  `clearance.rs:1076`'s `let-else` discard of a `BandError` into a unit
  variant: **bit-for-bit this defect, still live**.
- `work/fix/remap-name-misses-lose-the-id-they-caught-at-six-refactor-sites.md`
- `emit-topo-destroys-the-edge-key-in-a-split-lineage-cycle`
- `work/issues/every-band-construction-is-the-class-not-every-map-err.md`
  — the methodology finding, and the transferable one: **sweep the
  construction of the thing whose cause can be lost, not the syntax of
  one way of losing it.** `clearance.rs:1076` was invisible because the
  discard is a `let-else`; the CI red was a unit variant at a *rendering*
  site. That row also asks whether amending
  `docs/prompts/implementer-discipline.md` §5 is an `[ev]` question,
  since it is read by every lane by path.
