# Dual review: short readout at DR-147 (an M-tier miss)

Rule 9 owes a short readout at any M-tier miss that does not trigger a
full one: an FYI naming the miss, with no options (Ev, 2026-10-08, PR
4283). DR-147 is the first M-tier miss after DR-144, the trigger of
readout 4 (ruled on PR 4555). One more M-tier miss before twenty more
M-tier units makes the next full readout due.

## The unit

GERM, PR #4530: the section certificate answers torus × a cylinder
whose axis is square to the torus axis (a radial hole through a tube).
It is a new certificate arm, with difficulty M / TRICKY and arm HOLDOUT
(byte 222, mod 3 = 0). Both reviews ran concurrently, from one brief, on
the frozen head `102aa89a7`.

## The miss

R2 raised a **MAJOR**: the PR's new slow-set rows fail at the nightly's
ε rows.

- The square-wall search fails at ε = 1e-12 on 4 of 4 fresh seeds. Its
  "in-band" tilts, 1e-18..1e-13, fall outside the band at that ε, so
  the arm correctly refuses R-reach and the harness counts the
  refusal as a mismatch.
- The same search fails at ε = 1e-6 on 1 of 5 seeds. An R-tan refusal
  window is judged against the pose family's own margin only, not
  against every degeneracy.
- A tilted-rod row fails at ε = 1e-6: a fixed 1e-9 volume tolerance
  against a correct body that is 4e-9 off.

Each failure was shown by a red run at a named seed. They are test
code, not kernel code. Neither reviewer found a wrong answer in the
arm itself. The per-PR CI was green throughout, because its ε rows skip
the slow set, so only the nightly would have shown these.

R1 raised no MAJOR. It ran the slow search at the default ε only and
never mentions the ε rows. Taken first, the sequential arm would have
shipped the rows and turned the nightly red at two ε rows.

## How it was coded

The coding was blinded (byte 115). The finding was tallied once: the
three failures were counted as one defect under 6(c), because they share
one cause, new slow rows validated only at the default ε with ε-unaware
bounds. That was a close call, since the mechanisms differ: a tilt
range, a refusal window, and a volume tolerance. The pair is fair: the
briefs were identical, there was no relaxation, neither review was
interrupted, and neither disclosed a glimpse of the other.

## What became of it

The fix pass took the finding and swept the whole diff for the pattern:

- the tilts are drawn from the band through the arm's own lever;
- every refusal window checks every degeneracy;
- every carrier and volume tolerance is ε-aware;
- the mpmath oracle reads the run's ε.

Every touched row passes at all three ε. The unit merged as DR-147.
