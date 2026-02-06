pub mod part_1 {
    type Coordinate = i64;
    type X = Coordinate;
    type Y = Coordinate;
    type RedTile = (X, Y);
    type RedTiles = Vec<RedTile>;
    type RedTileReferences<'a> = Vec<&'a RedTile>;

    /// Turn each line into a coordinate point.
    fn parse_tiles(input: &str) -> RedTiles {
        input
            .lines()
            .map(|line| {
                let mut parts = line
                    .split(",")
                    .map(|coordinate_str| coordinate_str.parse::<Coordinate>().unwrap());
                let x_coordinate = parts.next().unwrap();
                let y_coordinate = parts.next().unwrap();
                let red_tile = (x_coordinate, y_coordinate);

                red_tile
            })
            .collect()
    }

    /// Cross product of vectors (o -> a) and (o -> b).
    /// Positive = counter-clockwise, negative = clockwise, zero = collinear.
    fn cross(o: &RedTile, a: &RedTile, b: &RedTile) -> i64 {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    }

    /// Find the [convex hull](https://www.geeksforgeeks.org/dsa/convex-hull-using-graham-scan/) of the red tile points.
    fn find_outermost_red_tiles<'a>(red_tiles: &'a RedTiles) -> RedTileReferences<'a> {
        if red_tiles.len() <= 1 {
            return red_tiles.iter().collect();
        }

        // Find the point with the lowest y (and leftmost x as tiebreaker).
        let pivot_idx = red_tiles
            .iter()
            .enumerate()
            .min_by_key(|&(_, tile)| (tile.1, tile.0))
            .unwrap()
            .0;
        let pivot = &red_tiles[pivot_idx];

        // Collect references to all points, excluding the pivot.
        let mut points: RedTileReferences<'a> = red_tiles
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != pivot_idx)
            .map(|(_, tile)| tile)
            .collect();

        // Sort by polar angle with respect to the pivot.
        // For equal angles, sort by distance (closer first).
        points.sort_by(|a, b| {
            let c = cross(pivot, a, b);
            if c == 0 {
                // Collinear — sort by distance from pivot.
                let dist_a = (a.0 - pivot.0).pow(2) + (a.1 - pivot.1).pow(2);
                let dist_b = (b.0 - pivot.0).pow(2) + (b.1 - pivot.1).pow(2);
                dist_a.cmp(&dist_b)
            } else if c > 0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        // Build the hull using a stack.
        let mut hull: RedTileReferences<'a> = vec![pivot];
        for point in points {
            while hull.len() > 1 && cross(hull[hull.len() - 2], hull[hull.len() - 1], point) <= 0 {
                hull.pop();
            }
            hull.push(point);
        }

        hull
    }

    /// Find the largest area made using two red tiles as opposite corners.
    fn find_largest_area(red_tiles: &RedTileReferences) -> i64 {
        let mut largest_area = 0;
        // iterate through each pair of red tiles
        for (index, &tile_1) in red_tiles.iter().enumerate() {
            for &tile_2 in red_tiles[(index + 1)..].iter() {
                let area = {
                    let x_size = (tile_1.0 - tile_2.0 + 1).abs();
                    let y_size = (tile_1.1 - tile_2.1 + 1).abs();
                    x_size * y_size
                };
                if area > largest_area {
                    largest_area = area
                };
            }
        }

        largest_area
    }

    fn find_largest_area_from_raw_input(input: &str) -> i64 {
        let red_tiles = parse_tiles(input);
        let outermost_red_tiles = find_outermost_red_tiles(&red_tiles);
        let largest_area = find_largest_area(&outermost_red_tiles);
        largest_area
    }

    pub fn solve(input: &str) -> i64 {
        find_largest_area_from_raw_input(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn find_largest_area_from_raw_input_works() {
            let input = include_str!("./test_input.txt");
            assert_eq!(find_largest_area_from_raw_input(input), 50);
        }

        #[test]
        fn parse_tiles_works() {
            let input = "1,2\n3,4\n56,78\n9,10\n";
            let expected_red_tiles = vec![(1, 2), (3, 4), (56, 78), (9, 10)];
            assert_eq!(parse_tiles(input), expected_red_tiles);
        }

        #[test]
        fn find_outermost_red_tiles_works() {
            let red_tiles: RedTiles = vec![
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3),
            ];

            let expected_outermost_red_tiles: RedTileReferences = vec![
                &red_tiles[0],
                &red_tiles[1],
                &red_tiles[2],
                &red_tiles[3],
                &red_tiles[5],
                &red_tiles[6],
            ];

            assert_eq!(
                find_outermost_red_tiles(&red_tiles),
                expected_outermost_red_tiles
            );
        }
    }
}
