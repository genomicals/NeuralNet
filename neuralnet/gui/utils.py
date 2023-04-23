import tkinter as tk
from tkinter import *
import threading

#***********************************************************************************
#utility functions
#***********************************************************************************


# def convertListToMatrix(list):
#     #list = [1,1,1,1,1,1,1,1, 1,0,1,0,1,0,1,0, 0,-1,0,-1,0,-1,0,-1, -1,-1,-1,-1,-1,-1,-1,-1]
#     # matrix = [[ 1, 1, 1, 1],
#     #           [ 1, 1, 1, 1],
#     #           [ 1, 1, 1, 1],
#     #           [ 0, 0, 0, 0],
#     #           [ 0, 0, 0, 0],
#     #           [-1,-1,-1,-1],
#     #           [-1,-1,-1,-1],
#     #           [-1,-1,-1,-1]]

#     matrix = [[0 for _ in range(4)] for _ in range(8)]
#     print(str(list))

#     i = 0
#     while i < 32:
        
#         for j in range(0,8,2):

#             for k in range(0,4):
#                 col = k

#                 if i % 2 == 0:
#                     row = j
#                 else:
#                     row = j+1

#                 matrix[row][col] = list[i]
#                 print(f"Matrix: ({row},{col})")
#                 print(f"List: {i}")
#                 print(list[i])
#                 i += 1

#                 # square = board_squares[row][col]
#                 # square.delete("piece")

#                 # piece = board_list[i]
#                 # if (piece == 1):
#                 #     square.create_image(25, 25, anchor=CENTER, image=redPiece, tag="piece")
#                 # elif (piece == -1):
#                 #     square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
#                 # elif (piece == 2):
#                 #     square.create_image(25, 25, anchor=CENTER, image=redKing, tag="piece")
#                 # elif (piece == -2):
#                 #     square.create_image(25, 25, anchor=CENTER, image=blackKing, tag="piece")

#     return matrix
def getListIndex(coords):
    coordsDict = {(0,0): 0,
                  (1,1): 1,
                  (0,2): 2,
                  (1,3): 3,
                  (0,4): 4,
                  (1,5): 5,
                  (0,6): 6,
                  (1,7): 7,
                  (2,0): 8,
                  (3,1): 9,
                  (2,2): 10,
                  (3,3): 11,
                  (2,4): 12,
                  (3,5): 13,
                  (2,6): 14,
                  (3,7): 15,
                  (4,0): 16,
                  (5,1): 17,
                  (4,2): 18,
                  (5,3): 19,
                  (4,4): 20,
                  (5,5): 21,
                  (4,6): 22,
                  (5,7): 23,
                  (6,0): 24,
                  (7,1): 25,
                  (6,2): 26,
                  (7,3): 27,
                  (6,4): 28,
                  (7,5): 29,
                  (6,6): 30,
                  (7,7): 31}
    
    return coordsDict[coords]

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
    rb2 = tk.Radiobutton(frame, text="Player vs AI", variable=game, value=1)
    rb2.pack(side="top", anchor="w")

    def on_ok_click():
        popup.destroy()

    ok_button = tk.Button(popup, text="Ok!", command=on_ok_click)
    ok_button.pack()

    # run the popup window and wait for it to be closed
    popup.wait_window()

    return game.get()

#makes a board based on a given 8x4 matrix
def make_board(list, board_squares, blackPiece, redPiece, blackKing, redKing):

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

                piece = list[i]
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
    #         #print(f"Matrix: row: {row}, col: {col}")
    #         if ((row)%2 == 1):
                
    #             square = board_squares[row+1][(col*2)+2]
    #             #print(f"Board: row: {row+1}, col: {(col*2)+2}")
    #             square.delete("piece")
    #             #square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
    #             #square.create_image(25, 25, anchor=CENTER, image=redPiece, tag="piece")
    #         else:
    #             square = board_squares[row+1][(col*2)+1]
    #             #print(f"Board: row: {row+1}, col: {(col*2)+1}")
    #             square.delete("piece")
    #             #square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
    #             #square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")

    #         piece = board_matrix[row][col]
    #         #print(f"Piece: {piece}")
    #         if (piece == 1):
    #             square.create_image(25, 25, anchor=CENTER, image=redPiece, tag="piece")
    #         elif (piece == -1):
    #             square.create_image(25, 25, anchor=CENTER, image=blackPiece, tag="piece")
    #         elif (piece == 2):
    #             square.create_image(25, 25, anchor=CENTER, image=redKing, tag="piece")
    #         elif (piece == -2):
    #             square.create_image(25, 25, anchor=CENTER, image=blackKing, tag="piece")


