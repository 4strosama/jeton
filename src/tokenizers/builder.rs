use crate::decoders::Decoder;
use crate::models::Model;
use crate::normalizers::Normalizer;

use super::extensions::VocabExtension;
use super::implementation::TokenizerImpl;
use super::post_processor::PostProcessor;
use super::pre_tokenizer::PreTokenizer;

pub struct TokenizerBuilder<M, N, PT, PP, D> {
    pub model: Option<M>,
    pub normalizer: Option<N>,
    pub pre_tokenizer: Option<PT>,
    pub post_processor: Option<PP>,
    pub decoder: Option<D>,
    pub vocab_extension: VocabExtension,
}

impl<M, N, PT, PP, D> TokenizerBuilder<M, N, PT, PP, D>
where
    M: Model,
    N: Normalizer,
    PT: PreTokenizer,
    PP: PostProcessor,
    D: Decoder,
{
    pub fn new() -> Self {
        Self {
            model: None,
            normalizer: None,
            pre_tokenizer: None,
            post_processor: None,
            decoder: None,
            vocab_extension: VocabExtension::new(),
        }
    }

    pub fn build(self) -> Result<TokenizerImpl<M, N, PT, PP, D>> {
        let model = self.model.unwrap();

        Ok(TokenizerImpl {
            model,
            normalizer: self.normalizer,
            pre_tokenizer: self.pre_tokenizer,
            post_processor: self.post_processor,
            decoder: self.decoder,
            vocab_extension: self.vocab_extension,
        })
    }
}
