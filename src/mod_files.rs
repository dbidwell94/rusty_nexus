pub mod models;

use raxios::Raxios;
use std::rc::Rc;

pub struct ModFiles {
    raxios: Rc<Raxios>,
}

impl From<&Rc<Raxios>> for ModFiles {
    fn from(raxios: &Rc<Raxios>) -> Self {
        Self {
            raxios: raxios.clone(),
        }
    }
}

impl ModFiles {}

#[cfg(test)]
mod tests {}
