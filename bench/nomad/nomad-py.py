import re
import time
from dateutil import parser
import json
import csv

myfile = "nomad-all.json"

data = []
with open(myfile) as f:
    for line in f:
      data.append(json.loads(line))

with open("parsed-nomad.log", "w") as newf:
    for obj in data:
        event = obj['type']
        task = obj['task']
        timestamp = obj['@timestamp']
        timestamp = parser.parse(timestamp)
        if event == 'Started' or event == 'Task Setup':
            newf.write("{},{},{}\n".format(timestamp, task, event))

with open("parsed-nomad.log", "r") as newf:
  lines = newf.readlines()

lines.sort(key = lambda l : l.split(',')[0])

with open("parsed-nomad2.log", "w+") as newf:
    newf.write("".join(lines))

f = open('parsed-nomad2.log',mode='r', newline='')
reader = csv.reader(f, delimiter=',')

with open("nomad-init.csv", "w+") as newf:
    newf.write("init_time,test_name\n")
    pre_line = next(reader)
    while(True):
        try:
            cur_line = next(reader)
            if pre_line[2] == "Task Setup" and cur_line[2] == "Started" and pre_line[1] == cur_line[1]:
                timediff = parser.parse(cur_line[0]) - parser.parse(pre_line[0])
                newf.write("{},{}\n".format(str(timediff).rsplit(':')[-1][1:], pre_line[1]))


            pre_line = cur_line
        except Exception as error:
            # handle the exception
            print("An exception occurred:", error,  type(error).__name__)
            break
f.close()
