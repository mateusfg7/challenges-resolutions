fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
  let mut result: Vec<String> = vec![];
    
  for candidate in data {
    let (age, handicap) = candidate;
      
    if age >= 55 && handicap > 7 { result.push(String::from("Senior")) }
    else { result.push(String::from("Open")) }  
  }
    
  result
}
