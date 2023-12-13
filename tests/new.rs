use light_id::LightId;

#[test]
fn new () {
  let _ = LightId::new();
}

#[test]
fn skip () {
  let mut gen = LightId::from("abc");

  gen.skip(2);

  assert_eq!("c", gen.current());
}

#[test]
fn skip_2 () {
  let mut gen = LightId::from("abc");
  
  gen.skip(3);
  assert_eq!("ba", gen.current());
}

#[test]
fn skip_3 () {
  let mut gen = LightId::from("abc");
  
  gen.skip(12);
  assert_eq!("bba", gen.current());
}

#[test]
fn skip_4 () {
  let mut gen = LightId::from("abc");

  let prev = gen.increment_by(100).current();

  gen.skip(100);

  assert_eq!(prev, gen.current());
}