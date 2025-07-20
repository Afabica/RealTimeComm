# Create Next.js app (React + Next.js

- npx create-next-app@latest my-next-tailwind-app

# Tailwind installation

- npm install -D tailwindcss postcss autoprefixer
- npx tailwincss init -p

# Configure Tailwind

<mark>
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};

</mark>

# Add Tailwind directieves to global CSS

@tailwind base;
@tailwind components;
@tailwind utilities;

<mark>
// pages/_app.tsx or _app.js
import '../styles/globals.css'

export default function MyApp({ Component, pageProps }) {
return <Component {...pageProps} />
}

</mark>

# Recommended packages for design & developer experience

npm install @headlessui/react @heroicons/react
npm install clsx

# Electron

npm install electron --save-dev
npm install electron-builder --save-dev

## Create Electron Main Process

<mark>
const { app, BrowserWindow } = require('electron');
const path = require('path');

function createWindow() {
const mainWindow = new BrowserWindow({
width: 1024,
height: 768,
webPreferences: {
nodeIntegration: false,
contextIsolation: true,
preload: path.join(\_\_dirname, 'preload.js'), // optional if using preload script
},
});

// Load your Next.js app's URL or local build:
mainWindow.loadURL('http://l

</mark>

## Add preload.js

<mark>
const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electronAPI', {
sendMessage: (msg) => ipcRenderer.send('message', msg),
onMessage: (callback) => ipcRenderer.on('message', callback),
});

</mark>

## Updated package.json

<mark>
"scripts": {
  "dev": "next dev",
  "build": "next build",
  "start": "next start",
  "electron": "electron .",
  "electron-dev": "concurrently \"next dev\" \"wait-on http://localhost:3000 && electron .\""
}

</mark>
