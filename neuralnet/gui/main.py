import tkinter as tk
from tkinter import *
from PIL import ImageTk, Image
import utils
import threading
import random
import time
import tools

#***********************************************************************************
#setup and gameplay
#************************************************************************************

# def on_square_click(event, row, col):
#     square = event.widget
#     print(f"Square clicked: {row},{col}")

def make_next_move(event, row, col, window, board_squares, blackPiece, redPiece, blackKing, redKing, GameManager):
    squareInfo = event.widget
    # print(squareInfo.grid_info())
    # print(f"Square clicked: {row},{col}")

    player_turn_label = tk.Label(window, text="", font=("Arial", 16))
    player_turn_label.grid(row=10, column=0, columnspan=10, sticky="nesw")
    
    list = GameManager.peek_board()
    utils.make_board(list, board_squares, blackPiece, redPiece, blackKing, redKing)
    
    square = board_squares[row][col]
    if (((row-1)+(col-1)) % 2 == 1):
        player_turn_label.config(text="Invalid Piece")
        return

    tile = utils.getListIndex((row-1,col-1))
    piece = list[tile]
    # if (piece >= 0):
    #     player_turn_label.config(text="Invalid Piece")
    #     return
    
    popup = tk.Toplevel()
    popup.title("Choose Move")

    # create a frame to hold the radio buttons
    frame = tk.Frame(popup)
    frame.pack()

    # add a label to the frame
    label = tk.Label(frame, text="Choose the move you would like to make:")
    label.pack(side="top", anchor="w")

    # # Center the popup window in the main window
    popup.update_idletasks()
    width = 300
    height = 400
    x = (window.winfo_screenwidth() // 2) - (width // 2)
    y = (window.winfo_screenheight() // 2) - (height // 2)
    popup.geometry('{}x{}+{}+{}'.format(width, height, x, y))  

    move = tk.IntVar()

#     pub enum Action {
#     MoveNorthwest,
#     MoveNortheast,
#     MoveSouthwest,
#     MoveSoutheast,
#     JumpNorthwest,
#     JumpNortheast,
#     JumpSouthwest,
#     JumpSoutheast,
# }
    
    rb1 = tk.Radiobutton(frame, text="ForwardLeft", variable=move, value=0)
    rb1.pack(side="top", anchor="w")
    rb2 = tk.Radiobutton(frame, text="ForwardRight", variable=move, value=1)
    rb2.pack(side="top", anchor="w")
    rb3 = tk.Radiobutton(frame, text="BackLeft", variable=move, value=2)
    rb3.pack(side="top", anchor="w")
    rb4 = tk.Radiobutton(frame, text="BackRight", variable=move, value=3)
    rb4.pack(side="top", anchor="w")

    rb4 = tk.Radiobutton(frame, text="JumpForwardLeft", variable=move, value=4)
    rb4.pack(side="top", anchor="w")
    rb5 = tk.Radiobutton(frame, text="JumpForwardRight", variable=move, value=5)
    rb5.pack(side="top", anchor="w")
    rb6 = tk.Radiobutton(frame, text="JumpBackLeft", variable=move, value=6)
    rb6.pack(side="top", anchor="w")
    rb7 = tk.Radiobutton(frame, text="JumpBackRight", variable=move, value=7)
    rb7.pack(side="top", anchor="w")


    cancel_flag = False  # initialize flag variable

    def on_go_click():
        popup.destroy()

    def on_cancel_click():
        nonlocal cancel_flag  # use nonlocal to modify outer variable
        cancel_flag = True
        popup.destroy()
        

    go_button = tk.Button(popup, text="Go", command=on_go_click)
    go_button.pack()
    cancel_button = tk.Button(popup, text="Cancel", command=on_cancel_click)
    cancel_button.pack()

    # run the popup window and wait for it to be closed
    popup.wait_window()

    if cancel_flag:
        player_turn_label.config(text="Player's Turn (Black)")
        window.update()
        return

    #!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    #check if the move is valid

    action = move.get()
    print(action)
    
    player_turn_label.config(text="AI's Turn (Red)")
    window.update()
    status = GameManager.make_move(tile, action)
    utils.make_board(GameManager.peek_board(), board_squares, blackPiece, redPiece, blackKing, redKing)

    if status > 0:
        popup = tk.Toplevel()
        popup.title("Game Over!")

        # create a frame to hold the radio buttons
        frame = tk.Frame(popup)
        frame.pack()
        
        if(status == 1):
            # add a label to the frame
            label = tk.Label(frame, text="Congratulations! You beat the AI!")
            label.pack(side="top", anchor="w")
            message = "You Win!"
        else:
            # add a label to the frame
            label = tk.Label(frame, text="HAHA you suck lol")
            label.pack(side="top", anchor="w")
            message = "You lost!"

        # # Center the popup window in the main window
        popup.update_idletasks()
        width = 300
        height = 200
        x = (window.winfo_screenwidth() // 2) - (width // 2)
        y = (window.winfo_screenheight() // 2) - (height // 2)
        popup.geometry('{}x{}+{}+{}'.format(width, height, x, y))  

        def on_ok_click():
            popup.destroy()
            
        go_button = tk.Button(popup, text="Ok", command=on_ok_click)
        go_button.pack()

        # run the popup window and wait for it to be closed
        popup.wait_window()
        player_turn_label.config(text=message)
        return

    player_turn_label.config(text="Player's Turn (Black)")
    window.update()


def new_game(window, board_squares, blackPiece, redPiece, blackKing, redKing, GameManager):
    print("Starting a new game...")

    #get starting information about the new game
    game = utils.getGameType(window)

    if(game == 1):
        print("AI vs Player")
        starter = utils.getStartPlayer(window)
    else:
        print("Player vs Player")
        starter = 0

    if (starter == 1):
        print("AI starts")
    else:
        print("Player1 starts")

    startInfo = (starter,game)

    player_turn_label = tk.Label(window, text="", font=("Arial", 16))
    player_turn_label.grid(row=10, column=0, columnspan=10, sticky="nesw")

    #if playing the ai, start game with backend
    if (startInfo[1] == 1):
        aiStarts = False
        #tell the backend who starts (1 for ai, 0 for player)
        if startInfo[0] == 1:
            aiStarts = True

        if aiStarts:
            player_turn_label.config(text="AI's Turn (Red)")
            window.update()
        
        list = GameManager.peek_board()
        
        utils.make_board(list, board_squares, blackPiece, redPiece, blackKing, redKing)
        window.update()
        #!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        GameManager.new_game(aiStarts)

        utils.make_board(GameManager.peek_board(), board_squares, blackPiece, redPiece, blackKing, redKing)

        # Create a label to show the current player's turn
        player_turn_label.config(text="Player's turn (Black)")
        window.update()



class CheckersGame:

    window = tk.Tk()
    window.title("Checkers Game")

    #create the black piece 
    image = Image.open("images/BlackPiece.png")     
    resize_image = image.resize((40, 40))
    blackPiece = ImageTk.PhotoImage(resize_image)

    #create the black king piece
    image = Image.open("images/blackKing.png")   
    resize_image = image.resize((40, 40))
    blackKing = ImageTk.PhotoImage(resize_image)

    #create the red piece
    image = Image.open("images/RedPiece.png")   
    resize_image = image.resize((40, 40))
    redPiece = ImageTk.PhotoImage(resize_image)

    #create the red king piece
    image = Image.open("images/redKing.png")   
    resize_image = image.resize((40, 40))
    redKing = ImageTk.PhotoImage(resize_image)

    #!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    gen = tools.GenerationManager()
    GameManager = gen.create_game()
    #GameManager.new_game(False)

    # class GameManager:

    #         @staticmethod
    #         def new_game(aiStarts):
    #             if aiStarts:
    #                 time.sleep(5)

    #         @staticmethod
    #         def make_move(tile, action):
    #             #0 make another move, 1 player wins, 2 ai wins
    #             print("AI making move...")
    #             time.sleep(3)
    #             return 0
            
    #         @staticmethod
    #         def peek_board(n):
    #             if n==1:
    #                 return [1,1,1,1,1,1,1,1,  1,0,1,0,1,0,1,0,  0,-1,2,-1,0,-1,0,-1,  -1,-1,-1,-1,-1,-1,-1,-1]
    #             elif n==0:
    #                 return [1,1,1,1,1,1,1,1,  1,0,1,0,1,0,1,0,  0,-1,0,-1,0,-1,0,-1,  -1,-1,-1,-1,-1,-1,-1,-1]
    #             else:
    #                 return [1,1,1,1,1,1,1,1,  1,0,1,0,1,-2,1,0,  0,-1,0,-1,0,-1,0,-1,  -1,-1,-1,-1,-1,-1,-1,-1]


    #Create the board 
    board_size = 10
    board_squares = [[None for _ in range(11)] for _ in range(board_size)]
    for row in range(board_size):
        for col in range(board_size):
            color = "black" if (row + col) % 2 == 0 else "#800000"
            canvas = tk.Canvas(window, width=50, height=50, bg = color, highlightthickness=0)
            canvas.grid(row=row, column=col)
            board_squares[row][col] = canvas
            canvas.create_rectangle(0, 0, 60, 60, width=1, outline="gold")
            canvas.bind("<Button-1>", lambda event, row=row, col=col, window=window, board_squares=board_squares, blackPiece=blackPiece, redPiece=redPiece, blackKing=blackKing, redKing=redKing, GameManager=GameManager: make_next_move(event, row, col, window, board_squares, blackPiece, redPiece, blackKing, redKing, GameManager))
    # Add row labels
    for row in range(board_size-1):
        label = tk.Label(window, text=str(board_size - row-1), font=("Arial", 16))
        label.grid(row=row, column=0, sticky="nesw")
    # Add column labels
    for col in range(board_size-2):
        label = tk.Label(window, text=chr(ord("A")+col), font=("Arial", 16))
        label.grid(row=0, column=col+1, sticky="nesw")
    # Add row labels
    for row in range(board_size-1):
        label = tk.Label(window, text=str(board_size - row-1), font=("Arial", 16))
        label.grid(row=row, column=9, sticky="nesw")
    # Add column labels
    for col in range(board_size-2):
        label = tk.Label(window, text=chr(ord("A")+col), font=("Arial", 16))
        label.grid(row=9, column=col+1, sticky="nesw")


    #set the corners to background color
    corner00 = tk.Label(window, text=str(" "), font=("Arial", 16))
    corner00.grid(row=0, column=0, sticky="nesw")
    corner99 = tk.Label(window, text=str(" "), font=("Arial", 16))
    corner99.grid(row=9, column=9, sticky="nesw")
    corner09 = tk.Label(window, text=str(" "), font=("Arial", 16))
    corner09.grid(row=0, column=9, sticky="nesw")
    corner90 = tk.Label(window, text=str(" "), font=("Arial", 16))
    corner90.grid(row=9, column=0, sticky="nesw")

    #Create menu bar
    menubar = tk.Menu(window)
    window.config(menu=menubar)

    # Add game tab
    Game = tk.Menu(menubar, tearoff=0)
    menubar.add_cascade(label="Game", menu=Game)
    Game.add_command(label="New Game", command=lambda window=window, board_squares=board_squares, blackPiece=blackPiece, redPiece=redPiece, blackKing=blackKing, redKing=redKing, GameManager=GameManager: new_game(window, board_squares, blackPiece, redPiece, blackKing, redKing, GameManager))    
    Game.add_separator()
    Game.add_command(label="Exit", command=window.quit)

    #Add generations tab
    Generations = tk.Menu(menubar, tearoff=0)
    menubar.add_cascade(label="Generations", menu=Generations)
    Generations.add_command(label="Train Generations", command=lambda window=window, gen=gen: utils.train_generations(window, gen))
    Generations.add_separator()

    #!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    #get functions for loading and saving
    Generations.add_command(label="Load a Generation")
    Generations.add_command(label="Save Current Generation")

    # Center the popup window in the main window
    window.update_idletasks()
    width = 500
    height = 550
    x = (window.winfo_screenwidth() // 2) - (width // 2)
    y = (window.winfo_screenheight() // 2) - (height // 2)
    window.geometry('{}x{}+{}+{}'.format(width, height, x, y))  


    def run(self):
        self.window.mainloop()

if __name__ == "__main__":
    game = CheckersGame()
    game.run()