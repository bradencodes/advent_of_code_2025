mod part_1 {
    type Coordinate = u64;
    type X = Coordinate;
    type Y = Coordinate;
    type Z = Coordinate;
    type Location = (X, Y, Z);
    type JunctionBoxLocations = Vec<Location>;
    type Pair<'a> = (&'a Location, &'a Location);
    type Distance = f64;
    struct PairDistance<'a> {
        pair: Pair<'a>,
        distance: Distance,
    }

    fn to_f64_tuple(loc: &Location) -> (f64, f64, f64) {
        (loc.0 as f64, loc.1 as f64, loc.2 as f64)
    }

    fn calculate_distance(pair: &Pair) -> Distance {
        let (x1, y1, z1) = to_f64_tuple(pair.0);
        let (x2, y2, z2) = to_f64_tuple(pair.1);

        let distance = ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt();

        distance
    }

    fn find_sizes_of_3_largest_circuits_from_n_connections(input: &str, n_connections: u64) -> u64 {
        let sizes = 1;

        // 1. Parse the input to get all of the junction box locations.
        let junction_box_locations: JunctionBoxLocations = input
            .lines()
            .map(|line| {
                let parsed_coordinates_line = line
                    .split(',')
                    .map(|coordinate_string| coordinate_string.parse::<Coordinate>().unwrap())
                    .collect::<Vec<Coordinate>>();
                let location: Location = (
                    parsed_coordinates_line[0] as X,
                    parsed_coordinates_line[1] as Y,
                    parsed_coordinates_line[2] as Z,
                );

                location
            })
            .collect();

        // 2. Make an ordered list of all the closest pairs of junction boxes.
        let sorted_box_pairs = {
            let mut box_pair_distances = {
                let mut vector = Vec::new();

                for (index, box_1) in junction_box_locations.iter().enumerate() {
                    for box_2 in junction_box_locations[index + 1..].iter() {
                        let pair: Pair = (box_1, box_2);
                        let distance = calculate_distance(&pair);
                        vector.push(PairDistance { pair, distance });
                    }
                }

                vector
            };

            box_pair_distances.sort_by(|pair_distance_1, pair_distance_2| {
                pair_distance_1
                    .distance
                    .partial_cmp(&pair_distance_2.distance)
                    .unwrap()
            });

            box_pair_distances
        };

        // 3. Iterate through the closest pairs list the given number of times.

        //     a. If the pair is not already in the same circuit, connect them.

        // 4. Find the 3 largest circuits and multiply them. Return this value

        sizes
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn find_sizes_of_3_largest_circuits_from_n_connections_works() {
            let input = include_str!("./test_input.txt");
            assert_eq!(
                find_sizes_of_3_largest_circuits_from_n_connections(input, 10),
                40
            );
        }
    }
}
