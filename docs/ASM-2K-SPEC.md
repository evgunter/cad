# ASM-2K — the multi-solid instancing kernel door (binding spec)

Binds the substrate report's C2 correction (the R1 ladder's "no
kernel prerequisites" was false for multi-solid sources) as its own
kernel-only unit, extracted from ASM-2b so it can run PARALLEL to
ASM-ROOTS: this unit touches `topo` and the `names` layer only —
no document-layer files, no schema, no persistence. Consumers when
they land: ASM-2b (sub-assembly instantiation), the ASM-ROOTS
gather's multi-solid roots, ASM-3's pattern materialization.
Difficulty class: **M**. Deviations reported in the PR, never
silently absorbed.

## D-1: `graft_disjoint` accepts multi-solid sources

Today the door refuses a multi-solid source
(`crates/topo/src/instance.rs:63-79` at the substrate recon;
re-read the current state). Extend it — or add a sibling
`graft_disjoint_all` if the single-solid signature is
load-bearing for existing callers — so a source `Body` with N
solids grafts all N into the aggregate with fresh arena keys,
preserving per-solid identity (which solid each face came from
remains derivable from provenance). Semantics identical to N
sequential single-solid grafts in the source body's solid order
(D9-deterministic); state this as a tested equivalence, not prose.
The step-import loop (`step-import/src/lib.rs:502-616`) is the
consumer pattern to preserve: per-solid validation when N>1, then
aggregate validation — this unit must not change that loop's
behavior (its bodies are single-solid; a no-diff run of the import
suite is part of the evidence).

## D-2: `name_pattern` lifts the multi-body-master wall

`names/emit.rs` refuses "pattern of a multi-body master —
deferred (typed, R7)" (emit.rs:126 at recon). Lift it with the
uniform-wrapping rule: `RoleSeg::Instance { i, of }` wraps EVERY
stable name of the master, multi-solid masters included — a
master's names are already unique within the master (derivation
paths distinguish its solids), and the Instance(i) qualifier
preserves that uniqueness per instance. No per-solid sub-index is
introduced (the solid is recoverable from the wrapped name's own
derivation, not from the instance qualifier). The R7 register
entry retires with a pointer here.

## D-3: provenance

`GeomSource::placed(node, instance)` rides unchanged; grafted
multi-solid instances carry the same placed-provenance the
single-solid path mints today. No new provenance vocabulary.

## D-4: what this unit does NOT do

No `InstantiatePart`, no document-layer wiring, no schema or
persistence change, no Python surface beyond mechanical tag arms
if a new typed error appears (prefer extending existing error
types' coverage without new variants). The document-layer
consumers arrive in ASM-2b/ASM-3; this unit's deliverable is the
kernel door plus its falsifiers.

## Acceptance rows (executable falsifiers, in-suite)

1. Multi-solid graft ≡ sequential single-solid grafts: same
   aggregate census, same arena-key count, bit-equal volumes,
   solid order preserved (the D-1 equivalence, tested).
2. Fresh-key discipline: grafting the same source twice yields
   disjoint key ranges; no id collisions (the SolidSpec-collision
   class from the recon, guarded).
3. Per-solid + aggregate validation parity: an invalid solid
   inside a multi-solid source refuses at the per-solid gate
   naming which solid; the aggregate gate still runs on success.
4. Naming: a linear pattern of a TWO-solid master yields N×2
   solids whose stable names are all distinct, each carrying
   Instance(i) over the master's own name; census counts equal
   N × master counts; the former typed refusal is GONE and its
   test row flipped (not deleted — flipped to assert success).
5. Resolution: names of instance i resolve to instance i's
   geometry (spot-checked via the existing resolve doors on a
   probe document or direct kernel-level name table).
6. step-import suite: zero behavioral diff (full import battery
   green, censuses unmoved).
7. Cold clippy for touched crates; hosted CI green; k-lint
   discipline (a fired gate is reported, never silenced with
   geometry).

## Standing brief lines (verbatim obligations)

OUTPUT DISCIPLINE: ≤~150 lines per tool call, chunked reads,
skeleton-first writes, report ≤150 lines. Run every build/battery
row as a synchronous FOREGROUND Bash call, one at a time, reading
each result before the next; NEVER arm waiters, monitors, or
background chains for your own builds/tests; when the build-slot
queue is busy, a BLOCKING foreground wait is the correct state —
re-issue a timed-out call rather than parking (kill your own
previous waiter first, or use -n/--express). Merge origin/main
immediately before opening the PR and re-merge whenever main
moves; after any push confirm checks STARTED. Comments state the
INVARIANT, not the history. Commit and push after every coherent
unit.
