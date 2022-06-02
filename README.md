# Rustfuck

</br>

## Description
___
Basic Brainfuck interpeter written in Rust.

The main purpose for this project is intension to learn Rust language.
___

</br>
</br>

## Instructions
___

| **RF** |          **Instruction**          |                      **Description**                      |
|:------:|:---------------------------------:|:---------------------------------------------------------:|
|    >   | MoveRight                         | Move pointer to the right                                 |
|    <   | MoveLeft                          | Move pointer to the left                                  |
|    +   | IncrementValue                    | Increment memory cell at the pointer                      |
|    -   | DecrementValue                    | Decrement memory cell at the pointer                      |
|    .   | Output                            | Print value of the pointed memory cell to stdout          |
|    ,   | Input                             | Input a character and store it in the cell at the pointer |
|    [   | OpenLoop                          | Indicates loop beginning                                  |
|    ]   | CloseLoop                         | Indicates loop ending                                     |
|        | ExecuteLoopBody(Vec<Instruction>) | Execute instructions inside loop brackets                 |
|        | Ignore                            | Every other character is ignored by interpreter           |

___

</br>
</br>

## Roadmap

___
## 1.0
- [x] Basic working interpreter
- [x] Reading code from file
___
## 1.1
- [x] Contract optimization
- [x] Clearloop optimization
- ~~[x] Copyloop optimization~~
- ~~[x] Multiloop optimization~~
- [x] Diophantine loop optimization
- [ ] Scanloop optimization
- [ ] Offsetops optimization
- [ ] Nested Diophantine loops optimization
___
## 1.2
- [ ] Extended Stein's Algorithm optimization
- [ ] u16 and u32 memory cells
- [ ] Custom memory tape length
- [ ] Tests
___
## 2.0
- [ ] 2DFuck implementation