# Künstliche Intelligenz - Gruppe 2   [![Build Status](https://travis-ci.com/deep-green/ki2.svg?branch=master)](https://travis-ci.com/deep-green/ki2)

## Schnittstelle
__URL:__ ec2-54-93-171-91.eu-central-1.compute.amazonaws.com  
__Port:__ 5000

### Beispiel (node.js)
```js
let socket = require('socket.io-client')('http://localhost:5000');
socket.on('connect', function() {
    socket.emit('getMove', { data: 'FEN' });
});

socket.on('getMove', function(msg) {
    console.log(msg);
});
```
