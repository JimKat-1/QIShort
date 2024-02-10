use super::branch::{Branch, BranchKey, Action};
use crate::hardware::KeyEv;
use anyhow::{Result, anyhow, bail};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("current branch empty")]
    NoBranch,
    #[error("key release not press")]
    KeyReleaseNotPress,
    #[error("key not found")]
    KeyNotFound,
}

pub struct HandlerState<'a> {
    current_branch: Option<&'a mut Branch>,
}

impl<'a> HandlerState<'a> {
    pub fn new(root: &'a mut Branch) -> Self {
        Self {
            current_branch: Some(root),
        }
    }

    pub fn handle_keypress(&'a mut self, keyev: &KeyEv) -> Result<()> {
        //     Some(ref mut b) => b,
        //     None => anyhow!(HandlerError::NoBranch)
        // };
        let (key, press) = keyev.into_key();
        if !press { bail!(HandlerError::KeyReleaseNotPress); }
        let action = self.current_branch.as_mut().ok_or(anyhow!(HandlerError::NoBranch))?.get_mut(&BranchKey::Key(key)).ok_or(anyhow!(HandlerError::KeyNotFound))?;

        match action {
            Action::Branch(branch) => {
                self.current_branch = Some(branch);
            },
            Action::Cmd(ref c) => {
                let mut cmd = std::process::Command::new("sh").arg("-c").arg(c).spawn();
            },
            _ => unimplemented!("Bruh!!!")
        }

        Ok(())
    }
}






// #[derive(Debug, Error)]
// pub enum HandlerError {
//     #[error("current branch empty")]
//     NoBranch,
//     #[error("key release not press")]
//     KeyReleaseNotPress,
//     #[error("key not found")]
//     KeyNotFound,
// }
//
// pub struct HandlerState<'a> {
//     current_branch: /* Option< */&'a mut Branch/*>*/,
// }
//
// impl<'a> HandlerState<'a> {
//     pub fn new(root: &'a mut Branch) -> Self {
//         Self {
//             current_branch: /* Some( */root/* ) */,
//         }
//     }
//
//     pub fn handle_keypress(&'a mut self, keyev: &KeyEv) -> Result<()> {
//         //     Some(ref mut b) => b,
//         //     None => bail!(HandlerError::NoBranch)
//         // };
//         let (key, press) = keyev.into_key();
//         if !press { bail!(HandlerError::KeyReleaseNotPress); }
//         let aaa = &mut *(*self.current_branch);
//         let action = aaa.get_mut(&BranchKey::Key(key)).ok_or(anyhow!(HandlerError::KeyNotFound))?;
//
//         match action {
//             Action::Branch(ref mut branch) => {
//                 self.current_branch = branch;
//             },
//             Action::Cmd(ref c) => {
//                 let mut cmd = std::process::Command::new("sh").arg("-c").arg(c).spawn();
//             },
//             _ => unimplemented!("Bruh!!!")
//         }
//
//         Ok(())
//     }
// }
