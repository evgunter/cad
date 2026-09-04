# DOCM — the document model (plan)

**STATUS: OPEN (2026-09-03).** Opened 2026-09-03 from `docs/WORK-TRACKS-2026-09.md` (DOCM section), which is this
program's charter until this plan supersedes it. Live state is
`work/docm/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`docm/`** — unit branches
`docm/<unit>-<slug>`, orchestrator branch `docm/orchestrator`.
Away-channel tag `(DOCM orchestrator)`. A/B ordinal band
**DOCM = 1800–1899**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit, per that entry's rule.

## Charter

Own the document model. Today the persist schema, the `DocEdit`
vocabulary, datum frames and operand selectors have no owner: S-BOOL's
keep_out names the schema "contended ground — announce before
landing", LIB's names recipe doors and the resolver door "design
conversations before they are units", M10's says eval/schema outside
its lane is "LIB and SEAT ground". This program takes the ground and
runs the conversations, in the order that unblocks the most, each as
its own `[ev]` PR with a firm recommendation and the honest
counterarguments. Builds go to the class-matched track (FIX, CHROME,
VIEW) or stay here when ruled and hard.

## Territory

The fence is drawn against code-quality Track V in the opening
commit: Track V cedes `persist/*`, `program.rs`, `doc.rs`, `edit.rs`,
`node.rs`, `names/role.rs`, `eval/{parts,memo}.rs`, `resolve/*` less
`vdiff.rs`, and the mate/assembly files, and keeps the rest of
`crates/editor-core/` and `crates/profile/`. The four V rows on the
ceded ground moved with it: `C6` (the `ProgramStep`/`WireStep`/`SegTag`
mirrors), `D365` (content-key mode tags), `D366` (`NodeErrorKind`'s
projection and pncad-py's 48-arm mirror — LIB answers the Python side)
and `debug-in-prose-residue-after-finding-sink` (`PersistError`,
`EditError`, `NamingError`). They keep their ids and their `track: V`
provenance. `mate.rs`, `mate/*` and `assembly.rs` are this program's
since S-MATE's closing sweep (`docs/DOC-LEDGER.md` sweep 6).

## The rulings, and the slate they cut

Two conversations, ratified in-chat 2026-09-04 and recorded as
companion docs:

- **`docs/DOCM-REFERENCES-DESIGN.md`** (DM1–DM6): what a recipe
  reference may be. Builds that stay here: `DOCM-1` (the derived
  sketch frame, the sense beside the pose, the carrier-kind read),
  `DOCM-2` (`Node::Part`), `DOCM-3` (`Node::Union` n-ary,
  `DocEdit::SetMembers`, pairwise-distinct inputs).
  `no-docedit-splices-a-deleted-node` is parked on `DOCM-3` (DM6).
- **`docs/DOCM-IDENTITY-DESIGN.md`** (DI1–DI5): a held value names
  the world it came from. Build that stays here: `DOCM-4`
  (`Evaluation` carries its document's identity; A4's refusal
  sentence narrows to the seam). The viewer builds went to CHROME
  (DI2's re-mount, DI4, DI5) and VIEW (DI1).

Needing no ruling — the persisted-variant question was already
answered by the Band-4 no-schema-version rule (`docs/DESIGN.md`,
BOOL-13): `capend-top-bottom-contradicted-by-negative-extrude`,
`fused-step-slot-aliases-arrival-spec` and
`revolve-pole-export-interior-on-axis-vertex` are E-class units in
place (ids kept); so are `D365`'s census, `C6`'s `WireStep` member,
`D366`'s kind-mirror decision and
`debug-in-prose-residue-after-finding-sink`.

## The questions still open, in order

1. **The instantiation seam** — `instantiation-seam-drops-mate-identity`:
   carry `MintedDeclaration` (and `unminted`) across `PartValue` so a
   carried refutation names its mate and the outermost gate sees
   inner mint health; whether an inner mint refusal is the outer
   document's error reads the Q1 ruling's letter and is put to Ev.
2. **The check registry's subject** — `check-registry-gathers-product-twice`:
   `run_checks` computes the product once and hands residents a
   subject; `assemble` takes a pre-gathered product; the `product.rs`
   Dual arms are edited by announced seam to M10.
3. **A certified range query** —
   `certify-locally-valid-range-instead-of-sampling`: the
   slot-widening override (document parameters already widen through
   `EvalOptions::param_box`), the indeterminate-means-subdivide
   verdict contract, and pacing; the build reuses M10-3's driver.
   Waits on M10's parameter-aware certification settling.

Riders: half (2) of `mate-clocking-has-no-gui-path` (FIX has half
(1)); `unify-discipline-machinery-onto-registry` step 2 once the
parameter-coincidence unit exists.

## What is out

Viewer chrome and architecture (CHROME, VIEW); the analysis lane
(M10's, then PROPS'); `resolve/vdiff.rs` (S-BOOL's); the pncad façade
and bindings (LIB's — every `.pyi` consequence of a ruling here is
LIB's unit, filed by this program).

## Exit shape

Every ruling above is ratified in a companion doc or a README beside
the code, every build it cut has merged or has a home, and the three
open questions have ratified answers; the walk convention applies.
