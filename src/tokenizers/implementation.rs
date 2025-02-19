use crate::decoders::Decoder;
use crate::models::Model;
use crate::normalizers::Normalizer;

use super::extensions::VocabExtension;
use super::post_processor::PostProcessor;
use super::pre_tokenizer::PreTokenizer;

#[derive(Debug, Clone)]
pub struct TokenizerImpl<M, N, PT, PP, D> {
    model: M,
    normalizer: Option<N>,
    pre_tokenizer: Option<PT>,
    post_processor: Option<PP>,
    decoder: Option<D>,
    vocab_extension: VocabExtension,
}

impl<M, N, PT, PP, D> TokenizerImpl<M, N, PT, PP, D>
where
    M: Model,
    N: Normalizer,
    PT: PreTokenizer,
    PP: PostProcessor,
    D: Decoder,
{
    pub fn new(model: M) -> Self {
        Self {
            model,
            normalizer: None,
            pre_tokenizer: None,
            post_processor: None,
            decoder: None,
            vocab_extension: VocabExtension::new(),
        }
    }
}
