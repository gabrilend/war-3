use location.h

struct Map {
    width: u32,
    height: u32,
    tiles: Vec<Vec<Location>>,
}

impl Map {
    fn new(width: u32, height: u32) -> Map {
        let mut tiles = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(Location::new(x, y));
            }
            tiles.push(row);
        }
        Map {
            width: width,
            height: height,
            tiles: tiles,
        }
    }

    fn get_location(&self, x: u32, y: u32) -> Option<&Location>{
        if x < self.width && y < self.height {
            Some(&self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }
        

    fn get_neighbors(&self, x: u32, y: u32, direction: u8) -> &mut Location;
