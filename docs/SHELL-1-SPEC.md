# SHELL-1 — the `ShellNaming` birth channel

**Status: RATIFIED by the SHELL orchestrator (2026-09-04) as a faithful
elaboration of RECIPE-DOORS D5 (Ev, 2026-08-29) and VERB-SEAT V1.**
Binds the implementer of unit `SHELL-1` (`work/shell/SHELL-1.md`,
branch `shell/1-naming`); deleted at merge per `docs/DOC-LEDGER.md`.
Read `docs/prompts/implementer-discipline.md` in full first.

## 0. What this unit is

`topo::shell` and `topo::shell_open` (`crates/topo/src/shell.rs`)
return a bare `Body<T>`. Every other birth verb writes a record the
document layer's naming emitter consumes — `BlendNaming`
(`crates/sweep/src/blend/naming.rs`), `SplitNaming`
(`crates/topo/src/splitting/finish.rs`), `BooleanNaming`
(`crates/topo/src/boolean/ops.rs`) — and a recipe node without one
mints no `StableName`s. LIB-G17 (`Node::Shell`) is parked on exactly
this, and SEAT's V1 states the rule: a verb without a birth-record
shape cannot join the verb enum.

This unit gives the two shell doors a `ShellNaming` record, written
**by the doors' own steps as they run** (NAMING-DESIGN N4: mint-time
wiring facts, never reconstructed by post-hoc inspection), and returns
it beside the body. Nothing about what the verbs BUILD changes; the
existing acceptance suites must stay green with only their reads
re-spelled (`.body`).

## 1. The result type

```rust
/// Everything `shell` / `shell_open` built: the thin solid and the
/// birth record its consumers name entities through.
#[derive(Debug)]
pub struct Shelled<T: Real> {
    pub body: Body<T>,
    pub naming: ShellNaming,
}

pub fn shell<…>(body: &Body<T>, thickness: T, tolerance: f64, tol: Tol)
    -> Result<Shelled<T>, ShellError<T>>;
pub fn shell_open<…>(body: &Body<T>, thickness: T, open_faces: &[FaceKey],
    tolerance: f64, tol: Tol) -> Result<Shelled<T>, ShellError<T>>;
```

The `Extruded` / `Revolved` / `BooleanBody` shape: one struct, the
body and its record, both `pub`. **Not** a second door (`shell_named`
beside `shell`): the record must be unconstructible-without and
impossible to skip, which is the D5/V1 point. `Shelled` and
`ShellNaming` are re-exported from `topo`'s root beside `ShellError`.

The `tolerance: f64` parameter is **untouched** — SEAT's
`shell-doors-take-tolerance-beside-tol` owns it (§6 owes SEAT a
measurement, nothing more).

## 2. The record

Keys are arena keys and nothing else; attributing them to stable
names is the document layer's job (the standing division
`MintedDeclaration`'s docs state). Two key spaces meet here and the
record says which is which on every row:

- **Source keys** — the operand's. The result body is a clone of the
  operand with the cavity grafted in, so a surviving OUTER entity
  keeps its operand key; a row still states it (no `Direct` shortcut a
  consumer has to know about — the thicken-every-boundary semantics
  will one day break the identity, and the row shape must not).
- **Result keys** — the returned body's. Cavity entities are born in
  the cavity clone (whose keys equal the operand's, because it IS a
  clone) and cross into the result through `insert_void`'s graft map
  (`VoidInserted`, `crates/topo/src/boolean/voids.rs`) — that map is
  the only bridge, and the record reads it at insertion time.