def train_generations(window, GenerationManager):
    # Create new pop-up window
    popup = tk.Toplevel(window)
    popup.title("Generation Generator")

    # Create label and text entry widgets for name input
    label = tk.Label(popup, text="How many generations would you like to train?")
    label.grid(row=0, column=0)

    num_generations = tk.Entry(popup)
    num_generations.grid(row=1, column=0)

    #Train a specified number of generations
    def train(infinite):
        print("running train...")
        if (infinite != True):
            n = int(num_generations.get())
        else:
            n = 1

        popup.destroy()
        popup2 = tk.Toplevel(window)
        popup2.title("Generation Trainer")

        # create a frame
        frame = tk.Frame(popup2)
        frame.pack()

        status = tk.Label(frame)
        status.pack()

        # Center the popup window in the main window
        popup2.update_idletasks()
        width = 300
        height = 300
        x = (popup2.winfo_screenwidth() // 2) - (width // 2)
        y = (popup2.winfo_screenheight() // 2) - (height // 2)
        popup2.geometry('{}x{}+{}+{}'.format(width, height, x, y))

        stop = False
        totalTrained = 0

        def stop_training():
            nonlocal stop  # use nonlocal to modify outer variable
            stop = True

        # Create stop button to stop the training
        stop_button = tk.Button(popup2, text="Stop Training", command = stop_training)
        stop_button.pack()

        i = 0

        while(i < n):
            status.config(text=f"Training {i+1}/{n} generations...")
            popup2.update()
            #call train generation funciton!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            #Start a new thread to train the generation
            thread = threading.Thread(target=GenerationManager.train_generation)
            thread.start()

            thread.join()

            totalTrained = totalTrained + 1

            if (stop == True):
                break

            if infinite == False:
                i = i+1
            else:
                i = i+1
                n = n+1

        result = tk.Label(frame)
        result.pack(side="top", anchor="w")
        result.config(text=f"Training of {totalTrained}/{n} generations completed!")
            

    # Create OK button that retrieves name and closes window
    ok_button = tk.Button(popup, text="OK", command=lambda: train(False))
    ok_button.grid(row=2, column=0)  

    # Create infinite button
    infinite_button = tk.Button(popup, text="Infinite", command=lambda: train(True))
    infinite_button.grid(row=3, column=0)  

    # Center the popup window in the main window
    popup.update_idletasks()
    width = popup.winfo_width()
    height = popup.winfo_height()
    x = (window.winfo_screenwidth() // 2) - (width // 2)
    y = (window.winfo_screenheight() // 2) - (height // 2)
    popup.geometry('{}x{}+{}+{}'.format(width, height, x, y))  

def save_generation(window, GenerationManager):

    def save(popup, name):
        GenerationManager.save_generation(name)
        popup.destroy()

    # Create new pop-up window
    popup = tk.Toplevel(window)
    popup.title("Save Current Generation")

    # Create label and text entry widgets for name input
    label = tk.Label(popup, text="What would you like to name this generation?")
    label.grid(row=0, column=0)

    name = tk.Entry(popup)
    name.grid(row=1, column=0)

    name = name.get()

    # Create OK button that retrieves name and closes window
    ok_button = tk.Button(popup, text="Save", command=lambda popup=popup, name=name: save(popup, name))
    ok_button.grid(row=2, column=0)  

    # Center the popup window in the main window
    popup.update_idletasks()
    width = popup.winfo_width()
    height = popup.winfo_height()
    x = (window.winfo_screenwidth() // 2) - (width // 2)
    y = (window.winfo_screenheight() // 2) - (height // 2)
    popup.geometry('{}x{}+{}+{}'.format(width, height, x, y))  

def load_generation(window, GenerationManager):

    def load(popup, name):
        GenerationManager.load_generation(name)
        popup.destroy()

    # Create new pop-up window
    popup = tk.Toplevel(window)
    popup.title("Load a Generation")

    # Create label and text entry widgets for name input
    label = tk.Label(popup, text="What is the name of the generation you would like to load?")
    label.grid(row=0, column=0)

    name = tk.Entry(popup)
    name.grid(row=1, column=0)

    name = name.get()

    # Create OK button that retrieves name and closes window
    ok_button = tk.Button(popup, text="Load", command=lambda popup=popup, name=name: load(popup, name))
    ok_button.grid(row=2, column=0)  

    # Center the popup window in the main window
    popup.update_idletasks()
    width = popup.winfo_width()
    height = popup.winfo_height()
    x = (window.winfo_screenwidth() // 2) - (width // 2)
    y = (window.winfo_screenheight() // 2) - (height // 2)
    popup.geometry('{}x{}+{}+{}'.format(width, height, x, y))  