open System

type Item = (int * int)

let rec readItems n =
    match n with
    | 0 -> []
    | _ ->
        let itemLine = Console.ReadLine().Split ' '
        let value = int itemLine[0]
        let weight = int itemLine[1]
        Item(value, weight) :: readItems (n - 1)

let dp (C: int) (items: Item list) =
    let aux (item: Item) (prev: int array) =
        let v, w = item
        Array.init (C + 1) (fun weight ->
            if weight = 0 then 0
            elif weight < w then prev.[weight]
            else max prev.[weight] (prev.[weight - w] + v))

    let bc = Array.zeroCreate (C + 1)

    items
    |> List.fold
        (fun (prev, acc) item ->
            let after = aux item prev
            after, after :: acc)
        (bc, [])
    |> snd

let backtrack (items: Item array) (table: int array list) =
    let rec aux (table: int array list) (row: int) (col: int) =
        match table with
        | h :: m :: t ->
            if h.[col] <> m.[col] then
                let v, w = items.[row - 1]
                row - 1 :: aux (m :: t) (row - 1) (col - w)
            else
                aux (m :: t) (row - 1) col
        | h :: [] -> if h.[col] <> 0 then [ row - 1 ] else []
        | _ -> failwith "unreachable"

    aux table table.Length (table.[0].Length - 1)

let rec solve () =
    let inputLine = Console.ReadLine()

    match inputLine with
    | null -> ()
    | line ->
        let inputLine = line.Split ' '

        let C = int inputLine[0]
        let n = int inputLine[1]

        let items = readItems n
        let itemsArr = List.toArray items
        let table = dp C items

        let result = backtrack itemsArr table |> List.rev

        Console.WriteLine result.Length
        Console.WriteLine(result |> List.map string |> String.concat " ")
        solve ()

solve ()
