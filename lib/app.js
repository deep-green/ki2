let addon = require('../native');

const app = require('express')();
const http = require('http').Server(app);
const io = require('socket.io')(http);

const Chess = require('chess.js').Chess;


io.on('connection', socket => {
    socket.on('receive', (id, data) => {
        let chess = new Chess(data.FEN);
        let moves = chess.moves();
        chess.move(moves[Math.floor(Math.random() * moves.length)]);
        let move = chess.history({verbose: true});

        socket.to(id).emit('makeMove', move[0].from + move[0].to);
    });

    socket.on('disconnect', (id) => {
        socket.to(id).emit('disconnect', {});
    });
});

http.listen(5000);
