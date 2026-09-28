use std::collections::HashMap;

struct PackedPeriodCol(u8);

impl PackedPeriodCol {
    fn new(period: u8, column: u8) -> Self {
        if period > 7 || column > 18 {
            panic!("Invalid period or column.");
        }

        // 000    00000
        // period column
        PackedPeriodCol(period << 5 | column)
    }

    fn get_period(&self) -> u8 {
        (self.0 & (0b111 << 5)) >> 5
    }

    fn get_column(&self) -> u8 {
        self.0 & 0b11111
    }
}

enum ElementType {
    Alkali,
    AlkalineEarth,
    Transition,
    PostTransition,
    Metalloid,
    NonMetal,
    Halogen,
    Noble,
    Lanthanoid,
    Actinoid,
}

use ElementType::*;

struct ElementData {
    name: &'static str,
    number: i8,
    molar_mass: f32,
    ionic_charge: Option<i8>,
    period_colum: PackedPeriodCol,
    kind: ElementType,
}

fn main() {
    let input = std::env::args().skip(1).collect::<String>();
    let mut elements: HashMap<&str, ElementData> = HashMap::new();

    elements.insert(
        "H",
        ElementData {
            name: "Hydrogen",
            number: 1,
            molar_mass: 1.008,
            ionic_charge: Some(1),
            period_colum: PackedPeriodCol::new(1, 1),
            kind: NonMetal,
        },
    );

    elements.insert(
        "He",
        ElementData {
            name: "Helium",
            number: 2,
            molar_mass: 4.003,
            ionic_charge: Some(0),
            period_colum: PackedPeriodCol::new(1, 18),
            kind: Noble,
        },
    );
}
