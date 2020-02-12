const {remote} = require('electron');
const $ = require('jquery');

function fullscreen(){
    var window = remote.getCurrentWindow();
    window.maximize();
    window.focus();
    window.setMinimumSize(1600, 800);
    var audio = new Audio('../AUDIO/START/startup.wav');
    audio.play();
}

// Box width
var bw = 400;
// Box height
var bh = 400;
// Padding
var p = 0;

var canvas = document.getElementById("tracklist");
var context = canvas.getContext("2d");
function drawBoard(){
    for (var x = 0; x <= bw; x += 20) {
        context.moveTo(0.5 + x + p, p);
        context.lineTo(0.5 + x + p, bh + p);
    }

    for (var x = 0; x <= bh; x += 60) {
        context.moveTo(p, 0.5 + x + p);
        context.lineTo(bw + p, 0.5 + x + p);
    }
    context.strokeStyle = "white";
    context.stroke();
}


function mixerrack(){
    for(i = 1; i <= 200; i++){
        $("#mixerrack").append('<li class="list-group-item mixi bg-dark"><kbd>' + i + '</kbd></li>');
        console.log(i);
    }
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



  function searchF() {
    // Declare variables
    var input, filter, ul, li, a, i, txtValue;
    input = document.getElementById('friendSearch');
    filter = input.value.toUpperCase();
    ul = document.getElementById("flist");
    li = ul.getElementsByTagName('a');
  
    // Loop through all list items, and hide those who don't match the search query
    for (i = 0; i < li.length; i++) {
      a = li[i].getElementsByTagName("p")[0];
      txtValue = a.textContent || a.innerText;
      if (txtValue.toUpperCase().indexOf(filter) > -1) {
        li[i].style.display = "";
      } else {
        li[i].style.display = "none";
      }
    }
  }
  