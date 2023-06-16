use crate::{ArabicReshaper, ReshaperConfig};

/// Iterator for the [ArabicReshaper], you can use this type to iterate over
/// strings in a [Iterator] and reshape them
pub struct ArabicReshaperIter<I>
where
    I: Iterator,
{
    reshaper: ArabicReshaper,
    underlying: I,
}

impl<I> Iterator for ArabicReshaperIter<I>
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.underlying.next().map(|v| self.reshaper.reshape(v))
    }
}

/// Wrap an iterator to reshape strings
pub trait ArabicReshaperExt: Iterator + Sized
where
    Self::Item: AsRef<str>,
{
    /// Reshape the iterator with the default [ArabicReshaper] config
    fn reshape_default(self) -> ArabicReshaperIter<Self> {
        ArabicReshaperIter {
            reshaper: ArabicReshaper::default(),
            underlying: self,
        }
    }

    /// Reshape the iterator using the given config
    fn reshape_with_config(self, config: ReshaperConfig) -> ArabicReshaperIter<Self> {
        ArabicReshaperIter {
            reshaper: ArabicReshaper::new(config),
            underlying: self,
        }
    }
}

impl<I: Iterator> ArabicReshaperExt for I where I::Item: AsRef<str> {}
