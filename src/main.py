import random

import waitress
from flask import Flask, redirect

from urls import urls

app = Flask("Fun Stuff")


@app.route("/", methods=["GET"])
def random_url():
    return redirect(random.choice(list(urls.values())))


@app.route("/<url>", methods=["GET"])
def chosen_url(url):
    try:
        return redirect(urls[url])
    except KeyError:
        return "<!DOCTYPE html><html><body><h1>ERROR 404</h1><h2>The URL doesn't exist</h2></body></html>", 404


@app.route("/all_urls", methods=["GET"])
def all_urls():
    return urls


waitress.serve(app, host="0.0.0.0", port=5001)
