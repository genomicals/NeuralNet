import tools
#import numpy as np //tools-0.1.0

#to activate venv virtual environment run "source c:/Users/joceb/OneDrive/Documents/WSU/cpts_434/neuralnet/venv/Scripts/activate" in project dir
#go to first tools dir and run "maturin develop --release"
#run "python main.py" in second neuralnet dir


if __name__ == "__main__":
    print(tools.sum_as_string(1, 3))

    test_obj = tools.GameManager()
    test_obj1 = tools.GenerationManager()
    return_num = test_obj.make_move(5, 2)
    print(return_num)

    x: list[str] = []

    print(f"Running internal tests: {tools.run_tests()}")


    #network = nn.NeuralNetwork()
    #test_matrix = np.array([[0, 1, 2, 3], [2, 3, 4]], dtype = object)

    #test_dict = {1: "hi", 2: "test"}
    #tools.test_mod.test_func(test_dict)
    #print(tools.test_func(test_dict))

    #test_nn = tools.NeuralNetwork(external = 3.3)
    #test_nn.update_internal(1.0)
    #test_nn.update_internal(2)
    #print(test_nn.calculate())
    #print(test_nn.external)
    #print(test_nn.internal)

    #test_nn = tools.AI()

