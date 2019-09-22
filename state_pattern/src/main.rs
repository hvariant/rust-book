mod oop;

use oop::*;

fn main() {
    let mut post = Post::new();
    let s = "Why do this when you can do that?";

    post.add_text(s);
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("please");
    assert_eq!("", post.content());

    post.approve();
    post.add_text("yay");
    assert_eq!("", post.content());

    post.approve();
    post.add_text("yayyay");
    assert_eq!(s, post.content());
}
