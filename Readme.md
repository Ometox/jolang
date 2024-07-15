# Jolang
---

A jit inerpreted language created by joachim barre

it is based on a set of basic instruction an use a single register and a memory tape that as an initial state and size define in the object 
numbers are currently 64 bits

## instruction set

currently the language is composed of ten instructions :

| id | symbol | decription                                                                          |
| -- | --     | --                                                                                  |
| 0  | <      | move the memory tape backward                                                       |
| 1  | >      | move the memory tape forward                                                        |
| 4  | L      | load a value from the tape into the register                                        |
| 3  | S      | store what in the rengister into the current memory tape entry                      |
| 4  | +      | add the current value on the memory tape to the register                            |
| 5  | -      | subtract the current value on the memory tape from the register                     |
| 6  | *      | multiply the current register value by the current memory tape value                |
| 7  | /      | divide the current register value by the current memory tape value                  |
| 8  | P      | print the register value to stdout                                                  |
| 9  | [      | label : nexts jumps will jump to the instruction after this one                     | 
| 10 | ]      | jumps to the last label                                                             |
| 11 | }      | jumps to the last label if the current register value is 0                          |
| 12 | Q      | exit program with the exit code contained in the register                           |
| 13 | I      | increase the register value by 1                                                    |
| 14 | D      | decrease register value by 1                                                        |

## file formats

| extention | description            |
| --        | --                     |
| .joo      | compiled jolang object |
| .jol      | source code            |
