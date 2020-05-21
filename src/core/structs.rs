pub struct Stats {
    // Countries
    // Date
    // Delta downloads
    // Downloads
    // Flatpak versions
    // OSTree versions
    // Refs
    // Updates
}

pub struct Refs {
    pub refs: Vec<Ref>
}

pub struct Ref {
    pub appid: &'static str
}