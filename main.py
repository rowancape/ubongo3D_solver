import sys

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


calc_field = [field, field]

if len(calc_field) < 1:
  sys.exit("Field invalid")

obj_layers = len(block1)
obj_rows = len(block1[0])
obj_points = len(block1[0][0])

print("LOOP START")
print("______________________________________")

for layer_index, layer in enumerate(calc_field):
  for row_index, row in enumerate(layer):
    for point_index, point in enumerate(row):

      if obj_layers > len(calc_field) - layer_index:
        print("Object cannot fit in the field anymore in the layer axis")
        print(f"Object layer axis size: {obj_layers}. Remaining field layer axis space: {len(calc_field) - layer_index}")

      if obj_rows > len(layer) - row_index:
        print("Object cannot fit in the field anymore in the row axis")
        print(f"Object row axis size: {obj_rows}. Remaining field row axis space: {len(layer) - row_index}")

      if obj_points >  len(row) - point_index:
        print("Object cannot fit in the field anymore in the point axis")
        print(f"Object point axis size: {obj_points}. Remaining field point axis space: {len(row) - point_index}")
        print(point_index + 1)

