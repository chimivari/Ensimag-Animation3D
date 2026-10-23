import matplotlib.pyplot as plt
import numpy as np

frames = np.loadtxt("output2.txt")

fig = plt.figure()
ax = fig.add_subplot(projection="3d")

out = frames[1, :].reshape((23, 3))
X, Y, Z = out[:, 0], out[:, 1], out[:, 2]
ax.scatter(X, Y, Z)
ax.set_xlabel("X Label")
ax.set_ylabel("Y Label")
ax.set_zlabel("Z Label")
plt.show()
