const bx = window.innerWidth
const by = window.innerHeight

const app = new PIXI.Application(width=bx, height=by)
document.body.appendChild(app.view)

const player = {
  y: 0,
  nextY: 0,
  xVel: 0,
  yVel: 0,
  sprite: new PIXI.Graphics()
}

player.sprite.beginFill(0x0000FF)
player.sprite.drawRect(bx/2-25, player.y, 50, 50)
app.stage.addChild(player.sprite)

const terrain = {
  blocks: []
}