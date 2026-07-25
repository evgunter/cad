# M4 PR 6 binding spec — persistence (schema v1)

Status: DRAFT until PR 5 merges (implementation dispatch waits for
that merge; the spec is written at the PR-5-review seam so the
implementer launches the moment main is ready). BINDING once
dispatched. F3 + F8 ratified; M4-PLAN item 6. Deviations via the
REPORT mechanism only.

## D1 — Format: snapshot + edit log, versioned text

F3 as ratified: serde-based TEXT format; the file is a `schema:
<integer>` header followed by a SNAPSHOT (the full Doc) and an EDIT
LOG (DocEdits since snapshot). Schema starts at 1; migrations are
explicit version-to-version functions from v1 onward (identity today,
but the MECHANISM ships now: a `migrate(from_version, value) →
Result<Value, MigrationError>` chain the loader walks). Format
aesthetics (JSON vs RON vs custom) are the implementer's PR-spec
choice — REPORT which and why; the contract is the shape above.

## D2 — Floats: bit-exact, shortest round-trip, NaN/inf refused

Floats serialize as Ryu shortest-round-trip strings; load must
reproduce the exact bits (round-trip property tested on subnormals,
-0.0, ulp neighbors — the PR 1 replay corpus is the seed). NaN/inf
REFUSED at persist time with a typed error naming the node/slot (the
kernel never legitimately produces them; persisting one is a
surfaced bug, not data). -0.0 is DATA (preserved), not refused.

## D3 — What persists

The Doc: nodes (all F4 vocabulary incl. Declare), parameters,
expressions, witness bytes (opaque, base64 or hex — bit-exact),
appearance store (keys = StableNames serialized structurally),
recorded ε (D5 below), schema version, and the edit log. NOT
persisted: evaluations, name tables, memo/content/naming keys, arena
anything — the recipe IS the save; everything else re-derives. A
load followed by evaluate must reproduce bit-identical tables (the
save/load/replay-identity CI row, D6).

## D4 — ε recorded in-document; SetTolerance = replay + diff

The document records its ε (the ambient-ε mechanism gains a recorded
source of truth at load). `SetTolerance` applies as: persist-grade
replay at the new ε + the PR 4 diff engine reporting exactly the
flipped predicates (the first REAL two-ε diff — discharging PR 4
review Finding 6's wait). The OnceLock ambient mechanism stays for
process bootstrap; a loaded document's recorded ε wins (REPORT the
exact wiring; a process may not host two ε values simultaneously —
that constraint stands, refuse loudly on conflict).

## D5 — Content-key format tag bump

PR 4 review Finding 8, banked for exactly this moment: keys still
write `write_tag(1)` while their input set grew (witness datum,
naming-key context). Keys remain process-internal (D3: never
persisted) — but bump the tag NOW and add a comment tying the tag to
the input-set shape, so any future persistence of keys inherits an
honest version. (If keys stay unpersisted this is one line; do not
build key migration machinery.)

## D6 — CI rows

1. save/load/replay-identity: full corpus documents round-trip
   bit-identically AND replay to bit-identical name tables + bodies
   at ε ∈ {1e-6, 1e-9, 1e-12} + Interval.
2. ε-change diff: an ε edit on a margin-thin fixture reports exactly
   its flipped predicates (goldened).
3. Schema-version refusal: a file with version 2 (unknown) refuses
   typed; a truncated/corrupt file refuses typed with position info.
   No silent best-effort loads, ever.

## D7 — Black-box appearance metadata (Evan's #92 ask, banked)

Schema v1 freeze is the decision point Evan named, so it lands here:
appearance values gain an OPAQUE extension arm — `Custom { key:
String, bytes: Vec<u8> }` (or a `metadata: BTreeMap<String,
Vec<u8>>` on the appearance record — implementer picks the shape
that serializes cleaner, REPORT which). Constraint from #92: F3/
bit_eq compatible = bit-exact serializable, trivially satisfied by
bytes. Kernel/editor-core NEVER interprets the bytes (black-box for
GUI/tooling layers); N3/N5 retire/vanish semantics apply to the
whole appearance record, Custom arm included. Shape RULING
(discussed with Evan 2026-07-25): bytes, not a generic parameter or
dyn trait — a generic M makes the v1 file format open-ended (F3
migrations can't be defined over an uncontrolled type) and a dyn
registry makes loaded-tooling part of the format implicitly; bytes
are the second instance of the witness-datum convention. Convention
REQUIRED of producers: metadata values begin with a leading version
integer (mirroring WitnessDatum.schema); typed views live in the
layer owning the key namespace.

## D8 — Out of scope

STEP import; any evaluation-layer change beyond D4's ε wiring and
D5's tag line; compression; multi-file/assembly formats (wrapper
seams stay reserved); undo-history persistence beyond the edit log;
GUI. #93/#99 are separate lanes.

## D9 — Process (standing)

One implementer + one adversarial reviewer + one fix pass; OUTPUT
DISCIPLINE header; persistent clone, push per unit; RAM discipline
(post-restart: ~10G, up to two cargo lanes machine-wide — pgrep
first); fail loud; Actions gates the merge; ci-local.sh kept in
sync. Model per docs/MODEL-AB-LOG.md coin flip at dispatch;
reviewer blinded, rubric required.
