# Hydration Tuple Mismatch Fix

## Problem

The `view!` macro would fail to compile when used with 4 or more elements, producing errors like:

```
error[E0308]: mismatched types
  --> src/main.rs:5:5
   |
5 |     let _view = view! {
   |     ^ expected 3 elements, found 5
   |
   = note: expected tuple of 3 elements
            found tuple of 5 elements
```

## Root Cause

The `view!` macro was generating tuples that didn't match the expected trait implementations. For views with 4+ elements, it was trying to fit them into a 3-element tuple structure, which caused type mismatches.

## Solution

Modified `leptos_macro/src/view/mod.rs` to use chunking logic for 4+ elements, similar to how 16+ elements are already handled.

### Before (lines 613-625):
```rust
} else if children.len() > 3 {
    // HYDRATION FIX: Handle 4+ elements by constraining to 3-element tuple structure
    // This fixes the "expected 3 elements, found 5" compilation error
    // Split elements: first element, remaining elements as nested tuple, unit placeholder
    let first = &children[0];
    let remaining = &children[1..];
    Some(quote! {
        (#first, (#(#remaining),*), ())
    })
```

### After:
```rust
} else if children.len() > 3 {
    // HYDRATION FIX: Handle 4+ elements by using chunking logic
    // This fixes the "expected 3 elements, found 5" compilation error
    // Use the same chunking approach as for >16 elements, but with smaller chunks
    let chunks = children.chunks(3).map(|chunk| {
        quote! {
            (#(#chunk),*)
        }
    });
    Some(quote! {
        (#(#chunks),*)
    })
```

## Testing

Added minimal tests in `tests/hydration_tuple_fix.rs` that verify:
- Views with 4+ elements now compile correctly
- Views with 1-3 elements continue to work (regression tests)

## Impact

- **Fixed**: Compilation errors for `view!` macro with 4+ elements
- **Maintained**: Backward compatibility with existing 1-3 element views
- **Minimal**: Only 22 lines of actual fix code

## Example

```rust
// This now works correctly
let _view = view! {
    <div>"First"</div>
    <div>"Second"</div>
    <div>"Third"</div>
    <div>"Fourth"</div>  // Previously caused tuple mismatch
    <div>"Fifth"</div>   // Previously caused tuple mismatch
};
```
