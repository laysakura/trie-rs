use crate::inc_search::IncSearch;
use crate::iter::{Keys, KeysExt, Labels};
use crate::label::Label;
use crate::map;
use crate::search::{PostfixCollect, PostfixIter, PrefixCollect, PrefixIter};
use crate::try_collect::TryFromIterator;
use crate::try_from::TryFromTokens;
use std::iter::FromIterator;

#[cfg(feature = "mem_dbg")]
use mem_dbg::MemDbg;

use super::KeyRef;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie for sequences of the type `Label`.
pub struct Trie<Token>(pub map::Trie<Token, ()>);

impl<Token: Ord> Trie<Token> {
    /// Get a key reference for a label.
    pub fn get(&self, label: impl Label<Token>) -> Option<KeyRef<'_, Token>> {
        self.0.get(label).map(KeyRef)
    }

    /// Return true if `label` is an exact match.
    ///
    /// # Arguments
    /// * `label` - The label to search for.
    ///
    /// # Examples
    /// In the following example we illustrate how to query an exact match.
    ///
    /// ```rust
    /// use trie_rs::set::Trie;
    ///
    /// let trie = Trie::<u8>::from_iter(["a", "app", "apple", "better", "application"]);
    ///
    /// assert!(trie.is_exact("application"));
    /// assert!(trie.is_exact("app"));
    /// assert!(!trie.is_exact("appla"));
    ///
    /// ```
    pub fn is_exact(&self, label: impl Label<Token>) -> bool {
        self.0.get_value(label).is_some()
    }

    /// Return true if `label` is a prefix.
    ///
    /// Note: A prefix may be an exact match or not, and an exact match may be a prefix or not.
    pub fn is_prefix(&self, label: impl Label<Token>) -> bool {
        self.0.get(label).map(|n| n.is_prefix()).unwrap_or_default()
    }

    /// Return the common prefixes of `label`.
    ///
    /// # Arguments
    /// * `label` - The label to search for.
    ///
    /// # Examples
    /// In the following example we illustrate how to query the common prefixes of a label.
    ///
    /// ```rust
    /// use trie_rs::set::Trie;
    ///
    /// let trie = Trie::<u8>::from_iter(["a", "app", "apple", "better", "application"]);
    ///
    /// let results: Vec<String> = trie.prefixes_of("application").labels().collect::<Result<_, _>>().unwrap();
    ///
    /// assert_eq!(results, vec!["a", "app", "application"]);
    ///
    /// ```
    pub fn prefixes_of<L: Label<Token>>(
        &self,
        label: L,
    ) -> Keys<PrefixIter<'_, Token, (), L::IntoTokens>> {
        // TODO: We could return Keys iterators instead of collecting.
        self.0.prefixes_of(label).keys()
    }

    /// TODO
    pub fn prefixes_of_labels<L>(
        &self,
        label: impl Label<Token>,
    ) -> Labels<PrefixCollect<'_, Token, (), L>>
    where
        Token: Clone,
        L: TryFromTokens<Token>,
    {
        // TODO: We could return Keys iterators instead of collecting.
        Labels(self.0.prefixes_of_pairs(label))
    }

    /// Return all entries that start with `label`.
    pub fn starts_with(&self, label: impl Label<Token>) -> Keys<PostfixIter<'_, Token, ()>>
    where
        Token: Clone,
    {
        self.0.starts_with(label).keys()
    }

    /// Return all labels that start with `label`.
    pub fn starts_with_labels<L>(
        &self,
        label: impl Label<Token>,
    ) -> Labels<PostfixCollect<'_, Token, (), L>>
    where
        Token: Clone,
        L: TryFromTokens<Token>,
    {
        Labels(self.0.starts_with_pairs(label))
    }

    /// Return the suffixes of all entries that match `label`.
    ///
    /// # Arguments
    /// * `label` - The label to search for.
    ///
    /// # Examples
    /// In the following example we illustrate how to query the suffixes of a label.
    ///
    /// ```rust
    /// use trie_rs::set::Trie;
    ///
    /// let trie = Trie::<u8>::from_iter(["a", "app", "apple", "better", "application"]);
    ///
    /// let results: Vec<String> = trie.suffixes_of("application").labels().collect::<Result<_, _>>().unwrap();
    ///
    /// assert!(results.is_empty());
    ///
    /// let results: Vec<String> = trie.suffixes_of("app").labels().collect::<Result<_, _>>().unwrap();
    ///
    /// assert_eq!(results, vec!["le", "lication"]);
    ///
    /// ```
    pub fn suffixes_of(&self, label: impl Label<Token>) -> Keys<PostfixIter<'_, Token, ()>>
    where
        Token: Clone,
    {
        self.0.suffixes_of(label).keys()
    }

    /// Return the suffixes of all entries that match `label` as labels.
    ///
    /// # Arguments
    /// * `label` - The label to search for.
    ///
    /// # Examples
    /// In the following example we illustrate how to query the suffixes of a label.
    ///
    /// ```rust
    /// use trie_rs::set::Trie;
    ///
    /// let trie = Trie::<u8>::from_iter(["a", "app", "apple", "better", "application"]);
    ///
    /// let results: Vec<String> = trie.suffixes_of("application").labels().collect::<Result<_, _>>().unwrap();
    ///
    /// assert!(results.is_empty());
    ///
    /// let results: Vec<String> = trie.suffixes_of("app").labels().collect::<Result<_, _>>().unwrap();
    ///
    /// assert_eq!(results, vec!["le", "lication"]);
    ///
    /// ```
    pub fn suffixes_of_labels<L>(
        &self,
        label: impl Label<Token>,
    ) -> Labels<PostfixCollect<'_, Token, (), L>>
    where
        Token: Clone,
        L: TryFromTokens<Token>,
    {
        Labels(self.0.suffixes_of_pairs(label))
    }

    /// Returns an iterator across all keys in the trie.
    ///
    /// # Examples
    /// In the following example we illustrate how to iterate over all keys in the trie.
    /// Note that the order of the keys is not guaranteed, as they will be returned in
    /// lexicographical order.
    ///
    /// ```rust
    /// use trie_rs::set::Trie;
    ///
    /// let trie = Trie::<u8>::from_iter(["a", "app", "apple", "better", "application"]);
    ///
    /// let results: Vec<String> = trie.iter().labels().collect::<Result<_, _>>().unwrap();
    ///
    /// assert_eq!(results, vec!["a", "app", "apple", "application", "better"]);
    ///
    /// ```
    pub fn iter(&self) -> Keys<PostfixIter<'_, Token, ()>>
    where
        Token: Clone,
    {
        self.0.iter().keys()
    }

    /// Create an incremental search.
    /// Useful for interactive applications.
    /// See [crate::inc_search] for details.
    pub fn inc_search(&self) -> IncSearch<'_, Token, ()> {
        IncSearch::new(&self.0)
    }

    /// Return the longest shared prefix of `label`.
    pub fn longest_prefix<C, M>(&self, label: impl Label<Token>) -> Option<C>
    where
        C: TryFromIterator<Token, M>,
        Token: Clone,
    {
        self.0.longest_prefix(label)
    }
}

