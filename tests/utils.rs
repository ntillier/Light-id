use light_id::LightId;

#[test]
fn len () {
  let gen = LightId::new();

  assert_eq!(0, gen.len());
}

#[test]
fn len_2 () {
  let mut gen = LightId::new();

  gen.increment_by(61);

  assert_eq!(1, gen.len());
}

#[test]
fn len_3 () {
  let mut gen = LightId::new();

  gen.increment_by(62);

  assert_eq!(2, gen.len());
}

#[test]
fn last () {
  let mut gen = LightId::from("abc");

  gen.last("caa");

  assert_eq!(18, gen.count());
}