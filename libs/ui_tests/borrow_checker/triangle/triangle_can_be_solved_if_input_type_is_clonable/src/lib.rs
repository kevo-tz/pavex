use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;
use pavex::response::Response;

// The call graph looks like this:
//
//   A
//  / \
// B   |&
//  \  |
// handler
//
// `A` cannot be borrowed by `handler` after it has been moved to construct `B`.
// `A` is cloneable though!
// Pavex should detect this and clone `A` before calling `B`'s constructor.

#[derive(Clone)]
pub struct A;

pub struct B;

pub fn a() -> A {
    todo!()
}

pub fn b(_a: A) -> B {
    todo!()
}

pub fn handler(_a: &A, _b: B) -> Response {
    todo!()
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    // A is a singleton, so it will be an input parameter of the dependency closure for `handler`
    bp.singleton(f!(crate::a)).clone_if_necessary();
    bp.request_scoped(f!(crate::b));
    bp.route(GET, "/home", f!(crate::handler));
    bp
}
