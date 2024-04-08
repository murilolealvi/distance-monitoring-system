import sqlite3 as sql
from os import path

ROOT = path.dirname(path.relpath(__file__))

def create_post(name, content):
    conn = sql.connect(path.join(ROOT,'database.db'))
    cursor = conn.cursor()
    cursor.execute('INSERT INTO posts(name, content) VALUE(?, ?)', (name, content))
    conn.commit()
    conn.close()

def get_posts():
    conn = sql.connect(path.join(ROOT, 'database.db'))
    cursor = conn.cursor()
    cursor.execute('SELECT * FROM POSTS')
    posts = cursor.fetchall()
    return posts