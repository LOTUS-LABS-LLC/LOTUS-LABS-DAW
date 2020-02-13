const { app, BrowserWindow, ipcMain} = require('electron')
const DiscordRPC = require('discord-rpc');
const config = require('./package.json');

let win
const clientId = '676326859851563018';

// only needed for discord allowing spectate, join, ask to join
DiscordRPC.register(clientId);

const rpc = new DiscordRPC.Client({ transport: 'ipc' });
const startTimestamp = new Date();

async function setActivity() {
  if (!rpc || !win) {
    return;
  }

  rpc.setActivity({
    details: `LOTUS DAW`,
    state: 'Ver ' + config.version,
    startTimestamp,
    largeImageKey: 'lotus',
    largeImageText: 'Lotus DAW',
    smallImageKey: 'online',
    smallImageText: 'Online',
    instance: false,
  });
}

rpc.on('ready', () => {
  setActivity();

  // activity can only be set every 15 seconds
  setInterval(() => {
    setActivity();
  }, 15e3);
});

rpc.login({ clientId }).catch(console.error);

console.log(config.version);

function createWindow () {
  win = new BrowserWindow({
    width: 450,
    height: 350,
    icon: "PANEL/IMG/lotus.jpg",
    frame: false,
    webPreferences: {
      nodeIntegration: true
    }
  })

  win.loadFile('PANEL/HTML/load.html')

  win.on('closed', () => {
    win = null
  })
}

app.on('ready', createWindow)

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

app.on('activate', () => {
  if (win === null) {
    createWindow()
  }
})
