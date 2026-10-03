import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon
P=[(0,0),(8,0),(8,2),(7,1),(6,1),(5,2),(4,1),(3,2),(0,2)]
fig,ax=plt.subplots(1,3,figsize=(15,4.2))
def base(a,t):
    a.set_xlim(-0.5,8.5);a.set_ylim(-0.4,2.5);a.set_aspect('equal');a.set_title(t,fontsize=11);a.axis('off')
a=ax[0];base(a,"Input: notched block (profile, extruded in z)\nsplit plane y = 1 (dashed)")
a.add_patch(Polygon(P,closed=True,fc='#cfd8e3',ec='k'))
a.axhline(1,ls='--',c='crimson')
a.plot([4],[1],'o',c='crimson');a.annotate("V-tip (4,1)\nconcave edge",(4,1),(2.2,0.35),arrowprops=dict(arrowstyle='->'),fontsize=9)
a.plot([6,7],[1,1],c='darkgreen',lw=4);a.annotate("notch floor face\n(lies IN y = 1)",(6.5,1),(5.6,0.2),arrowprops=dict(arrowstyle='->'),fontsize=9)
a=ax[1];base(a,"Above half today: 3 solids\nleft and middle TOUCH along the tip line (pinch)")
for poly,c in [([(0,1),(4,1),(3,2),(0,2)],'#f4c7a1'),([(4,1),(6,1),(5,2)],'#a1d4f4'),([(7,1),(8,1),(8,2)],'#c7f4a1')]:
    a.add_patch(Polygon(poly,closed=True,fc=c,ec='k'))
a.plot([4],[1],'o',ms=10,mfc='none',mec='crimson',mew=2)
a.annotate("two copies of the tip vertex\non one point: contact",(4,1),(4.3,0.2),arrowprops=dict(arrowstyle='->'),fontsize=9)
a=ax[2];base(a,"Plane at y = 1 + δ, 0 < δ ≤ ε (zoom, not to scale)\nkernel reads the tip ON: same pinch")
a.set_xlim(2.5,5.5);a.set_ylim(0.6,1.6)
a.add_patch(Polygon([(3,2),(4,1),(5,2)],closed=False,fill=False,ec='k'))
a.plot([3,4,5],[2,1,2],c='k')
a.axhline(1.12,ls='--',c='crimson');a.text(2.6,1.15,"y = 1 + δ",color='crimson',fontsize=9)
a.plot([4],[1],'o',c='crimson')
a.annotate("true cut: tips 2δ apart,\nsliver δ thick below",(4,1.12),(4.4,0.75),arrowprops=dict(arrowstyle='->'),fontsize=9)
plt.tight_layout();plt.savefig("notched-split.png",dpi=110)
