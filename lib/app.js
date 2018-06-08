let addon = require('../native');

const app = require('express')();
const http = require('http').Server(app);
const io = require('socket.io')(http);

const Chess = require('chess.js').Chess;

let clients = {};

io.on('connect', socket => {
    socket.on("register", data => {
        clients[data.nickname] = socket.id;
    });

    socket.on('receive', data => {
        let chess = new Chess(data.FEN);
        let moves = chess.moves();
        chess.move(moves[Math.floor(Math.random() * moves.length)]);
        let move = chess.history({verbose: true});

        socket.to(clients[data.to]).emit('makeMove', move[0].from + move[0].to);
    });
});

http.listen(5000);
