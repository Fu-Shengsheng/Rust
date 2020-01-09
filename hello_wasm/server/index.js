const Koa = require('koa')
const path = require('path')
const static = require('koa-static')

const app = new Koa()
const staticPath = ''

app.use(static(
    path.join(__dirname, staticPath)
))

app.listen(7777, () => {
    console.log('server listening port 7777')
})