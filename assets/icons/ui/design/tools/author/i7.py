from lib import *
from i2 import pt, arrowhead, arrow
from i4 import G,B,face,surf_patch
ICONS={}
SW=3.2
def icon(fn): ICONS[fn.__name__]=fn; return fn
def small(name,title=None):
    ic=Icon(name,title); ic.fam='small'; return ic
@icon
def pattern_small():
    ic=small('pattern','Pattern'); I=Iso().fit([(0,0,0),(34,0,0),(0,34,0),(34,34,0),(0,0,15),(34,34,15)],58,56)
    for (x,y,c) in ((0,0,G),(19,0,B),(0,19,B)):
        pass
    for (x,y,c) in sorted([(0,0,G),(19,0,B),(0,19,B)],key=lambda q:q[0]+q[1]):
        I.box(ic,x,y,0,x+15,y+15,15,c,SW,detail=True)
    return ic
@icon
def interference_small():
    ic=small('interference','Interference'); I=Iso().fit([(0,-12,0),(36,28,0),(0,-12,20),(36,28,20)],58,56)
    I.box(ic,0,0,0,20,20,20,G,SW,detail=False); I.box(ic,14,-12,0,36,8,20,G,SW,detail=False)
    sq=I.poly((14,0,20),(20,0,20),(20,8,20),(14,8,20)); ic.fill(sq,A2); ic.stroke(sq,INK,SW)
    return ic
def interference_small():
    ic=small('interference','Interference'); I=Iso().fit([(0,-12,0),(36,28,0),(0,-12,20),(36,28,20)],58,56)
    I.box(ic,0,0,0,20,20,20,G,SW,detail=False); I.box(ic,14,-12,0,36,8,20,G,SW,detail=False)
    I.box(ic,14,0,0,20,8,20,B,SW,detail=False)
    return ic
def interference_small():
    ic=small('interference','Interference')
    I2=Iso(1,43,26); I2.box(ic,-10,-10,-10,10,10,10,G,SW,detail=True)
    I1=Iso(1,21,40); I1.box(ic,-10,-10,-10,10,10,10,G,SW,detail=True)
    I3=Iso(1,32,33); I3.box(ic,-5,-5,-5,5,5,5,B,SW,detail=False)
    return ic
@icon
def joint_small():
    ic=small('joint','Joint')
    I1=Iso(1,16,46); I1.box(ic,-9,-9,-9,9,9,9,G,SW,detail=False)
    I2=Iso(1,48,20); I2.box(ic,-9,-9,-9,9,9,9,G,SW,detail=False)
    ic.stroke('M22 40L42 26',A3,9) if False else None
    ic.stroke('M22 40L42 26',INK,11.2); ic.stroke('M22 40L42 26',A2,5.6)
    return ic
@icon
def as_built_small():
    ic=small('as_built','As-built joint')
    I1=Iso(1,16,34); I1.box(ic,-9,-9,-9,9,9,9,G,SW,detail=False)
    I2=Iso(1,48,34); I2.box(ic,-9,-9,-9,9,9,9,G,SW,detail=False)
    ic.stroke('M22 34L42 34',INK,11.2); ic.stroke('M22 34L42 34',A2,5.6)
    return ic
@icon
def iso_analysis_small():
    ic=small('iso_analysis','Isoparametric analysis'); I=Iso().fit([(0,0,0),(40,0,0),(40,40,0),(0,40,0),(0,0,12),(40,40,12)],58,52)
    surf_patch(ic,I,n=1,fillc=P1,line=A3,lw=SW,out=INK,ow=SW); return ic
@icon
def component_small():
    ic=small('component','Component'); I=Iso().fit([(0,-16,0),(16,16,0),(0,-16,32),(16,16,32),(0,16,0),(16,-16,0)],58,58); c=16
    I.box(ic,0,-c,0,c,0,c,B,SW,detail=False); I.box(ic,0,-c,c,c,0,2*c,G,SW,detail=False)
    I.box(ic,0,0,0,c,c,c,G,SW,detail=True)
    return ic
def component_small():
    ic=small('component','Component'); I=Iso().fit([(0,0,0),(32,32,0),(0,0,32),(32,32,32),(0,32,0),(32,0,0)],58,58); c=16
    I.box(ic,0,0,0,c,c,c,B,SW,detail=False); I.box(ic,0,0,c,c,c,2*c,G,SW,detail=True)
    I.box(ic,c,0,c,2*c,c,2*c,G,SW,detail=True); I.box(ic,0,c,c,c,2*c,2*c,G,SW,detail=True)
    return ic
def component_small():
    ic=small('component','Component')
    I2=Iso(1,43,26); I2.box(ic,-10,-10,-10,10,10,10,G,SW,detail=True)
    I1=Iso(1,21,40); I1.box(ic,-10,-10,-10,10,10,10,G,SW,detail=True)
    return ic
@icon
def curvature_comb_small():
    ic=small('curvature_comb','Curvature comb')
    ic.stroke('M7 52Q26 14 54 42','#34465b',5.2)
    for d in ('M13 42L6 33','M24 31L19 15','M34 28L35 10','M45 33L55 20'): ic.stroke(d,A3,3.4)
    return ic
