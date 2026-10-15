from pathlib import Path
import math, json, copy, xml.etree.ElementTree as ET

ROOT=Path(__file__).resolve().parent
NS='http://www.w3.org/2000/svg'; ET.register_namespace('',NS)
INK='#34465b'
GREY=['#e2e8ef','#b8c3cf','#8e9faf']
BLUE=['#b9ddf8','#328bd3','#236ead']
EDGE=3.0

def el(tag,attrs=None,text=None):
    n=ET.Element('{'+NS+'}'+tag,attrs or {});n.text=text;return n
def path(d,fill='none',stroke=None,width=EDGE,attrs=None):
    a={'d':d,'fill':fill}
    if stroke:a.update({'stroke':stroke,'stroke-width':str(width),'stroke-linejoin':'round','stroke-linecap':'round'})
    if attrs:a.update(attrs)
    return el('path',a)
def poly(points):return 'M'+' L'.join(f'{x:.4f} {y:.4f}' for x,y in points)+' Z'
def line(points,colour=INK,width=EDGE,attrs=None):return path('M'+' L'.join(f'{x:.4f} {y:.4f}' for x,y in points),stroke=colour,width=width,attrs=attrs)
def circle(x,y,r,fill):return el('circle',{'cx':str(x),'cy':str(y),'r':str(r),'fill':fill})
def P(x,y,z):return (7.75+math.sqrt(3)/2*(x+y),46+.5*x-.5*y-z)

def cube(top,left,right,bottom_left,bottom_front,bottom_right,tones,weight=EDGE):
    back=top
    # Three flat faces and one clean silhouette, with each shared edge drawn once.
    front_top=(left[0]+right[0]-back[0],left[1]+right[1]-back[1])
    out=[path(poly([left,front_top,bottom_front,bottom_left]),tones[1]),
         path(poly([front_top,right,bottom_right,bottom_front]),tones[2]),
         path(poly([left,back,right,front_top]),tones[0])]
    out.append(path(poly([left,back,right,bottom_right,bottom_front,bottom_left]),stroke=INK,width=weight))
    for tip in (left,right,bottom_front):out.append(line([front_top,tip],width=weight))
    return out

def body():return cube((32,6),(8,20),(56,20),(8,46),(32,60),(56,46),GREY)

def extrude():
    # Large generated volume, with only the two visible faces of its grey base band.
    left,front,right=(8,30),(32,44),(56,30)
    band_left,band_front,band_right=(8,39),(32,53),(56,39)
    out=[path(poly([band_left,band_front,(32,61),(8,47)]),GREY[1]),
         path(poly([band_front,band_right,(56,47),(32,61)]),GREY[2]),
         path(poly([left,front,band_front,band_left]),BLUE[1]),
         path(poly([front,right,band_right,band_front]),BLUE[2]),
         path(poly([left,(32,16),right,front]),BLUE[0]),
         path(poly([left,(32,16),right,(56,47),(32,61),(8,47)]),stroke=INK),
         line([left,front,right]),line([band_left,band_front,band_right]),line([front,(32,61)])]
    out.append(path('M32 1 L40 9 H35 V12 H29 V9 H24 Z',BLUE[2]))
    return out

def fillet():
    # A broad quarter-round on the top-right edge; no ghost edge or highlight line.
    W=D=28;H=26;R=16
    curve=[(W-R+R*math.cos(i*math.pi/2/32),H-R+R*math.sin(i*math.pi/2/32)) for i in range(33)]
    top=[P(0,0,H),P(W-R,0,H),P(W-R,D,H),P(0,D,H)]
    side=[P(W,0,0),P(W,D,0),P(W,D,H-R),P(W,0,H-R)]
    ribbon=[P(x,0,z) for x,z in curve]+[P(x,D,z) for x,z in reversed(curve)]
    cap=[P(0,0,0),P(W,0,0)]+[P(x,0,z) for x,z in curve]+[P(0,0,H)]
    out=[path(poly(top),GREY[0]),path(poly(side),GREY[2]),path(poly(ribbon),BLUE[1]),path(poly(cap),GREY[1])]
    outer=[P(0,0,0),P(W,0,0),P(W,D,0),P(W,D,H-R)]+[P(x,D,z) for x,z in curve[1:]]+[P(0,D,H),P(0,0,H)]
    out.append(path(poly(outer),stroke=INK))
    out.append(line([P(x,0,z) for x,z in curve]))
    out.append(line([P(W,0,0),P(W,0,H-R)]))
    out.append(line([P(W,0,H-R),P(W,D,H-R)]))
    out.append(line([P(W-R,0,H),P(W-R,D,H)]))
    out.append(line([P(0,0,H),P(W-R,0,H)]))
    return out

