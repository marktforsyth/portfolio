package main

import (
	"fmt"
	"image/color"
	_ "image/png"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
)

var img *ebiten.Image

type awesomeRect struct {
	X float64
	Y float64
	Rotation float64
	Size float64
	Mode string
}
var myAwesomeRect awesomeRect

func init() {
	var err error
	img, _, err = ebitenutil.NewImageFromFile("Dark Chalk Void Symbol.png")
	if err != nil {
		fmt.Println(err)
	}

	myAwesomeRect = awesomeRect{
		X: 10,
		Y: 20,
		Rotation: 0,
		Size: 1,
		Mode: "shrinking",
	}
}

type Game struct {}

func (g *Game) Update() error {
	return nil
}

func (g *Game) Draw(screen *ebiten.Image) {
	screen.Fill(color.RGBA{0xfc, 0xfc, 0, 0xff})

	op := &ebiten.DrawImageOptions{}
	//op.GeoM.Translate(myAwesomeRect.X, myAwesomeRect.Y)
	//op.GeoM.Scale(myAwesomeRect.Size, myAwesomeRect.Size)
	//op.GeoM.Rotate(myAwesomeRect.Rotation)
	
	mouseX, mouseY := ebiten.CursorPosition()
	op.GeoM.Translate(float64(mouseX), float64(mouseY));

	screen.DrawImage(img, op)

	myAwesomeRect.X+=5
	myAwesomeRect.Y-=10
	if myAwesomeRect.Size > 0.3 && myAwesomeRect.Mode == "shrinking" {
		myAwesomeRect.Size *= 0.99
	} else {
		myAwesomeRect.Mode = "growing"
	}
	
	if myAwesomeRect.Mode == "growing" {
		myAwesomeRect.Size *= 1.01
	}

	myAwesomeRect.Rotation += 0.01
}

func (g *Game) Layout(outsideWidth, outsideHeight int) (screenWidth, screenHeight int) {
	return 1000, 600
}

func main() {
	ebiten.SetWindowSize(1000, 600)
	ebiten.SetWindowTitle("My Awesome GoLang GUI Window")

	if err := ebiten.RunGame(&Game{}); err != nil {
		fmt.Println(err)
	}
}