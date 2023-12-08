use std::collections::HashMap;
use std::fs::read_to_string;
use std::panic::panic_any;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfKind = 4,
    FullHouse = 5,
    FourOfKind = 6,
    FiveOfKind = 7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum CardType {
    TypeJ = 1,
    Type2 = 2,
    Type3 = 3,
    Type4 = 4,
    Type5 = 5,
    Type6 = 6,
    Type7 = 7,
    Type8 = 8,
    Type9 = 9,
    TypeT = 10,
    // TypeJ = 11,
    TypeQ = 12,
    TypeK = 13,
    TypeA = 14,
}

#[derive(Debug)]
struct Hand {
    card_type: HandType,
    cards: [CardType; 5],
    bid: i32,
}

pub fn solve1() {
    let file_str = read_to_string("src/day7/input.txt").unwrap();
    let mut hands: Vec<Hand> = Vec::new();

    for line in file_str.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut contacts: HashMap<CardType, i32> = HashMap::new();

        let mut hand = Hand {
            card_type: HandType::FiveOfKind,
            cards: std::array::from_fn(|_| CardType::Type2),
            bid: parts[1].parse::<i32>().unwrap(),
        };

        for (index, ch) in parts[0].chars().enumerate() {
            let ct: CardType = match ch {
                '2' => CardType::Type2,
                '3' => CardType::Type3,
                '4' => CardType::Type4,
                '5' => CardType::Type5,
                '6' => CardType::Type6,
                '7' => CardType::Type7,
                '8' => CardType::Type8,
                '9' => CardType::Type9,
                'T' => CardType::TypeT,
                'J' => CardType::TypeJ,
                'Q' => CardType::TypeQ,
                'K' => CardType::TypeK,
                'A' => CardType::TypeA,
                _ => panic_any(ch),
            };
            hand.cards[index] = ct;

            *contacts.entry(ct).or_insert(0) += 1;
        }

        if contacts.len() == 1 {
            hand.card_type = HandType::FiveOfKind;
        } else if contacts.len() == 2 {
            let f = 4;
            let has_four_of_kind = contacts.iter().any(|(_, c)| c.eq(&f));

            if has_four_of_kind {
                hand.card_type = HandType::FourOfKind;
            } else {
                hand.card_type = HandType::FullHouse;
            }
        } else if contacts.len() == 3 {
            let f = 3;
            let has_three_of_kind = contacts.iter().any(|(_, c)| c.eq(&f));

            if has_three_of_kind {
                hand.card_type = HandType::ThreeOfKind;
            } else {
                hand.card_type = HandType::TwoPair;
            }
        } else if contacts.len() == 4 {
            hand.card_type = HandType::OnePair;
        } else {
            hand.card_type = HandType::HighCard;
        }

        hands.push(hand);
    }

    hands.sort_unstable_by_key(|item| (item.card_type, item.cards));

    let mut result: i32 = 0;
    for (i, h) in hands.iter().enumerate() {
        result += ((i as i32) + 1) * h.bid;
    }

    println!("{:?}", result)
}

pub fn solve2() {
    let file_str = read_to_string("src/day7/input.txt").unwrap();
    let mut hands: Vec<Hand> = Vec::new();

    for line in file_str.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut contacts: HashMap<CardType, i32> = HashMap::new();

        let mut hand = Hand {
            card_type: HandType::FiveOfKind,
            cards: std::array::from_fn(|_| CardType::Type2),
            bid: parts[1].parse::<i32>().unwrap(),
        };

        for (index, ch) in parts[0].chars().enumerate() {
            let ct: CardType = match ch {
                'J' => CardType::TypeJ,
                '2' => CardType::Type2,
                '3' => CardType::Type3,
                '4' => CardType::Type4,
                '5' => CardType::Type5,
                '6' => CardType::Type6,
                '7' => CardType::Type7,
                '8' => CardType::Type8,
                '9' => CardType::Type9,
                'T' => CardType::TypeT,
                // 'J' => CardType::TypeJ,
                'Q' => CardType::TypeQ,
                'K' => CardType::TypeK,
                'A' => CardType::TypeA,
                _ => panic_any(ch),
            };
            hand.cards[index] = ct;

            *contacts.entry(ct).or_insert(0) += 1;
        }

        let count_j: Option<&i32> = contacts.get(&CardType::TypeJ);

        if contacts.len() == 1 {
            hand.card_type = HandType::FiveOfKind;
        } else if contacts.len() == 2 {
            let f = 4;
            let has_four_of_kind = contacts.iter().any(|(_, c)| c.eq(&f));

            // JJJJ1
            // 1111J
            // 11112
            // 111JJ
            // 11122
            // JJJ11

            if has_four_of_kind {
                match count_j {
                    None => {
                        hand.card_type = HandType::FourOfKind;
                    }
                    Some(_) => {
                        hand.card_type = HandType::FiveOfKind;
                    }
                }
            } else {
                match count_j {
                    None => {
                        hand.card_type = HandType::FullHouse;
                    }
                    Some(_) => {
                        hand.card_type = HandType::FiveOfKind;
                    }
                }
            }
        } else if contacts.len() == 3 {
            // T55J5, KTJJT, and QQQJA are now all four of a kind

            // JJJ12
            // 111J2
            // 11JJ2
            // 1122J

            // JJJ12
            // 111JJ
            // 111J2

            let f = 3;
            let has_three_of_kind = contacts.iter().any(|(_, c)| c.eq(&f));

            if has_three_of_kind {
                match count_j {
                    None => {
                        hand.card_type = HandType::ThreeOfKind;
                    }
                    Some(c) => {
                        if *c == 1 {
                            hand.card_type = HandType::FourOfKind;
                        } else if *c == 2 {
                            hand.card_type = HandType::FiveOfKind;
                        } else {
                            hand.card_type = HandType::FourOfKind;
                        }
                    }
                }
            } else {
                match count_j {
                    None => {
                        hand.card_type = HandType::TwoPair;
                    }
                    Some(c) => {
                        if *c == 1 {
                            hand.card_type = HandType::FullHouse;
                        } else {
                            hand.card_type = HandType::FourOfKind;
                        }
                    }
                }
            }
        } else if contacts.len() == 4 {
            // A23A4
            // A23AJ
            // J23J4

            match count_j {
                None => {
                    hand.card_type = HandType::OnePair;
                }
                Some(_) => {
                    hand.card_type = HandType::ThreeOfKind;
                }
            }
        } else {
            match count_j {
                None => {
                    hand.card_type = HandType::HighCard;
                }
                Some(_) => {
                    hand.card_type = HandType::OnePair;
                }
            }
        }

        hands.push(hand);
    }

    hands.sort_unstable_by_key(|item| (item.card_type, item.cards));

    let mut result: i32 = 0;
    for (i, h) in hands.iter().enumerate() {
        result += ((i as i32) + 1) * h.bid;
    }

    println!("{:?}", result)
}
