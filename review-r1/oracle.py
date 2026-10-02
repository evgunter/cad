import re,math,sys,ast,collections
def arcpts(p,q,b,n=20000,sign=1):
    if b==0: return [p]
    th=4*math.atan(b)*sign
    (x0,y0),(x1,y1)=p,q
    c=math.hypot(x1-x0,y1-y0); r=c/(2*math.sin(abs(th)/2))
    mx,my=(x0+x1)/2,(y0+y1)/2
    # center: offset perpendicular
    d=math.sqrt(max(r*r-c*c/4,0)); ux,uy=(x1-x0)/c,(y1-y0)/c; nx,ny=-uy,ux
    # for |th|<pi center on side; choose so that sweep from p to q by th (ccw if th>0)
    s=1 if (abs(th)<math.pi)==(th>0) else -1
    cx,cy=mx+s*d*nx,my+s*d*ny
    a0=math.atan2(y0-cy,x0-cx)
    return [(cx+r*math.cos(a0+th*i/n),cy+r*math.sin(a0+th*i/n)) for i in range(n)]
def poly(ch,sign):
    pts=[]
    for i,(x,y,b) in enumerate(ch):
        x1,y1,_=ch[(i+1)%len(ch)]
        pts+=arcpts((x,y),(x1,y1),b,sign=sign)
    return pts
def area(p): return 0.5*sum(p[i][0]*p[(i+1)%len(p)][1]-p[(i+1)%len(p)][0]*p[i][1] for i in range(len(p)))
def clip(p,ax,lo,hi):
    for (k,bound,keep) in [(ax,lo,lambda v:v>=lo),(ax,hi,lambda v:v<=hi)]:
        out=[]
        for i in range(len(p)):
            a,b=p[i],p[(i+1)%len(p)]
            ia,ib=keep(a[k]),keep(b[k])
            if ia: out.append(a)
            if ia!=ib:
                t=(bound-a[k])/(b[k]-a[k]); out.append((a[0]+t*(b[0]-a[0]),a[1]+t*(b[1]-a[1])))
        p=out
    return p
def inside_area(ch,sign):
    p=poly(ch,sign)
    if area(p)<0: p=p[::-1]
    return area(p), area(clip(clip(p,0,-1,1),1,-1,1))
Z={"top":(0.5,1.0),"bottom":(-0.5,1.0),"through":(-0.5,2.0),"void":(0.25,0.5),"flushtop":(0.5,0.5),"flushbot":(0.0,0.5),"flushboth":(0.0,1.0)}
