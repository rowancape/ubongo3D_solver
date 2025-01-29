from itertools import product

# fmt: off
field = [[[0, 0, 0, 1], 
          [1, 0, 0, 0], 
          [0, 0, 0, 0]],
          
         [[0, 0, 0, 1], 
          [1, 0, 0, 0], 
          [0, 0, 0, 0]]]

block1 = [[[1, 1, 1], 
           [1, 0, 0]], 

          [[0, 0, 1], 
           [0, 0, 0]]]

block2 = [[[1, 1, 1], 
           [1, 0, 1]], 

          [[0, 0, 0], 
           [0, 0, 0]]]

block3 = [[[1, 1], 
           [1, 1]], 

          [[1, 0], 
           [0, 0]]]

block4 = [[[1, 1, 1], 
           [0, 1, 0]], 

          [[1, 0, 0], 
           [0, 0, 0]]]        
# fmt: on

blocks = [block1, block2, block3, block4]


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


rotated_blocks = [generateRotations(block) for block in blocks]

all_combinations = product(*rotated_blocks)

for combination in all_combinations:
    True
    # Compute every possible location on the field for each object with the rotational states in
    #  the current combination
