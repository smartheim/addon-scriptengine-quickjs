# Changes compared to upstream 

* qjs-sys: Use newer quick-js than upstream.
* Don't use CString. This brings us into memory management hell,
  especially with user defined c_functions. All toCString methods return a String.
  
Added new ExtractValue for "Value", so that a "Value" can be taken as an input to rust-in-js functions.
```rust
impl ExtractValue for Value {
    fn extract_value(v: &Local<Value>) -> Option<Self> {
        v.inner.as_ref().and_then(|f|v.ctxt.clone_value(f).inner.take())
    }
}
```