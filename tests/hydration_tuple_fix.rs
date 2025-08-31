// Minimal test for the hydration tuple mismatch fix
// This demonstrates the actual issue: view! macro with 4+ elements caused tuple mismatch

use leptos::prelude::*;

#[test]
fn test_four_elements_compiles() {
    // This would fail before the fix with: "expected 3 elements, found 4"
    let _view = view! {
        <div>"First"</div>
        <div>"Second"</div>
        <div>"Third"</div>
        <div>"Fourth"</div>
    };
}

#[test]
fn test_five_elements_compiles() {
    // This would fail before the fix with: "expected 3 elements, found 5"
    let _view = view! {
        <div>"First"</div>
        <div>"Second"</div>
        <div>"Third"</div>
        <div>"Fourth"</div>
        <div>"Fifth"</div>
    };
}

#[test]
fn test_ten_elements_compiles() {
    // This would fail before the fix with: "expected 3 elements, found 10"
    let _view = view! {
        <div>"First"</div>
        <div>"Second"</div>
        <div>"Third"</div>
        <div>"Fourth"</div>
        <div>"Fifth"</div>
        <div>"Sixth"</div>
        <div>"Seventh"</div>
        <div>"Eighth"</div>
        <div>"Ninth"</div>
        <div>"Tenth"</div>
    };
}

#[test]
fn test_three_elements_still_works() {
    // This should continue to work (regression test)
    let _view = view! {
        <div>"First"</div>
        <div>"Second"</div>
        <div>"Third"</div>
    };
}

#[test]
fn test_single_element_still_works() {
    // This should continue to work (regression test)
    let _view = view! {
        <div>"Single"</div>
    };
}
