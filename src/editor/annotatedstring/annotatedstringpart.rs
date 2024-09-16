use super::AnotationType;

#[derive(Debug)]
pub struct AnnotatedStringPart<'a>{
    pub string: &'a string,
    pub annotation_type: Option<AnnotationType>,
}

