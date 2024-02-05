// https://en.wikipedia.org/wiki/Casting_out_nines
// https://www.codewars.com/kata/reviews/5f5cecfe69f1cd0001af463a/groups/5f5ced0969f1cd0001af463f

fn digital_root(n: i64) -> i64 {
    (n - 1) % 9 + 1
}
