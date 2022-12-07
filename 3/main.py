import fileinput

total=0
character_list=['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']
group=[]

with fileinput.input("input.txt") as input:
    for line in input:
        line=line.replace("\n", "")

        group.append(line)

        if len(group) is not 3:
            continue

        all_group_frequency={}
        for group_line in group:
            group_additions=[]

            for letter in group_line:

                if letter in group_additions:
                    continue

                group_additions.append(letter)

                if letter not in all_group_frequency:
                    all_group_frequency[letter] = 0

                all_group_frequency[letter] = all_group_frequency[letter] + 1
                

        sorted_frequencies = {key: val for key, val in sorted(all_group_frequency.items(), key = lambda ele: ele[1], reverse = True)}
        target_letter=None

        for key in sorted_frequencies:
            frequency=sorted_frequencies[key]

            if frequency is 3:
                target_letter=key
                break

        # if there is a duplicate, find its priority
        is_duplicate_component_uppercase=target_letter.isupper()

        component_to_search=target_letter.lower()
        character_points = character_list.index(component_to_search) + 1

        if is_duplicate_component_uppercase:
            character_points += 26

        # add the priority to the running sum
        total += character_points
        group=[]

print(total)
