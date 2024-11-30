import sys
from smpl_to_stk import *
from optimize_stk import *
from stk_interpreter import *
from stk_to_ascii_piet import *
from ascii_piet_to_piet import *
from piet_interpreter import *

if len(sys.argv) <= 2:
    print ("missing argument, add one or more of",["gen_all","gen_stk","gen_piet","run_stk","run_piet","debug","gif"],"seperated by a ','")
    exit()

smpl_name = sys.argv[1]
name = smpl_name[:smpl_name.rfind(".")]
stk_name = name + ".stk"
stk_optim_name = name + "_optim" + ".stk"
ascii_name = name + ".ascii"
piet_name = name + ".png"
gif_name = name + ".gif"

if "gen_all" in sys.argv[2] or "gen_stk" in sys.argv[2]:
    smpl_to_stk(smpl_name, stk_name)
    optimize_stk(stk_name, stk_optim_name)
if "gen_all" in sys.argv[2] or "gen_piet" in sys.argv[2]:
    stk_to_ascii_piet(stk_optim_name, ascii_name, optim=True)
    ascii_piet_to_piet(ascii_name, piet_name)

if "run_stk" in sys.argv[2]:
    stk_interpreter(stk_optim_name, "debug" in sys.argv[2])
if "run_piet" in sys.argv[2]:
    piet_interpreter(piet_name,
                     o_file = (gif_name if "gif" in sys.argv[2] else ""),
                     debug="debug" in sys.argv[2],
                     max_count=(-1 if len(sys.argv)<=3 else int(sys.argv[3])),
                     gif_speed=(1 if len(sys.argv)<=4 else int(sys.argv[4])))
