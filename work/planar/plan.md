# PLANAR — the plan

The plane instance of C5's co-surface arm, and its readers. Present
state only; the story is in `log.md`.

## The units, in order

| # | unit | cost | waits on | tier |
|---|---|---|---|---|
| 1 | `a-pose-proves-two-spans-apart-from-their-covers` | H | — | dual |
| 2 | `two-curves-in-one-plane-meet-by-the-in-plane-arms` | H | 1 | dual |
| 3 | `check-9-reads-ring-against-outer-through-the-plane-instance` | H | 2 | dual |
| 4 | `the-census-reads-one-plane-records-pairs-through-the-plane-instance` | H | 2 | single FULL |
| 5 | `the-seg-differential-agrees-with-the-plane-instance` | H | 2, ONECOPY's touch kind | single FULL |
| 6 | `profile-validation-reads-its-pairs-through-the-plane-instance` | H | 5 | dual |
| 7 | `both-planar-ray-walks-share-one-driver-beside-the-reader` | H | 5 | single FULL |

ONECOPY's `a-touchs-crossing-or-side-is-re-decided-from-tangents-at-its-one-point`
(P1) puts the kind on `curve_pair::Meeting`; it can run beside units 1
and 2, and unit 5 waits on it.

## Parallelism

- Unit 1 and ONECOPY's touch-kind row run at once: both change
  `curve_pair`, in different functions (the pose's stages against the
  meeting's type), so they share files and not logic.
- Units 3, 4 and 5 run at once once unit 2 lands: three crates' readers,
  none reading another's change.
- Units 6 and 7 run at once once unit 5 agrees: the pair pass and the
  ray walk are separate passes of `validate.rs`, and unit 7's move of
  `ray_walk` touches nothing unit 6 changes.

## Questions the ruling does not settle

Listed in the cut's PR (the PR that opened this program) and in the
body of the unit each blocks. None is decided here.

## The rows this program carries

- `check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector`
  (P1) and `point-in-carrier-loop-is-a-second-copy-of-seg-ray-crossings`
  (P2), the ruling's two rows, parked on the units that close them.
- Claimed from CPAIR, RESTFRONT and PROFILE, each with `parent:` set to
  its carrier: `the-curve-pair-pose-reads-a-conics-whole-carrier-before-its-arc`
  (unit 1), `check-9-drops-the-one-circle-escalation` and
  `check-9-lines-meet-escalates-an-in-band-side-before-its-collinear-arm`
  (unit 3), `validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc`
  (unit 6), `profile-ray-casting-exhaustion-is-a-copy-of-the-ray-walk`
  (unit 7).
