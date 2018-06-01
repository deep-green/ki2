let addon = require('../native');

const app = require('express')();
const http = require('http').Server(app);
const io = require('socket.io')(http);

io.on('connection', socket => {
    socket.on('recieve', data => {
        io.emit('makeMove', addon.getMove(data.FEN));
    });
});

http.listen(5000);