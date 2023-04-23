# NeuralNet - Checkers AI
Repository for our CptS 434 Neural Networks course. An AI that plays checkers and evolves through a genetic algorithm.

## Setup
### Prerequisites
* Python 3.10 or later
* Rust + PyO3 (maturin)

### Building Rust Tools
Running this in the "tools/" directory will install the Rust tools into the current Python environment:
```
maturin develop --release
```

### Usage
Run in the "neuralnet/gui/" directory to get started:
```
python main.py
```

Alternatively, you can import the tools module in your own Python script or even the Python REPL. Basic usage looks like so:
```
import tools

gen = tools.GenerationManager()
gen.train_generation()
gen.train_generations(100)
gen.save_generation("test_gen") #save to disk
gen.load_generation("test_gen")

game = gen.create_game()
result = game.start_game(true) #start the game
#result: 0 = next move, 1 = player won, 2 = ai won
game.make_move(2, 1) #move the piece at tile 2 northeast
```

## License
This work is dual-licensed under GPL Version 2 and GPL Version 3. You may choose between these licenses on a case-by-case basis.