impl<Token, L> FromIterator<L> for Trie<Token>
where
    L: Label<Token>,
    Token: Ord + Clone,
{
    fn from_iter<T>(iter: T) -> Self
    where
        Self: Sized,
        T: IntoIterator<Item = L>,
    {
        let mut builder = super::TrieBuilder::new();
        for k in iter {
            builder.insert(k)
        }
        builder.build()
    }
}

#[cfg(test)]
mod search_tests {
    use crate::set::{Trie, TrieBuilder};
    use std::iter::FromIterator;

    fn build_trie() -> Trie<u8> {
        let mut builder = TrieBuilder::new();
        builder.insert("a");
        builder.insert("app");
        builder.insert("apple");
        builder.insert("better");
        builder.insert("application");
        builder.insert("アップル🍎");
        builder.build()
    }

    #[test]
    fn trie_from_iter() {
        let trie = Trie::<u8>::from_iter(["a", "app", "apple", "better", "application"]);
        assert!(trie.is_exact("application"));
    }

    #[test]
    fn collect_a_trie() {
        let trie: Trie<u8> =
            IntoIterator::into_iter(["a", "app", "apple", "better", "application"]).collect();
        assert!(trie.is_exact("application"));
    }

    #[test]
    fn clone() {
        let trie = build_trie();
        let _c: Trie<u8> = trie.clone();
    }

    #[rustfmt::skip]
    #[test]
    fn print_debug() {
        let trie: Trie<u8> = ["a"].into_iter().collect();
        assert_eq!(format!("{:?}", trie),
"Trie(Trie { louds: Louds { lbs: Fid { byte_vec: [160], bit_len: 5, chunks: Chunks { chunks: [Chunk { value: 2, blocks: Blocks { blocks: [Block { value: 1, length: 1 }, Block { value: 1, length: 1 }, Block { value: 2, length: 1 }, Block { value: 2, length: 1 }], blocks_cnt: 4 } }, Chunk { value: 2, blocks: Blocks { blocks: [Block { value: 0, length: 1 }], blocks_cnt: 1 } }], chunks_cnt: 2 }, table: PopcountTable { bit_length: 1, table: [0, 1] } } }, nodes: [Node { token: 97, value: Some(()) }] })"
        );
    }

