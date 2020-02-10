const {remote, ipcRenderer} = require('electron');
const $ = require('jquery');

function fullscreen(){
    var window = remote.BrowserWindow.getFocusedWindow();
    window.maximize();
    var audio = new Audio('../AUDIO/START/startup.wav');
    audio.play();
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
    }else{
        window.maximize();
    }
}

function exitWindow () {
    var window = remote.BrowserWindow.getFocusedWindow();
    window.close();
}

$(function () {
    $('[data-toggle="tooltip"]').tooltip()
  })