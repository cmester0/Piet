import sys
import png
import time
from stk_execute import *

def stk_interpreter(i_file, debug=False):
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

    input_eof = False

    block = blocks["main"]
    index = 0
    while index < len(block):
        cmd = block[index].split()
        # print (" ".join(cmd), stack)
        match cmd[0]:
            case "inN":
                if not input_eof:
                    value = sys.stdin.buffer.peek(1)
                    if value == b"":
                        stack.append(-1)
                        input_eof = True
                    else:
                        for x in reversed(range(len(value))):
                            if "".join(map(chr, value[:x+1])).isnumeric():
                                stack.append(int("".join(map(chr, sys.stdin.buffer.read(x+1)))))
                                break
                else:
                    stack.append(-1)
            case "inC":
                if not input_eof:
                    value = sys.stdin.buffer.peek(1)
                    if value == b"":
                        stack.append(-1)
                        input_eof = True
                    else:
                        stack.append(ord(chr(sys.stdin.buffer.read(1)[0])))
                else:
                    stack.append(-1)
            case "outN":
                if len(stack) >= 1:
                    print(stack.pop(),end="")
            case "outC":
                if len(stack) >= 1:
                    print(chr(stack.pop()),end="")
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
                stack = cmd_interpreter(cmd, stack)
        index += 1

        if debug:
            time.sleep(0.01)
            print (cmd, stack)
