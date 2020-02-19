var crypto = require('crypto-js');
var uuid = require('uuid');

module.exports = function decrypt(val) {
    var value = (val + '').toString().split(',');
    var keys = value[1].split('+');
    var id = keys[0];
    var key1 = [];
    var key2 = [];
    var key3 = [];
    var key4 = [];
    var key5 = [];





    return keys;


    //ORDER OF DECRYPTION TOP TO BOTTOM
    crypto.TripleDES.decrypt();
    crypto.Rabbit.decrypt();
    crypto.RC4.decrypt();
    crypto.DES.decrypt();
    crypto.AES.decrypt();
}