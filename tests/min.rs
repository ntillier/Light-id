use light_id::LightId;

#[test]
fn min() {
  let mut gen = LightId::from("abc");

  gen.min(10);

  assert_eq!("aaaaaaaaaa", gen.current());
}

#[test]
fn min_with_skip() {
  let mut gen = LightId::from("abc");

  gen.min(10).skip(10);

  assert_eq!("aaaaaaaacb", gen.current());
}

#[test]
fn skip_with_min() {
  let mut gen = LightId::from("abc");

  gen.skip(10).min(10);

  assert_eq!("aaaaaaaacb", gen.current());
}