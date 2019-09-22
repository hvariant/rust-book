mod lib;
mod oop;

use lib::ApproveResult::*;
use lib::*;

fn main() {
    let mut post = Post::new();
    let s = "Why do this when you can do that?";
    post.add_text(s);

    let post = post.request_review();
    if let StillPending(post) = post.approve() {
        if let Approved(post) = post.approve() {
            assert_eq!(s, post.content());
        } else {
            panic!("at the disco: electric boogaloo");
        }
    } else {
        panic!("at the disco");
    }
}
