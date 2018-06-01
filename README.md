# Künstliche Intelligenz - Gruppe 2   [![Build Status](https://travis-ci.com/deep-green/ki2.svg?branch=master)](https://travis-ci.com/deep-green/ki2)

## Schnittstelle
__URL:__ ec2-54-93-171-91.eu-central-1.compute.amazonaws.com  
__Port:__ 5000

### Beispiel - Backend -> KI (node.js)
```js
let socket = require('socket.io-client')('http://ec2-54-93-171-91.eu-central-1.compute.amazonaws.com:5000');

socket.emit('receive', { FEN: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1' });

socket.on('makeMove', function(msg) {
    console.log(msg);
});
```

### Beispiel - Client -> Backend (node.js)
```js
let socket = require('socket.io-client')('http://ec2-54-93-171-91.eu-central-1.compute.amazonaws.com:4999');

socket.on('connect', function() {
    socket.emit('makeMove', {
        FEN: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ID_game: 2,
    });
});

socket.on('receive', data => {
    console.log(data);
});

socket.on('reject', () => {
    console.log("reject");
});
```
