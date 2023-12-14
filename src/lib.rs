//! # light_id
//!
//! `light_id` is a Rust crate for generating and manipulating light-weight IDs. It provides a flexible and customizable way to generate and switch between different bases for IDs.
//!
//! ## Features
//!
//! - Lightweight and customizable ID generation.
//! - Switching IDs between different bases.
//! - Skipping and iterating through IDs.
//!
//! ## Example
//!
//! ```rust
//! use light_id::{LightId, IdSwitcher};
//!
//! let mut generator = LightId::new();
//! println!("Current ID: {}", generator.next());
//!
//! let switcher = IdSwitcher::new("0123456789", "abcdef");
//! let switched_id = switcher.switch("2");
//! println!("Switched ID: {}", switched_id);
//! ```
//!
//! ## Installation
//!
//! Add the following lines to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! light-id = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! ### LightId
//!
//! The `LightId` struct allows you to generate and manipulate IDs with various options.
//!
//! ```rust
//! use light_id::LightId;
//!
//! let mut generator = LightId::new();
//! generator.increment();
//! println!("Current ID: {}", generator.current());
//! ```
//!
//! ### IdSwitcher
//!
//! The `IdSwitcher` struct facilitates switching IDs between different bases.
//!
//! ```rust
//! use light_id::IdSwitcher;
//!
//! let switcher = IdSwitcher::new("0123456789", "abcdef");
//! let switched_id = switcher.switch("2");
//! println!("Switched ID: {}", switched_id);
//! ```
//!
//! ## API Documentation
//!
//! See the detailed documentation for each struct, including methods and usage examples.
//!
//! - [`LightId`](struct.LightId.html)
//! - [`IdSwitcher`](struct.IdSwitcher.html)
//!
//! ## License
//!
//! This crate is licensed under the [MIT License](https://opensource.org/licenses/MIT).
//!
//! ## Changelog
//!
//! - **0.1.0** (2023-12-14): Initial release

mod utils;

pub const DEFAULT_CHARACTERS: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct LightId {
    pub characters: Vec<char>,
    pub min_length: usize,
    status: usize,
}

impl PartialEq for LightId {
    fn eq(&self, other: &Self) -> bool {
        self.count() == other.count() && self.characters == other.characters
    }
}

impl PartialOrd for LightId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.count().cmp(&other.count()))
    }
}

impl LightId {
    /// Creates a new [`LightId`] with the default configuration.
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    /// ```
    pub fn new() -> Self {
        LightId {
            status: 0,
            characters: DEFAULT_CHARACTERS.chars().collect(),
            min_length: 0,
        }
    }

    /// Creates a new [`LightId`] with a custom alphabet
    /// ```
    /// use light_id::LightId;
    ///
    /// let generator = LightId::from("abcdef");
    /// ```
    /// If the provided `characters` is equal to [`DEFAULT_CHARACTERS`], the expression can be replaced with
    /// ```
    /// use light_id::LightId;
    ///
    /// let generator = LightId::new();
    /// ```
    pub fn from<S: AsRef<str>>(characters: S) -> Self {
        LightId {
            status: 0,
            characters: characters.as_ref().chars().collect(),
            min_length: 0,
        }
    }

    /// Skip the first `n` ids
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// generator.skip(2);
    ///
    /// assert_eq!("2", generator.current());
    /// ```
    pub fn skip(&mut self, n: usize) -> &mut Self {
        self.status = n;

        self
    }

    /// Skips the first ids until the provided id.
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// generator.last("c");
    ///
    /// assert_eq!("c", generator.current());
    /// ```
    pub fn last<S: AsRef<str>>(&mut self, id: S) -> &mut Self {
        self.status = utils::parse_id(id.as_ref(), &self.characters);
        self
    }

    /// Sets the min length of the ids
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// generator.min(6);
    ///
    /// assert_eq!("000000", generator.current());
    /// ```
    pub fn min(&mut self, n: usize) -> &mut Self {
        self.min_length = n;

        self
    }

