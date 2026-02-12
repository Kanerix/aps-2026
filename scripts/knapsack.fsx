open System

let inputLine = Console.ReadLine().Split ' '

let C = int inputLine[0]
let n = int inputLine[1]

type Item = (int * int)

let rec readItems n =
    match n with
    | 0 -> []
    | _ ->
        let itemLine = Console.ReadLine().Split ' '
        let value = int itemLine[0]
        let weight = int itemLine[1]
        Item(value, weight) :: readItems (n - 1)

let items = readItems n

let rec row n =
    match n with
    | 0 -> []
    | _ -> 0 :: row (n - 1)

let rec dp (C: int) (items: Item list) =
    let rec aux (item: Item) (prev: int list) (weight: int) =
        match item with
        | _ when weight >= C + 1 -> []
        | _, w when weight = 0 -> prev[0] :: aux item prev (weight + 1)
        | _, w when weight < w -> prev[weight - 1] :: aux item prev (weight + 1)
        | v, w ->
            let p = prev[weight - 1]
            let o = prev[weight - w] + v
            let choice = max p o
            choice :: aux item prev (weight + 1)

    let bc = row C

    items |> List.fold (fun curr item -> aux item curr 0) bc

dp C items |> List.iter (printf "%d ")
