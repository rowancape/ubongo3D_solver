import time

block = [[[1, 1, 1], 
          [0, 1, 0]], 

         [[1, 0, 0], 
          [0, 0, 0]]]


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

def printObj(object):
    for layer in object:
        print("————————")
        for row in layer:
            print(row)


def generateRotations(object):
    rotations = []
    rotationStates = []

    for i in range(4):
        for j in range(4):
            for k in range(4):
                isDuplicate = False
                rotatedObject = rotateX(rotateY(rotateZ(object, k), j), i)
                rotations.append(rotatedObject)
                rotationStates.append([i, j, k])
    return rotations

rotations = generateRotations(block)

iter = 0
for rotation in rotations:
    print("NEW ROTATION")
    print("——————————————————")
    printObj(rotation)
    time.sleep(5)
    iter += 1

print(iter)