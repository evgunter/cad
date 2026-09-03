# PCURVE — the edge-description unification program (plan)

**STATUS: RATIFIED (Evan, PR #1061, 2026-08-27).** A third
concurrent program beside LIB (usable-as-a-library) and VERBS
(kernel breadth), opened at M9's close on Evan's direction. Its
subject is the work U2 ratified and deliberately did NOT schedule:
migrating edge descriptions onto (surface, pcurve), plus the items
that have been waiting on that migration rather than on each other.

Branch prefix `pcurve/`; orchestrator branch `pcurve/orchestrator`;
away-channel tag `(PCURVE orchestrator)`. Live state is
`work/pcurve/log.md`'s tail, never this file.

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
  as a description for TRANSIENT edges only (Evan's Q2 answer; the
  fence criterion as corrected below).
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
   `General`; `MappedCurve` becomes the authority record behind the
   transience fence. **The binding constraint is NOT what
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
2. **P-2 — #498's home. NARROWED TO INTERIOR ISOS by its own
   substrate (2026-08-29); the diagonal half is SPLIT OUT.** Interior
   `Intersection` carriers inherit `General` as their named home.

   **What the substrate falsified.** The plan said these "carry a
   typed permanent refusal". True and reachable for INTERIOR isos
   (measured: `IsoUnsupported` on a widened chart, verbatim payload).
   **False as the binding statement for DIAGONAL loci** — those never
   reach `nurbs_iso_derive` at all, because the EDGE-certification
   lane refuses them first at limb 2 with `hull_sup ~= 2.6e-4 m`, five
   to six orders past every ε in the corpus. The cause is
   `PXN_IMAGE_DEGREE = 1` (`geom-brep/src/edge_nurbs.rs:376-394`),
   whose own doc states the class boundary and banks the fix to #264's
   envelope findings. The identical loci certify at 2.7e-11-1.5e-10 m
   through the SSI's own degree-3 image. **So the refusal's TEXT names
   the diagonal class; the CAUSE for that class is a degree constant
   in another crate at another lifecycle stage** — another instance
   of [[refusal-text-is-not-cause]] (the memory does not enumerate, so
   no ordinal is claimed), and the second time a PCURVE substrate has
   corrected this plan.

   **`General` costs nothing on the certification side.** Measured:
   BOTH sub-classes certify through `certify_general` today,
   unmodified, at the `(surface, mate)` pair `mate_surface` already
   computes. P-1 wired `General` through `chart_box`, `shift_branch`
   and `recertify`, so the loop walk, face window and tier-3 at-rest
   replay need nothing. The real work is (a) a DERIVER — the existing
   4-candidate closed-form schedule never tries an interior knot — and
   (b) raising `mint_pcurves`' bound to `PcurveFittedLane`, measured
   as 4 topo signatures plus 4 static sweep sites. **The bound is
   signature churn, not a capability loss**: `Dual<T>` implements
   `PcurveFittedLane` (refusing impl), so no scalar is excluded —
   correcting `certify_fitted`'s docs and DESIGN.md frontier (c),
   which both treat that ripple as blocking.

   **The exit is honest but narrower than #498's text.** The body
   builds and validates at rest; volume, area, tessellation and offset
   of the affected face then refuse TYPED at six named sites, every
   one of which cites "the cut-loft unit". That is a real improvement
   over "cannot be built" and it is what the spec will claim.

   **Already done, drop from the slate**: the "free retirement" bullet
   below (`replace_face`'s "a v-row is not an `IsoCurve`") was retired
   by P-1.
2b. **P-2b — the diagonal half, NOT SCHEDULED HERE.** Blocked on
   `PXN_IMAGE_DEGREE`, which is edge-certification work banked to
   #264. #498 should be read as two sub-classes with different owners;
   P-2 as scoped cannot flip the diagonal one however it lands.
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
- **RESOLVED (was "OPEN, needs Evan"; corrected 2026-08-27)**: the
  ratified scaffold answer (Q2 — `MappedCurve` retained as a
  description SOLELY for pre-body edges) **was not implementable as
  written**. `MappedCurve` reaches REST through `describe_minted_edges`
  and six fillet strut sites, so "pre-body" never fenced it. **Evan
  ratified the corrected criterion in chat: the boundary is
  TRANSIENCE.** Q2's substance — narrow `MappedCurve` rather than a
  dedicated `Scaffold` rung — is UNCHANGED and was explicitly not
  revisited ("i think i don't want to revisit Scaffold"); only the
  doc's account of where the fence falls was wrong. Carried into U2's
  STATUS line and binding on P-1b item 2. This bullet said the question
  was undecided for a day after it was decided — the stale half of a
  correction that landed in U2 and not here.
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
prefer-intrinsic rules, retained as a description only for transient
scaffolding; adoption still reproduces native descriptions
bitwise and D9 bit-replay holds; #498 executes its own retirement
text; every new row ε-row three-outcome honest;
hosted CI green on every merge; the walk convention applies at exit.
