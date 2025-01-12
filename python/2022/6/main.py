import fileinput
from collections import Counter

characters_to_process=0
with fileinput.input("input.txt") as input:
    for line in input:
        line.replace("\n", "")

        start=0
        end=13
        found=False

        while found is False:
            four_character_string = line[start:end+1]
            c = Counter(four_character_string)
            found=True

            for key in c:
                value=c[key]

                if value is 1:
                    continue
                
                if value > 1:
                    found=False
                    start += 1
                    end += 1
                    break

            if found is False:
                continue

        characters_to_process=end+1

print(characters_to_process)

            

            




