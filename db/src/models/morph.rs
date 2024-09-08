use super::MediaType;

use super::CommentType;

pub trait Morph: Sized {
    fn model_id(&self) -> i32;
    fn media_type() -> MediaType;
    fn coomment_type() -> CommentType;
}
