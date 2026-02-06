pub mod part_1 {
    type Coordinate = u64;
    type X = Coordinate;
    type Y = Coordinate;
    type RedTile = (X, Y);
    type RedTiles = Vec<RedTile>;
    type RedTilesReference<'a> = Vec<&'a RedTile>;

    /// Turn each line into a coordinate point.
    fn parse_tiles(input: &str) -> RedTiles {
        todo!()
    }

    /// Find the [convex hull](https://www.geeksforgeeks.org/dsa/convex-hull-using-graham-scan/) of the red tile points.
    fn find_outermost_red_tiles<'a>(red_tiles: &'a RedTiles) -> RedTilesReference<'a> {
        todo!()
    }

    /// Find the largest area made using two red tiles as opposite corners.
    fn find_largest_area(red_tiles: &RedTilesReference) -> u64 {
        todo!()
    }

    fn find_largest_area_from_raw_input(input: &str) -> u64 {
        let red_tiles = parse_tiles(input);
        let outermost_red_tiles = find_outermost_red_tiles(&red_tiles);
        let largest_area = find_largest_area(&outermost_red_tiles);
        largest_area
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn find_largest_area_works() {
            let input = include_str!("./test_input.txt");
            assert_eq!(find_largest_area_from_raw_input(input), 50);
        }
    }
}
