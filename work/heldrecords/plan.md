# HELDRECORDS — the plan

No orchestrator yet. Live state is `work/heldrecords/log.md`'s tail and the
item files beside this plan.

Branch prefix **`heldrecords/`**; away-channel tag `(HELDRECORDS orchestrator)`.

Opened 2026-10-28 by SPLITDOOR's seam cut. The rows moved from
`work/splitdoor/` by `git mv`, with their ids and bodies unchanged.

## Charter

Ev ratified the shape on PR 5967. The finish line is `split` gating each
side at tier 3′ against the records it holds, re-measured at three ε with
no mover left. The order is Ev's ratified sequence:

1. **The type and door change.** `a-finished-body-holds-its-contact-records-at-every-door`
   is an X row and the first dispatch. Its design is decided, so it needs
   no designers. It changes a type every door reads, so its review tier is
   dual.
2. **Records in, shared crossings.** The split takes its operand's records,
   and crossings on one recorded locus share one point key
   (`the-splits-own-pinch-and-cut-copies-take-two-point-keys-and-no-record`).
   That row's "pinch copies" framing is off: pinch copies already share the
   point, and the failing cases are crossings.
3. **The split records its own coincident edges.** It cites a new `SplitOn`
   edge row (`a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`).
   The census's shared-bounds exemption then comes out, once the Boolean's
   pinch and slit lanes record too.
4. **The rest.**
   - The elliptic hole's nesting: `a-hole-in-an-elliptic-section-keeps-a-face-of-its-own-that-tier-3-prime-refuses`.
   - The in-band census typing: `a-split-side-whose-census-escalates-in-band-refuses-as-not-at-rest`.
   - The coincident rim's record: `a-union-touching-along-a-coincident-rim-records-only-its-vertex-pair`.
   - ATREST's curved count row.
   - REACHHOLD's cross-solid lane, for the Boolean's own verdict.
5. **Flip the gate**, then re-measure at three ε, including the unfiltered
   editor-core suite.

## Exit criteria

None set: the program closes when its slate is empty.

## From TOUCHREC (merged 2026-11-20)

No orchestrator: the first sitting (2026-11-01) worked the P1 rows and
handed the track back `ready`. Live state is `docs/doc-ledger/touchrec-merges-into-heldrecords.md`'s
tail and the item files beside this plan.

Branch prefix **`touchrec/`**; away-channel tag `(TOUCHREC orchestrator)`.

Opened 2026-11-01 by CLEAVE's split on its semantic seams (Ev, in chat): every
row moved from `work/cleave/` by `git mv`, id and body unchanged, each
with a dated `## Slate triage` note.

### Charter

**A result that is right but cannot say why.** Results touching along a slit's or ring's line, or two lumps along an edge less a flush partner, build without their contact records and fail tier 3′; a single cap near a rod's axis fails 3′ on its own records. The Eq. 15.3 lump keeps operand A's copy of a coincident region by its own table, and a Zero vertex pair keeps its smallest point key, not the glue's kept side. A boolean's row on a vertex it minted inside a face names the face and not the pierce.

HELDRECORDS holds the same question at every door; this track is the boolean's own records of its touches. CLEAVE's PR 6185 filed a row on a fate touch-end whose partner record is missing upstream; it moved here when 6185 merged.

### The slate

| pri | status | item | cost | title |
|---|---|---|---|---|
| P2 | open | `a-kept-vertex-touching-a-face-at-another-pierces-ring-has-no-partner-record` | M | A kept vertex resting on a face at a point another kept vertex pierces is recorded on the face, not paired with that pierce's ring vertex |
| P2 | open | `a-single-cap-near-a-rods-axis-fails-tier-3-prime-on-its-own-records` | M | Two rods at right angles, one capped about 1e-4 off the other's axis, build union and capped-minus-full bodies that fail tier 3′ on their own records |
| P2 | open | `a-union-beside-a-slit-misses-an-edge-overlap-record` | M | A union of a slit form with a part flush beside the slit, its edge along the slit's line, builds without the edge-overlap contact record: tier 3' refuses UnrecordedContact EdgeEdgeOverlap |
| P2 | open | `a-union-touching-along-a-rings-line-fails-tier-3-prime` | M | A result whose two solids touch along a slit's or ring's line, where one holds an edge, fails tier 3′ CensusUndecidable |
| P2 | open | `two-lumps-touching-along-an-edge-less-a-flush-partner-miss-their-line-records` | M | Two lumps touching along an edge, less a flush partner over part of the edge, build without the records of their touch: tier 3' refuses UnrecordedContact |
| P3 | open | `a-ring-vertex-minted-in-the-pierced-face-has-no-mints-row` | M | The ring vertices a boolean mints in a pierced face and at a node are not in Mints, so a row naming one would publish a clone key as an input vertex |
| P4 | open | `operands-convert-to-array-indices-by-hand` | E | An operand and its array index convert by hand at about 25 sites in crates/topo/src, with no one conversion |

### Posture for the next sitting

- **The kept side is one fact** (PR 6210): `boolean::glue::KEPT` (A, by
  `[coincidence.booleans]`), guarded at compile time; the lump table,
  its readers, the zip and the Zero record classes all read it, and
  `share_points` orders every class by `ops::PointOwners`. Code
  elsewhere in the boolean still assumes A (flipping `KEPT` breaks two
  `union_flush_onto_edge_contact` subtracts), which is why the guard
  exists: a ruled constant is not a knob.
- **The tier-3′ records** (the five P2 rows) are the track's body.
  `a-union-beside-a-slit-misses-an-edge-overlap-record` and
  `two-lumps-touching-along-an-edge-less-a-flush-partner-miss-their-line-records`
  share their witnesses with CLEAVE's line-wedge rows
  (`a_keel_on_the_floor_subtracts_as_a_slit.rs`,
  `union_flush_onto_edge_contact.rs`), so announce the seam with CLEAVE
  while either is live. The census over a Boolean result is parked on
  REACHHOLD (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`).

### Exit criteria

None set: the program closes when its slate is empty.
