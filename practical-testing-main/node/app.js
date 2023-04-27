const express = require('express')
const app = express()
const port = 3000

app.get('/', (req, res) => {
  res.send('Hello World, Say Hi! Jumpbox')
})

app.listen(port, () => {
  console.log(`----- Node Server Starting -------`)
})