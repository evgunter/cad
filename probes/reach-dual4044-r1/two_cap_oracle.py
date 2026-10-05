"""Oracle: volume of unit ball ∩ {n2.p >= s2} ∩ {n1.p < s1} by slicing
along n2 (each slice: a disc ∩ half-plane, closed form), composite GL."""
import math
def dirv(lat, az):
    la, z = math.radians(lat), math.radians(az)
    return (math.cos(la)*math.cos(z), math.sin(la), math.cos(la)*math.sin(z))
dot = lambda a,b: sum(x*y for x,y in zip(a,b))
def seg(R2, v0):
    if R2 <= 0: return 0.0
    R = math.sqrt(R2)
    if v0 >= R: return math.pi*R2
    if v0 <= -R: return 0.0
    return R2*math.acos(-v0/R) + v0*math.sqrt(R2-v0*v0)
def vol(n1, s1, n2, s2):
    c12 = dot(n1, n2); g = math.sqrt(1-c12*c12)
    def area(t):
        R2 = 1 - t*t
        # slice centre n2*t; side n1.p < s1 → signed dist into side = (s1 - c12*t)/g
        return seg(R2, (s1 - c12*t)/g)
    xs = [0.0,-0.5384693101056831,0.5384693101056831,-0.9061798459386640,0.9061798459386640]
    ws = [0.5688888888888889,0.4786286704993665,0.4786286704993665,0.2369268850561891,0.2369268850561891]
    a, b = s2, 1.0; N = 40000; h = (b-a)/N; tot = 0.0
    for k in range(N):
        m = a+(k+.5)*h
        tot += sum(w*area(m+x*h/2) for x,w in zip(xs,ws))*h/2
    return tot
n1 = dirv(70, 90)
for lat, az, kern in [(72,150,7.024532425085975e-4),(75,30,6.989406182353747e-4),(70,210,None)]:
    v = vol(n1, 0.985, dirv(lat, az), 0.985)
    print(lat, az, repr(v), "kernel", kern, "rel", None if kern is None else abs(kern-v)/v)
