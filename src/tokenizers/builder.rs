use crate::decoders::Decoder;
use crate::models::Model;
use crate::normalizers::Normalizer;

use super::post_processor::PostProcessor;
use super::pre_tokenizer::PreTokenizer;

pub struct TokenizerBuilder<M, N, PT, PP, D> {
    model: Option<M>,
    normalizer: Option<N>,
    pre_tokenizer: Option<PT>,
    post_processor: Option<PP>,
    decoder: Option<D>,
}

impl<M, N, PT, PP, D> Default for TokenizerBuilder<M, N, PT, PP, D>
where
    M: Model,
    N: Normalizer,
    PT: PreTokenizer,
    PP: PostProcessor,
    D: Decoder,
{
    fn default() -> Self {
        Self {}
    }
}
