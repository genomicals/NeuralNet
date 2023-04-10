import tkinter as tk

class CheckersGame:
    def __init__(self):
        self.window = tk.Tk()
        self.window.title("Checkers Game")

        # Create checkers board GUI
        self.board_size = 10
        self.board_squares = [[None for _ in range(self.board_size)] for _ in range(self.board_size)]
        for row in range(self.board_size):
            for col in range(self.board_size):
                color = "black" if (row + col) % 2 == 0 else "white"
                square = tk.Canvas(self.window, width=50, height=50, bg=color, highlightthickness=0)
                square.grid(row=row, column=col)
                self.board_squares[row][col] = square
        

        # Add row labels
        for row in range(self.board_size):
            label = tk.Label(self.window, text=str(row), font=("Arial", 16))
            label.grid(row=row, column=0, sticky="nesw")

        # Add column labels
        for col in range(self.board_size-2):
            label = tk.Label(self.window, text=chr(ord("A")+col), font=("Arial", 16))
            label.grid(row=0, column=col+1, sticky="nesw")



        # Add row labels
        for row in range(self.board_size-1):
            label = tk.Label(self.window, text=str(row+1), font=("Arial", 16))
            label.grid(row=row+1, column=9, sticky="nesw")

        # Add column labels
        for col in range(self.board_size-2):
            label = tk.Label(self.window, text=chr(ord("A")+col), font=("Arial", 16))
            label.grid(row=9, column=col+1, sticky="nesw")


        
        corner00 = tk.Label(self.window, text=str(" "), font=("Arial", 16))
        corner00.grid(row=0, column=0, sticky="nesw")

        corner99 = tk.Label(self.window, text=str(" "), font=("Arial", 16))
        corner99.grid(row=9, column=9, sticky="nesw")

        corner09 = tk.Label(self.window, text=str(" "), font=("Arial", 16))
        corner09.grid(row=0, column=9, sticky="nesw")

        corner90 = tk.Label(self.window, text=str(" "), font=("Arial", 16))
        corner90.grid(row=9, column=0, sticky="nesw")

         # Create menu bar
        menubar = tk.Menu(self.window)
        self.window.config(menu=menubar)

        # Add "File" menu
        Game = tk.Menu(menubar, tearoff=0)
        menubar.add_cascade(label="Game", menu=Game)
        Game.add_command(label="New Game", command=self.new_game)
        Game.add_separator()
        Game.add_command(label="Exit", command=self.window.quit)

        Generations = tk.Menu(menubar, tearoff=0)
        menubar.add_cascade(label="Generations", menu=Generations)
        Generations.add_command(label="Train Generations", command=self.train_generations)
        Generations.add_separator()
        Generations.add_command(label="Load a Generation")
        Generations.add_command(label="Save Current Generation")

        # Center the popup window in the main window
        self.window.update_idletasks()
        width = 500
        height = 500
        x = (self.window.winfo_screenwidth() // 2) - (width // 2)
        y = (self.window.winfo_screenheight() // 2) - (height // 2)
        self.window.geometry('{}x{}+{}+{}'.format(width, height, x, y))  


    def train_generations(self):
         # Create new pop-up window
        popup = tk.Toplevel(self.window)
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
            popup2 = tk.Toplevel(self.window)
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
        x = (self.window.winfo_screenwidth() // 2) - (width // 2)
        y = (self.window.winfo_screenheight() // 2) - (height // 2)
        popup.geometry('{}x{}+{}+{}'.format(width, height, x, y))  

    

    def new_game(self):
        print("Starting a new game...")


    def run(self):
        self.window.mainloop()

if __name__ == "__main__":
    game = CheckersGame()
    game.run()