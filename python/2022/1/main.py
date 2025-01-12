import fileinput
import heapq
counter=0
top_three=[]
sum_tracker=0

with fileinput.input("input.txt") as input:
    for a_line in input:
        a_line=a_line.replace("\n", "")

        if a_line is "":
            counter += 1

            top_three = heapq.nlargest(3, top_three + [sum_tracker])
            sum_tracker = 0

        else:

            sum_tracker += int(a_line)

sum_of_top_three=sum(top_three)
print(sum_of_top_three)