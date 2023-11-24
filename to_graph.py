import re
import sys
import matplotlib.pyplot as plt 


path = sys.argv[1]
file = open(path)
lines = file.readlines()

def to_mil(in_str):
    number_part = re.match("[\d\.]+",in_str)[0]
    if re.search("ns",in_str):
        number_part = float(re.match("[\d\.]+",in_str)[0])/1_000_000
    elif re.search("µs",in_str):
        number_part = float(re.match("[\d\.]+",in_str)[0])/1000
    elif re.search("ms",in_str):
        number_part = float(re.match("[\d\.]+",in_str)[0])
    elif re.search("s",in_str):
        number_part = float(re.match("[\d\.]+",in_str)[0])*1000
    return number_part


header = lines[0].split(", ")
data = {}
for i in header:
    data[i] = []

lines = lines[1:]
for i in range(0,len(lines)):
    lines[i] = lines[i].split(", ")
    data[header[0]].append(float(lines[i][0]))
    for j in range(1, len(header)):
        data[header[j]].append(to_mil(lines[i][j]))


if "-s" in sys.argv:
    path = path +".s"
    for i in header[1:]:
        # div = min(data[i])
        div = data[i][0]
        for j in range(0,len(data[i])):
            data[i][j] = float(data[i][j])/div

for i in header[1:]:
    plt.plot(data[header[0]],data[i])

plt.legend(header[1:])
plt.xlabel(header[0])
plt.ylabel("Time")
plt.savefig(path+".png")

