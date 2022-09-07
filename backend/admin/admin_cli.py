from dotenv import dotenv_values
import requests
import os
from os import listdir
from os.path import isfile, join

config = {
    **dotenv_values("../crates/api/.env"),
}

file_path = os.path.dirname(os.path.realpath(__file__))
sts_path = file_path+"/statements"

def request(data):
    return requests.post(config["MY_URL"]+"/api/admin/update-problem-statement", json=data, headers={ 'X-Admin-Key': config["ADMIN_KEY"] })

def update_statements():
    files = [f for f in listdir(sts_path) if isfile(join(sts_path, f))]

    for ff in files:
        fh = open(sts_path+"/"+ff, "r", encoding="utf-8")
        cont = fh.read()
        fh.close()
        cname = ff.split('.md')[0]
        request({ "problem": cname, "version": "1.0", "statement": cont })

update_statements()