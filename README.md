# Survive

Placeholder name of a castle defence game for some experimentation with Bevy and WASM.

Inspired by [Defend your Castle](https://en.wikipedia.org/wiki/Defend_Your_Castle) which can be played at the internet archive
[here](https://archive.org/details/defendyourcastle_flash)

### Run

Although this can be cross compiled to many platforms, the primary platform is WASM.

The blow command will create a webserver so you can run the game.

```
 cargo run --target wasm32-unknown-unknown                                   
```

Note: You may need other bevy specific prerequisites for your OS to build this.

Check out the [Bevy setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/) for OS specific instructions