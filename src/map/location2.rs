use entity::Entity;
use map::{distance, Map};

struct LocationStats {
    stats: [u8, 8],
}

impl LocationStats {
    new(temperature: u8,
        temp_acceleration: u8,
        humidity: u8,
        humidity_acceleration: u8,
        mana_pressure: u8,
        blood_pressure: u8,
        wind: u8,
        wind_direction: u8
    ) -> LocationStats {
        LocationStats {
            stats: [temperature,
                    temp_acceleration,
                    humidity,
                    humidity_acceleration,
                    mana_pressure,
                    blood_pressure,
                    wind,
                    wind_direction],
        }
    }

    pub fn randomize() {
        for i in 0..self.stats.len() {
            self.stats[i] = rand::thread_rng().gen::<u8>();
        }
    }
}


struct Location<'a> {
    map: &'a Map,
    x: u8,
    y: u8,
    stats: LocationStats,
    num_stats: u8,
    attention: Option<&'a Entity>,
    objects: Vec<Entity>,
}

impl<'a> Location<'a> {
    fn new(x: i32, y: i32) -> Location<'a> {
        Location {
            x: x,
            y: y,
            stats: LocationStats::new(0, 0, 0, 0, 0, 0, 0, 0),
            num_stats: 8,
            attention: None,
            objects: Vec::new(),

        }
    }

    pub fn get_distance(&self, other: &Location) -> i32 {
        distance((self.x, self.y), (other.x, other.y))
    }

    fn add_object(&mut self, object: Entity) {
        object.location = (self.x, self.y);
        self.objects.push(object);
    }

    fn remove_object(&mut self, &mut object: Entity) {
        self.objects.pop(object);
    }

    fn push_out(&mut self, other: &mut Location) { 
        for i in 0..self.num_stats {
            let diff: u8 = self.stats.stats[i] - other.stats.stats[i];
            let diff = diff / 8;
            if diff = 0 {
                diff = 1;
            }
            self.stats.stats[i] -= diff;
            other.stats.stats[i] += diff;
        }
    }

    fn pull_in(&mut self, other: &mut Location) {
        for i in 0..self.num_stats {
            let diff: u8 = self.stats.stats[i] - other.stats.stats[i];
            let diff = diff / 8;
            if diff = 0 {
                diff = 1;
            }
            self.stats.stats[i] += diff;
            other.stats.stats[i] -= diff;
        }
    }

    fn breathe(&mut self, breath: u8) {
        // for each x neighbor
        for i in -1..1 {
            // for each y neighbor
            for j in -1..1 {
                // skip self and locations that don't exist
                if  i != 0 &&
                    j != 0 &&
                    self.map.get_location(self.x, self.y) != None {
                    // if breath > 0, push out "breath" times
                    if breath > 0 { 
                        for k in 0..breath {
                            self.push_out(self.get_neighbour((i, j)));
                        }
                    // if breath < 0, pull in
                    } else {
                        for k in 0..breath {
                            self.pull_in(self.get_neighbour((i, j)));
                        }
                    }
                }
            }
        }
    }

    fn get_neighbour(&self, direction: (u8, u8)) -> &mut Location {
        self.map.get_location(self.x + direction.x, self.y + direction.y)
    }
}

