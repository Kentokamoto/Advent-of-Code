import sys
import re

PREAMBLE_SIZE=25

class Xmas:
    def __init__(self):
        self.preamble = set()
        self.queue = []

    def addVal(self, val ):
        if len(self.queue) >= PREAMBLE_SIZE:
            if not self.twoSum( val ):
                return False
            remVal = self.queue.pop(0)
            self.preamble.remove(remVal)

        self.preamble.add(val)
        self.queue.append(val)
        return True

    def twoSum(self, val):
        for p in self.preamble:
            diff = val - p
            if diff == val:
                continue

            if diff in self.preamble:
                return True

        return False

def findWeakness( vals, mysteryNumber ):
    low = 0
    high = 1
    currSum = vals[low] + vals[high]
    arr = [vals[low], vals[high]]
    while currSum != mysteryNumber and high < len( vals ) and low < high:
        print( "{} | {}".format( low, high ) )
        if low + 1 <= high and currSum < mysteryNumber:
            high += 1
            currSum += vals[high]
            arr.append(vals[high])
        else:
            arr.pop(0)
            currSum -= vals[low]
            low += 1
    if low >= high:
        print( "cannot find array") 
        return -1
    localMin = -1
    localMax = -1
    print( "{} | {}".format( low, high ) )
    for a in arr:
        localMin = a if localMin == -1 else min( a, localMin)
        localMax = a if localMax == -1 else max( a, localMax)
    print( "Min: {}, Max: {}".format( localMin, localMax ) )
    return localMin + localMax

def readFile(fileName):
    xmas = Xmas()
    file = open(fileName, 'r')
    lines = file.readlines()
    row = 0
    mysteryNumber = None
    vals = []
    for line in lines:
        vals.append(int(line))
        if not xmas.addVal(int(line)):
            mysteryNumber = int(line) if not mysteryNumber else mysteryNumber
        
    file.close()
    print( mysteryNumber )
    return (vals, mysteryNumber)

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    vals, mysteryNumber = readFile(sys.argv[1])
    weakness = findWeakness(vals, mysteryNumber)
    print( "Weakness: {}".format(weakness))

main()
