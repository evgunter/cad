# ASM-1 — document identity + content pins (binding spec)

Binds ASSEMBLY-DESIGN A4 (identity ≠ pin; D9 makes the pin
well-defined), A2's ε-seam posture, and the substrate report's
ASM-1 decision list (cad-work/asm-r1-substrate/report.md §2, §7,
§8, minor-A4). Difficulty class: **M** (decision-heavy,
code-light). Everything below is binding; deviations are reported
in the PR, never silently absorbed.

## D-1: `DocumentId` — authored, not random

A newtype over `u128`, a field of `Doc`, hex-displayed. It is
**authored data supplied at construction** — never minted from
ambient randomness inside the kernel, because corpus and demo
regeneration must stay byte-identical (D9 reproducibility of save
bytes). Two constructors:

- `DocumentId::derive(label: &str)` — deterministic (first 16
  bytes of SHA-256 of the label); what demos/corpus/tests use.
- `DocumentId::new_random()` — OS randomness, for interactive
  authoring; lives in the DOCUMENT layer (`pncad`), not
  editor-core, so the kernel stays deterministic-by-construction.

Identity survives every edit; nothing about content constrains it.
Workspace-level uniqueness is enforced by the store (D-5), not by
construction.

## D-2: `ContentPin` — SHA-256 over canonical bytes

`sha2` dependency (RustCrypto; ubiquitous, far past the 2-week
floor). The pin is the 32-byte SHA-256 of D-3's canonical bytes,
hex-displayed. Collision resistance is required because the pin IS
version identity (A4); the in-process memo's FNV `ContentKey`
stays a deliberately separate vocabulary (substrate C3) — no
unification, and a comment on each type names the other to keep
them from drifting together.

## D-3: canonical bytes — full replayed snapshot, two exclusions

**(AMENDED per Evan's #345 comment, 2026-08-10: include-by-default
replaces the earlier semantic projection.)** `canonical_bytes(doc)`
serializes, with the existing deterministic machinery (BTreeMaps,
pair-list keys, ryu floats), the REPLAYED document (log applied
first, same discipline as `persist::save`) with exactly two
exclusions, each structurally justified:

- the **edit log** — history is not state: two edit paths to one
  snapshot are the same version, and undo must not move pins;
- the **`DocumentId`** — A4: the id answers "which part," the pin
  "which version"; a document copied under a fresh id is
  detectably the same content.

Everything else is INCLUDED — nodes, order, structural + document
params, recorded ε, witnesses, metadata, appearance, and the root
list when ASM-ROOTS lands. Rationale: include-by-default is
evolution-safe — a future `Doc` field pins automatically instead
of relying on someone remembering to classify it into a projection
(a silent-omission trap); exclusions are explicit carve-outs made
when a field demonstrably becomes a problem, rationale recorded
then. Consequence stated honestly: an appearance-only edit moves
the pin, so a consuming assembly sees an update whose
re-verification passes trivially — accepted v1 noise; a future
carve-out, if earned, rides a schema seam. Sharper consequence,
same posture (R2 review of #364): an UNDONE INSERT moves the pin —
delete does not decrement the monotone `next_id`, and the counter
is document state in the include-by-default preimage. "Undo must
not move pins" therefore holds exactly for value edits (whose
inverses restore the full state); structural insert/delete pairs
leave counter residue and pin as a new version. Accepted v1 noise
on the same terms; a `next_id` carve-out, if earned, rides a
schema seam.

## D-4: the wrapper

`DocRef { id: DocumentId, pin: ContentPin }` in editor-core —
the value future `InstantiatePart` nodes carry. Serde like any
payload; Display as `id@pin-prefix`.

## D-5: the workspace store (read-side only, in `pncad`)

`Workspace::open(dir)`: scans `*.pncad` files, reads each header's
id, builds the id → path map. **Duplicate id = typed refusal
naming both paths.** `Workspace::resolve(&DocRef) → Result<Doc>`:
load (v5 validation as today), recompute the canonical pin,
**mismatch = typed `PinMismatch`** carrying found/wanted pins and
recourse text naming the future "accept updated version" edit
(A4). The ambient-ε reconciliation refusal already fires at load
and is left exactly as is. No write side in this unit — creating
files stays `save` + fs at the callers.

## D-6: schema v5, clean break

`Doc` gains the id field ⇒ SCHEMA_VERSION 5. Migration table
stays empty; v≤4 refuses typed with the regenerate recourse —
the ratified precedent (v1→v4 were all clean breaks). Corpus and
fixtures regenerate with `derive`-minted ids. ASM-ROOTS takes its
own bump when it lands (noted, not decided here). The save format
gains the id line in the header region so D-5's scan can read
identity cheaply.

## D-7: Python surface — mechanical only

New error variants get their pncad-py tag arms (the tripwire will
force them; keep the arms mechanical). NO new Python doors in
this unit — identity/pin/workspace doors are recorded as
bindings-parity pickups, minimizing the collision surface with
the concurrent program (report §9's five files).

## Acceptance rows (each an executable falsifier, all in-suite)

1. Pin determinism: same construction twice → equal pins.
2. History independence: two DIFFERENT edit paths to one
   snapshot → equal pins; an undone edit → pin unchanged.
3. Exclusions: id retarget → pin unchanged (metadata/appearance
   rows moved to row 4 by the D-3 amendment).
4. Inclusion by default: a node edit, a param edit, an ε change,
   a witness change, a metadata edit, an appearance edit → pin
   MOVES (each its own row).
5. id/pin separation: equal-content docs under two ids → equal
   pins, distinct ids.
6. Duplicate-id workspace → typed refusal naming both paths.
7. Pin mismatch at resolve → typed `PinMismatch` with both pins.
8. A v4 file → typed version refusal (existing shape, re-pinned
   for v5).
9. Clippy from COLD state for touched crates; hosted CI green.

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
moves; after any push confirm checks STARTED. If the k-lint gate
fires, do NOT change geometry to silence it — escalate. Comments
state the INVARIANT, not the history. Commit and push after every
coherent unit.
