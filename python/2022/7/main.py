"""
- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)

$ cd /
$ ls
dir fts
dir jnwr
dir lrvl
dir nzgprvw
dir snwqjgj
16394 tllvcdr.sjl
195891 zbdp.gqb
dir zddrb
$ cd fts
$ ls
dir dlqtffw
dir rbfmmjvd
254713 wvwhrb.dhh
$ cd dlqtffw
$ ls
73533 nqbvg.fgd
$ cd ..
$ cd rbfmmjvd
$ ls

[root]
16394 + 195891

[root, fts]
254713

[root, fts, dlqtffw]
73533

root;fts: 0

..

:assign:
    - [root, fts] sum += 73533
    - dlqtffw -> 73533
    - directories: [dlqtffw]

[root, fts, rbfmmjvd]

{
  root:{
    size: 0
    children: {
      fts: {
        size: 0 
        children: {} 
      }
    }
  }
}


"""


import fileinput


def is_number(input):

    value = False

    try:
        input_as_int = int(input)
        value = type(input_as_int) is int
    except:
        pass

    return value


def get_directory(dictionary, complete_path):
    dictionary_cursor = dictionary["root"]

    for path in complete_path:

        if path == "root":
            continue

        dictionary_cursor = dictionary_cursor["children"][path]

    return dictionary_cursor


directory_dictionary = {"root": {"size": 0, "children": {}}}
active_directory_path = ["root"]
running_sum = 0
with fileinput.input("input.txt") as input:
    for line in input:
        line = line.replace("\n", "")

        split_line = line.split(" ")

        dictionary_cursor = directory_dictionary["root"]
        for path in active_directory_path:

            if path == "root":
                continue

            dictionary_cursor = dictionary_cursor["children"][path]

        if split_line[0] == "$":

            if split_line[1] == "cd":

                if split_line[2] == "/":
                    active_directory_path = ["root"]
                    continue

                if split_line[2] == "..":
                    active_directory_path.pop()
                    continue

                active_directory_path.append(split_line[2])
                continue

        if split_line[0] == "dir":
            dictionary_cursor["children"][split_line[1]] = {"size": 0, "children": {}}
            continue

        if is_number(split_line[0]):
            dictionary_cursor["children"][split_line[1]] = {
                "size": int(split_line[0]),
                "children": {},
            }
            continue

# print(directory_dictionary)


def get_directories_sizes(head):
    for child_key in head["children"]:
        child = head["children"][child_key]

        if not child["children"]:
            head["size"] += child["size"]
        else:
            head["size"] += get_directories_sizes(child)

    return head["size"]


root_start = directory_dictionary["root"]
get_directories_sizes(root_start)

"""
root
  - a (8) | total=0
    ~ b 5 | total=0
    ~ c 5 | total=0
    - d (3) | total=3
      ~ e 3 | total=3
  - b (4) | total=7
    ~ c 3 | total=7
    ~ d 1 | total=7
"""


def sum_directories_less_than_100000(head, total):
    if not bool(head["children"]):
        return head, total
    else:
        total += head["size"] if head["size"] <= 100000 else 0
        for child_key in head["children"]:
            child = head["children"][child_key]
            total = sum_directories_less_than_100000(child, total)[1]

        return head, total


th_sum = sum_directories_less_than_100000(root_start, 0)
print(th_sum[1])

total_used = root_start["size"]
available = 70000000
target_unused = 30000000

current_unused = available - total_used
shortfall = target_unused - current_unused


def find_file_to_delete(head, target):
    if not bool(head["children"]):
        return target

    if head["size"] >= shortfall and head["size"] < target:
        target = head["size"]

    for child_key in head["children"]:
        child = head["children"][child_key]
        target = find_file_to_delete(child, target)

    return target


t = find_file_to_delete(root_start, available)
print(t)
