# Künstliche Intelligenz - Gruppe 2   [![Build Status](https://travis-ci.com/deep-green/ki2.svg?branch=master)](https://travis-ci.com/deep-green/ki2)

## Schnittstelle
__URL:__ ec2-54-93-171-91.eu-central-1.compute.amazonaws.com  
__Port:__ 5000

### Beispiel (node.js)
```js
let socket = require('socket.io-client')('http://ec2-54-93-171-91.eu-central-1.compute.amazonaws.com:5000');

socket.emit('receive', { FEN: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1' });

socket.on('makeMove', function(msg) {
    console.log(msg);
});
```
