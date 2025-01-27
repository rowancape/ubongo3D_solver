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

# RotateY:
# N:th element from each row goes to n:th layer, said element of upper layers coming first 

def rotateX(object, rotations = 1):
    rotations = rotations % 4

    for _ in range(rotations):
        layers = len(object)
        rows = len(object[0])
        points = len(object[0][0])

        rotatedObject = [
            [
                [object[j][rows - 1 - i][k] for k in range(points)]
                for j in range(layers)
            ]
            for i in range(rows)
        ]
        object = rotatedObject

    for layer in object:
        print("->")
        for row in layer:
            print(row)
    
    return object

def rotateY(object, rotations = 1):
    rotations = rotations % 4

    for _ in range(rotations):
        layers = len(object)
        rows = len(object[0])
        points = len(object[0][0])

        rotatedObject = [
            [
                [object[layers - 1 - k][j][i] for k in range(layers)]
                for j in range(rows)
            ]
            for i in range(points)
        ]
        object = rotatedObject

    for layer in object:
        print("->")
        for row in layer:
            print(row)
    
    return object

def rotateZ(object, rotations = 1):
    rotations = rotations % 4

    for _ in range(rotations):
        layers = len(object)
        rows = len(object[0])
        points = len(object[0][0])

        rotatedObject = [
            [
                [object[i][rows - 1 - k][j] for k in range(rows)]
                for j in range(points)
            ]
            for i in range(layers)
        ]
        object = rotatedObject

    for layer in object:
        print("->")
        for row in layer:
            print(row)
    
    return object
    

rotateY(block4, 3)
