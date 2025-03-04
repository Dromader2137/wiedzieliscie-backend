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
    env = {}
    for env_var in env_vars:
        [var_name, var_value] = env_var.strip().split(" ")
        env[var_name] = var_value
    global addr, port
    addr = env["WIEDZIELISCIE_BACKEND_URL"].split(":")[0]
    port = int(env["WIEDZIELISCIE_BACKEND_URL"].split(":")[1])

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

    expect("resend", "status", resend_response.status_code, 400)

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
    expect("login", "status", status, 200)
    expect_pattern("login", "jwt", login_request.text, "{\"jwt\":.*")

    response = json.loads(login_request.text)
    return response["jwt"]

def test_logout():
    jwt = test_login()
    
    if stop:
        return

    logout_url = f"http://{addr}:{port}/auth/logout"
    logout_data = {
        "jwt": jwt,
    }
    logout_request = requests.post(logout_url, json=logout_data)
    status = logout_request.status_code
    expect("logout", "status", status, 200)

def test_retrieve_user():
    jwt = test_login()

    if stop:
        return

    retrieve_url = f"http://{addr}:{port}/auth/retrieve_user"
    retrieve_data = {
        "jwt": jwt,
    }
    retrieve_response = requests.post(retrieve_url, json=retrieve_data)
    response = json.loads(retrieve_response.text)
    user_data = {
        "account_id": 1,
        "email": "user.mail.1@user.io",
        "first_name": "user",
        "last_name": "number1",
        "gender": "m",
    }
    expect("retrieve", "response", response, user_data)

def test_password_reset():
    test_register()

    if stop:
        return

    reset_url = f"http://{addr}:{port}/auth/password_reset"
    reset_data = {
        "email": "user.mail.1@user.io",
        "plaintext_password": "new_pass"
    }
    reset_response = requests.post(reset_url, json=reset_data)
    expect("reset", "status", reset_response.status_code, 200)
    
    conn = sqlite3.connect("db.sqlite")
    cur = conn.cursor()

    cur.execute("SELECT reset_token FROM password_resets WHERE user_id = ?", "1")
    rows = cur.fetchall()
    res_token = rows[0][0]

    verify_url = f"http://{addr}:{port}/auth/password_reset/verify/{res_token}"
    verify_response = requests.get(verify_url)

    expect_pattern("reset", "verify_response", verify_response.text, ".*successful.*")

def test_retrieve_user_email():
    test_register()

    if stop:
        return

    retrieve_url = f"http://{addr}:{port}/user/retrieve/email"
    retrieve_data = {
        "email": "user.mail.1@user.io",
    }
    retrieve_response = requests.post(retrieve_url, json=retrieve_data)
    response = json.loads(retrieve_response.text)
    user_data = {
        "account_id": 1,
        "email": "user.mail.1@user.io",
        "first_name": "user",
        "last_name": "number1",
        "gender": "m",
        "points": 0
    }
    expect("retrieve", "response", response, user_data)

def test_retrieve_user_id():
    test_register()

    if stop:
        return

    retrieve_url = f"http://{addr}:{port}/user/retrieve/id"
    retrieve_data = {
        "account_id": 1
    }
    retrieve_response = requests.post(retrieve_url, json=retrieve_data)
    response = json.loads(retrieve_response.text)
    user_data = {
        "account_id": 1,
        "email": "user.mail.1@user.io",
        "first_name": "user",
        "last_name": "number1",
        "gender": "m",
        "points": 0
    }
    expect("retrieve", "response", response, user_data)

def test_retrieve_user_name():
    test_register()

    if stop:
        return

    retrieve_url = f"http://{addr}:{port}/user/retrieve/name"
    retrieve_data = {
        "first_name": "user",
        "last_name": "number1"
    }
    retrieve_response = requests.post(retrieve_url, json=retrieve_data)
    response = json.loads(retrieve_response.text)
    user_data = {
        "account_id": 1,
        "email": "user.mail.1@user.io",
        "first_name": "user",
        "last_name": "number1",
        "gender": "m",
        "points": 0
    }
    expect("retrieve", "response", response, user_data)

def test_retrieve_user_count():
    test_register()

    if stop:
        return

    retrieve_url = f"http://{addr}:{port}/user/retrieve/count"
    retrieve_response = requests.get(retrieve_url)
    response = json.loads(retrieve_response.text)
    user_data = 1
    expect("retrieve", "response", response, user_data)

set_env()

tests = [
    (test_register, "Register and verify"),
    (test_register_resend, "Register resend"),
    (test_login, "Login"),
    (test_logout, "Logout"),
    (test_retrieve_user, "Retrieve user"),
    (test_password_reset, "Password reset"),
    (test_retrieve_user_email, "Retrieve user email"),
    (test_retrieve_user_name, "Retrieve user name"),
    (test_retrieve_user_id, "Retrieve user id"),
    (test_retrieve_user_count, "Retrieve user count"),
]

for (test, i) in tests:
    instance = subprocess.Popen(["cargo", "run"], 
                                stdout=subprocess.DEVNULL, 
                                stderr=subprocess.DEVNULL)
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