```rust
/// Mint-time naming facts of one shell (source keys ← the operand,
/// result keys ← the returned body). Rows are written as the doors
/// act, in the deterministic order the construction visits entities
/// (D9); rows are historical — a result key listed here may have
/// died in a LATER step, and every such death is listed in `dead`.
#[derive(Clone, Debug, Default)]
pub struct ShellNaming {
    /// Outer wall face (result) ← the source face it is. Every source
    /// face that was not designated open, face-arena order.
    pub outer: Vec<(FaceKey, FaceKey)>,
    /// Inner (cavity) twin face (result) ← the source face it was
    /// offset from. Every source face, face-arena order — a designated
    /// face's twin is listed too; it dies in the rim surgery and is
    /// then in `dead`.
    pub inner: Vec<(FaceKey, FaceKey)>,
    /// Inner twin edge (result) ← source edge, edge-arena order.
    pub inner_edges: Vec<(EdgeKey, EdgeKey)>,
    /// Inner twin vertex (result) ← source vertex, vertex-arena order.
    pub inner_vertices: Vec<(VertexKey, VertexKey)>,
    /// One row per designated CHART, in designation order (the order
    /// `open_faces` first names each chart).
    pub rims: Vec<RimNaming>,
    /// What the construction retired, result keys: the designated
    /// chart's merged-away faces and seam edges/apex vertices
    /// (`canonicalize_chart`, both sides), the cavity counterpart
    /// killed by `kfmrh`, and any promoted-rim scaffolding.
    pub dead: ShellRetired,
}

/// The rim a designated chart became.
#[derive(Clone, Debug)]
pub struct RimNaming {
    /// The designated faces of this chart, source keys, in
    /// designation order.
    pub sources: Vec<FaceKey>,
    /// The rim face (result): the survivor of the chart's reduction,
    /// now annular.
    pub rim: FaceKey,
    /// The rim's RING (result): the cavity counterpart's outer loop,
    /// as `kfmrh` returned it.
    pub ring: LoopKey,
    /// Ring edge (result) ← the source boundary edge of the
    /// designated chart it is the inward twin of; ring cycle order.
    pub ring_edges: Vec<(EdgeKey, EdgeKey)>,
    /// Ring vertex (result) ← the source boundary vertex it is the
    /// inward twin of; ring cycle order.
    pub ring_vertices: Vec<(VertexKey, VertexKey)>,
    /// A designated face with a hole yields one extra rim region per
    /// hole: the promoted rim face (result, `mfkrh`'s product) ← the
    /// source ring loop it pairs (source key), pairing order.
    pub holes: Vec<(FaceKey, LoopKey)>,
}

#[derive(Clone, Debug, Default)]
pub struct ShellRetired {
    pub faces: Vec<FaceKey>,
    pub edges: Vec<EdgeKey>,
    pub vertices: Vec<VertexKey>,
}
```

The exact field set is the kernel's to refine — LIB's ask (issue
record `work/shell/shell-needs-shellnaming-birth-channel.md`) is the
floor: wall per source face, inner twin per source face, rim per
opened face keyed to the source `FaceKey`, rim trim edges keyed to
the source boundary edges. Add a row if a step mints something the
list above cannot name; do not drop one. Every added or changed row
is disclosed in the PR body with the reason.

**Where each row is written** (the construction's own sites, in
`shell_open`):

| row | site |
|---|---|
| `outer` | after the designation check, from the operand's face walk |
| `inner*` | immediately after `insert_void`, by walking the cavity's arenas through `VoidInserted` |
| `rims[i].sources`, `.rim` | the per-chart loop: `sources` from the grouping read, `rim` from `canonicalize_chart(group)` |
| `rims[i].ring`, `.ring_edges`, `.ring_vertices` | from `kfmrh`'s `KfmrhResult` and a walk of that ring, each edge/vertex looked up in the `inner_*` rows written above |
| `rims[i].holes` | the `mfkrh` loop, `(made.face, rim_ring)` |
| `dead` | every `kef`/`kev`/`kemr`/`kfmrh` call's own result, at the call |

`ring_edges`' second column must be found in `inner_edges`' first
column — that lookup either succeeds or the construction has minted a
ring edge the record cannot explain, which is `ShellError::Corrupt`
territory, not a silent gap.

## 3. Consumers touched

Every caller of `shell` / `shell_open` takes `.body` (25 files at
main `0430cb55`; re-list at your merge base — the sweep pattern is
`topo::shell(` / `shell_open(` over `crates/` and `demos/`; it cannot
match a call through a `use topo::shell;` alias, so grep that spelling
too). This is mechanical and is the whole reason the unit is priced M
rather than S. `crates/verbs` (SEAT's) is NOT touched: the
`VerbRecord::Shell` arm is SEAT's migration and consumes this shape as
delivered. `demos/tour/src/teapot.rs` keeps its by-description scan
(LIB-G17 replaces it, not this unit); its `shell_open` read is
re-spelled and nothing else moves.

## 4. Acceptance (rows in `crates/sweep/tests/verbs_shell.rs`, or a
sibling `verbs_shell_naming.rs` sharing its fixtures)

