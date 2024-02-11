#!/usr/bin/python
import json

f = open("./dictionary.json","r")
j = json.loads(f.read())
dedup = list(map(lambda f: f.lower(),dict.fromkeys(j)))
dedup.sort()
f.close()
f = open("./dictionary.json","w")
f.write(json.dumps(dedup).replace("\'","\"").replace(",",",\n"))
f.close()