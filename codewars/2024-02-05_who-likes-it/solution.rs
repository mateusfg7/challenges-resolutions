fn likes(names: &[&str]) -> String {
    let mut message = String::new();
    
    if names.len() == 0 {
        message.push_str("no one likes this");
    } else if names.len() == 1 {
        message.push_str(format!("{} likes this", names[0]).as_str());
    } else if names.len() == 2 {
        message.push_str(format!("{} and {} like this", names[0], names[1]).as_str());
    } else if names.len() == 3 {
        message.push_str(format!("{}, {} and {} like this", names[0], names[1], names[2]).as_str());
    } else if names.len() > 3 {
        message.push_str(format!("{}, {} and {} others like this", names[0], names[1], names.len()-2).as_str());
    }
    
    return message
}
