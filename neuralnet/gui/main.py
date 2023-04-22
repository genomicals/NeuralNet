import tkinter as tk
from tkinter import *
from PIL import ImageTk, Image
import utils
import threading

#***********************************************************************************
#setup and gameplay
#************************************************************************************

def new_game(window, board_squares, blackPiece, redPiece, blackKing, redKing):
    print("Starting a new game...")
    list = [1,1,1,1,1,1,1,1,  1,0,1,0,1,0,1,0,  0,-1,0,-1,0,-1,0,-1,  -1,-1,-1,-1,-1,-1,-1,-1]
    utils.make_board(list, board_squares, blackPiece, redPiece, blackKing, redKing)

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

    #if playing the ai, start game with backend
    if (startInfo[1] == 1):
        i=1
        #tell the backend who starts (1 for ai, 0 for player)
    
        #GameManager game = start_game()!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!





    def runGame():
        while (True):
            i = 1

     # Start a new thread
    thread = threading.Thread(target=runGame)
    thread.daemon = True  # Set the thread as a daemon thread so it will stop when the program exits
    thread.start()





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

    # Create a label to show the current player's turn
    player_turn_label = tk.Label(window, text="Player 1's Turn", font=("Arial", 16))
    player_turn_label.grid(row=10, column=0, columnspan=10, sticky="nesw")

    #Create menu bar
    menubar = tk.Menu(window)
    window.config(menu=menubar)

    # Add game tab
    Game = tk.Menu(menubar, tearoff=0)
    menubar.add_cascade(label="Game", menu=Game)
    Game.add_command(label="New Game", command=lambda window=window, board_squares=board_squares, blackPiece=blackPiece, redPiece=redPiece, blackKing=blackKing, redKing=redKing: new_game(window, board_squares, blackPiece, redPiece, blackKing, redKing))    
    Game.add_separator()
    Game.add_command(label="Exit", command=window.quit)

    #Add generations tab
    Generations = tk.Menu(menubar, tearoff=0)
    menubar.add_cascade(label="Generations", menu=Generations)
    Generations.add_command(label="Train Generations", command=lambda window=window: utils.train_generations(window))
    Generations.add_separator()
    Generations.add_command(label="Load a Generation")
    Generations.add_command(label="Save Current Generation")

    # Center the popup window in the main window
    window.update_idletasks()
    width = 500
    height = 500
    x = (window.winfo_screenwidth() // 2) - (width // 2)
    y = (window.winfo_screenheight() // 2) - (height // 2)
    window.geometry('{}x{}+{}+{}'.format(width, height, x, y))  

    # #Add the label displaying what player's turn it is. 
    # player = 0
    # label = tk.Label(window, text=f"Player {player}'s Turn")
    # label.grid(row=0, column=0)


    def run(self):
        self.window.mainloop()

if __name__ == "__main__":
    game = CheckersGame()
    game.run()