import numpy as np

x = np.arange(9).reshape(3, 3)

print(x @ x)

# print(x.swapaxes(0, 1))
# print(x.T)