#!/usr/bin/env python3
"""R1 probe fixtures, round 2.

Vertex azimuth is now a parameter (the 90 deg placement of round 1
lands in chart_direction's wrap-misread window AND leaves quarter-arc
rims after splits; 180 deg is safe on both counts, isolating the
D2-adjudication behavior from those defects).

Two-band solids (torus band + cylinder band, shared rims, vertex at
180 deg -> rims split at u_ref into two half-turn arcs, both readable):
  a180 outer / b180 inner / c180 inside-out / d180 inverted-cylinder.

Washer solids (ONE torus band + two planar discs at z=+-2, rims
radius 10): the enclosed solid is the column {rho<=10,|z|<=2} plus the
outer torus bulge, V = 400 pi + 40 pi^2 + 32 pi/3 mm3.
  washer180: control (vertex azimuth 180, correct winding read).
  washer90:  identical geometry, vertex azimuth 90 -> chart_direction
             samples straddle the u = pi wrap -> winding misread ->
             the mint decodes the COMPLEMENT (inner) torus half. What
             happens next (silent wrong body vs loud refusal) is the
             point of the probe.
"""
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))

SCAFFOLD = open(os.path.join(HERE, "band_a.stp")).read().split("#100=")[0]

def tf(b):
    return ".T." if b else ".F."

def rims(e, az_deg):
    a = math.radians(az_deg)
    x, y = 10.0 * math.cos(a), 10.0 * math.sin(a)
    e.append(f"#110=CARTESIAN_POINT('',({x:.17g},{y:.17g},-2.0));")
    e.append("#111=VERTEX_POINT('',#110);")
    e.append("#112=CARTESIAN_POINT('',(0.0,0.0,-2.0));")
    e.append("#113=DIRECTION('',(0.0,0.0,1.0));")
    e.append("#114=DIRECTION('',(1.0,0.0,0.0));")
    e.append("#115=AXIS2_PLACEMENT_3D('',#112,#113,#114);")
    e.append("#116=CIRCLE('',#115,10.0);")
    e.append("#117=EDGE_CURVE('',#111,#111,#116,.T.);")
    e.append(f"#120=CARTESIAN_POINT('',({x:.17g},{y:.17g},2.0));")
    e.append("#121=VERTEX_POINT('',#120);")
    e.append("#122=CARTESIAN_POINT('',(0.0,0.0,2.0));")
    e.append("#123=DIRECTION('',(0.0,0.0,1.0));")
    e.append("#124=DIRECTION('',(1.0,0.0,0.0));")
    e.append("#125=AXIS2_PLACEMENT_3D('',#122,#123,#124);")
    e.append("#126=CIRCLE('',#125,10.0);")
    e.append("#127=EDGE_CURVE('',#121,#121,#126,.T.);")

def torus_face(e, t_bot, t_top, t_sense):
    e.append("#100=CARTESIAN_POINT('',(0.0,0.0,0.0));")
    e.append("#101=DIRECTION('',(0.0,0.0,1.0));")
    e.append("#102=DIRECTION('',(1.0,0.0,0.0));")
    e.append("#103=AXIS2_PLACEMENT_3D('',#100,#101,#102);")
    e.append("#104=TOROIDAL_SURFACE('',#103,10.0,2.0);")
    e.append(f"#130=ORIENTED_EDGE('',*,*,#117,{tf(t_bot)});")
    e.append("#131=EDGE_LOOP('',(#130));")
    e.append("#132=FACE_OUTER_BOUND('',#131,.T.);")
    e.append(f"#133=ORIENTED_EDGE('',*,*,#127,{tf(t_top)});")
    e.append("#134=EDGE_LOOP('',(#133));")
    e.append("#135=FACE_BOUND('',#134,.T.);")
    e.append(f"#136=ADVANCED_FACE('',(#132,#135),#104,{tf(t_sense)});")

