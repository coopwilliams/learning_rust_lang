// Demo of implementing state patterns in Rust.

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());
    
    post.approve();
    assert_eq!("", post.content()); // need 2 approvals
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
