pub enum DomainResult<T, E> {
    Ok(T),
    NotFound,
    Err(E),
}

impl<T, E> DomainResult<T, E> {
    pub fn ok(value: T) -> Self {
        Self::Ok(value)
    }

    pub fn not_found() -> Self {
        Self::NotFound
    }

    pub fn err(error: E) -> Self {
        Self::Err(error)
    }
}
