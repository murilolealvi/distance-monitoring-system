import numpy as np

A = np.array([[1, 1], [1.5, 4.0]])
B = np.array([2200, 5050])

x = np.linalg.inv(A).dot(B) #slow
print(x)

x = np.linalg.solve(A, B) #faster
print(x)