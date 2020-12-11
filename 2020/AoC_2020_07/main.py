import sys
import re


def dfs( graph, key, depth, seen, searchWord ):
    if searchWord == key and depth <= 0:
        return True
    if key in seen:
        return False
    seen.add( key )

    for bag in graph[key]:
        if dfs( graph, bag[1], depth - 1, seen, searchWord ):
            #print( bag[1] )
            return True;
    seen.remove( key )
    return False

def bagDFS( graph, key, seen ):
    if key in seen:
        return seen[key]

    bagTotal = 0
    additionalBags = 0
    for val in graph[key]:
        bagTotal += val[0]
        additionalBags += val[0]*bagDFS( graph, val[1], seen )

    print( "{}: {} + {}".format(key, bagTotal, additionalBags ))
    seen[key] = bagTotal + (additionalBags)
    return seen[key]

def bagContents( s ):
    matches = re.findall("((\d+)([\w\s]+)bags?)+", s)
    l = []
    for m in matches:
        l.append( ( int(m[1]), m[2].strip() )) 
    #print( l )
    return l

def readFile(fileName):
    graph = dict()
    file = open(fileName, 'r')
    lines = file.readlines()
    regex = re.compile("([\w\s]+)bags contain")
    for line in lines:
        key = regex.match( line ).group(1).strip()
        #print( key )
        value = bagContents( line )
        graph[key] = value

    file.close()
    return graph

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    graph = readFile(sys.argv[1])
    #print( graph)
    depth = 1
    seen = set()
    count = 0
    for k,v in graph.items():
        if dfs( graph, k, depth, seen, "shiny gold"):
            count += 1
        seen.clear()
    x = dict()
    v = bagDFS( graph, "shiny gold", x)
    print( v )
    
main()
