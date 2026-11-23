#!/usr/bin/env python3
"""R1 probe fixtures for PR #252 (M7-5 band seam re-mint).

Synthetic solids: torus band (major R=10, minor r=2, axis z, u_ref x)
glued to a cylinder band (radius 10, z in [-2,2]) along two shared rim
circles (radius 10 at z=+-2, one vertex each at azimuth 90deg so both
rims must split at u_ref). Both faces are SEAMLESS periodic bands.

  A: torus states the OUTER half (v -pi/2..pi/2), cylinder inner wall.
     V = 40*pi^2 + 32*pi/3
  B: torus states the INNER half (v pi/2..3pi/2), cylinder outer wall.
     V = 40*pi^2 - 32*pi/3
  C: A with the torus same_sense flipped -> consistently inside-out
     encoding of B's region; passes every face-local gate, must refuse
     at the kernel backstop (check 6 / negative volume), NEVER green.
  D: A with the cylinder same_sense flipped -> inverted cylinder band,
     must refuse typed pre-body ("ORIENTATION-INVERTED cylinder band").

Also doctors nist_ftc_11 for the C2 attack: rotate rim vertex #85 off
the u_ref azimuth by delta (still on its rim circle).
"""
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/home/evan/.local/share/cad-work/band-seam-review/cad"
FTC = REPO + "/crates/step-import/tests/fixtures/wild/nist/nist_ftc_11_asme1_rb.stp"

SCAFFOLD = r"""ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('R1 review probe: synthetic seamless band solid'),'2;1');
FILE_NAME('band_probe.stp','2026-08-08T00:00:00',(''),(''),'','','');
FILE_SCHEMA(('CONFIG_CONTROL_DESIGN'));
ENDSEC;
DATA;
#5=APPLICATION_CONTEXT('configuration controlled 3D designs of mechanical parts and assemblies');
#6=APPLICATION_PROTOCOL_DEFINITION('International Standard','ap203_configuration_controlled_3d_design_of_mechanical_parts_and_assemblies_mim_lf',2004,#5);
#7=PRODUCT_CONTEXT('',#5,'mechanical');
#8=PRODUCT('band_probe','band_probe',$,(#7));
#9=PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#8));
#10=PRODUCT_DEFINITION_FORMATION('',$,#8);
#11=PRODUCT_DEFINITION_CONTEXT('part definition',#5,'design');
#12=PRODUCT_DEFINITION('',$,#10,#11);
#18=(NAMED_UNIT(*)PLANE_ANGLE_UNIT()SI_UNIT($,.RADIAN.));
#19=DIMENSIONAL_EXPONENTS(0.0,0.0,0.0,0.0,0.0,0.0,0.0);
#20=PLANE_ANGLE_MEASURE_WITH_UNIT(PLANE_ANGLE_MEASURE(0.0174532925),#18);
#24=(CONVERSION_BASED_UNIT('DEGREE',#20)NAMED_UNIT(#19)PLANE_ANGLE_UNIT());
#28=(NAMED_UNIT(*)SI_UNIT($,.STERADIAN.)SOLID_ANGLE_UNIT());
#32=(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT(.MILLI.,.METRE.));
#34=UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(0.01),#32,'DISTANCE_ACCURACY_VALUE','');
#36=(GEOMETRIC_REPRESENTATION_CONTEXT(3)GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#34))GLOBAL_UNIT_ASSIGNED_CONTEXT((#24,#28,#32))REPRESENTATION_CONTEXT('None','None'));
#37=AXIS2_PLACEMENT_3D('',#38,#39,#40);
#38=CARTESIAN_POINT('',(0.0,0.0,0.0));
#39=DIRECTION('',(0.0,0.0,1.0));
#40=DIRECTION('',(1.0,0.0,0.0));
#41=SHAPE_REPRESENTATION('',(#37),#36);
#42=PRODUCT_DEFINITION_SHAPE('','',#12);
#43=SHAPE_DEFINITION_REPRESENTATION(#42,#41);
"""

def tf(b):
    return ".T." if b else ".F."

