use std::collections::HashMap;
use crate::{key::Key, hardware::KeyEv};
use anyhow::{Result, bail};
use thiserror::Error;
use std::ops::{Deref, DerefMut};
// pub mod dir;

pub struct Branch(HashMap<BranchKey, Action>);

impl Deref for Branch {
    type Target = HashMap<BranchKey, Action>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Branch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Error)]
pub enum BranchError {
    #[error("you need to pass at least one key")]
    ZeroKeys,
    #[error("string \"{0}\" does not correspond to keys")]
    StringDoesNotCorrespondToKeys(String),
}

#[derive(Eq, Hash, PartialEq)]
pub enum BranchKey {
    Key(Key),
}

pub enum Action {
    Branch(Branch),
    Cmd(String),

    // // The Func is an internal function, the Strings contain arguments
    // Func(Func, Vec<String>),

    // Not implemented yet
    Lua,
    None
}

impl Branch {
    pub fn new() -> Self {
        Branch(HashMap::new())
    }

    pub fn new_key(&mut self, key: Key, action: Action) {
        self.insert(BranchKey::Key(key), action);
    }

    /// Create new shortcut from keys. Returns false if the shortcut couldn'
    pub fn new_shortcut(&mut self, keys: &[Key], action: Action) -> Result<()> {
        if keys.len() == 0 {
            bail!(BranchError::ZeroKeys);
        }

        let mut b = Branch::new();
        b.new_key(*keys.last().unwrap(), action);
        for key in keys.iter().skip(1).rev() {
            let mut nb = Branch::new();
            nb.new_key(*key, Action::Branch(b));
            b = nb;
        }

        self.new_key(keys[0], Action::Branch(b));

        Ok(())
    }

    pub fn new_shortcut_from_str(&mut self, s: &str, action: Action) -> Result<()> {
        let keys = Key::from_str_many(s).ok_or(BranchError::StringDoesNotCorrespondToKeys(s.to_string()))?;
        self.new_shortcut(keys.as_slice(), action)?;

        Ok(())
    }
}
