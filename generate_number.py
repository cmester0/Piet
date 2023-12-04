from stk_execute import *

def evaluate(instrs, stk):
    stack = list(stk)
    for cmd in instrs:
        match cmd:
            case "push 1":
                stack.append(1)
            case "push 2":
                stack.append(2)
            case "push 3":
                stack.append(3)
            case "push 5":
                stack.append(5)
            case default:
                stack = cmd_interpreter([cmd], stack)
    return stack

import heapq

results = {"0": (2, ["push 1", "not"])}
# Weight, Curr_Stack, Curr_instrs, Total_instrs
heap = []
heapq.heappush(heap, (1, [], ["push 1"], ["push 1"]))
heapq.heappush(heap, (2, [], ["push 2"], ["push 2"]))
heapq.heappush(heap, (3, [], ["push 3"], ["push 3"]))
heapq.heappush(heap, (5, [], ["push 5"], ["push 5"]))

def add_neighbors(weight, stack, total_instructions):
    def add_next_instr(weight_change, next_instr):
        heapq.heappush(heap, (weight + weight_change, stack, [next_instr], total_instructions + [next_instr]))

    add_next_instr(1, "push 1")
    add_next_instr(2, "push 2")
    add_next_instr(3, "push 3")
    add_next_instr(5, "push 5")
    add_next_instr(1, "pop")
    add_next_instr(1, "add")
    add_next_instr(1, "sub")
    add_next_instr(1, "mul")
    add_next_instr(1, "div")
    add_next_instr(1, "mod")
    add_next_instr(1, "dup")
    if len(stack) >= 2 and stack[-2] > 0: # only allow valid roll? (Otherwise is better pop, pop?)
        add_next_instr(1, "roll")

def optimize_stack(search_stack):
    while not search_stack in results:
        weight, stack, instructions, total_instructions = heapq.heappop(heap)
        stack = evaluate(instructions, stack)
        stk_str = ",".join(map(str, stack))

        if len(stack) > 0 and stack[-1] == 0:
            continue

        if not stk_str in results:
            results[stk_str] = (weight, total_instructions)
            add_neighbors(weight, stack, total_instructions)

    return results[search_stack][1]

def optimize_number(N):
    optimize_stack(str(N))

if __name__ == "__main__":
    inp = list(map(str, map(lambda x: int(x) if x.isnumeric() else ord(x), input().split(","))))
    for i in range(len(inp)):
        sub_inp = ",".join(inp[:i+1])
        optimize_stack(sub_inp)
        print (sub_inp,"(" + str(results[sub_inp][0]) + ")", ":", results[sub_inp][1])
