pub struct MapOptions {
    pub minimap: String,
    pub description: String,
    pub directions: Vec<String>,
}

//TODO: global lazy_static 
//TODO2: better handling of multilines - files?
fn get_map_description(index: usize) -> &'static str {
    return vec!("
        This is you starting point. You are seeing a wide open corridor. There are paints over the walls. \nYou look at the windows, there is a red sky outside.\nYou only have an option, straight forward.
    ")[index];
}

fn clean_white_spaces(value: &str) -> String {
    return value.to_string().replace(" ", "");
}

fn get_map() -> Vec<&'static str> {
    return vec!("##############
                 #............1
                 #.############
                 #.#
                 #.#
                 #.#
                 #.#
                 #.#
                 ###");
}

// fn scan_directions(x: usize, y: usize) -> bool {

// }

pub fn point(index: usize, x: usize, y: usize) -> MapOptions {
    // Ignoring first `"` in split
    // TODO: better split to this, to use x as is
    let column = x + 1;

    let map: Vec<&str> = get_map();
    // removing extra spaces
    let map_string = clean_white_spaces(map[index]);
    let mut lines: Vec<&str> = map_string.split("\n").collect();
    let mut columns: Vec<&str> = lines[y].split("").collect();
    
    columns[column] = "J";

    let new_line = columns.join("");
    lines[y] = new_line.as_str();
    let new_map = lines.join("\n");

    let mut directions: Vec<String> = vec!();

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

    return MapOptions{
        minimap: new_map,
        description: get_map_description(index).trim().to_string(),
        directions,
    }
}