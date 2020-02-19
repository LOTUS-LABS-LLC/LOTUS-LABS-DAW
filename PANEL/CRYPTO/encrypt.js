var CryptoJS = require('crypto-js');
var uuid = require('uuid');

function randomNum(min, max) {
    return Math.random() * (max - min) + min;
}

module.exports = function encrypt(val) {
    var id = uuid.v4();
    var layer1key = [];
    var layer2key = [];
    var layer3key = [];
    var layer4key = [];
    var layer5key = [];
    var layer1int = [];
    var layer2int = [];
    var layer3int = [];
    var layer4int = [];
    var layer5int = [];
    for(i = 0; i<= randomNum(15,20); i++){
        p = parseInt(randomNum(1,10));
        layer1int.push(p);
        layer1key.push(id.toString().substring(p,p+1));
    }
    var layer1msg = CryptoJS.AES.encrypt(val, layer1key.join(''));

    for(i = 0; i<= randomNum(15,20); i++){
        p = parseInt(randomNum(1,10));
        layer2int.push(p);
        layer2key.push(id.toString().substring(p,p+1));
    }
    var layer2msg = CryptoJS.DES.encrypt(layer1msg.toString(), layer2key.join(''));

    for(i = 0; i<= randomNum(15,20); i++){
        p = parseInt(randomNum(1,10));
        layer3int.push(p);
        layer3key.push(id.toString().substring(p,p+1));
    }
    var layer3msg = CryptoJS.RC4.encrypt(layer2msg.toString(), layer3key.join(''));

    for(i = 0; i<= randomNum(15,20); i++){
        p = parseInt(randomNum(1,10));
        layer4int.push(p);
        layer4key.push(id.toString().substring(p,p+1));
    }
    var layer4msg = CryptoJS.Rabbit.encrypt(layer3msg.toString(), layer4key.join(''));

    for(i = 0; i<= randomNum(15,20); i++){
        p = parseInt(randomNum(1,10));
        layer5int.push(p);
        layer5key.push(id.toString().substring(p,p+1));
    }
    var layer5msg = CryptoJS.TripleDES.encrypt(layer4msg.toString(), layer5key.join(''));

    




    return layer5msg+','+id+'+'+layer1int.join('')+'+'+layer2int.join('')+'+'+layer3int.join('')+'+'+layer4int.join('')+'+'+layer5int.join('');
}