use polite::AccessControl;

#[derive(Clone, Debug, PartialEq)]
enum Group {
    Development,
    Accounting,
    Management,
}

#[derive(Clone, Debug, PartialEq)]
enum GroupScope {
    Group(Group),
}

#[derive(Clone, Debug, PartialEq)]
enum GroupAction {
    Create,
    Delete,
    View,
}

impl AccessControl for Group {
    type Action = GroupAction;
    type Scope = GroupScope;

    fn in_scope(&self, scope: &Self::Scope) -> bool {
        match scope {
            GroupScope::Group(group) => self == group,
        }
    }
}

struct User {
    id: usize,
    email: String,
    groups: Vec<Group>,
}

#[derive(Clone, Debug, PartialEq)]
enum UserScope {
    Domain(String),
    Group(Group),
}

#[derive(Clone, Debug, PartialEq)]
enum UserAction {
    Create,
    Delete,
    View,
}

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

polite::generate!();

#[test]
fn test() {
    let r = vec![
        Rule::User(polite::Rule {
            actions: vec![UserAction::View],
            scope: polite::Scope::Constrained(vec![UserScope::Domain(String::from("example.org"))]),
        }),
        Rule::Group(polite::Rule {
            actions: vec![GroupAction::Create, GroupAction::Delete, GroupAction::View],
            scope: polite::Scope::Any,
        }),
    ];

    let u = User {
        id: 1,
        email: String::from("foo@example.org"),
        groups: Vec::new(),
    };

    use polite::Subject;

    assert!(Subject::<User>::new().allowed_by(&r));
    assert!(Subject::from(&u).allowed_by(&r));
    assert!(Subject::from(&u).action(UserAction::View).allowed_by(&r));
    assert!(!Subject::from(&u).action(UserAction::Create).allowed_by(&r));

    assert!(
        Subject::<User>::new()
            .action(UserAction::View)
            .allowed_by(&r)
    );

    let u = User {
        id: 1,
        email: String::from("foo@bar.com"),
        groups: Vec::new(),
    };

    assert!(!Subject::from(&u).allowed_by(&r));
    assert!(!Subject::from(&u).action(UserAction::View).allowed_by(&r));

    assert!(
        !Subject::<User>::new()
            .action(UserAction::Create)
            .allowed_by(&r)
    );

    assert_eq!(
        Subject::<User>::new().get_constraints(&r),
        vec![vec![UserScope::Domain(String::from("example.org"))]],
    );
}
