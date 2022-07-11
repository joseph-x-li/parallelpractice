from torch import rand
from torchtyping import TensorType, patch_typeguard
from typeguard import typechecked

patch_typeguard()  # use before @typechecked

@typechecked
def func(x: TensorType["batch"],
         y: TensorType["batch"]) -> TensorType["batch"]:
    return x + y

func(rand(3), rand(3))  # works

import time
time.sleep(100) # waits 100s

func(rand(3), rand(2)) # raises error ONLY AFTER 100s