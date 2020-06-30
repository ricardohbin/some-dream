#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub strength: i8,
    pub agility: i8,
    pub intelligence: i8,
    pub will: i8,
    pub charisma: i8,
    pub intimidation: i8,
    pub wealth: i8,
    pub resistence: i8,
}

#[derive(Debug, Clone, Copy)]
pub struct VitalPoints {
    pub life: i8,
    pub luck: i8,
    pub cardio: i8,
    pub social: i8,
}