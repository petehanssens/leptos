// Minimal test to verify the hydration tuple fix
use leptos::prelude::*;

fn main() {
    // This would fail before the fix with: "expected 3 elements, found 4"
    let _view = view! {
        <div>"First"</div>
        <div>"Second"</div>
        <div>"Third"</div>
        <div>"Fourth"</div>
    };
    
    println!("✅ Four elements compile successfully!");
    
    // This would also fail before the fix with: "expected 3 elements, found 5"
    let _view2 = view! {
        <div>"First"</div>
        <div>"Second"</div>
        <div>"Third"</div>
        <div>"Fourth"</div>
        <div>"Fifth"</div>
    };
    
    println!("✅ Five elements compile successfully!");
    
    // This should continue to work (regression test)
    let _view3 = view! {
        <div>"First"</div>
        <div>"Second"</div>
        <div>"Third"</div>
    };
    
    println!("✅ Three elements still work!");
    
    println!("🎉 All hydration tuple tests passed!");
}
