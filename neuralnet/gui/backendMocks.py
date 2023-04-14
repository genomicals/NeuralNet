import random;

class backendMocks:

    def get_move():
        actions = [ "MoveNorthwest",
                    "MoveNortheast",
                    "MoveSouthwest",
                    "MoveSoutheast",
                    "JumpNorthwest",
                    "JumpNortheast",
                    "JumpSouthwest",
                    "JumpSoutheast",]
        move = random.randint(0, (actions.count - 1))
        index = random.randint(0, )

        print("AI's turn: ")
        print()
        print("Enter index of piece you would like to move:")
        print("Move options: ")
        print("MoveNorthwest \nMoveNortheast \nMoveSouthwest \nMoveSoutheast \nJumpNorthwest \nJumpNortheast \nJumpSouthwest \nJumpSoutheast")
        print()
        move = input("Enter move here: ")



        return (index, actions[move])

    def make_move():
        return

    def peek_red():
        return 

    def peek_black():
        return