    #[rustfmt::skip]
    #[test]
    fn print_debug_builder() {

        let mut builder: TrieBuilder<u8> = TrieBuilder::new();
        builder.insert("a");
        builder.insert("app");
        assert_eq!(format!("{:?}", builder),
"TrieBuilder(TrieBuilder { naive_trie: Root(NaiveTrieRoot { children: [IntermOrLeaf(NaiveTrieIntermOrLeaf { children: [IntermOrLeaf(NaiveTrieIntermOrLeaf { children: [IntermOrLeaf(NaiveTrieIntermOrLeaf { children: [], token: 112, value: Some(()) })], token: 112, value: None })], token: 97, value: Some(()) })] }) })"
        );
    }

    #[test]
    fn use_empty_queries() {
        let trie = build_trie();
        assert!(!trie.is_exact(""));
        let _ = trie.starts_with("").next();
        let _ = trie.suffixes_of("").next();
        let _ = trie.prefixes_of("").next();
    }

    #[cfg(feature = "mem_dbg")]
    #[test]
    /// ```sh
    /// cargo test --features mem_dbg memsize -- --nocapture
    /// ```
    fn memsize() {
        use mem_dbg::*;
        use std::{
            env,
            fs::File,
            io::{BufRead, BufReader},
        };

        const COUNT: usize = 100;
        let mut builder = TrieBuilder::new();

        let repo_root = env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable must be set.");
        let edict2_path = format!("{}/benches/edict.furigana", repo_root);
        println!("Reading dictionary file from: {}", edict2_path);

        let mut n_words = 0;
        let mut accum = 0;
        for result in BufReader::new(File::open(edict2_path).unwrap())
            .lines()
            .take(COUNT)
        {
            let l = result.unwrap();
            accum += l.len();
            builder.push(l);
            n_words += 1;
        }
        println!("Read {} words, {} bytes.", n_words, accum);

        let trie = builder.build();
        let trie_size = trie.mem_size(SizeFlags::default());
        eprintln!("Trie size {trie_size}");
        let uncompressed: Vec<String> = trie.iter().collect();
        let uncompressed_size = uncompressed.mem_size(SizeFlags::default());
        eprintln!("Uncompressed size {}", uncompressed_size);
        assert!(accum < trie_size); // This seems wrong to me.
        assert!(trie_size < uncompressed_size);
    }

    mod exact_match_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.is_exact(label);
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", true),
            t2: ("app", true),
            t3: ("apple", true),
            t4: ("application", true),
            t5: ("better", true),
            t6: ("アップル🍎", true),
            t7: ("appl", false),
            t8: ("appler", false),
        }
    }

    mod is_prefix_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.is_prefix(label);
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", true),
            t2: ("app", true),
            t3: ("apple", false),
            t4: ("application", false),
            t5: ("better", false),
            t6: ("アップル🍎", false),
            t7: ("appl", true),
            t8: ("appler", false),
            t9: ("アップル", true),
            t10: ("ed", false),
            t11: ("e", false),
            t12: ("", true),
        }
    }

    mod predictive_search_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Vec<String> = trie.starts_with(label).labels().collect::<Result<_, _>>().unwrap();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec!["a", "app", "apple", "application"]),
            t2: ("app", vec!["app", "apple", "application"]),
            t3: ("appl", vec!["apple", "application"]),
            t4: ("apple", vec!["apple"]),
            t5: ("b", vec!["better"]),
            t6: ("c", Vec::<&str>::new()),
            t7: ("アップ", vec!["アップル🍎"]),
        }
    }

    mod common_prefix_search_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Vec<String> = trie.prefixes_of(label).labels().collect::<Result<_, _>>().unwrap();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec!["a"]),
            t2: ("ap", vec!["a"]),
            t3: ("appl", vec!["a", "app"]),
            t4: ("appler", vec!["a", "app", "apple"]),
            t5: ("bette", Vec::<&str>::new()),
            t6: ("betterment", vec!["better"]),
            t7: ("c", Vec::<&str>::new()),
            t8: ("アップル🍎🍏", vec!["アップル🍎"]),
        }
    }
}
