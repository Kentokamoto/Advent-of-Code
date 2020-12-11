import sys
import math

class Group:
    def __init__( self ):
        self.persons = []
        self.answeredYes = set()
        self.histogram = dict()

    def addPerson( self, yesVals ):
        self.persons.append( yesVals )
        for c in yesVals:
            if c != '\n':
                self.answeredYes.add(c)
                if c in self.histogram:
                    self.histogram[c] += 1
                else:
                    self.histogram[c] = 1

    def getCount(self):
        return len(self.answeredYes)
    
    def getEveryoneCount(self):
        total = 0
        for k,v in self.histogram.items():
            if v == len( self.persons ):
                total += 1 
        return total

def getTotal( groups ):
    total = 0
    for group in groups:
        total += group.getCount()
    
    return total

def getEveryoneTotal(groups):
    total = 0
    for group in groups:
        total += group.getEveryoneCount()
    return total

def readFile(fileName):
    groups = []
    group = Group()
    file = open(fileName, 'r')
    lines = file.readlines()
    for line in lines:
        #print( "line: {}".format(line), end = "" )
        if line != '\n':
            group.addPerson( line )
        else:
            #print("newline")
            groups.append(group) 
            group = Group()
    groups.append(group)
    file.close()
    return groups

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    groups = readFile(sys.argv[1])
    print("Total Count: {}".format(getTotal(groups)))
    print("Total Everyone Count: {}".format(getEveryoneTotal(groups)))

main()
