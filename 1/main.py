counter=0
elf_dictionary={}
elf_carrying_most_calories=None
top_three=[None, None, None]
sum_tracker=0

def check_for_top_three(value):

    for i, value_in_list in enumerate(top_three):
        
        if value_in_list is None:
            return i

        if value > value_in_list:
            return i

    return None

def insert_into_top_three(index, value):
    temp_value=None

    for i, value_in_list in enumerate(top_three):

        if i is index:
            temp_value = value_in_list
            top_three[i] = value

        if i is index + 1:
            top_three[i] = temp_value
            temp_value=value_in_list
            index += 1

with open(f"input.txt") as input:
    a_line=input.readline()
    while a_line:
        a_line=a_line.replace("\n", "")

        if a_line is "":
            counter += 1
            elf_dictionary[counter] = sum_tracker

            if elf_carrying_most_calories is None or elf_dictionary[elf_carrying_most_calories] < sum_tracker:
                elf_carrying_most_calories = counter

            top_three_index=check_for_top_three(sum_tracker)
            if top_three_index is not None:
                insert_into_top_three(top_three_index, sum_tracker)

            sum_tracker = 0
                

        else:

            sum_tracker += int(a_line)

        a_line=input.readline()

most_calories=elf_dictionary[elf_carrying_most_calories]
sum_of_top_three=0
for i in top_three:
    sum_of_top_three += i
print(sum_of_top_three)



