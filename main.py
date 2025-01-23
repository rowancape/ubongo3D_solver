from itertools import product

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

                if point == 0:
                    possible_start_points[obj_index].append(
                        [layer_index, row_index, point_index]
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

for obj1_sp, obj2_sp, obj3_sp, obj4_sp in product(*possible_start_points):
    obj_sps = [obj1_sp, obj2_sp, obj3_sp, obj4_sp]

    for obj_index, object in enumerate(objects):
        for li, layer in enumerate(object):
            for ri, row in enumerate(layer):
                for pi, point in enumerate(row):
                    calc_field[obj_sps[obj_index][0]+li][obj_sps[obj_index][1]+ri][obj_sps[obj_index][2]+pi] += point

    num_layers = len(calc_field)
    num_rows = len(calc_field[0])
    num_cols = len(calc_field[0][0])
    wasSuccess = 0

    for layer_idx, row_idx, col_idx in product(range(num_layers), range(num_rows), range(num_cols)):
        point = calc_field[layer_idx][row_idx][col_idx]

        if point != 1:
            calc_field = [[[0, 0, 0, 1], [1, 0, 0, 0], [0, 0, 0, 0]], [[0, 0, 0, 1], [1, 0, 0, 0], [0, 0, 0, 0]]]
            wasSuccess += 1
            break
    
    if wasSuccess == 0:
        print("SUCCESS!")
        for layer in calc_field:
            for row in layer:
                print(row)
    
