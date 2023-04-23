def sum_as_string(a: int, b: int) -> str:
    """
    Formats the sum of two numbers as string.
    """


def run_tests() -> bool:
    """
    Runs internal module tests that cannot be tested by Python.
    """


class GenerationManager:
    """
    The main way to train and interact with the AI.
    """

    def train_generations(self, num_generations: int) -> None:
        """
        Trains the AI for a certain number of generations.
        """

    def train_generation(self) -> None:
        """
        Trains the AI for a single generation.
        """

    def create_game(self) -> GameManager:
        """
        Starts a GameManager for the current best AI.
        """

    
    def save_generation(self, name: str) -> None:
        """
        Save the current generation under the given name
        """

    def load_generation(self, name: str) -> None:
        """
        Load the generation with the given name
        """


class GameManager:
    """
    Used to play against a particular AI.
    """

    def new_game(self, player: bool) -> None:
        """
        Initialize a new game.
        """

    def make_move(self, tile: int, action: int) -> int:
        """
        Allows the Player to take their turn, and then has the AI take its turn.
        """

    def peek_board(self) -> list[int]:
        """
        Returns the current board as a list.
        """

