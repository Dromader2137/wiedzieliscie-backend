import os
import requests
import subprocess
import json
import time
import socket

addr = ""
port = 0

def set_env():
    env_file = open(".env")
    env_txt = env_file.read().strip()
    env_vars = env_txt.split("\n")
    for env_var in env_vars:
        [var_name, var_value] = env_var.strip().split(" ")
        print(f"Setting {var_name} to {var_value}")
        os.environ[var_name] = var_value
        global env
    global addr, port
    addr = os.environ["WIEDZIELISCIE_BACKEND_URL"].split(":")[0]
    port = int(os.environ["WIEDZIELISCIE_BACKEND_URL"].split(":")[1])

stop = False
def expect(t, n, a, b):
    global stop
    if stop:
        print(f"{t}: {n} is {a}")
    elif a != b:
        print(f"{t}: Expected {n} to be {b}, but it is {a}")
        stop = True

def test_register():
    url = f"http://{addr}:{port}/auth/register"
    data = {
        "email": "user.mail.1@user.io",
        "plaintext_password": "user_1_passwd",
        "first_name": "user",
        "last_name": "number1",
        "gender": "m"
    }
    register_request = requests.post(url, json=data)
    status = register_request.status_code
    response = json.loads(register_request.text)
    expect("register", "status", status, 201)
    expect("register", "account_id", response, {"account_id": 1})
    
    pass

set_env()

tests = [
    test_register
]

for test in tests:
    instance = subprocess.Popen(["cargo", "run"], 
                                stdout=subprocess.DEVNULL, 
                                stderr=subprocess.DEVNULL, 
                                env=os.environ)
    sock = socket.socket()
    while sock.connect_ex((addr, port)) != 0:
        time.sleep(0.1)
    test()
    instance.kill()
    if stop:
        break
    else:
        print("ok")

