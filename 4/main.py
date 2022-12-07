import fileinput
overlaps=0
with fileinput.input("input.txt") as input:
    for line in input:
        line=line.replace("\n","")

        #21-81,20-96

        # split the line [21-81, 20-96]
        split_line=line.split(",")
        # split the pair [21, 81], [20, 96]
        first_group=split_line[0].split("-")
        second_group=split_line[1].split("-")



        first_group_fully_contains= (int(first_group[0]) <= int(second_group[0])) and (int(first_group[1]) >= int(second_group[1]))
        second_group_fully_contains= (int(first_group[0]) >= int(second_group[0])) and (int(first_group[1]) <= int(second_group[1]))

        is_overlap = first_group_fully_contains is True or second_group_fully_contains is True

        if is_overlap:
            overlaps += 1

print(overlaps)