fn get_map() -> Vec<&'static str> {
    return vec!("##############
                 #............1
                 #.############
                 #.############
                 #.############
                 #.############
                 #.############
                 #.############
                 ##############");
}

pub fn add_to_map(map_index: usize, x: usize, y: usize) -> String {
    let map: Vec<&str> = get_map();
    // removing extra spaces
    let map_string = map[map_index].replace(" ", "");
    let mut lines: Vec<&str> = map_string.split("\n").collect();
    let mut rows: Vec<&str> = lines[y].split("").collect();
    // Ignoring first `"`
    rows[x+1] = "J";
    let new_line = rows.join("");
    lines[y] = new_line.as_str();
    let new_map = lines.join("\n");
    return new_map;
}