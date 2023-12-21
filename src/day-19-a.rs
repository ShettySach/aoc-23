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

fn solve<'a>(
    rating: &Rating,
    start: &[Rule<'a>],
    workflows: &HashMap<&'a str, Vec<Rule<'a>>>,
) -> usize {
    for rule in start {
        match rule {
            Rule::Dest(dest) => {
                return send_dest(rating, *dest, workflows);
            }
            Rule::TrueRule(true_rule) => {
                if check_rule(*rating, *true_rule) {
                    return send_dest(rating, true_rule.dest, workflows);
                }
            }
        }
    }
    0
}

fn send_dest<'a>(
    rating: &Rating,
    dest: Dest<'a>,
    workflows: &HashMap<&'a str, Vec<Rule<'a>>>,
) -> usize {
    match dest {
        Dest::A => rating.x + rating.m + rating.a + rating.s,
        Dest::Name(name) => solve(rating, &workflows[name], workflows),
        Dest::R => 0,
    }
}

fn main() {
    let mut data = include_str!("inputs.txt").split("\n\n");
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
        let val = solve(rating, start, &workflows);
        res += val;
    });
    println!("Part 1 = {}", res);
}
