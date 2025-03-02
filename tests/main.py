import os
import requests
import subprocess
import json
import time
import socket
import sqlite3
import re

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

def expect_pattern(t, n, a, b):
    global stop
    if stop:
        print(f"{t}: {n} is {a}")
    elif re.search(b, a) == None:
        print(f"{t}: Expected {n} to contain {b}, but it is {a}")
        stop = True

def test_register():
    register_url = f"http://{addr}:{port}/auth/register"
    register_data = {
        "email": "user.mail.1@user.io",
        "plaintext_password": "user_1_passwd",
        "first_name": "user",
        "last_name": "number1",
        "gender": "m"
    }
    register_request = requests.post(register_url, json=register_data)
    status = register_request.status_code
    response = json.loads(register_request.text)
    expect("register", "status", status, 201)
    expect("register", "account_id", response, {"account_id": 1})

    global stop
    if stop:
        return

    conn = sqlite3.connect("db.sqlite")
    cur = conn.cursor()

    cur.execute("SELECT verification_token FROM verifications WHERE user_id = ?", str(response["account_id"]))
    rows = cur.fetchall()
    ver_token = rows[0][0]

    verify_url = f"http://{addr}:{port}/auth/verify/{ver_token}"
    verify_response = requests.get(verify_url)

    expect_pattern("register", "verify_response", verify_response.text, ".*successful.*")

def test_register_resend():
    register_url = f"http://{addr}:{port}/auth/register"
    register_data = {
        "email": "user.mail.1@user.io",
        "plaintext_password": "user_1_passwd",
        "first_name": "user",
        "last_name": "number1",
        "gender": "m"
    }
    register_request = requests.post(register_url, json=register_data)
    status = register_request.status_code
    response = json.loads(register_request.text)
    expect("register", "status", status, 201)
    expect("register", "account_id", response, {"account_id": 1})

    global stop
    if stop:
        return

    resend_url = f"http://{addr}:{port}/auth/resend_verification/1"
    resend_response = requests.post(resend_url)

    expect("register", "verify_response", resend_response.status_code, 400)

def test_login():
    test_register()

    global stop
    if stop:
        return
    
    login_url = f"http://{addr}:{port}/auth/login"
    login_data = {
        "email": "user.mail.1@user.io",
        "plaintext_password": "user_1_passwd",
    }
    login_request = requests.post(login_url, json=login_data)
    status = login_request.status_code
    expect("register", "status", status, 200)
    expect_pattern("register", "account_id", login_request.text, "{\"jwt\":.*")

set_env()

tests = [
    (test_register, "Register and verify"),
    (test_register_resend, "Register resend"),
    (test_login, "Login")
]

for (test, i) in tests:
    instance = subprocess.Popen(["cargo", "run"], 
                                stdout=subprocess.DEVNULL, 
                                stderr=subprocess.DEVNULL, 
                                env=os.environ)
    sock = socket.socket()
    while sock.connect_ex((addr, port)) != 0:
        time.sleep(0.1)
    test()
    instance.kill()
    instance.wait()
    if stop:
        print(f"{i}: Error")
        break
    else:
        print(f"{i}: Ok")

