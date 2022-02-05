## Custom-built expression parser and loader for complex numbers in rust

parser infrastructure is from https://github.com/rodolf0/tox/

| `graphType` | Description      |
| ----------- | ---------------- |
| `0`         | Re-Im, Color     |
| `1`         | Re-Im, BW        |
| `2`         | Im-Re, Color     |
| `3`         | Im-Re, BW        |
| `4`         | Mod-Arg, (Color) |

```
wasm-pack build --target web
```

```js
window.init = init;
window.evaluate = evaluate;
```
