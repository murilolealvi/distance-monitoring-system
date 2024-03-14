import matplotlib.pyplot as plt
import numpy as np

x = np.arange(1, 10)
y = np.arange(1, 10)

plt.scatter(x, y) #pontos
plt.show()

x1 = np.arange(0, 1000)
plt.plot(x1, x1**2)
plt.show()


x2 = np.arange(-1000, 1000)
plt.plot(x2, x2**2)
plt.show()