//Part B calculation error - gives close results but has some error

use hashbrown::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
enum Dest<'a> {
    A,
    R,
    Name(&'a str),
}

fn to_dest<'a>(origin: &'a str) -> Dest<'a> {
    match origin {
        "A" => Dest::A,
        "R" => Dest::R,
        _ => Dest::Name(origin),
    }
}

#[derive(Debug, Clone, Copy)]
struct TrueRule<'a> {
    part: char,
    sym: Ordering,
    val: usize,
    dest: Dest<'a>,
}

fn check_rule(rating: Rating, rule: TrueRule) -> bool {
    match rule.part {
        'x' => (rating.x).cmp(&rule.val) == rule.sym,
        'm' => (rating.m).cmp(&rule.val) == rule.sym,
        'a' => (rating.a).cmp(&rule.val) == rule.sym,
        's' => (rating.s).cmp(&rule.val) == rule.sym,
        _ => panic!("check_rule!"),
    }
}

#[derive(Debug, Clone, Copy)]
enum Rule<'a> {
    TrueRule(TrueRule<'a>),
    Dest(Dest<'a>),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct MinMax {
    minim: usize,
    maxim: usize,
}

impl MinMax {
    fn default_vals() -> Self {
        Self {
            minim: 1,
            maxim: 4000,
        }
    }
}

fn comp(mx: MinMax, sym: Ordering, val: usize) -> MinMax {
    match sym {
        Ordering::Less => MinMax {
            minim: mx.minim,
            maxim: usize::min(mx.maxim, val),
        },
        _ => MinMax {
            minim: usize::max(mx.minim, val),
            maxim: mx.maxim,
        },
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Ranging {
    x: MinMax,
    m: MinMax,
    a: MinMax,
    s: MinMax,
}

impl Ranging {
    fn default_vals() -> Self {
        Self {
            x: MinMax::default_vals(),
            m: MinMax::default_vals(),
            a: MinMax::default_vals(),
            s: MinMax::default_vals(),
        }
    }
}

fn condn_comp(ranging: Ranging, true_rule: TrueRule) -> bool {
    if true_rule.sym == Ordering::Greater {
        match true_rule.part {
            'x' => ranging.x.maxim > true_rule.val,
            'm' => ranging.m.maxim > true_rule.val,
            'a' => ranging.a.maxim > true_rule.val,
            's' => ranging.s.maxim > true_rule.val,
            _ => panic!("condn_comp"),
        }
    } else {
        match true_rule.part {
            'x' => ranging.x.minim < true_rule.val,
            'm' => ranging.m.minim < true_rule.val,
            'a' => ranging.a.minim < true_rule.val,
            's' => ranging.s.minim < true_rule.val,
            _ => panic!("condn_comp"),
        }
    }
}

fn match_comp(mut ranging: Ranging, true_rule: TrueRule) -> Ranging {
    match true_rule.part {
        'x' => ranging.x = comp(ranging.x, true_rule.sym, true_rule.val),
        'm' => ranging.m = comp(ranging.m, true_rule.sym, true_rule.val),
        'a' => ranging.a = comp(ranging.a, true_rule.sym, true_rule.val),
        's' => ranging.s = comp(ranging.s, true_rule.sym, true_rule.val),
        _ => panic!("match_comp!"),
    }
    ranging
}

fn part_b<'a>(
    start: &[Rule<'a>],
    workflows: &HashMap<&'a str, Vec<Rule<'a>>>,
    rangevec: &mut Vec<Ranging>,
    mut ranging: Ranging,
) -> Vec<Ranging> {
    for rule in start {
        match rule {
            Rule::Dest(dest) => {
                helper_b(dest, workflows, rangevec, ranging);
            }
            Rule::TrueRule(true_rule) => {
                if condn_comp(ranging, *true_rule) {
                    let new_ranging = match_comp(ranging, *true_rule);
                    helper_b(&true_rule.dest, workflows, rangevec, new_ranging);
                }
                let new_rule = TrueRule {
                    sym: if true_rule.sym == Ordering::Greater {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    },
                    val: if true_rule.sym == Ordering::Greater {
                        true_rule.val + 1
                    } else {
                        true_rule.val - 1
                    },
                    ..*true_rule
                };

                ranging = match_comp(ranging, new_rule);
            }
        };
    }

    rangevec.clone()
}

fn helper_b<'a>(
    dest: &Dest<'a>,
    workflows: &HashMap<&'a str, Vec<Rule<'a>>>,
    rangevec: &mut Vec<Ranging>,
    ranging: Ranging,
) {
    match dest {
        Dest::A => {
            rangevec.push(ranging);
        }
        Dest::Name(name) => {
            part_b(&workflows[name], workflows, rangevec, ranging);
        }
        _ => {}
    }
}

fn part_a<'a>(
    rating: &Rating,
    start: &[Rule<'a>],
    workflows: &HashMap<&'a str, Vec<Rule<'a>>>,
) -> usize {
    for rule in start {
        match rule {
            Rule::Dest(dest) => {
                return helper_a(rating, *dest, workflows);
            }
            Rule::TrueRule(true_rule) => {
                if check_rule(*rating, *true_rule) {
                    return helper_a(rating, true_rule.dest, workflows);
                }
            }
        }
    }
    0
}

fn helper_a<'a>(
    rating: &Rating,
    dest: Dest<'a>,
    workflows: &HashMap<&'a str, Vec<Rule<'a>>>,
) -> usize {
    match dest {
        Dest::A => rating.x + rating.m + rating.a + rating.s,
        Dest::Name(name) => part_a(rating, &workflows[name], workflows),
        Dest::R => 0,
    }
}

fn main() {
    let mut data = include_str!("tests.txt").split("\n\n");
    let workflows: HashMap<&str, Vec<Rule>> = data
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut l = l.split("{");

            let name = l.next().unwrap();

            let rulevec: Vec<Rule> = l
                .next()
                .unwrap()
                .trim_end_matches('}')
                .split(',')
                .map(|rule| {
                    if rule.contains(":") {
                        let mut rule = rule.split(":");
                        let rulepart = rule.next().unwrap();

                        let (part, sym, val) = if rulepart.contains('<') {
                            let mut rulepart = rulepart.split("<");
                            let part = rulepart.next().unwrap().chars().next().unwrap();
                            let sym = Ordering::Less;
                            let val: usize = rulepart.next().unwrap().parse().unwrap();
                            (part, sym, val)
                        } else {
                            let mut rulepart = rulepart.split(">");
                            let part = rulepart.next().unwrap().chars().next().unwrap();
                            let sym = Ordering::Greater;
                            let val: usize = rulepart.next().unwrap().parse().unwrap();
                            (part, sym, val)
                        };
                        let dest = to_dest(rule.next().unwrap());

                        Rule::TrueRule(TrueRule {
                            part,
                            sym,
                            val,
                            dest,
                        })
                    } else {
                        let dest = to_dest(rule);
                        Rule::Dest(dest)
                    }
                })
                .collect();

            (name, rulevec)
        })
        .collect();

    let ratings: Vec<Rating> = data
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let l = l
                .chars()
                .filter(|&c| !"{}xmas=".contains(c))
                .collect::<String>();
            let mut l = l.split(",");
            let x: usize = l.next().unwrap().parse().unwrap();
            let m: usize = l.next().unwrap().parse().unwrap();
            let a: usize = l.next().unwrap().parse().unwrap();
            let s: usize = l.next().unwrap().parse().unwrap();

            Rating { x, m, a, s }
        })
        .collect();

    let start = workflows.get("in").unwrap();
    let mut res = 0;

    ratings.iter().for_each(|rating| {
        let val = part_a(rating, start, &workflows);
        res += val;
    });
    println!("Part A = {}", res);

    let ranging = Ranging::default_vals();
    let mut rangevec: Vec<Ranging> = Vec::new();

    let rangevec = part_b(start, &workflows, &mut rangevec, ranging);

    // dbg!(rangevec.clone());

    let mut res = 0;

    rangevec.iter().for_each(|ranging| {
        let x = ranging.x.maxim - ranging.x.minim + 1;
        let m = ranging.m.maxim - ranging.m.minim + 1;
        let a = ranging.a.maxim - ranging.a.minim + 1;
        let s = ranging.s.maxim - ranging.s.minim + 1;

        res += x * m * a * s;
    });

    println!("Part B = {}", res);
}
