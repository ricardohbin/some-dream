const PLAYER: &str = "P";

pub struct MapOptions {
    pub minimap: String,
    pub description: String,
    pub directions: Vec<String>,
    pub x: usize,
    pub y: usize,
    pub index: usize,
}

//TODO: global lazy_static 
//TODO2: better handling of multilines - files?
fn get_map_description(index: usize) -> &'static str {
    vec!("
        This is you starting point. You are seeing a wide open corridor. There are paints over the walls. \nYou look at the windows, there is a red sky outside.\nYou only have an option, straight forward.
    ",
    "This is the second room! Description soon....",
    "This is a wide open area. Feel free to explore.")[index]
}

fn clean_white_spaces(value: &str) -> String {
    value.to_string().replace(" ", "")
}

fn get_map() -> Vec<&'static str> {
    return vec!(
        //map 0
        "##############
         #............1
         #.############
         #.#
         #.#
         #.#
         #.#
         #.#
         ###",
         // map 1
        "#############
         0...........#
         #........?..2
         #############",
         // map 2
        "##########################################
         #........................................#
         #........................................#
         #...........M..............C.............#
         #........................................#
         #.....D..................................#
         1........................................#
         ##########################################
        "
    );
}

fn get_map_points(index: usize, is_returning: bool) -> (usize, usize) {
    match (index, is_returning) {
        (0, false) => (1, 7),
        (0, true) => (12, 1),
        (1, false) => (1, 1),
        (1, true) => (11, 2),
        (2, false) => (1, 6),
        _ => panic!("This can't happen in start point")
    }
}

pub fn point(index: usize, x: usize, y: usize) -> MapOptions {
    // Ignoring first `"` in split
    // TODO: better split to this, to use x as is
    let column = x + 1;

    let map: Vec<&str> = get_map();

    // removing extra spaces
    let map_string = clean_white_spaces(map[index]);

    // note: char '\n' instead of "\n" is proposital
    let mut lines: Vec<&str> = map_string.split('\n').collect();

    let mut columns: Vec<&str> = lines[y].split("").collect();

    
    //check if there's an interaction before. Only numbers that means rooms
    if &lines[y][x..column] != "#" && &lines[y][x..column] != "." {
        let result_change_map: Result<usize, std::num::ParseIntError> = lines[y][x..column].parse::<usize>();

        if let Ok(map_index) = result_change_map {
            let map_points = get_map_points(map_index, index > map_index);
            return point(map_index, map_points.0, map_points.1);
        }
    }
    
    columns[column] = PLAYER;

    // creating a binding to avoid temporary value dropped while borrowed
    let new_line = columns.join("");
    lines[y] = new_line.as_str();
    let new_map = lines.join("\n");

    let mut directions: Vec<String> = vec!();

    // checking directions and points of interest
    // TODO: put this in a function
    if columns[column - 1] != "#" {
        directions.push("e".to_string());
    }

    if columns[column + 1] != "#" {
        directions.push("w".to_string());
    }

    if &lines[y + 1][x..column] != "#" {
        directions.push("s".to_string());
    }

    if &lines[y - 1][x..column] != "#" {
        directions.push("n".to_string());
    }

    MapOptions{
        minimap: new_map,
        description: get_map_description(index).trim().to_string(),
        directions,
        x,
        y,
        index,
    }
}