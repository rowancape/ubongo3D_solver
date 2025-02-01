from itertools import product
import copy
import time

# fmt: off
rightCornerLeftCenter = [[[0, 0, 0, 1], 
                          [1, 0, 0, 0], 
                          [0, 0, 0, 0]],
          
                         [[0, 0, 0, 1], 
                          [1, 0, 0, 0], 
                          [0, 0, 0, 0]]]

threeCorners = [[[1, 0, 0, 1], 
                 [0, 0, 0, 0], 
                 [0, 0, 0, 1]],
          
                [[1, 0, 0, 1], 
                 [0, 0, 0, 0], 
                 [0, 0, 0, 1]]]

yellowU = [[[1, 1, 1], 
            [1, 0, 1]]]

redSimple = [[[1, 1], 
              [1, 1]], 

             [[0, 1], 
              [0, 0]]]

greenL = [[[1, 1, 1],
           [0, 0, 1]]]

blue3 = [[[1, 1],
          [0, 1]]]

red4 = [[[1, 1],
         [0, 1]],
                   
        [[1, 0],
         [0, 0]]]

yellow4 = [[[1, 0],
            [1, 1]],
                   
           [[1, 0],
            [0, 0]]]

yellowTL = [[[0, 1], 
             [1, 1],
             [0, 1]], 

            [[0, 1], 
             [0, 0],
             [0, 0]]]

yellowDoubleL = [[[1, 1], 
                  [0, 1],
                  [0, 1]], 

                 [[0, 0], 
                  [0, 0],
                  [0, 1]]]

blueDoubleL = [[[0, 1], 
                [0, 1],
                [1, 1]], 

               [[0, 0], 
                [0, 0],
                [1, 0]]]

greenDoubleL = [[[1, 1, 1], 
                 [1, 0, 0]], 

                [[0, 0, 0], 
                 [1, 0, 0]]]

redDoubleL = [[[1, 1, 1], 
               [0, 0, 1]], 

              [[1, 0, 0], 
               [0, 0, 0]]]

blueSquiggly = [[[1, 0], 
                 [1, 1],
                 [0, 1]], 

                [[1, 0], 
                 [0, 0],
                 [0, 0]]]

greenSquiggly = [[[0, 1], 
                  [1, 1],
                  [1, 0]], 

                 [[0, 1], 
                  [0, 0],
                  [0, 0]]]
# fmt: on

field = rightCornerLeftCenter
blocks = [greenDoubleL, yellowDoubleL, redSimple, yellowTL]


def printObj(object):
    for layer in object:
        print("————————")
        for row in layer:
            print(row)
                

def rotateX(object, rotations=1):
    rotations = rotations % 4

    for _ in range(rotations):
        layers = len(object)
        rows = len(object[0])
        points = len(object[0][0])

        rotatedObject = [
            [[object[j][rows - 1 - i][k] for k in range(points)] for j in range(layers)]
            for i in range(rows)
        ]
        object = rotatedObject

    return object


def rotateY(object, rotations=1):
    rotations = rotations % 4

    for _ in range(rotations):
        layers = len(object)
        rows = len(object[0])
        points = len(object[0][0])

        rotatedObject = [
            [[object[layers - 1 - k][j][i] for k in range(layers)] for j in range(rows)]
            for i in range(points)
        ]
        object = rotatedObject

    return object


def rotateZ(object, rotations=1):
    rotations = rotations % 4

    for _ in range(rotations):
        layers = len(object)
        rows = len(object[0])
        points = len(object[0][0])

        rotatedObject = [
            [[object[i][rows - 1 - k][j] for k in range(rows)] for j in range(points)]
            for i in range(layers)
        ]
        object = rotatedObject

    return object


