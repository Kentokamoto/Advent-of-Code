import sys
import re

def isInfiniteLoop(instructions, accumulator ):
    currIndex = 0
    seen = set()
    isInfinite = False
    while currIndex < len(instructions):
        if currIndex in seen:
            isInfinite = True
            break
        else:
            seen.add(currIndex)
        currInst = instructions[currIndex]
        if currInst[0] == "acc":
            accumulator += currInst[1]
            currIndex += 1
        elif currInst[0] == "jmp":
            currIndex += currInst[1]
        elif currInst[0] == "nop":
            currIndex += 1
    if not isInfinite:
        print( "acc: ", accumulator )
    return isInfinite

def readFile(fileName):
    instructions = []
    file = open(fileName, 'r')
    lines = file.readlines()
    regex = re.compile("(nop|acc|jmp) (\+|-)(\d+)")
    for line in lines:
        match = regex.match( line )
        val = int( match.group(3) ) if match.group(2) == '+' else\
              -1* int( match.group(3) )
        instructions.append( [match.group(1).strip(), val] )

    file.close()
    return instructions

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    instructions = readFile(sys.argv[1])
    for i in range( len( instructions ) ):
        acc = 0
        temp = instructions[i][0]
        if instructions[i][0] == "nop":
            instructions[i][0] = "jmp"
        elif instructions[i][0] == "jmp":
            instructions[i][0] = "nop"
        else:
            continue       

        if not isInfiniteLoop( instructions, acc ):
            break
        else:
            instructions[i][0] = temp
main()
