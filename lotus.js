const { app, BrowserWindow, ipcMain} = require('electron')

let win

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
