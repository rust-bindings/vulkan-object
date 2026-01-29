#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`Bitmask`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Bitmask\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"bitWidth\","]
#[doc = "    \"extensions\","]
#[doc = "    \"flagExtensions\","]
#[doc = "    \"flagName\","]
#[doc = "    \"flags\","]
#[doc = "    \"name\","]
#[doc = "    \"protect\","]
#[doc = "    \"returnedOnly\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"bitWidth\": {"]
#[doc = "      \"title\": \"Bitwidth\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flagExtensions\": {"]
#[doc = "      \"title\": \"Flagextensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flagName\": {"]
#[doc = "      \"title\": \"Flagname\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"flags\": {"]
#[doc = "      \"title\": \"Flags\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Flag\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"returnedOnly\": {"]
#[doc = "      \"title\": \"Returnedonly\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Bitmask {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "bitWidth")]
    pub bit_width: i64,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "flagExtensions")]
    pub flag_extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "flagName")]
    pub flag_name: ::std::string::String,
    pub flags: ::std::vec::Vec<Flag>,
    pub name: ::std::string::String,
    pub protect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
}
impl ::std::convert::From<&Bitmask> for Bitmask {
    fn from(value: &Bitmask) -> Self {
        value.clone()
    }
}
impl Bitmask {
    pub fn builder() -> builder::Bitmask {
        Default::default()
    }
}
#[doc = "`Command`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Command\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alias\","]
#[doc = "    \"allowNoQueues\","]
#[doc = "    \"cFunctionPointer\","]
#[doc = "    \"cPrototype\","]
#[doc = "    \"device\","]
#[doc = "    \"errorCodes\","]
#[doc = "    \"extensions\","]
#[doc = "    \"implicitExternSyncParams\","]
#[doc = "    \"instance\","]
#[doc = "    \"legacy\","]
#[doc = "    \"name\","]
#[doc = "    \"params\","]
#[doc = "    \"primary\","]
#[doc = "    \"protect\","]
#[doc = "    \"queues\","]
#[doc = "    \"renderPass\","]
#[doc = "    \"returnType\","]
#[doc = "    \"secondary\","]
#[doc = "    \"successCodes\","]
#[doc = "    \"tasks\","]
#[doc = "    \"version\","]
#[doc = "    \"videoCoding\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alias\": {"]
#[doc = "      \"title\": \"Alias\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"allowNoQueues\": {"]
#[doc = "      \"title\": \"Allownoqueues\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"cFunctionPointer\": {"]
#[doc = "      \"title\": \"Cfunctionpointer\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"cPrototype\": {"]
#[doc = "      \"title\": \"Cprototype\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"device\": {"]
#[doc = "      \"title\": \"Device\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"errorCodes\": {"]
#[doc = "      \"title\": \"Errorcodes\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"implicitExternSyncParams\": {"]
#[doc = "      \"title\": \"Implicitexternsyncparams\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"instance\": {"]
#[doc = "      \"title\": \"Instance\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"legacy\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/Legacy\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"title\": \"Params\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Param\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"primary\": {"]
#[doc = "      \"title\": \"Primary\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"queues\": {"]
#[doc = "      \"title\": \"Queues\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"renderPass\": {"]
#[doc = "      \"title\": \"Renderpass\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"returnType\": {"]
#[doc = "      \"title\": \"Returntype\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"secondary\": {"]
#[doc = "      \"title\": \"Secondary\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"successCodes\": {"]
#[doc = "      \"title\": \"Successcodes\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tasks\": {"]
#[doc = "      \"title\": \"Tasks\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/Version\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"videoCoding\": {"]
#[doc = "      \"title\": \"Videocoding\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Command {
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "allowNoQueues")]
    pub allow_no_queues: bool,
    #[serde(rename = "cFunctionPointer")]
    pub c_function_pointer: ::std::string::String,
    #[serde(rename = "cPrototype")]
    pub c_prototype: ::std::string::String,
    pub device: bool,
    #[serde(rename = "errorCodes")]
    pub error_codes: ::std::vec::Vec<::std::string::String>,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "implicitExternSyncParams")]
    pub implicit_extern_sync_params: ::std::vec::Vec<::std::string::String>,
    pub instance: bool,
    pub legacy: ::std::option::Option<Legacy>,
    pub name: ::std::string::String,
    pub params: ::std::vec::Vec<Param>,
    pub primary: bool,
    pub protect: ::std::option::Option<::std::string::String>,
    pub queues: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "renderPass")]
    pub render_pass: i64,
    #[serde(rename = "returnType")]
    pub return_type: ::std::string::String,
    pub secondary: bool,
    #[serde(rename = "successCodes")]
    pub success_codes: ::std::vec::Vec<::std::string::String>,
    pub tasks: ::std::vec::Vec<::std::string::String>,
    pub version: ::std::option::Option<Version>,
    #[serde(rename = "videoCoding")]
    pub video_coding: i64,
}
impl ::std::convert::From<&Command> for Command {
    fn from(value: &Command) -> Self {
        value.clone()
    }
}
impl Command {
    pub fn builder() -> builder::Command {
        Default::default()
    }
}
#[doc = "`Constant`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\","]
#[doc = "    \"value\","]
#[doc = "    \"valueStr\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"valueStr\": {"]
#[doc = "      \"title\": \"Valuestr\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"videoStdHeader\": {"]
#[doc = "      \"title\": \"Videostdheader\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Constant {
    pub name: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    pub value: Value,
    #[serde(rename = "valueStr")]
    pub value_str: ::std::string::String,
    #[serde(
        rename = "videoStdHeader",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_std_header: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Constant> for Constant {
    fn from(value: &Constant) -> Self {
        value.clone()
    }
}
impl Constant {
    pub fn builder() -> builder::Constant {
        Default::default()
    }
}
#[doc = "`Enum`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Enum\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"bitWidth\","]
#[doc = "    \"extensions\","]
#[doc = "    \"fieldExtensions\","]
#[doc = "    \"fields\","]
#[doc = "    \"name\","]
#[doc = "    \"protect\","]
#[doc = "    \"returnedOnly\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"bitWidth\": {"]
#[doc = "      \"title\": \"Bitwidth\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fieldExtensions\": {"]
#[doc = "      \"title\": \"Fieldextensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"Fields\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/EnumField\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"returnedOnly\": {"]
#[doc = "      \"title\": \"Returnedonly\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"videoStdHeader\": {"]
#[doc = "      \"title\": \"Videostdheader\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Enum {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "bitWidth")]
    pub bit_width: i64,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "fieldExtensions")]
    pub field_extensions: ::std::vec::Vec<::std::string::String>,
    pub fields: ::std::vec::Vec<EnumField>,
    pub name: ::std::string::String,
    pub protect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
    #[serde(
        rename = "videoStdHeader",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_std_header: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Enum> for Enum {
    fn from(value: &Enum) -> Self {
        value.clone()
    }
}
impl Enum {
    pub fn builder() -> builder::Enum {
        Default::default()
    }
}
#[doc = "`EnumField`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"EnumField\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"extensions\","]
#[doc = "    \"name\","]
#[doc = "    \"negative\","]
#[doc = "    \"protect\","]
#[doc = "    \"value\","]
#[doc = "    \"valueStr\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"negative\": {"]
#[doc = "      \"title\": \"Negative\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"valueStr\": {"]
#[doc = "      \"title\": \"Valuestr\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EnumField {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    pub name: ::std::string::String,
    pub negative: bool,
    pub protect: ::std::option::Option<::std::string::String>,
    pub value: i64,
    #[serde(rename = "valueStr")]
    pub value_str: ::std::string::String,
}
impl ::std::convert::From<&EnumField> for EnumField {
    fn from(value: &EnumField) -> Self {
        value.clone()
    }
}
impl EnumField {
    pub fn builder() -> builder::EnumField {
        Default::default()
    }
}
#[doc = "`Extension`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Extension\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"depends\","]
#[doc = "    \"deprecatedBy\","]
#[doc = "    \"device\","]
#[doc = "    \"featureRequirement\","]
#[doc = "    \"instance\","]
#[doc = "    \"name\","]
#[doc = "    \"nameString\","]
#[doc = "    \"obsoletedBy\","]
#[doc = "    \"platform\","]
#[doc = "    \"promotedTo\","]
#[doc = "    \"protect\","]
#[doc = "    \"provisional\","]
#[doc = "    \"ratified\","]
#[doc = "    \"specVersion\","]
#[doc = "    \"specialUse\","]
#[doc = "    \"vendorTag\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bitmasks\": {"]
#[doc = "      \"title\": \"Bitmasks\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Bitmask\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"commands\": {"]
#[doc = "      \"title\": \"Commands\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Command\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"depends\": {"]
#[doc = "      \"title\": \"Depends\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"deprecatedBy\": {"]
#[doc = "      \"title\": \"Deprecatedby\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"device\": {"]
#[doc = "      \"title\": \"Device\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"enumFields\": {"]
#[doc = "      \"title\": \"Enumfields\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"$ref\": \"#/$defs/EnumField\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"enums\": {"]
#[doc = "      \"title\": \"Enums\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Enum\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"featureRequirement\": {"]
#[doc = "      \"title\": \"Featurerequirement\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/FeatureRequirement\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flagBits\": {"]
#[doc = "      \"title\": \"Flagbits\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"$ref\": \"#/$defs/Flag\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flags\": {"]
#[doc = "      \"title\": \"Flags\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"$ref\": \"#/$defs/Flags\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"handles\": {"]
#[doc = "      \"title\": \"Handles\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Handle\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"instance\": {"]
#[doc = "      \"title\": \"Instance\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameString\": {"]
#[doc = "      \"title\": \"Namestring\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"obsoletedBy\": {"]
#[doc = "      \"title\": \"Obsoletedby\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"platform\": {"]
#[doc = "      \"title\": \"Platform\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"promotedTo\": {"]
#[doc = "      \"title\": \"Promotedto\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"provisional\": {"]
#[doc = "      \"title\": \"Provisional\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"ratified\": {"]
#[doc = "      \"title\": \"Ratified\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"specVersion\": {"]
#[doc = "      \"title\": \"Specversion\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"specialUse\": {"]
#[doc = "      \"title\": \"Specialuse\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"structs\": {"]
#[doc = "      \"title\": \"Structs\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Struct\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"vendorTag\": {"]
#[doc = "      \"title\": \"Vendortag\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Extension {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub bitmasks: ::std::vec::Vec<Bitmask>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub commands: ::std::vec::Vec<Command>,
    pub depends: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deprecatedBy")]
    pub deprecated_by: ::std::option::Option<::std::string::String>,
    pub device: bool,
    #[serde(
        rename = "enumFields",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub enum_fields: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<EnumField>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub enums: ::std::vec::Vec<Enum>,
    #[serde(rename = "featureRequirement")]
    pub feature_requirement: ::std::vec::Vec<FeatureRequirement>,
    #[serde(
        rename = "flagBits",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub flag_bits: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<Flag>>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub flags: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<Flags>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub handles: ::std::vec::Vec<Handle>,
    pub instance: bool,
    pub name: ::std::string::String,
    #[serde(rename = "nameString")]
    pub name_string: ::std::string::String,
    #[serde(rename = "obsoletedBy")]
    pub obsoleted_by: ::std::option::Option<::std::string::String>,
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(rename = "promotedTo")]
    pub promoted_to: ::std::option::Option<::std::string::String>,
    pub protect: ::std::option::Option<::std::string::String>,
    pub provisional: bool,
    pub ratified: bool,
    #[serde(rename = "specVersion")]
    pub spec_version: ::std::string::String,
    #[serde(rename = "specialUse")]
    pub special_use: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub structs: ::std::vec::Vec<Struct>,
    #[serde(rename = "vendorTag")]
    pub vendor_tag: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Extension> for Extension {
    fn from(value: &Extension) -> Self {
        value.clone()
    }
}
impl Extension {
    pub fn builder() -> builder::Extension {
        Default::default()
    }
}
#[doc = "`FeatureRequirement`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FeatureRequirement\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"depends\","]
#[doc = "    \"field\","]
#[doc = "    \"struct\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"depends\": {"]
#[doc = "      \"title\": \"Depends\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"field\": {"]
#[doc = "      \"title\": \"Field\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"struct\": {"]
#[doc = "      \"title\": \"Struct\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FeatureRequirement {
    pub depends: ::std::option::Option<::std::string::String>,
    pub field: ::std::string::String,
    #[serde(rename = "struct")]
    pub struct_: ::std::string::String,
}
impl ::std::convert::From<&FeatureRequirement> for FeatureRequirement {
    fn from(value: &FeatureRequirement) -> Self {
        value.clone()
    }
}
impl FeatureRequirement {
    pub fn builder() -> builder::FeatureRequirement {
        Default::default()
    }
}
#[doc = "`Flag`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Flag\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"extensions\","]
#[doc = "    \"multiBit\","]
#[doc = "    \"name\","]
#[doc = "    \"protect\","]
#[doc = "    \"value\","]
#[doc = "    \"valueStr\","]
#[doc = "    \"zero\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"multiBit\": {"]
#[doc = "      \"title\": \"Multibit\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"valueStr\": {"]
#[doc = "      \"title\": \"Valuestr\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"zero\": {"]
#[doc = "      \"title\": \"Zero\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Flag {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "multiBit")]
    pub multi_bit: bool,
    pub name: ::std::string::String,
    pub protect: ::std::option::Option<::std::string::String>,
    pub value: i64,
    #[serde(rename = "valueStr")]
    pub value_str: ::std::string::String,
    pub zero: bool,
}
impl ::std::convert::From<&Flag> for Flag {
    fn from(value: &Flag) -> Self {
        value.clone()
    }
}
impl Flag {
    pub fn builder() -> builder::Flag {
        Default::default()
    }
}
#[doc = "`Flags`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Flags\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"baseFlagsType\","]
#[doc = "    \"bitWidth\","]
#[doc = "    \"bitmaskName\","]
#[doc = "    \"extensions\","]
#[doc = "    \"name\","]
#[doc = "    \"protect\","]
#[doc = "    \"returnedOnly\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"baseFlagsType\": {"]
#[doc = "      \"title\": \"Baseflagstype\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"bitWidth\": {"]
#[doc = "      \"title\": \"Bitwidth\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"bitmaskName\": {"]
#[doc = "      \"title\": \"Bitmaskname\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"returnedOnly\": {"]
#[doc = "      \"title\": \"Returnedonly\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Flags {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "baseFlagsType")]
    pub base_flags_type: ::std::string::String,
    #[serde(rename = "bitWidth")]
    pub bit_width: i64,
    #[serde(rename = "bitmaskName")]
    pub bitmask_name: ::std::option::Option<::std::string::String>,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    pub name: ::std::string::String,
    pub protect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
}
impl ::std::convert::From<&Flags> for Flags {
    fn from(value: &Flags) -> Self {
        value.clone()
    }
}
impl Flags {
    pub fn builder() -> builder::Flags {
        Default::default()
    }
}
#[doc = "`Format`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Format\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"blockExtent\","]
#[doc = "    \"blockSize\","]
#[doc = "    \"chroma\","]
#[doc = "    \"className\","]
#[doc = "    \"components\","]
#[doc = "    \"compressed\","]
#[doc = "    \"name\","]
#[doc = "    \"packed\","]
#[doc = "    \"planes\","]
#[doc = "    \"spirvImageFormat\","]
#[doc = "    \"texelsPerBlock\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"blockExtent\": {"]
#[doc = "      \"title\": \"Blockextent\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"blockSize\": {"]
#[doc = "      \"title\": \"Blocksize\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"chroma\": {"]
#[doc = "      \"title\": \"Chroma\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"className\": {"]
#[doc = "      \"title\": \"Classname\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"components\": {"]
#[doc = "      \"title\": \"Components\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/FormatComponent\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"compressed\": {"]
#[doc = "      \"title\": \"Compressed\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"packed\": {"]
#[doc = "      \"title\": \"Packed\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"planes\": {"]
#[doc = "      \"title\": \"Planes\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/FormatPlane\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"spirvImageFormat\": {"]
#[doc = "      \"title\": \"Spirvimageformat\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"texelsPerBlock\": {"]
#[doc = "      \"title\": \"Texelsperblock\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Format {
    #[serde(rename = "blockExtent")]
    pub block_extent: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "blockSize")]
    pub block_size: i64,
    pub chroma: ::std::option::Option<::std::string::String>,
    #[serde(rename = "className")]
    pub class_name: ::std::string::String,
    pub components: ::std::vec::Vec<FormatComponent>,
    pub compressed: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub packed: ::std::option::Option<i64>,
    pub planes: ::std::vec::Vec<FormatPlane>,
    #[serde(rename = "spirvImageFormat")]
    pub spirv_image_format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "texelsPerBlock")]
    pub texels_per_block: i64,
}
impl ::std::convert::From<&Format> for Format {
    fn from(value: &Format) -> Self {
        value.clone()
    }
}
impl Format {
    pub fn builder() -> builder::Format {
        Default::default()
    }
}
#[doc = "`FormatComponent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FormatComponent\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bits\","]
#[doc = "    \"numericFormat\","]
#[doc = "    \"planeIndex\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bits\": {"]
#[doc = "      \"title\": \"Bits\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"numericFormat\": {"]
#[doc = "      \"title\": \"Numericformat\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"planeIndex\": {"]
#[doc = "      \"title\": \"Planeindex\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FormatComponent {
    pub bits: ::std::string::String,
    #[serde(rename = "numericFormat")]
    pub numeric_format: ::std::string::String,
    #[serde(rename = "planeIndex")]
    pub plane_index: ::std::option::Option<i64>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&FormatComponent> for FormatComponent {
    fn from(value: &FormatComponent) -> Self {
        value.clone()
    }
}
impl FormatComponent {
    pub fn builder() -> builder::FormatComponent {
        Default::default()
    }
}
#[doc = "`FormatPlane`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FormatPlane\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"compatible\","]
#[doc = "    \"heightDivisor\","]
#[doc = "    \"index\","]
#[doc = "    \"widthDivisor\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"compatible\": {"]
#[doc = "      \"title\": \"Compatible\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"heightDivisor\": {"]
#[doc = "      \"title\": \"Heightdivisor\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"title\": \"Index\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"widthDivisor\": {"]
#[doc = "      \"title\": \"Widthdivisor\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FormatPlane {
    pub compatible: ::std::string::String,
    #[serde(rename = "heightDivisor")]
    pub height_divisor: i64,
    pub index: i64,
    #[serde(rename = "widthDivisor")]
    pub width_divisor: i64,
}
impl ::std::convert::From<&FormatPlane> for FormatPlane {
    fn from(value: &FormatPlane) -> Self {
        value.clone()
    }
}
impl FormatPlane {
    pub fn builder() -> builder::FormatPlane {
        Default::default()
    }
}
#[doc = "`Handle`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Handle\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"device\","]
#[doc = "    \"dispatchable\","]
#[doc = "    \"extensions\","]
#[doc = "    \"instance\","]
#[doc = "    \"name\","]
#[doc = "    \"parent\","]
#[doc = "    \"protect\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"device\": {"]
#[doc = "      \"title\": \"Device\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"dispatchable\": {"]
#[doc = "      \"title\": \"Dispatchable\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"instance\": {"]
#[doc = "      \"title\": \"Instance\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"parent\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/Handle\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Handle {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    pub device: bool,
    pub dispatchable: bool,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    pub instance: bool,
    pub name: ::std::string::String,
    pub parent: ::std::option::Option<::std::boxed::Box<Handle>>,
    pub protect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&Handle> for Handle {
    fn from(value: &Handle) -> Self {
        value.clone()
    }
}
impl Handle {
    pub fn builder() -> builder::Handle {
        Default::default()
    }
}
#[doc = "`Legacy`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Legacy\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"extensions\","]
#[doc = "    \"link\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"link\": {"]
#[doc = "      \"title\": \"Link\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/Version\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Legacy {
    pub extensions: ::std::vec::Vec<::std::string::String>,
    pub link: ::std::option::Option<::std::string::String>,
    pub version: ::std::option::Option<Version>,
}
impl ::std::convert::From<&Legacy> for Legacy {
    fn from(value: &Legacy) -> Self {
        value.clone()
    }
}
impl Legacy {
    pub fn builder() -> builder::Legacy {
        Default::default()
    }
}
#[doc = "`Member`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Member\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bitFieldWidth\","]
#[doc = "    \"cDeclaration\","]
#[doc = "    \"const\","]
#[doc = "    \"externSync\","]
#[doc = "    \"fixedSizeArray\","]
#[doc = "    \"fullType\","]
#[doc = "    \"length\","]
#[doc = "    \"limitType\","]
#[doc = "    \"name\","]
#[doc = "    \"noAutoValidity\","]
#[doc = "    \"nullTerminated\","]
#[doc = "    \"optional\","]
#[doc = "    \"optionalPointer\","]
#[doc = "    \"pointer\","]
#[doc = "    \"selection\","]
#[doc = "    \"selector\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bitFieldWidth\": {"]
#[doc = "      \"title\": \"Bitfieldwidth\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cDeclaration\": {"]
#[doc = "      \"title\": \"Cdeclaration\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"const\": {"]
#[doc = "      \"title\": \"Const\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"externSync\": {"]
#[doc = "      \"title\": \"Externsync\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"fixedSizeArray\": {"]
#[doc = "      \"title\": \"Fixedsizearray\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fullType\": {"]
#[doc = "      \"title\": \"Fulltype\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"length\": {"]
#[doc = "      \"title\": \"Length\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"limitType\": {"]
#[doc = "      \"title\": \"Limittype\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"noAutoValidity\": {"]
#[doc = "      \"title\": \"Noautovalidity\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"nullTerminated\": {"]
#[doc = "      \"title\": \"Nullterminated\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"optional\": {"]
#[doc = "      \"title\": \"Optional\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"optionalPointer\": {"]
#[doc = "      \"title\": \"Optionalpointer\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"pointer\": {"]
#[doc = "      \"title\": \"Pointer\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"selection\": {"]
#[doc = "      \"title\": \"Selection\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"selector\": {"]
#[doc = "      \"title\": \"Selector\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Member {
    #[serde(rename = "bitFieldWidth")]
    pub bit_field_width: ::std::option::Option<i64>,
    #[serde(rename = "cDeclaration")]
    pub c_declaration: ::std::string::String,
    #[serde(rename = "const")]
    pub const_: bool,
    #[serde(rename = "externSync")]
    pub extern_sync: i64,
    #[serde(rename = "fixedSizeArray")]
    pub fixed_size_array: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "fullType")]
    pub full_type: ::std::string::String,
    pub length: ::std::option::Option<::std::string::String>,
    #[serde(rename = "limitType")]
    pub limit_type: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(rename = "noAutoValidity")]
    pub no_auto_validity: bool,
    #[serde(rename = "nullTerminated")]
    pub null_terminated: bool,
    pub optional: bool,
    #[serde(rename = "optionalPointer")]
    pub optional_pointer: bool,
    pub pointer: bool,
    pub selection: ::std::vec::Vec<::std::string::String>,
    pub selector: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&Member> for Member {
    fn from(value: &Member) -> Self {
        value.clone()
    }
}
impl Member {
    pub fn builder() -> builder::Member {
        Default::default()
    }
}
#[doc = "`Param`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Param\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alias\","]
#[doc = "    \"cDeclaration\","]
#[doc = "    \"const\","]
#[doc = "    \"externSync\","]
#[doc = "    \"externSyncPointer\","]
#[doc = "    \"fixedSizeArray\","]
#[doc = "    \"fullType\","]
#[doc = "    \"length\","]
#[doc = "    \"name\","]
#[doc = "    \"noAutoValidity\","]
#[doc = "    \"nullTerminated\","]
#[doc = "    \"optional\","]
#[doc = "    \"optionalPointer\","]
#[doc = "    \"pointer\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alias\": {"]
#[doc = "      \"title\": \"Alias\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cDeclaration\": {"]
#[doc = "      \"title\": \"Cdeclaration\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"const\": {"]
#[doc = "      \"title\": \"Const\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"externSync\": {"]
#[doc = "      \"title\": \"Externsync\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"externSyncPointer\": {"]
#[doc = "      \"title\": \"Externsyncpointer\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"fixedSizeArray\": {"]
#[doc = "      \"title\": \"Fixedsizearray\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fullType\": {"]
#[doc = "      \"title\": \"Fulltype\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"length\": {"]
#[doc = "      \"title\": \"Length\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"noAutoValidity\": {"]
#[doc = "      \"title\": \"Noautovalidity\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"nullTerminated\": {"]
#[doc = "      \"title\": \"Nullterminated\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"optional\": {"]
#[doc = "      \"title\": \"Optional\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"optionalPointer\": {"]
#[doc = "      \"title\": \"Optionalpointer\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"pointer\": {"]
#[doc = "      \"title\": \"Pointer\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Param {
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cDeclaration")]
    pub c_declaration: ::std::string::String,
    #[serde(rename = "const")]
    pub const_: bool,
    #[serde(rename = "externSync")]
    pub extern_sync: i64,
    #[serde(rename = "externSyncPointer")]
    pub extern_sync_pointer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fixedSizeArray")]
    pub fixed_size_array: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "fullType")]
    pub full_type: ::std::string::String,
    pub length: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(rename = "noAutoValidity")]
    pub no_auto_validity: bool,
    #[serde(rename = "nullTerminated")]
    pub null_terminated: bool,
    pub optional: bool,
    #[serde(rename = "optionalPointer")]
    pub optional_pointer: bool,
    pub pointer: bool,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&Param> for Param {
    fn from(value: &Param) -> Self {
        value.clone()
    }
}
impl Param {
    pub fn builder() -> builder::Param {
        Default::default()
    }
}
#[doc = "`Spirv`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Spirv\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capability\","]
#[doc = "    \"enable\","]
#[doc = "    \"extension\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capability\": {"]
#[doc = "      \"title\": \"Capability\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"enable\": {"]
#[doc = "      \"title\": \"Enable\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SpirvEnables\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extension\": {"]
#[doc = "      \"title\": \"Extension\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Spirv {
    pub capability: bool,
    pub enable: ::std::vec::Vec<SpirvEnables>,
    pub extension: bool,
    pub name: ::std::string::String,
}
impl ::std::convert::From<&Spirv> for Spirv {
    fn from(value: &Spirv) -> Self {
        value.clone()
    }
}
impl Spirv {
    pub fn builder() -> builder::Spirv {
        Default::default()
    }
}
#[doc = "`SpirvEnables`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SpirvEnables\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"extension\","]
#[doc = "    \"feature\","]
#[doc = "    \"member\","]
#[doc = "    \"property\","]
#[doc = "    \"requires\","]
#[doc = "    \"struct\","]
#[doc = "    \"value\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"extension\": {"]
#[doc = "      \"title\": \"Extension\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"feature\": {"]
#[doc = "      \"title\": \"Feature\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"member\": {"]
#[doc = "      \"title\": \"Member\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"property\": {"]
#[doc = "      \"title\": \"Property\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requires\": {"]
#[doc = "      \"title\": \"Requires\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"struct\": {"]
#[doc = "      \"title\": \"Struct\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"title\": \"Version\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SpirvEnables {
    pub extension: ::std::option::Option<::std::string::String>,
    pub feature: ::std::option::Option<::std::string::String>,
    pub member: ::std::option::Option<::std::string::String>,
    pub property: ::std::option::Option<::std::string::String>,
    pub requires: ::std::option::Option<::std::string::String>,
    #[serde(rename = "struct")]
    pub struct_: ::std::option::Option<::std::string::String>,
    pub value: ::std::option::Option<::std::string::String>,
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SpirvEnables> for SpirvEnables {
    fn from(value: &SpirvEnables) -> Self {
        value.clone()
    }
}
impl SpirvEnables {
    pub fn builder() -> builder::SpirvEnables {
        Default::default()
    }
}
#[doc = "`Struct`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Struct\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"allowDuplicate\","]
#[doc = "    \"extendedBy\","]
#[doc = "    \"extends\","]
#[doc = "    \"extensions\","]
#[doc = "    \"members\","]
#[doc = "    \"name\","]
#[doc = "    \"protect\","]
#[doc = "    \"returnedOnly\","]
#[doc = "    \"sType\","]
#[doc = "    \"union\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"allowDuplicate\": {"]
#[doc = "      \"title\": \"Allowduplicate\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"extendedBy\": {"]
#[doc = "      \"title\": \"Extendedby\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extends\": {"]
#[doc = "      \"title\": \"Extends\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"members\": {"]
#[doc = "      \"title\": \"Members\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Member\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"returnedOnly\": {"]
#[doc = "      \"title\": \"Returnedonly\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"sType\": {"]
#[doc = "      \"title\": \"Stype\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"union\": {"]
#[doc = "      \"title\": \"Union\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/Version\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"videoStdHeader\": {"]
#[doc = "      \"title\": \"Videostdheader\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Struct {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowDuplicate")]
    pub allow_duplicate: bool,
    #[serde(rename = "extendedBy")]
    pub extended_by: ::std::vec::Vec<::std::string::String>,
    pub extends: ::std::vec::Vec<::std::string::String>,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    pub members: ::std::vec::Vec<Member>,
    pub name: ::std::string::String,
    pub protect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
    #[serde(rename = "sType")]
    pub s_type: ::std::option::Option<::std::string::String>,
    pub union: bool,
    pub version: ::std::option::Option<Version>,
    #[serde(
        rename = "videoStdHeader",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_std_header: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Struct> for Struct {
    fn from(value: &Struct) -> Self {
        value.clone()
    }
}
impl Struct {
    pub fn builder() -> builder::Struct {
        Default::default()
    }
}
#[doc = "`SyncAccess`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncAccess\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"equivalent\","]
#[doc = "    \"flag\","]
#[doc = "    \"support\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"equivalent\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncEquivalent\""]
#[doc = "    },"]
#[doc = "    \"flag\": {"]
#[doc = "      \"$ref\": \"#/$defs/Flag\""]
#[doc = "    },"]
#[doc = "    \"support\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncSupport\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncAccess {
    pub equivalent: SyncEquivalent,
    pub flag: Flag,
    pub support: SyncSupport,
}
impl ::std::convert::From<&SyncAccess> for SyncAccess {
    fn from(value: &SyncAccess) -> Self {
        value.clone()
    }
}
impl SyncAccess {
    pub fn builder() -> builder::SyncAccess {
        Default::default()
    }
}
#[doc = "`SyncEquivalent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncEquivalent\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"accesses\","]
#[doc = "    \"max\","]
#[doc = "    \"stages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"accesses\": {"]
#[doc = "      \"title\": \"Accesses\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Flag\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"max\": {"]
#[doc = "      \"title\": \"Max\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"stages\": {"]
#[doc = "      \"title\": \"Stages\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Flag\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncEquivalent {
    pub accesses: ::std::option::Option<::std::vec::Vec<Flag>>,
    pub max: bool,
    pub stages: ::std::option::Option<::std::vec::Vec<Flag>>,
}
impl ::std::convert::From<&SyncEquivalent> for SyncEquivalent {
    fn from(value: &SyncEquivalent) -> Self {
        value.clone()
    }
}
impl SyncEquivalent {
    pub fn builder() -> builder::SyncEquivalent {
        Default::default()
    }
}
#[doc = "`SyncPipeline`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncPipeline\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"depends\","]
#[doc = "    \"name\","]
#[doc = "    \"stages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"depends\": {"]
#[doc = "      \"title\": \"Depends\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stages\": {"]
#[doc = "      \"title\": \"Stages\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SyncPipelineStage\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncPipeline {
    pub depends: ::std::vec::Vec<::std::string::String>,
    pub name: ::std::string::String,
    pub stages: ::std::vec::Vec<SyncPipelineStage>,
}
impl ::std::convert::From<&SyncPipeline> for SyncPipeline {
    fn from(value: &SyncPipeline) -> Self {
        value.clone()
    }
}
impl SyncPipeline {
    pub fn builder() -> builder::SyncPipeline {
        Default::default()
    }
}
#[doc = "`SyncPipelineStage`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncPipelineStage\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"after\","]
#[doc = "    \"before\","]
#[doc = "    \"order\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"after\": {"]
#[doc = "      \"title\": \"After\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"before\": {"]
#[doc = "      \"title\": \"Before\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"order\": {"]
#[doc = "      \"title\": \"Order\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncPipelineStage {
    pub after: ::std::option::Option<::std::string::String>,
    pub before: ::std::option::Option<::std::string::String>,
    pub order: ::std::option::Option<::std::string::String>,
    pub value: ::std::string::String,
}
impl ::std::convert::From<&SyncPipelineStage> for SyncPipelineStage {
    fn from(value: &SyncPipelineStage) -> Self {
        value.clone()
    }
}
impl SyncPipelineStage {
    pub fn builder() -> builder::SyncPipelineStage {
        Default::default()
    }
}
#[doc = "`SyncStage`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncStage\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"equivalent\","]
#[doc = "    \"flag\","]
#[doc = "    \"support\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"equivalent\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncEquivalent\""]
#[doc = "    },"]
#[doc = "    \"flag\": {"]
#[doc = "      \"$ref\": \"#/$defs/Flag\""]
#[doc = "    },"]
#[doc = "    \"support\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncSupport\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncStage {
    pub equivalent: SyncEquivalent,
    pub flag: Flag,
    pub support: SyncSupport,
}
impl ::std::convert::From<&SyncStage> for SyncStage {
    fn from(value: &SyncStage) -> Self {
        value.clone()
    }
}
impl SyncStage {
    pub fn builder() -> builder::SyncStage {
        Default::default()
    }
}
#[doc = "`SyncSupport`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncSupport\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"max\","]
#[doc = "    \"queues\","]
#[doc = "    \"stages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"max\": {"]
#[doc = "      \"title\": \"Max\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"queues\": {"]
#[doc = "      \"title\": \"Queues\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"stages\": {"]
#[doc = "      \"title\": \"Stages\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Flag\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncSupport {
    pub max: bool,
    pub queues: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub stages: ::std::option::Option<::std::vec::Vec<Flag>>,
}
impl ::std::convert::From<&SyncSupport> for SyncSupport {
    fn from(value: &SyncSupport) -> Self {
        value.clone()
    }
}
impl SyncSupport {
    pub fn builder() -> builder::SyncSupport {
        Default::default()
    }
}
#[doc = "`Value`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Value\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Value {
    Integer(i64),
    Number(f64),
}
impl ::std::convert::From<&Self> for Value {
    fn from(value: &Value) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Value {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for Value {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Value {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Value {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`Version`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Version\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"featureRequirement\","]
#[doc = "    \"name\","]
#[doc = "    \"nameApi\","]
#[doc = "    \"nameString\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"featureRequirement\": {"]
#[doc = "      \"title\": \"Featurerequirement\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/FeatureRequirement\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameApi\": {"]
#[doc = "      \"title\": \"Nameapi\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameString\": {"]
#[doc = "      \"title\": \"Namestring\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Version {
    #[serde(rename = "featureRequirement")]
    pub feature_requirement: ::std::vec::Vec<FeatureRequirement>,
    pub name: ::std::string::String,
    #[serde(rename = "nameApi")]
    pub name_api: ::std::string::String,
    #[serde(rename = "nameString")]
    pub name_string: ::std::string::String,
}
impl ::std::convert::From<&Version> for Version {
    fn from(value: &Version) -> Self {
        value.clone()
    }
}
impl Version {
    pub fn builder() -> builder::Version {
        Default::default()
    }
}
#[doc = "`VideoCodec`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoCodec\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capabilities\","]
#[doc = "    \"formats\","]
#[doc = "    \"name\","]
#[doc = "    \"profiles\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capabilities\": {"]
#[doc = "      \"title\": \"Capabilities\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"formats\": {"]
#[doc = "      \"title\": \"Formats\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoFormat\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"profiles\": {"]
#[doc = "      \"title\": \"Profiles\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoProfiles\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoCodec {
    pub capabilities: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub formats: ::std::collections::HashMap<::std::string::String, VideoFormat>,
    pub name: ::std::string::String,
    pub profiles: ::std::collections::HashMap<::std::string::String, VideoProfiles>,
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&VideoCodec> for VideoCodec {
    fn from(value: &VideoCodec) -> Self {
        value.clone()
    }
}
impl VideoCodec {
    pub fn builder() -> builder::VideoCodec {
        Default::default()
    }
}
#[doc = "`VideoFormat`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoFormat\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"properties\","]
#[doc = "    \"requiredCaps\","]
#[doc = "    \"usage\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"properties\": {"]
#[doc = "      \"title\": \"Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"requiredCaps\": {"]
#[doc = "      \"title\": \"Requiredcaps\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoRequiredCapabilities\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"usage\": {"]
#[doc = "      \"title\": \"Usage\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoFormat {
    pub name: ::std::string::String,
    pub properties: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(rename = "requiredCaps")]
    pub required_caps: ::std::vec::Vec<VideoRequiredCapabilities>,
    pub usage: ::std::string::String,
}
impl ::std::convert::From<&VideoFormat> for VideoFormat {
    fn from(value: &VideoFormat) -> Self {
        value.clone()
    }
}
impl VideoFormat {
    pub fn builder() -> builder::VideoFormat {
        Default::default()
    }
}
#[doc = "`VideoProfileMember`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoProfileMember\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"title\": \"Values\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoProfileMember {
    pub name: ::std::string::String,
    pub values: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
impl ::std::convert::From<&VideoProfileMember> for VideoProfileMember {
    fn from(value: &VideoProfileMember) -> Self {
        value.clone()
    }
}
impl VideoProfileMember {
    pub fn builder() -> builder::VideoProfileMember {
        Default::default()
    }
}
#[doc = "`VideoProfiles`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoProfiles\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"members\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"members\": {"]
#[doc = "      \"title\": \"Members\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoProfileMember\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoProfiles {
    pub members: ::std::collections::HashMap<::std::string::String, VideoProfileMember>,
    pub name: ::std::string::String,
}
impl ::std::convert::From<&VideoProfiles> for VideoProfiles {
    fn from(value: &VideoProfiles) -> Self {
        value.clone()
    }
}
impl VideoProfiles {
    pub fn builder() -> builder::VideoProfiles {
        Default::default()
    }
}
#[doc = "`VideoRequiredCapabilities`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoRequiredCapabilities\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"member\","]
#[doc = "    \"struct\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"member\": {"]
#[doc = "      \"title\": \"Member\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"struct\": {"]
#[doc = "      \"title\": \"Struct\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoRequiredCapabilities {
    pub member: ::std::string::String,
    #[serde(rename = "struct")]
    pub struct_: ::std::string::String,
    pub value: ::std::string::String,
}
impl ::std::convert::From<&VideoRequiredCapabilities> for VideoRequiredCapabilities {
    fn from(value: &VideoRequiredCapabilities) -> Self {
        value.clone()
    }
}
impl VideoRequiredCapabilities {
    pub fn builder() -> builder::VideoRequiredCapabilities {
        Default::default()
    }
}
#[doc = "`VideoStd`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoStd\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"constants\": {"]
#[doc = "      \"title\": \"Constants\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Constant\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"enums\": {"]
#[doc = "      \"title\": \"Enums\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Enum\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"headers\": {"]
#[doc = "      \"title\": \"Headers\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoStdHeader\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"structs\": {"]
#[doc = "      \"title\": \"Structs\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Struct\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoStd {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub constants: ::std::collections::HashMap<::std::string::String, Constant>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub enums: ::std::collections::HashMap<::std::string::String, Enum>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub headers: ::std::collections::HashMap<::std::string::String, VideoStdHeader>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub structs: ::std::collections::HashMap<::std::string::String, Struct>,
}
impl ::std::convert::From<&VideoStd> for VideoStd {
    fn from(value: &VideoStd) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for VideoStd {
    fn default() -> Self {
        Self {
            constants: Default::default(),
            enums: Default::default(),
            headers: Default::default(),
            structs: Default::default(),
        }
    }
}
impl VideoStd {
    pub fn builder() -> builder::VideoStd {
        Default::default()
    }
}
#[doc = "`VideoStdHeader`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoStdHeader\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"depends\","]
#[doc = "    \"headerFile\","]
#[doc = "    \"name\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"depends\": {"]
#[doc = "      \"title\": \"Depends\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"headerFile\": {"]
#[doc = "      \"title\": \"Headerfile\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"title\": \"Version\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoStdHeader {
    pub depends: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "headerFile")]
    pub header_file: ::std::string::String,
    pub name: ::std::string::String,
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&VideoStdHeader> for VideoStdHeader {
    fn from(value: &VideoStdHeader) -> Self {
        value.clone()
    }
}
impl VideoStdHeader {
    pub fn builder() -> builder::VideoStdHeader {
        Default::default()
    }
}
#[doc = "`VulkanObject`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VulkanObject\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"bitmasks\": {"]
#[doc = "      \"title\": \"Bitmasks\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Bitmask\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"commands\": {"]
#[doc = "      \"title\": \"Commands\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Command\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"constants\": {"]
#[doc = "      \"title\": \"Constants\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Constant\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"enums\": {"]
#[doc = "      \"title\": \"Enums\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Enum\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Extension\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flags\": {"]
#[doc = "      \"title\": \"Flags\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Flags\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"formats\": {"]
#[doc = "      \"title\": \"Formats\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Format\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"handles\": {"]
#[doc = "      \"title\": \"Handles\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Handle\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"headerVersion\": {"]
#[doc = "      \"title\": \"Headerversion\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"headerVersionComplete\": {"]
#[doc = "      \"title\": \"Headerversioncomplete\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"platforms\": {"]
#[doc = "      \"title\": \"Platforms\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"spirv\": {"]
#[doc = "      \"title\": \"Spirv\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Spirv\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"structs\": {"]
#[doc = "      \"title\": \"Structs\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Struct\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"syncAccess\": {"]
#[doc = "      \"title\": \"Syncaccess\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SyncAccess\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"syncPipeline\": {"]
#[doc = "      \"title\": \"Syncpipeline\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SyncPipeline\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"syncStage\": {"]
#[doc = "      \"title\": \"Syncstage\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SyncStage\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"vendorTags\": {"]
#[doc = "      \"title\": \"Vendortags\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"versions\": {"]
#[doc = "      \"title\": \"Versions\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Version\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"videoCodecs\": {"]
#[doc = "      \"title\": \"Videocodecs\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoCodec\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"videoStd\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/VideoStd\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VulkanObject {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub bitmasks: ::std::collections::HashMap<::std::string::String, Bitmask>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub commands: ::std::collections::HashMap<::std::string::String, Command>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub constants: ::std::collections::HashMap<::std::string::String, Constant>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub enums: ::std::collections::HashMap<::std::string::String, Enum>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extensions: ::std::collections::HashMap<::std::string::String, Extension>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub flags: ::std::collections::HashMap<::std::string::String, Flags>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub formats: ::std::collections::HashMap<::std::string::String, Format>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub handles: ::std::collections::HashMap<::std::string::String, Handle>,
    #[serde(rename = "headerVersion", default)]
    pub header_version: ::std::string::String,
    #[serde(rename = "headerVersionComplete", default)]
    pub header_version_complete: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub platforms: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub spirv: ::std::vec::Vec<Spirv>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub structs: ::std::collections::HashMap<::std::string::String, Struct>,
    #[serde(
        rename = "syncAccess",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub sync_access: ::std::vec::Vec<SyncAccess>,
    #[serde(
        rename = "syncPipeline",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub sync_pipeline: ::std::vec::Vec<SyncPipeline>,
    #[serde(
        rename = "syncStage",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub sync_stage: ::std::vec::Vec<SyncStage>,
    #[serde(
        rename = "vendorTags",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub vendor_tags: ::std::vec::Vec<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub versions: ::std::collections::HashMap<::std::string::String, Version>,
    #[serde(
        rename = "videoCodecs",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub video_codecs: ::std::collections::HashMap<::std::string::String, VideoCodec>,
    #[serde(
        rename = "videoStd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_std: ::std::option::Option<VideoStd>,
}
impl ::std::convert::From<&VulkanObject> for VulkanObject {
    fn from(value: &VulkanObject) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for VulkanObject {
    fn default() -> Self {
        Self {
            bitmasks: Default::default(),
            commands: Default::default(),
            constants: Default::default(),
            enums: Default::default(),
            extensions: Default::default(),
            flags: Default::default(),
            formats: Default::default(),
            handles: Default::default(),
            header_version: Default::default(),
            header_version_complete: Default::default(),
            platforms: Default::default(),
            spirv: Default::default(),
            structs: Default::default(),
            sync_access: Default::default(),
            sync_pipeline: Default::default(),
            sync_stage: Default::default(),
            vendor_tags: Default::default(),
            versions: Default::default(),
            video_codecs: Default::default(),
            video_std: Default::default(),
        }
    }
}
impl VulkanObject {
    pub fn builder() -> builder::VulkanObject {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Bitmask {
        aliases:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        bit_width: ::std::result::Result<i64, ::std::string::String>,
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        flag_extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        flag_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        flags: ::std::result::Result<::std::vec::Vec<super::Flag>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        protect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        returned_only: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for Bitmask {
        fn default() -> Self {
            Self {
                aliases: Err("no value supplied for aliases".to_string()),
                bit_width: Err("no value supplied for bit_width".to_string()),
                extensions: Err("no value supplied for extensions".to_string()),
                flag_extensions: Err("no value supplied for flag_extensions".to_string()),
                flag_name: Err("no value supplied for flag_name".to_string()),
                flags: Err("no value supplied for flags".to_string()),
                name: Err("no value supplied for name".to_string()),
                protect: Err("no value supplied for protect".to_string()),
                returned_only: Err("no value supplied for returned_only".to_string()),
            }
        }
    }
    impl Bitmask {
        pub fn aliases<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn bit_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.bit_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bit_width: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn flag_extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.flag_extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flag_extensions: {}", e));
            self
        }
        pub fn flag_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.flag_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flag_name: {}", e));
            self
        }
        pub fn flags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Flag>>,
            T::Error: ::std::fmt::Display,
        {
            self.flags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flags: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn protect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.protect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect: {}", e));
            self
        }
        pub fn returned_only<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.returned_only = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for returned_only: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Bitmask> for super::Bitmask {
        type Error = super::error::ConversionError;
        fn try_from(value: Bitmask) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                aliases: value.aliases?,
                bit_width: value.bit_width?,
                extensions: value.extensions?,
                flag_extensions: value.flag_extensions?,
                flag_name: value.flag_name?,
                flags: value.flags?,
                name: value.name?,
                protect: value.protect?,
                returned_only: value.returned_only?,
            })
        }
    }
    impl ::std::convert::From<super::Bitmask> for Bitmask {
        fn from(value: super::Bitmask) -> Self {
            Self {
                aliases: Ok(value.aliases),
                bit_width: Ok(value.bit_width),
                extensions: Ok(value.extensions),
                flag_extensions: Ok(value.flag_extensions),
                flag_name: Ok(value.flag_name),
                flags: Ok(value.flags),
                name: Ok(value.name),
                protect: Ok(value.protect),
                returned_only: Ok(value.returned_only),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Command {
        alias: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        allow_no_queues: ::std::result::Result<bool, ::std::string::String>,
        c_function_pointer: ::std::result::Result<::std::string::String, ::std::string::String>,
        c_prototype: ::std::result::Result<::std::string::String, ::std::string::String>,
        device: ::std::result::Result<bool, ::std::string::String>,
        error_codes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        implicit_extern_sync_params:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        instance: ::std::result::Result<bool, ::std::string::String>,
        legacy: ::std::result::Result<::std::option::Option<super::Legacy>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<::std::vec::Vec<super::Param>, ::std::string::String>,
        primary: ::std::result::Result<bool, ::std::string::String>,
        protect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        queues:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        render_pass: ::std::result::Result<i64, ::std::string::String>,
        return_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        secondary: ::std::result::Result<bool, ::std::string::String>,
        success_codes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        tasks: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        version:
            ::std::result::Result<::std::option::Option<super::Version>, ::std::string::String>,
        video_coding: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for Command {
        fn default() -> Self {
            Self {
                alias: Err("no value supplied for alias".to_string()),
                allow_no_queues: Err("no value supplied for allow_no_queues".to_string()),
                c_function_pointer: Err("no value supplied for c_function_pointer".to_string()),
                c_prototype: Err("no value supplied for c_prototype".to_string()),
                device: Err("no value supplied for device".to_string()),
                error_codes: Err("no value supplied for error_codes".to_string()),
                extensions: Err("no value supplied for extensions".to_string()),
                implicit_extern_sync_params: Err(
                    "no value supplied for implicit_extern_sync_params".to_string(),
                ),
                instance: Err("no value supplied for instance".to_string()),
                legacy: Err("no value supplied for legacy".to_string()),
                name: Err("no value supplied for name".to_string()),
                params: Err("no value supplied for params".to_string()),
                primary: Err("no value supplied for primary".to_string()),
                protect: Err("no value supplied for protect".to_string()),
                queues: Err("no value supplied for queues".to_string()),
                render_pass: Err("no value supplied for render_pass".to_string()),
                return_type: Err("no value supplied for return_type".to_string()),
                secondary: Err("no value supplied for secondary".to_string()),
                success_codes: Err("no value supplied for success_codes".to_string()),
                tasks: Err("no value supplied for tasks".to_string()),
                version: Err("no value supplied for version".to_string()),
                video_coding: Err("no value supplied for video_coding".to_string()),
            }
        }
    }
    impl Command {
        pub fn alias<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.alias = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alias: {}", e));
            self
        }
        pub fn allow_no_queues<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.allow_no_queues = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allow_no_queues: {}", e));
            self
        }
        pub fn c_function_pointer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.c_function_pointer = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for c_function_pointer: {}",
                    e
                )
            });
            self
        }
        pub fn c_prototype<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.c_prototype = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c_prototype: {}", e));
            self
        }
        pub fn device<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.device = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for device: {}", e));
            self
        }
        pub fn error_codes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_codes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_codes: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn implicit_extern_sync_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.implicit_extern_sync_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for implicit_extern_sync_params: {}",
                    e
                )
            });
            self
        }
        pub fn instance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.instance = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instance: {}", e));
            self
        }
        pub fn legacy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Legacy>>,
            T::Error: ::std::fmt::Display,
        {
            self.legacy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for legacy: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Param>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
        pub fn primary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.primary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for primary: {}", e));
            self
        }
        pub fn protect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.protect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect: {}", e));
            self
        }
        pub fn queues<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.queues = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for queues: {}", e));
            self
        }
        pub fn render_pass<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.render_pass = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for render_pass: {}", e));
            self
        }
        pub fn return_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.return_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for return_type: {}", e));
            self
        }
        pub fn secondary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.secondary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for secondary: {}", e));
            self
        }
        pub fn success_codes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.success_codes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for success_codes: {}", e));
            self
        }
        pub fn tasks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tasks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tasks: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Version>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
        pub fn video_coding<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.video_coding = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_coding: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Command> for super::Command {
        type Error = super::error::ConversionError;
        fn try_from(value: Command) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alias: value.alias?,
                allow_no_queues: value.allow_no_queues?,
                c_function_pointer: value.c_function_pointer?,
                c_prototype: value.c_prototype?,
                device: value.device?,
                error_codes: value.error_codes?,
                extensions: value.extensions?,
                implicit_extern_sync_params: value.implicit_extern_sync_params?,
                instance: value.instance?,
                legacy: value.legacy?,
                name: value.name?,
                params: value.params?,
                primary: value.primary?,
                protect: value.protect?,
                queues: value.queues?,
                render_pass: value.render_pass?,
                return_type: value.return_type?,
                secondary: value.secondary?,
                success_codes: value.success_codes?,
                tasks: value.tasks?,
                version: value.version?,
                video_coding: value.video_coding?,
            })
        }
    }
    impl ::std::convert::From<super::Command> for Command {
        fn from(value: super::Command) -> Self {
            Self {
                alias: Ok(value.alias),
                allow_no_queues: Ok(value.allow_no_queues),
                c_function_pointer: Ok(value.c_function_pointer),
                c_prototype: Ok(value.c_prototype),
                device: Ok(value.device),
                error_codes: Ok(value.error_codes),
                extensions: Ok(value.extensions),
                implicit_extern_sync_params: Ok(value.implicit_extern_sync_params),
                instance: Ok(value.instance),
                legacy: Ok(value.legacy),
                name: Ok(value.name),
                params: Ok(value.params),
                primary: Ok(value.primary),
                protect: Ok(value.protect),
                queues: Ok(value.queues),
                render_pass: Ok(value.render_pass),
                return_type: Ok(value.return_type),
                secondary: Ok(value.secondary),
                success_codes: Ok(value.success_codes),
                tasks: Ok(value.tasks),
                version: Ok(value.version),
                video_coding: Ok(value.video_coding),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Constant {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        value: ::std::result::Result<super::Value, ::std::string::String>,
        value_str: ::std::result::Result<::std::string::String, ::std::string::String>,
        video_std_header: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Constant {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                value: Err("no value supplied for value".to_string()),
                value_str: Err("no value supplied for value_str".to_string()),
                video_std_header: Ok(Default::default()),
            }
        }
    }
    impl Constant {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
        pub fn value_str<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_str: {}", e));
            self
        }
        pub fn video_std_header<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.video_std_header = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for video_std_header: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<Constant> for super::Constant {
        type Error = super::error::ConversionError;
        fn try_from(value: Constant) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                type_: value.type_?,
                value: value.value?,
                value_str: value.value_str?,
                video_std_header: value.video_std_header?,
            })
        }
    }
    impl ::std::convert::From<super::Constant> for Constant {
        fn from(value: super::Constant) -> Self {
            Self {
                name: Ok(value.name),
                type_: Ok(value.type_),
                value: Ok(value.value),
                value_str: Ok(value.value_str),
                video_std_header: Ok(value.video_std_header),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Enum {
        aliases:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        bit_width: ::std::result::Result<i64, ::std::string::String>,
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        field_extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        fields: ::std::result::Result<::std::vec::Vec<super::EnumField>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        protect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        returned_only: ::std::result::Result<bool, ::std::string::String>,
        video_std_header: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Enum {
        fn default() -> Self {
            Self {
                aliases: Err("no value supplied for aliases".to_string()),
                bit_width: Err("no value supplied for bit_width".to_string()),
                extensions: Err("no value supplied for extensions".to_string()),
                field_extensions: Err("no value supplied for field_extensions".to_string()),
                fields: Err("no value supplied for fields".to_string()),
                name: Err("no value supplied for name".to_string()),
                protect: Err("no value supplied for protect".to_string()),
                returned_only: Err("no value supplied for returned_only".to_string()),
                video_std_header: Ok(Default::default()),
            }
        }
    }
    impl Enum {
        pub fn aliases<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn bit_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.bit_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bit_width: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn field_extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.field_extensions = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for field_extensions: {}",
                    e
                )
            });
            self
        }
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EnumField>>,
            T::Error: ::std::fmt::Display,
        {
            self.fields = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fields: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn protect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.protect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect: {}", e));
            self
        }
        pub fn returned_only<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.returned_only = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for returned_only: {}", e));
            self
        }
        pub fn video_std_header<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.video_std_header = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for video_std_header: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<Enum> for super::Enum {
        type Error = super::error::ConversionError;
        fn try_from(value: Enum) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                aliases: value.aliases?,
                bit_width: value.bit_width?,
                extensions: value.extensions?,
                field_extensions: value.field_extensions?,
                fields: value.fields?,
                name: value.name?,
                protect: value.protect?,
                returned_only: value.returned_only?,
                video_std_header: value.video_std_header?,
            })
        }
    }
    impl ::std::convert::From<super::Enum> for Enum {
        fn from(value: super::Enum) -> Self {
            Self {
                aliases: Ok(value.aliases),
                bit_width: Ok(value.bit_width),
                extensions: Ok(value.extensions),
                field_extensions: Ok(value.field_extensions),
                fields: Ok(value.fields),
                name: Ok(value.name),
                protect: Ok(value.protect),
                returned_only: Ok(value.returned_only),
                video_std_header: Ok(value.video_std_header),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EnumField {
        aliases:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        negative: ::std::result::Result<bool, ::std::string::String>,
        protect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<i64, ::std::string::String>,
        value_str: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EnumField {
        fn default() -> Self {
            Self {
                aliases: Err("no value supplied for aliases".to_string()),
                extensions: Err("no value supplied for extensions".to_string()),
                name: Err("no value supplied for name".to_string()),
                negative: Err("no value supplied for negative".to_string()),
                protect: Err("no value supplied for protect".to_string()),
                value: Err("no value supplied for value".to_string()),
                value_str: Err("no value supplied for value_str".to_string()),
            }
        }
    }
    impl EnumField {
        pub fn aliases<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn negative<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.negative = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for negative: {}", e));
            self
        }
        pub fn protect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.protect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
        pub fn value_str<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_str: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EnumField> for super::EnumField {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EnumField,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                aliases: value.aliases?,
                extensions: value.extensions?,
                name: value.name?,
                negative: value.negative?,
                protect: value.protect?,
                value: value.value?,
                value_str: value.value_str?,
            })
        }
    }
    impl ::std::convert::From<super::EnumField> for EnumField {
        fn from(value: super::EnumField) -> Self {
            Self {
                aliases: Ok(value.aliases),
                extensions: Ok(value.extensions),
                name: Ok(value.name),
                negative: Ok(value.negative),
                protect: Ok(value.protect),
                value: Ok(value.value),
                value_str: Ok(value.value_str),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Extension {
        bitmasks: ::std::result::Result<::std::vec::Vec<super::Bitmask>, ::std::string::String>,
        commands: ::std::result::Result<::std::vec::Vec<super::Command>, ::std::string::String>,
        depends: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        deprecated_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        device: ::std::result::Result<bool, ::std::string::String>,
        enum_fields: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::EnumField>>,
            ::std::string::String,
        >,
        enums: ::std::result::Result<::std::vec::Vec<super::Enum>, ::std::string::String>,
        feature_requirement: ::std::result::Result<
            ::std::vec::Vec<super::FeatureRequirement>,
            ::std::string::String,
        >,
        flag_bits: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::Flag>>,
            ::std::string::String,
        >,
        flags: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::Flags>>,
            ::std::string::String,
        >,
        handles: ::std::result::Result<::std::vec::Vec<super::Handle>, ::std::string::String>,
        instance: ::std::result::Result<bool, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        name_string: ::std::result::Result<::std::string::String, ::std::string::String>,
        obsoleted_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        platform: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        promoted_to: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        protect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        provisional: ::std::result::Result<bool, ::std::string::String>,
        ratified: ::std::result::Result<bool, ::std::string::String>,
        spec_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        special_use:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        structs: ::std::result::Result<::std::vec::Vec<super::Struct>, ::std::string::String>,
        vendor_tag: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Extension {
        fn default() -> Self {
            Self {
                bitmasks: Ok(Default::default()),
                commands: Ok(Default::default()),
                depends: Err("no value supplied for depends".to_string()),
                deprecated_by: Err("no value supplied for deprecated_by".to_string()),
                device: Err("no value supplied for device".to_string()),
                enum_fields: Ok(Default::default()),
                enums: Ok(Default::default()),
                feature_requirement: Err("no value supplied for feature_requirement".to_string()),
                flag_bits: Ok(Default::default()),
                flags: Ok(Default::default()),
                handles: Ok(Default::default()),
                instance: Err("no value supplied for instance".to_string()),
                name: Err("no value supplied for name".to_string()),
                name_string: Err("no value supplied for name_string".to_string()),
                obsoleted_by: Err("no value supplied for obsoleted_by".to_string()),
                platform: Err("no value supplied for platform".to_string()),
                promoted_to: Err("no value supplied for promoted_to".to_string()),
                protect: Err("no value supplied for protect".to_string()),
                provisional: Err("no value supplied for provisional".to_string()),
                ratified: Err("no value supplied for ratified".to_string()),
                spec_version: Err("no value supplied for spec_version".to_string()),
                special_use: Err("no value supplied for special_use".to_string()),
                structs: Ok(Default::default()),
                vendor_tag: Err("no value supplied for vendor_tag".to_string()),
            }
        }
    }
    impl Extension {
        pub fn bitmasks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Bitmask>>,
            T::Error: ::std::fmt::Display,
        {
            self.bitmasks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bitmasks: {}", e));
            self
        }
        pub fn commands<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Command>>,
            T::Error: ::std::fmt::Display,
        {
            self.commands = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for commands: {}", e));
            self
        }
        pub fn depends<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.depends = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for depends: {}", e));
            self
        }
        pub fn deprecated_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.deprecated_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deprecated_by: {}", e));
            self
        }
        pub fn device<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.device = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for device: {}", e));
            self
        }
        pub fn enum_fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::vec::Vec<super::EnumField>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.enum_fields = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_fields: {}", e));
            self
        }
        pub fn enums<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Enum>>,
            T::Error: ::std::fmt::Display,
        {
            self.enums = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enums: {}", e));
            self
        }
        pub fn feature_requirement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FeatureRequirement>>,
            T::Error: ::std::fmt::Display,
        {
            self.feature_requirement = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for feature_requirement: {}",
                    e
                )
            });
            self
        }
        pub fn flag_bits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::Flag>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.flag_bits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flag_bits: {}", e));
            self
        }
        pub fn flags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::Flags>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.flags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flags: {}", e));
            self
        }
        pub fn handles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Handle>>,
            T::Error: ::std::fmt::Display,
        {
            self.handles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for handles: {}", e));
            self
        }
        pub fn instance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.instance = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instance: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn name_string<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name_string = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name_string: {}", e));
            self
        }
        pub fn obsoleted_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.obsoleted_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for obsoleted_by: {}", e));
            self
        }
        pub fn platform<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.platform = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for platform: {}", e));
            self
        }
        pub fn promoted_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.promoted_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for promoted_to: {}", e));
            self
        }
        pub fn protect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.protect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect: {}", e));
            self
        }
        pub fn provisional<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.provisional = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provisional: {}", e));
            self
        }
        pub fn ratified<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.ratified = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ratified: {}", e));
            self
        }
        pub fn spec_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.spec_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spec_version: {}", e));
            self
        }
        pub fn special_use<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.special_use = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for special_use: {}", e));
            self
        }
        pub fn structs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Struct>>,
            T::Error: ::std::fmt::Display,
        {
            self.structs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for structs: {}", e));
            self
        }
        pub fn vendor_tag<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.vendor_tag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vendor_tag: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Extension> for super::Extension {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Extension,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bitmasks: value.bitmasks?,
                commands: value.commands?,
                depends: value.depends?,
                deprecated_by: value.deprecated_by?,
                device: value.device?,
                enum_fields: value.enum_fields?,
                enums: value.enums?,
                feature_requirement: value.feature_requirement?,
                flag_bits: value.flag_bits?,
                flags: value.flags?,
                handles: value.handles?,
                instance: value.instance?,
                name: value.name?,
                name_string: value.name_string?,
                obsoleted_by: value.obsoleted_by?,
                platform: value.platform?,
                promoted_to: value.promoted_to?,
                protect: value.protect?,
                provisional: value.provisional?,
                ratified: value.ratified?,
                spec_version: value.spec_version?,
                special_use: value.special_use?,
                structs: value.structs?,
                vendor_tag: value.vendor_tag?,
            })
        }
    }
    impl ::std::convert::From<super::Extension> for Extension {
        fn from(value: super::Extension) -> Self {
            Self {
                bitmasks: Ok(value.bitmasks),
                commands: Ok(value.commands),
                depends: Ok(value.depends),
                deprecated_by: Ok(value.deprecated_by),
                device: Ok(value.device),
                enum_fields: Ok(value.enum_fields),
                enums: Ok(value.enums),
                feature_requirement: Ok(value.feature_requirement),
                flag_bits: Ok(value.flag_bits),
                flags: Ok(value.flags),
                handles: Ok(value.handles),
                instance: Ok(value.instance),
                name: Ok(value.name),
                name_string: Ok(value.name_string),
                obsoleted_by: Ok(value.obsoleted_by),
                platform: Ok(value.platform),
                promoted_to: Ok(value.promoted_to),
                protect: Ok(value.protect),
                provisional: Ok(value.provisional),
                ratified: Ok(value.ratified),
                spec_version: Ok(value.spec_version),
                special_use: Ok(value.special_use),
                structs: Ok(value.structs),
                vendor_tag: Ok(value.vendor_tag),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FeatureRequirement {
        depends: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        field: ::std::result::Result<::std::string::String, ::std::string::String>,
        struct_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for FeatureRequirement {
        fn default() -> Self {
            Self {
                depends: Err("no value supplied for depends".to_string()),
                field: Err("no value supplied for field".to_string()),
                struct_: Err("no value supplied for struct_".to_string()),
            }
        }
    }
    impl FeatureRequirement {
        pub fn depends<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.depends = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for depends: {}", e));
            self
        }
        pub fn field<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.field = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for field: {}", e));
            self
        }
        pub fn struct_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.struct_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for struct_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FeatureRequirement> for super::FeatureRequirement {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FeatureRequirement,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                depends: value.depends?,
                field: value.field?,
                struct_: value.struct_?,
            })
        }
    }
    impl ::std::convert::From<super::FeatureRequirement> for FeatureRequirement {
        fn from(value: super::FeatureRequirement) -> Self {
            Self {
                depends: Ok(value.depends),
                field: Ok(value.field),
                struct_: Ok(value.struct_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Flag {
        aliases:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        multi_bit: ::std::result::Result<bool, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        protect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<i64, ::std::string::String>,
        value_str: ::std::result::Result<::std::string::String, ::std::string::String>,
        zero: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for Flag {
        fn default() -> Self {
            Self {
                aliases: Err("no value supplied for aliases".to_string()),
                extensions: Err("no value supplied for extensions".to_string()),
                multi_bit: Err("no value supplied for multi_bit".to_string()),
                name: Err("no value supplied for name".to_string()),
                protect: Err("no value supplied for protect".to_string()),
                value: Err("no value supplied for value".to_string()),
                value_str: Err("no value supplied for value_str".to_string()),
                zero: Err("no value supplied for zero".to_string()),
            }
        }
    }
    impl Flag {
        pub fn aliases<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn multi_bit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.multi_bit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for multi_bit: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn protect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.protect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
        pub fn value_str<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value_str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value_str: {}", e));
            self
        }
        pub fn zero<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.zero = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for zero: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Flag> for super::Flag {
        type Error = super::error::ConversionError;
        fn try_from(value: Flag) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                aliases: value.aliases?,
                extensions: value.extensions?,
                multi_bit: value.multi_bit?,
                name: value.name?,
                protect: value.protect?,
                value: value.value?,
                value_str: value.value_str?,
                zero: value.zero?,
            })
        }
    }
    impl ::std::convert::From<super::Flag> for Flag {
        fn from(value: super::Flag) -> Self {
            Self {
                aliases: Ok(value.aliases),
                extensions: Ok(value.extensions),
                multi_bit: Ok(value.multi_bit),
                name: Ok(value.name),
                protect: Ok(value.protect),
                value: Ok(value.value),
                value_str: Ok(value.value_str),
                zero: Ok(value.zero),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Flags {
        aliases:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        base_flags_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        bit_width: ::std::result::Result<i64, ::std::string::String>,
        bitmask_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        protect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        returned_only: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for Flags {
        fn default() -> Self {
            Self {
                aliases: Err("no value supplied for aliases".to_string()),
                base_flags_type: Err("no value supplied for base_flags_type".to_string()),
                bit_width: Err("no value supplied for bit_width".to_string()),
                bitmask_name: Err("no value supplied for bitmask_name".to_string()),
                extensions: Err("no value supplied for extensions".to_string()),
                name: Err("no value supplied for name".to_string()),
                protect: Err("no value supplied for protect".to_string()),
                returned_only: Err("no value supplied for returned_only".to_string()),
            }
        }
    }
    impl Flags {
        pub fn aliases<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn base_flags_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.base_flags_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base_flags_type: {}", e));
            self
        }
        pub fn bit_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.bit_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bit_width: {}", e));
            self
        }
        pub fn bitmask_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.bitmask_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bitmask_name: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn protect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.protect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect: {}", e));
            self
        }
        pub fn returned_only<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.returned_only = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for returned_only: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Flags> for super::Flags {
        type Error = super::error::ConversionError;
        fn try_from(value: Flags) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                aliases: value.aliases?,
                base_flags_type: value.base_flags_type?,
                bit_width: value.bit_width?,
                bitmask_name: value.bitmask_name?,
                extensions: value.extensions?,
                name: value.name?,
                protect: value.protect?,
                returned_only: value.returned_only?,
            })
        }
    }
    impl ::std::convert::From<super::Flags> for Flags {
        fn from(value: super::Flags) -> Self {
            Self {
                aliases: Ok(value.aliases),
                base_flags_type: Ok(value.base_flags_type),
                bit_width: Ok(value.bit_width),
                bitmask_name: Ok(value.bitmask_name),
                extensions: Ok(value.extensions),
                name: Ok(value.name),
                protect: Ok(value.protect),
                returned_only: Ok(value.returned_only),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Format {
        block_extent:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        block_size: ::std::result::Result<i64, ::std::string::String>,
        chroma: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        class_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        components:
            ::std::result::Result<::std::vec::Vec<super::FormatComponent>, ::std::string::String>,
        compressed: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        packed: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        planes: ::std::result::Result<::std::vec::Vec<super::FormatPlane>, ::std::string::String>,
        spirv_image_format: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        texels_per_block: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for Format {
        fn default() -> Self {
            Self {
                block_extent: Err("no value supplied for block_extent".to_string()),
                block_size: Err("no value supplied for block_size".to_string()),
                chroma: Err("no value supplied for chroma".to_string()),
                class_name: Err("no value supplied for class_name".to_string()),
                components: Err("no value supplied for components".to_string()),
                compressed: Err("no value supplied for compressed".to_string()),
                name: Err("no value supplied for name".to_string()),
                packed: Err("no value supplied for packed".to_string()),
                planes: Err("no value supplied for planes".to_string()),
                spirv_image_format: Err("no value supplied for spirv_image_format".to_string()),
                texels_per_block: Err("no value supplied for texels_per_block".to_string()),
            }
        }
    }
    impl Format {
        pub fn block_extent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.block_extent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for block_extent: {}", e));
            self
        }
        pub fn block_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.block_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for block_size: {}", e));
            self
        }
        pub fn chroma<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.chroma = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chroma: {}", e));
            self
        }
        pub fn class_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.class_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class_name: {}", e));
            self
        }
        pub fn components<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FormatComponent>>,
            T::Error: ::std::fmt::Display,
        {
            self.components = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for components: {}", e));
            self
        }
        pub fn compressed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.compressed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for compressed: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn packed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.packed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for packed: {}", e));
            self
        }
        pub fn planes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FormatPlane>>,
            T::Error: ::std::fmt::Display,
        {
            self.planes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for planes: {}", e));
            self
        }
        pub fn spirv_image_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.spirv_image_format = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for spirv_image_format: {}",
                    e
                )
            });
            self
        }
        pub fn texels_per_block<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.texels_per_block = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for texels_per_block: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<Format> for super::Format {
        type Error = super::error::ConversionError;
        fn try_from(value: Format) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                block_extent: value.block_extent?,
                block_size: value.block_size?,
                chroma: value.chroma?,
                class_name: value.class_name?,
                components: value.components?,
                compressed: value.compressed?,
                name: value.name?,
                packed: value.packed?,
                planes: value.planes?,
                spirv_image_format: value.spirv_image_format?,
                texels_per_block: value.texels_per_block?,
            })
        }
    }
    impl ::std::convert::From<super::Format> for Format {
        fn from(value: super::Format) -> Self {
            Self {
                block_extent: Ok(value.block_extent),
                block_size: Ok(value.block_size),
                chroma: Ok(value.chroma),
                class_name: Ok(value.class_name),
                components: Ok(value.components),
                compressed: Ok(value.compressed),
                name: Ok(value.name),
                packed: Ok(value.packed),
                planes: Ok(value.planes),
                spirv_image_format: Ok(value.spirv_image_format),
                texels_per_block: Ok(value.texels_per_block),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FormatComponent {
        bits: ::std::result::Result<::std::string::String, ::std::string::String>,
        numeric_format: ::std::result::Result<::std::string::String, ::std::string::String>,
        plane_index: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for FormatComponent {
        fn default() -> Self {
            Self {
                bits: Err("no value supplied for bits".to_string()),
                numeric_format: Err("no value supplied for numeric_format".to_string()),
                plane_index: Err("no value supplied for plane_index".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FormatComponent {
        pub fn bits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.bits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bits: {}", e));
            self
        }
        pub fn numeric_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.numeric_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for numeric_format: {}", e));
            self
        }
        pub fn plane_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.plane_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plane_index: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FormatComponent> for super::FormatComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FormatComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bits: value.bits?,
                numeric_format: value.numeric_format?,
                plane_index: value.plane_index?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FormatComponent> for FormatComponent {
        fn from(value: super::FormatComponent) -> Self {
            Self {
                bits: Ok(value.bits),
                numeric_format: Ok(value.numeric_format),
                plane_index: Ok(value.plane_index),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FormatPlane {
        compatible: ::std::result::Result<::std::string::String, ::std::string::String>,
        height_divisor: ::std::result::Result<i64, ::std::string::String>,
        index: ::std::result::Result<i64, ::std::string::String>,
        width_divisor: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for FormatPlane {
        fn default() -> Self {
            Self {
                compatible: Err("no value supplied for compatible".to_string()),
                height_divisor: Err("no value supplied for height_divisor".to_string()),
                index: Err("no value supplied for index".to_string()),
                width_divisor: Err("no value supplied for width_divisor".to_string()),
            }
        }
    }
    impl FormatPlane {
        pub fn compatible<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.compatible = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for compatible: {}", e));
            self
        }
        pub fn height_divisor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.height_divisor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height_divisor: {}", e));
            self
        }
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {}", e));
            self
        }
        pub fn width_divisor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.width_divisor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width_divisor: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FormatPlane> for super::FormatPlane {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FormatPlane,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                compatible: value.compatible?,
                height_divisor: value.height_divisor?,
                index: value.index?,
                width_divisor: value.width_divisor?,
            })
        }
    }
    impl ::std::convert::From<super::FormatPlane> for FormatPlane {
        fn from(value: super::FormatPlane) -> Self {
            Self {
                compatible: Ok(value.compatible),
                height_divisor: Ok(value.height_divisor),
                index: Ok(value.index),
                width_divisor: Ok(value.width_divisor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Handle {
        aliases:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        device: ::std::result::Result<bool, ::std::string::String>,
        dispatchable: ::std::result::Result<bool, ::std::string::String>,
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        instance: ::std::result::Result<bool, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        parent: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::Handle>>,
            ::std::string::String,
        >,
        protect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Handle {
        fn default() -> Self {
            Self {
                aliases: Err("no value supplied for aliases".to_string()),
                device: Err("no value supplied for device".to_string()),
                dispatchable: Err("no value supplied for dispatchable".to_string()),
                extensions: Err("no value supplied for extensions".to_string()),
                instance: Err("no value supplied for instance".to_string()),
                name: Err("no value supplied for name".to_string()),
                parent: Err("no value supplied for parent".to_string()),
                protect: Err("no value supplied for protect".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Handle {
        pub fn aliases<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn device<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.device = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for device: {}", e));
            self
        }
        pub fn dispatchable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.dispatchable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dispatchable: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn instance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.instance = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instance: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn parent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::boxed::Box<super::Handle>>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent: {}", e));
            self
        }
        pub fn protect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.protect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Handle> for super::Handle {
        type Error = super::error::ConversionError;
        fn try_from(value: Handle) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                aliases: value.aliases?,
                device: value.device?,
                dispatchable: value.dispatchable?,
                extensions: value.extensions?,
                instance: value.instance?,
                name: value.name?,
                parent: value.parent?,
                protect: value.protect?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Handle> for Handle {
        fn from(value: super::Handle) -> Self {
            Self {
                aliases: Ok(value.aliases),
                device: Ok(value.device),
                dispatchable: Ok(value.dispatchable),
                extensions: Ok(value.extensions),
                instance: Ok(value.instance),
                name: Ok(value.name),
                parent: Ok(value.parent),
                protect: Ok(value.protect),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Legacy {
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        link: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version:
            ::std::result::Result<::std::option::Option<super::Version>, ::std::string::String>,
    }
    impl ::std::default::Default for Legacy {
        fn default() -> Self {
            Self {
                extensions: Err("no value supplied for extensions".to_string()),
                link: Err("no value supplied for link".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl Legacy {
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn link<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Version>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Legacy> for super::Legacy {
        type Error = super::error::ConversionError;
        fn try_from(value: Legacy) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                extensions: value.extensions?,
                link: value.link?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::Legacy> for Legacy {
        fn from(value: super::Legacy) -> Self {
            Self {
                extensions: Ok(value.extensions),
                link: Ok(value.link),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Member {
        bit_field_width: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        c_declaration: ::std::result::Result<::std::string::String, ::std::string::String>,
        const_: ::std::result::Result<bool, ::std::string::String>,
        extern_sync: ::std::result::Result<i64, ::std::string::String>,
        fixed_size_array:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        full_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        length: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        no_auto_validity: ::std::result::Result<bool, ::std::string::String>,
        null_terminated: ::std::result::Result<bool, ::std::string::String>,
        optional: ::std::result::Result<bool, ::std::string::String>,
        optional_pointer: ::std::result::Result<bool, ::std::string::String>,
        pointer: ::std::result::Result<bool, ::std::string::String>,
        selection:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        selector: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Member {
        fn default() -> Self {
            Self {
                bit_field_width: Err("no value supplied for bit_field_width".to_string()),
                c_declaration: Err("no value supplied for c_declaration".to_string()),
                const_: Err("no value supplied for const_".to_string()),
                extern_sync: Err("no value supplied for extern_sync".to_string()),
                fixed_size_array: Err("no value supplied for fixed_size_array".to_string()),
                full_type: Err("no value supplied for full_type".to_string()),
                length: Err("no value supplied for length".to_string()),
                limit_type: Err("no value supplied for limit_type".to_string()),
                name: Err("no value supplied for name".to_string()),
                no_auto_validity: Err("no value supplied for no_auto_validity".to_string()),
                null_terminated: Err("no value supplied for null_terminated".to_string()),
                optional: Err("no value supplied for optional".to_string()),
                optional_pointer: Err("no value supplied for optional_pointer".to_string()),
                pointer: Err("no value supplied for pointer".to_string()),
                selection: Err("no value supplied for selection".to_string()),
                selector: Err("no value supplied for selector".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Member {
        pub fn bit_field_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.bit_field_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bit_field_width: {}", e));
            self
        }
        pub fn c_declaration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.c_declaration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c_declaration: {}", e));
            self
        }
        pub fn const_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.const_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for const_: {}", e));
            self
        }
        pub fn extern_sync<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.extern_sync = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extern_sync: {}", e));
            self
        }
        pub fn fixed_size_array<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.fixed_size_array = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fixed_size_array: {}",
                    e
                )
            });
            self
        }
        pub fn full_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.full_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for full_type: {}", e));
            self
        }
        pub fn length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for length: {}", e));
            self
        }
        pub fn limit_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit_type: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn no_auto_validity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.no_auto_validity = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for no_auto_validity: {}",
                    e
                )
            });
            self
        }
        pub fn null_terminated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.null_terminated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for null_terminated: {}", e));
            self
        }
        pub fn optional<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.optional = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for optional: {}", e));
            self
        }
        pub fn optional_pointer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.optional_pointer = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for optional_pointer: {}",
                    e
                )
            });
            self
        }
        pub fn pointer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.pointer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pointer: {}", e));
            self
        }
        pub fn selection<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.selection = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for selection: {}", e));
            self
        }
        pub fn selector<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.selector = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for selector: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Member> for super::Member {
        type Error = super::error::ConversionError;
        fn try_from(value: Member) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bit_field_width: value.bit_field_width?,
                c_declaration: value.c_declaration?,
                const_: value.const_?,
                extern_sync: value.extern_sync?,
                fixed_size_array: value.fixed_size_array?,
                full_type: value.full_type?,
                length: value.length?,
                limit_type: value.limit_type?,
                name: value.name?,
                no_auto_validity: value.no_auto_validity?,
                null_terminated: value.null_terminated?,
                optional: value.optional?,
                optional_pointer: value.optional_pointer?,
                pointer: value.pointer?,
                selection: value.selection?,
                selector: value.selector?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Member> for Member {
        fn from(value: super::Member) -> Self {
            Self {
                bit_field_width: Ok(value.bit_field_width),
                c_declaration: Ok(value.c_declaration),
                const_: Ok(value.const_),
                extern_sync: Ok(value.extern_sync),
                fixed_size_array: Ok(value.fixed_size_array),
                full_type: Ok(value.full_type),
                length: Ok(value.length),
                limit_type: Ok(value.limit_type),
                name: Ok(value.name),
                no_auto_validity: Ok(value.no_auto_validity),
                null_terminated: Ok(value.null_terminated),
                optional: Ok(value.optional),
                optional_pointer: Ok(value.optional_pointer),
                pointer: Ok(value.pointer),
                selection: Ok(value.selection),
                selector: Ok(value.selector),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Param {
        alias: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        c_declaration: ::std::result::Result<::std::string::String, ::std::string::String>,
        const_: ::std::result::Result<bool, ::std::string::String>,
        extern_sync: ::std::result::Result<i64, ::std::string::String>,
        extern_sync_pointer: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        fixed_size_array:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        full_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        length: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        no_auto_validity: ::std::result::Result<bool, ::std::string::String>,
        null_terminated: ::std::result::Result<bool, ::std::string::String>,
        optional: ::std::result::Result<bool, ::std::string::String>,
        optional_pointer: ::std::result::Result<bool, ::std::string::String>,
        pointer: ::std::result::Result<bool, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Param {
        fn default() -> Self {
            Self {
                alias: Err("no value supplied for alias".to_string()),
                c_declaration: Err("no value supplied for c_declaration".to_string()),
                const_: Err("no value supplied for const_".to_string()),
                extern_sync: Err("no value supplied for extern_sync".to_string()),
                extern_sync_pointer: Err("no value supplied for extern_sync_pointer".to_string()),
                fixed_size_array: Err("no value supplied for fixed_size_array".to_string()),
                full_type: Err("no value supplied for full_type".to_string()),
                length: Err("no value supplied for length".to_string()),
                name: Err("no value supplied for name".to_string()),
                no_auto_validity: Err("no value supplied for no_auto_validity".to_string()),
                null_terminated: Err("no value supplied for null_terminated".to_string()),
                optional: Err("no value supplied for optional".to_string()),
                optional_pointer: Err("no value supplied for optional_pointer".to_string()),
                pointer: Err("no value supplied for pointer".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Param {
        pub fn alias<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.alias = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alias: {}", e));
            self
        }
        pub fn c_declaration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.c_declaration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c_declaration: {}", e));
            self
        }
        pub fn const_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.const_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for const_: {}", e));
            self
        }
        pub fn extern_sync<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.extern_sync = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extern_sync: {}", e));
            self
        }
        pub fn extern_sync_pointer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extern_sync_pointer = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for extern_sync_pointer: {}",
                    e
                )
            });
            self
        }
        pub fn fixed_size_array<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.fixed_size_array = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fixed_size_array: {}",
                    e
                )
            });
            self
        }
        pub fn full_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.full_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for full_type: {}", e));
            self
        }
        pub fn length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for length: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn no_auto_validity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.no_auto_validity = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for no_auto_validity: {}",
                    e
                )
            });
            self
        }
        pub fn null_terminated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.null_terminated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for null_terminated: {}", e));
            self
        }
        pub fn optional<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.optional = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for optional: {}", e));
            self
        }
        pub fn optional_pointer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.optional_pointer = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for optional_pointer: {}",
                    e
                )
            });
            self
        }
        pub fn pointer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.pointer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pointer: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Param> for super::Param {
        type Error = super::error::ConversionError;
        fn try_from(value: Param) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alias: value.alias?,
                c_declaration: value.c_declaration?,
                const_: value.const_?,
                extern_sync: value.extern_sync?,
                extern_sync_pointer: value.extern_sync_pointer?,
                fixed_size_array: value.fixed_size_array?,
                full_type: value.full_type?,
                length: value.length?,
                name: value.name?,
                no_auto_validity: value.no_auto_validity?,
                null_terminated: value.null_terminated?,
                optional: value.optional?,
                optional_pointer: value.optional_pointer?,
                pointer: value.pointer?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Param> for Param {
        fn from(value: super::Param) -> Self {
            Self {
                alias: Ok(value.alias),
                c_declaration: Ok(value.c_declaration),
                const_: Ok(value.const_),
                extern_sync: Ok(value.extern_sync),
                extern_sync_pointer: Ok(value.extern_sync_pointer),
                fixed_size_array: Ok(value.fixed_size_array),
                full_type: Ok(value.full_type),
                length: Ok(value.length),
                name: Ok(value.name),
                no_auto_validity: Ok(value.no_auto_validity),
                null_terminated: Ok(value.null_terminated),
                optional: Ok(value.optional),
                optional_pointer: Ok(value.optional_pointer),
                pointer: Ok(value.pointer),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Spirv {
        capability: ::std::result::Result<bool, ::std::string::String>,
        enable: ::std::result::Result<::std::vec::Vec<super::SpirvEnables>, ::std::string::String>,
        extension: ::std::result::Result<bool, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Spirv {
        fn default() -> Self {
            Self {
                capability: Err("no value supplied for capability".to_string()),
                enable: Err("no value supplied for enable".to_string()),
                extension: Err("no value supplied for extension".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl Spirv {
        pub fn capability<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.capability = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for capability: {}", e));
            self
        }
        pub fn enable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SpirvEnables>>,
            T::Error: ::std::fmt::Display,
        {
            self.enable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enable: {}", e));
            self
        }
        pub fn extension<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.extension = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extension: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Spirv> for super::Spirv {
        type Error = super::error::ConversionError;
        fn try_from(value: Spirv) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capability: value.capability?,
                enable: value.enable?,
                extension: value.extension?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::Spirv> for Spirv {
        fn from(value: super::Spirv) -> Self {
            Self {
                capability: Ok(value.capability),
                enable: Ok(value.enable),
                extension: Ok(value.extension),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpirvEnables {
        extension: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        feature: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        member: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        property: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        requires: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        struct_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SpirvEnables {
        fn default() -> Self {
            Self {
                extension: Err("no value supplied for extension".to_string()),
                feature: Err("no value supplied for feature".to_string()),
                member: Err("no value supplied for member".to_string()),
                property: Err("no value supplied for property".to_string()),
                requires: Err("no value supplied for requires".to_string()),
                struct_: Err("no value supplied for struct_".to_string()),
                value: Err("no value supplied for value".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl SpirvEnables {
        pub fn extension<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extension = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extension: {}", e));
            self
        }
        pub fn feature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.feature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for feature: {}", e));
            self
        }
        pub fn member<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.member = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for member: {}", e));
            self
        }
        pub fn property<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.property = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for property: {}", e));
            self
        }
        pub fn requires<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.requires = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for requires: {}", e));
            self
        }
        pub fn struct_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.struct_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for struct_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SpirvEnables> for super::SpirvEnables {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpirvEnables,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                extension: value.extension?,
                feature: value.feature?,
                member: value.member?,
                property: value.property?,
                requires: value.requires?,
                struct_: value.struct_?,
                value: value.value?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::SpirvEnables> for SpirvEnables {
        fn from(value: super::SpirvEnables) -> Self {
            Self {
                extension: Ok(value.extension),
                feature: Ok(value.feature),
                member: Ok(value.member),
                property: Ok(value.property),
                requires: Ok(value.requires),
                struct_: Ok(value.struct_),
                value: Ok(value.value),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Struct {
        aliases:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        allow_duplicate: ::std::result::Result<bool, ::std::string::String>,
        extended_by:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        extends:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        members: ::std::result::Result<::std::vec::Vec<super::Member>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        protect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        returned_only: ::std::result::Result<bool, ::std::string::String>,
        s_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        union: ::std::result::Result<bool, ::std::string::String>,
        version:
            ::std::result::Result<::std::option::Option<super::Version>, ::std::string::String>,
        video_std_header: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Struct {
        fn default() -> Self {
            Self {
                aliases: Err("no value supplied for aliases".to_string()),
                allow_duplicate: Err("no value supplied for allow_duplicate".to_string()),
                extended_by: Err("no value supplied for extended_by".to_string()),
                extends: Err("no value supplied for extends".to_string()),
                extensions: Err("no value supplied for extensions".to_string()),
                members: Err("no value supplied for members".to_string()),
                name: Err("no value supplied for name".to_string()),
                protect: Err("no value supplied for protect".to_string()),
                returned_only: Err("no value supplied for returned_only".to_string()),
                s_type: Err("no value supplied for s_type".to_string()),
                union: Err("no value supplied for union".to_string()),
                version: Err("no value supplied for version".to_string()),
                video_std_header: Ok(Default::default()),
            }
        }
    }
    impl Struct {
        pub fn aliases<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.aliases = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for aliases: {}", e));
            self
        }
        pub fn allow_duplicate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.allow_duplicate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allow_duplicate: {}", e));
            self
        }
        pub fn extended_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extended_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extended_by: {}", e));
            self
        }
        pub fn extends<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extends = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extends: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn members<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Member>>,
            T::Error: ::std::fmt::Display,
        {
            self.members = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for members: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn protect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.protect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect: {}", e));
            self
        }
        pub fn returned_only<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.returned_only = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for returned_only: {}", e));
            self
        }
        pub fn s_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.s_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for s_type: {}", e));
            self
        }
        pub fn union<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.union = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for union: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Version>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
        pub fn video_std_header<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.video_std_header = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for video_std_header: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<Struct> for super::Struct {
        type Error = super::error::ConversionError;
        fn try_from(value: Struct) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                aliases: value.aliases?,
                allow_duplicate: value.allow_duplicate?,
                extended_by: value.extended_by?,
                extends: value.extends?,
                extensions: value.extensions?,
                members: value.members?,
                name: value.name?,
                protect: value.protect?,
                returned_only: value.returned_only?,
                s_type: value.s_type?,
                union: value.union?,
                version: value.version?,
                video_std_header: value.video_std_header?,
            })
        }
    }
    impl ::std::convert::From<super::Struct> for Struct {
        fn from(value: super::Struct) -> Self {
            Self {
                aliases: Ok(value.aliases),
                allow_duplicate: Ok(value.allow_duplicate),
                extended_by: Ok(value.extended_by),
                extends: Ok(value.extends),
                extensions: Ok(value.extensions),
                members: Ok(value.members),
                name: Ok(value.name),
                protect: Ok(value.protect),
                returned_only: Ok(value.returned_only),
                s_type: Ok(value.s_type),
                union: Ok(value.union),
                version: Ok(value.version),
                video_std_header: Ok(value.video_std_header),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SyncAccess {
        equivalent: ::std::result::Result<super::SyncEquivalent, ::std::string::String>,
        flag: ::std::result::Result<super::Flag, ::std::string::String>,
        support: ::std::result::Result<super::SyncSupport, ::std::string::String>,
    }
    impl ::std::default::Default for SyncAccess {
        fn default() -> Self {
            Self {
                equivalent: Err("no value supplied for equivalent".to_string()),
                flag: Err("no value supplied for flag".to_string()),
                support: Err("no value supplied for support".to_string()),
            }
        }
    }
    impl SyncAccess {
        pub fn equivalent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SyncEquivalent>,
            T::Error: ::std::fmt::Display,
        {
            self.equivalent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for equivalent: {}", e));
            self
        }
        pub fn flag<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Flag>,
            T::Error: ::std::fmt::Display,
        {
            self.flag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flag: {}", e));
            self
        }
        pub fn support<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SyncSupport>,
            T::Error: ::std::fmt::Display,
        {
            self.support = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for support: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SyncAccess> for super::SyncAccess {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SyncAccess,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                equivalent: value.equivalent?,
                flag: value.flag?,
                support: value.support?,
            })
        }
    }
    impl ::std::convert::From<super::SyncAccess> for SyncAccess {
        fn from(value: super::SyncAccess) -> Self {
            Self {
                equivalent: Ok(value.equivalent),
                flag: Ok(value.flag),
                support: Ok(value.support),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SyncEquivalent {
        accesses: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Flag>>,
            ::std::string::String,
        >,
        max: ::std::result::Result<bool, ::std::string::String>,
        stages: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Flag>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SyncEquivalent {
        fn default() -> Self {
            Self {
                accesses: Err("no value supplied for accesses".to_string()),
                max: Err("no value supplied for max".to_string()),
                stages: Err("no value supplied for stages".to_string()),
            }
        }
    }
    impl SyncEquivalent {
        pub fn accesses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Flag>>>,
            T::Error: ::std::fmt::Display,
        {
            self.accesses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accesses: {}", e));
            self
        }
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn stages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Flag>>>,
            T::Error: ::std::fmt::Display,
        {
            self.stages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stages: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SyncEquivalent> for super::SyncEquivalent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SyncEquivalent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accesses: value.accesses?,
                max: value.max?,
                stages: value.stages?,
            })
        }
    }
    impl ::std::convert::From<super::SyncEquivalent> for SyncEquivalent {
        fn from(value: super::SyncEquivalent) -> Self {
            Self {
                accesses: Ok(value.accesses),
                max: Ok(value.max),
                stages: Ok(value.stages),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SyncPipeline {
        depends:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        stages:
            ::std::result::Result<::std::vec::Vec<super::SyncPipelineStage>, ::std::string::String>,
    }
    impl ::std::default::Default for SyncPipeline {
        fn default() -> Self {
            Self {
                depends: Err("no value supplied for depends".to_string()),
                name: Err("no value supplied for name".to_string()),
                stages: Err("no value supplied for stages".to_string()),
            }
        }
    }
    impl SyncPipeline {
        pub fn depends<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.depends = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for depends: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn stages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SyncPipelineStage>>,
            T::Error: ::std::fmt::Display,
        {
            self.stages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stages: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SyncPipeline> for super::SyncPipeline {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SyncPipeline,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                depends: value.depends?,
                name: value.name?,
                stages: value.stages?,
            })
        }
    }
    impl ::std::convert::From<super::SyncPipeline> for SyncPipeline {
        fn from(value: super::SyncPipeline) -> Self {
            Self {
                depends: Ok(value.depends),
                name: Ok(value.name),
                stages: Ok(value.stages),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SyncPipelineStage {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        before: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        order: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SyncPipelineStage {
        fn default() -> Self {
            Self {
                after: Err("no value supplied for after".to_string()),
                before: Err("no value supplied for before".to_string()),
                order: Err("no value supplied for order".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl SyncPipelineStage {
        pub fn after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for after: {}", e));
            self
        }
        pub fn before<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.before = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for before: {}", e));
            self
        }
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SyncPipelineStage> for super::SyncPipelineStage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SyncPipelineStage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                order: value.order?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::SyncPipelineStage> for SyncPipelineStage {
        fn from(value: super::SyncPipelineStage) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                order: Ok(value.order),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SyncStage {
        equivalent: ::std::result::Result<super::SyncEquivalent, ::std::string::String>,
        flag: ::std::result::Result<super::Flag, ::std::string::String>,
        support: ::std::result::Result<super::SyncSupport, ::std::string::String>,
    }
    impl ::std::default::Default for SyncStage {
        fn default() -> Self {
            Self {
                equivalent: Err("no value supplied for equivalent".to_string()),
                flag: Err("no value supplied for flag".to_string()),
                support: Err("no value supplied for support".to_string()),
            }
        }
    }
    impl SyncStage {
        pub fn equivalent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SyncEquivalent>,
            T::Error: ::std::fmt::Display,
        {
            self.equivalent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for equivalent: {}", e));
            self
        }
        pub fn flag<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Flag>,
            T::Error: ::std::fmt::Display,
        {
            self.flag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flag: {}", e));
            self
        }
        pub fn support<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SyncSupport>,
            T::Error: ::std::fmt::Display,
        {
            self.support = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for support: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SyncStage> for super::SyncStage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SyncStage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                equivalent: value.equivalent?,
                flag: value.flag?,
                support: value.support?,
            })
        }
    }
    impl ::std::convert::From<super::SyncStage> for SyncStage {
        fn from(value: super::SyncStage) -> Self {
            Self {
                equivalent: Ok(value.equivalent),
                flag: Ok(value.flag),
                support: Ok(value.support),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SyncSupport {
        max: ::std::result::Result<bool, ::std::string::String>,
        queues: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        stages: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Flag>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SyncSupport {
        fn default() -> Self {
            Self {
                max: Err("no value supplied for max".to_string()),
                queues: Err("no value supplied for queues".to_string()),
                stages: Err("no value supplied for stages".to_string()),
            }
        }
    }
    impl SyncSupport {
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn queues<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.queues = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for queues: {}", e));
            self
        }
        pub fn stages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Flag>>>,
            T::Error: ::std::fmt::Display,
        {
            self.stages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stages: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SyncSupport> for super::SyncSupport {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SyncSupport,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max: value.max?,
                queues: value.queues?,
                stages: value.stages?,
            })
        }
    }
    impl ::std::convert::From<super::SyncSupport> for SyncSupport {
        fn from(value: super::SyncSupport) -> Self {
            Self {
                max: Ok(value.max),
                queues: Ok(value.queues),
                stages: Ok(value.stages),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Version {
        feature_requirement: ::std::result::Result<
            ::std::vec::Vec<super::FeatureRequirement>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        name_api: ::std::result::Result<::std::string::String, ::std::string::String>,
        name_string: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Version {
        fn default() -> Self {
            Self {
                feature_requirement: Err("no value supplied for feature_requirement".to_string()),
                name: Err("no value supplied for name".to_string()),
                name_api: Err("no value supplied for name_api".to_string()),
                name_string: Err("no value supplied for name_string".to_string()),
            }
        }
    }
    impl Version {
        pub fn feature_requirement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FeatureRequirement>>,
            T::Error: ::std::fmt::Display,
        {
            self.feature_requirement = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for feature_requirement: {}",
                    e
                )
            });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn name_api<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name_api = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name_api: {}", e));
            self
        }
        pub fn name_string<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name_string = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name_string: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Version> for super::Version {
        type Error = super::error::ConversionError;
        fn try_from(value: Version) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                feature_requirement: value.feature_requirement?,
                name: value.name?,
                name_api: value.name_api?,
                name_string: value.name_string?,
            })
        }
    }
    impl ::std::convert::From<super::Version> for Version {
        fn from(value: super::Version) -> Self {
            Self {
                feature_requirement: Ok(value.feature_requirement),
                name: Ok(value.name),
                name_api: Ok(value.name_api),
                name_string: Ok(value.name_string),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoCodec {
        capabilities: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        formats: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::VideoFormat>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        profiles: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::VideoProfiles>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VideoCodec {
        fn default() -> Self {
            Self {
                capabilities: Err("no value supplied for capabilities".to_string()),
                formats: Err("no value supplied for formats".to_string()),
                name: Err("no value supplied for name".to_string()),
                profiles: Err("no value supplied for profiles".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl VideoCodec {
        pub fn capabilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.capabilities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for capabilities: {}", e));
            self
        }
        pub fn formats<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::VideoFormat>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.formats = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for formats: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn profiles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::VideoProfiles>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.profiles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for profiles: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<VideoCodec> for super::VideoCodec {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoCodec,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capabilities: value.capabilities?,
                formats: value.formats?,
                name: value.name?,
                profiles: value.profiles?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::VideoCodec> for VideoCodec {
        fn from(value: super::VideoCodec) -> Self {
            Self {
                capabilities: Ok(value.capabilities),
                formats: Ok(value.formats),
                name: Ok(value.name),
                profiles: Ok(value.profiles),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoFormat {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        properties: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        required_caps: ::std::result::Result<
            ::std::vec::Vec<super::VideoRequiredCapabilities>,
            ::std::string::String,
        >,
        usage: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for VideoFormat {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                properties: Err("no value supplied for properties".to_string()),
                required_caps: Err("no value supplied for required_caps".to_string()),
                usage: Err("no value supplied for usage".to_string()),
            }
        }
    }
    impl VideoFormat {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.properties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for properties: {}", e));
            self
        }
        pub fn required_caps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::VideoRequiredCapabilities>>,
            T::Error: ::std::fmt::Display,
        {
            self.required_caps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required_caps: {}", e));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<VideoFormat> for super::VideoFormat {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoFormat,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                properties: value.properties?,
                required_caps: value.required_caps?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::VideoFormat> for VideoFormat {
        fn from(value: super::VideoFormat) -> Self {
            Self {
                name: Ok(value.name),
                properties: Ok(value.properties),
                required_caps: Ok(value.required_caps),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoProfileMember {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        values: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VideoProfileMember {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl VideoProfileMember {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<VideoProfileMember> for super::VideoProfileMember {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoProfileMember,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::VideoProfileMember> for VideoProfileMember {
        fn from(value: super::VideoProfileMember) -> Self {
            Self {
                name: Ok(value.name),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoProfiles {
        members: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::VideoProfileMember>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for VideoProfiles {
        fn default() -> Self {
            Self {
                members: Err("no value supplied for members".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl VideoProfiles {
        pub fn members<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::VideoProfileMember>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.members = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for members: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<VideoProfiles> for super::VideoProfiles {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoProfiles,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                members: value.members?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::VideoProfiles> for VideoProfiles {
        fn from(value: super::VideoProfiles) -> Self {
            Self {
                members: Ok(value.members),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoRequiredCapabilities {
        member: ::std::result::Result<::std::string::String, ::std::string::String>,
        struct_: ::std::result::Result<::std::string::String, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for VideoRequiredCapabilities {
        fn default() -> Self {
            Self {
                member: Err("no value supplied for member".to_string()),
                struct_: Err("no value supplied for struct_".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl VideoRequiredCapabilities {
        pub fn member<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.member = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for member: {}", e));
            self
        }
        pub fn struct_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.struct_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for struct_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<VideoRequiredCapabilities> for super::VideoRequiredCapabilities {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoRequiredCapabilities,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                member: value.member?,
                struct_: value.struct_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::VideoRequiredCapabilities> for VideoRequiredCapabilities {
        fn from(value: super::VideoRequiredCapabilities) -> Self {
            Self {
                member: Ok(value.member),
                struct_: Ok(value.struct_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoStd {
        constants: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Constant>,
            ::std::string::String,
        >,
        enums: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Enum>,
            ::std::string::String,
        >,
        headers: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::VideoStdHeader>,
            ::std::string::String,
        >,
        structs: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Struct>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VideoStd {
        fn default() -> Self {
            Self {
                constants: Ok(Default::default()),
                enums: Ok(Default::default()),
                headers: Ok(Default::default()),
                structs: Ok(Default::default()),
            }
        }
    }
    impl VideoStd {
        pub fn constants<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Constant>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.constants = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constants: {}", e));
            self
        }
        pub fn enums<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Enum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.enums = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enums: {}", e));
            self
        }
        pub fn headers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::VideoStdHeader>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.headers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for headers: {}", e));
            self
        }
        pub fn structs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Struct>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.structs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for structs: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<VideoStd> for super::VideoStd {
        type Error = super::error::ConversionError;
        fn try_from(value: VideoStd) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                constants: value.constants?,
                enums: value.enums?,
                headers: value.headers?,
                structs: value.structs?,
            })
        }
    }
    impl ::std::convert::From<super::VideoStd> for VideoStd {
        fn from(value: super::VideoStd) -> Self {
            Self {
                constants: Ok(value.constants),
                enums: Ok(value.enums),
                headers: Ok(value.headers),
                structs: Ok(value.structs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoStdHeader {
        depends:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        header_file: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VideoStdHeader {
        fn default() -> Self {
            Self {
                depends: Err("no value supplied for depends".to_string()),
                header_file: Err("no value supplied for header_file".to_string()),
                name: Err("no value supplied for name".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl VideoStdHeader {
        pub fn depends<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.depends = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for depends: {}", e));
            self
        }
        pub fn header_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.header_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for header_file: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<VideoStdHeader> for super::VideoStdHeader {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoStdHeader,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                depends: value.depends?,
                header_file: value.header_file?,
                name: value.name?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::VideoStdHeader> for VideoStdHeader {
        fn from(value: super::VideoStdHeader) -> Self {
            Self {
                depends: Ok(value.depends),
                header_file: Ok(value.header_file),
                name: Ok(value.name),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VulkanObject {
        bitmasks: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Bitmask>,
            ::std::string::String,
        >,
        commands: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Command>,
            ::std::string::String,
        >,
        constants: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Constant>,
            ::std::string::String,
        >,
        enums: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Enum>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Extension>,
            ::std::string::String,
        >,
        flags: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Flags>,
            ::std::string::String,
        >,
        formats: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Format>,
            ::std::string::String,
        >,
        handles: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Handle>,
            ::std::string::String,
        >,
        header_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        header_version_complete:
            ::std::result::Result<::std::string::String, ::std::string::String>,
        platforms: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        spirv: ::std::result::Result<::std::vec::Vec<super::Spirv>, ::std::string::String>,
        structs: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Struct>,
            ::std::string::String,
        >,
        sync_access:
            ::std::result::Result<::std::vec::Vec<super::SyncAccess>, ::std::string::String>,
        sync_pipeline:
            ::std::result::Result<::std::vec::Vec<super::SyncPipeline>, ::std::string::String>,
        sync_stage: ::std::result::Result<::std::vec::Vec<super::SyncStage>, ::std::string::String>,
        vendor_tags:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        versions: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::Version>,
            ::std::string::String,
        >,
        video_codecs: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::VideoCodec>,
            ::std::string::String,
        >,
        video_std:
            ::std::result::Result<::std::option::Option<super::VideoStd>, ::std::string::String>,
    }
    impl ::std::default::Default for VulkanObject {
        fn default() -> Self {
            Self {
                bitmasks: Ok(Default::default()),
                commands: Ok(Default::default()),
                constants: Ok(Default::default()),
                enums: Ok(Default::default()),
                extensions: Ok(Default::default()),
                flags: Ok(Default::default()),
                formats: Ok(Default::default()),
                handles: Ok(Default::default()),
                header_version: Ok(Default::default()),
                header_version_complete: Ok(Default::default()),
                platforms: Ok(Default::default()),
                spirv: Ok(Default::default()),
                structs: Ok(Default::default()),
                sync_access: Ok(Default::default()),
                sync_pipeline: Ok(Default::default()),
                sync_stage: Ok(Default::default()),
                vendor_tags: Ok(Default::default()),
                versions: Ok(Default::default()),
                video_codecs: Ok(Default::default()),
                video_std: Ok(Default::default()),
            }
        }
    }
    impl VulkanObject {
        pub fn bitmasks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Bitmask>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bitmasks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bitmasks: {}", e));
            self
        }
        pub fn commands<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Command>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.commands = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for commands: {}", e));
            self
        }
        pub fn constants<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Constant>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.constants = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constants: {}", e));
            self
        }
        pub fn enums<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Enum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.enums = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enums: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Extension>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn flags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Flags>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.flags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flags: {}", e));
            self
        }
        pub fn formats<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Format>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.formats = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for formats: {}", e));
            self
        }
        pub fn handles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Handle>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.handles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for handles: {}", e));
            self
        }
        pub fn header_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.header_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for header_version: {}", e));
            self
        }
        pub fn header_version_complete<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.header_version_complete = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for header_version_complete: {}",
                    e
                )
            });
            self
        }
        pub fn platforms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.platforms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for platforms: {}", e));
            self
        }
        pub fn spirv<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Spirv>>,
            T::Error: ::std::fmt::Display,
        {
            self.spirv = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spirv: {}", e));
            self
        }
        pub fn structs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Struct>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.structs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for structs: {}", e));
            self
        }
        pub fn sync_access<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SyncAccess>>,
            T::Error: ::std::fmt::Display,
        {
            self.sync_access = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sync_access: {}", e));
            self
        }
        pub fn sync_pipeline<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SyncPipeline>>,
            T::Error: ::std::fmt::Display,
        {
            self.sync_pipeline = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sync_pipeline: {}", e));
            self
        }
        pub fn sync_stage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SyncStage>>,
            T::Error: ::std::fmt::Display,
        {
            self.sync_stage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sync_stage: {}", e));
            self
        }
        pub fn vendor_tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.vendor_tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vendor_tags: {}", e));
            self
        }
        pub fn versions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::Version>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.versions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for versions: {}", e));
            self
        }
        pub fn video_codecs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::VideoCodec>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.video_codecs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_codecs: {}", e));
            self
        }
        pub fn video_std<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VideoStd>>,
            T::Error: ::std::fmt::Display,
        {
            self.video_std = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_std: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<VulkanObject> for super::VulkanObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VulkanObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bitmasks: value.bitmasks?,
                commands: value.commands?,
                constants: value.constants?,
                enums: value.enums?,
                extensions: value.extensions?,
                flags: value.flags?,
                formats: value.formats?,
                handles: value.handles?,
                header_version: value.header_version?,
                header_version_complete: value.header_version_complete?,
                platforms: value.platforms?,
                spirv: value.spirv?,
                structs: value.structs?,
                sync_access: value.sync_access?,
                sync_pipeline: value.sync_pipeline?,
                sync_stage: value.sync_stage?,
                vendor_tags: value.vendor_tags?,
                versions: value.versions?,
                video_codecs: value.video_codecs?,
                video_std: value.video_std?,
            })
        }
    }
    impl ::std::convert::From<super::VulkanObject> for VulkanObject {
        fn from(value: super::VulkanObject) -> Self {
            Self {
                bitmasks: Ok(value.bitmasks),
                commands: Ok(value.commands),
                constants: Ok(value.constants),
                enums: Ok(value.enums),
                extensions: Ok(value.extensions),
                flags: Ok(value.flags),
                formats: Ok(value.formats),
                handles: Ok(value.handles),
                header_version: Ok(value.header_version),
                header_version_complete: Ok(value.header_version_complete),
                platforms: Ok(value.platforms),
                spirv: Ok(value.spirv),
                structs: Ok(value.structs),
                sync_access: Ok(value.sync_access),
                sync_pipeline: Ok(value.sync_pipeline),
                sync_stage: Ok(value.sync_stage),
                vendor_tags: Ok(value.vendor_tags),
                versions: Ok(value.versions),
                video_codecs: Ok(value.video_codecs),
                video_std: Ok(value.video_std),
            }
        }
    }
}
