import tkinter as tk
from tkinter import *

#***********************************************************************************
#utility functions
#***********************************************************************************
def getStartPlayer(window):
    popup = tk.Toplevel()
    popup.title("Starting Player")

    # create a frame to hold the radio buttons
    frame = tk.Frame(popup)
    frame.pack()

    # add a label to the frame
    label = tk.Label(frame, text="Select the starting player:")
    label.pack(side="top", anchor="w")

    # # Center the popup window in the main window
    popup.update_idletasks()
    width = 300
    height = 200
    x = (window.winfo_screenwidth() // 2) - (width // 2)
    y = (window.winfo_screenheight() // 2) - (height // 2)
    popup.geometry('{}x{}+{}+{}'.format(width, height, x, y))  

    startPlayer = tk.IntVar()
    
    rb1 = tk.Radiobutton(frame, text="Player1", variable=startPlayer, value=0)
    rb1.pack(side="top", anchor="w")
    rb2 = tk.Radiobutton(frame, text="AI", variable=startPlayer, value=1)
    rb2.pack(side="top", anchor="w")

    def on_ok_click():
        popup.destroy()

    ok_button = tk.Button(popup, text="Ok!", command=on_ok_click)
    ok_button.pack()

    # run the popup window and wait for it to be closed
    popup.wait_window()

    return startPlayer.get()

def getGameType(window):
    popup = tk.Toplevel()
    popup.title("Game Settings")

    # create a frame to hold the radio buttons
    frame = tk.Frame(popup)
    frame.pack()

    # add a label to the frame
    label = tk.Label(frame, text="Select the game type:")
    label.pack(side="top", anchor="w")

    # # Center the popup window in the main window
    popup.update_idletasks()
    width = 300
    height = 200
    x = (window.winfo_screenwidth() // 2) - (width // 2)
    y = (window.winfo_screenheight() // 2) - (height // 2)
    popup.geometry('{}x{}+{}+{}'.format(width, height, x, y))  

    # Create a variable to store the selected value
    game = tk.IntVar()

    # Create the radio buttons
    rb1 = tk.Radiobutton(frame, text="Player vs Player", variable=game, value=0)
    rb1.pack(side="top", anchor="w")
    rb2 = tk.Radiobutton(frame, text="Player vs AI", variable=game, value=1)
    rb2.pack(side="top", anchor="w")

    def on_ok_click():
        popup.destroy()

    ok_button = tk.Button(popup, text="Ok!", command=on_ok_click)
    ok_button.pack()

    # run the popup window and wait for it to be closed
    popup.wait_window()

    return game.get()

#makes a board based on a given list of 32 spaces
def make_board(board_list, board_squares, blackPiece, redPiece, blackKing, redKing):
    i = 0

    while i < 32:

        for j in range(1,9,2):

            for k in range(1,9):
                col = k

                if i % 2 == 0:
                    row = j
                else:
                    row = j+1

                square = board_squares[row][col]
                square.delete("piece")

                piece = board_list[i]
                if (piece == 1):
                    square.create_image(25, 25, anchor=CENTER, image=redPiece, tag="piece")
                elif (piece == -1):
                    square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
                elif (piece == 2):
                    square.create_image(25, 25, anchor=CENTER, image=redKing, tag="piece")
                elif (piece == -2):
                    square.create_image(25, 25, anchor=CENTER, image=blackKing, tag="piece")

                i += 1
                

                    



    # for row in range(len(board_matrix)):
    #     for col in range(len(board_matrix[0])):

    #         if ((row+1)%2 == 1):

    #             square = board_squares[row+1][col*2+1]
    #             square.delete("piece")
    #             #square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
    #             #square.create_image(25, 25, anchor=CENTER, image=redPiece, tag="piece")
    #         else:
    #             square = board_squares[row+1][(col+1)*2]
    #             square.delete("piece")
    #             #square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
    #             #square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")

    #         piece = board_matrix[row][col]
    #         if (piece == 1):
    #             square.create_image(25, 25, anchor=CENTER, image=redPiece, tag="piece")
    #         elif (piece == -1):
    #             square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
    #         elif (piece == 2):
    #             square.create_image(25, 25, anchor=CENTER, image=redKing, tag="piece")
    #         elif (piece == -2):
    #             square.create_image(25, 25, anchor=CENTER, image=blackKing, tag="piece")


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