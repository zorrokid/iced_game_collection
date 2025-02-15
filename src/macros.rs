#[macro_export]
macro_rules! impl_has_oid {
    // macro will accept zero or more type names (`$t`) separated by spaces.
    // The `ty` fragment specifier indicates that `$t` should be a type
    ($($t:ty)*) => {

   //The following block is the macro's expansion. It will be repeated for each type provided to the macro. The `$($t:ty)*` pattern from the previous step is expanded here:
   // - `$t` is replaced with each type provided to the macro.
   // - The `impl HasIdField for $t` block is generated for each type.
        $(
            impl HasIdField for $t {
                fn get_id(&self) -> &Option<ObjectId> {
                    &self._id
                }
                fn set_id(&mut self, id: ObjectId) {
                    self._id = Some(id);
                }
            }
        )*
    };
}
