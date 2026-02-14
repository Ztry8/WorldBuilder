# WorldBuilder
## Lightweight and simple heightmap generator for open-world game landscapes!
![screenshot](https://github.com/Ztry8/WorldBuilder/blob/main/maps/map.png)

### About generator

The program generates a heightmap in PNG format based on [Perlin noise](https://en.wikipedia.org/wiki/Perlin_noise).  
The map is divided into 5 types of cells (1 cell = 1 pixel = 1 square meter):
| Cell         |   Default Color    | Height (Y coordinate)    |
| ------------- | ----- |------------- |
| Water    | Blue  | -1 |
| Plain    | Black | 0 |
| Hill     | White | 1 |
| Mountain | Gray  | 2 |
| Cliff    | Dark gray  | 3 |  

You can change the generation result by modifying the constants:
| Constant        |   Meaning    | 
| ------------- | ----- |
| SIZE    | The size of the world (map). | 
| SIZE_K    | The smaller the SIZE_K, the fewer the mountains and lakes, but the bigger their size, and vice versa. | 
| SEED     | Generation seed. | 
| BORDERS | If true, the map is divided into 64x64 cell chunks with borders in the form of mountains. | 

(You can also change the cells colors through constants in the code.)

### Building
```bash
git clone https://github.com/Ztry8/WorldBuilder.git
cd WorldBuilder
cargo run --release
```
After the work is completed,  
there will be a file named ```map.png``` in the root directory.

### Time
Usually, an open-world game map is created once manually  
and doesn't need to be generated every time the game starts (unlike roguelikes),  
so the program was made without speed optimizations.  
The generation time (SIZE = 1280, SIZE_K = 0.1) is around ```0.3 seconds```.

### Results
#### SIZE = 1280; SIZE_K = 0.1
![screenshot](https://github.com/Ztry8/WorldBuilder/blob/main/maps/1.png)
#### SIZE = 1280; SIZE_K = 0.05
![screenshot](https://github.com/Ztry8/WorldBuilder/blob/main/maps/2.png)
