#[macro_export]
macro_rules! generate {
    () => {
        polite::generate! {
            @inner $
        }
    };

    (@inner $D:tt) => {
        polite::traitable::generate! {
            (AccessControl) => {

                #[derive(Debug, Clone)]
                pub enum Rule {
                    $D( $ty ( polite::Rule<$ty_full> ), )*
                }

                $D(
                    impl polite::AsRule<$ty_full> for &Rule {
                        fn as_rule(&self) -> Option<&polite::Rule<$ty_full>> {
                            let Rule::$ty (r) = self else {
                                return None;
                            };

                            Some(r)
                        }
                    }
                )*

            }
        }
    };
}
