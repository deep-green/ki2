let addon = require('../native');

const app = require('express')();
const http = require('http').Server(app);
const io = require('socket.io')(http);

io.on('connect', socket => {
    socket.on('getMove', msg => {
        io.emit('getMove', addon.getMove(msg.data));
    });
});

http.listen(5000);