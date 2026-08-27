# PCURVE — the edge-description unification program (plan)

**STATUS: RATIFIED (Evan, PR #1061, 2026-08-27).** A third
concurrent program beside LIB (usable-as-a-library) and VERBS
(kernel breadth), opened at M9's close on Evan's direction. Its
subject is the work U2 ratified and deliberately did NOT schedule:
migrating edge descriptions onto (surface, pcurve), plus the items
that have been waiting on that migration rather than on each other.

Branch prefix `pcurve/`; orchestrator branch `pcurve/orchestrator`;
away-channel tag `(PCURVE orchestrator)`. Live state is
`docs/PCURVE-LOG.md`'s tail, never this file.

## Why a program and not a unit

The migration was ratified as design in M9-D (#514, U2) with
scheduling explicitly delegated — "a post-M9 kernel candidate,
orchestrator-scheduled". It never got a tracking issue, which is how
a ratified design becomes invisible work. It is also not one unit:
the description change touches the kernel's edge vocabulary, its
certification lanes, adoption's bitwise reproduction contract, and
at least three consumers that are currently blocked or paying for
its absence. That is a track.

## Ratified ground (not re-litigated here)

`docs/PCURVE-UNIFY-DESIGN.md`, ratified by Evan 2026-08-15 (#514):

- **U2**: `EdgeGeometry`'s conventional variants collapse to ONE
  form — **(surface, `Pcurve`)** — while `Pcurve` KEEPS its exact
  variants as CERTIFICATION LANES (Harmonic / IsoLine / IsoArc /
  Fitted) and gains a `General` curve-in-UV arm at the honest Fitted
  grade. The classes survive as what they really are: exactness
  certificates, not taxonomy.
- **`MappedCurve` demotes to an AUTHORITY RECORD** carried beside
  the description, with tier-3's prefer-intrinsic rules reading the
  record instead of the negative space. A narrow scaffold retains it
  as a description for PRE-BODY edges only (Evan's Q2 answer).
- **Q3 (the authority record's home) is per-edge KERNEL data**,
  adopted by dominant argument. Its pushback window closed
  unexercised at M9's ratification (#1041), so this is settled.
- **OQ4 is NOT re-opened** — carrier-primary stands; this unifies
  descriptions, not the primary geometry.
- Planar faces keep ZERO stored pcurves: derive-on-demand becomes
  THE single door rather than one of two taxonomies.

## The slate

1. **P-1 — the migration itself.** `EdgeGeometry`'s conventional
   variants collapse to (surface, `Pcurve`); `Pcurve` gains
   `General`; `MappedCurve` becomes the authority record with the
   pre-body scaffold fenced. **The binding constraint is NOT what
   this plan first said** (corrected from P-1's substrate,
   2026-08-27). It named adoption's bitwise reproduction;
   `bitwise_iso_match` (adopt.rs:958) in fact quantifies over
   CARRIER NURBS PAYLOAD bits, which the migration never touches,
   and the description-side assertions there are class-COUNT checks
   that move mechanically. The real expense is **residual-meter
   incommensurability**: the three conventional forms do not measure
   the same thing — `Mapped` has no surface at all, and `Seam`
   carries two half-plane/side statements with no home in
   `|C − S(P)|`. `Certificate.max_residual` is byte-pinned and the
   iso residual's arithmetic order differs from `Pcurve::eval`'s, so
   "the same number" is not free. A spec that plans around adoption
   and ignores the meters will mis-scope the unit.
2. **P-2 — #498's home.** Interior/diagonal `Intersection` carriers
   (the trimmed-NURBS/cut-loft pcurve lane) inherit `General` as
   their named home. Today they carry a typed permanent refusal.
3. **P-3 — REMOVED FROM THE SLATE (P-1 substrate, 2026-08-27).**
   This item said lily wall 8's `CurvedEdgeUnsupported` flips with
   the migration. **It does not.** `gate_operand_edges`
   (boolean/reduce.rs:368-380) refuses on the EDGE CARRIER's kind —
   `Curve3::Nurbs(_)` — which is carrier geometry, not an edge
   DESCRIPTION. The migration changes descriptions and leaves that
   gate untouched, so wall 8 cannot flip here however P-1 lands. Its
   real dependency is a NURBS-carrier arm in the operand gate:
   breadth work, belonging with the register that owns kind gates.
   The fourth instance in this project of a stated blocker not being
   the binding one (after wall 7/#1031 and wall 2/#1059).

## Adjacent, sequenced but not owned yet

- **#1058 item 3** — `topo::shell` re-mints the whole body's pcurve
  map per call, so an `n`-chart body pays `n + k` whole-body mints.
  MEASURED non-urgent (16–23 ms at 3–6 charts in release, quadratic
  term invisible), with the lever named: a composite door deferring
  the mint to one call, for which `pcurves::staleness_posture::
  DECLARED` already has the vocabulary. Rides P-1 only if P-1's
  shape makes it cheap; otherwise it stays measured-and-banked.

## Excluded, named

The germ-chord / transverse curved×curved SSI lane (banked, and
NOT description work); #968's torus declared-`Rest` lane and
#1059's wall-2 gate question (both operand-gate admission, not
descriptions); M10's clearance certificate. Naming them because
"pcurve" is a tempting bucket for anything curved-and-hard.

## Amendments from P-1's substrate (2026-08-27, before any spec)

- **P-1 SPLITS**, at the `geom-brep` crate boundary: **P-1a** =
  representation + meter + authority record; **P-1b** = consumers +
  the scaffold fence + tests. The taxonomy is bigger than the design
  estimated — 16 multi-variant match groups, 163 `EdgeGeometry::`
  sites in `src/` and 152 across 53 test files, against the design's
  "~10 dispatch sites". A `Copy`/non-`Copy` mismatch (`EdgeGeometry`
  is `Copy`, `Pcurve` is not) breaks 22 deref sites by itself.
- **`General` is the CHEAP piece, not the risky one**:
  `certify_fitted` already exists, is callerless, and its own docs
  name U2's `General` arm as its waiting consumer.
- **The M9-3 payoff HELD in part, measured**: the planar zip emits
  100% `Intersection` (migrates free); the curved boss union leaves
  3 `MappedCurve` at rest. Both intrinsic emission arms migrate for
  free; the two conventional arms need work but are well-posed.
  (The substrate also reported that M9-3's promised arc/circle
  emission site never landed. **That is wrong and is not carried
  here** — `EdgeCurveSpec::arc_of_circle` exists at certify.rs:427
  and is called at ops.rs:1064, the D6 conventional re-description
  lane M9-3 shipped. The seam MINT path is a different site, and
  M9-3's own PR body said plainly it stays a straight chord.)
- **OPEN, and it needs Evan**: the ratified scaffold answer (Q2 —
  `MappedCurve` retained as a description SOLELY for pre-body edges)
  **is not implementable as written**. `MappedCurve` reaches REST
  through `describe_minted_edges` and six fillet strut sites, so
  "pre-body" does not fence it. The implementable fence is
  **transience**, not pre-body. Recorded as a design question, not
  decided here.
- **Free retirement, unclaimed by any plan**: `replace_face.rs:1249`'s
  "a v-row is not an `IsoCurve`" refusal retires with the
  representation change.

## Process

Standard: substrate → binding spec → one implementer + one blinded
reviewer + fix pass; A/B rows AT MERGE with per-phase figures;
ordinals claimed on main at review dispatch; hosted CI the only
gate; the standing brief lines (OUTPUT DISCIPLINE, the verbatim
foreground sentence AND its `setsid` exception, lane-private
publish paths, no `Co-Authored-By` in lane commits, k-lint
discipline, merge-main + build the union).

## Exit shape (proposed)

`EdgeGeometry` has ONE conventional form; the exact classes survive
as certification lanes with `General` as the honest floor;
`MappedCurve` is an authority record read by tier 3's
prefer-intrinsic rules, retained as a description only for pre-body
scaffolding; adoption still reproduces native descriptions
bitwise and D9 bit-replay holds; #498 and lily wall 8 execute their
own retirement texts; every new row ε-row three-outcome honest;
hosted CI green on every merge; the walk convention applies at exit.
