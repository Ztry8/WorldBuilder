# WorldBuilder
[![GitHub last commit](https://img.shields.io/github/last-commit/ztry8/worldbuilder)](https://github.com/worldbuilder/commits)
[![License](https://img.shields.io/github/license/worldbuilder)](https://github.com/worldbuilder/blob/main/LICENSE)

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

### Configuration

Generation is configured via `config.ron` in the root directory:
| Field        |   Meaning    | 
| ------------- | ----- |
| `size`    | The size of the world (map). | 
| `size_k`    | The smaller the value, the fewer the mountains and lakes, but the bigger their size, and vice versa. | 
| `borders` | If `Some(tile)`, the map is divided into `tile`x`tile` cell chunks with borders in the form of mountains. | 
| `*_color` | RGBA colors for each cell type. |

If `config.ron` is missing or invalid, default values are used automatically.

The generation seed is derived from the current time, so every run produces a unique map.

Example `config.ron`:
```ron
(
    size: 1280,
    size_k: 0.1,
    borders: Some(64),
    water_color: (0, 0, 255, 255),
    no_color: (0, 0, 0, 255),
    low_color: (255, 255, 255, 255),
    mid_color: (100, 100, 100, 255),
    high_color: (50, 50, 50, 255),
)
```

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
The generation time (size = 1280, size_k = 0.1) is around ```0.3 seconds```.

### Results
#### size = 1280; size_k = 0.1
![screenshot](https://github.com/Ztry8/WorldBuilder/blob/main/maps/1.png)
#### size = 1280; size_k = 0.05
![screenshot](https://github.com/Ztry8/WorldBuilder/blob/main/maps/2.png)