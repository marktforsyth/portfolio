package main

import (
	"image/color"
	_ "image/png"
	"log"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
)

var circle *ebiten.Image

type drawingBrush struct {
	Size float64
	Color color.RGBA
}
var brush drawingBrush

func init() {
	var err error
	circle, _, err = ebitenutil.NewImageFromFile("images/purple.png")

	if err != nil {
		log.Panic(err)
	}

	brush = drawingBrush{
		Size: 50.0,
	}
}

type Game struct{}

func (g *Game) Update() error {
	return nil
}

func (g *Game) Draw(screen *ebiten.Image) {
	screen.Fill(color.RGBA{0xff, 0xff, 0xff, 0xff})

	op := &ebiten.DrawImageOptions{}

	mouseX, mouseY := ebiten.CursorPosition()
	op.GeoM.Scale(brush.Size/300, brush.Size/300)
	op.GeoM.Translate(float64(mouseX) - brush.Size/2, float64(mouseY) - brush.Size/2)

	screen.DrawImage(circle, op)
}

func (g *Game) Layout(outsideWidth, outsideHeight int) (screenWidth, screenHeight int) {
	return 1000, 500
}

func main() {
	game := &Game{}

	ebiten.SetWindowSize(1000, 500)
	ebiten.SetWindowTitle("Drawing App")

	if err := ebiten.RunGame(game); err != nil {
		log.Fatal(err)
	}
}