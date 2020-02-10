const {ipcRenderer} = require('electron');
let $ = require('jquery');

i=0;
setInterval(() => {
    i++;
    $('.progress-bar').css('width', i+'%').attr('aria-valuenow', i);
    if(i >= 300){
        console.log(i);
        i = 0;
        window.location.href = '../HTML/dashboard.html';
    }
}, 20);
