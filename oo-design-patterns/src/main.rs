use oo_design_patterns::states_as_traits;
use oo_design_patterns::states_as_types;

fn main() {
    // STATES AS TRAITS
    let mut trait_post = states_as_traits::Post::new();

    trait_post.add_text("I ate a salad for lunch today");
    assert_eq!("", trait_post.content());

    trait_post.request_review();
    assert_eq!("", trait_post.content());

    // Bonus 3
    trait_post.add_text(" and the day before");
    assert_eq!("", trait_post.content());

    // Bonus 1: Add a reject method that changes the post’s state from PendingReview back to Draft
    trait_post.reject();
    assert_eq!("", trait_post.content());

    trait_post.add_text(" and yesterday");
    assert_eq!("", trait_post.content());

    trait_post.request_review();
    assert_eq!("", trait_post.content());

    // Bonus 2: Require two calls to approve before the state can be changed to Published
    trait_post.approve();
    assert_eq!("", trait_post.content());

    // Bonus 3: Allow users to add text content only when a post is in the Draft state
    trait_post.add_text(" and the day before");
    assert_eq!("", trait_post.content());

    trait_post.approve();
    assert_eq!(
        "I ate a salad for lunch today and yesterday",
        trait_post.content()
    );

    // Bonus 3
    trait_post.add_text(" and the day before");
    assert_eq!(
        "I ate a salad for lunch today and yesterday",
        trait_post.content()
    );

    // STATES AS TYPES
    let mut type_post = states_as_types::Post::new();
    type_post.add_text("First post");
    // // compiler will not allow this
    // assert_eq!("", type_post.content());

    let type_post = type_post.request_review();
    // // compiler will not allow this
    // assert_eq!("", type_post.content());

    // Bonus 1: Add a reject method that changes the post’s state from PendingReview back to Draft
    let mut type_post = type_post.reject();
    // // compiler will not allow this
    // assert_eq!("", type_post.content());

    // Bonus 3: Allow users to add text content only when a post is in the Draft state
    type_post.add_text(" added");

    let mut type_post = type_post.request_review();

    // Bonus 2: Require two calls to approve before the state can be changed to Published
    let res = type_post.approve(2);
    match res {
        Some(_) => assert!(false),
        None => assert!(true),
    };

    match type_post.approve(2) {
        Some(final_post) => {
            assert_eq!("First post added", final_post.content());
        }
        None => assert!(false),
    }

    // let type_post: Post = type_post.approve(2)?;
}
