import <distance.h>;

struct Location {
  u8 x;
  u8 y;
};

impl Location {
  fn distance(&self) -> u8 {
    distance(self.x, self.y)
  }
}

fn to_meta_location(location: Location) -> MetaLocation {
  MetaLocation {
    x: location.x,
    y: location.y,
    distance: distance(location.x, location.y),
  }
}

struct meta_location {
    meta_location: meta_location,
}

impl meta_location {
    fn to_meta_location(&self) -> meta_location {
        meta_location {
            meta_location: self.meta_location,
        }
    }
    fn new(x: u8, y: u8) -> meta_location {
        meta_location {
            meta_location: meta_location {
                x: x,
                y: y,
                distance: distance(x, y),
            },
        }
    }
}

struct meta_location { meta_location: Box<meta_location>, }

impl meta_location { 
    fn to_meta_location(&self) -> meta_location { 
        meta_location { meta_location: Box::new(self.meta_location.clone()), 
        } 
    } 
    fn new(x: u8, y: u8) -> meta_location { 
        meta_location {
            meta_location: Box::new(meta_location { 
                x: x, y: y, distance: distance(x, y), 
            }),
        } 
    } 
}
