use std::cmp::Ordering;

pub const DEFAULT_CHARACTERS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub struct LightId {
    pub characters: Vec<char>,
    pub min_length: usize,
    pub status: Vec<usize>,
}

impl PartialEq for LightId {
    fn eq(&self, other: &Self) -> bool {
        self.count() == other.count()
    }
}

impl PartialOrd for LightId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
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
            status: vec![0],
            characters: DEFAULT_CHARACTERS.chars().collect(),
            min_length: 0,
        }
    }

    fn fill(&mut self) -> &mut Self {
        while self.status.len() < self.min_length {
            self.status.push(0);
        }

        self
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
            status: vec![0],
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
    /// generator.skip(1);
    /// 
    /// assert_eq!("b", generator.current());
    /// ```
    pub fn skip(&mut self, n: usize) -> &mut Self {
        let mut status: Vec<usize> = vec![];

        let mut remaining = n;

        loop {
            if remaining >= self.characters.len() {
                let q = remaining % self.characters.len();

                status.push(q);

                remaining = (remaining - q) / self.characters.len() - 1;
            } else {
                status.push(remaining);

                break;
            }
        }

        self.status = status;

        self.fill();

        self
    }

    /// Skips the first ids until the provided id.
    /// ```
    /// use light_id::LightId;
    /// 
    /// let mut generator = LightId::new();
    /// 
    /// generator.last("abc");
    /// 
    /// assert_eq!("abc", generator.current());
    /// ```
    pub fn last<S: AsRef<str>>(&mut self, id: S) -> &mut Self {
        let mut status: Vec<usize> = vec![];

        for char in id.as_ref().chars() {
            status.push(
                self.characters
                    .iter()
                    .position(|i| i == &char)
                    .expect("Invalid character"),
            );
        }

        status.reverse();

        self.status = status;

        self.fill();

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
    /// assert_eq!("aaaaaa", generator.current());
    /// ```
    pub fn min(&mut self, n: usize) -> &mut Self {
        self.min_length = n;

        self.fill();

        self
    }

    pub fn chars<S: AsRef<str>>(&mut self, characters: S) -> &mut Self {
        self.characters = characters.as_ref().chars().collect();
        self
    }

    pub fn clone(&self) -> Self {
        LightId {
            status: self.status.clone(),
            characters: self.characters.clone(),
            min_length: self.min_length.clone(),
        }
    }

    pub fn count(&self) -> usize {
        let mut count = 0;

        for (index, value) in self.status.iter().enumerate() {
            count += value * usize::pow(self.characters.len() + 1, index.try_into().unwrap());
        }

        return count;
    }

    pub fn decrement(&mut self) -> &mut Self {
        let mut i = 0;

        while i < self.status.len() {
            if self.status[i] == 0 {
                self.status[i] = self.characters.len() - 1;
            } else {
                self.status[i] -= 1;
                return self;
            }

            i += 1;
        }

        if self.status.len() > self.min_length {
            self.status.pop();
        }

        self
    }

    pub fn decrement_by(&mut self, count: usize) -> &mut Self {
        for _ in 0..count {
            self.decrement();
        }

        self
    }

    pub fn increment(&mut self) -> &mut Self {
        for i in self.status.iter_mut() {
            *i += 1;

            if i > &mut (self.characters.len() - 1) {
                *i = 0;
            } else {
                return self;
            }
        }

        self.status.push(0);

        self
    }

    pub fn increment_by(&mut self, count: usize) -> &mut Self {
        for _ in 0..count {
            self.increment();
        }

        self
    }

    pub fn next(&mut self) -> String {
        let string = self.current();
        self.increment();
        string
    }

    pub fn current(&self) -> String {
        let mut current = String::new();

        for i in self.status.iter().rev() {
            current.push(self.characters[*i]);
        }

        current
    }
}