def body(t_bot, t_top, t_sense, c_bot, c_top, c_sense, cyl_first=False, vx=0.0, vy=10.0):
    e = []
    # torus surface (R=10 r=2, center origin, axis z, u_ref x)
    e.append("#100=CARTESIAN_POINT('',(0.0,0.0,0.0));")
    e.append("#101=DIRECTION('',(0.0,0.0,1.0));")
    e.append("#102=DIRECTION('',(1.0,0.0,0.0));")
    e.append("#103=AXIS2_PLACEMENT_3D('',#100,#101,#102);")
    e.append("#104=TOROIDAL_SURFACE('',#103,10.0,2.0);")
    # cylinder surface (radius 10, axis z through (0,0,-2))
    e.append("#105=CARTESIAN_POINT('',(0.0,0.0,-2.0));")
    e.append("#106=DIRECTION('',(0.0,0.0,1.0));")
    e.append("#107=DIRECTION('',(1.0,0.0,0.0));")
    e.append("#108=AXIS2_PLACEMENT_3D('',#105,#106,#107);")
    e.append("#109=CYLINDRICAL_SURFACE('',#108,10.0);")
    # bottom rim: circle radius 10 at z=-2, vertex at azimuth 90deg
    e.append("#110=CARTESIAN_POINT('',('+repr(vx)+','+repr(vy)+',-2.0));")
    e.append("#111=VERTEX_POINT('',#110);")
    e.append("#112=CARTESIAN_POINT('',(0.0,0.0,-2.0));")
    e.append("#113=DIRECTION('',(0.0,0.0,1.0));")
    e.append("#114=DIRECTION('',(1.0,0.0,0.0));")
    e.append("#115=AXIS2_PLACEMENT_3D('',#112,#113,#114);")
    e.append("#116=CIRCLE('',#115,10.0);")
    e.append("#117=EDGE_CURVE('',#111,#111,#116,.T.);")
    # top rim: circle radius 10 at z=+2, vertex at azimuth 90deg
    e.append("#120=CARTESIAN_POINT('',(0.0,10.0,2.0));")
    e.append("#121=VERTEX_POINT('',#120);")
    e.append("#122=CARTESIAN_POINT('',(0.0,0.0,2.0));")
    e.append("#123=DIRECTION('',(0.0,0.0,1.0));")
    e.append("#124=DIRECTION('',(1.0,0.0,0.0));")
    e.append("#125=AXIS2_PLACEMENT_3D('',#122,#123,#124);")
    e.append("#126=CIRCLE('',#125,10.0);")
    e.append("#127=EDGE_CURVE('',#121,#121,#126,.T.);")
    # torus face
    e.append(f"#130=ORIENTED_EDGE('',*,*,#117,{tf(t_bot)});")
    e.append("#131=EDGE_LOOP('',(#130));")
    e.append("#132=FACE_OUTER_BOUND('',#131,.T.);")
    e.append(f"#133=ORIENTED_EDGE('',*,*,#127,{tf(t_top)});")
    e.append("#134=EDGE_LOOP('',(#133));")
    e.append("#135=FACE_BOUND('',#134,.T.);")
    e.append(f"#136=ADVANCED_FACE('',(#132,#135),#104,{tf(t_sense)});")
    # cylinder face
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

def write(name, txt):
    p = os.path.join(HERE, name)
    open(p, "w", newline="").write(txt)
    print("wrote", p)

# A: torus outer half (start rim = bottom, traversed +u, sense T);
#    cylinder inner wall (sense F, bottom -u).
write("band_a.stp", body(t_bot=True, t_top=False, t_sense=True,
                         c_bot=False, c_top=True, c_sense=False))
# B: torus inner half (start rim = top); cylinder outer wall (sense T).
write("band_b.stp", body(t_bot=False, t_top=True, t_sense=True,
                         c_bot=True, c_top=False, c_sense=True))
# C: A with torus sense flipped -> inside-out encoding of B's region.
write("band_c_insideout.stp", body(t_bot=True, t_top=False, t_sense=False,
                                   c_bot=False, c_top=True, c_sense=False))
# D: A with cylinder sense flipped -> inverted cylinder band (pre-body
#    typed refusal expected). Cylinder listed first so its mint runs first.
write("band_d_invcyl.stp", body(t_bot=True, t_top=False, t_sense=True,
                                c_bot=False, c_top=True, c_sense=True,
                                cyl_first=True))

# C2 attack: rotate ftc_11's rim vertex #85 (on circle #90, r=16,
# center origin) off the u_ref azimuth by delta radians.
ftc = open(FTC, errors="replace").read()
def rotate_vertex(txt, delta):
    old = "#84=CARTESIAN_POINT('',(0.0,-16.0,0.0));"
    assert old in txt
    x = -16.0 * math.sin(delta)
    y = -16.0 * math.cos(delta)
    new = f"#84=CARTESIAN_POINT('',({x:.17g},{y:.17g},0.0));"
    return txt.replace(old, new)

# arc offset (kernel metres) = 0.016 m * delta
write("ftc11_voff_tiny.stp", rotate_vertex(ftc, 1e-10 / 0.016))   # 1e-10 m off
write("ftc11_voff_mid.stp", rotate_vertex(ftc, 1.6e-6 / 0.016))   # 1.6e-6 m off
# ref_direction attack: rotate the SURFACE u_ref of cylinder #72
# (placement #71, ref dir #70) by 1e-4 rad; vertices then sit
# 16mm*1e-4 = 1.6e-6 m off the seam azimuth.
d = 1e-4
old = "#70=DIRECTION('',(0.0,-1.0,0.0));"
assert old in ftc
new = f"#70=DIRECTION('',({math.sin(d):.17g},{-math.cos(d):.17g},0.0));"
write("ftc11_uref_off.stp", ftc.replace(old, new, 1))
