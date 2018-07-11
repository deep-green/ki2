let addon = require('../native');

const app = require('express')();
const http = require('http').Server(app);
const io = require('socket.io')(http);

io.on('connect', socket => {
    socket.on('receive', data => {
        let msg = {
            ID_game: data.ID_game,
            FEN: addon.getMove(data.FEN)
        };

        socket.emit('makeMove', msg);
    });
});

http.listen(5000);
