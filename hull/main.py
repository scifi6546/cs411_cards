import matplotlib.pyplot as plt
import json
file = open("points.json")
points = json.load(file)
print(points)
plt.scatter(points[0],points[1])
file = open("hull.json");
hull_data = json.load(file)
plt.scatter(hull_data[0],hull_data[1])
plt.show()
