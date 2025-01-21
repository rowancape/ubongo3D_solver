import sys

# fmt: off
field = [[0, 0, 0, 1], 
         [1, 0, 0, 0], 
         [0, 0, 0, 0]]

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

calc_field = [field, field]


def mainLoop():
    for layer_index, layer in enumerate(calc_field):
        if obj_layers > len(calc_field) - layer_index:
            print("Object cannot fit in the field anymore in the layer axis")
            print(
                f"Object layer axis size: {obj_layers}. Remaining field layer axis space: {len(calc_field) - layer_index}"
            )
            break

        for row_index, row in enumerate(layer):
            if obj_rows > len(layer) - row_index:
                print("Object cannot fit in the field anymore in the row axis")
                print(
                    f"Object row axis size: {obj_rows}. Remaining field row axis space: {len(layer) - row_index}"
                )
                break

            for point_index, point in enumerate(row):
                if obj_points > len(row) - point_index:
                    print("Object cannot fit in the field anymore in the point axis")
                    print(
                        f"Object point axis size: {obj_points}. Remaining field point axis space: {len(row) - point_index}"
                    )
                    break

                if point != 1:
                    possible_start_points[obj_index].append(
                        [point_index + 1, row_index + 1, layer_index + 1]
                    )


possible_start_points = [[], [], [], []]
objects = [block1, block2, block3, block4]

print("LOOP START")
print("————————————————————————————————")

for obj_index, object in enumerate(objects):
    obj_layers = len(object)
    obj_rows = len(object[0])
    obj_points = len(object[0][0])
    mainLoop()

for single_obj_start_points in possible_start_points:
    print(single_obj_start_points)

