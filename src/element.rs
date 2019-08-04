/// Wrapper for `web_sys::Element` to simplify calling different interfaces.
///
/// # Fields
///
/// * `el` is the `Option` wrapped `Element`.
pub struct Element {
    pub el: Option<web_sys::Element>,
}

impl Element {
    /// Returns the first Element on the body that matches the specified
    /// selector `selector`.
    ///
    /// # Syntax
    ///
    /// ```
    /// let element = Element::qs(selector);
    /// ```
    ///
    /// ## Parameters
    ///
    /// ### `selector`
    ///
    /// A selector to match the elements in the body; this must be valid CSS
    /// syntax or the returned value is `None`. The first element which matches
    /// the selector is returned.
    ///
    /// ## Return value
    ///
    /// The first element in the body that matches `selector`. The entire
    /// hierarchy of elements is considered when matching, including those
    /// outside of the body; in other words, `selector` is first applied to the
    /// whole document, not the body, to generate an initial list of potential
    /// elements. The resulting elements are then examined to see if they are
    /// desendants of the body. The first match of those remaining elements is
    /// returned.
    ///
    /// If no matches are found, the returned value is an Option<Element>
    /// containing `None`. (The returned value is not `None`).
    pub fn qs(selector: &str) -> Option<Element> {
        // Gets body as a Element.
        let body: web_sys::Element = web_sys::window()?.document()?.body()?.into();
        // Selects an Element, use .ok()? for error handling.
        let el = body.query_selector(selector).ok()?;
        // Returns Option wrapped Element, which could contain None if no
        // matching element was found.
        Some(Element { el })
    }
}
