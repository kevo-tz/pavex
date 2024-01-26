use pavex::blueprint::constructor::{Constructor, Lifecycle};
use pavex::blueprint::Blueprint;
use pavex::f;
use pavex::kit::ApiKit;

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    let mut kit = ApiKit::new();
    let c = Constructor::new(f!(crate::custom_path_params), Lifecycle::RequestScoped); // (1)!
    kit.path_params = Some(c);
    kit.register(&mut bp);
    bp
}
