from enum import IntEnum
from collections import namedtuple

class ChoiceWeight(IntEnum):
    Rock: int = 1
    Paper: int = 2
    Sissor: int = 3

class ResultScore(IntEnum):
    Lost: int = 0
    Draw: int = 3
    Win: int = 6

ChoiceMap = namedtuple('ChoiceMap', 'rock paper sissor')

if __name__ == "__main__":
    their_hand: ChoiceMap = ChoiceMap("A", "B", "C")
    my_hand: ChoiceMap = ChoiceMap("X", "Y", "Z")
    my_score: int = 0

    with open('guide.txt', 'r') as f:
        for line in f:
            # opponent_choice, my_choice
            round_choices = line.rstrip().split(' ')
            round_result = None
            match round_choices:
                case [their_hand.rock, my_hand.rock]:
                        round_result = ResultScore.Draw
                case [their_hand.rock, my_hand.paper]:
                    round_result = ResultScore.Win
                case [their_hand.rock, my_hand.sissor]:
                    round_result = ResultScore.Lost
                case [their_hand.paper, my_hand.rock]:
                    round_result = ResultScore.Lost
                case [their_hand.paper, my_hand.paper]:
                    round_result = ResultScore.Draw
                case [their_hand.paper, my_hand.sissor]:
                    round_result = ResultScore.Win
                case [their_hand.sissor, my_hand.rock]:
                    round_result = ResultScore.Win
                case [their_hand.sissor, my_hand.paper]:
                    round_result = ResultScore.Lost
                case [their_hand.sissor, my_hand.sissor]:
                    round_result = ResultScore.Draw
                case other:
                    raise ValueError("Round result wasn't assigned correctly!")
            my_score += round_result

            match round_choices[1]:
                case my_hand.rock:
                    my_score += ChoiceWeight.Rock
                case my_hand.paper:
                    my_score += ChoiceWeight.Paper
                case my_hand.sissor:
                    my_score += ChoiceWeight.Sissor
                case other:
                    raise ValueError("Round choice didn't translate correctly!")
                
            #print(f"Round result: {round_result} {my_score}")

    print(f"Total Score: {my_score}")