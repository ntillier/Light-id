use light_id::LightId;

#[test]
fn increment () {
  let mut gen = LightId::from("abc");

  for _ in 0..3 {
    gen.increment();
  }

  assert_eq!("ba", gen.current());
}

#[test]
fn increment_2 () {
  let mut gen = LightId::from("abc");

  for _ in 0..12 {
    gen.increment();
  }

  assert_eq!("bba", gen.current());
}

#[test]
fn increment_3 () {
  let mut gen = LightId::from("abc");

  for _ in 0..26 {
    gen.increment();
  }

  assert_eq!("ccc", gen.current());
}

#[test]
fn increment_by () {
  let mut gen1 = LightId::from("abc");
  let mut gen2 = gen1.clone();

  for _ in 0..100 {
    gen1.increment();
  }

  gen2.increment_by(100);

  assert_eq!(gen1.current(), gen2.current());
}

#[test]
fn decrement () {
  let mut gen = LightId::from("abc");

  let mut history = vec![];

  for _ in 0..100 {
    history.push(gen.next());
    println!("{}", history.last().unwrap());
  }

  for _ in 0..100 {
    assert_eq!(history.pop().unwrap(), gen.decrement().current());
  }
}

#[test]
fn decrement_by () {
  let mut gen = LightId::from("abc");

  gen.increment_by(100);

  gen.decrement_by(100);

  assert_eq!("a", gen.current());
}