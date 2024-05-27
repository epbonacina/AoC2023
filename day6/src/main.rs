struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn from(times: Vec<u64>, record_distances: Vec<u64>) -> Vec<Race> {
        let mut races = Vec::new();
        for (time, record_distance) in times.into_iter().zip(record_distances) {
            races.push(Race {
                time,
                record_distance,
            });
        }
        races
    }
}

fn get_number_of_ways_i_could_beat_the_record(races: Vec<Race>) -> u32 {
    let mut number_of_ways_for_each_race = Vec::new();
    for race in races {
        let mut number_of_ways_for_this_race = 0;
        for hold_time in 1..race.time {
            let speed = hold_time;
            let distance = speed * (race.time - hold_time);
            if distance > race.record_distance {
                number_of_ways_for_this_race += 1;
            }
        }
        number_of_ways_for_each_race.push(number_of_ways_for_this_race);
    }
    match number_of_ways_for_each_race.into_iter().reduce(|acc, elem| acc * elem) {
        Some(value) => return value.clone(),
        None => 0,
    }
}

fn main() {
    let times = vec![59, 70, 78, 78];
    let distances = vec![430, 1218, 1213, 1276];

    // let times = vec![7, 15, 30];
    // let distances = vec![9, 40, 200];

    let races = Race::from(times, distances);
    let ways = get_number_of_ways_i_could_beat_the_record(races);
    println!("Part one solution: {}", ways);

    // let times = vec![71530];
    // let distances = vec![940200];

    let times = vec![59707878];
    let distances = vec![430121812131276];

    let races = Race::from(times, distances);
    let ways = get_number_of_ways_i_could_beat_the_record(races);
    println!("Part two solution: {}", ways);
}
