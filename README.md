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

## Python Unittests & Travis CI
> Wichtig: Dateien mit Tests müssen im Ordner tests/ liegen und mit "test" in Minuskeln beginnen (z.B.: testExample.py oder test_MyTests.py)

### Beispiel Unittest in Python
```python
class MyTestCase(unittest.TestCase):
    def test_something(self):
        self.assertEqual(True, True)
```
> Funktionen in Testklassen müssen mit "test_" beginnen um als Tests erkannt zu werden.
