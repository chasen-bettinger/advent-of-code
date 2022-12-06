counter=0
elf_dictionary={}
elf_carrying_most_calories=None
top_three=[None, None, None]
sum_tracker=0
ROCK="Rock"
PAPER="Paper"
SCISSORS="SCISSORS"
LOST="Lost"
DRAW="Draw"
WON="Won"
opponent_plays={"A": ROCK, "B": PAPER, "C": SCISSORS}
my_plays={"X": LOST, "Y": DRAW, "Z": WON}
choice_points={ROCK: 1, PAPER: 2, SCISSORS: 3}
outcome_points={LOST: 0, DRAW: 3, WON: 6}
outcome_matrix={
    LOST: {
        PAPER: ROCK,
        ROCK: SCISSORS,
        SCISSORS: PAPER
    },
    DRAW: {
        PAPER: PAPER,
        ROCK: ROCK,
        SCISSORS: SCISSORS
    },
    WON: {
        PAPER: SCISSORS,
        ROCK: PAPER,
        SCISSORS: ROCK
    }
}

score=0
with open(f"input.txt") as input:
    a_line=input.readline()
    while a_line:
        a_line=a_line.replace("\n", "")

        split_line=a_line.split(" ")
        
        opponent_play=opponent_plays[split_line[0]]
        my_outcome=my_plays[split_line[1]]

        points_of_game=outcome_points[my_outcome]
        points_for_choice=choice_points[outcome_matrix[my_outcome][opponent_play]]

        score += points_of_game
        score += points_for_choice

        a_line=input.readline()

print(score)




