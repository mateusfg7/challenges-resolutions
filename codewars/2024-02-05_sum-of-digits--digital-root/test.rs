#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
      assert_eq!(digital_root(16), 7);
      assert_eq!(digital_root(42), 6);
      assert_eq!(digital_root(942), 6);
      assert_eq!(digital_root(132189), 6);
      assert_eq!(digital_root(493193), 2);
      assert_eq!(digital_root(357951456852), 6)
    }    
}
