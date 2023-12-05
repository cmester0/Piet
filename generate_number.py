import heapq
from stk_execute import *
import sys
import math

class StackOptimizer():
    def __init__(self):
        self.results = {"0": (2, ["push 1", "not"])}
        # Weight, Curr_Stack, Curr_instrs, Total_instrs
        self.heap = []
        heapq.heappush(self.heap, (1, [], ["push 1"], ["push 1"]))
        heapq.heappush(self.heap, (2, [], ["push 2"], ["push 2"]))
        heapq.heappush(self.heap, (3, [], ["push 3"], ["push 3"]))
        heapq.heappush(self.heap, (5, [], ["push 5"], ["push 5"]))

    def evaluate(self, instrs, stk):
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
                    cmd_interpreter([cmd], stack)
        return stack

    def add_neighbors(self, weight, stack, total_instructions):
        def add_next_instr(weight_change, next_instr):
            heapq.heappush(self.heap, (weight + weight_change, stack, [next_instr], total_instructions + [next_instr]))

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
        if len(stack) >= 2 and len(stack)-2 >= stack[-2] > 0: # only allow valid roll? (Otherwise is better pop, pop?)
            add_next_instr(1, "roll")

    def optimize_stack(self, search_stack):
        while not search_stack in self.results:
            weight, stack, instructions, total_instructions = heapq.heappop(self.heap)
            stack = self.evaluate(instructions, stack)
            stk_str = ",".join(map(str, stack))

            # Ignore 0 on top of the stack ?
            if len(stack) > 0 and stack[-1] == 0:
                continue

            if not stk_str in self.results:
                # print (stk_str)
                self.results[stk_str] = (weight, total_instructions)
                self.add_neighbors(weight, stack, total_instructions)

        return self.results[search_stack][1]

    def int_root(self, x, n):
        if x <= 0 or n <= 1:
            return x

        L = 0
        R = x
        while L <= R:
            m = (L + R) // 2
            if pow(m,n) < x:
                L = m + 1
            elif pow(m,n) > x:
                R = m - 1
            else:
                return m
        return R

    # TODO: Handle negative numbers!
    def optimize_number(self, N):
        instrs = []
        root = 1
        while self.int_root(N,root) > 173:
            root += 1
        instrs += self.optimize_stack(str(self.int_root(N,root)))
        for i in range(root-1):
            instrs.append("dup")
        for i in range(root-1):
            instrs.append("mul")
        if N - pow(self.int_root(N,root), root) > 0:
            instrs+=self.optimize_number(N - pow(self.int_root(N,root),root))
            instrs.append("add")

        return instrs

if __name__ == "__main__":
    stack_optimizer = StackOptimizer()
    if len(sys.argv) >= 2 and sys.argv[1] == "num":
        for i in range(int(input())):
            print (i,stack_optimizer.optimize_number(i))
    else:
        inp = list(map(str, map(lambda x: int(x) if x.lstrip("-").isnumeric() else ord(x), input().split(","))))
        print (inp)
        for i in range(len(inp)):
            sub_inp = ",".join(inp[:i+1])
            stack_optimizer.optimize_stack(sub_inp)
            print (sub_inp,"(" + str(stack_optimizer.results[sub_inp][0]) + ")", ":", stack_optimizer.results[sub_inp][1])