def pair(t_bot, t_top, t_sense, c_bot, c_top, c_sense, cyl_first=False, az=180.0):
    e = []
    rims(e, az)
    torus_face(e, t_bot, t_top, t_sense)
    e.append("#105=CARTESIAN_POINT('',(0.0,0.0,-2.0));")
    e.append("#106=DIRECTION('',(0.0,0.0,1.0));")
    e.append("#107=DIRECTION('',(1.0,0.0,0.0));")
    e.append("#108=AXIS2_PLACEMENT_3D('',#105,#106,#107);")
    e.append("#109=CYLINDRICAL_SURFACE('',#108,10.0);")
    e.append(f"#140=ORIENTED_EDGE('',*,*,#117,{tf(c_bot)});")
    e.append("#141=EDGE_LOOP('',(#140));")
    e.append("#142=FACE_OUTER_BOUND('',#141,.T.);")
    e.append(f"#143=ORIENTED_EDGE('',*,*,#127,{tf(c_top)});")
    e.append("#144=EDGE_LOOP('',(#143));")
    e.append("#145=FACE_BOUND('',#144,.T.);")
    e.append(f"#146=ADVANCED_FACE('',(#142,#145),#109,{tf(c_sense)});")
    faces = "(#146,#136)" if cyl_first else "(#136,#146)"
    e.append(f"#150=CLOSED_SHELL('',{faces});")
    e.append("#151=MANIFOLD_SOLID_BREP('Solid1',#150);")
    e.append("#152=ADVANCED_BREP_SHAPE_REPRESENTATION('ABSR',(#151),#36);")
    e.append("#153=SHAPE_REPRESENTATION_RELATIONSHIP('SRR','None',#152,#41);")
    return SCAFFOLD + "\n".join(e) + "\nENDSEC;\nEND-ISO-10303-21;\n"

def washer(az):
    # torus outer half (bot rim +u, sense T) + two discs.
    e = []
    rims(e, az)
    torus_face(e, t_bot=True, t_top=False, t_sense=True)
    # bottom disc: plane z=-2, chart normal +z, outward -z -> sense F;
    # boundary pairs against torus's bot +u use -> disc uses -u (.F.).
    e.append("#160=CARTESIAN_POINT('',(0.0,0.0,-2.0));")
    e.append("#161=DIRECTION('',(0.0,0.0,1.0));")
    e.append("#162=DIRECTION('',(1.0,0.0,0.0));")
    e.append("#163=AXIS2_PLACEMENT_3D('',#160,#161,#162);")
    e.append("#164=PLANE('',#163);")
    e.append("#165=ORIENTED_EDGE('',*,*,#117,.F.);")
    e.append("#166=EDGE_LOOP('',(#165));")
    e.append("#167=FACE_OUTER_BOUND('',#166,.T.);")
    e.append("#168=ADVANCED_FACE('',(#167),#164,.F.);")
    # top disc: plane z=+2, outward +z -> sense T; pairs against torus
    # top -u use -> disc uses +u (.T.).
    e.append("#170=CARTESIAN_POINT('',(0.0,0.0,2.0));")
    e.append("#171=DIRECTION('',(0.0,0.0,1.0));")
    e.append("#172=DIRECTION('',(1.0,0.0,0.0));")
    e.append("#173=AXIS2_PLACEMENT_3D('',#170,#171,#172);")
    e.append("#174=PLANE('',#173);")
    e.append("#175=ORIENTED_EDGE('',*,*,#127,.T.);")
    e.append("#176=EDGE_LOOP('',(#175));")
    e.append("#177=FACE_OUTER_BOUND('',#176,.T.);")
    e.append("#178=ADVANCED_FACE('',(#177),#174,.T.);")
    e.append("#150=CLOSED_SHELL('',(#136,#168,#178));")
    e.append("#151=MANIFOLD_SOLID_BREP('Solid1',#150);")
    e.append("#152=ADVANCED_BREP_SHAPE_REPRESENTATION('ABSR',(#151),#36);")
    e.append("#153=SHAPE_REPRESENTATION_RELATIONSHIP('SRR','None',#152,#41);")
    return SCAFFOLD + "\n".join(e) + "\nENDSEC;\nEND-ISO-10303-21;\n"

def write(name, txt):
    open(os.path.join(HERE, name), "w", newline="").write(txt)
    print("wrote", name)

write("band_a180.stp", pair(True, False, True, False, True, False))
write("band_b180.stp", pair(False, True, True, True, False, True))
write("band_c180.stp", pair(True, False, False, False, True, False))
write("band_d180.stp", pair(True, False, True, False, True, True, cyl_first=True))
write("washer180.stp", washer(180.0))
write("washer90.stp", washer(90.0))
