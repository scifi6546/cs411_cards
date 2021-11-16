import matplotlib.pyplot as plt
import json
def plot_hull(path,title):
    file = open(path)
    result = json.load(file)
    plt.scatter(result["points"]["x"],result["points"]["y"],label="points")
    plt.scatter(result["hull"]["x"],result["hull"]["y"],label="hull")
    plt.title(title)
    plt.savefig("report/"+title+".png")
    plt.show()
plot_hull("10.json","10 points")
plot_hull("100.json","100 points")
plot_hull("1000.json","1000 points")
plot_hull("10000.json","10000 points")
