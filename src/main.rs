use std::collections::HashMap;

struct PackedPeriodCol(u8);

enum IonicCharge {
    Single(i8),
    Multiple(&'static [i8])
}

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
    ionic_charge: IonicCharge,
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
            ionic_charge: IonicCharge::Single(1),
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
            ionic_charge: IonicCharge::Single(0),
            period_colum: PackedPeriodCol::new(1, 18),
            kind: Noble,
        },
    );

    elements.insert(
        "Li",
        ElementData {
            name: "Lithium",
            number: 3,
            molar_mass: 6.941,
            ionic_charge: IonicCharge::Single(1),
            period_colum: PackedPeriodCol::new(2, 1),
            kind: Alkali,
        },
    );

    elements.insert(
        "Be",
        ElementData {
            name: "Beryllium",
            number: 4,
            molar_mass: 9.012,
            ionic_charge: IonicCharge::Single(2),
            period_colum: PackedPeriodCol::new(2, 2),
            kind: AlkalineEarth,
        },
    );

    elements.insert(
        "B",
        ElementData {
            name: "Boron",
            number: 5,
            molar_mass: 10.811,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(2, 13),
            kind: Metalloid,
        },
    );

    elements.insert(
        "C",
        ElementData {
            name: "Carbon",
            number: 6,
            molar_mass: 12.011,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(2, 14),
            kind: NonMetal,
        },
    );
    elements.insert(
        "N",
        ElementData {
            name: "Nitrogen",
            number: 7,
            molar_mass: 14.007,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(2, 15),
            kind: NonMetal,
        },
    );
    elements.insert(
        "O",
        ElementData {
            name: "Oxygen",
            number: 8,
            molar_mass: 15.999,
            ionic_charge: IonicCharge::Single(-2),
            period_colum: PackedPeriodCol::new(2, 16),
            kind: NonMetal,
        },
    );
    elements.insert(
        "F",
        ElementData {
            name: "Fluorine",
            number: 9,
            molar_mass: 18.998,
            ionic_charge: IonicCharge::Single(-1),
            period_colum: PackedPeriodCol::new(2, 17),
            kind: Halogen,
        },
    );
    elements.insert(
        "Ne",
        ElementData {
            name: "Neon",
            number: 10,
            molar_mass: 20.180,
            ionic_charge: IonicCharge::Single(0),
            period_colum: PackedPeriodCol::new(2, 18),
            kind: Noble,
        },
    );
    elements.insert(
        "Na",
        ElementData {
            name: "Sodium",
            number: 11,
            molar_mass: 22.990,
            ionic_charge: IonicCharge::Single(1),
            period_colum: PackedPeriodCol::new(3, 1),
            kind: Alkali,
        },
    );
    elements.insert(
        "Mg",
        ElementData {
            name: "Magnesium",
            number: 12,
            molar_mass: 24.305,
            ionic_charge: IonicCharge::Single(2),
            period_colum: PackedPeriodCol::new(3, 2),
            kind: AlkalineEarth,
        },
    );
    elements.insert(
        "Al",
        ElementData {
            name: "Aluminum",
            number: 13,
            molar_mass: 26.982,
            ionic_charge: IonicCharge::Single(3),
            period_colum: PackedPeriodCol::new(3, 13),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Si",
        ElementData {
            name: "Silicon",
            number: 14,
            molar_mass: 28.086,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(3, 14),
            kind: Metalloid,
        },
    );
    elements.insert(
        "P",
        ElementData {
            name: "Phosphorus",
            number: 15,
            molar_mass: 30.974,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(3, 15),
            kind: NonMetal,
        },
    );
    elements.insert(
        "S",
        ElementData {
            name: "Sulfur",
            number: 16,
            molar_mass: 32.066,
            ionic_charge: IonicCharge::Single(-2),
            period_colum: PackedPeriodCol::new(3, 16),
            kind: NonMetal,
        },
    );
    elements.insert(
        "Cl",
        ElementData {
            name: "Chlorine",
            number: 17,
            molar_mass: 35.453,
            ionic_charge: IonicCharge::Single(-1),
            period_colum: PackedPeriodCol::new(3, 17),
            kind: Halogen,
        },
    );
    elements.insert(
        "Ar",
        ElementData {
            name: "Argon",
            number: 18,
            molar_mass: 39.948,
            ionic_charge: IonicCharge::Single(0),
            period_colum: PackedPeriodCol::new(3, 18),
            kind: Noble,
        },
    );

    elements.insert(
        "K",
        ElementData {
            name: "Potassium",
            number: 19,
            molar_mass: 39.098,
            ionic_charge: IonicCharge::Single(1),
            period_colum: PackedPeriodCol::new(4, 1),
            kind: Alkali,
        },
    );
    elements.insert(
        "Ca",
        ElementData {
            name: "Calcium",
            number: 20,
            molar_mass: 40.078,
            ionic_charge: IonicCharge::Single(2),
            period_colum: PackedPeriodCol::new(4, 2),
            kind: AlkalineEarth,
        },
    );
    elements.insert(
        "Sc",
        ElementData {
            name: "Scandium",
            number: 21,
            molar_mass: 44.956,
            ionic_charge: IonicCharge::Single(3),
            period_colum: PackedPeriodCol::new(4, 3),
            kind: Transition,
        },
    );
    elements.insert(
        "Ti",
        ElementData {
            name: "Titanium",
            number: 22,
            molar_mass: 47.867,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 4),
            kind: Transition,
        },
    );
    elements.insert(
        "V",
        ElementData {
            name: "Vanadium",
            number: 23,
            molar_mass: 50.942,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 5),
            kind: Transition,
        },
    );
    elements.insert(
        "Cr",
        ElementData {
            name: "Chromium",
            number: 24,
            molar_mass: 51.996,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 6),
            kind: Transition,
        },
    );
    elements.insert(
        "Mn",
        ElementData {
            name: "Manganese",
            number: 25,
            molar_mass: 54.938,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 7),
            kind: Transition,
        },
    );
    elements.insert(
        "Fe",
        ElementData {
            name: "Iron",
            number: 26,
            molar_mass: 55.845,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 8),
            kind: Transition,
        },
    );
    elements.insert(
        "Co",
        ElementData {
            name: "Cobalt",
            number: 27,
            molar_mass: 58.933,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 9),
            kind: Transition,
        },
    );
    elements.insert(
        "Ni",
        ElementData {
            name: "Nickel",
            number: 28,
            molar_mass: 58.693,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 10),
            kind: Transition,
        },
    );
    elements.insert(
        "Cu",
        ElementData {
            name: "Copper",
            number: 29,
            molar_mass: 63.546,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 11),
            kind: Transition,
        },
    );
    elements.insert(
        "Zn",
        ElementData {
            name: "Zinc",
            number: 30,
            molar_mass: 65.38,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 12),
            kind: Transition,
        },
    );
    elements.insert(
        "Ga",
        ElementData {
            name: "Gallium",
            number: 31,
            molar_mass: 69.723,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 13),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Ge",
        ElementData {
            name: "Germanium",
            number: 32,
            molar_mass: 72.631,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 14),
            kind: Metalloid,
        },
    );
    elements.insert(
        "As",
        ElementData {
            name: "Arsenic",
            number: 33,
            molar_mass: 74.922,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(4, 15),
            kind: Metalloid,
        },
    );
    elements.insert(
        "Se",
        ElementData {
            name: "Selenium",
            number: 34,
            molar_mass: 78.971,
            ionic_charge: IonicCharge::Single(-2),
            period_colum: PackedPeriodCol::new(4, 16),
            kind: NonMetal,
        },
    );
    elements.insert(
        "Br",
        ElementData {
            name: "Bromine",
            number: 35,
            molar_mass: 79.904,
            ionic_charge: IonicCharge::Single(-1),
            period_colum: PackedPeriodCol::new(4, 17),
            kind: Halogen,
        },
    );
    elements.insert(
        "Kr",
        ElementData {
            name: "Krypton",
            number: 36,
            molar_mass: 83.798,
            ionic_charge: IonicCharge::Single(0),
            period_colum: PackedPeriodCol::new(4, 18),
            kind: Noble,
        },
    );

    elements.insert(
        "Rb",
        ElementData {
            name: "Rubidium",
            number: 37,
            molar_mass: 85.468,
            ionic_charge: IonicCharge::Single(1),
            period_colum: PackedPeriodCol::new(5, 1),
            kind: Alkali,
        },
    );
    elements.insert(
        "Sr",
        ElementData {
            name: "Strontium",
            number: 38,
            molar_mass: 87.62,
            ionic_charge: IonicCharge::Single(2),
            period_colum: PackedPeriodCol::new(5, 2),
            kind: AlkalineEarth,
        },
    );
    elements.insert(
        "Y",
        ElementData {
            name: "Yttrium",
            number: 39,
            molar_mass: 88.906,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 3),
            kind: Transition,
        },
    );
    elements.insert(
        "Zr",
        ElementData {
            name: "Zirconium",
            number: 40,
            molar_mass: 91.224,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 4),
            kind: Transition,
        },
    );
    elements.insert(
        "Nb",
        ElementData {
            name: "Niobium",
            number: 41,
            molar_mass: 92.906,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 5),
            kind: Transition,
        },
    );
    elements.insert(
        "Mo",
        ElementData {
            name: "Molybdenum",
            number: 42,
            molar_mass: 95.95,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 6),
            kind: Transition,
        },
    );
    elements.insert(
        "Tc",
        ElementData {
            name: "Technetium",
            number: 43,
            molar_mass: 98.907,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 7),
            kind: Transition,
        },
    );
    elements.insert(
        "Ru",
        ElementData {
            name: "Ruthenium",
            number: 44,
            molar_mass: 101.07,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 8),
            kind: Transition,
        },
    );
    elements.insert(
        "Rh",
        ElementData {
            name: "Rhodium",
            number: 45,
            molar_mass: 102.906,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 9),
            kind: Transition,
        },
    );
    elements.insert(
        "Pd",
        ElementData {
            name: "Palladium",
            number: 46,
            molar_mass: 106.42,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 10),
            kind: Transition,
        },
    );
    elements.insert(
        "Ag",
        ElementData {
            name: "Silver",
            number: 47,
            molar_mass: 107.868,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 11),
            kind: Transition,
        },
    );
    elements.insert(
        "Cd",
        ElementData {
            name: "Cadmium",
            number: 48,
            molar_mass: 112.414,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 12),
            kind: Transition,
        },
    );
    elements.insert(
        "In",
        ElementData {
            name: "Indium",
            number: 49,
            molar_mass: 114.818,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 13),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Sn",
        ElementData {
            name: "Tin",
            number: 50,
            molar_mass: 118.711,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 14),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Sb",
        ElementData {
            name: "Antimony",
            number: 51,
            molar_mass: 121.760,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(5, 15),
            kind: Metalloid,
        },
    );
    elements.insert(
        "Te",
        ElementData {
            name: "Tellurium",
            number: 52,
            molar_mass: 127.6,
            ionic_charge: IonicCharge::Single(-2),
            period_colum: PackedPeriodCol::new(5, 16),
            kind: Metalloid,
        },
    );
    elements.insert(
        "I",
        ElementData {
            name: "Iodine",
            number: 53,
            molar_mass: 126.904,
            ionic_charge: IonicCharge::Single(-1),
            period_colum: PackedPeriodCol::new(5, 17),
            kind: Halogen,
        },
    );
    elements.insert(
        "Xe",
        ElementData {
            name: "Xenon",
            number: 54,
            molar_mass: 131.294,
            ionic_charge: IonicCharge::Single(0),
            period_colum: PackedPeriodCol::new(5, 18),
            kind: Noble,
        },
    );

    elements.insert(
        "Cs",
        ElementData {
            name: "Cesium",
            number: 55,
            molar_mass: 132.905,
            ionic_charge: IonicCharge::Single(1),
            period_colum: PackedPeriodCol::new(6, 1),
            kind: Alkali,
        },
    );
    elements.insert(
        "Ba",
        ElementData {
            name: "Barium",
            number: 56,
            molar_mass: 137.328,
            ionic_charge: IonicCharge::Single(2),
            period_colum: PackedPeriodCol::new(6, 2),
            kind: AlkalineEarth,
        },
    );
    elements.insert(
        "La",
        ElementData {
            name: "Lanthanum",
            number: 57,
            molar_mass: 138.905,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Ce",
        ElementData {
            name: "Cerium",
            number: 58,
            molar_mass: 140.116,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Pr",
        ElementData {
            name: "Praseodymium",
            number: 59,
            molar_mass: 140.908,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Nd",
        ElementData {
            name: "Neodymium",
            number: 60,
            molar_mass: 144.243,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Pm",
        ElementData {
            name: "Promethium",
            number: 61,
            molar_mass: 144.913,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Sm",
        ElementData {
            name: "Samarium",
            number: 62,
            molar_mass: 150.36,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Eu",
        ElementData {
            name: "Europium",
            number: 63,
            molar_mass: 151.964,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Gd",
        ElementData {
            name: "Gadolinium",
            number: 64,
            molar_mass: 157.25,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Tb",
        ElementData {
            name: "Terbium",
            number: 65,
            molar_mass: 158.925,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Dy",
        ElementData {
            name: "Dysprosium",
            number: 66,
            molar_mass: 162.500,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Ho",
        ElementData {
            name: "Holmium",
            number: 67,
            molar_mass: 164.930,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Er",
        ElementData {
            name: "Erbium",
            number: 68,
            molar_mass: 167.259,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Tm",
        ElementData {
            name: "Thalium",
            number: 69,
            molar_mass: 168.934,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Yb",
        ElementData {
            name: "Ytterbium",
            number: 70,
            molar_mass: 173.055,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Lu",
        ElementData {
            name: "Lutetium",
            number: 71,
            molar_mass: 174.967,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 3),
            kind: Lanthanoid,
        },
    );
    elements.insert(
        "Hf",
        ElementData {
            name: "Hafnium",
            number: 72,
            molar_mass: 178.49,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 4),
            kind: Transition,
        },
    );
    elements.insert(
        "Ta",
        ElementData {
            name: "Tantalum",
            number: 73,
            molar_mass: 180.948,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 5),
            kind: Transition,
        },
    );
    elements.insert(
        "W",
        ElementData {
            name: "Tungsten",
            number: 74,
            molar_mass: 183.84,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 6),
            kind: Transition,
        },
    );
    elements.insert(
        "Re",
        ElementData {
            name: "Rhenium",
            number: 75,
            molar_mass: 186.207,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 7),
            kind: Transition,
        },
    );
    elements.insert(
        "Os",
        ElementData {
            name: "Osmium",
            number: 76,
            molar_mass: 190.23,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 8),
            kind: Transition,
        },
    );
    elements.insert(
        "Ir",
        ElementData {
            name: "Iridium",
            number: 77,
            molar_mass: 192.217,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 9),
            kind: Transition,
        },
    );
    elements.insert(
        "Pt",
        ElementData {
            name: "Platinum",
            number: 78,
            molar_mass: 195.085,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 10),
            kind: Transition,
        },
    );
    elements.insert(
        "Au",
        ElementData {
            name: "Gold",
            number: 79,
            molar_mass: 196.967,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 11),
            kind: Transition,
        },
    );
    elements.insert(
        "Hg",
        ElementData {
            name: "Mercury",
            number: 80,
            molar_mass: 200.592,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 12),
            kind: Transition,
        },
    );
    elements.insert(
        "Tl",
        ElementData {
            name: "Thalium",
            number: 81,
            molar_mass: 204.383,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 13),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Pb",
        ElementData {
            name: "Lead",
            number: 82,
            molar_mass: 207.2,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 14),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Bi",
        ElementData {
            name: "Bismuth",
            number: 83,
            molar_mass: 208.980,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(6, 15),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Po",
        ElementData {
            name: "Polonium",
            number: 84,
            molar_mass: 208.982,
            ionic_charge: IonicCharge::Single(-2),
            period_colum: PackedPeriodCol::new(6, 16),
            kind: PostTransition,
        },
    );
    elements.insert(
        "At",
        ElementData {
            name: "Astitine",
            number: 85,
            molar_mass: 209.987,
            ionic_charge: IonicCharge::Single(-1),
            period_colum: PackedPeriodCol::new(6, 17),
            kind: Halogen,
        },
    );
    elements.insert(
        "Rn",
        ElementData {
            name: "Radon",
            number: 86,
            molar_mass: 222.018,
            ionic_charge: IonicCharge::Single(0),
            period_colum: PackedPeriodCol::new(6, 18),
            kind: Noble,
        },
    );

    elements.insert(
        "Fr",
        ElementData {
            name: "Francium",
            number: 87,
            molar_mass: 223.020,
            ionic_charge: IonicCharge::Single(1),
            period_colum: PackedPeriodCol::new(7, 1),
            kind: Alkali,
        },
    );
    elements.insert(
        "Ra",
        ElementData {
            name: "Radium",
            number: 88,
            molar_mass: 226.025,
            ionic_charge: IonicCharge::Single(2),
            period_colum: PackedPeriodCol::new(7, 2),
            kind: AlkalineEarth,
        },
    );
    elements.insert(
        "Ac",
        ElementData {
            name: "Actinium",
            number: 89,
            molar_mass: 227.028,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Th",
        ElementData {
            name: "Thorium",
            number: 90,
            molar_mass: 232.038,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Pa",
        ElementData {
            name: "Protactinium",
            number: 91,
            molar_mass: 231.036,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "U",
        ElementData {
            name: "Uranium",
            number: 92,
            molar_mass: 238.029,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Np",
        ElementData {
            name: "Neptunium",
            number: 93,
            molar_mass: 237.048,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Pu",
        ElementData {
            name: "Plutonium",
            number: 94,
            molar_mass: 244.064,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Am",
        ElementData {
            name: "Americium",
            number: 95,
            molar_mass: 243.061,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Cm",
        ElementData {
            name: "Curium",
            number: 96,
            molar_mass: 247.070,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Bk",
        ElementData {
            name: "Berkelium",
            number: 97,
            molar_mass: 247.070,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Cf",
        ElementData {
            name: "Californium",
            number: 98,
            molar_mass: 251.080,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Es",
        ElementData {
            name: "Einsteinium",
            number: 99,
            molar_mass: 254.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Fm",
        ElementData {
            name: "Fermium",
            number: 100,
            molar_mass: 257.095,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Md",
        ElementData {
            name: "Mendelevium",
            number: 101,
            molar_mass: 258.1,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "No",
        ElementData {
            name: "Nobelium",
            number: 102,
            molar_mass: 259.101,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Lr",
        ElementData {
            name: "Lawrencium",
            number: 103,
            molar_mass: 262.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 3),
            kind: Actinoid,
        },
    );
    elements.insert(
        "Rf",
        ElementData {
            name: "Rutherfordium",
            number: 104,
            molar_mass: 261.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 4),
            kind: Transition,
        },
    );
    elements.insert(
        "Db",
        ElementData {
            name: "Dubnium",
            number: 105,
            molar_mass: 262.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 5),
            kind: Transition,
        },
    );
    elements.insert(
        "Sg",
        ElementData {
            name: "Seaborgium",
            number: 106,
            molar_mass: 266.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 6),
            kind: Transition,
        },
    );
    elements.insert(
        "Bh",
        ElementData {
            name: "Bohrium",
            number: 107,
            molar_mass: 264.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 7),
            kind: Transition,
        },
    );
    elements.insert(
        "Hs",
        ElementData {
            name: "Hassium",
            number: 108,
            molar_mass: 269.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 8),
            kind: Transition,
        },
    );
    elements.insert(
        "Mt",
        ElementData {
            name: "Meitnerium",
            number: 109,
            molar_mass: 278.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 9),
            kind: Transition,
        },
    );
    elements.insert(
        "Ds",
        ElementData {
            name: "Darmstadium",
            number: 110,
            molar_mass: 281.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 10),
            kind: Transition,
        },
    );
    elements.insert(
        "Rg",
        ElementData {
            name: "Roentgenium",
            number: 111,
            molar_mass: 280.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 11),
            kind: Transition,
        },
    );
    elements.insert(
        "Cn",
        ElementData {
            name: "Copernicium",
            number: 112,
            molar_mass: 285.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 12),
            kind: Transition,
        },
    );
    elements.insert(
        "Nh",
        ElementData {
            name: "Nihonium",
            number: 113,
            molar_mass: 286.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 13),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Fl",
        ElementData {
            name: "Flerovium",
            number: 114,
            molar_mass: 289.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 14),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Mc",
        ElementData {
            name: "Moscovium",
            number: 115,
            molar_mass: 289.,
            ionic_charge: IonicCharge::Multiple(&[]),
            period_colum: PackedPeriodCol::new(7, 15),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Lv",
        ElementData {
            name: "Livermoriumm",
            number: 116,
            molar_mass: 293.,
            ionic_charge: IonicCharge::Single(-2),
            period_colum: PackedPeriodCol::new(7, 16),
            kind: PostTransition,
        },
    );
    elements.insert(
        "Ts",
        ElementData {
            name: "Tennessine",
            number: 117,
            molar_mass: 294.,
            ionic_charge: IonicCharge::Single(-2),
            period_colum: PackedPeriodCol::new(7, 17),
            kind: Halogen,
        },
    );
    elements.insert(
        "Og",
        ElementData {
            name: "Oganesson",
            number: 118,
            molar_mass: 294.,
            ionic_charge: IonicCharge::Single(0),
            period_colum: PackedPeriodCol::new(7, 18),
            kind: Noble,
        },
    );
}
