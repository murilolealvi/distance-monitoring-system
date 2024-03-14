import numpy as np

a = np.array([1, 2, 3, 4])

print(a)
print(a.shape)
print(a.dtype)
print(a.ndim)
print(a.size)
print(a.itemsize)

a += np.array([4]) #sum all the elements
print(a)
a *= 2 #multiply all the elements
print(a)

#dot product with numpy
a = np.array([1, 2, 3])
b = np.array([4, 5, 6])
dot = np.dot(a, b)
print(dot)

sum1 = a*b
dot = (sum1).sum()
print(dot)

dot = a @ b
print(dot)


#multidimensional array

a = np.array([[1, 2], [4, 5]])
print(a.shape)
print(a[0,:])
print(a.T)
print(np.linalg.inv(a)) #inversa da matriz
print(np.linalg.det(a)) #determinante da matriz
print(np.diag(a)) #retorna elementos da diagonal

print(a[a>2])
print(np.where(a>2, a, -1)) #otherwise, insert -1


a = np.array([1, 2, 3, 4])
even = np.argwhere(a%2==0).flatten()
print(a[even])



a = np.arange(1,7)
b = a[np.newaxis, :]
print(b) #two dimensional array

a = np.array([[7, 8, 9, 10, 11, 12], [13, 14, 15, 16 ,17, 18]])
print(a.mean(axis=None))
print(a.std(axis=None))


a = np.array([[1, 2], [3 ,4]])
eigenvalues, eigenvectors = np.linalg.eig(a)
print(eigenvalues)

b = eigenvectors[:,0] * eigenvalues[0]
print(b)
b = a @ eigenvectors[:,0]
print(b)
