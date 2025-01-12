import fileinput
import pprint
import json
import re
building_dictionary=True
stack_dictionary={}
def populate_dictionary(line, dictionary):
    position_counter=1
    empty_line_counter=0
    for item in line: 

        if empty_line_counter is 4:
            position_counter += 1
            empty_line_counter=0

        if item is "":
            empty_line_counter += 1
            continue

        convert_to_int=None

        try:
            convert_to_int=int(item)
        except ValueError as error:
            pass


        if isinstance(convert_to_int, int) is True:
            break

        if position_counter not in dictionary:
            dictionary[position_counter] = []

        dictionary[position_counter].insert(0, item)
        position_counter += 1
    
    return dictionary

with fileinput.input("input.txt") as input:
    for line in input:
        line=line.replace("\n", "").split(" ")
        if len(line) is 1 and line[0] is '': 
            building_dictionary=False
            stack_dictionary=dict(sorted(stack_dictionary.items()))
            continue

        if building_dictionary is True:
            stack_dictionary=populate_dictionary(line, stack_dictionary)
            continue

        quantity=int(line[1])
        from_location=int(line[3])
        to_location=int(line[5])

        # "move 4 from 2 to 1"

        # move pointer to from
        sending_queue=stack_dictionary[from_location]
        receiving_queue=stack_dictionary[to_location]
        buffer_queue=[]
        # create a queue of size {quantity} using append
        while len(buffer_queue) != quantity:
            item_to_add=sending_queue.pop()
            buffer_queue.insert(0, item_to_add)
        
        # flush the queue into the to
        for item in buffer_queue:
            receiving_queue.append(item)

result=''
for key in stack_dictionary:
    value=stack_dictionary[key]
    value_to_add=value[len(value)-1]
    value_to_add=re.sub("\[", "", value_to_add)
    value_to_add=re.sub("\]", "", value_to_add)
    result+=value_to_add
print(result)
    

