import sys
input_eof = False
def cmd_interpreter(cmd, stack):
    global input_eof
    match cmd[0]:
        case "push":
            stack.append(int(cmd[1]))
        case "pop":
            if len(stack) >= 1:
                stack.pop()
        case "add":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(a + b)
        case "sub":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(b - a)
        case "mul":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(a * b)
        case "div":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(b // a) # TODO: Ignore if div by zero
        case "mod":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(b % a)
        case "not":
            if len(stack) >= 1:
                a = stack.pop()
                stack.append(1 if a == 0 else 0)
        case "greater":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(1 if b > a else 0)
        case "dup":
            if len(stack) >= 1:
                a = stack.pop()
                stack.append(a)
                stack.append(a)
        case "roll":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop() # TODO: Ignore b negative
                if b < 0:
                    print ("Error")
                else:
                    a = a % b
                    if a != 0:
                        stack[-b:] = stack[-a:] + stack[-b:-a]
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
        case default:
            print("Invalid cmd:", cmd)
    # return stack
