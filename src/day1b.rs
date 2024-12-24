use heapless::Vec;
use crate::util::count_lines;

const INPUT_CONTENT: &str = include_str!("../data/day1/input.txt");
const INPUT_NUM_LINES: usize = count_lines(INPUT_CONTENT);

type Id = u32;
type IdList = Vec<Id, INPUT_NUM_LINES>;

fn parse_lists(context: &str) -> (IdList, IdList) {
    let mut list_a = IdList::new();
    let mut list_b = IdList::new();
    for line in context.lines() {
        let mut iter = line.split_whitespace();
        let _ = list_a.push(iter.next().unwrap().parse().unwrap());
        let _ = list_b.push(iter.next().unwrap().parse().unwrap());
    }

    (list_a, list_b)
}

fn calculate_answer(list_a: IdList, list_b: IdList) -> usize
{
    let mut total = 0;
    for a in list_a.into_iter() {
        total += list_b.iter().filter(|x| **x == a).count() * a as usize;
    }
    return total;
}

pub fn answer() -> usize {
    let (mut lista, mut listb) = parse_lists(INPUT_CONTENT);
    lista.sort_unstable();
    listb.sort_unstable();
    let answer = calculate_answer(lista, listb);
    return answer;
}

