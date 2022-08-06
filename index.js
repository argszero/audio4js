const { js_new, js_play, js_stop, js_set_volume } = require("./index.node");

class Player {
    constructor() {
        this.player = js_new();
    }
    play(path, skipSeconds) {
        return js_play.call(this.player, path, skipSeconds);
    }
    stop() {
        return js_stop.call(this.player);
    }
    setVolume(volume) {
        return js_set_volume.call(this.player, volume);
    }

}
module.exports = Player;