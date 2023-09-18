import re
import time
from dateutil import parser

myfile = "MESSAGE.json"


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
      newf.write("{},{},{},{}\n".format(timestamp, namespace, pod_name, event))

with open("parsed.log", "r") as newf:
  lines = newf.readlines()


lines.sort(key = lambda l : l.split(',')[0])

with open("parsed2.log", "w+") as newf:
    newf.write("".join(lines))

