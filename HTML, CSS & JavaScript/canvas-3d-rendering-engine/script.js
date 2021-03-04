var atoms = [
  {
    location: [8, 5, 5],
    color: [255, 255, 255]
  },
  {
    location: [8, -5, 5],
    color: [255, 255, 255]
  },
  {
    location: [8, 5, -5],
    color: [255, 255, 255]
  },
  {
    location: [8, -5, -5],
    color: [255, 255, 255]
  },
  {
    location: [3, 5, 5],
    color: [255, 255, 255]
  },
  {
    location: [3, -5, 5],
    color: [255, 255, 255]
  },
  {
    location: [3, 5, -5],
    color: [255, 255, 255]
  },
  {
    location: [3, -5, -5],
    color: [255, 255, 255]
  },
]
var camera = {
  location: [0, 0, 0],
  angle: 0,
  lens: 2,
  width: 50,
  height: 50
}
var light = {
  location: [0, 0, 0],
  strength: 30
}

function render(atoms, camera, light) {
  var pixels = []
  
  atoms.forEach((atom) => {
    var cdx = Math.abs(atom.location[0] - camera.location[0])
    var cdy = atom.location[1] - camera.location[1]
    var cdz = atom.location[2] - camera.location[2]
    
    var x = 4*(500/camera.width)*(camera.location[1]+(camera.lens/cdx * cdy))+500/2
    var y = -(4*(500/camera.height)*(camera.location[2]+(camera.lens/cdx * cdz)))+500/2
    
    var ldx = Math.abs(atom.location[0] - light.location[0])
    var ldy = Math.abs(atom.location[1] - light.location[1])
    var ldz = Math.abs(atom.location[2] - light.location[2])
    
    var distance = Math.sqrt(ldx*ldx+ldy*ldy+ldz*ldz)
    var alpha = alpha = 1-(distance/light.strength)
    
    pixels.push({
      location: [x, y],
      color: atom.color,
      alpha: alpha
    })
  })
  
  return pixels
}

var canvas = document.getElementById('canvas')
var ctx = canvas.getContext('2d')

function dot(x, y, color, alpha) {
  ctx.fillStyle = 'rgba(' + color + ', ' + alpha + ')'

  ctx.beginPath()
  ctx.arc(x, y, 3, 0, 2*Math.PI)
  ctx.fill()
  ctx.closePath()
}

var pixels = render(atoms, camera, light)
pixels.forEach((pixel) => (
  dot(pixel.location[0], pixel.location[1], pixel.color, pixel.alpha)
))