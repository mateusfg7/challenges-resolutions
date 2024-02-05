// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
  assert_eq!(true, solution("abc", "c"));
  assert_eq!(false, solution("strawberry", "banana"));
  assert_eq!(true, solution("mateus", "eus"));
  assert_eq!(true, solution("mateus", "ateus"));
  assert_eq!(false, solution("mateus", "esu"));
}