def revolve(small=False):
    # A larger opening makes the inner wall and axis survive at 18–26 points.
    inner,outer,height=8,24,16
    def Q(r,z,t):
        a=math.radians(t);return (32+r*math.cos(a),52-z-.35*r*math.sin(a)-7)
    def angles(a,b):return [a+(b-a)*i/48 for i in range(49)]
    ts=angles(0,180)
    wall=[Q(inner,0,t) for t in ts]+[Q(inner,height,t) for t in reversed(ts)]
    out=[path(poly(wall),BLUE[2])]
    ts=angles(-45,225)
    top=[Q(outer,height,t) for t in ts]+[Q(inner,height,t) for t in reversed(ts)]
    out.append(path(poly(top),BLUE[0],INK))
    for lo,hi,tone in [(180,225,BLUE[1]),(-45,0,BLUE[2])]:
        ts=angles(lo,hi)
        out.append(path(poly([Q(outer,0,t) for t in ts]+[Q(outer,height,t) for t in reversed(ts)]),tone,INK))
    profile=[(inner,0),(outer,0),(outer,height),(inner,height)]
    for t,tone in [(-45,BLUE[1]),(225,GREY[1])]:out.append(path(poly([Q(r,z,t) for r,z in profile]),tone,INK))
    axis={'stroke-dasharray':'4 5'}
    out.append(path('M32 13 V55',stroke='white',width=3.4,attrs=axis))
    out.append(path('M32 13 V55',stroke=INK,width=2.4,attrs=axis))
    def A(t):
        a=math.radians(t);x,y=Q(outer,height,t)
        nx,ny=math.cos(a)/outer,-math.sin(a)/(.35*outer);n=math.hypot(nx,ny)
        return(x+6.3*nx/n,y+6.3*ny/n)
    base=A(40);tip=A(18)
    dx,dy=tip[0]-base[0],tip[1]-base[1];length=math.hypot(dx,dy);dx/=length;dy/=length
    out.append(line([A(t) for t in angles(155,40)],BLUE[2],3.8))
    out.append(path(poly([tip,(base[0]-2.7*dy,base[1]+2.7*dx),(base[0]+2.7*dy,base[1]-2.7*dx)]),BLUE[2]))
    return out

def section(small=False):
    W=D=28;H=26
    front=[P(0,0,0),P(W,0,0),P(W,0,H),P(0,0,H)]
    hole=[P(14+7.2*math.cos(i*math.tau/64),0,13+7.2*math.sin(i*math.tau/64)) for i in range(65)]
    cut=poly(front)+' '+poly(hole)
    out=[path(poly([P(W,0,0),P(W,D,0),P(W,D,H),P(W,0,H)]),GREY[2],INK),
         path(poly([P(0,0,H),P(W,0,H),P(W,D,H),P(0,D,H)]),GREY[0],INK),
         path(cut,BLUE[0],INK,attrs={'fill-rule':'evenodd'})]
    defs=el('defs');clip=el('clipPath',{'id':'section-cut'})
    clip.append(path(cut,attrs={'clip-rule':'evenodd'}));defs.append(clip);out.append(defs)
    hatch=el('g',{'clip-path':'url(#section-cut)'})
    # Fine, repeated hatching distinguishes a section from a generated face.
    for c in ([6,18,30,42,54] if small else [6,15,24,33,42,51]):
        hatch.append(line([P(c,0,0),P(c-H,0,H)],BLUE[1],1.8))
    out.append(hatch)
    return out

