type Coordinate = u64;
type X = Coordinate;
type Y = Coordinate;
type Z = Coordinate;
type JunctionBox = (X, Y, Z);
type JunctionBoxes = Vec<JunctionBox>;
type Pair<'a> = (&'a JunctionBox, &'a JunctionBox);
type Distance = f64;
struct PairDistance<'a> {
    pair: Pair<'a>,
    distance: Distance,
}
type Circuit<'a> = Vec<&'a JunctionBox>;
type Circuits<'a> = Vec<Circuit<'a>>;

fn parse_junction_boxes(input: &str) -> JunctionBoxes {
    input
        .lines()
        .map(|line| {
            let parsed_coordinates_line = line
                .split(',')
                .map(|coordinate_string| coordinate_string.parse::<Coordinate>().unwrap())
                .collect::<Vec<Coordinate>>();
            let junction_box: JunctionBox = (
                parsed_coordinates_line[0] as X,
                parsed_coordinates_line[1] as Y,
                parsed_coordinates_line[2] as Z,
            );

            junction_box
        })
        .collect()
}

fn to_f64_tuple(loc: &JunctionBox) -> (f64, f64, f64) {
    (loc.0 as f64, loc.1 as f64, loc.2 as f64)
}

fn calculate_distance(pair: &Pair) -> Distance {
    let (x1, y1, z1) = to_f64_tuple(pair.0);
    let (x2, y2, z2) = to_f64_tuple(pair.1);

    let distance = ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt();

    distance
}

fn create_sorted_box_pair_distances<'a>(
    junction_boxes: &'a JunctionBoxes,
) -> Vec<PairDistance<'a>> {
    let mut box_pair_distances = {
        let mut vector = Vec::new();

        for (index, box_1) in junction_boxes.iter().enumerate() {
            for box_2 in junction_boxes[(index + 1)..].iter() {
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
}

fn init_circuits<'a>(junction_boxes: &'a JunctionBoxes) -> Circuits<'a> {
    junction_boxes
        .iter()
        .map(|location| vec![location])
        .collect()
}

fn connect<'a>(
    junction_box_1: &JunctionBox,
    junction_box_2: &JunctionBox,
    circuits: &mut Circuits<'a>,
) {
    let index_of_circuit_with_junction_box_1 = circuits
        .iter()
        .position(|circuit| circuit.contains(&junction_box_1))
        .unwrap();
    let index_of_circuit_with_junction_box_2 = circuits
        .iter()
        .position(|circuit| circuit.contains(&junction_box_2))
        .unwrap();

    let are_junction_boxes_in_same_circuit =
        index_of_circuit_with_junction_box_1 == index_of_circuit_with_junction_box_2;

    if !are_junction_boxes_in_same_circuit {
        // Remove the second circuit from the vector
        let circuit_2 = circuits.remove(index_of_circuit_with_junction_box_2);

        // Adjust index_1 if needed (if index_2 was before index_1)
        let adjusted_index_1 =
            if index_of_circuit_with_junction_box_2 < index_of_circuit_with_junction_box_1 {
                index_of_circuit_with_junction_box_1 - 1
            } else {
                index_of_circuit_with_junction_box_1
            };

        // Extend the first circuit with elements from the second
        circuits[adjusted_index_1].extend(circuit_2);
    }
}

pub mod part_1 {
    use super::*;

    fn find_sizes_of_3_largest_circuits_from_n_connections(input: &str, n_connections: u64) -> u64 {
        // 1. Parse the input to get all of the junction box locations.
        let junction_boxes = parse_junction_boxes(input);

        // 2. Make an ordered list of all the closest pairs of junction boxes.
        let sorted_box_pair_distances = create_sorted_box_pair_distances(&junction_boxes);

        let mut circuits = init_circuits(&junction_boxes);

        // 3. Iterate through the closest pairs list the given number of times.
        for i in 0..(n_connections as usize) {
            let PairDistance { pair, .. } = &sorted_box_pair_distances[i];
            // a. connect each pair.
            connect(pair.0, pair.1, &mut circuits);
        }

        // 4. Find the 3 largest circuits and multiply them. Return this value
        let mut sizes = 1;
        let sorted_circuit_lengths = {
            let mut circuit_lengths: Vec<usize> =
                circuits.iter().map(|circuit| circuit.len()).collect();
            circuit_lengths.sort();
            circuit_lengths.reverse();

            circuit_lengths
        };
        for i in 0..3 {
            let circuit_length = sorted_circuit_lengths[i];
            sizes *= circuit_length;
        }

        sizes as u64
    }

    pub fn solve(input: &str) -> u64 {
        find_sizes_of_3_largest_circuits_from_n_connections(input, 1000)
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

pub mod part_2 {
    use std::u64::MAX;

    use super::*;

    fn find_distance_for_single_circuit(input: &str) -> u64 {
        let junction_boxes = parse_junction_boxes(input);
        let sorted_box_pair_distances = create_sorted_box_pair_distances(&junction_boxes);
        let mut circuits = init_circuits(&junction_boxes);

        for PairDistance { pair, .. } in sorted_box_pair_distances {
            connect(pair.0, pair.1, &mut circuits);

            let is_single_circuit = circuits.len() == 1;
            if is_single_circuit {
                let distance_of_junction_boxes_that_form_single_circuit = pair.0.0 * pair.1.0;
                return distance_of_junction_boxes_that_form_single_circuit;
            }
        }

        return MAX;
    }

    pub fn solve(input: &str) -> u64 {
        find_distance_for_single_circuit(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn find_distance_for_single_circuit_works() {
            let input = include_str!("./test_input.txt");
            assert_eq!(find_distance_for_single_circuit(input), 25272);
        }
    }
}
