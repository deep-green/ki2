let addon = require('../native');

const app = require('express')();
const http = require('http').Server(app);
const io = require('socket.io')(http);

io.on('connection', socket => {
    socket.on('receive', data => {
        io.emit('makeMove', addon.getMove(data.FEN));
    });

    socket.on('disconnect', () => {
        io.emit('disconnect', {});
    });
});

http.listen(5000);