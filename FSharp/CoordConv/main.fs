open System.Text.RegularExpressions

module Converters =
  let cartToPol x y =
    let r = sqrt (x**2.0 + y**2.0)
    let theta = atan y/x

    r, theta
  
  let polToCart r theta =
    let x = r*cos theta
    let y = r*sin theta

    x, y

module Processing =
  let rec proccessCmds (stage, coType, firstCo, secndCo) =
    let statement =
      match stage with
      | 0 ->
        "Are your input coordinates cartesian (c) or polar (p)?"
      | 1 ->
        match coType with
        | "polar" -> "Enter radius:"
        | "cartesian" -> "Enter x:"
        | _ -> "Error, wrong type"
      | 2 ->
        match coType with
        | "polar" -> "Enter theta:"
        | "cartesian" -> "Enter y:"
        | _ -> "Error, wrong type:"
      | 3 ->
        match coType with
        | "polar" -> "Cartesian coordinates:"
        | "cartesian" -> "Polar coordinates:"
        | _ -> "Error, type not found"
      | _ -> "Error, stage not found"
    printfn "\n%s" statement

    if stage<>3 then
      let command = System.Console.ReadLine()
      let isANumber = Regex.Match(command, "\\d*\\.?\\d+") in

      let result = 
        match stage with
        | 0 ->
          match command with
          | "polar" | "p" | "P" ->
            1, "polar", firstCo, secndCo
          | "cartesian" | "c" | "C" ->
            1, "cartesian", firstCo, secndCo
          | _ ->
            printfn "Error, coordinate system not recognized"
            0, "error", 0.0, 0.0
        | 1 ->
          if isANumber.Success then
            2, coType, (float command), secndCo
          else
            printfn "Error, not a valid number."
            1, coType, firstCo, secndCo
        | 2 ->
          if isANumber.Success then
            3, coType, firstCo, (float command)
          else
            printfn "Error, not a valid number."
            2, coType, firstCo, secndCo
        | _ -> 0, "error", 0.0, 0.0
    
      proccessCmds result
    else
      let convFirstCo, convSecndCo =
        match coType with
        | "polar" -> Converters.polToCart firstCo secndCo
        | "cartesian" -> Converters.cartToPol firstCo secndCo
        | _ -> 0.0, 0.0
      printfn "%f, %f" convFirstCo convSecndCo
    
  proccessCmds (0, "none", 0.0, 0.0)