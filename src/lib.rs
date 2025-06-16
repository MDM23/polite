mod codegen;
mod query;

use std::fmt::Debug;

pub use traitable;

pub use query::Subject;

pub trait AccessControl {
    type Scope: std::fmt::Debug + Clone + PartialEq;
    type Action: std::fmt::Debug + Clone + PartialEq;

    fn in_scope(&self, scope: &Self::Scope) -> bool;
}

#[derive(Clone, Debug)]
pub enum Scope<S: std::fmt::Debug + PartialEq> {
    Any,
    Constrained(Vec<S>),
}

pub struct Rule<T: AccessControl> {
    pub actions: Vec<T::Action>,
    pub scope: Scope<T::Scope>,
}

impl<T: AccessControl> Debug for Rule<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("actions", &self.actions)
            .field("scope", &self.scope)
            .finish()
    }
}

impl<T: AccessControl> Clone for Rule<T> {
    fn clone(&self) -> Self {
        Self {
            actions: self.actions.clone(),
            scope: self.scope.clone(),
        }
    }
}

pub trait AsRule<T: AccessControl> {
    fn as_rule(&self) -> Option<&Rule<T>>;
}

pub fn get_rules<T>(role: impl IntoIterator<Item = impl AsRule<T>>) -> Vec<Rule<T>>
where
    T: AccessControl + 'static,
{
    role.into_iter()
        .filter_map(|rule| rule.as_rule().cloned())
        .collect()
}
