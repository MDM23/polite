# polite

> `Poli`cy `t`riage `e`engine.
>
> Be `polite`, ask for permission!

The package can be used to build a simple, yet powerful policy engine. Create
strongly typed policies by writing some enums / structs and implementing a
trait.

## Simple example

We start with our models that we want to reason about:

```rs
pub enum Group {
	Accounting,
	Management,
}

pub struct User {
	pub id: Uuid;
	pub name: String;
	pub email: String;
	pub groups: Vec<Group>;
}
```

Next, we define an enum with the possible scopes that the model can be part of:

```rs
pub enum UserScope {
	Domain(String),
	Group(Group),
}
```

Now, we define the available actions that can be performed on that model:

```rs
pub enum UserAction {
	Create,
	Delete,
	Update,
	View,
}
```

Then we implement the `polite::AccessControl` trait. The `in_scope` method
should evaluate a given scope and return `true` if the model is part of it:

```rs
impl AccessControl for User {
	type Action = UserAction;
	type Scope = UserScope;

	fn in_scope(&self, scope: &Self::Scope) -> bool {
		match scope {
            // Allow access to all users that have an email address with the
            // given domain.
            UserScope::Domain(d) => self.email.ends_with(&format!("@{d}")),

            // Allow access to all users that are member of the given group.
            UserScope::Group(g) => self.groups.contains(g),
		}
	}
}
```

Finally, the `polite::generate` macro finds all implementers of the
`AccessControl` trait and generates the necessary logic to work with our
policies:

```rs
polite::generate!();
```
