// Next Up: Make lines not double back on themselves
// if there is an x & y in that direction && coordinate doesn't already exist

const canvas = document.getElementById('canvas')
const ctx = canvas.getContext('2d')

ctx.fillStyle = 'gray'
ctx.lineWidth = '1.5'

const grid = []
for (let x = 1; x < 50; x++) {
    for (let y = 1; y < 50; y++) {
        grid.push([x, y])
    }
}

let path = [[1, 1]]

for (let d = 0; d < 100; d++) {
    let possiblePoints = []
    let x = path[d][0]
    let y = path[d][1]
    let options = 0
    
    console.log(path)

    console.log(path.indexOf([1, 1]))

    if (y - 1 > 1 && path.indexOf([x, (y - 1)]) != -1) {
        possiblePoints.push([x, (y - 1)])
        options += 1
    }
    if (y + 1 < 50 && path.indexOf([x, (y + 1)]) != -1) {
        possiblePoints.push([x, (y + 1)])
        options += 1
    }
    if (x - 1 > 1 && path.indexOf([(x - 1), y]) != -1) {
        possiblePoints.push([(x - 1), y])
        options += 1
    }
    if (x + 1 < 50 && path.indexOf([(x + 1), y]) != -1) {
        possiblePoints.push([(x + 1), y])
        options += 1
    }
    
    let choice = Math.floor((Math.random() * options))
    //console.log(possiblePoints)
    path.push(possiblePoints[choice])
}

function render() {
    grid.forEach(point => {
        ctx.fillRect((point[0]*10)-1, (point[1]*10)-1, 2, 2)
    })
    
    for (let p = 0; p < path.length; p++) {
        if (path[p + 1] != undefined) {
            ctx.moveTo(((path[p][0])*10), ((path[p][1])*10))
            ctx.lineTo(((path[p + 1][0])*10), ((path[p + 1][1])*10))
            ctx.stroke()
        }
    }
}

render()