Every row reads the record and checks it AGAINST THE BODY — a row that
only counts rows is decorative.

1. **Sealed box.** `outer` has six rows, each `(k, k)` resolving in
   the result on the `Outer` shell; `inner` has six rows, each twin
   resolving on the `Void` shell, its plane at distance exactly the
   wall from its source's plane along the source's inward normal;
   `inner_edges`/`inner_vertices` cover every source edge/vertex,
   injectively; `rims` and `dead` empty.
2. **Coverage, both arms.** For the box, the vessel cup and the
   two-ended tube: every live face of the result appears in exactly
   one of `outer`, `inner`, `rims[*].rim`, `rims[*].holes`; every live
   edge in exactly one of `outer`-implied identity (a source edge
   that survived), `inner_edges`, or a rim's `ring_edges`; and nothing
   in `dead` resolves. State the identity for outer edges/vertices in
   the type's docs (they keep operand keys) and PIN it here.
3. **The revolved cup** (`a_revolved_cap_opens_to_one_annular_rim`'s
   fixture): one `RimNaming`; `rim` is the survivor, carries exactly
   one ring equal to `.ring`; every `ring_edges` row's result edge is
   on that ring and its source edge bounded the designated chart in
   the operand; `dead` lists the merged half-cap's twin faces and the
   seam edges on BOTH sides (non-empty — this is the row that proves
   `dead` is written at the Euler calls, not inferred).
4. **The annular cap** (`an_annular_cap_opens_to_two_disjoint_rims`)
   and **the counterbored drum** (`shellfix1_r1_probes` P2): `holes`
   non-empty, each promoted face resolving, its outer loop the twin of
   the source ring the row names.
5. **Determinism.** Build the cup twice; the two records are equal
   field by field (D9 — order is a function of the construction).
6. **The ring-edge lookup refuses loud**: a unit-level red that
   shows what happens when a ring edge has no `inner_edges` row is
   not constructible through the public doors; state that in-file
   rather than planting a fake, and pin the invariant by the
   coverage row instead.
7. `shell_runs_no_intersection_machinery` and every existing shell
   row unchanged in claim; the Interval-lane shell rows
   (`torax_interval.rs`, `sf2b_interval_probe.rs`) compile and pass
   with `.body` only — ask for `CI-Config: lane=interval` on the head
   commit and say in the PR which lane gated.

## 5. Docs

- `crates/topo/src/shell.rs` module docs gain one section, "The
  record", stating §2's two key spaces and the historical-rows rule.
  Present tense; no unit tags.
- `crates/geom-brep/README.md`'s shelling vocabulary is unchanged.
  If `docs/KERNEL-VERBS.md`'s shell row states "returns a body",
  correct it.

## 6. Owed to neighbours (report, do not act)

- **SEAT** (`work/seat/shell-doors-take-tolerance-beside-tol`): while
  sweeping the callers, record in the PR body what every call passes
  for `tolerance` (literal, derived, or threaded) — the census that
  issue says must precede any change. Do not change the parameter.
- **LIB-G17**: the PR body states the delivered row set in one table
  so LIB's emitter can be written from it without reading the diff.
- **VERBS-RIMCAP** (`verbs/rimcap-1`, open): its tests call
  `topo::shell`. Whichever merges second merges main and re-spells.

## 7. Stops

STOP and report (do not work around) if: a step mints an entity the
rows above cannot name (report the shape; the record grows a row by
ratification, not by improvisation); or the graft map lacks a cavity
entity that is live in the result (a `VoidInserted` gap — its own
issue, not this unit's fix).

## 8. Lane rules

Own worktree, own `CARGO_TARGET_DIR` outside the checkout, never the
orchestrator's checkout, read other branches with `git show`. One
heavy cargo job at a time on this box. NO `Co-Authored-By` trailer in
lane commits (A/B blinding); if one lands in a pushed commit, note it
in the PR body and carry on. Push after every coherent step; hosted
CI is the gate; poll its run in the foreground.
