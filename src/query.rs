use crate::{AccessControl, AsRule};

pub struct Subject<'a, T: AccessControl> {
    model: Option<&'a T>,
    actions: Option<Vec<T::Action>>,
}

impl<'a, T> From<&'a T> for Subject<'a, T>
where
    T: AccessControl,
{
    fn from(value: &'a T) -> Self {
        Self {
            model: Some(value),
            actions: None,
        }
    }
}

impl<'a, T> Subject<'a, T>
where
    T: AccessControl,
{
    pub fn new() -> Self {
        Self {
            model: None,
            actions: None,
        }
    }

    pub fn model(mut self, m: &'a T) -> Self {
        self.model = Some(m);
        self
    }

    pub fn action(mut self, action: T::Action) -> Self {
        self.actions = Some(vec![action]);
        self
    }

    pub fn allowed_by(&self, role: impl IntoIterator<Item = impl AsRule<T>>) -> bool {
        for rule in role {
            let Some(rule) = rule.as_rule() else {
                continue;
            };

            if let Some(m) = self.model {
                match &rule.scope {
                    crate::Scope::Any => (),
                    crate::Scope::Constrained(scopes) => {
                        if scopes.iter().any(|s| !m.in_scope(s)) {
                            continue;
                        }
                    }
                }
            }

            if let Some(actions) = &self.actions {
                if actions.iter().any(|a| !rule.actions.contains(a)) {
                    continue;
                }
            }

            return true;
        }

        false
    }

    pub fn get_constraints(
        &self,
        role: impl IntoIterator<Item = impl AsRule<T>>,
    ) -> Vec<Vec<T::Scope>> {
        let mut constraints = Vec::new();

        for rule in role {
            let Some(rule) = rule.as_rule() else {
                continue;
            };

            match &rule.scope {
                crate::Scope::Any => return Vec::new(),
                crate::Scope::Constrained(scopes) => {
                    if let Some(m) = self.model {
                        if scopes.iter().any(|s| !m.in_scope(s)) {
                            continue;
                        }
                    }

                    constraints.push(scopes.clone());
                }
            }
        }

        constraints
    }
}
