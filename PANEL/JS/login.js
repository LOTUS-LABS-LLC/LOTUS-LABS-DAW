const {remote, ipcRenderer} = require('electron');
const $ = require('jquery');

function fullscreen(){
    var window = remote.getCurrentWindow();
    window.maximize();
    window.focus();
    window.setMinimumSize(1600, 800);
}

document.getElementById('min').addEventListener('click', minWindow);
document.getElementById('max').addEventListener('click', maxWindow);
document.getElementById('exit').addEventListener('click', exitWindow);

function minWindow () {
    var window = remote.BrowserWindow.getFocusedWindow();
    window.minimize();
}

function maxWindow () {
    var window = remote.BrowserWindow.getFocusedWindow();
    if(window.isMaximized()){
        window.unmaximize();
        $("#maxmin").removeClass('fa-compress').addClass('fa-expand');
    }else{
        window.maximize();
        $("#maxmin").removeClass('fa-expand').addClass('fa-compress');
    }
}

function exitWindow () {
    var window = remote.BrowserWindow.getFocusedWindow();
    window.close();
}