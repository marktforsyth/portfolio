package main

import (
	"github.com/llgcode/draw2d/draw2dimg"

	"image"
	"image/color"
)

func main() {
  dest := image.NewRGBA(image.Rect(0, 0, 500, 500))
  gc := draw2dimg.NewGraphicContext(dest)

  gc.SetFillColor(color.RGBA{0, 0, 0, 0})
  gc.SetStrokeColor(color.RGBA{200, 200, 200, 255})
  gc.SetLineWidth(3)

  gc.BeginPath()
  drawRandomLines(gc)
  gc.Close()
  gc.FillStroke()

  draw2dimg.SaveToPngFile("drawing.png", dest)
}

func drawRandomLines(gc *draw2dimg.GraphicContext) {
  gc.MoveTo(10, 10)
  gc.LineTo(10, 400)
  gc.LineTo(490, 490)
  gc.LineTo(490, 10)
  gc.LineTo(10, 10)
}