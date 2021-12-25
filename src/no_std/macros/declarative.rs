/// Abbr. of `let`
/// 
/// # Examples
/// 
/// ```
/// use aoko::l;
/// 
/// l!(foo = 233; bar: u16 = 666;);
/// assert_eq!((233, 666), (foo, bar));
/// ```
#[macro_export]
macro_rules! l {
    ($($a:ident $(:$t:ty)? = $b:expr);* $(;)?) => {
        $(let $a $(:$t)? = $b;)*
    };
}

/// Abbr. of `let mut`
/// 
/// # Examples
/// 
/// ```
/// use aoko::lm;
/// 
/// lm!(foo = 233; bar: u16 = 1024;);
/// foo = 666; bar = 2048;
/// assert_eq!((666, 2048), (foo, bar));
/// ```
#[macro_export]
macro_rules! lm {
    ($($a:ident $(:$t:ty)? = $b:expr);* $(;)?) => {
        $(let mut $a $(:$t)? = $b;)*
    };
}

/// Swaps two variables' value.
/// 
/// # Principles
/// 
/// Shadowing by two immutable variables.
/// 
/// # Examples
/// 
/// ```
/// use aoko::swap;
/// 
/// let (a, b, c, d) = (1, 2, 3, 4);
/// swap!(a, b; c, d;);
/// assert_eq!((a, b, c, d), (2, 1, 4, 3));
/// ```
#[macro_export]
macro_rules! swap {
    ($($a:ident, $b:ident);* $(;)?) => {
        $(let ($b, $a) = ($a, $b);)*
    };
}

/// Swaps two variables' value.
/// 
/// # Principles
/// 
/// Shadowing by two mutable variables.
/// 
/// # Examples
/// 
/// ```
/// use aoko::swap_mut;
/// 
/// let (a, b, c, d) = (1, 2, 3, 4);
/// swap_mut!(a, b; c, d;);
/// assert_eq!((a, b, c, d), (2, 1, 4, 3));
/// 
/// a = 10; b = 20; c = 30; d = 40;
/// assert_eq!((a, b, c, d), (10, 20, 30, 40));
/// ```
#[macro_export]
macro_rules! swap_mut {
    ($($a:ident, $b:ident);* $(;)?) => {
        $(let (mut $b, mut $a) = ($a, $b);)*
    };
}

/// Assert multiple expressions.
///
/// # Principles
/// 
/// Loop `assert!` statements.
/// 
/// # Examples
/// 
/// ```
/// use aoko::asserts;
/// 
/// asserts!(true; 1 + 1 == 2; "".chars().next().is_none(););
/// ```
#[macro_export]
macro_rules! asserts {
    ($($a:expr);* $(;)?) => {
        $(assert!($a);)*
    };
}

/// Assert multiple eq expressions.
///
/// # Principles
/// 
/// Loop `assert_eq!` statements.
/// 
/// # Examples
/// 
/// ```
/// use aoko::assert_eqs;
/// 
/// assert_eqs!(0, 0; "", ""; 'z', 'z'; true, true;);
/// ```
#[macro_export]
macro_rules! assert_eqs {
    ($($a:expr, $b:expr);* $(;)?) => {
        $(assert_eq!($a, $b);)*
    };
}