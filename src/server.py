import os
from flask import Flask
from flask import render_template

app = Flask(__name__, template_folder=os.path.abspath("./templates/"), static_folder=os.path.abspath("./static/"))

@app.route("/")
def index():
    return render_template("index.html")

@app.route("/about")
def about():
    return render_template("about.html")

@app.route("/post/<name>")
def test_post(name):
    return render_template("post.html", post=render_template(f"posts/{name}.html"))

@app.route("/posts")
def posts():
    return render_template("posts.html")

@app.route("/robots.txt")
def robots():
    with open("./static/robots.txt", "r") as f:
        data = f.read()
    return data
