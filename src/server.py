import os
import json5
from flask import Flask
from flask import render_template, send_file, abort

app = Flask(__name__, template_folder=os.path.abspath("./templates/"), static_folder=os.path.abspath("./static/"))
app.config["TEMPLATES_AUTO_RELOAD"] = True

with open("./src/posts.json", "rb") as f:
    posts = json5.load(f)

@app.route("/")
def index():
    return render_template("index.html")

@app.route("/about")
def about():
    return render_template("about.html")

@app.route("/post/<name>")
def post(name):
    if name in posts:
        return render_template(
                "post.html",
                post=render_template(f"posts/{name}.html"),
                title=posts[name]["title"]
                )
    else:
        abort(404)

@app.route("/robots.txt")
def robots():
    with open("./static/robots.txt", "r") as f:
        data = f.read()
    return data

@app.route("/favicon.ico")
def favicon():
    return send_file("../static/images/favicon.ico")

@app.errorhandler(404)
def page_not_found(e):
    return render_template('404.html'), 404
