macro_rules! added_to_owned {
    ($type:ty, $added:ty) => {
        fn added_to_owned(added: &$added) -> Result<$type> {
            Ok(added.1.to_owned())
        }
    };
}

macro_rules! commit {
    ($type:ty, $long:ident, $tuple_index:tt) => {
        paste! {
            pub(crate) fn commit(&mut self, base: &[$type]) -> Result<Vec<$type>> {
                let mut res = if self.deleted.is_empty() {
                    base.to_owned()
                } else {
                    base.iter().enumerate()
                        .filter(|(index, _)| !self.deleted.iter().any(|deleted| &deleted.$tuple_index == index))
                        .map(|(_, kept)| kept.clone()).collect::<Vec<$type>>()
                };
                if !self.added.is_empty() {
                    for added in self.added.iter() {
                        res.push(
                            [<$long Helper>]::<'a>::added_to_owned(added)
                            .with_context(|| format!("Bug: failed to execute added_to_owned for {}", stringify!([<$long Helper>])))?)
                    };
                };
                Ok(res)
            }
        }
    };
}

macro_rules! get_low_sorted_last {
    ($type:ty, $long:ident) => {
        pub(crate) fn get_low_sorted_last(&self) -> Result<Vec<&$type>> {
            let mut last_sorted = self
                .lowercased
                .last()
                .with_context(|| format!("Bug: failed to get {}Helper.lowercased.last()", stringify!($long)))?
                .iter()
                .collect::<Vec<&$type>>();
            last_sorted.sort();
            Ok(last_sorted)
        }
    };
}

pub(crate) use {added_to_owned, commit, get_low_sorted_last};
