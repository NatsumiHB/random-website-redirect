import random

import waitress
from flask import Flask, redirect, request

from urls import urls

app = Flask("Fun Stuff")


@app.route("/", methods=["GET"])
def random_url():
    return redirect(random.choice(list(urls.values())))


@app.route("/<url>", methods=["GET"])
def chosen_url(url):
    if url in urls:
        return redirect(urls[url])
    else:
        return 404


waitress.serve(app, host="0.0.0.0", port=5001)
