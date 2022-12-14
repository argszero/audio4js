
# audio4js

[![npm](https://img.shields.io/npm/v/audio4js.svg)](https://npmjs.org/package/audio4js) [![downloads](https://img.shields.io/npm/dm/audio4js.svg)](https://npmjs.org/package/audio4js)

**audio4js:** an audio player lib for node backend by rodio

This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Usage

1. Install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# or access https://rustup.rs/
```

2. Install [neon](https://neon-bindings.com/)
```
npm install --global neon-cli
# OR
yarn global add neon-cli
```

3. Install `neon-code`
```
npm install audio4js --save
```

## Example

```js
import Player from 'audio4js';

const main = async ()=>{
    const player = new Player();
    player.play('/Users/argszero/sample/mp3/file_example_MP3_1MG.mp3', 5);
    setTimeout(()=>{
        console.log('set volume to 10');
        player.setVolume(10);
    }, 5000);
    setTimeout(()=>{
        console.log('stop');
        player.stop();
    }, 15000);
}
main();
```