def generateRotations(object):
    rotations = []
    rotationStates = []

    for i in range(4):
        for j in range(4):
            for k in range(4):
                isDuplicate = False
                rotatedObject = rotateX(rotateY(rotateZ(object, k), j), i)

                for rotIndex, rotation in enumerate(rotations):
                    if rotation == rotatedObject:
                        isDuplicate = True
                        print("FOUND DUPE!")
                        print(f"existing rotation -> {rotationStates[rotIndex]}:")
                        for layer in rotation:
                            print("————————>")
                            for row in layer:
                                print(row)
                        print(f"new rotation -> [{i}, {j}, {k}]:")
                        for layer in rotatedObject:
                            print("————————>")
                            for row in layer:
                                print(row)
                        break

                if len(rotatedObject) >= 3 or isDuplicate:
                    continue

                else:
                    rotations.append(rotatedObject)
                    rotationStates.append([i, j, k])
    return rotations


def generatePossibleStartCoords(object):
    layers = len(object)
    rows = len(object[0])
    points = len(object[0][0])

    possibleStartCoords = []

    for li, layer in enumerate(field):
        if layers > len(field) - li:
            break

        for ri, row in enumerate(layer):
            if rows > len(layer) - ri:
                break

            for pi, point in enumerate(row):
                if points > len(row) - pi:
                    break

                possibleStartCoords.append([pi, ri, li])

    return possibleStartCoords


def verifyPossibleStartCoords(object, startCoords):
    def isValidPlacement(x, y, z, activeField):
        for zi, layer in enumerate(object):
            for yi, row in enumerate(layer):
                for xi, point in enumerate(row):
                    activeField[z + zi][y + yi][x + xi] += point
                    if activeField[z + zi][y + yi][x + xi] > 1:
                        return False
        return True

    validStartCoords = []

    for coord in startCoords:
        x, y, z = coord[0], coord[1], coord[2]
        activeField = copy.deepcopy(field)
        if isValidPlacement(x, y, z, activeField):
            validStartCoords.append(coord)

    return validStartCoords


def placeObject(activeField, object, x, y, z):
    for zi, layer in enumerate(object):
        for yi, row in enumerate(layer):
            for xi, point in enumerate(row):
                activeField[z + zi][y + yi][x + xi] += point
                if activeField[z + zi][y + yi][x + xi] > 1:
                    return False
    return True


rotatedBlocks = [generateRotations(block) for block in blocks]

allCombinations = product(*rotatedBlocks)

success = False
itercount = 0
lastFourFieldStatesBase = [copy.deepcopy(field)]

for combination in allCombinations:
    allCoords = [[], [], [], []]

    for i, object in enumerate(combination):
        startCoords = generatePossibleStartCoords(object)
        verifiedCoords = verifyPossibleStartCoords(object, startCoords)
        allCoords[i].extend(verifiedCoords)

    for coordCombination in product(*allCoords):
        coordCombList = list(coordCombination)

        activeField = copy.deepcopy(field)
        lastFourFieldStates = copy.deepcopy(lastFourFieldStatesBase)

        for i in range(4):
            x, y, z = coordCombList[i][0], coordCombList[i][1], coordCombList[i][2]
            if placeObject(activeField, combination[i], x, y, z):
                lastFourFieldStates.append(copy.deepcopy(activeField))
                if i == 3:
                    for fsi, fieldState in enumerate(lastFourFieldStates):
                        if fsi > 0:
                            print("\n")
                            print("CORRESPONDING OBJECT:")
                            printObj(combination[fsi-1])
                        print(f"FIELD STATE {fsi}:")
                        printObj(fieldState)
                    success = True
                    break
                continue
            else:
                break

        if success:
            break

    if success:
        break

    if itercount % 1000 == 0:
        print(f"Still computing on combination: {itercount}")
    itercount += 1

print(f"Final itercount: {itercount}")

if success:
    print("————————————")
    print("————————————")
    print("————————————")
    print("SOLUTION FOUND")
    print("FIELD:")
    printObj(activeField)
    print("————————————")
    print("START COORDS FOR SOLUTION:")
    print(coordCombList)
    print("————————————")
    print("OBJECTS:")
    for obj in range(4):
        print(f"\nObject {obj} —— Startpoint: [{coordCombList[obj][0]}, {coordCombList[obj][1]}, {coordCombList[obj][2]}]:")
        printObj(combination[obj])
else:
    print("Found no solution!")
    print("Found no solution!")
    print("Found no solution!")
    print("Found no solution!")
    print("Found no solution!")