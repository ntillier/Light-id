pub const DEFAULT_CHARACTERS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub struct LightId {
    pub characters: Vec<char>,
    pub min_length: usize,
    pub status: usize,
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
        /*let mut status: Vec<usize> = vec![];

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
        }*/

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
        /*let mut status: Vec<usize> = vec![];

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
        */

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

    pub fn chars<S: AsRef<str>>(&mut self, characters: S) -> &mut Self {
        self.characters = characters.as_ref().chars().collect();
        self
    }

    pub fn clone(&self) -> Self {
        LightId {
            status: self.status.clone(),
            characters: self.characters.clone(),
            min_length: self.min_length.clone()
        }
    }

    pub fn count(&self) -> usize {
        return self.status;
    }

    pub fn decrement(&mut self) -> &mut Self {
        self.decrement_by(1)
    }

    pub fn decrement_by(&mut self, count: usize) -> &mut Self {
        self.status -= count;
        self
    }

    pub fn increment(&mut self) -> &mut Self {
        self.increment_by(1)
    }

    pub fn increment_by(&mut self, count: usize) -> &mut Self {
        self.status += count;

        self
    }

    pub fn next(&mut self) -> String {
        let string = self.current();
        self.increment();
        string
    }

    pub fn current(&self) -> String {

        let mut current = String::new();
        
        let mut remaining = self.status;

        loop {
            current.push(self.characters[remaining % self.characters.len()]);
            
            remaining /= self.characters.len();

            if remaining == 0 {
                break;
            }
        }

        while current.len() < self.min_length {
            current.push(self.characters[0]);
        }

        current.chars().rev().collect()
    }
}
