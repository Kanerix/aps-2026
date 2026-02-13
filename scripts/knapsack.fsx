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
        | _ when weight = 0 -> 0 :: aux item prev (weight + 1)
        | _, w when weight < w -> prev[weight] :: aux item prev (weight + 1)
        | v, w ->
            let pi = prev[weight]
            let ci = prev[weight - w] + v
            max pi ci :: aux item prev (weight + 1)

    let bc = row (C + 1)

    items
    |> List.fold
        (fun (prev, acc) item ->
            let after = aux item prev 0
            let combined = after :: acc
            after, combined)
        (bc, [])
    |> snd
    |> List.rev

let table = dp C items

let rec backtrack (table: int list list) =
    match table with
    | [] -> []
    | h :: m :: t ->
        Console.WriteLine h
        0 :: backtrack t
    | h :: t -> 0 :: backtrack t

let result = backtrack table

Console.Write result
