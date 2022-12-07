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

        def iterate_group(group):
            starting_value=int(group[0])
            ending_value=int(group[1])
            group_list=[starting_value]
            current_value=starting_value + 1
            while current_value<=ending_value:
                group_list.append(current_value)
                current_value += 1
            
            return group_list

        first_group_list=iterate_group(first_group)
        second_group_list=iterate_group(second_group)

        for item in second_group_list:
            value=int(item)
            if value in first_group_list:
                overlaps += 1
                break

print(overlaps)