import sys
import png
import time
from stk_execute import *

def stk_interpreter(i_file, debug=False):
    reset_stk_executer()

    inp_lines = []
    with open(i_file, 'r') as f:
        inp_lines = f.readlines()
    lines = []
    for x in inp_lines:
        if "#" in x:
            lines.append(x[:x.find("#")])
        else:
            lines.append(x)

    blocks = dict()
    for l in [x.split("\n") for x in "".join(lines).split("\n\n")]:
        blocks[l[0].split()[1]] = list(filter(lambda x: x != "", l[1:]))
    blocks["term"] = []

    stack = []

    block = blocks["main"]
    index = 0
    while index < len(block):
        cmd = block[index].split()
        # print (" ".join(cmd), stack)
        match cmd[0]:
            case "branch":
                a = stack.pop()
                if (a == 0):
                    if debug:
                        print (cmd[2])
                    block = blocks[cmd[2]]
                else:
                    if a != 1:
                        print ("Brancing on non-bool:", a)
                    if debug:
                        print (cmd[1])
                    block = blocks[cmd[1]]
                index = 0
                continue
            case "goto":
                block = blocks[cmd[1]]
                index = 0
                if debug:
                    print ("goto", cmd)
                continue
            case "term":
                break
            case "debug":
                print (stack)
            case default:
                cmd_interpreter(cmd, stack)
        index += 1

        if debug:
            time.sleep(0.01)
            print (cmd, stack)
