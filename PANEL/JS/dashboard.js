const {remote} = require('electron');
const clamp = require('clamp');
const $ = require('jquery');

var canvas = document.getElementById("tracklist");
var context = canvas.getContext("2d");

var playlist_focused = false;

var PLPOS = [0,0];
var PLBOUND = [1500,1000];
var PLCELL = [20,60];
var PLCELLMAX = [100,100];
var UI_SENS = [1,0.5];

var ui_refresh = Date.now();
var ui_refresh_rate = 3;

function addEventListeners(){
    $("#tracklist").on("mouseenter", function(e){
    console.log("FOCUS")
    playlist_focused=true;
    });

    $("#tracklist").on("mouseleave", function(e){
        playlist_focused=false;
    });

    $("#tracklist").on("wheel", function(e){
        if(!window.event.ctrlKey){
            if(playlist_focused){
                PLPOS[0] -= e.originalEvent.deltaX*UI_SENS[0];
                PLPOS[1] -= e.originalEvent.deltaY*UI_SENS[1];
                clampPLBound();
            }
        }else{
            if(!window.event.shiftKey && window.event.altKey){
                //zoom cell width
                //todo: show poly rhythym vert lines?
                PLCELL[0] += e.originalEvent.deltaX*0.5;

            }else if(window.event.shiftKey && !window.event.altKey){
                //zoom cell height
                //todo: show average audio peak in a horiz line?
                PLCELL[1] += e.originalEvent.deltaY*0.5;

            }else{
                //zoom cell w and h
                PLCELL[0] += e.originalEvent.deltaY*0.5;
                PLCELL[1] += e.originalEvent.deltaY*0.5;
            }
            PLCELL[0] = clamp(PLCELL[0], 10, PLCELLMAX[0]);
            PLCELL[1] = clamp(PLCELL[1], 10, PLCELLMAX[1]);
        }
        //console.log(PLPOS.toString());
        if((Date.now() - ui_refresh) >= ui_refresh_rate){
            drawBoard(PLPOS[0],PLPOS[1]);
            ui_refresh = Date.now();
        }
    });
}

function initPlaylistBound(){
    if($("#tracklist").width() > PLBOUND[0]){
        PLBOUND[0] = $("#tracklist").width()+100;
    }
    if($("#tracklist").height() > PLBOUND[1]){
        PLBOUND[1] = $("#tracklist").height()+120;
    }
}

function clampPLBound(){
    PLPOS[0] = clamp(PLPOS[0], -1*PLBOUND[0], 0);
    PLPOS[1] = clamp(PLPOS[1], -1*PLBOUND[1], 0);
}

function resize_check(){
    // var temp = $(".playlist")[0].innerHTML;
    // $(".playlist")[0].innerHTML = "";
    // $(".playlist")[0].innerHTML = temp;
    // var canvas = document.getElementById("tracklist");
    // var context = canvas.getContext("2d");
    canvas.setAttribute('width', $("#tracklist").width());
    canvas.setAttribute('height', $("#tracklist").height());
    drawBoard(PLPOS[0],PLPOS[1]);
}

function fullscreen(){
    var window = remote.getCurrentWindow();
    window.maximize();
    window.focus();
    window.setMinimumSize(1600, 800);
    var audio = new Audio('../AUDIO/START/startup.wav');
    audio.play();
    resize_check();
}

function drawBoard(offsetX,offsetY){
    if(offsetX >= PLCELL[0] || offsetX <= -1*PLCELL[0]){
        offsetX = Math.trunc(offsetX/PLCELL[0]);
    }
    context.beginPath();
    context.clearRect(0, 0, $("#tracklist").width(), $("#tracklist").height());
    for (var x = 0.5; x <= PLBOUND[0]; x += PLCELL[0]) {
        for (var y = 0.5; y <= PLBOUND[1]; y += PLCELL[1]) {
            //draw horizontal line
            if(y >= -1*offsetY){
                context.moveTo(0, y+offsetY);
                context.lineTo(PLBOUND[0], y+offsetY);
            }
        }
        //draw vertical line
        if(x >= -1*offsetX){
            context.moveTo(x+offsetX, 0);
            context.lineTo(x+offsetX, PLBOUND[1]);
        }
        
    }
    context.strokeStyle = "white";
    context.stroke();
}




function mixerrack(){
    for(i = 1; i <= 200; i++){
        $("#mixerrack").append('<li class="list-group-item mixi bg-dark"><kbd>' + i + '</kbd><div class="slider-container"><span class="bar"><span class="fill"></span></span><input type="range" id="slider' + i + '" class="slider" min="0" max="100" value="50"></div></li>');
        $("#slider" + i).on("input", setBar(i));
      }
}

function setBar(number){
  var min = parseInt($("#slider" + number).attr("min"));
  var max = parseInt($("#slider" + number).attr("max"));
  var val = parseInt($("#slider" + number).val());
  var percent = ((val - min) / (max - min) * 100);
  console.log(number)
  $(".bar .fill").height(percent + "%");
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
  