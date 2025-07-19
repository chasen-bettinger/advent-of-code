import os
import json

print("Hello World")

files_in_p12 = []
for root, dirs, files in os.walk("src/p_12"):
    for file in files:
        # file_path = os.path.join(root, file)
        files_in_p12.append(file)

bad_files = []
for root, dirs, files in os.walk("src/puzzle_12"):
    for file in files:

        if file not in files_in_p12:
            print(f"File {file} not found in src/p_12")
            break

        with open(os.path.join(root, file), "r") as f:
            data = json.load(f)

            a_plant = data["plant"]
            a_area = data["area"]
            a_sides = data["sides"]

            with open(os.path.join("src/p_12", file), "r") as f:
                data_p12 = json.load(f)

                p12_plant = data_p12["plant"]
                p12_area = data_p12["area"]
                p12_sides = data_p12["sides"]

                bad_plant = p12_plant != a_plant
                bad_area = p12_area != a_area
                bad_sides = p12_sides != a_sides

                if bad_plant or bad_area or bad_sides:
                    bad_files.append(file)
                    # print(f"File {file} has bad plant, area, or sides")
                    # break

        # file_path = os.path.join(root, file)

        # print(f"Found file: {file_path}")

print(bad_files)
