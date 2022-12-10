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

directory_dictionary={"root": {
  "size": 0,
  "children": {}
}}
active_directory=None
read_mode=False
with fileinput.input("input.txt") as input:
  for line in input:
    line=line.replace("\n","")

    split_line=line.split(" ")

    if split_line[0] is "$":

      if split_line[1] is "cd":

        if split_line[2] is "/":
          active_directory="root"
          continue

        active_directory = split_line[2]
        continue

    if split_line[0] is "dir":

        




