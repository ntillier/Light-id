pub const DEFAULT_CHARACTERS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

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
            min_length: 0
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
            min_length: 0
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
        let mut status = 0;

        for (index, char) in id.as_ref().chars().enumerate() {
            status += self
                .characters
                .iter()
                .position(|i| i == &char)
                .expect("Invalid character")
                * usize::pow(self.characters.len() + 1, index.try_into().unwrap());
        }

        self.status = status;

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
            min_length: self.min_length.clone()
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

    /// Decrements the current id
    /// ```
    /// use light_id::LightId;
    /// 
    /// let mut generator = LightId::new();
    /// 
    /// generator.increment();
    /// generator.decrement();
    /// 
    /// assert_eq!("a", generator.current());
    /// ```
    /// It is and alias of
    /// ```
    /// use light_id::LightId;
    /// 
    /// let mut generator = LightId::new();
    /// 
    /// generator.decrement_by(1);
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
    /// assert_eq!("a", generator.current());
    /// ```
    pub fn decrement_by(&mut self, count: usize) -> &mut Self {
        if count > self.status {
            self.status = 0;
        } else {
            self.status -= count;
        }
        self
    }

    /// Increments the current id
    /// ```
    /// use light_id::LightId;
    /// 
    /// let mut generator = LightId::new();
    /// 
    /// generator.increment();
    /// 
    /// assert_eq!("b", generator.current());
    /// ```
    /// It is and alias of
    /// ```
    /// use light_id::LightId;
    /// 
    /// let mut generator = LightId::new();
    /// 
    /// generator.increment_by(1);
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
    /// generator.increment_by(2);
    /// 
    /// assert_eq!("c", generator.current());
    /// ```
    pub fn increment_by(&mut self, count: usize) -> &mut Self {
        self.status += count;

        self
    }

    /// Increments the id and returns it
    /// ```
    /// use light_id::LightId;
    /// 
    /// let mut generator = LightId::new();
    /// 
    /// assert_eq!("a", generator.next());
    /// assert_eq!("b", generator.next());
    /// assert_eq!("c", generator.next());
    /// ```
    /// It is an alias of
    /// ```
    /// use light_id::LightId;
    /// 
    /// let mut generator = LightId::new();
    /// 
    /// let value = generator.current();
    /// generator.increment();
    /// 
    /// assert_eq!("a", value);
    /// ```
    pub fn next(&mut self) -> String {
        let string = self.current();
        self.increment();
        string
    }

    /// Returns the current id.
    /// ```
    /// use light_id::LightId;
    /// 
    /// let mut generator = LightId::new();
    /// 
    /// assert_eq!("a", generator.current());
    /// ```
    pub fn current(&self) -> String {

        let mut current = String::new();
        
        let mut remaining: usize = self.status;

        loop {
            current.push(self.characters[remaining % self.characters.len()]);
            
            remaining = remaining / self.characters.len();

            if remaining == 0 {
                break;
            }
        }

        while current.len() < self.min_length {
            current.push(self.characters[0]);
        }

        current.chars().rev().collect()
    }

    /// Returns the length of the current id.
    /// ```
    /// use light_id::LightId;
    /// 
    /// let mut generator = LightId::new();
    /// 
    /// assert_eq!(0, generator.len());
    /// 
    /// generator.increment();
    /// 
    /// assert_eq!(1, generator.len());
    /// ```
    pub fn len (&self) -> usize {
        if self.status == 0 {
            return self.min_length;
        }
        return std::cmp::max(self.min_length, self.status.ilog(self.characters.len()) as usize + 1);
    }
}