    /// Sets the possible characters, in their order of importance (custom base)
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// generator.chars("abc");
    ///
    /// assert_eq!("a", generator.current());
    /// ```
    pub fn chars<S: AsRef<str>>(&mut self, characters: S) -> &mut Self {
        self.characters = characters.as_ref().chars().collect();
        self
    }

    /// Clone the current [`LighId`].
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// let mut generator2 = generator.clone();
    /// ```
    pub fn clone(&self) -> Self {
        LightId {
            status: self.status.clone(),
            characters: self.characters.clone(),
            min_length: self.min_length.clone(),
        }
    }

    /// Returns the current number of ids
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// generator.increment();
    ///
    /// assert_eq!(1, generator.count());
    /// ```
    pub fn count(&self) -> usize {
        return self.status;
    }

    /// Decrements the current id.
    /// Internally uses an alias to [`LightId::decrement_by`]
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// generator.increment();
    /// generator.decrement();
    ///
    /// assert_eq!("0", generator.current());
    /// ```
    pub fn decrement(&mut self) -> &mut Self {
        self.decrement_by(1)
    }

    /// Decrements the current id with a given factor
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// generator.increment_by(10);
    /// generator.decrement_by(10);
    ///
    /// assert_eq!("0", generator.current());
    /// ```
    pub fn decrement_by(&mut self, count: usize) -> &mut Self {
        if count > self.status {
            self.status = 0;
        } else {
            self.status -= count;
        }
        self
    }

    /// Increments the current id by one.
    /// Internally uses an alias to [`LightId::increment_by`]
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// generator.increment();
    ///
    /// assert_eq!("1", generator.current());
    /// ```
    pub fn increment(&mut self) -> &mut Self {
        self.increment_by(1)
    }

    /// Increments the current id with a given factor
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// generator.increment_by(10);
    ///
    /// assert_eq!("a", generator.current());
    /// ```
    pub fn increment_by(&mut self, count: usize) -> &mut Self {
        self.status += count;

        self
    }

    /// Increments the id by one and returns it.
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// assert_eq!("0", generator.next());
    /// assert_eq!("1", generator.next());
    /// assert_eq!("2", generator.next());
    /// ```
    /// Internally uses an alias to [`LightId::current`] and [`LightId::increment`]
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// let value = generator.current();
    /// generator.increment();
    ///
    /// assert_eq!("0", value);
    /// ```
    pub fn next(&mut self) -> String {
        self.status += 1;
        utils::format_id(&(&self.status - 1), &self.min_length, &self.characters)
    }

    /// Returns the current id.
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// assert_eq!("0", generator.current());
    /// ```
    pub fn current(&self) -> String {
        utils::format_id(&self.status, &self.min_length, &self.characters)
    }

    /// Returns the length of the current id.
    /// ```
    /// use light_id::LightId;
    ///
    /// let mut generator = LightId::new();
    ///
    /// assert_eq!(1, generator.len());
    ///
    /// generator.increment_by(100);
    ///
    /// assert_eq!(2, generator.len());
    /// ```
    pub fn len(&self) -> usize {
        if self.status == 0 {
            return std::cmp::max(self.min_length, 1);
        }
        return std::cmp::max(
            self.min_length,
            self.status.ilog(self.characters.len()) as usize + 1,
        );
    }

    /// Returns the nth id.
    /// ```
    /// use light_id::LightId;
    /// 
    /// let generator = LightId::new();
    /// 
    /// assert_eq!("2", generator.nth(2));
    /// ```
    pub fn nth(&self, n: usize) -> String {
        utils::format_id(&n, &self.min_length, &self.characters)
    }

    /// Returns the index of the provided id
    /// ```
    /// use light_id::LightId;
    /// 
    /// let generator = LightId::new();
    /// 
    /// assert_eq!(2, generator.index("2"));
    /// ```
    pub fn index<S: AsRef<str>>(&self, id: S) -> usize {
        utils::parse_id(id.as_ref(), &self.characters)
    }
}

