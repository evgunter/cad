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

The fence is drawn against code-quality Track V in this opening
commit: Track V cedes `persist/*`, `program.rs`, `doc.rs`, `edit.rs`,
`node.rs`, `names/role.rs`, `eval/{parts,memo}.rs`, `resolve/*` less
`vdiff.rs`, and the mate/assembly files, and keeps the rest of
`crates/editor-core/` and `crates/profile/`. The four V rows on the
ceded ground moved with it: `C6` (the `ProgramStep`/`WireStep`/`SegTag`
mirrors), `D365` (content-key mode tags), `D366` (`NodeErrorKind`'s
projection and pncad-py's 48-arm mirror — LIB answers the Python side)
and `debug-in-prose-residue-after-finding-sink` (`PersistError`,
`EditError`, `NamingError`). They keep their ids and their `track: V`
provenance.

## The questions, in order

1. **Persisted-variant compatibility** — what adding or renaming a
   persisted enum variant requires (forward-compat stance, migration,
   when `SCHEMA_VERSION` bumps). Unblocks
   `capend-top-bottom-contradicted-by-negative-extrude` and
   `fused-step-slot-aliases-arrival-spec`, both E after the ruling,
   and gives `C6`'s collapse its rule.
2. **Frames** — `sketch-frame-from-face` (`needs_ev` already): a frozen
   `Datum::Frame` from `face_frame` versus a derived-frame datum over
   a `StableName`, and a carrier-kind interrogation door versus
   any-face wording. Unblocks `add-profile-mints-no-frame` and
   `add-profile-placement-on-picked-face-frame` (CHROME builds) and
   `no-door-mints-mate-frame-from-face` (LIB held the plan; its
   hand-off is recorded by this file's existence here).
3. **Operand selectors** — `split-side-and-pattern-instance-as-operand`:
   a part-selecting operand versus a projection node; then widen
   `denotes_body`.
4. **Deleting from a chain** — `no-docedit-splices-a-deleted-node`:
   survivor policy per node kind, typed refusals, the schema bump. The
   build is H and stays here.
5. **Document identity (LIBRARY-DESIGN A4)** —
   `save-a-copy-duplicate-id-bricks-store` (an explicit fork-identity
   act), `memo-admission-and-resolver-state` (the eval memo checks
   resolver state, or the refusal contract narrows; the class sweep
   after is H), `document-seam-no-in-session-change-detection`
   (store refresh shape, save-as warning, chooser vocabulary).
6. **Layer-3 identity across rewinds** —
   `layer3-recipenodeid-aliases-across-rewinds`: one rule for every
   holder of a `RecipeNodeId` (generation stamp at consume, clear on
   replacement, or stable names). The viewer build is VIEW's.
7. **Free-move commit** — `no-persistent-setplacement-session-op`,
   constrained by the G3 ratification (free-move is never persisted):
   a `SetPlacement` door, a commit-probe affordance, or a documented
   no.
8. **Revolve naming D1** — `revolve-pole-export-interior-on-axis-vertex`:
   what the pole export yields for an interior on-axis vertex now
   editor-reachable; one row per direction after.
9. **The instantiation seam** — `instantiation-seam-drops-mate-identity`:
   carry `MintedDeclaration` across `PartValue`; whether an inner mint
   refusal is the outer document's error narrows the Q1 ruling's letter
   and is put to Ev as such.
10. **The check registry's subject** — `check-registry-gathers-product-twice`:
    `run_checks` computes the product once and hands residents a
    subject; the `assemble` variant and the `product.rs` Dual arms are
    edited by announced seam.
11. **A certified range query** —
    `certify-locally-valid-range-instead-of-sampling` (M10 residue):
    a slot-widening override, an indeterminate-means-subdivide verdict
    contract, and pacing; the build reuses M10-3's driver.

Riders that need no ruling: `D365`'s injectivity census (E, as the
verb tags already are); `D366` once question 1 settles how a kind
mirror is declared; half (2) of `mate-clocking-has-no-gui-path`
(FIX has half (1)); `unify-discipline-machinery-onto-registry` step 2
once the parameter-coincidence unit exists.

## What is out

Viewer chrome and architecture (CHROME, VIEW); the analysis lane
(M10's, then PROPS'); `resolve/vdiff.rs` (S-BOOL's); the pncad façade
and bindings (LIB's — every `.pyi` consequence of a ruling here is
LIB's unit, filed by this program).

## Exit shape

Every question above has a ratified answer in `docs/DESIGN.md` or a
README beside the code, and every build it unblocked has a home; the
walk convention applies.
