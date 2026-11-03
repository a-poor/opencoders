//! Constructors and checked collections for form creation and answers.
use super::*;
use serde::{Deserialize, Serialize};

/// Invalid locally checked form input. The server owns semantic validation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FormInputError {
    /// A form must contain at least one field.
    #[error("a form must contain at least one field")]
    EmptyFields,
}

/// Nonempty ordered form fields. Duplicate keys and condition semantics are
/// validated by the server; the library does not render or evaluate a form.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct FormFieldsInput(Vec<FormFieldInput>);
impl FormFieldsInput {
    /// Create a nonempty list with one field.
    pub fn new(first: impl Into<FormFieldInput>) -> Self {
        Self(vec![first.into()])
    }
    /// Append another field, preserving caller order.
    pub fn push(&mut self, field: impl Into<FormFieldInput>) {
        self.0.push(field.into());
    }
    /// Inspect the fields.
    pub fn as_slice(&self) -> &[FormFieldInput] {
        &self.0
    }
    /// Take ownership of the fields.
    pub fn into_vec(self) -> Vec<FormFieldInput> {
        self.0
    }
}
impl TryFrom<Vec<FormFieldInput>> for FormFieldsInput {
    type Error = FormInputError;
    fn try_from(fields: Vec<FormFieldInput>) -> Result<Self, Self::Error> {
        if fields.is_empty() {
            Err(FormInputError::EmptyFields)
        } else {
            Ok(Self(fields))
        }
    }
}
impl<'de> Deserialize<'de> for FormFieldsInput {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Self::try_from(Vec::<FormFieldInput>::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}
impl SessionFormCreateRequest {
    /// Create a pending form without starting a model call or waiting for a reply.
    pub fn new(title: impl Into<String>, fields: FormFieldsInput) -> Self {
        Self {
            title: title.into(),
            fields,
            id: RequestField::Omitted,
            metadata: None,
        }
    }
    /// Use a caller-provided ID. Duplicate IDs are conflicts, not idempotent writes.
    pub fn id(mut self, id: FormId) -> Self {
        self.id = RequestField::Value(id);
        self
    }
    /// Attach application metadata; empty maps are preserved.
    pub fn metadata(mut self, metadata: FormMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}
impl SessionFormReplyRequest {
    /// Submit the caller's explicit answers. Defaults are not applied locally.
    pub fn new(answer: FormAnswer) -> Self {
        Self { answer }
    }
}
impl FormOptionInput {
    /// An option's wire value and presentation label.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
        }
    }
}
impl FormWhenInput {
    /// A condition on an earlier field; all conditions must match on the server.
    pub fn new(key: impl Into<String>, op: FormWhenInputOp, value: FormConditionValue) -> Self {
        Self {
            key: key.into(),
            op,
            value,
        }
    }
}
impl FormStringFieldInput {
    /// A string field with all optional properties omitted.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            type_: FormStringFieldInputType::String,
            title: None,
            description: None,
            required: None,
            hidden: None,
            when: None,
            format: None,
            min_length: None,
            max_length: None,
            pattern: None,
            placeholder: None,
            default: None,
            options: None,
            custom: None,
        }
    }
}
impl FormNumberFieldInput {
    /// A numeric field with all optional properties omitted.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            type_: FormNumberFieldInputType::Number,
            title: None,
            description: None,
            required: None,
            hidden: None,
            when: None,
            minimum: None,
            maximum: None,
            default: None,
        }
    }
}
impl FormIntegerFieldInput {
    /// An integer field. The server enforces integer answers.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            type_: FormIntegerFieldInputType::Integer,
            title: None,
            description: None,
            required: None,
            hidden: None,
            when: None,
            minimum: None,
            maximum: None,
            default: None,
        }
    }
}
impl FormBooleanFieldInput {
    /// A boolean field with all optional properties omitted.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            type_: FormBooleanFieldInputType::Boolean,
            title: None,
            description: None,
            required: None,
            hidden: None,
            when: None,
            default: None,
        }
    }
}
impl FormMultiselectFieldInput {
    /// A multiselect field. An empty options list is sent unchanged.
    pub fn new(key: impl Into<String>, options: Vec<FormOptionInput>) -> Self {
        Self {
            key: key.into(),
            type_: FormMultiselectFieldInputType::Multiselect,
            title: None,
            description: None,
            required: None,
            hidden: None,
            when: None,
            options,
            min_items: None,
            max_items: None,
            custom: None,
            default: None,
        }
    }
}
impl FormExternalFieldInput {
    /// An external action; v2.2.5 requires an explicit `true` acknowledgement.
    /// The library does not open or fetch the URL.
    pub fn new(key: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            url: url.into(),
            type_: FormExternalFieldInputType::External,
            title: None,
            description: None,
        }
    }
}
macro_rules! field_from {($($variant:ident:$ty:ty),*)=>{$(impl From<$ty> for FormFieldInput{fn from(value:$ty)->Self{Self::$variant(Box::new(value))}})*};}
field_from!(String:FormStringFieldInput,Number:FormNumberFieldInput,Integer:FormIntegerFieldInput,Boolean:FormBooleanFieldInput,Multiselect:FormMultiselectFieldInput,External:FormExternalFieldInput);
