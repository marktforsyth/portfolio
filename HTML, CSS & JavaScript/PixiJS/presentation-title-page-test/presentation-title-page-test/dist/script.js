const ww = window.innerWidth
const wh = window.innerHeight

const app = new PIXI.Application(width=ww, height=wh)
document.body.appendChild(app.view)

const dot = new PIXI.Graphics()
let xVel = 15
let yVel = 11
let radius = 530

dot.beginFill(0x0000FF)
dot.drawCircle(0, 0, radius)

dot.x = 938
dot.y = 219
app.stage.addChild(dot)

const title = new PIXI.Text('The Many Distinct Ways to...')
title.style = new PIXI.TextStyle({
  fill: 0xFFFFFF,
  fontFamily: 'Sans Serif',
  fontSize: 100
})

title.anchor.set(0.5)
title.x = ww/2
title.y = wh/2

app.stage.addChild(title)