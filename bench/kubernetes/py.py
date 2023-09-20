import re
import time
from dateutil import parser
import csv

myfile = "MESSAGES.json"


r_expression = re.compile(r'.*===K3S-CUSTOM-998===.*reason=\\\"(?P<event>\w+)\\\".*\sobject=\\\"(?P<object>.*)\\\"\stime=\\\"(?P<timestamp>.*?)\\\"')
#r_expression = re.compile('.*===K3S-CUSTOM-998===.*reason=\\\"(?P<event>\w+)')
with open(myfile) as f:
    f = f.readlines()

with open("parsed.log", "w") as newf:
  for line in f:
    contents = r_expression.search(line)
    if contents is not None:
      event = contents.group('event')
      namespace, pod_name = contents.group('object').split('/')
      timestamp = contents.group('timestamp')
      timestamp = parser.parse(timestamp)
      if event == 'SuccessfulCreate' or event == 'Started':
        newf.write("{},{},{}\n".format(timestamp, pod_name, event))

with open("parsed.log", "r") as newf:
  lines = newf.readlines()


lines.sort(key = lambda l : l.split(',')[0])

with open("parsed2.log", "w+") as newf:
    newf.write("".join(lines))


f = open('parsed2.log',mode='r', newline='')
reader = csv.reader(f, delimiter=',')

with open("kube-init.csv", "w+") as newf:
    newf.write("init_time,test_name\n")
    pre_line = next(reader)
    while(True):
        try:
            cur_line = next(reader)
            if pre_line[2] == "SuccessfulCreate" and cur_line[2] == "Started" and pre_line[1] == cur_line[1]:
                timediff = parser.parse(cur_line[0]) - parser.parse(pre_line[0])
                newf.write("{},{}\n".format(str(timediff).rsplit(':')[-1][1:], pre_line[1].rsplit('-',2)[0]))


            pre_line = cur_line
        except Exception as error:
            # handle the exception
            print("An exception occurred:", error,  type(error).__name__)
            break
f.close()
