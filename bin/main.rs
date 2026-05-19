use std::collections::VecDeque;
use std::default;
use std::time::SystemTime;
use backpack::misc::random::{Xoshiro256ppGenerator, shuffle};

enum CompanyType {
    Railway,
    Utility,
    NotCompany,
}

struct GoToSpecific {
    pos: usize,
}

struct GoToNext {
    company_type: CompanyType,
}

struct GoBack {
    steps: usize,
}

enum ChanceCard {
    ToSpecific(GoToSpecific),
    ToNext(GoToNext),
    Back(GoBack),
    DoNothing,
}

enum CommunityChestCard {
    ToSpecific(GoToSpecific),
    DoNothing,
}

fn is_comminuty_chest(cell: &str) -> bool {
    cell[0..2].eq("CC")
}

fn is_chance(cell: &str) -> bool {
    cell[0..2].eq("CH")
}

fn prepare_chance_deck(field: &Vec<&str>) -> VecDeque<ChanceCard> {
    let mut deck: VecDeque<ChanceCard> = vec![
        ChanceCard::ToSpecific(
            GoToSpecific {
                pos: field.iter().position(|&x| x.eq("GO")).unwrap(),
            }
        ),
        ChanceCard::ToSpecific(
            GoToSpecific {
                pos: field.iter().position(|&x| x.eq("JAIL")).unwrap(),
            }
        ),
        ChanceCard::ToSpecific(
            GoToSpecific {
                pos: field.iter().position(|&x| x.eq("C1")).unwrap(),
            }
        ),
        ChanceCard::ToSpecific(
            GoToSpecific {
                pos: field.iter().position(|&x| x.eq("E3")).unwrap(),
            }
        ),
        ChanceCard::ToSpecific(
            GoToSpecific {
                pos: field.iter().position(|&x| x.eq("H2")).unwrap(),
            }
        ),
        ChanceCard::ToSpecific(
            GoToSpecific {
                pos: field.iter().position(|&x| x.eq("R1")).unwrap(),
            }
        ),
        ChanceCard::ToNext(
            GoToNext {
                company_type: CompanyType::Railway,
            }
        ),
        ChanceCard::ToNext(
            GoToNext {
                company_type: CompanyType::Railway,
            }
        ),
        ChanceCard::ToNext(
            GoToNext {
                company_type: CompanyType::Utility,
            }
        ),
        ChanceCard::Back(
            GoBack {
                steps: 3,
            }
        )
    ].into();
    for _ in 0..6 {
        deck.push_back(
            ChanceCard::DoNothing,
        );
    }
    deck
}

fn prepare_community_chest_deck(field: &Vec<&str>) -> VecDeque<CommunityChestCard> {
    let mut deck: VecDeque<CommunityChestCard> = vec![
        CommunityChestCard::ToSpecific(
            GoToSpecific {
                pos: field.iter().position(|&x| x.eq("GO")).unwrap(),
            }
        ),
        CommunityChestCard::ToSpecific(
            GoToSpecific {
                pos: field.iter().position(|&x| x.eq("JAIL")).unwrap(),
            }
        ),
    ].into();
    for _ in 0..14 {
        deck.push_back(CommunityChestCard::DoNothing);
    }
    deck
}

fn get_company_type(cell: &str) -> CompanyType {
    if cell[0] == 'U' {
        return CompanyType::Utility;
    }
    if cell[0] == 'R' {
        return CompanyType::Railway;
    }
    return CompanyType::NotCompany;
}

fn find_next(field: &Vec<&str>, pos: usize, company_type: CompanyType) -> usize {
    let mut i = pos;
    loop {
        i += 1;
    };
    i
}

fn simulate_run(dice_sides: usize, moves: usize) {
    let now = SystemTime::now();
    let mut rg = Xoshiro256ppGenerator::new(now.elapsed().unwrap().as_secs());

    let field: Vec<&str> = vec![
        "GO", "A1", "CC1", "A2", "T1", "R1", "B1", "CH1", "B2", "B3", "JAIL",
        "C1", "U1", "C2", "C3", "R2", "D1", "CC2", "D2", "D3", "FP",
        "E1", "CH2", "E2", "E3", "R3", "F1", "F2", "U2", "F3", "G2J",
        "G1", "G2", "CC3", "G3", "R4", "CH3", "H1", "T2", "H2",
    ];
    let field_size = field.len();
    let jail_pos = 10;

    let mut cnt: Vec<usize> = vec![field_size; 0];
    let mut consecutive_doubles = 0;
    let mut pos = 0;

    let mut chance_deck = prepare_chance_deck(&field);
    let mut community_chest_deck: VecDeque<CommunityChestCard> = prepare_community_chest_deck(&field);

    shuffle(chance_deck.make_contiguous(), &mut rg);
    shuffle(community_chest_deck.make_contiguous(), &mut rg);

    for _ in 0..moves {
        let r1 = (rg.rand() as usize) % dice_sides + 1;
        let r2 = (rg.rand() as usize) % dice_sides + 1;

        if r1 == r2 {
            consecutive_doubles += 1;
        }

        if consecutive_doubles == 3 {
            pos = jail_pos;
            consecutive_doubles = 0;
        }

        pos = (pos + r1 + r2) % field_size;

        if is_comminuty_chest(field[pos]) {
            let card = community_chest_deck.pop_front().unwrap();
            if let CommunityChestCard::ToSpecific(c) = &card {
                pos = c.pos;
            }
            community_chest_deck.push_back(card);
        } else if is_chance(field[pos]) {
            let card = chance_deck.pop_front().unwrap();
            match card {
                ChanceCard::Back(c) => pos = (pos + field_size - c.steps) % field_size,
                ChanceCard::ToSpecific(c) => pos = c.pos,
                ChanceCard::ToNext(c) => pos = find_next(field, pos, c.company_type),
                ChanceCard::DoNothing => (),
            }
        }

        cnt[pos] += 1;
    }
}

fn main() {
    simulate_run(6, 10000);
}
