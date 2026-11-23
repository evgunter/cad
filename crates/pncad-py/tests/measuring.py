"""The predicates the suite measures with.

A measurement states what it asks (`[tolerance.measure]`: there is no
default), so a test that reads a volume or an area as a number asks for
it at a stated resolution, and a test that decides something asks the
question itself.
"""

from pncad import Settle, m

#: The resolution the suite's numbers are read at: 1024·ε of mean
#: boundary displacement at the binding's ε of 1e-9 m. The pins and
#: tolerances a test compares a volume or an area with were set where
#: every face met it, and a face that converges within the work wall
#: keeps its bits under it. A test whose assertion needs another width
#: states that one.
PINNED = Settle.resolution(1.024e-6 * m)


def met(measurement):
    """`measurement`'s enclosure, where every face met its resolution:
    `MassMeasurement.met()`, which raises where the resolution went
    unmet, so a bracket that stopped short is a failure of the row,
    named with its stop."""
    return measurement.met()


def pinned(body):
    """`body`'s volume and area read as numbers at `PINNED`."""
    return met(body.mass_properties(PINNED))
