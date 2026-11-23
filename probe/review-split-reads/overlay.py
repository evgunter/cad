import struct,sys
def f(h): return struct.unpack('>d',bytes.fromhex(h))[0]
def load(p):
    out=[]
    for line in open(p).read().splitlines()[1:]:
        t=line.split()
        if not t or t[0].startswith('Receipt'): continue
        out.append((t[0],[(f(t[1+2*i]),f(t[2+2*i])) for i in range(2)]))
    return out
def meet(a,b): return all(max(x[0],y[0])<min(x[1],y[1]) for x,y in zip(a,b))
def area(b): return (b[0][1]-b[0][0])*(b[1][1]-b[1][0])
for n in sys.argv[2:]:
    base=load(f'{sys.argv[1]}/base/plate-{n}.txt'); head=load(f'{sys.argv[1]}/head/plate-{n}.txt')
    bad=[(c,b,hc) for c,b in base if c=='certified' for hc,hb in head if meet(b,hb) and hc!='certified']
    tot=area(base[0][1]) if False else None
    print(n,'base-certified regions overlapping a non-certified head leaf:',len(bad))
    for c in ('certified',): 
        print(' certified area base',sum(area(b) for c2,b in base if c2==c),'head',sum(area(b) for c2,b in head if c2==c))
    # union coverage
    print(' total area base',sum(area(b) for _,b in base),'head',sum(area(b) for _,b in head))
