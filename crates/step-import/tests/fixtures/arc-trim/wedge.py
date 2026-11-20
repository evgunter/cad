import sys, math
# usage: wedge.py <bspline|circle> <e2_same_sense T|F> <swap_e2_vertices 0|1>
kind, ss, swap = sys.argv[1], sys.argv[2], sys.argv[3] == '1'
R, H = 10.0, 5.0
L = []
n = [1000]
def e(s):
    n[0] += 1; L.append(f"#{n[0]}={s};"); return f"#{n[0]}"
def pt(p): return e("CARTESIAN_POINT('',(%r,%r,%r))" % tuple(float(x) for x in p))
def dr(d): return e("DIRECTION('',(%r,%r,%r))" % tuple(float(x) for x in d))
def ax(o, z, x): return e(f"AXIS2_PLACEMENT_3D('',{pt(o)},{dr(z)},{dr(x)})")
V = [(0,0,0),(R,0,0),(0,R,0),(0,0,H),(R,0,H),(0,R,H)]
VP = [e(f"VERTEX_POINT('',{pt(v)})") for v in V]
def line(a, b):
    d = [b[i]-a[i] for i in range(3)]; l = math.sqrt(sum(x*x for x in d)); d = [x/l for x in d]
    return e(f"LINE('',{pt(a)},{e(f'VECTOR({chr(39)}{chr(39)},{dr(d)},1.)')})")
def arc(z, bottom=False):
    if kind == 'comp' and bottom:
        P = [(0,R),(-R,R),(-R,0),(-R,-R),(0,-R),(R,-R),(R,0)]
        c = [pt((x,y,z)) for x,y in P]
        w = math.sqrt(0.5)
        return e(f"( BOUNDED_CURVE() B_SPLINE_CURVE(2,({','.join(c)}),.UNSPECIFIED.,.F.,.U.) B_SPLINE_CURVE_WITH_KNOTS((3,2,2,3),(0.0,1.0,2.0,3.0),.UNSPECIFIED.) CURVE() GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_CURVE((1.0,{w!r},1.0,{w!r},1.0,{w!r},1.0)) REPRESENTATION_ITEM('') )")
    if kind == 'circle':
        return e(f"CIRCLE('',{ax((0,0,z),(0,0,1),(1,0,0))},{R})")
    if kind == 'semi' and bottom:
        c = [pt((R,0,z)), pt((R,R,z)), pt((0,R,z)), pt((-R,R,z)), pt((-R,0,z))]
        w = math.sqrt(0.5)
        return e(f"( BOUNDED_CURVE() B_SPLINE_CURVE(2,({','.join(c)}),.UNSPECIFIED.,.F.,.U.) B_SPLINE_CURVE_WITH_KNOTS((3,2,3),(0.0,1.0,2.0),.UNSPECIFIED.) CURVE() GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_CURVE((1.0,{w!r},1.0,{w!r},1.0)) REPRESENTATION_ITEM('') )")
    c = [pt((R,0,z)), pt((R,R,z)), pt((0,R,z))]
    w = math.sqrt(0.5)
    return e(f"( BOUNDED_CURVE() B_SPLINE_CURVE(2,({','.join(c)}),.UNSPECIFIED.,.F.,.U.) B_SPLINE_CURVE_WITH_KNOTS((3,3),(0.0,1.0),.UNSPECIFIED.) CURVE() GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_CURVE((1.0,{w!r},1.0)) REPRESENTATION_ITEM('') )")
def edge(i, j, crv, s='.T.'):
    return e(f"EDGE_CURVE('',{VP[i]},{VP[j]},{crv},{s})")
E1 = edge(0,1,line(V[0],V[1]))
E2 = edge(2,1,arc(0.0),f'.{ss}.') if swap else edge(1,2,arc(0.0,True),f'.{ss}.')
E3 = edge(2,0,line(V[2],V[0]))
E4 = edge(3,4,line(V[3],V[4]))
E5 = edge(4,5,arc(H))
E6 = edge(5,3,line(V[5],V[3]))
E7 = edge(0,3,line(V[0],V[3]))
E8 = edge(1,4,line(V[1],V[4]))
E9 = edge(2,5,line(V[2],V[5]))
def face(surf, uses, sense):
    oes = [e(f"ORIENTED_EDGE('',*,*,{ed},.{o}.)") for ed, o in uses]
    lp = e(f"EDGE_LOOP('',({','.join(oes)}))")
    b = e(f"FACE_OUTER_BOUND('',{lp},.T.)")
    return e(f"ADVANCED_FACE('',({b}),{surf},.{sense}.)")
F = [
 face(e(f"PLANE('',{ax((0,0,0),(0,0,1),(1,0,0))})"), [(E3,'F'),(E2,'F'),(E1,'F')], 'F'),
 face(e(f"PLANE('',{ax((0,0,H),(0,0,1),(1,0,0))})"), [(E4,'T'),(E5,'T'),(E6,'T')], 'T'),
 face(e(f"PLANE('',{ax((0,0,0),(0,-1,0),(1,0,0))})"), [(E1,'T'),(E8,'T'),(E4,'F'),(E7,'F')], 'T'),
 face(e(f"PLANE('',{ax((0,0,0),(-1,0,0),(0,1,0))})"), [(E7,'T'),(E6,'F'),(E9,'F'),(E3,'T')], 'T'),
 face(e(f"CYLINDRICAL_SURFACE('',{ax((0,0,0),(0,0,1),(1,0,0))},{R})"), [(E2,'T'),(E9,'T'),(E5,'F'),(E8,'F')], 'T'),
]
sh = e(f"CLOSED_SHELL('',({','.join(F)}))")
so = e(f"MANIFOLD_SOLID_BREP('Solid1',{sh})")
head = open(sys.argv[4]).read()
print(head.replace('@BODY@', '\n'.join(L)).replace('@SOLID@', so))
