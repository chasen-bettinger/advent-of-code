import fileinput

total=0
character_list=['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']
with fileinput.input("input.txt") as input:
    for line in input:
        line=line.replace("\n", "")

        # find the length of the string
        # assumption: length of line is always even
        mid_point_of_components=int(len(line) / 2)
        # split the string in half
        first_half_of_components=line[:mid_point_of_components]
        second_half_of_components=line[mid_point_of_components:]

        # iterate through both parts
        #   in the first iteration, store the results
        #   in the second iteration, look for duplicates
        duplicate_component=None
        for component in second_half_of_components:
            if component in first_half_of_components:
                duplicate_component=component
                break
                
        # if there is a duplicate, find its priority
        is_duplicate_component_uppercase=duplicate_component.isupper()

        component_to_search=duplicate_component.lower()
        character_points = character_list.index(component_to_search) + 1

        if is_duplicate_component_uppercase:
            character_points += 26

        # add the priority to the running sum
        total += character_points

print(total)
