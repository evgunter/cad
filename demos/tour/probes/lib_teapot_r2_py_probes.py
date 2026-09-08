import math, sys, unittest, traceback
sys.path.insert(0, '/root/.local/share/cad-work/lib-teapot-r2/cad/crates/pncad-py/tests')
from pncad import *
import test_north_star as ns
T = ns.TestTeapot('test_the_vessel_hollows_sealed_and_opens_at_its_named_mouth')

# --- (1) canonical order, falsified by geometry ---
doc = Doc()
frame, axis = T.frame_and_axis(doc)
pot = T.revolved(doc, frame, axis, T.vessel_meridian())
sharp = T.revolved(doc, frame, axis, T.lid_meridian())
ev = evaluate(doc)
bands = T.seg_faces(ev, pot, SegTag.Band); bands_pi = T.seg_faces(ev, pot, SegTag.BandPi)
for i, n in enumerate(bands):
    p = ev.face_frame(pot, n)
    print(f"bands[{i}] origin.y={p.origin[1]} name={n}")
for i, n in enumerate(bands_pi):
    p = ev.face_frame(pot, n)
    print(f"bands_pi[{i}] origin.y={p.origin[1]}")
print("mouth halves at Y_MOUTH:", ev.face_frame(pot, bands[3]).origin[1] == T.Y_MOUTH * m, ev.face_frame(pot, bands_pi[3]).origin[1] == T.Y_MOUTH * m)
rims = T.rim_edges(ev, sharp)
for i, n in enumerate(rims):
    p = ev.edge_frame(sharp, n)
    print(f"rims[{i}] centre.y={p.origin[1]} name={n}")
first = doc.insert(Node.fillet(sharp, T.ROLL * m, [rims[1]]))
ev = evaluate(doc)
carried = ev.select(first, Selector.of(NamePat.of_kind(EntityKind.Edge).seg(SegPat.tag(SegTag.FromTarget).of([NamePat.any().seg(SegPat.tag(SegTag.BandRim))]))))
for i, n in enumerate(carried):
    print(f"carried[{i}] centre.y={ev.edge_frame(first, n).origin[1]}")
print("carried[1] at Y_FLANGE:", ev.edge_frame(first, carried[1]).origin[1] == T.Y_FLANGE * m, "carried[3] at Y_TOP:", ev.edge_frame(first, carried[3]).origin[1] == T.Y_TOP * m)

# --- (2) the refusal text, verbatim ---
doc = Doc()
*_, joins = T.teapot(doc)
ev = evaluate(doc)
for node in joins:
    try:
        ev.value(node)
    except EvaluationError as e:
        print("kind:", e.kind, "| text:", str(e))

# --- (3) the mutant: vertex 3 for vertex 4 ---
class Mutant(ns.TestTeapot):
    RIMS = ((1, 14.0 / 256.0), (2, 3.0 / 64.0), (3, 5.0 / 256.0))
for name in ['test_the_lid_rolls_the_three_rims_it_names']:
    t = Mutant(name)
    try:
        t.setUp(); getattr(t, name)(); print("MUTANT v=3: PASSED (row did not go red)")
    except Exception as e:
        print("MUTANT v=3: RED:", type(e).__name__, str(e)[:400])
