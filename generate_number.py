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

solved = set([0])
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
    add_next_instr(1, "roll")

def optimize_number(N):
    while not N in solved:
        weight, stack, instructions, total_instructions = heapq.heappop(heap)
        stack = evaluate(instructions, stack)
        stk_str = ",".join(map(str, stack))

        if len(stack) > 0 and stack[-1] == 0:
            continue

        if not stk_str in results:
            results[stk_str] = (weight, total_instructions)
            add_neighbors(weight, stack, total_instructions)

            if len(stack) == 1 and not stack[0] in solved:
                solved.add(stack[0])

    return results[str(N)][1]

if __name__ == "__main__":
    N = int(input())
    for i in range(N+1):
        optimize_number(i)
        print (results)
        print (i, "(" + str(results[str(i)][0]) + ")", ":", results[str(i)][1])