pub struct IdSwitcher {
    source: Vec<char>,
    source_min: usize,
    target: Vec<char>,
    target_min: usize,
}

impl IdSwitcher {

    /// Create a new [`IdSwitcher`].
    /// It can be used to switch the ids from a source base to a target base.
    /// ```
    /// use light_id::IdSwitcher;
    /// 
    /// let switcher = IdSwitcher::new("0123456789", "abcdefghij");
    /// ```
    pub fn new<S: AsRef<str>>(source: S, target: S) -> Self {
        IdSwitcher {
            source: source.as_ref().chars().collect(),
            source_min: 0,
            target: target.as_ref().chars().collect(),
            target_min: 0
        }
    }

    /// Returns a copy of the [`IdSwitcher`].
    /// ```
    /// use light_id::IdSwitcher;
    /// 
    /// let switcher = IdSwitcher::new("0123456789", "abcdefghij");
    /// let switcher_2 = switcher.clone();
    /// ```
    pub fn clone (&self) -> Self {
        IdSwitcher {
            source: self.source.clone(),
            source_min: self.source_min.clone(),
            target: self.target.clone(),
            target_min: self.target_min.clone(),
        }
    }

    /// Sets the min length of the converted ids.
    /// ```
    /// use light_id::IdSwitcher;
    /// 
    /// let mut switcher = IdSwitcher::new("0123456789", "abcdefghij");
    /// 
    /// switcher.min_target(10);
    /// 
    /// assert_eq!("aaaaaaaaaa", switcher.switch("0"));
    /// ```
    pub fn min_target(&mut self, n: usize) -> &mut Self {
        self.target_min = n;

        self
    }

    /// Sets the min length of the source ids.
    /// ```
    /// use light_id::IdSwitcher;
    /// 
    /// let mut switcher = IdSwitcher::new("0123456789", "abcdefghij");
    /// 
    /// switcher.min_source(10);
    /// 
    /// assert_eq!("0000000000", switcher.switch_reverse("a"));
    /// ```
    pub fn min_source(&mut self, n: usize) -> &mut Self {
        self.source_min = n;

        self
    }

    /// Switches an id count from the source base to the target base.
    /// ```
    /// use light_id::IdSwitcher;
    /// 
    /// let switcher = IdSwitcher::new("0123456789", "abcdefghij");
    /// 
    /// assert_eq!("a", switcher.switch_count(0));
    /// ```
    pub fn switch_count(&self, id: usize) -> String {
        utils::format_id(&id, &self.target_min, &self.target)
    }

    /// Switches an id from the source base to the target base.
    /// ```
    /// use light_id::IdSwitcher;
    /// 
    /// let switcher = IdSwitcher::new("0123456789", "abcdefghij");
    /// 
    /// assert_eq!("a", switcher.switch("0"));
    /// ```
    pub fn switch<S: AsRef<str>>(&self, id: S) -> String {
        self.switch_count(utils::parse_id(id.as_ref(), &self.source))
    }

    /// Switches an id count from the target base to the source base.
    /// ```
    /// use light_id::IdSwitcher;
    /// 
    /// let switcher = IdSwitcher::new("0123456789", "abcdefghij");
    /// 
    /// assert_eq!("0", switcher.switch_count_reverse(0));
    /// ```
    pub fn switch_count_reverse(&self, id: usize) -> String {
        utils::format_id(&id, &self.source_min, &self.source)
    }

    /// Switches an id from the target base to the source base.
    /// ```
    /// use light_id::IdSwitcher;
    /// 
    /// let switcher = IdSwitcher::new("0123456789", "abcdefghij");
    /// 
    /// assert_eq!("0", switcher.switch_reverse("a"));
    /// ```
    pub fn switch_reverse<S: AsRef<str>>(&self, id: S) -> String {
        self.switch_count_reverse(utils::parse_id(id.as_ref(), &self.target))
    }
}
