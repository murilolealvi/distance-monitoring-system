from flask import Flask, render_template, request

app = Flask(__name__)


@app.route('/', methods=['GET', 'POST'])
def index():
    #return 'Hello World'
    var = "Game: Guess the number..."
    if request.method == "GET":
        return render_template('index.html', var=var)
    else:
        n = 2
        guess = int(request.form.get("name"))
        if n==guess:
            return '<h1>You win!!</h1>'
        else:
            return '<h1>You lost...</h1>'

@app.route('/sobre')
def about():
    return 'About page'

@app.route('/<string:name>')
def error(name):
    return f'<h1>Error 404: {name} not found</h1>'


if __name__ == "__main__":
    app.run(debug=True)