def component():
    out=cube((40,8),(27,15.5),(53,15.5),(27,31),(40,38.5),(53,31),GREY,3)
    out+=cube((24,26),(11,33.5),(37,33.5),(11,49),(24,56.5),(37,49),GREY,3)
    return out

def folder():return [path('M8 24 V18 Q8 14 12 14 H25 L31 20 H52 Q56 20 56 24 V49 Q56 53 52 53 H12 Q8 53 8 49 Z',GREY[1],INK)]

def sketch(small=False):
    frame=el('rect',{'x':'8','y':'8','width':'48','height':'48','rx':'2','fill':'none','stroke':'#6f879c','stroke-width':'2.8','stroke-dasharray':'0.1 7.5','stroke-linecap':'round'})
    vertices=[(18,27),(35,17),(46,44),(19,43)]
    out=[frame,path(poly(vertices),'#d5eafb',BLUE[2],3.4)]
    for x,y in vertices:out.append(circle(x,y,3.3,BLUE[2]))
    return out

def menu(key):
    # Preserve the approved bold, single-colour application family.
    if key=='undo':return [path('M50 45 C50 32 42 22 29 22 H14 M22 14 L14 22 L22 30',stroke=INK,width=5)]
    if key=='zoom':return [path('M38.5 38.5 L52 52',stroke=INK,width=5),el('circle',{'cx':'27','cy':'27','r':'15','fill':'none','stroke':INK,'stroke-width':'5'})]
    return [copy.deepcopy(e) for e in ET.parse(ROOT/'application-source'/(key+'.svg')).getroot()]

def draw(key,small=False):
    if key in ('undo','save','settings','zoom'):return menu(key)
    return {'extrude':extrude,'revolve':lambda:revolve(small),'fillet':fillet,'section':lambda:section(small),'body':body,'component':component,'folder':folder,'sketch':lambda:sketch(small)}[key]()

layout=json.loads((ROOT/'layout.json').read_text())
(ROOT/'icons').mkdir(exist_ok=True);(ROOT/'icons-small').mkdir(exist_ok=True)
board=el('svg',{'width':'544','height':'320','viewBox':'0 0 544 320','role':'img'})
board.append(el('title',text='Technical Accent — refined'));board.append(el('rect',{'width':'544','height':'320','fill':'white'}))
for sec in layout['sections']:
    board.append(el('text',{'x':'16','y':str(sec['heading_y']),'font-family':'Arial,sans-serif','font-size':'13','font-weight':'600','fill':'#30343b'},sec['heading']))
    for item in sec['icons']:
        for small in (False,True):
            icon=el('svg',{'width':'48','height':'48','viewBox':'0 0 64 64','role':'img'})
            icon.append(el('title',text='Technical Accent refined — '+item['name']))
            for e in draw(item['id'],small):icon.append(e)
            ET.ElementTree(icon).write(ROOT/('icons-small' if small else 'icons')/(item['id']+'.svg'),encoding='unicode')
            if not small:
                icon.attrib.update(x=str(item['x']),y=str(item['y']),id=item['id']);board.append(icon)
        board.append(el('text',{'x':str(item['x']+24),'y':str(item['label_y']),'text-anchor':'middle','font-family':'Arial,sans-serif','font-size':'12','fill':'#30343b'},item['name']))
ET.indent(board,space='  ');ET.ElementTree(board).write(ROOT/'preview.svg',encoding='unicode')
(ROOT/'palette.json').write_text(json.dumps({'ink':INK,'grey':GREY,'blue':BLUE,'geometry_stroke':EDGE,'action_stroke':5},indent=2)+'\n')
(ROOT/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print('Created a simplified, bolder Technical Accent variant with small-slot SVGs.')
