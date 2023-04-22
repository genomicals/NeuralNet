import tkinter as tk
from tkinter import *
from PIL import ImageTk, Image

#***********************************************************************************
#utility functions
#***********************************************************************************

def make_board(board_matrix, board_squares, blackPiece, redPiece, blackKing, redKing):

    for row in range(len(board_matrix)):
        for col in range(len(board_matrix[0])):

            if ((row+1)%2 == 1):

                square = board_squares[row+1][col*2+1]
                square.delete("piece")
                #square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
                #square.create_image(25, 25, anchor=CENTER, image=redPiece, tag="piece")
            else:
                square = board_squares[row+1][(col+1)*2]
                square.delete("piece")
                #square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
                #square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")

            piece = board_matrix[row][col]
            if (piece == 1):
                square.create_image(25, 25, anchor=CENTER, image=redPiece, tag="piece")
            elif (piece == -1):
                square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
            elif (piece == 2):
                square.create_image(25, 25, anchor=CENTER, image=redKing, tag="piece")
            elif (piece == -2):
                square.create_image(25, 25, anchor=CENTER, image=blackKing, tag="piece")
                
def new_game(board_squares, blackPiece, redPiece, blackKing, redKing):
    print("Starting a new game...")
    matrix = [[1,1,1,1],[1,1,1,1],[1,1,1,1],[0,-2,0,0],[0,2,0,0],[-1,-1,-1,-1],[-1,-1,-1,-1],[-1,-1,-1,-1]]
    make_board(matrix, board_squares, blackPiece, redPiece, blackKing, redKing)


def train_generations(window):
        # Create new pop-up window
    popup = tk.Toplevel(window)
    popup.title("Generation Generator")

    # Create label and text entry widgets for name input
    label = tk.Label(popup, text="How many generations would you like to train?")
    label.grid(row=0, column=0)

    num_generations = tk.Entry(popup)
    num_generations.grid(row=1, column=0)

    
    def train():
        print("running train")
        n = int(num_generations.get())
        popup.destroy()
        popup2 = tk.Toplevel(window)
        popup2.title("Generating")
        status = tk.Label(popup2)
        status.grid(row=0, column=0)
        # Center the popup window in the main window
        popup2.update_idletasks()
        width = 300
        height = 300
        x = (popup2.winfo_screenwidth() // 2) - (width // 2)
        y = (popup2.winfo_screenheight() // 2) - (height // 2)
        popup2.geometry('{}x{}+{}+{}'.format(width, height, x, y))

        stop = False

        def stop_training():
            global stop
            stop = True

        # Create OK button that retrieves name and closes window
        stop_button = tk.Button(popup2, text="Stop Training", command = stop_training)
        stop_button.grid(row=5, column=0)   

        for i in range(0, n):
            popup2.geometry('{}x{}+{}+{}'.format(width, height, x, y))
            status.config(text=f"Training {i+1}/{n} generations...")
            #call train generation funciton-----------------------------------------------
            if (stop == True):
                break
        stop = False

        result = tk.Label(popup2)
        result.grid(row=1, column=0)
        result.config(text=f"Training of {n} generations completed!")
            

    # Create OK button that retrieves name and closes window
    ok_button = tk.Button(popup, text="OK", command = train)
    ok_button.grid(row=2, column=0)  
    

    # Center the popup window in the main window
    popup.update_idletasks()
    width = popup.winfo_width()
    height = popup.winfo_height()
    x = (window.winfo_screenwidth() // 2) - (width // 2)
    y = (window.winfo_screenheight() // 2) - (height // 2)
    popup.geometry('{}x{}+{}+{}'.format(width, height, x, y))  

#***********************************************************************************
#setup and gameplay
#************************************************************************************
class CheckersGame:

    window = tk.Tk()
    window.title("Checkers Game")

    #create the black piece 
    image = Image.open("BlackPiece.png")     
    resize_image = image.resize((40, 40))
    blackPiece = ImageTk.PhotoImage(resize_image)

    #create the black king piece
    image = Image.open("blackKing.png")   
    resize_image = image.resize((40, 40))
    blackKing = ImageTk.PhotoImage(resize_image)

    #create the red piece
    image = Image.open("RedPiece.png")   
    resize_image = image.resize((40, 40))
    redPiece = ImageTk.PhotoImage(resize_image)

    #create the red king piece
    image = Image.open("redKing.png")   
    resize_image = image.resize((40, 40))
    redKing = ImageTk.PhotoImage(resize_image)

    #Create the board 
    board_size = 10
    board_squares = [[None for _ in range(10)] for _ in range(board_size)]
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

    #Create menu bar
    menubar = tk.Menu(window)
    window.config(menu=menubar)

    # Add game tab
    Game = tk.Menu(menubar, tearoff=0)
    menubar.add_cascade(label="Game", menu=Game)
    Game.add_command(label="New Game", command=lambda board_squares=board_squares, blackPiece=blackPiece, redPiece=redPiece, blackKing=blackKing, redKing=redKing: new_game(board_squares, blackPiece, redPiece, blackKing, redKing))    
    Game.add_separator()
    Game.add_command(label="Exit", command=window.quit)

   

    #Add generations tab
    Generations = tk.Menu(menubar, tearoff=0)
    menubar.add_cascade(label="Generations", menu=Generations)
    Generations.add_command(label="Train Generations", command=lambda window=window: train_generations(window))
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

    def run(self):
        self.window.mainloop()

if __name__ == "__main__":
    game = CheckersGame()
    game.run()