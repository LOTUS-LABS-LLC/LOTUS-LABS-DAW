var mod = require('../native');
class OutFn {
start_as_api = mod.start_as_api;
main_headless = mod.main_headless;
}
module.exports = new OutFn();