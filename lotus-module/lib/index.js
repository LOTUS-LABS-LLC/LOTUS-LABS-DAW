var mod = require('../native');
class OutFn {
constructor(){
}
start_as_api = mod.start_as_api_fn;
main_headless = mod.main_headless_fn;
}
module.exports = new OutFn();