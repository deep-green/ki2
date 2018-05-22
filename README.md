# Künstliche Intelligenz - Gruppe 2   [![Build Status](https://travis-ci.com/deep-green/ki2.svg?branch=master)](https://travis-ci.com/deep-green/ki2)

## Schnittstelle
### Emit
| Channel | Namespace | Data |
|:--------|:----------|:-----|
| getMove | /         | FEN  |

### Listen
| Channel | Namespace | Data |
|:--------|:----------|:-----|
| getMove | /         | Move |

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
