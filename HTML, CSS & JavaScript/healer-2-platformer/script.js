// Import canvas and define size --
var canvas = document.getElementById('canvas')
var ctx = canvas.getContext('2d')

var canvasSize = {
  w: 500,
  h: 500
}

// Import images --
var charIcon = document.getElementById('character')
var blockIcon = document.getElementById('block')

// Define player stats --
var player = {
  x: (canvasSize.w / 2) - (80 / 2),
  y: 0,
  size: 80,
  speed: 5,
  height: 15,
  yVel: 0
}

// Setup blocks for floor and walls & define size --
var blocks = []
var blockSize = 80
var lvlLength = 100

function makeBlocks() {
  // 1st row
  for (var x = 0; x < lvlLength; x++) {
    var locX = (x * blockSize) + ((canvasSize.w / 2) - (blockSize / 2))
    var locY = (canvasSize.h - blockSize)

    blocks.push([locX, locY])
  }

  // 2nd row
  var row2 = []

  for (var x = 0; x < lvlLength; x++) {
    var chance = Math.random()

    if (chance < 0.35) {
      var locX = (x * blockSize) + ((canvasSize.w / 2) - (blockSize / 2))
      var locY = (canvasSize.h - (blockSize * 2))

      blocks.push([locX, locY])
      row2.push(x)
    }
  }

  // 3rd row
  for (var x = 0; x < row2.length; x++) {
    var chance = Math.random()

    if (chance < 0.4) {
      var locX = (row2[x] * blockSize) + ((canvasSize.w / 2) - (blockSize / 2))
      var locY = (canvasSize.h - (blockSize * 3))

      blocks.push([locX, locY])
    }
  }
}

// Setup collision detection functions --

// Check for x collision
function xCollide(x, ax) {
  if ((x < (ax + blockSize) && x >= ax) || (ax < (x + player.size) && ax >= x)) {
    return true
  }
}

// Check for y collision
function yCollide(y, ay) {
  if ((y < (ay + blockSize) && y >= ay) || (ay < (y + player.size) && ay >= y)) {
    return true
  }
}

// Are we colliding more x, or more y?
function compareWhere(player, item) {
  // Define difference variables
  var xDif = Math.abs(player.x - item[0])
  var yDif = Math.abs(player.y - item[1])

  // Check which one is bigger
  if (xDif > yDif) {
    return 'x'
  } else if (yDif >= xDif) { // In nuetral, divert to y being bigger
    return 'y'
  }
}

// Put them together to check for collision in 2 dimensions
function willCollide(dx, dy) {
  // Loop through all of the blocks
  for (var blk = 0; blk < blocks.length; blk++) {
    // Define individual block
    var block = blocks[blk]

    // Now check for collision
    if (xCollide((player.x + dx), block[0]) && yCollide((player.y + dy), block[1])) {
      return block
    }
  }

  // If we haven't collided yet, say we're good to go
  return false
}

// Expiriment of a move player function --* ...? [Insert here]

// "Listen" for the arrow keys --

// Define variables for each key
var up = false
var left = false
var right = false

// Listen for key press
document.onkeydown = function(event) {
  if (event.keyCode === 37) { // Left arrow
    left = true
  } if (event.keyCode === 39) { // Right arrow
    right = true
  } if (event.keyCode === 38) { // Up arrow
    up = true
  }
}

// Listen for key release
document.onkeyup = function(event) {
  if (event.keyCode === 37) { // Left arrow
    left = false
  } if (event.keyCode === 39) { // Right arrow
    right = false
  } if (event.keyCode === 38) { // Up arrow
    up = false
  }
}

// Draw all the things! --
function drawWorld() {
  // Find the middle of the screen - used for player and blocks
  var middleScreen = (canvasSize.w / 2) - (player.size / 2)

  // Draw all of the blocks --

  // Cycle through all the blocks
  for (var blk = 0; blk < blocks.length; blk++) {
    // Define individual block
    var block = blocks[blk]
    
    // Figure out where we're drawing it
    var blkDrawX = block[0] + (middleScreen - player.x)
    var blkDrawY = block[1] // Nothing changes for now

    // Draw block
    ctx.drawImage(blockIcon, blkDrawX, blkDrawY, blockSize, blockSize)
  }

  // Draw the player --
  ctx.drawImage(charIcon, middleScreen, player.y, player.size, player.size)
}

// The main loop that gets run every frame --
function main() {
  // Handle player movement and gravity --

  // Make variables for collision down, left and right
  var colliding = {
    up: willCollide(0, -5),
    down: willCollide(0, Math.max(5, player.yVel)),
    left: willCollide(-5, 0),
    right: willCollide(5, 0)
  }

  // Move the player .unless. they collide
  if (up === true && colliding.up === false && colliding.down != false) { // Jumping
    player.yVel -= player.height
  }

  if (left === true) { // Going left
    if (colliding.left === false) {
      player.x -= player.speed
    } else if (compareWhere(player, colliding.left) === 'x') {
      player.x = (colliding.left[0] + blockSize) // Stop at wall
    }
  } if (right === true) { // Going right
    if (colliding.right === false) {
      player.x += player.speed
    } else if (compareWhere(player, colliding.left) === 'x'){
      player.x = (colliding.right[0] - player.size) // Stop at wall
    }
  }

  // Update gravity
  if (colliding.down != false && up === false && compareWhere(player, colliding.down) === 'y') { // If we're on the ground, stay there
    player.y = (colliding.down[1] - player.size)
    player.yVel = 0
  } else { // If not on the ground, gravity kicks in
    player.yVel += 0.5
  }

  // Move the player up or down
  player.y += player.yVel

  // Clear, redraw and keep moving --

  // Clear canvas before drawing
  ctx.clearRect(0, 0, canvasSize.w, canvasSize.h)

  // Draw all of the things
  drawWorld()

  // Update animation loop
  window.requestAnimationFrame(main)
}

// Run all of the things finally
makeBlocks()
drawWorld()

//ctx.drawImage(charIcon, 20, 420, player.size, player.size)

window.requestAnimationFrame(main)