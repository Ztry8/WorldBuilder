# WorldBuilder
## Lightweight and simple heightmap generator for open-world game landscapes!
### The program was made for generating the world of the game [Grokvil](https://grokvil.world/), which I'm working on.
![screenshot](https://github.com/Ztry8/WorldBuilder/blob/main/maps/1.png)

### About generator

The program generates a heightmap in PNG format based on [Perlin noise](https://en.wikipedia.org/wiki/Perlin_noise).  
The map is divided into 4 types of pixels:
| Pixel         |   Color    | Height (Y coordinate)    |
| ------------- | ----- |------------- |
| Water    | Blue  | -1 |
| Plain    | Black | 0 |
| Hill     | White | 1 |
| Mountain | Gray  | 2 |
| Cliff    | Dark gray  | 3 |

### Building
