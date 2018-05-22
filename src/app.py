from flask import Flask
import socketio
import eventlet.wsgi

from src.Evaluator import Evaluator

sio = socketio.Server(logger=True, async_mode=None)
app = Flask(__name__)
app.wsgi_app = socketio.Middleware(sio, app.wsgi_app)

@sio.on('getMove', namespace='/')
def connect(sid, fen):
    sio.emit('getMove', Evaluator(fen).getMove(), room=sid, namespace='/')


if __name__ == '__main__':
    eventlet.wsgi.server(eventlet.listen(('', 5000)), app)
