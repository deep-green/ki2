let addon = require('../native');

var app = require('express')();
var http = require('http').Server(app);
var io = require('socket.io')(http);

io.on('connect', function(socket){
    socket.on('getMove', data => {
        io.emit('getMove', addon.getMove());
    });
});

http.listen(5000);