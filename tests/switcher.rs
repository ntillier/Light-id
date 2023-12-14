
use light_id::{IdSwitcher, LightId};

#[test]
fn new_switch () {
  let _ = IdSwitcher::new("0123456789", "01");
}

#[test]
fn switch () {
  let switch = IdSwitcher::new("0123456789", "01");

  assert_eq!("1", switch.switch("1"));
}

#[test]
fn switch_2 () {
  let switch = IdSwitcher::new("0123456789", "01");

  assert_eq!("1100100", switch.switch("100"));
}

#[test]
fn switch_3 () {
  let switch = IdSwitcher::new("01", "0123456789");

  assert_eq!("100", switch.switch("1100100"));
}

#[test]
fn switch_4 () {
  let switch = IdSwitcher::new("0123456789abcdef", "0123456789");
  let mut generator = LightId::from("0123456789abcdef");

  for _ in 0..1000 {
    generator.increment();

    assert_eq!(generator.count().to_string(), switch.switch(generator.current()));
  }
}

#[test]
fn min () {
  let mut switch = IdSwitcher::new("abcdefg", "abcdefghijklmnop");
  switch.min_target(10);

  assert_eq!("aaaaaaaaaa", switch.switch("a"));
}

#[test]
fn reverse () {
  let switch = IdSwitcher::new("0123456789", "abcdefghij");

  for i in 0..100 {
    assert_eq!(i.to_string(), switch.switch_reverse(switch.switch(i.to_string())));
  }
}