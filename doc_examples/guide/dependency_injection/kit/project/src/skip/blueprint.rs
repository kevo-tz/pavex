use pavex::blueprint::Blueprint;
use pavex::kit::ApiKit;

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    let mut kit = ApiKit::new();
    // The constructor for `PathParams` will not be registered.
    kit.path_params = None;
    kit.register(&mut bp);
    bp
}
