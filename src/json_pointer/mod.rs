mod model;

pub trait JsonPointerResolvable<T> {
    fn resolve_json_pointer(self, json_pointer: String) -> Option<T>;
}
