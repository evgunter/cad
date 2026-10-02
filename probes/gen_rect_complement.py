#!/usr/bin/env python3
"""Ball of radius R mm split into a chart rectangle (lat A..B, az 0..W)
and its complement. Writes rect_complement.step. Reviewer probe, PR 3843."""
import math, sys
R = 10.0
A, B, W = math.radians(10), math.radians(40), 1.0
out = sys.argv[1] if len(sys.argv) > 1 else "rect_complement.step"
L = []
n = [100]
def e(s):
    n[0] += 1; L.append(f"#{n[0]} = {s};"); return f"#{n[0]}"
def pt(p): return e("CARTESIAN_POINT('',(%r,%r,%r))" % tuple(float(x) for x in p))
def d(v): return e("DIRECTION('',(%r,%r,%r))" % tuple(float(x) for x in v))
def ax(o, z, x): return e(f"AXIS2_PLACEMENT_3D('',{pt(o)},{d(z)},{d(x)})")
def sph(lat, az): return (R*math.cos(lat)*math.cos(az), R*math.cos(lat)*math.sin(az), R*math.sin(lat))
V = {k: e(f"VERTEX_POINT('',{pt(sph(*v))})") for k, v in
     {1: (A, 0), 2: (A, W), 3: (B, W), 4: (B, 0)}.items()}
def circ(o, z, x, r): return e(f"CIRCLE('',{ax(o,z,x)},{float(r)!r})")
cA = circ((0, 0, R*math.sin(A)), (0, 0, 1), (1, 0, 0), R*math.cos(A))
cB = circ((0, 0, R*math.sin(B)), (0, 0, 1), (1, 0, 0), R*math.cos(B))
m1 = circ((0, 0, 0), (math.sin(W), -math.cos(W), 0), (math.cos(W), math.sin(W), 0), R)
m0 = circ((0, 0, 0), (0, -1, 0), (1, 0, 0), R)
E1 = e(f"EDGE_CURVE('',{V[1]},{V[2]},{cA},.T.)")
E2 = e(f"EDGE_CURVE('',{V[2]},{V[3]},{m1},.T.)")
E3 = e(f"EDGE_CURVE('',{V[4]},{V[3]},{cB},.T.)")
E4 = e(f"EDGE_CURVE('',{V[1]},{V[4]},{m0},.T.)")
S = e(f"SPHERICAL_SURFACE('',{ax((0,0,0),(0,0,1),(1,0,0))},{R!r})")
def face(edges, sense=".T."):
    oes = [e(f"ORIENTED_EDGE('',*,*,{ed},{s})") for ed, s in edges]
    lp = e("EDGE_LOOP('',(%s))" % ",".join(oes))
    return e(f"ADVANCED_FACE('',({e(f'FACE_OUTER_BOUND({chr(39)*2},{lp},.T.)')}),{S},{sense})")
LR = [(E1, ".T."), (E2, ".T."), (E3, ".F."), (E4, ".F.")]
LC = [(E4, ".T."), (E3, ".T."), (E2, ".F."), (E1, ".F.")]
if len(sys.argv) > 2 and sys.argv[2] == "swap": LR, LC = LC, LR
FR = face(LR)
FC = face(LC, ".F." if "senseF" in sys.argv else ".T.")
SH = e(f"CLOSED_SHELL('',({FR},{FC}))")
BR = e(f"MANIFOLD_SOLID_BREP('',{SH})")
head = """ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('rect complement probe'),'2;1');
FILE_NAME('rc','2026-10-02T00:00:00',('hand'),('hand'),'hand','hand','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));
ENDSEC;
DATA;
#1 = APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2);
#2 = APPLICATION_CONTEXT('core data for automotive mechanical design processes');
#3 = SHAPE_DEFINITION_REPRESENTATION(#4,#10);
#4 = PRODUCT_DEFINITION_SHAPE('','',#5);
#5 = PRODUCT_DEFINITION('design','',#6,#9);
#6 = PRODUCT_DEFINITION_FORMATION('','',#7);
#7 = PRODUCT('rc','rc','',(#8));
#8 = PRODUCT_CONTEXT('',#2,'mechanical');
#9 = PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design');
#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#14,%s),#74);
#11 = CARTESIAN_POINT('',(0.,0.,0.));
#12 = DIRECTION('',(0.,0.,1.));
#13 = DIRECTION('',(1.,0.,0.));
#14 = AXIS2_PLACEMENT_3D('',#11,#12,#13);
""" % BR
tail = """#70 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );
#71 = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) );
#72 = ( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() );
#73 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#70,'distance_accuracy_value','confusion accuracy');
#74 = ( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#73)) GLOBAL_UNIT_ASSIGNED_CONTEXT((#70,#71,#72)) REPRESENTATION_CONTEXT('Context #1','3D Context with UNIT and REPRESENTATION') );
ENDSEC;
END-ISO-10303-21;
"""
open(out, "w").write(head + "\n".join(L) + "\n" + tail)
