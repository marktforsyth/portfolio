const ww = window.innerWidth
const wh = window.innerHeight

const app = new PIXI.Application(width=ww, height=wh)
document.body.appendChild(app.view)

const dot = new PIXI.Graphics()
let xVel = 15
let yVel = 11
let radius = 30

dot.beginFill(0x0000FF)
dot.drawCircle(0, 0, radius)

dot.x = ww/2
dot.y = wh/2
app.stage.addChild(dot)

app.ticker.add(() => {
  if (radius <= 200) {
    dot.x += xVel
    dot.y += yVel
    
    if (dot.x + radius >= ww || dot.x - radius <= 0) {
      xVel = -xVel
    }
    if (dot.y + radius >= wh || dot.y - radius <= 0) {
      yVel = -yVel
    }
  }
  radius += 1
  dot.drawCircle(0, 0, radius)
})