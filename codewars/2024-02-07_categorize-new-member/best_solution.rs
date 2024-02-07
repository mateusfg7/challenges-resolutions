// https://www.codewars.com/kata/reviews/5d109395ed0ee50001b4dbfb/groups/5d49c5d40b44010001010ef2
fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.into_iter()
        .map(|(age, handicap)| {
            if age >= 55 && handicap > 7 {
                "Senior"
            } else {
                "Open"
            }
            .to_string()
        })
        .collect()
}
